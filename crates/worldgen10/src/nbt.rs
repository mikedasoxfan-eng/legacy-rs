//! Named Binary Tag format as b1.7.3 reads and writes it (big-endian, 10 tag types).

use std::io::{self, Read, Write};

#[derive(Clone, Debug, PartialEq)]
pub enum Tag {
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    /// Element type id and elements.
    List(u8, Vec<Tag>),
    Compound(Compound),
}

/// Key/value pairs in insertion order; serialized in Java HashMap iteration order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Compound(pub Vec<(String, Tag)>);

impl Compound {
    pub fn java_order(&self) -> Vec<usize> {
        let mut sorted: Vec<usize> = (0..self.0.len()).collect();
        sorted.sort_by(|&a, &b| self.0[a].0.encode_utf16().cmp(self.0[b].0.encode_utf16()));
        let mut ranks = vec![0; sorted.len()];
        for (rank, &i) in sorted.iter().enumerate() {
            ranks[i] = rank;
        }
        let mut order = crate::structures::long_map_order::LongMapOrder::default();
        for (i, (key, _)) in self.0.iter().enumerate() {
            let hash = key
                .encode_utf16()
                .fold(0i32, |h, c| h.wrapping_mul(31).wrapping_add(c as i32));
            order.insert(hash, ranks[i]);
        }
        order.iter().map(|rank| sorted[rank]).collect()
    }
    pub fn new() -> Self {
        Compound(Vec::new())
    }
    pub fn put(&mut self, key: &str, tag: Tag) {
        if let Some(e) = self.0.iter_mut().find(|e| e.0 == key) {
            e.1 = tag;
        } else {
            self.0.push((key.to_string(), tag));
        }
    }
    pub fn get(&self, key: &str) -> Option<&Tag> {
        self.0.iter().find(|e| e.0 == key).map(|e| &e.1)
    }
    pub fn has(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
    // Getters with the original's defaults for missing keys.
    pub fn byte(&self, key: &str) -> i8 {
        match self.get(key) {
            Some(Tag::Byte(v)) => *v,
            _ => 0,
        }
    }
    pub fn boolean(&self, key: &str) -> bool {
        self.byte(key) != 0
    }
    pub fn short(&self, key: &str) -> i16 {
        match self.get(key) {
            Some(Tag::Short(v)) => *v,
            _ => 0,
        }
    }
    pub fn int(&self, key: &str) -> i32 {
        match self.get(key) {
            Some(Tag::Int(v)) => *v,
            _ => 0,
        }
    }
    pub fn long(&self, key: &str) -> i64 {
        match self.get(key) {
            Some(Tag::Long(v)) => *v,
            _ => 0,
        }
    }
    pub fn string(&self, key: &str) -> String {
        match self.get(key) {
            Some(Tag::String(v)) => v.clone(),
            _ => String::new(),
        }
    }
    pub fn bytes(&self, key: &str) -> Option<&[u8]> {
        match self.get(key) {
            Some(Tag::ByteArray(v)) => Some(v),
            _ => None,
        }
    }
    pub fn compound(&self, key: &str) -> Option<&Compound> {
        match self.get(key) {
            Some(Tag::Compound(c)) => Some(c),
            _ => None,
        }
    }
    pub fn list(&self, key: &str) -> &[Tag] {
        match self.get(key) {
            Some(Tag::List(_, v)) => v,
            _ => &[],
        }
    }
}

impl Tag {
    pub fn id(&self) -> u8 {
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
        }
    }

    fn write_payload<W: Write>(&self, w: &mut W) -> io::Result<()> {
        match self {
            Tag::Byte(v) => w.write_all(&[*v as u8]),
            Tag::Short(v) => w.write_all(&v.to_be_bytes()),
            Tag::Int(v) => w.write_all(&v.to_be_bytes()),
            Tag::Long(v) => w.write_all(&v.to_be_bytes()),
            Tag::Float(v) => w.write_all(&v.to_be_bytes()),
            Tag::Double(v) => w.write_all(&v.to_be_bytes()),
            Tag::ByteArray(v) => {
                w.write_all(&(v.len() as i32).to_be_bytes())?;
                w.write_all(v)
            }
            Tag::String(s) => write_utf(w, s),
            Tag::List(_, items) => {
                // NBTTagList writes the type of its first element (1 = byte when empty).
                let ty = items
                    .first()
                    .map(Tag::id)
                    .unwrap_or(1);
                w.write_all(&[ty])?;
                w.write_all(&(items.len() as i32).to_be_bytes())?;
                for t in items {
                    t.write_payload(w)?;
                }
                Ok(())
            }
            Tag::Compound(c) => {
                for i in c.java_order() {
                    let (k, t) = &c.0[i];
                    write_named(w, k, t)?;
                }
                w.write_all(&[0])
            }
        }
    }

    fn read_payload<R: Read>(id: u8, r: &mut R, depth: u32) -> io::Result<Tag> {
        if depth > 512 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "NBT nested too deep",
            ));
        }
        Ok(match id {
            1 => Tag::Byte(read_n::<1, _>(r)?[0] as i8),
            2 => Tag::Short(i16::from_be_bytes(read_n(r)?)),
            3 => Tag::Int(i32::from_be_bytes(read_n(r)?)),
            4 => Tag::Long(i64::from_be_bytes(read_n(r)?)),
            5 => Tag::Float(f32::from_be_bytes(read_n(r)?)),
            6 => Tag::Double(f64::from_be_bytes(read_n(r)?)),
            7 => {
                let n = i32::from_be_bytes(read_n(r)?).max(0) as usize;
                let mut v = vec![0; n];
                r.read_exact(&mut v)?;
                Tag::ByteArray(v)
            }
            8 => Tag::String(read_utf(r)?),
            9 => {
                let ty = read_n::<1, _>(r)?[0];
                let n = i32::from_be_bytes(read_n(r)?).max(0) as usize;
                let mut items = Vec::with_capacity(n.min(4096));
                for _ in 0..n {
                    items.push(Tag::read_payload(ty, r, depth + 1)?);
                }
                Tag::List(ty, items)
            }
            10 => {
                let mut c = Compound::new();
                loop {
                    let ty = read_n::<1, _>(r)?[0];
                    if ty == 0 {
                        break;
                    }
                    let name = read_utf(r)?;
                    let t = Tag::read_payload(ty, r, depth + 1)?;
                    c.put(&name, t);
                }
                Tag::Compound(c)
            }
            _ => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!("bad NBT tag type {id}"),
                ))
            }
        })
    }
}

fn read_n<const N: usize, R: Read>(r: &mut R) -> io::Result<[u8; N]> {
    let mut b = [0u8; N];
    r.read_exact(&mut b)?;
    Ok(b)
}

/// Java modified UTF-8 is identical to UTF-8 for the strings the game writes.
fn write_utf<W: Write>(w: &mut W, s: &str) -> io::Result<()> {
    w.write_all(&(s.len() as u16).to_be_bytes())?;
    w.write_all(s.as_bytes())
}

fn read_utf<R: Read>(r: &mut R) -> io::Result<String> {
    let n = u16::from_be_bytes(read_n(r)?) as usize;
    let mut v = vec![0; n];
    r.read_exact(&mut v)?;
    Ok(String::from_utf8_lossy(&v).into_owned())
}

fn write_named<W: Write>(w: &mut W, name: &str, t: &Tag) -> io::Result<()> {
    w.write_all(&[t.id()])?;
    write_utf(w, name)?;
    t.write_payload(w)
}

/// `NBTBase.writeTag` for a root compound (named "").
pub fn write_root<W: Write>(w: &mut W, root: &Compound) -> io::Result<()> {
    write_named(w, "", &Tag::Compound(root.clone()))
}

/// Canonical comparison fixtures sort compound names instead of Java HashMap order.
/// Production saves continue to use write_root.
pub fn write_root_canonical<W: Write>(w: &mut W, root: &Compound) -> io::Result<()> {
    fn payload<W: Write>(w: &mut W, tag: &Tag) -> io::Result<()> {
        match tag {
            Tag::Compound(compound) => {
                let mut entries: Vec<_> = compound.0.iter().collect();
                entries.sort_by(|a, b| a.0.encode_utf16().cmp(b.0.encode_utf16()));
                for (name, value) in entries {
                    w.write_all(&[value.id()])?;
                    write_utf(w, name)?;
                    payload(w, value)?;
                }
                w.write_all(&[0])
            }
            Tag::List(_, items) => {
                w.write_all(&[items.first().map_or(1, Tag::id)])?;
                w.write_all(&(items.len() as i32).to_be_bytes())?;
                for value in items { payload(w, value)?; }
                Ok(())
            }
            _ => tag.write_payload(w),
        }
    }
    w.write_all(&[10, 0, 0])?;
    payload(w, &Tag::Compound(root.clone()))
}

/// `CompressedStreamTools.func_1141_a`: the root must be a compound.
pub fn read_root<R: Read>(r: &mut R) -> io::Result<Compound> {
    let ty = read_n::<1, _>(r)?[0];
    if ty != 10 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Root tag must be a named compound tag",
        ));
    }
    let _name = read_utf(r)?;
    match Tag::read_payload(10, r, 0)? {
        Tag::Compound(c) => Ok(c),
        _ => unreachable!(),
    }
}

pub fn read_gzip(bytes: &[u8]) -> io::Result<Compound> {
    read_root(&mut flate2::read::GzDecoder::new(bytes))
}

pub fn write_gzip(root: &Compound) -> io::Result<Vec<u8>> {
    let mut e = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
    write_root(&mut e, root)?;
    e.finish()
}
