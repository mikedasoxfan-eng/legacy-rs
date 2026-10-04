//! b1.7.3 save format: `level.dat` (`SaveHandler`/`WorldInfo`), McRegion files
//! (`RegionFile`, `RegionFileCache`) and chunk NBT (`ChunkLoader`), readable by the
//! original game.

use crate::nbt::{self, Compound, Tag};
use crate::overworld::CHUNK_VOLUME;
use crate::world::chunk::{Chunk, Nibbles};
use crate::world::tile::{ItemStack, TileEntity};
use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------- level.dat

/// `WorldInfo`
#[derive(Clone, Debug)]
pub struct LevelInfo {
    pub seed: i64,
    pub spawn: (i32, i32, i32),
    pub time: i64,
    pub size_on_disk: i64,
    pub last_played: i64,
    pub name: String,
    pub version: i32,
    pub rain_time: i32,
    pub raining: bool,
    pub thunder_time: i32,
    pub thundering: bool,
    /// Kept as-is so a player saved by the real game survives a round trip.
    pub player: Option<Compound>,
}

impl LevelInfo {
    /// `WorldInfo.getDimension`: the saved player's `Dimension` (0 without a player).
    pub fn dimension(&self) -> i32 {
        self.player.as_ref().map_or(0, |p| p.int("Dimension"))
    }
}

impl LevelInfo {
    /// `new WorldInfo(seed, name)`
    pub fn new(seed: i64, name: &str) -> Self {
        LevelInfo {
            seed,
            spawn: (0, 0, 0),
            time: 0,
            size_on_disk: 0,
            last_played: 0,
            name: name.to_string(),
            version: 19132,
            rain_time: 0,
            raining: false,
            thunder_time: 0,
            thundering: false,
            player: None,
        }
    }

    fn from_nbt(c: &Compound) -> Self {
        LevelInfo {
            seed: c.long("RandomSeed"),
            spawn: (c.int("SpawnX"), c.int("SpawnY"), c.int("SpawnZ")),
            time: c.long("Time"),
            size_on_disk: c.long("SizeOnDisk"),
            last_played: c.long("LastPlayed"),
            name: c.string("LevelName"),
            version: c.int("version"),
            rain_time: c.int("rainTime"),
            raining: c.boolean("raining"),
            thunder_time: c.int("thunderTime"),
            thundering: c.boolean("thundering"),
            player: c.compound("Player").cloned(),
        }
    }

    /// `updateTagCompound`
    fn to_nbt(&self) -> Compound {
        let mut c = Compound::new();
        c.put("RandomSeed", Tag::Long(self.seed));
        c.put("SpawnX", Tag::Int(self.spawn.0));
        c.put("SpawnY", Tag::Int(self.spawn.1));
        c.put("SpawnZ", Tag::Int(self.spawn.2));
        c.put("Time", Tag::Long(self.time));
        c.put("SizeOnDisk", Tag::Long(self.size_on_disk));
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        c.put("LastPlayed", Tag::Long(now));
        c.put("LevelName", Tag::String(self.name.clone()));
        c.put("version", Tag::Int(self.version));
        c.put("rainTime", Tag::Int(self.rain_time));
        c.put("raining", Tag::Byte(self.raining as i8));
        c.put("thunderTime", Tag::Int(self.thunder_time));
        c.put("thundering", Tag::Byte(self.thundering as i8));
        if let Some(p) = &self.player {
            c.put("Player", Tag::Compound(p.clone()));
        }
        c
    }
}

// ---------------------------------------------------------------- region files

/// `RegionFile`: 32x32 chunks; 4 KiB sectors; header of offsets and timestamps.
pub struct RegionFile {
    file: File,
    offsets: [i32; 1024],
    timestamps: [i32; 1024],
    sector_free: Vec<bool>,
    size_delta: i32,
}

impl RegionFile {
    pub fn open(path: &Path) -> io::Result<RegionFile> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)?;
        let mut size_delta = 0;
        let mut len = file.metadata()?.len();
        if len < 4096 {
            file.seek(SeekFrom::Start(0))?;
            file.write_all(&[0u8; 8192])?;
            size_delta += 8192;
            len = file.metadata()?.len();
        }
        if len & 4095 != 0 {
            // (sic) pads by len & 4095 bytes, which only aligns when that is exactly half.
            file.seek(SeekFrom::End(0))?;
            file.write_all(&vec![0u8; (len & 4095) as usize])?;
            len = file.metadata()?.len();
        }
        let sectors = (len / 4096) as usize;
        let mut sector_free = vec![true; sectors];
        sector_free[0] = false;
        sector_free[1] = false;
        let mut header = vec![0u8; 8192];
        file.seek(SeekFrom::Start(0))?;
        file.read_exact(&mut header)?;
        let mut offsets = [0i32; 1024];
        let mut timestamps = [0i32; 1024];
        for i in 0..1024 {
            let o = i32::from_be_bytes(header[i * 4..i * 4 + 4].try_into().unwrap());
            offsets[i] = o;
            if o != 0 && ((o >> 8) + (o & 255)) as usize <= sector_free.len() {
                for k in 0..(o & 255) as usize {
                    sector_free[(o >> 8) as usize + k] = false;
                }
            }
            timestamps[i] =
                i32::from_be_bytes(header[4096 + i * 4..4096 + i * 4 + 4].try_into().unwrap());
        }
        Ok(RegionFile {
            file,
            offsets,
            timestamps,
            sector_free,
            size_delta,
        })
    }

    /// `getChunkDataInputStream`, decompressed. `None` if absent or unreadable.
    pub fn read_chunk(&mut self, x: i32, z: i32) -> Option<Vec<u8>> {
        if !(0..32).contains(&x) || !(0..32).contains(&z) {
            return None;
        }
        let o = self.offsets[(x + z * 32) as usize];
        if o == 0 {
            return None;
        }
        let (sector, count) = ((o >> 8) as usize, (o & 255) as usize);
        if sector + count > self.sector_free.len() {
            return None;
        }
        self.file.seek(SeekFrom::Start(sector as u64 * 4096)).ok()?;
        let mut hdr = [0u8; 5];
        self.file.read_exact(&mut hdr).ok()?;
        let len = i32::from_be_bytes(hdr[..4].try_into().unwrap());
        if len as usize > 4096 * count || len < 1 {
            return None;
        }
        let mut data = vec![0u8; len as usize - 1];
        self.file.read_exact(&mut data).ok()?;
        let mut out = Vec::new();
        match hdr[4] {
            1 => flate2::read::GzDecoder::new(&data[..])
                .read_to_end(&mut out)
                .ok()?,
            2 => flate2::read::ZlibDecoder::new(&data[..])
                .read_to_end(&mut out)
                .ok()?,
            _ => return None,
        };
        Some(out)
    }

    /// `write(x, z, data, length)` with zlib-compressed data, allocating sectors like the original.
    pub fn write_chunk(&mut self, x: i32, z: i32, compressed: &[u8]) -> io::Result<()> {
        if !(0..32).contains(&x) || !(0..32).contains(&z) {
            return Ok(());
        }
        let len = compressed.len();
        let o = self.offsets[(x + z * 32) as usize];
        let (mut sector, count) = ((o >> 8) as usize, (o & 255) as usize);
        let needed = (len + 5) / 4096 + 1;
        if needed >= 256 {
            return Ok(());
        }
        if sector != 0 && count == needed {
            self.write_at(sector, compressed)?;
        } else {
            for k in 0..count {
                self.sector_free[sector + k] = true;
            }
            let mut start = self.sector_free.iter().position(|&f| f);
            let mut run = 0;
            if let Some(first) = start {
                for i in first..self.sector_free.len() {
                    if run != 0 {
                        if self.sector_free[i] {
                            run += 1;
                        } else {
                            run = 0;
                        }
                    } else if self.sector_free[i] {
                        start = Some(i);
                        run = 1;
                    }
                    if run >= needed {
                        break;
                    }
                }
            }
            if run >= needed {
                sector = start.unwrap();
                self.set_offset(x, z, (sector << 8 | needed) as i32)?;
                for k in 0..needed {
                    self.sector_free[sector + k] = false;
                }
                self.write_at(sector, compressed)?;
            } else {
                self.file.seek(SeekFrom::End(0))?;
                sector = self.sector_free.len();
                for _ in 0..needed {
                    self.file.write_all(&[0u8; 4096])?;
                    self.sector_free.push(false);
                }
                self.size_delta += 4096 * needed as i32;
                self.write_at(sector, compressed)?;
                self.set_offset(x, z, (sector << 8 | needed) as i32)?;
            }
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i32)
            .unwrap_or(0);
        self.timestamps[(x + z * 32) as usize] = now;
        self.file
            .seek(SeekFrom::Start(4096 + (x + z * 32) as u64 * 4))?;
        self.file.write_all(&now.to_be_bytes())?;
        Ok(())
    }

    fn write_at(&mut self, sector: usize, data: &[u8]) -> io::Result<()> {
        self.file.seek(SeekFrom::Start(sector as u64 * 4096))?;
        self.file
            .write_all(&((data.len() + 1) as i32).to_be_bytes())?;
        self.file.write_all(&[2])?;
        self.file.write_all(data)
    }

    fn set_offset(&mut self, x: i32, z: i32, v: i32) -> io::Result<()> {
        self.offsets[(x + z * 32) as usize] = v;
        self.file.seek(SeekFrom::Start((x + z * 32) as u64 * 4))?;
        self.file.write_all(&v.to_be_bytes())
    }

    /// `getSizeDelta`
    pub fn take_size_delta(&mut self) -> i32 {
        std::mem::take(&mut self.size_delta)
    }
}

// ---------------------------------------------------------------- world directory

/// A world save directory (`SaveHandler` + `McRegionChunkLoader` + `RegionFileCache`).
pub struct Storage {
    pub dir: PathBuf,
    /// Where `region/` lives: the save directory, or `DIM-1` for the Nether
    /// (`SaveOldDir.getChunkLoader`).
    chunk_dir: PathBuf,
    regions: HashMap<(i32, i32), RegionFile>,
    session: i64,
}

impl Storage {
    /// Open (creating if needed) a save directory and take the session lock.
    pub fn open(dir: &Path) -> io::Result<Storage> {
        std::fs::create_dir_all(dir.join("region"))?;
        std::fs::create_dir_all(dir.join("data"))?;
        std::fs::create_dir_all(dir.join("players"))?;
        let session = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        std::fs::write(dir.join("session.lock"), session.to_be_bytes())?;
        Ok(Storage {
            dir: dir.to_path_buf(),
            chunk_dir: dir.to_path_buf(),
            regions: HashMap::new(),
            session,
        })
    }

    /// The same save (and session lock) with the chunk loader for `dimension`: the Nether
    /// keeps its regions in `DIM-1`.
    pub fn for_dimension(&self, dimension: crate::world::Dimension) -> io::Result<Storage> {
        let chunk_dir = if dimension.is_hell_world() { self.dir.join("DIM-1") } else { self.dir.clone() };
        std::fs::create_dir_all(chunk_dir.join("region"))?;
        Ok(Storage { dir: self.dir.clone(), chunk_dir, regions: HashMap::new(), session: self.session })
    }

    /// `checkSessionLock`: someone else opened this world since we did.
    pub fn session_ok(&self) -> bool {
        std::fs::read(self.dir.join("session.lock"))
            .map(|b| b.len() == 8 && i64::from_be_bytes(b[..8].try_into().unwrap()) == self.session)
            .unwrap_or(false)
    }

    pub fn load_level(&self) -> Option<LevelInfo> {
        for name in ["level.dat", "level.dat_old"] {
            if let Ok(bytes) = std::fs::read(self.dir.join(name)) {
                if let Ok(root) = nbt::read_gzip(&bytes) {
                    if let Some(data) = root.compound("Data") {
                        return Some(LevelInfo::from_nbt(data));
                    }
                }
            }
        }
        None
    }

    /// `saveWorldInfoAndPlayer`: write level.dat_new, keep the previous file as level.dat_old.
    pub fn save_level(&self, info: &LevelInfo) -> io::Result<()> {
        let mut root = Compound::new();
        root.put("Data", Tag::Compound(info.to_nbt()));
        let bytes = nbt::write_gzip(&root)?;
        let (new, old, cur) = (
            self.dir.join("level.dat_new"),
            self.dir.join("level.dat_old"),
            self.dir.join("level.dat"),
        );
        std::fs::write(&new, bytes)?;
        let _ = std::fs::remove_file(&old);
        let _ = std::fs::rename(&cur, &old);
        let _ = std::fs::remove_file(&cur);
        std::fs::rename(&new, &cur)
    }

    fn region(&mut self, cx: i32, cz: i32) -> io::Result<&mut RegionFile> {
        let key = (cx >> 5, cz >> 5);
        if !self.regions.contains_key(&key) {
            if self.regions.len() >= 256 {
                self.regions.clear();
            }
            let path = self
                .chunk_dir
                .join("region")
                .join(format!("r.{}.{}.mcr", key.0, key.1));
            self.regions.insert(key, RegionFile::open(&path)?);
        }
        Ok(self.regions.get_mut(&key).unwrap())
    }

    /// `McRegionChunkLoader.loadChunk`
    pub fn load_chunk(&mut self, cx: i32, cz: i32) -> Option<Chunk> {
        self.load_chunk_for_dimension(cx, cz, crate::world::Dimension::Overworld)
    }

    /// The chunk loader receives the world provider when repairing missing light arrays.
    pub fn load_chunk_for_dimension(
        &mut self,
        cx: i32,
        cz: i32,
        dimension: crate::world::Dimension,
    ) -> Option<Chunk> {
        let raw = self.region(cx, cz).ok()?.read_chunk(cx & 31, cz & 31)?;
        let root = nbt::read_root(&mut &raw[..]).ok()?;
        let mut level = match root.compound("Level") {
            Some(l) if l.has("Blocks") => l.clone(),
            Some(_) => {
                eprintln!("Chunk file at {cx},{cz} is missing block data, skipping");
                return None;
            }
            None => {
                eprintln!("Chunk file at {cx},{cz} is missing level data, skipping");
                return None;
            }
        };
        if level.int("xPos") != cx || level.int("zPos") != cz {
            eprintln!("Chunk file at {cx},{cz} is in the wrong location; relocating.");
            level.put("xPos", Tag::Int(cx));
            level.put("zPos", Tag::Int(cz));
        }
        let mut c = chunk_from_nbt_for_dimension(&level, dimension)?;
        // ChunkBlockMap: ids without a block become air.
        for b in c.blocks.iter_mut() {
            if *b != 0 && !crate::block::props(*b).exists {
                *b = 0;
            }
        }
        Some(c)
    }

    /// `McRegionChunkLoader.saveChunk`. Returns the region file growth (`SizeOnDisk` delta).
    pub fn save_chunk(&mut self, c: &Chunk, world_time: i64) -> io::Result<i64> {
        let mut root = Compound::new();
        root.put("Level", Tag::Compound(chunk_to_nbt(c, world_time)));
        let mut raw = Vec::new();
        nbt::write_root(&mut raw, &root)?;
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        z.write_all(&raw)?;
        let compressed = z.finish()?;
        let r = self.region(c.x, c.z)?;
        r.write_chunk(c.x & 31, c.z & 31, &compressed)?;
        Ok(r.take_size_delta() as i64)
    }
}

// ---------------------------------------------------------------- chunk <-> NBT

/// `ChunkLoader.storeChunkInCompound` (no entities are simulated yet).
pub fn chunk_to_nbt(c: &Chunk, world_time: i64) -> Compound {
    let mut l = Compound::new();
    l.put("xPos", Tag::Int(c.x));
    l.put("zPos", Tag::Int(c.z));
    l.put("LastUpdate", Tag::Long(world_time));
    l.put("Blocks", Tag::ByteArray(c.blocks.to_vec()));
    l.put("Data", Tag::ByteArray(c.data.0.to_vec()));
    l.put("SkyLight", Tag::ByteArray(c.sky_light.0.to_vec()));
    l.put("BlockLight", Tag::ByteArray(c.block_light.0.to_vec()));
    l.put("HeightMap", Tag::ByteArray(c.height_map.to_vec()));
    l.put("TerrainPopulated", Tag::Byte(c.populated as i8));
    l.put("Entities", Tag::List(10, c.raw_entities.clone()));
    let mut tes = Vec::new();
    let mut keys: Vec<_> = c.tile_entities.keys().copied().collect();
    keys.sort();
    for k in keys {
        let te = &c.tile_entities[&k];
        let mut t = Compound::new();
        let (x, y, z) = (c.x * 16 + k.0, k.1, c.z * 16 + k.2);
        match te {
            TileEntity::Chest(slots) => {
                t.put("id", Tag::String("Chest".into()));
                t.put("x", Tag::Int(x));
                t.put("y", Tag::Int(y));
                t.put("z", Tag::Int(z));
                let mut items = Vec::new();
                for (i, s) in slots.iter().enumerate() {
                    if let Some(s) = s {
                        let mut it = Compound::new();
                        it.put("Slot", Tag::Byte(i as i8));
                        it.put("id", Tag::Short(s.id));
                        it.put("Count", Tag::Byte(s.count as i8));
                        it.put("Damage", Tag::Short(s.damage));
                        items.push(Tag::Compound(it));
                    }
                }
                t.put("Items", Tag::List(10, items));
            }
            TileEntity::MobSpawner { mob, delay } => {
                t.put("id", Tag::String("MobSpawner".into()));
                t.put("x", Tag::Int(x));
                t.put("y", Tag::Int(y));
                t.put("z", Tag::Int(z));
                t.put("EntityId", Tag::String(mob.clone()));
                t.put("Delay", Tag::Short(*delay as i16));
            }
        }
        tes.push(Tag::Compound(t));
    }
    tes.extend(c.raw_tile_entities.iter().cloned().map(Tag::Compound));
    l.put("TileEntities", Tag::List(10, tes));
    l
}

fn nibbles(bytes: Option<&[u8]>) -> Option<Nibbles> {
    // NibbleArray.isValid(): data present; the arrays the game writes are always full size.
    let b = bytes?;
    if b.len() != CHUNK_VOLUME / 2 {
        return None;
    }
    Some(Nibbles(b.to_vec().into_boxed_slice().try_into().ok()?))
}

/// `ChunkLoader.loadChunkIntoWorldFromCompound`. Note `lowestBlockHeight` is not stored and
/// stays 0, as in the original.
pub fn chunk_from_nbt(l: &Compound) -> Option<Chunk> {
    chunk_from_nbt_for_dimension(l, crate::world::Dimension::Overworld)
}

/// `ChunkLoader.loadChunkIntoWorldFromCompound` with the world's provider.
pub fn chunk_from_nbt_for_dimension(
    l: &Compound,
    dimension: crate::world::Dimension,
) -> Option<Chunk> {
    let blocks = l.bytes("Blocks")?;
    if blocks.len() != CHUNK_VOLUME {
        return None;
    }
    let mut c = Chunk::new(
        l.int("xPos"),
        l.int("zPos"),
        blocks.to_vec().into_boxed_slice().try_into().ok()?,
    );
    c.lowest_block_height = 0;
    c.data = nibbles(l.bytes("Data")).unwrap_or_default();
    c.populated = l.boolean("TerrainPopulated");
    let height = l.bytes("HeightMap").filter(|h| h.len() == 256);
    match (height, nibbles(l.bytes("SkyLight"))) {
        (Some(h), Some(sky)) => {
            c.height_map.copy_from_slice(h);
            c.sky_light = sky;
        }
        _ => {
            c.sky_light = Nibbles::default();
            c.init_height_and_sky_for_dimension(dimension);
        }
    }
    c.block_light = nibbles(l.bytes("BlockLight")).unwrap_or_default();
    c.raw_entities = l.list("Entities").to_vec();
    c.has_entities = !c.raw_entities.is_empty();
    for t in l.list("TileEntities") {
        let Tag::Compound(t) = t else { continue };
        let (x, y, z) = (t.int("x"), t.int("y"), t.int("z"));
        let te = match t.string("id").as_str() {
            "Chest" => {
                let mut slots = Box::new([None; 27]);
                for it in t.list("Items") {
                    let Tag::Compound(it) = it else { continue };
                    let slot = (it.byte("Slot") as u8) as usize;
                    if slot < 27 {
                        slots[slot] = Some(ItemStack {
                            id: it.short("id"),
                            count: it.byte("Count") as u8,
                            damage: it.short("Damage"),
                        });
                    }
                }
                TileEntity::Chest(slots)
            }
            "MobSpawner" => TileEntity::MobSpawner {
                mob: t.string("EntityId"),
                delay: t.short("Delay") as i32,
            },
            _ => {
                // Not modelled yet: keep it for the next save if it sits on a container block.
                let (lx, lz) = (x - c.x * 16, z - c.z * 16);
                if (0..16).contains(&lx) && (0..16).contains(&lz) && (0..128).contains(&y) {
                    let id = c.block(lx, y, lz);
                    if id != 0 && crate::block::props(id).container {
                        c.raw_tile_entities.push(t.clone());
                    }
                }
                continue;
            }
        };
        // Chunk.addTileEntity -> setChunkBlockTileEntity: only on a matching container block.
        let (lx, lz) = (x - c.x * 16, z - c.z * 16);
        if (0..16).contains(&lx) && (0..16).contains(&lz) && (0..128).contains(&y) {
            let id = c.block(lx, y, lz);
            if id != 0 && crate::block::props(id).container {
                if !c.tile_entities.contains_key(&(lx, y, lz)) {
                    c.tile_entity_order.push((lx, y, lz));
                }
                c.tile_entities.insert((lx, y, lz), te);
            }
        }
    }
    Some(c)
}
