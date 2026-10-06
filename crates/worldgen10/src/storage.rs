//! b1.7.3 save format: `level.dat` (`SaveHandler`/`WorldInfo`), McRegion files
//! (`RegionFile`, `RegionFileCache`) and chunk NBT (`ChunkLoader`), readable by the
//! original game.

use crate::nbt::{self, Compound, Tag};
use crate::world::chunk::{Chunk, Nibbles};
use crate::world::tile::TileEntity;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------- level.dat

/// `WorldInfo`
#[derive(Clone, Debug)]
pub struct LevelInfo {
    pub seed: i64,
    pub game_type: i32,
    pub map_features: bool,
    pub hardcore: bool,
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
            game_type: 0,
            hardcore: false,
            map_features: true,            spawn: (0, 0, 0),
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

    pub fn from_nbt(c: &Compound) -> Self {
        LevelInfo {
            seed: c.long("RandomSeed"),
            game_type: c.int("GameType"),
            map_features: !c.has("MapFeatures") || c.boolean("MapFeatures"),
            hardcore: c.boolean("hardcore"),
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
    pub fn to_nbt(&self) -> Compound {
        let mut c = Compound::new();
        c.put("RandomSeed", Tag::Long(self.seed));
        c.put("GameType", Tag::Int(self.game_type));
        c.put("MapFeatures", Tag::Byte(self.map_features as i8));
        c.put("hardcore", Tag::Byte(self.hardcore as i8));
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

    /// `func_22202_c`: the chunk has a slot in this region.
    pub fn has_chunk(&self, x: i32, z: i32) -> bool {
        (0..32).contains(&x) && (0..32).contains(&z) && self.offsets[(x + z * 32) as usize] != 0
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
    session: i64,
    legacy_chunks: bool,
    threaded_loader10: std::sync::Arc<crate::threaded_io10::ThreadedChunkLoader>,
}

impl Storage {
    /// Open (creating if needed) a save directory and take the session lock.
    pub fn open(dir: &Path) -> io::Result<Storage> {
        Self::open_with_players(dir, true)
    }

    /// SaveHandler's boolean controls creation of the players directory.
    /// RegionFileCache creates region/ lazily when a chunk is first accessed.
    pub fn open_with_players(dir: &Path, players: bool) -> io::Result<Storage> {
        std::fs::create_dir_all(dir.join("data"))?;
        if players { std::fs::create_dir_all(dir.join("players"))?; }
        let session = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as i64)
            .unwrap_or(0);
        std::fs::write(dir.join("session.lock"), session.to_be_bytes())?;
        Ok(Storage {
            dir: dir.to_path_buf(),
            chunk_dir: dir.to_path_buf(),
            session,
            legacy_chunks: false,
            threaded_loader10: crate::threaded_io10::ThreadedChunkLoader::new(dir),
        })
    }

    /// The same save (and session lock) with the chunk loader for `dimension`: the Nether
    /// keeps its regions in `DIM-1`.
    pub fn for_dimension(&self, dimension: crate::world::Dimension) -> io::Result<Storage> {
        let chunk_dir = match dimension { crate::world::Dimension::Nether => self.dir.join("DIM-1"), crate::world::Dimension::End => self.dir.join("DIM1"), _ => self.dir.clone() };
        std::fs::create_dir_all(&chunk_dir)?;
        let threaded_loader10 = crate::threaded_io10::ThreadedChunkLoader::new(&chunk_dir);
        Ok(Storage { dir: self.dir.clone(), chunk_dir, session: self.session, legacy_chunks: self.legacy_chunks, threaded_loader10 })
    }

    /// SaveHandler (as returned by SaveFormatOld) uses ChunkLoader, whereas
    /// SaveOldDir uses McRegionChunkLoader. Keep that choice across dimensions.
    pub fn open_legacy_with_players(dir: &Path, players: bool) -> io::Result<Storage> {
        let mut storage = Self::open_with_players(dir, players)?;
        storage.legacy_chunks = true;
        Ok(storage)
    }

    /// `checkSessionLock`: someone else opened this world since we did.
    pub fn session_ok(&self) -> bool {
        std::fs::read(self.dir.join("session.lock"))
            .map(|b| b.len() >= 8 && i64::from_be_bytes(b[..8].try_into().unwrap()) == self.session)
            .unwrap_or(false)
    }

    /// `SaveFormatOld.func_22173_b`: level.dat (or level.dat_old) without opening the save.
    pub fn read_level(dir: &Path) -> Option<LevelInfo> {
        for name in ["level.dat", "level.dat_old"] {
            if let Ok(root) = crate::storage181::read_gzip_file(&dir.join(name)) {
                    match root.get("Data") {
                        Some(Tag::Compound(data)) => return Some(LevelInfo::from_nbt(data)),
                        None => return Some(LevelInfo::from_nbt(&Compound::new())),
                        _ => (),
                    }
            }
        }
        None
    }

    pub fn load_level(&self) -> Option<LevelInfo> {
        Self::read_level(&self.dir)
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
        let _ = std::fs::rename(&new, &cur);
        // File.renameTo's result is ignored. Even after a failed rename the
        // original deletes the new file instead of retaining it for recovery.
        let _ = std::fs::remove_file(&new);
        Ok(())
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
        let root = if self.legacy_chunks {
            crate::legacy_chunk181::LegacyChunkLoader181::new(&self.chunk_dir, true)
                .read_root(cx, cz)?
        } else if let Some(root) = self.threaded_loader10.load_pending(cx, cz) { root }
        else {
            let raw = crate::region_cache181::read_chunk(&self.chunk_dir,cx,cz).ok()??;
            nbt::read_root(&mut &raw[..]).ok()?
        };
        let level = match root.compound("Level") {
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
            // Vanilla writes the corrected coordinates to the outer root,
            // then reloads the unchanged Level compound: relocation is ineffective.
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
        if self.legacy_chunks {
            if !self.session_ok() {
                return Err(io::Error::new(io::ErrorKind::Other, "Failed to check session lock, aborting"));
            }
            return crate::legacy_chunk181::LegacyChunkLoader181::new(&self.chunk_dir, true)
                .save_chunk(c, world_time);
        }
        if !self.session_ok() {
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to check session lock, aborting"));
        }
        let mut root = Compound::new();
        root.put("Level", Tag::Compound(chunk_to_nbt(c, world_time)));
        self.threaded_loader10.save(c.x, c.z, root);
        Ok(0)
    }

    /// Release cached RegionFiles (SaveConverterMcRegion.flushCache).
    pub fn flush_region_cache(&mut self) { crate::threaded_io10::FILE_IO.flush(); crate::region_cache181::clear(); }

    pub fn save_name10(&self) -> String { self.dir.file_name().unwrap_or_default().to_string_lossy().into_owned() }
    pub fn data_file(&self, name: &str) -> PathBuf { self.dir.join("data").join(format!("{name}.dat")) }
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
    if let Some(ticks) = &c.tile_ticks10 {
        l.put("TileTicks", Tag::List(10, ticks.iter().cloned().map(Tag::Compound).collect()));
    }
    let mut tes = Vec::new();
    let mut insertion = c.tile_entity_order.clone();
    let mut remaining: Vec<_> = c.tile_entities.keys().copied()
        .filter(|k| !insertion.contains(k)).collect();
    remaining.sort();
    insertion.extend(remaining);
    let mut order = crate::world::java_hash::JavaHashSet::default();
    for k in insertion {
        if c.tile_entities.contains_key(&k) {
            order.insert(k.0.wrapping_mul(8976890)
                .wrapping_add(k.1.wrapping_mul(981131)).wrapping_add(k.2), k);
        }
    }
    for k in order.iter() {
        let te = &c.tile_entities[&k];
        let mut t = Compound::new();
        let (x, y, z) = (c.x * 16 + k.0, k.1, c.z * 16 + k.2);
        match te {
            TileEntity::RecordPlayer { record } => {
                t = crate::world::notes::tile_nbt("RecordPlayer", x, y, z);
                if *record > 0 { t.put("Record", Tag::Int(*record)); }
            }
            TileEntity::Note(note)=> { t=note.to_nbt(x,y,z); },
            TileEntity::Piston(piston)=> { t=piston.to_nbt(x,y,z); },
            TileEntity::Sign(sign) => { t = sign.to_nbt(x, y, z); }
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
                        it.put("id", Tag::Short(s.id as i16));
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
            TileEntity::Furnace(f) => {
                t = f.to_nbt(x, y, z);
            }
            TileEntity::Dispenser(items) => {
                t = crate::container::tiles::inventory_nbt("Trap", x, y, z, items.as_ref());
            }
        }
        tes.push(Tag::Compound(t));
    }
    tes.extend(c.raw_tile_entities.iter().cloned().map(Tag::Compound));
    l.put("TileEntities", Tag::List(10, tes));
    l
}

/// `ChunkLoader.loadChunkIntoWorldFromCompound`. Note `lowestBlockHeight` is not stored and
/// stays 0, as in the original.
pub fn chunk_from_nbt(l: &Compound) -> Option<Chunk> {
    chunk_from_nbt_for_dimension(l, crate::world::Dimension::Overworld)
}

/// `ChunkLoader.loadChunkIntoWorldFromCompound` with the world's provider.
pub fn chunk_from_nbt_for_dimension(
    l: &Compound,
    _dimension: crate::world::Dimension,
) -> Option<Chunk> {
    // NBTTagCompound.getByteArray returns an empty (non-null) array for an
    // absent tag. NibbleArray.isValid tests null only, never its length.
    // Consequently vanilla preserves missing and malformed arrays; it does
    // not silently repair them. A wrong tag type throws and loadChunk skips it.
    fn bytes<'a>(l: &'a Compound, key: &str) -> Option<&'a [u8]> {
        match l.get(key) {None => Some(&[]),Some(Tag::ByteArray(v))=>Some(v),_=>None}
    }
    let blocks = bytes(l,"Blocks")?;
    let mut c = Chunk::new(
        l.int("xPos"),
        l.int("zPos"),
        blocks.to_vec().into_boxed_slice(),
    );
    c.lowest_block_height = 0;
    c.data = Nibbles(bytes(l,"Data")?.to_vec().into_boxed_slice());
    c.populated = l.boolean("TerrainPopulated");
    c.height_map=bytes(l,"HeightMap")?.to_vec().into_boxed_slice();
    c.sky_light=Nibbles(bytes(l,"SkyLight")?.to_vec().into_boxed_slice());
    c.block_light=Nibbles(bytes(l,"BlockLight")?.to_vec().into_boxed_slice());
    c.has_no_sky10 = _dimension.has_no_sky();
    c.tile_ticks10 = l.has("TileTicks").then(|| l.list("TileTicks").iter().filter_map(|t| match t {
        Tag::Compound(c) => Some(c.clone()), _ => None,
    }).collect());
    c.raw_entities = l.list("Entities").to_vec();
    c.has_entities = !c.raw_entities.is_empty();
    for t in l.list("TileEntities") {
        let Tag::Compound(t) = t else { continue };
        let (x, y, z) = (t.int("x"), t.int("y"), t.int("z"));
        let te = match TileEntity::create_and_load181(t).ok()? {
            Some((_, tile)) => tile,
            None => {
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
