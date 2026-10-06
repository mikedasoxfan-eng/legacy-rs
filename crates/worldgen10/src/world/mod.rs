//! The parts of `World`, `Chunk`, `ChunkProvider` and `MetadataChunkBlock` that
//! world generation exercises, ported so that population replays exactly.
//!
//! Method names follow the Java (snake_cased) with the original name noted where
//! it was obfuscated. Call order matters everywhere in here: block hooks, light
//! scheduling and chunk loading all have side effects that later code observes.

pub mod behavior;
pub mod blocks10;
pub mod release10;
pub mod plants181;
mod blocks181;
pub mod beds181;
pub mod verify181;
mod light181;
pub mod chests181;
pub mod chunk;
pub mod cache181;
pub mod events;
pub mod java_hash;
pub mod light;
pub mod music;
pub mod server;
pub mod provider181;
pub mod legacy_cache181;
pub mod cache_verify181;
pub mod teleporter;
pub mod redstone;
pub mod rails;
pub mod dimension;
pub mod spawn181;
pub mod tick;
pub mod mchash181;
pub mod ticks_blocks;
pub mod tile;
pub mod tile_nbt181;
pub mod pistons;
pub mod notes;
mod tile_lifecycle;

use crate::block::{self, light_opacity, mat, material_of, props, MAT_AIR};
use crate::overworld::OverworldGenerator;
use crate::random::JavaRandom;
use chunk::{idx, Chunk, LightKind};
use light::LightUpdate;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::atomic::{AtomicI64, Ordering};

/// NextTickListEntry IDs survive World construction and dimension changes.
pub(crate) static NEXT_TICK_ENTRY_ID181: AtomicI64 = AtomicI64::new(0);
fn next_tick_entry_id181() -> i64 {
    NEXT_TICK_ENTRY_ID181.fetch_add(1, Ordering::Relaxed)
}


/// Which `java.util.Random` a block tick draws from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RandSel {
    /// The chunk generator RNG, shared by chunk creation and population.
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
impl TickKey {
    pub fn java_hash181(self)->i32 {
        self.x.wrapping_mul(1024*1024).wrapping_add(self.z.wrapping_mul(1024)).wrapping_add(self.y).wrapping_mul(256).wrapping_add(self.block)
    }
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
    x >= -30000000 && z >= -30000000 && x < 30000000 && z < 30000000
}

/// The `WorldProvider` selected when the world is constructed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dimension {
    Overworld,
    Nether,
    End,
}

impl Dimension {
    /// Compatibility name retained for the copied 1.8.1 callers. Dimension 1 is the End.
    #[allow(non_upper_case_globals)]
    pub const Sky: Self = Self::End;
    /// `WorldProvider.hasNoSky`
    pub fn has_no_sky(self) -> bool {
        self != Self::Overworld
    }
    /// `WorldProvider.isHellWorld`
    pub fn is_hell_world(self) -> bool {
        self != Self::Overworld
    }
    pub fn id(self) -> i32 { match self { Self::Nether => -1, Self::Overworld => 0, Self::Sky => 1 } }
    pub fn from_id(id:i32) -> Option<Self> { match id { -1=>Some(Self::Nether),0=>Some(Self::Overworld),1=>Some(Self::Sky),_=>None } }
    pub fn celestial_angle(self,time:i64,partial:f32)->f32 {match self {Self::Nether=>0.5,Self::Sky=>0.0,Self::Overworld=>tick::celestial_angle(time,partial)}}

    /// `WorldProvider.generateLightBrightnessTable` and the Hell override.
    pub fn light_brightness_table(self) -> [f32; 16] {
        // b1.8.1: the overworld minimum dropped from 0.05 to 0 (the lightmap adds its own floor).
        let ambient = if self == Self::Nether {
            0.1f32
        } else {
            0.0f32
        };
        let mut table = [0.0; 16];
        for i in 0..=15 {
            let darkness = 1.0f32 - i as f32 / 15.0f32;
            table[i] = (1.0 - darkness) / (darkness * 3.0 + 1.0) * (1.0 - ambient) + ambient;
        }
        table
    }
}

pub struct World {
    /// Explicit Block callbacks to IWorldAccess.markBlockRangeNeedsUpdate.
    /// Kept separate from sound/particle events, which have their own replay format.
    pub block_update_ranges181: Vec<[i32; 6]>,
    /// Opt-in observer; normal worlds do not retain a history of render requests.
    pub record_block_update_ranges181: bool,
    pub piston_extension_texture_override181: i32,
    pub maps:crate::maps::MapStorage,
    /// Ordered IWorldAccess callbacks since the last client tick.
    pub events: Vec<events::WorldEvent>,
    /// Call-local Random used by World.randomDisplayUpdates.
    pub display_seed_source: crate::entity::EntitySeedSource,
    pub seed: i64,
    pub terrain: OverworldGenerator,
    pub(crate) mineshafts: crate::structures::MineshaftGenerator,
    pub(crate) villages: crate::structures::village::VillageGenerator,
    pub(crate) nether_bridges10: crate::structures::nether10::NetherBridgeGenerator,
    pub(crate) strongholds: crate::structures::stronghold::StrongholdGenerator,
    pub(crate) structure_locations: crate::structures::locations::Locations,
    pub(crate) generator_epoch: u64,
    pub(crate) structure_rng_epoch: Option<u64>,
    pub dimension: Dimension,
    pub light_brightness_table: [f32; 16],
    nether: Option<crate::nether::NetherGenerator>,
    pub(crate) sky: Option<crate::end::EndGenerator>,
    pub chunks: ChunkMap,
    /// `World.rand`
    pub rand: JavaRandom,
    pub(crate) population_uses_world10: bool,
    /// Entity classes belong to the entities area; requests preserve generation decisions.
    pub villager_spawns10: Vec<crate::structures::village::VillagerSpawn10>,
    pub end_entity_spawns10: Vec<crate::end::EndEntitySpawn>,
    /// `World.lightingToUpdate`
    pub light_queue: Vec<LightUpdate>,
    /// `scheduledTickSet`: entry -> (scheduled time, `tickEntryID`).
    pub pending_ticks: HashMap<TickKey, (i64, i64)>,
    pub(crate) scheduled_tick_set10: java_hash::JavaHashSet<TickKey>,
    /// `scheduledTickTreeSet`, ordered like `NextTickListEntry.compareTo`.
    pub tick_order: std::collections::BTreeSet<(i64, i64, TickKey)>,
    pub world_time: i64,
    /// `World.scheduledUpdatesAreImmediate`
    pub immediate_updates: bool,
    /// `World.editingBlocks`
    pub editing_blocks: bool,
    /// `BlockRedstoneWire.wiresProvidePower` (false while a wire asks whether it is powered).
    pub wires_provide_power: bool,
    /// `BlockRedstoneWire.field_21031_b`: positions to notify after a propagation.
    pub wire_updates: java_hash::JavaHashSet<(i32, i32, i32)>,
    /// `BlockRedstoneTorch.torchUpdates` (recent flips, for burnout).
    pub torch_updates: Vec<redstone::TorchUpdate>,
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
    /// `checkSessionLock` failed (MinecraftException): another program opened this save.
    /// Nothing more is written; the client drops the world.
    pub session_conflict: bool,
    /// `WorldInfo` fields the world doesn't otherwise track (name, weather, saved player).
    pub info: crate::storage::LevelInfo,
    /// `ChunkProvider.chunkList`: loaded chunks in load order (`saveChunks` walks it).
    pub chunk_list: Vec<(i32, i32)>,
    /// Legacy multiplayer-server chunk handling; `None` uses the 1.8.1 singleplayer provider.
    pub server: Option<server::ServerChunks>,
    pub provider: provider181::ProviderChunks,
    /// `WorldServer.levelSaving`
    pub level_saving: bool,
    /// Chunks unloaded since the renderer last looked (drained by the client).
    pub unloaded: Vec<(i32, i32)>,
    pub raining_strength: f32,
    pub prev_raining_strength: f32,
    pub thundering_strength: f32,
    pub prev_thundering_strength: f32,
    /// `field_27168_F`: sky flash after lightning.
    pub lightning_sky_flash: i32,
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
    /// Entity IDs for simulated players; legacy camera entries have no ID.
    pub player_entities: Vec<Option<i32>>,
    positions_to_update: tick::JavaHashSet<(i32, i32)>,
    /// `BlockLeaves.adjacentTreeBlocks`: scratch kept between decay checks, as in the original.
    pub leaf_scratch: Option<Vec<i32>>,
    /// Lightning strikes (`EntityLightningBolt`), recorded until entities exist.
    pub lightning_strikes: Vec<(i32, i32, i32)>,
    pub entities: std::collections::BTreeMap<i32, crate::entity::Entity>,
    pub loaded_entity_list: Vec<i32>,
    pub unloaded_entity_list: Vec<i32>,
    pub spawn_eligible_chunks: java_hash::JavaHashSet<(i32, i32)>,
    pub chest_animations: HashMap<tile_lifecycle::TileId,chests181::ChestAnimation>,
    pub loaded_tile_entities: Vec<tile_lifecycle::TileId>,
    pub tile_objects: HashMap<tile_lifecycle::TileId, tile_lifecycle::TileObject>,
    pub next_tile_id: u64,
    pub piston_guard: [bool; 2],
    pub block_events: Vec<notes::BlockEvent>,
    pub scanning_tiles: bool,
    pub pending_tile_entities: Vec<tile_lifecycle::TileId>,
    /// World.field_34900_Q: unloaded tiles still get this pass before removal.
    pub unloaded_tile_entities181: Vec<tile_lifecycle::TileId>,
    pub(crate) tile_entity_removal10: Vec<tile_lifecycle::TileId>,
    pub dispenser_random: HashMap<(i32, i32, i32), JavaRandom>,
    pub container_drop_random: [JavaRandom; 3], // idle furnace, active furnace, dispenser
    pub keep_furnace_inventory: bool,
    pub container_sessions: HashMap<i32, crate::container::session::ContainerSession>,
    pub map_id: i32,
    pub container_effects: Vec<(i32, (i32, i32, i32), i32)>,
    pub spawner_animations: std::collections::HashMap<(i32, i32, i32), (f64, f64)>,
    pub weather_effects: Vec<crate::entity::Entity>,
    pub explosion_seed_source: crate::entity::EntitySeedSource,
    pub entity_seed_source: crate::entity::EntitySeedSource,
    pub chest_random: JavaRandom,
    pub math_random: JavaRandom,
    pub item_random: JavaRandom,
    pub next_entity_id: i32,
    pub difficulty: i32,
    /// World.multiplayerWorld: remote simulation guards in entity callbacks.
    pub multiplayer_world: bool,
    /// Release 1.0 World.field_35472_c, consulted by ItemBlock's top-layer guard.
    pub height10: i32,
    pub spawn_hostile_mobs: bool,
    pub spawn_peaceful_mobs: bool,
    pub block_bounds: [crate::entity::AxisAlignedBB; 256],
}

impl World {
    /// A world with a preset spawn (no spawn search), like loading `level.dat`.
    pub fn new(seed: i64, world_rand_seed: i64) -> Self {
        Self::with_dimension(seed, world_rand_seed, Dimension::Overworld)
    }

    /// Construct with `WorldProvider`/`WorldProviderHell`, before any chunks are loaded.
    pub fn with_dimension(seed: i64, world_rand_seed: i64, dimension: Dimension) -> Self {
        let mut terrain = OverworldGenerator::new(seed);
        if dimension == Dimension::Nether {
            terrain.biome_source = crate::biome::BiomeSource::hell();
        } else if dimension == Dimension::Sky {
            terrain.biome_source = crate::biome::BiomeSource::sky();
        }
        World {
            events: Vec::new(),
            maps:Default::default(),
            display_seed_source: crate::entity::EntitySeedSource::Time,
            seed,
            dimension,
            light_brightness_table: dimension.light_brightness_table(),
            nether: if dimension == Dimension::Nether {
                Some(crate::nether::NetherGenerator::new(seed))
            } else {
                None
            },
            terrain,
            sky: if dimension == Dimension::Sky {Some(crate::end::EndGenerator::new(seed))} else {None},
            mineshafts: Default::default(),
            villages: Default::default(),
            nether_bridges10: Default::default(),
            strongholds: Default::default(),
            structure_locations: Default::default(),
            generator_epoch: 0,
            structure_rng_epoch: None,
            chunks: ChunkMap::default(),
            rand: JavaRandom::new(world_rand_seed),
            population_uses_world10: false,
            villager_spawns10: Vec::new(),
            end_entity_spawns10: Vec::new(),
            light_queue: Vec::new(),
            pending_ticks: HashMap::new(),
            scheduled_tick_set10: Default::default(),
            tick_order: Default::default(),
            world_time: 0,
            immediate_updates: false,
            editing_blocks: false,
            wires_provide_power: true,
            wire_updates: Default::default(),
            torch_updates: Vec::new(),
            sand_fall_instantly: false,
            leaves_opaque: true,
            flow: [FlowState::default(); 2],
            spawn: (0, 0, 0),
            dropped_items: Vec::new(),
            falling_sand: Vec::new(),
            mod_count: 0,
            storage: None,
            session_conflict: false,
            info: crate::storage::LevelInfo::new(seed, "world"),
            chunk_list: Vec::new(),
            server: None,
            provider: Default::default(),
            level_saving: false,
            unloaded: Vec::new(),
            raining_strength: 0.0,
            prev_raining_strength: 0.0,
            thundering_strength: 0.0,
            prev_thundering_strength: 0.0,
            lightning_sky_flash: 0,
            lightning_flash: 0,
            skylight_subtracted: if dimension == Dimension::Nether { 11 } else { 0 },
            renderers_dirty: false,
            block_update_ranges181: Vec::new(),
            record_block_update_ranges181: false,
            piston_extension_texture_override181: -1,
            update_lcg: 0,
            sound_counter: 0,
            players: Vec::new(),
            player_entities: Vec::new(),
            positions_to_update: Default::default(),
            leaf_scratch: None,
            lightning_strikes: Vec::new(),
            entities: Default::default(),
            loaded_entity_list: Vec::new(),
            unloaded_entity_list: Vec::new(),
            weather_effects: Vec::new(),
            spawn_eligible_chunks: Default::default(),
            chest_animations: HashMap::new(),
            loaded_tile_entities: Vec::new(),
            tile_objects: HashMap::new(),
            next_tile_id: 0,
            piston_guard: [false;2],
            block_events: Vec::new(),
            scanning_tiles: false,
            pending_tile_entities: Vec::new(),
            unloaded_tile_entities181: Vec::new(),
            tile_entity_removal10: Vec::new(),
            dispenser_random: HashMap::new(),
            container_drop_random: std::array::from_fn(|_| {
                JavaRandom::new(crate::entity::EntitySeedSource::Time.next_seed())
            }),
            keep_furnace_inventory: false,
            container_sessions: HashMap::new(),
            map_id: -1,
            container_effects: Vec::new(),
            spawner_animations: Default::default(),
            explosion_seed_source: crate::entity::EntitySeedSource::Time,
            entity_seed_source: crate::entity::EntitySeedSource::Time,
            chest_random: JavaRandom::new(crate::entity::EntitySeedSource::Time.next_seed()),
            item_random: JavaRandom::new(crate::entity::EntitySeedSource::Time.next_seed()),
            math_random: JavaRandom::new(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_nanos() as i64,
            ),
            next_entity_id: 0,
            difficulty: 2,
            multiplayer_world: false,
            height10: 128,
            spawn_hostile_mobs: true,
            spawn_peaceful_mobs: true,
            block_bounds: {let mut bounds=crate::entity::block_table::INITIAL_BOUNDS;for id in 110..=122 {if let Some(b)=blocks10::initial_bounds(id){bounds[id as usize]=b;}}bounds},
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
        if self.server.is_none()
            && (!(-1875004..1875004).contains(&cx) || !(-1875004..1875004).contains(&cz))
        {
            return false;
        }
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
            RandSel::Populate => self.prand(),
            RandSel::World => &mut self.rand,
        }
    }

    /// The population RNG (`ChunkProviderGenerate.rand` / `ChunkProviderHell.hellRNG`).
    #[inline]
    pub fn prand(&mut self) -> &mut JavaRandom {
        if self.population_uses_world10 { return &mut self.rand; }
        if let Some(gen) = &mut self.sky {return gen.rand_mut();}
        match &mut self.nether {
            Some(gen) => gen.rand_mut(),
            None => self.terrain.rand_mut(),
        }
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
        if self.server.is_none() {
            self.provider.cancel(cx, cz);
            if !(-1875004..1875004).contains(&cx) || !(-1875004..1875004).contains(&cz) {
                return;
            }
        }
        let saved = self
            .storage
            .as_mut()
            .and_then(|s| s.load_chunk_for_dimension(cx, cz, self.dimension));
        if let Some(mut c) = saved {
            if let Some(ticks) = &c.tile_ticks10 {
                for tick in ticks {
                    self.schedule_block_update_from_load(tick.int("x"), tick.int("y"), tick.int("z"), tick.int("i"), tick.int("t"));
                }
            }
            c.has_no_sky10 = self.dimension.has_no_sky();
            c.last_save_time = self.world_time;
            c.loaded = true;
            self.chunks.insert((cx, cz), Box::new(c));
            self.chunk_list.push((cx, cz));
            self.mod_count += 1;
            let v = self.mod_count;
            self.chunk_mut(cx, cz).version = v;
            self.load_chunk_entities(cx, cz);
        } else {
            // ChunkProviderGenerate/ChunkProviderHell.provideChunk
            let raw = if let Some(gen) = &mut self.sky {gen.generate(cx,cz)} else {match &mut self.nether {
                Some(gen) => gen.generate(cx, cz),
                None => self.terrain.generate(cx, cz),
            }};
            if self.dimension == Dimension::Overworld && self.info.map_features {
                self.mineshafts.generate(self.seed,cx,cz);
                self.villages.generate(self.seed,cx,cz,&self.terrain.biome_source,&mut self.rand);
                self.strongholds.generate(self.seed, cx, cz, &self.terrain.biome_source);
                self.structure_locations.strongholds = self.strongholds.candidates;
            }
            if self.dimension == Dimension::Nether { self.nether_bridges10.generate(self.seed,cx,cz); }
            self.generator_epoch=self.generator_epoch.wrapping_add(1);
            let mut c = Box::new(Chunk::new(cx, cz, raw.blocks));
            c.has_no_sky10 = self.dimension.has_no_sky();
            // ChunkProviderHell returns a new Chunk without func_1024_c. Its
            // height/skylight arrays remain zero until individual block writes.
            if self.dimension != Dimension::Nether {
                c.init_height_and_sky_for_dimension(self.dimension);
                // func_1024_c's neighbour pass runs before the chunk is in the map, so
                // its own columns read as height 0 through the world.
                for x in 0..16 {
                    for z in 0..16 {
                        let h = c.height(x, z);
                        self.update_neighbor_heights(cx * 16 + x, cz * 16 + z, h);
                    }
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
            && self.chunk_exists(cx - 1, cz + 1)
        {
            self.populate_chunk(cx - 1, cz);
        }
        if self.chunk_exists(cx, cz - 1)
            && !self.chunk(cx, cz - 1).populated
            && self.chunk_exists(cx + 1, cz - 1)
            && self.chunk_exists(cx + 1, cz - 1)
            && self.chunk_exists(cx + 1, cz)
        {
            self.populate_chunk(cx, cz - 1);
        }
        if self.chunk_exists(cx - 1, cz - 1)
            && !self.chunk(cx - 1, cz - 1).populated
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
            if self.dimension == Dimension::Nether {
                crate::nether::populate::populate(self, cx, cz);
            } else if self.dimension == Dimension::Sky {
                crate::end::populate(self,cx,cz);
            } else {
                crate::populate::populate(self, cx, cz);
            }
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
        let changed=self.chunk_set_block_with_metadata(x >> 4, z >> 4, x & 15, y, z & 15, id, meta);
        self.update_all_light181(x,y,z);
        changed
    }

    pub fn set_block(&mut self, x: i32, y: i32, z: i32, id: u8) -> bool {
        if !in_xz_bounds(x, z) || !(0..128).contains(&y) {
            return false;
        }
        if !self.access_chunk(x >> 4, z >> 4) {
            return true; // EmptyChunk.setBlockID
        }
        let changed=self.chunk_set_block(x >> 4, z >> 4, x & 15, y, z & 15, id);
        self.update_all_light181(x,y,z);
        changed
    }

    pub fn set_block_metadata(&mut self, x: i32, y: i32, z: i32, meta: i32) -> bool {
        if !in_xz_bounds(x, z) || !(0..128).contains(&y) {
            return false;
        }
        if !self.access_chunk(x >> 4, z >> 4) {
            return false; // EmptyChunk.setBlockMetadata
        }
        self.chunk_mut(x >> 4, z >> 4).modified = true;
        if self.chunk(x >> 4, z >> 4).data.get(x & 15, y, z & 15) == meta {
            return false;
        }
        self.chunk_mut(x >> 4, z >> 4)
            .data
            .set(x & 15, y, z & 15, meta);
        if crate::block::props(self.get_block_id(x,y,z)).container {
            self.get_block_tile_entity(x,y,z);
            if let Some(id)=self.chunk(x>>4,z>>4).tile_entity_ids.get(&(x&15,y,z&15)).copied() {
                self.reset_tile_cache181(id);
                self.tile_objects.get_mut(&id).unwrap().cached_metadata=meta;
            }
        }
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

    pub fn set_block_and_metadata_with_notify(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        id: u8,
        meta: i32,
    ) -> bool {
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
        if !self.editing_blocks && !self.multiplayer_world {
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
        let column=(z<<4|x) as usize;
        if y>=c.precipitation_height[column]-1 {c.precipitation_height[column]=-999;}
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
                self.relight_column181(cx, cz, x, y + 1, z);
            }
        } else if y == height - 1 {
            self.relight_column181(cx, cz, x, y, z);
        }
        self.schedule_lighting_update(LightKind::Sky, wx, y, wz, wx, y, wz);
        self.schedule_lighting_update(LightKind::Block, wx, y, wz, wx, y, wz);
        self.chunk_mut(cx,cz).skylight_occlusion[(x+z*16) as usize]=true;
        self.chunk_mut(cx,cz).skylight_pending10 = true;
        if id != block::AIR && !self.multiplayer_world {
            behavior::on_block_added(self, id, wx, y, wz);
        }
        self.refresh_changed_block_tile181(cx, cz, x, y, z, id, old);
        true
    }

    /// `Chunk.setBlockIDWithMetadata`
    #[allow(clippy::too_many_arguments)]
    fn chunk_set_block_with_metadata(
        &mut self,
        cx: i32,
        cz: i32,
        x: i32,
        y: i32,
        z: i32,
        id: u8,
        meta: i32,
    ) -> bool {
        self.chunk_set_block_with_metadata_raw181(cx,cz,x,y,z,id as i32,meta).expect("Chunk.setBlockIDWithMetadata")
    }
    fn chunk_set_block_with_metadata_raw181(&mut self,cx:i32,cz:i32,x:i32,y:i32,z:i32,raw_id:i32,meta:i32) -> Result<bool,tile_nbt181::TileError181> {
        let id=raw_id as u8;
        let c = self.chunk_mut(cx, cz);
        let column=(z<<4|x) as usize;
        if y>=c.precipitation_height[column]-1 {c.precipitation_height[column]=-999;}
        let height = c.height(x, z);
        let old = c.blocks[idx(x, y, z)];
        if old as i32 == raw_id && c.data.get(x, y, z) == meta {
            return Ok(false);
        }
        let wx = cx * 16 + x;
        let wz = cz * 16 + z;
        c.blocks[idx(x, y, z)] = id;
        let failure = if !(0..256).contains(&raw_id) {Some(tile_nbt181::TileError181::ArrayIndexOutOfBounds)}
            else if raw_id!=0 && !block::props(id).exists && !self.multiplayer_world {Some(tile_nbt181::TileError181::NullPointer)}else{None};
        if failure.is_none() {self.touch(cx,cz);}
        if old != block::AIR {
            if !self.multiplayer_world {
                behavior::on_block_removal(self, old, wx, y, wz);
            } else if block::props(old).container {
                self.remove_block_tile_entity(wx, y, wz);
            }
        }
        self.chunk_mut(cx, cz).data.set(x, y, z, meta);
        if !self.dimension.has_no_sky() {
            if light_opacity(id) != 0 {
                if y >= height {
                    self.relight_column181(cx, cz, x, y + 1, z);
                }
            } else if y == height - 1 {
                self.relight_column181(cx, cz, x, y, z);
            }
            self.schedule_lighting_update(LightKind::Sky, wx, y, wz, wx, y, wz);
        }
        self.schedule_lighting_update(LightKind::Block, wx, y, wz, wx, y, wz);
        self.chunk_mut(cx,cz).skylight_occlusion[(x+z*16) as usize]=true;
        self.chunk_mut(cx,cz).skylight_pending10 = true;
        self.chunk_mut(cx, cz).data.set(x, y, z, meta);
        if id != block::AIR && !self.multiplayer_world {
            if let Some(e)=failure {return Err(e);}
            behavior::on_block_added(self, id, wx, y, wz);
        }
        // The original indexes blocksList even remotely for instanceof.
        if let Some(e)=failure {return Err(e);}
        self.refresh_changed_block_tile181(cx, cz, x, y, z, id, old);
        Ok(true)
    }

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
        self.chunk(x >> 4, z >> 4)
            .can_block_see_the_sky(x & 15, y, z & 15)
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
        self.chunk(x >> 4, z >> 4)
            .block_light_value(x & 15, y, z & 15, 0)
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
        self.chunk(x >> 4, z >> 4)
            .can_block_see_the_sky(x & 15, y, z & 15)
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
        block::visual181::opaque_instance(id,!self.leaves_opaque)
    }

    pub fn block_texture181(&mut self,id:u8,x:i32,y:i32,z:i32,side:usize)->Result<i32,()> {
        let meta=self.get_block_metadata(x,y,z) as usize;let bounds=self.block_bounds[id as usize];
        block::visual181::texture_at_with_bounds(id,side,meta,!self.leaves_opaque,x,y,z,bounds,|x,y,z|self.get_block_id(x,y,z))
    }
    pub fn should_side_render181(&mut self,id:u8,x:i32,y:i32,z:i32,side:i32)->bool {
        block::visual181::should_side_render(id,side,self.block_bounds[id as usize],!self.leaves_opaque,x,y,z,|x,y,z|self.get_block_id(x,y,z))
    }

    // ------------------------------------------------------------ light

    pub fn get_saved_light_value(&self, kind: LightKind, x: i32, y: i32, z: i32) -> i32 {
        let y = y.clamp(0, 127);
        if in_xz_bounds(x, z) {
            if !self.chunk_exists(x >> 4, z >> 4) {
                return 0;
            }
            self.chunk(x >> 4, z >> 4)
                .saved_light(kind, x & 15, y, z & 15)
        } else {
            kind.default_value()
        }
    }

    pub fn set_light_value(&mut self, kind: LightKind, x: i32, y: i32, z: i32, v: i32) {
        if in_xz_bounds(x, z) && (0..128).contains(&y) && self.chunk_exists(x >> 4, z >> 4) {
            self.chunk_mut(x >> 4, z >> 4)
                .set_light(kind, x & 15, y, z & 15, v);
            self.touch(x >> 4, z >> 4);
        }
    }

    pub fn neighbor_light_propagation_changed(
        &mut self,
        kind: LightKind,
        x: i32,
        y: i32,
        z: i32,
        mut v: i32,
    ) {
        if self.dimension.has_no_sky() && kind == LightKind::Sky {
            return;
        }
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
    pub fn schedule_lighting_update(
        &mut self,
        kind: LightKind,
        x0: i32,
        y0: i32,
        z0: i32,
        x1: i32,
        y1: i32,
        z1: i32,
    ) {
        // Kept for callers shared with the old generation pipeline. Vanilla b1.8.1 is empty.
        let _=(kind,x0,y0,z0,x1,y1,z1);
    }

    /// `updatingLighting`: process up to ~500 queued updates; true if more remain.
    pub fn updating_lighting(&mut self) -> bool {
        false
    }

    pub fn drain_lighting(&mut self) {
        while self.updating_lighting() {}
    }

    // ------------------------------------------------------------ ticks

    /// `scheduleBlockUpdate`
    pub fn schedule_block_update(&mut self, x: i32, y: i32, z: i32, id: u8, delay: i32) {
        // new NextTickListEntry(...) takes an id from the global counter every time.
        let entry_id = next_tick_entry_id181();
        let r = 8;
        if self.immediate_updates {
            if self.check_chunks_exist(x - r, y - r, z - r, x + r, y + r, z + r) {
                let cur = self.get_block_id(x, y, z);
                if cur == id && cur > 0 {
                    behavior::update_tick(self, cur, x, y, z, RandSel::World);
                }
            }
        } else if self.check_chunks_exist(x - r, y - r, z - r, x + r, y + r, z + r) {
            let key = TickKey {
                x,
                y,
                z,
                block: id as i32,
            };
            let t = if id>0 {self.world_time.wrapping_add(delay as i64)} else {0};
            if let std::collections::hash_map::Entry::Vacant(e) = self.pending_ticks.entry(key) {
                e.insert((t, entry_id));
                self.scheduled_tick_set10.insert(key.java_hash181(), key);
                self.tick_order.insert((t, entry_id, key));
            }
        }
    }

    // ------------------------------------------------------------ tile entities

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
        if self.storage.is_none() {
            return;
        }
        let tags = self.chunk_entity_nbt(cx, cz);
        let has_entities = !tags.is_empty();
        let ticks = self.pending_chunk_ticks(cx, cz, false).map(|ticks| ticks.into_iter().map(|(key, scheduled)| {
            let mut tag = crate::nbt::Compound::new();
            tag.put("i", crate::nbt::Tag::Int(key.block));
            tag.put("x", crate::nbt::Tag::Int(key.x));
            tag.put("y", crate::nbt::Tag::Int(key.y));
            tag.put("z", crate::nbt::Tag::Int(key.z));
            tag.put("t", crate::nbt::Tag::Int(scheduled.wrapping_sub(time) as i32));
            tag
        }).collect());
        let storage = self.storage.as_mut().unwrap();
        let c = self
            .chunks
            .get_mut(&(cx, cz))
            .expect("saving unloaded chunk");
        let raw_ticks = std::mem::replace(&mut c.tile_ticks10, ticks);
        let raw = std::mem::replace(&mut c.raw_entities, tags);
        c.last_save_time = time;
        match storage.save_chunk(c, time) {
            Ok(delta) => self.info.size_on_disk += delta,
            Err(e) => eprintln!("saving chunk {cx},{cz}: {e}"),
        }
        c.raw_entities = raw;
        c.tile_ticks10 = raw_ticks;
        c.has_entities = has_entities;
    }

    /// `Chunk.needsSaving`
    fn needs_saving(&self, cx: i32, cz: i32, all: bool) -> bool {
        let c = self.chunk(cx, cz);
        if c.never_save {
            return false;
        }
        if all {
            if c.has_entities && self.world_time != c.last_save_time {
                return true;
            }
        } else if c.has_entities && self.world_time >= c.last_save_time.wrapping_add(600) {
            return true;
        }
        c.modified
    }

    /// `ChunkProvider.saveChunks`: all modified chunks, or at most 24 when `all` is false.
    pub fn save_chunks(&mut self, all: bool) -> bool {
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
        if let Some(n) = self
            .player_entities
            .iter()
            .flatten()
            .find_map(|id| self.entities.get(id).and_then(|e| e.player_nbt()))
        {
            self.info.player = Some(n);
        }
        let info = self.level_info();
        if let Some(s) = &self.storage {
            if self.session_conflict || !s.session_ok() {
                eprintln!("Failed to check session lock, aborting save");
                self.session_conflict = true;
                return;
            }
            if let Err(e) = s.save_level(&info) {
                eprintln!("saving level.dat: {e}");
            }
            if let Err(e)=self.maps.save(&s.dir){eprintln!("saving maps: {e}");}
        }
    }

    /// `World.saveWorld(all, progress)`
    pub fn save_world(&mut self, all: bool) {
        if self.storage.is_some() {
            self.save_level();
            if !self.session_conflict {
                self.save_chunks(all);
            }
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
