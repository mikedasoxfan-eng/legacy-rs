//! The parts of `World`, `Chunk`, `ChunkProvider` and `MetadataChunkBlock` that
//! world generation exercises, ported so that population replays exactly.
//!
//! Method names follow the Java (snake_cased) with the original name noted where
//! it was obfuscated. Call order matters everywhere in here: block hooks, light
//! scheduling and chunk loading all have side effects that later code observes.

pub mod behavior;
pub mod chunk;
pub mod light;
pub mod server;
pub mod java_hash;
pub mod tick;
pub mod ticks_blocks;
pub mod tile;

use crate::block::{self, light_opacity, material_of, mat, props, MAT_AIR};
use crate::overworld::OverworldGenerator;
use crate::random::JavaRandom;
use chunk::{idx, Chunk, LightKind};
use light::LightUpdate;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use tile::TileEntity;

/// Which `java.util.Random` a block tick draws from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RandSel {
    /// `ChunkProviderGenerate.rand`, shared by chunk creation and population.
    Populate,
    /// `World.rand` (unseeded in vanilla).
    World,
}

/// A pending entry of `World.scheduledTickSet`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TickKey {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub block: i32,
}

/// An item entity spawned by a block drop. Kept only for inspection.
#[derive(Clone, Debug)]
pub struct DroppedItem {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub item: tile::ItemStack,
}

/// `BlockFlowing` keeps scratch state in instance fields that survive re-entrant
/// ticks; the original reads them back after recursion, so we must too.
#[derive(Clone, Copy, Debug, Default)]
pub struct FlowState {
    pub num_adjacent_sources: i32,
    pub optimal: [bool; 4],
    pub cost: [i32; 4],
}

/// FxHash-style hasher for chunk coordinates.
#[derive(Default)]
pub struct CoordHasher(u64);

impl Hasher for CoordHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, bytes: &[u8]) {
        for &b in bytes {
            self.0 = (self.0.rotate_left(5) ^ b as u64).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
        }
    }
    fn write_i32(&mut self, i: i32) {
        self.0 = (self.0.rotate_left(5) ^ i as u32 as u64).wrapping_mul(0x51_7c_c1_b7_27_22_0a_95);
    }
}

pub type ChunkMap = HashMap<(i32, i32), Box<Chunk>, BuildHasherDefault<CoordHasher>>;

#[inline]
fn in_xz_bounds(x: i32, z: i32) -> bool {
    x >= -32000000 && z >= -32000000 && x < 32000000 && z <= 32000000
}

pub struct World {
    pub seed: i64,
    pub terrain: OverworldGenerator,
    pub chunks: ChunkMap,
    /// `World.rand`
    pub rand: JavaRandom,
    /// `World.lightingToUpdate`
    pub light_queue: Vec<LightUpdate>,
    lighting_updates_counter: i32,
    /// `scheduledTickSet`: entry -> (scheduled time, `tickEntryID`).
    pub pending_ticks: HashMap<TickKey, (i64, u64)>,
    /// `scheduledTickTreeSet`, ordered like `NextTickListEntry.compareTo`.
    pub tick_order: std::collections::BTreeSet<(i64, u64, TickKey)>,
    /// `NextTickListEntry.nextTickEntryID` (a JVM-wide static in the original).
    pub next_tick_entry_id: u64,
    pub world_time: i64,
    /// `World.scheduledUpdatesAreImmediate`
    pub immediate_updates: bool,
    /// `World.editingBlocks`
    pub editing_blocks: bool,
    /// `BlockSand.fallInstantly` (a static in Java)
    pub sand_fall_instantly: bool,
    /// `Block.leaves.isOpaqueCube()`: true until a renderer sets fancy graphics.
    pub leaves_opaque: bool,
    pub flow: [FlowState; 2],
    pub spawn: (i32, i32, i32),
    pub dropped_items: Vec<DroppedItem>,
    pub falling_sand: Vec<(i32, i32, i32, u8)>,
    /// Bumped on every chunk modification; chunks record it in `Chunk::version`.
    pub mod_count: u64,
    /// Save directory; `None` keeps everything in memory.
    pub storage: Option<crate::storage::Storage>,
    /// `WorldInfo` fields the world doesn't otherwise track (name, weather, saved player).
    pub info: crate::storage::LevelInfo,
    /// `ChunkProvider.chunkList`: loaded chunks in load order (`saveChunks` walks it).
    pub chunk_list: Vec<(i32, i32)>,
    /// Multiplayer-server chunk handling; `None` is singleplayer (chunks never unload).
    pub server: Option<server::ServerChunks>,
    /// `WorldServer.levelSaving`
    pub level_saving: bool,
    /// Chunks unloaded since the renderer last looked (drained by the client).
    pub unloaded: Vec<(i32, i32)>,
    pub raining_strength: f32,
    pub prev_raining_strength: f32,
    pub thundering_strength: f32,
    pub prev_thundering_strength: f32,
    /// `field_27168_F`: sky flash after lightning.
    pub lightning_flash: i32,
    /// `skylightSubtracted`
    pub skylight_subtracted: i32,
    /// Set when `skylightSubtracted` changes (`updateAllRenderers`); cleared by the client.
    pub renderers_dirty: bool,
    /// `field_9437_g`: the LCG picking random-tick positions (unseeded in vanilla).
    pub update_lcg: i32,
    /// `soundCounter` for cave ambience.
    pub sound_counter: i32,
    /// Positions of the players (`playerEntities`); drive random ticks.
    pub players: Vec<(f64, f64, f64)>,
    positions_to_update: tick::JavaHashSet<(i32, i32)>,
    /// `BlockLeaves.adjacentTreeBlocks`: scratch kept between decay checks, as in the original.
    pub leaf_scratch: Option<Vec<i32>>,
    /// Lightning strikes (`EntityLightningBolt`), recorded until entities exist.
    pub lightning_strikes: Vec<(i32, i32, i32)>,
}

impl World {
    /// A world with a preset spawn (no spawn search), like loading `level.dat`.
    pub fn new(seed: i64, world_rand_seed: i64) -> Self {
        World {
            seed,
            terrain: OverworldGenerator::new(seed),
            chunks: ChunkMap::default(),
            rand: JavaRandom::new(world_rand_seed),
            light_queue: Vec::new(),
            lighting_updates_counter: 0,
            pending_ticks: HashMap::new(),
            tick_order: Default::default(),
            next_tick_entry_id: 0,
            world_time: 0,
            immediate_updates: false,
            editing_blocks: false,
            sand_fall_instantly: false,
            leaves_opaque: true,
            flow: [FlowState::default(); 2],
            spawn: (0, 0, 0),
            dropped_items: Vec::new(),
            falling_sand: Vec::new(),
            mod_count: 0,
            storage: None,
            info: crate::storage::LevelInfo::new(seed, "world"),
            chunk_list: Vec::new(),
            server: None,
            level_saving: false,
            unloaded: Vec::new(),
            raining_strength: 0.0,
            prev_raining_strength: 0.0,
            thundering_strength: 0.0,
            prev_thundering_strength: 0.0,
            lightning_flash: 0,
            skylight_subtracted: 0,
            renderers_dirty: false,
            update_lcg: 0,
            sound_counter: 0,
            players: Vec::new(),
            positions_to_update: Default::default(),
            leaf_scratch: None,
            lightning_strikes: Vec::new(),
        }
    }

    /// `provideChunk` for other world modules (see `access_chunk`).
    #[inline]
    pub(crate) fn access_chunk_pub(&mut self, cx: i32, cz: i32) -> bool {
        self.access_chunk(cx, cz)
    }

    /// Mark a chunk as changed: `isModified` for saving, and a new version for renderers.
    #[inline]
    fn touch(&mut self, cx: i32, cz: i32) {
        self.mod_count += 1;
        let v = self.mod_count;
        if let Some(c) = self.chunks.get_mut(&(cx, cz)) {
            c.version = v;
            c.modified = true;
        }
    }

    /// `provideChunk` as the world sees it: true if a real chunk is there (loading or
    /// generating it if allowed). On a server, chunks nobody loaded come back as the
    /// shared `EmptyChunk` instead (all air, writes ignored): we return false.
    #[inline]
    fn access_chunk(&mut self, cx: i32, cz: i32) -> bool {
        if self.chunk_exists(cx, cz) {
            return true;
        }
        if let Some(s) = &self.server {
            if !s.world_override && !s.load_override {
                return false;
            }
            self.server_load_chunk(cx, cz);
            return true;
        }
        self.prepare_chunk(cx, cz);
        true
    }

    #[inline]
    pub fn rng(&mut self, sel: RandSel) -> &mut JavaRandom {
        match sel {
            RandSel::Populate => self.terrain.rand_mut(),
            RandSel::World => &mut self.rand,
        }
    }

    /// The population RNG (`ChunkProviderGenerate.rand`).
    #[inline]
    pub fn prand(&mut self) -> &mut JavaRandom {
        self.terrain.rand_mut()
    }

    // ------------------------------------------------------------ chunk provider

    #[inline]
    pub fn chunk_exists(&self, cx: i32, cz: i32) -> bool {
        self.chunks.contains_key(&(cx, cz))
    }

    #[inline]
    pub fn chunk(&self, cx: i32, cz: i32) -> &Chunk {
        &self.chunks[&(cx, cz)]
    }

    #[inline]
    pub fn chunk_mut(&mut self, cx: i32, cz: i32) -> &mut Chunk {
        self.chunks.get_mut(&(cx, cz)).expect("chunk not loaded")
    }

    /// `ChunkProvider.provideChunk`: load (generate + maybe populate) if missing.
    #[inline]
    pub fn provide_chunk(&mut self, cx: i32, cz: i32) {
        if !self.chunk_exists(cx, cz) {
            self.prepare_chunk(cx, cz);
        }
    }

    /// `ChunkProvider.prepareChunk` / `ChunkProviderServer.loadChunk` for a chunk not in
    /// the map: read it from the save, else generate it; then run the population checks.
    pub(crate) fn prepare_chunk(&mut self, cx: i32, cz: i32) {
        let saved = self.storage.as_mut().and_then(|s| s.load_chunk(cx, cz));
        if let Some(mut c) = saved {
            c.last_save_time = self.world_time;
            c.loaded = true;
            self.chunks.insert((cx, cz), Box::new(c));
            self.chunk_list.push((cx, cz));
            self.mod_count += 1;
            let v = self.mod_count;
            self.chunk_mut(cx, cz).version = v;
        } else {
            // ChunkProviderGenerate.provideChunk
            let raw = self.terrain.generate(cx, cz);
            let mut c = Box::new(Chunk::new(cx, cz, raw.blocks));
            c.init_height_and_sky();
            // func_1024_c's neighbour pass runs before the chunk is in the map, so
            // its own columns read as height 0 through the world.
            for x in 0..16 {
                for z in 0..16 {
                    let h = c.height(x, z);
                    self.update_neighbor_heights(cx * 16 + x, cz * 16 + z, h);
                }
            }
            c.loaded = true;
            self.chunks.insert((cx, cz), c);
            self.chunk_list.push((cx, cz));
            self.touch(cx, cz);
        }

        if !self.chunk(cx, cz).populated
            && self.chunk_exists(cx + 1, cz + 1)
            && self.chunk_exists(cx, cz + 1)
            && self.chunk_exists(cx + 1, cz)
        {
            self.populate_chunk(cx, cz);
        }
        if self.chunk_exists(cx - 1, cz)
            && !self.chunk(cx - 1, cz).populated
            && self.chunk_exists(cx - 1, cz + 1)
            && self.chunk_exists(cx, cz + 1)
            && self.chunk_exists(cx - 1, cz)
        {
            self.populate_chunk(cx - 1, cz);
        }
        if self.chunk_exists(cx, cz - 1)
            && !self.chunk(cx, cz - 1).populated
            && self.chunk_exists(cx + 1, cz - 1)
            && self.chunk_exists(cx, cz - 1)
            && self.chunk_exists(cx + 1, cz)
        {
            self.populate_chunk(cx, cz - 1);
        }
        if self.chunk_exists(cx - 1, cz - 1)
            && !self.chunk(cx - 1, cz - 1).populated
            && self.chunk_exists(cx - 1, cz - 1)
            && self.chunk_exists(cx, cz - 1)
            && self.chunk_exists(cx - 1, cz)
        {
            self.populate_chunk(cx - 1, cz - 1);
        }
    }

    /// `ChunkProvider.populate` (only ever called for loaded chunks).
    pub fn populate_chunk(&mut self, cx: i32, cz: i32) {
        self.provide_chunk(cx, cz);
        if !self.chunk(cx, cz).populated {
            self.chunk_mut(cx, cz).populated = true;
            crate::populate::populate(self, cx, cz);
            self.touch(cx, cz); // setChunkModified
        }
    }

    /// `getChunkFromChunkCoords` for the client's renderers (singleplayer: loads the chunk).
    pub fn load_chunk(&mut self, cx: i32, cz: i32) {
        self.provide_chunk(cx, cz);
    }

    // ------------------------------------------------------------ block access

    /// `getBlockId`. Loads the chunk for in-range y.
    pub fn get_block_id(&mut self, x: i32, y: i32, z: i32) -> u8 {
        if !in_xz_bounds(x, z) || !(0..128).contains(&y) {
            return block::AIR;
        }
        if !self.access_chunk(x >> 4, z >> 4) {
            return block::AIR;
        }
        self.chunk(x >> 4, z >> 4).block(x & 15, y, z & 15)
    }

    #[inline]
    pub fn is_air_block(&mut self, x: i32, y: i32, z: i32) -> bool {
        self.get_block_id(x, y, z) == block::AIR
    }

    /// `getBlockMaterial` → material index.
    #[inline]
    pub fn get_block_material(&mut self, x: i32, y: i32, z: i32) -> u8 {
        material_of(self.get_block_id(x, y, z))
    }

    pub fn get_block_metadata(&mut self, x: i32, y: i32, z: i32) -> i32 {
        if !in_xz_bounds(x, z) || !(0..128).contains(&y) {
            return 0;
        }
        if !self.access_chunk(x >> 4, z >> 4) {
            return 0;
        }
        self.chunk(x >> 4, z >> 4).data.get(x & 15, y, z & 15)
    }

    pub fn set_block_and_metadata(&mut self, x: i32, y: i32, z: i32, id: u8, meta: i32) -> bool {
        if !in_xz_bounds(x, z) || !(0..128).contains(&y) {
            return false;
        }
        if !self.access_chunk(x >> 4, z >> 4) {
            return true; // EmptyChunk.setBlockIDWithMetadata
        }
        self.chunk_set_block_with_metadata(x >> 4, z >> 4, x & 15, y, z & 15, id, meta)
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, id: u8) -> bool {
        if !in_xz_bounds(x, z) || !(0..128).contains(&y) {
            return false;
        }
        if !self.access_chunk(x >> 4, z >> 4) {
            return true; // EmptyChunk.setBlockID
        }
        self.chunk_set_block(x >> 4, z >> 4, x & 15, y, z & 15, id)
    }

    pub fn set_block_metadata(&mut self, x: i32, y: i32, z: i32, meta: i32) -> bool {
        if !in_xz_bounds(x, z) || !(0..128).contains(&y) {
            return false;
        }
        if !self.access_chunk(x >> 4, z >> 4) {
            return true; // EmptyChunk.setBlockMetadata does nothing
        }
        self.chunk_mut(x >> 4, z >> 4).data.set(x & 15, y, z & 15, meta);
        self.touch(x >> 4, z >> 4);
        true
    }

    pub fn set_block_metadata_with_notify(&mut self, x: i32, y: i32, z: i32, meta: i32) {
        if self.set_block_metadata(x, y, z, meta) {
            let id = self.get_block_id(x, y, z);
            // notifyBlockChange vs notifyBlocksOfNeighborChange differ only in renderer marking.
            self.notify_blocks_of_neighbor_change(x, y, z, id);
        }
    }

    pub fn set_block_with_notify(&mut self, x: i32, y: i32, z: i32, id: u8) -> bool {
        if self.set_block(x, y, z, id) {
            self.notify_blocks_of_neighbor_change(x, y, z, id);
            true
        } else {
            false
        }
    }

    pub fn set_block_and_metadata_with_notify(&mut self, x: i32, y: i32, z: i32, id: u8, meta: i32) -> bool {
        if self.set_block_and_metadata(x, y, z, id, meta) {
            self.notify_blocks_of_neighbor_change(x, y, z, id);
            true
        } else {
            false
        }
    }

    pub fn notify_blocks_of_neighbor_change(&mut self, x: i32, y: i32, z: i32, id: u8) {
        self.notify_block_of_neighbor_change(x - 1, y, z, id);
        self.notify_block_of_neighbor_change(x + 1, y, z, id);
        self.notify_block_of_neighbor_change(x, y - 1, z, id);
        self.notify_block_of_neighbor_change(x, y + 1, z, id);
        self.notify_block_of_neighbor_change(x, y, z - 1, id);
        self.notify_block_of_neighbor_change(x, y, z + 1, id);
    }

    fn notify_block_of_neighbor_change(&mut self, x: i32, y: i32, z: i32, changed: u8) {
        if !self.editing_blocks {
            let id = self.get_block_id(x, y, z);
            if id != block::AIR {
                behavior::on_neighbor_block_change(self, id, x, y, z, changed);
            }
        }
    }

    // ------------------------------------------------------------ Chunk.setBlockID*

    /// `Chunk.setBlockID`
    fn chunk_set_block(&mut self, cx: i32, cz: i32, x: i32, y: i32, z: i32, id: u8) -> bool {
        let c = self.chunk_mut(cx, cz);
        let height = c.height(x, z);
        let old = c.blocks[idx(x, y, z)];
        if old == id {
            return false;
        }
        let wx = cx * 16 + x;
        let wz = cz * 16 + z;
        c.blocks[idx(x, y, z)] = id;
        self.touch(cx, cz);
        if old != block::AIR {
            behavior::on_block_removal(self, old, wx, y, wz);
        }
        self.chunk_mut(cx, cz).data.set(x, y, z, 0);
        if light_opacity(id) != 0 {
            if y >= height {
                self.relight_column(cx, cz, x, y + 1, z);
            }
        } else if y == height - 1 {
            self.relight_column(cx, cz, x, y, z);
        }
        self.schedule_lighting_update(LightKind::Sky, wx, y, wz, wx, y, wz);
        self.schedule_lighting_update(LightKind::Block, wx, y, wz, wx, y, wz);
        let h = self.chunk(cx, cz).height(x, z);
        self.update_neighbor_heights(wx, wz, h);
        if id != block::AIR {
            behavior::on_block_added(self, id, wx, y, wz);
        }
        true
    }

    /// `Chunk.setBlockIDWithMetadata`
    #[allow(clippy::too_many_arguments)]
    fn chunk_set_block_with_metadata(&mut self, cx: i32, cz: i32, x: i32, y: i32, z: i32, id: u8, meta: i32) -> bool {
        let c = self.chunk_mut(cx, cz);
        let height = c.height(x, z);
        let old = c.blocks[idx(x, y, z)];
        if old == id && c.data.get(x, y, z) == meta {
            return false;
        }
        let wx = cx * 16 + x;
        let wz = cz * 16 + z;
        c.blocks[idx(x, y, z)] = id;
        self.touch(cx, cz);
        if old != block::AIR {
            behavior::on_block_removal(self, old, wx, y, wz);
        }
        self.chunk_mut(cx, cz).data.set(x, y, z, meta);
        if light_opacity(id) != 0 {
            if y >= height {
                self.relight_column(cx, cz, x, y + 1, z);
            }
        } else if y == height - 1 {
            self.relight_column(cx, cz, x, y, z);
        }
        self.schedule_lighting_update(LightKind::Sky, wx, y, wz, wx, y, wz);
        self.schedule_lighting_update(LightKind::Block, wx, y, wz, wx, y, wz);
        let h = self.chunk(cx, cz).height(x, z);
        self.update_neighbor_heights(wx, wz, h);
        self.chunk_mut(cx, cz).data.set(x, y, z, meta);
        if id != block::AIR {
            behavior::on_block_added(self, id, wx, y, wz);
        }
        true
    }

    /// `Chunk.func_1003_g`: move a column's height and patch its skylight.
    fn relight_column(&mut self, cx: i32, cz: i32, x: i32, y: i32, z: i32) {
        let c = self.chunk_mut(cx, cz);
        let old_h = c.height(x, z);
        let mut h = old_h.max(y);
        while h > 0 && light_opacity(c.block(x, h - 1, z)) == 0 {
            h -= 1;
        }
        if h == old_h {
            return;
        }
        self.touch(cx, cz);
        let c = self.chunk_mut(cx, cz);
        c.height_map[(z << 4 | x) as usize] = h as u8;
        if h < c.lowest_block_height {
            c.lowest_block_height = h;
        } else {
            c.lowest_block_height = c.height_map.iter().map(|&v| v as i32).fold(127, i32::min);
        }
        let wx = cx * 16 + x;
        let wz = cz * 16 + z;
        if h < old_h {
            for yy in h..old_h {
                c.sky_light.set(x, yy, z, 15);
            }
        } else {
            self.schedule_lighting_update(LightKind::Sky, wx, old_h, wz, wx, h, wz);
            let c = self.chunk_mut(cx, cz);
            for yy in old_h..h {
                c.sky_light.set(x, yy, z, 0);
            }
        }
        let c = self.chunk_mut(cx, cz);
        let mut light = 15;
        let top = h;
        while h > 0 && light > 0 {
            h -= 1;
            let mut op = light_opacity(c.block(x, h, z));
            if op == 0 {
                op = 1;
            }
            light -= op;
            if light < 0 {
                light = 0;
            }
            c.sky_light.set(x, h, z, light);
        }
        while h > 0 && light_opacity(c.block(x, h - 1, z)) == 0 {
            h -= 1;
        }
        if h != top {
            self.schedule_lighting_update(LightKind::Sky, wx - 1, h, wz - 1, wx + 1, top, wz + 1);
        }
    }

    /// `Chunk.func_996_c` (+ `func_1020_f`): schedule skylight fixes against the
    /// four horizontal neighbour columns of world column (wx, wz) at height `h`.
    fn update_neighbor_heights(&mut self, wx: i32, wz: i32, h: i32) {
        self.check_neighbor_height(wx - 1, wz, h);
        self.check_neighbor_height(wx + 1, wz, h);
        self.check_neighbor_height(wx, wz - 1, h);
        self.check_neighbor_height(wx, wz + 1, h);
    }

    fn check_neighbor_height(&mut self, x: i32, z: i32, h: i32) {
        let nh = self.get_height_value(x, z);
        if nh > h {
            self.schedule_lighting_update(LightKind::Sky, x, h, z, x, nh, z);
        } else if nh < h {
            self.schedule_lighting_update(LightKind::Sky, x, nh, z, x, h, z);
        }
    }

    // ------------------------------------------------------------ queries

    /// `getHeightValue` (does not load chunks).
    pub fn get_height_value(&self, x: i32, z: i32) -> i32 {
        if !in_xz_bounds(x, z) || !self.chunk_exists(x >> 4, z >> 4) {
            return 0;
        }
        self.chunk(x >> 4, z >> 4).height(x & 15, z & 15)
    }

    /// `canBlockSeeTheSky` (loads the chunk).
    pub fn can_block_see_the_sky(&mut self, x: i32, y: i32, z: i32) -> bool {
        if !self.access_chunk(x >> 4, z >> 4) {
            return false;
        }
        self.chunk(x >> 4, z >> 4).can_block_see_the_sky(x & 15, y, z & 15)
    }

    /// `getFullBlockLightValue` (loads the chunk).
    pub fn get_full_block_light_value(&mut self, x: i32, y: i32, z: i32) -> i32 {
        if y < 0 {
            return 0;
        }
        let y = y.min(127);
        if !self.access_chunk(x >> 4, z >> 4) {
            return 0;
        }
        self.chunk(x >> 4, z >> 4).block_light_value(x & 15, y, z & 15, 0)
    }

    pub fn can_existing_block_see_the_sky(&self, x: i32, y: i32, z: i32) -> bool {
        if !in_xz_bounds(x, z) || y < 0 {
            return false;
        }
        if y >= 128 {
            return true;
        }
        if !self.chunk_exists(x >> 4, z >> 4) {
            return false;
        }
        self.chunk(x >> 4, z >> 4).can_block_see_the_sky(x & 15, y, z & 15)
    }

    pub fn block_exists(&self, x: i32, y: i32, z: i32) -> bool {
        (0..128).contains(&y) && self.chunk_exists(x >> 4, z >> 4)
    }

    pub fn check_chunks_exist(&self, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) -> bool {
        if y1 >= 0 && y0 < 128 {
            for cx in x0 >> 4..=x1 >> 4 {
                for cz in z0 >> 4..=z1 >> 4 {
                    if !self.chunk_exists(cx, cz) {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    /// `findTopSolidBlock`
    pub fn find_top_solid_block(&mut self, x: i32, z: i32) -> i32 {
        if !self.access_chunk(x >> 4, z >> 4) {
            return -1;
        }
        let c = self.chunk(x >> 4, z >> 4);
        let (lx, lz) = (x & 15, z & 15);
        let mut y = 127;
        while y > 0 {
            let m = mat(material_of(c.block(lx, y, lz)));
            if m.blocks_movement || m.is_liquid {
                return y + 1;
            }
            y -= 1;
        }
        -1
    }

    /// `isBlockNormalCube`
    pub fn is_block_normal_cube(&mut self, x: i32, y: i32, z: i32) -> bool {
        let id = self.get_block_id(x, y, z);
        let p = props(id);
        p.exists && mat(p.material).translucent_solid && p.normal_render
    }

    /// `Block.isOpaqueCube()` on the instance (differs from the lookup table for leaves).
    pub fn is_opaque_cube_instance(&self, id: u8) -> bool {
        if id == block::LEAVES {
            self.leaves_opaque
        } else {
            block::opaque_cube_lookup(id)
        }
    }

    // ------------------------------------------------------------ light

    pub fn get_saved_light_value(&self, kind: LightKind, x: i32, y: i32, z: i32) -> i32 {
        let y = y.clamp(0, 127);
        if in_xz_bounds(x, z) {
            if !self.chunk_exists(x >> 4, z >> 4) {
                return 0;
            }
            self.chunk(x >> 4, z >> 4).saved_light(kind, x & 15, y, z & 15)
        } else {
            kind.default_value()
        }
    }

    pub fn set_light_value(&mut self, kind: LightKind, x: i32, y: i32, z: i32, v: i32) {
        if in_xz_bounds(x, z) && (0..128).contains(&y) && self.chunk_exists(x >> 4, z >> 4) {
            self.chunk_mut(x >> 4, z >> 4).set_light(kind, x & 15, y, z & 15, v);
            self.touch(x >> 4, z >> 4);
        }
    }

    pub fn neighbor_light_propagation_changed(&mut self, kind: LightKind, x: i32, y: i32, z: i32, mut v: i32) {
        if !self.block_exists(x, y, z) {
            return;
        }
        match kind {
            LightKind::Sky => {
                if self.can_existing_block_see_the_sky(x, y, z) {
                    v = 15;
                }
            }
            LightKind::Block => {
                let id = self.get_block_id(x, y, z);
                if block::light_value(id) > v {
                    v = block::light_value(id);
                }
            }
        }
        if self.get_saved_light_value(kind, x, y, z) != v {
            self.schedule_lighting_update(kind, x, y, z, x, y, z);
        }
    }

    /// `scheduleLightingUpdate_do(..., true)`
    #[allow(clippy::too_many_arguments)]
    pub fn schedule_lighting_update(&mut self, kind: LightKind, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) {
        let mx = (x1 + x0) / 2;
        let mz = (z1 + z0) / 2;
        if !self.block_exists(mx, 64, mz) {
            return;
        }
        let n = self.light_queue.len();
        for i in 0..n.min(5) {
            let u = &mut self.light_queue[n - i - 1];
            if u.kind == kind && u.try_merge(x0, y0, z0, x1, y1, z1) {
                return;
            }
        }
        self.light_queue.push(LightUpdate::new(kind, x0, y0, z0, x1, y1, z1));
        if self.light_queue.len() > 1_000_000 {
            self.light_queue.clear();
        }
    }

    /// `updatingLighting`: process up to ~500 queued updates; true if more remain.
    pub fn updating_lighting(&mut self) -> bool {
        if self.lighting_updates_counter >= 50 {
            return false;
        }
        self.lighting_updates_counter += 1;
        let mut budget = 500;
        let more = loop {
            if self.light_queue.is_empty() {
                break false;
            }
            budget -= 1;
            if budget <= 0 {
                break true;
            }
            let mut u = self.light_queue.pop().unwrap();
            u.apply(self);
        };
        self.lighting_updates_counter -= 1;
        more
    }

    pub fn drain_lighting(&mut self) {
        while self.updating_lighting() {}
    }

    // ------------------------------------------------------------ ticks

    /// `scheduleBlockUpdate`
    pub fn schedule_block_update(&mut self, x: i32, y: i32, z: i32, id: u8, delay: i32) {
        // new NextTickListEntry(...) takes an id from the global counter every time.
        let entry_id = self.next_tick_entry_id;
        self.next_tick_entry_id += 1;
        let r = 8;
        if self.immediate_updates {
            if self.check_chunks_exist(x - r, y - r, z - r, x + r, y + r, z + r) {
                let cur = self.get_block_id(x, y, z);
                if cur == id && cur > 0 {
                    behavior::update_tick(self, cur, x, y, z, RandSel::World);
                }
            }
        } else if self.check_chunks_exist(x - r, y - r, z - r, x + r, y + r, z + r) {
            let key = TickKey { x, y, z, block: id as i32 };
            let t = delay as i64 + self.world_time;
            if let std::collections::hash_map::Entry::Vacant(e) = self.pending_ticks.entry(key) {
                e.insert((t, entry_id));
                self.tick_order.insert((t, entry_id, key));
            }
        }
    }

    // ------------------------------------------------------------ tile entities

    /// `getBlockTileEntity` (creates one for an orphaned container block, like the original).
    pub fn get_block_tile_entity(&mut self, x: i32, y: i32, z: i32) -> Option<&mut TileEntity> {
        let (cx, cz) = (x >> 4, z >> 4);
        if !self.access_chunk(cx, cz) {
            return None;
        }
        let key = (x & 15, y, z & 15);
        if !self.chunk(cx, cz).tile_entities.contains_key(&key) {
            let id = self.chunk(cx, cz).block(x & 15, y, z & 15);
            if !props(id).container {
                return None;
            }
            behavior::on_block_added(self, id, x, y, z);
        }
        self.chunk_mut(cx, cz).tile_entities.get_mut(&key)
    }

    /// `setBlockTileEntity` → `Chunk.setChunkBlockTileEntity`
    pub fn set_block_tile_entity(&mut self, x: i32, y: i32, z: i32, te: TileEntity) {
        let (cx, cz) = (x >> 4, z >> 4);
        if !self.access_chunk(cx, cz) {
            return;
        }
        let c = self.chunk_mut(cx, cz);
        let id = c.block(x & 15, y, z & 15);
        if id != block::AIR && props(id).container {
            c.tile_entities.insert((x & 15, y, z & 15), te);
        }
    }

    /// `removeBlockTileEntity`
    pub fn remove_block_tile_entity(&mut self, x: i32, y: i32, z: i32) {
        let _ = self.get_block_tile_entity(x, y, z);
        let (cx, cz) = (x >> 4, z >> 4);
        if !self.access_chunk(cx, cz) {
            return;
        }
        let c = self.chunk_mut(cx, cz);
        if c.loaded {
            c.tile_entities.remove(&(x & 15, y, z & 15));
        }
    }

    // ------------------------------------------------------------ saving and ticking

    /// The `WorldInfo` to write to level.dat.
    pub fn level_info(&self) -> crate::storage::LevelInfo {
        let mut i = self.info.clone();
        i.seed = self.seed;
        i.spawn = self.spawn;
        i.time = self.world_time;
        i
    }

    /// `ChunkProvider.func_28062_b`: write one chunk and clear its modified flag.
    pub(crate) fn save_chunk(&mut self, cx: i32, cz: i32) {
        let time = self.world_time;
        let Some(storage) = self.storage.as_mut() else { return };
        let c = self.chunks.get_mut(&(cx, cz)).expect("saving unloaded chunk");
        c.last_save_time = time;
        match storage.save_chunk(c, time) {
            Ok(delta) => self.info.size_on_disk += delta,
            Err(e) => eprintln!("saving chunk {cx},{cz}: {e}"),
        }
        c.has_entities = !c.raw_entities.is_empty();
    }

    /// `Chunk.needsSaving`
    fn needs_saving(&self, cx: i32, cz: i32, all: bool) -> bool {
        let c = self.chunk(cx, cz);
        if all {
            if c.has_entities && self.world_time != c.last_save_time {
                return true;
            }
        } else if c.has_entities && self.world_time >= c.last_save_time + 600 {
            return true;
        }
        c.modified
    }

    /// `ChunkProvider.saveChunks`: all modified chunks, or at most 24 when `all` is false.
    pub fn save_chunks(&mut self, all: bool) -> bool {
        if self.storage.is_none() {
            return true;
        }
        let mut saved = 0;
        for i in 0..self.chunk_list.len() {
            let (cx, cz) = self.chunk_list[i];
            if self.needs_saving(cx, cz, all) {
                self.save_chunk(cx, cz);
                self.chunk_mut(cx, cz).modified = false;
                saved += 1;
                if saved == 24 && !all {
                    return false;
                }
            }
        }
        true
    }

    /// `World.saveLevel`: level.dat (refusing if another program took the session lock).
    pub fn save_level(&mut self) {
        let info = self.level_info();
        if let Some(s) = &self.storage {
            if !s.session_ok() {
                eprintln!("Failed to check session lock, aborting save");
                return;
            }
            if let Err(e) = s.save_level(&info) {
                eprintln!("saving level.dat: {e}");
            }
        }
    }

    /// `World.saveWorld(all, progress)`
    pub fn save_world(&mut self, all: bool) {
        if self.storage.is_some() {
            self.save_level();
            self.save_chunks(all);
        }
    }

    // ------------------------------------------------------------ helpers for features

    #[inline]
    pub fn material_is_solid(&mut self, x: i32, y: i32, z: i32) -> bool {
        mat(self.get_block_material(x, y, z)).is_solid
    }

    #[inline]
    pub fn material_is(&mut self, x: i32, y: i32, z: i32, m: u8) -> bool {
        self.get_block_material(x, y, z) == m
    }

    pub fn is_material_air(m: u8) -> bool {
        m == MAT_AIR
    }
}
