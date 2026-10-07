//! `give`: put items into the player's inventory of a singleplayer world, outside the game.
//!
//! The game itself stays exactly like the original (Minecraft 1.0 has no /give and no way to
//! pick up blocks like the End portal frame); this edits the save the same way an NBT editor
//! would. Works for Beta 1.7.3, Beta 1.8.1 and 1.0 worlds (they share the level.dat format).
//!
//! Usage: give <world folder or level.dat> <item id> [count] [damage]
//! A copy of the old file is kept as level.dat.bak. Close the game first.

use std::io::{Read, Write};
use std::path::PathBuf;

const USAGE: &str = "\
give: put items into your inventory in a singleplayer world (close the game first)

Usage:  give <world folder or level.dat> <item id> [count] [damage]

Examples:
  give \"1.0\\saves\\My World\" 120 12      12 End portal frames
  give \"1.0\\saves\\My World\" 122         the dragon egg
  give \"saves\\World1\" 35 64 14           64 red wool

The old file is kept as level.dat.bak next to it.";

/// One NBT value. Names and strings stay raw bytes (Java's modified UTF-8), so everything
/// this program does not touch is written back byte for byte.
#[derive(Clone, Debug)]
enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(u32),
    Double(u64),
    ByteArray(Vec<u8>),
    String(Vec<u8>),
    List(u8, Vec<Tag>),
    Compound(Vec<(Vec<u8>, Tag)>),
    IntArray(Vec<i32>),
}

impl Tag {
    fn id(&self) -> u8 {
        match self {
            Tag::Byte(_) => 1,
            Tag::Short(_) => 2,
            Tag::Int(_) => 3,
            Tag::Long(_) => 4,
            Tag::Float(_) => 5,
            Tag::Double(_) => 6,
            Tag::ByteArray(_) => 7,
            Tag::String(_) => 8,
            Tag::List(..) => 9,
            Tag::Compound(_) => 10,
            Tag::IntArray(_) => 11,
        }
    }

    fn get(&self, name: &str) -> Option<&Tag> {
        match self {
            Tag::Compound(entries) => entries.iter().find(|(n, _)| n == name.as_bytes()).map(|(_, t)| t),
            _ => None,
        }
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Tag> {
        match self {
            Tag::Compound(entries) => entries.iter_mut().find(|(n, _)| n == name.as_bytes()).map(|(_, t)| t),
            _ => None,
        }
    }

    fn as_i64(&self) -> Option<i64> {
        match *self {
            Tag::Byte(v) => Some(v as i64),
            Tag::Short(v) => Some(v as i64),
            Tag::Int(v) => Some(v as i64),
            Tag::Long(v) => Some(v),
            _ => None,
        }
    }
}

struct Reader<'a> {
    data: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn take(&mut self, n: usize) -> Result<&[u8], String> {
        let end = self.at.checked_add(n).filter(|&e| e <= self.data.len()).ok_or("level.dat ends too early")?;
        let s = &self.data[self.at..end];
        self.at = end;
        Ok(s)
    }
    fn u8(&mut self) -> Result<u8, String> {
        Ok(self.take(1)?[0])
    }
    fn u16(&mut self) -> Result<u16, String> {
        Ok(u16::from_be_bytes(self.take(2)?.try_into().unwrap()))
    }
    fn i32(&mut self) -> Result<i32, String> {
        Ok(i32::from_be_bytes(self.take(4)?.try_into().unwrap()))
    }
    fn u64(&mut self) -> Result<u64, String> {
        Ok(u64::from_be_bytes(self.take(8)?.try_into().unwrap()))
    }
    fn bytes16(&mut self) -> Result<Vec<u8>, String> {
        let n = self.u16()? as usize;
        Ok(self.take(n)?.to_vec())
    }
    fn len(&mut self) -> Result<usize, String> {
        let n = self.i32()?;
        usize::try_from(n).map_err(|_| "negative length in level.dat".to_string())
    }
    fn payload(&mut self, id: u8) -> Result<Tag, String> {
        Ok(match id {
            1 => Tag::Byte(self.u8()? as i8),
            2 => Tag::Short(self.u16()? as i16),
            3 => Tag::Int(self.i32()?),
            4 => Tag::Long(self.u64()? as i64),
            5 => Tag::Float(self.i32()? as u32),
            6 => Tag::Double(self.u64()?),
            7 => {
                let n = self.len()?;
                Tag::ByteArray(self.take(n)?.to_vec())
            }
            8 => Tag::String(self.bytes16()?),
            9 => {
                let item = self.u8()?;
                let n = self.len()?;
                let mut v = Vec::with_capacity(n.min(1 << 16));
                for _ in 0..n {
                    v.push(self.payload(item)?);
                }
                Tag::List(item, v)
            }
            10 => {
                let mut v = Vec::new();
                loop {
                    let t = self.u8()?;
                    if t == 0 {
                        break;
                    }
                    let name = self.bytes16()?;
                    v.push((name, self.payload(t)?));
                }
                Tag::Compound(v)
            }
            11 => {
                let n = self.len()?;
                let mut v = Vec::with_capacity(n.min(1 << 16));
                for _ in 0..n {
                    v.push(self.i32()?);
                }
                Tag::IntArray(v)
            }
            other => return Err(format!("unknown NBT tag type {other} in level.dat")),
        })
    }
}

fn write_payload(out: &mut Vec<u8>, tag: &Tag) {
    match tag {
        Tag::Byte(v) => out.push(*v as u8),
        Tag::Short(v) => out.extend(v.to_be_bytes()),
        Tag::Int(v) => out.extend(v.to_be_bytes()),
        Tag::Long(v) => out.extend(v.to_be_bytes()),
        Tag::Float(v) => out.extend(v.to_be_bytes()),
        Tag::Double(v) => out.extend(v.to_be_bytes()),
        Tag::ByteArray(v) => {
            out.extend((v.len() as i32).to_be_bytes());
            out.extend(v);
        }
        Tag::String(v) => {
            out.extend((v.len() as u16).to_be_bytes());
            out.extend(v);
        }
        Tag::List(item, v) => {
            out.push(*item);
            out.extend((v.len() as i32).to_be_bytes());
            for t in v {
                write_payload(out, t);
            }
        }
        Tag::Compound(v) => {
            for (name, t) in v {
                out.push(t.id());
                out.extend((name.len() as u16).to_be_bytes());
                out.extend(name);
                write_payload(out, t);
            }
            out.push(0);
        }
        Tag::IntArray(v) => {
            out.extend((v.len() as i32).to_be_bytes());
            for x in v {
                out.extend(x.to_be_bytes());
            }
        }
    }
}

/// How many fit in one slot, as in the original items (1.0 values; blocks are 64).
fn max_stack(id: i16) -> i16 {
    match id {
        256..=259 | 261 | 267..=279 | 282..=286 | 290..=294 | 298..=317 | 323..=330 | 333 | 335
        | 342 | 343 | 346 | 354 | 355 | 358 | 359 | 373 | 2256..=2266 => 1,
        332 | 344 | 368 => 16,
        _ => 64,
    }
}

fn stack(slot: i8, id: i16, count: i8, damage: i16) -> Tag {
    // Same field order as the original ItemStack.writeToNBT, plus the slot InventoryPlayer adds first.
    Tag::Compound(vec![
        (b"Slot".to_vec(), Tag::Byte(slot)),
        (b"id".to_vec(), Tag::Short(id)),
        (b"Count".to_vec(), Tag::Byte(count)),
        (b"Damage".to_vec(), Tag::Short(damage)),
    ])
}

fn run() -> Result<String, String> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 3 || args.iter().any(|a| a == "--help" || a == "-h") {
        return Err(USAGE.to_string());
    }
    let mut path = PathBuf::from(&args[1]);
    if path.is_dir() {
        path.push("level.dat");
    }
    if !path.is_file() {
        return Err(format!("no level.dat at {}\n\n{USAGE}", path.display()));
    }
    let id: i16 = args[2].parse().ok().filter(|&v| v > 0).ok_or("the item id must be a number from 1 to 32767")?;
    let mut left: i32 = match args.get(3) {
        Some(c) => c.parse().ok().filter(|&v: &i32| (1..=36 * 64).contains(&v)).ok_or("the count must be a number from 1 to 2304")?,
        None => 1,
    };
    let damage: i16 = match args.get(4) {
        Some(d) => d.parse().map_err(|_| "the damage/colour value must be a number from 0 to 32767")?,
        None => 0,
    };

    let packed = std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut raw = Vec::new();
    flate2::read::GzDecoder::new(&packed[..])
        .read_to_end(&mut raw)
        .map_err(|e| format!("{} is not a gzip-compressed level.dat: {e}", path.display()))?;
    let mut r = Reader { data: &raw, at: 0 };
    if r.u8()? != 10 {
        return Err("level.dat does not start with a compound tag".into());
    }
    let root_name = r.bytes16()?;
    let mut root = r.payload(10)?;

    let player = root
        .get_mut("Data")
        .and_then(|d| d.get_mut("Player"))
        .ok_or("this level.dat has no player (open the world once in singleplayer first)")?;
    let inventory = match player.get_mut("Inventory") {
        Some(Tag::List(item, v)) if *item == 10 || v.is_empty() => {
            *item = 10;
            v
        }
        _ => return Err("the player in this level.dat has no inventory list".into()),
    };

    let max = max_stack(id) as i32;
    let requested = left;
    // Top up matching hotbar stacks first (without enchantments or other extra data).
    for t in inventory.iter_mut() {
        let slot = t.get("Slot").and_then(Tag::as_i64).unwrap_or(-1);
        let same = (0..9).contains(&slot)
            && t.get("id").and_then(Tag::as_i64) == Some(id as i64)
            && t.get("Damage").and_then(Tag::as_i64).unwrap_or(0) == damage as i64
            && t.get("tag").is_none();
        if !same || left == 0 {
            continue;
        }
        if let Some(Tag::Byte(c)) = t.get_mut("Count") {
            let add = (max - *c as i32).clamp(0, left);
            *c = (*c as i32 + add) as i8;
            left -= add;
        }
    }
    // New stacks go into the hotbar, so they show up in creative too (1.0's creative inventory
    // screen shows only the hotbar, and creative fills every empty hotbar slot with default
    // blocks). When the hotbar is full, its last slots move into free main-inventory slots.
    let slot_of = |t: &Tag| t.get("Slot").and_then(Tag::as_i64).unwrap_or(-1);
    let mut moved = 0;
    while left > 0 {
        let used: Vec<i64> = inventory.iter().map(slot_of).collect();
        let slot = if let Some(free) = (0..9i64).find(|s| !used.contains(s)) {
            free
        } else if let (Some(evict), Some(spare)) = (
            (0..9i64).rev().find(|s| !inventory.iter().any(|t| slot_of(t) == *s && t.get("id").and_then(Tag::as_i64) == Some(id as i64))),
            (9..36i64).find(|s| !used.contains(s)),
        ) {
            for t in inventory.iter_mut() {
                if slot_of(t) == evict {
                    if let Some(Tag::Byte(s)) = t.get_mut("Slot") {
                        *s = spare as i8;
                    }
                }
            }
            moved += 1;
            evict
        } else if let Some(free) = (9..36i64).find(|s| !used.contains(s)) {
            free
        } else {
            break;
        };
        let n = left.min(max);
        inventory.push(stack(slot as i8, id, n as i8, damage));
        left -= n;
    }
    let given = requested - left;
    if given == 0 {
        return Err("the inventory is full; empty some slots in the game and try again".into());
    }

    let mut out = vec![10u8];
    out.extend((root_name.len() as u16).to_be_bytes());
    out.extend(&root_name);
    write_payload(&mut out, &root);
    let backup = path.with_extension("dat.bak");
    std::fs::copy(&path, &backup).map_err(|e| format!("could not write the backup {}: {e}", backup.display()))?;
    let mut gz = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    gz.write_all(&out).map_err(|e| e.to_string())?;
    let packed = gz.finish().map_err(|e| e.to_string())?;
    let tmp = path.with_extension("dat.give-tmp");
    std::fs::write(&tmp, &packed).map_err(|e| format!("{}: {e}", tmp.display()))?;
    std::fs::rename(&tmp, &path).map_err(|e| format!("{}: {e}", path.display()))?;

    let mut msg = format!("Gave {given} of item {id}{} in {}.", if damage != 0 { format!(":{damage}") } else { String::new() }, path.display());
    if left > 0 {
        msg += &format!(" The inventory was full, so {left} did not fit.");
    }
    if moved > 0 {
        msg += &format!(" The hotbar was full, so {moved} of its items moved into your main inventory to make room.");
    }
    msg += &format!("\nBackup: {}", backup.display());
    Ok(msg)
}

fn main() {
    match run() {
        Ok(msg) => println!("{msg}"),
        Err(msg) => {
            eprintln!("{msg}");
            std::process::exit(1);
        }
    }
}
