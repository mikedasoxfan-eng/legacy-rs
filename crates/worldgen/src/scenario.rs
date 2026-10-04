//! Chunk-loading sequences from the vanilla client, so population (which depends
//! on load order) can be reproduced and checked against the oracle.

use crate::block;
use crate::world::World;

impl World {
    /// `World.getInitialSpawnLocation` (`generateSpawnPoint` on the server, which allows
    /// chunk loading while it searches): random walk until the top block is sand.
    pub fn find_initial_spawn(&mut self) {
        if let Some(s) = &mut self.server {
            s.world_override = true;
        }
        let mut x = 0;
        let mut z = 0;
        while !self.can_coordinate_be_spawn(x, z) {
            x += self.rand.next_int(64) - self.rand.next_int(64);
            z += self.rand.next_int(64) - self.rand.next_int(64);
        }
        self.spawn = (x, 64, z);
        if let Some(s) = &mut self.server {
            s.world_override = false;
        }
    }

    /// `WorldProvider.canCoordinateBeSpawn`
    fn can_coordinate_be_spawn(&mut self, x: i32, z: i32) -> bool {
        let id = self.get_first_uncovered_block(x, z);
        if self.dimension.is_hell_world() {
            id != block::BEDROCK && id != block::AIR && block::opaque_cube_lookup(id)
        } else {
            id == block::SAND
        }
    }

    /// `World.getFirstUncoveredBlock`
    pub fn get_first_uncovered_block(&mut self, x: i32, z: i32) -> u8 {
        let mut y = 63;
        while !self.is_air_block(x, y + 1, z) {
            y += 1;
        }
        self.get_block_id(x, y, z)
    }

    /// `Minecraft.func_6255_d` ("Building terrain"): touch a 17x17 chunk grid
    /// around spawn, draining the lighting queue after each.
    pub fn preload_spawn_area(&mut self) {
        let (sx, _, sz) = self.spawn;
        let r = 128;
        let mut dx = -r;
        while dx <= r {
            let mut dz = -r;
            while dz <= r {
                self.get_block_id(sx + dx, 64, sz + dz);
                self.drain_lighting();
                dz += 16;
            }
            dx += 16;
        }
    }

    /// The full vanilla new-world flow: spawn search, then preload.
    pub fn new_vanilla(seed: i64, world_rand_seed: i64) -> World {
        let mut w = World::new(seed, world_rand_seed);
        w.find_initial_spawn();
        w.preload_spawn_area();
        w
    }

    /// `MinecraftServer.initWorld`: load a 25x25 chunk start region around spawn,
    /// draining lighting after each chunk.
    pub fn prepare_server_spawn_area(&mut self) {
        let (sx, _, sz) = self.spawn;
        let r = 196;
        let mut dx = -r;
        while dx <= r {
            let mut dz = -r;
            while dz <= r {
                self.server_load_chunk((sx + dx) >> 4, (sz + dz) >> 4);
                self.drain_lighting();
                dz += 16;
            }
            dx += 16;
        }
    }

    /// Open a world: from `dir` if it has a level.dat (`seed` is then ignored), otherwise
    /// a new one created with the singleplayer or server flow. `world_rand_seed` stands in
    /// for the unseeded `World.rand`.
    pub fn open(
        dir: Option<&std::path::Path>,
        seed: i64,
        world_rand_seed: i64,
        server: bool,
    ) -> std::io::Result<(World, bool)> {
        let storage = match dir {
            Some(d) => Some(crate::storage::Storage::open(d)?),
            None => None,
        };
        let existing = storage.as_ref().and_then(|s| s.load_level());
        let seed = existing.as_ref().map_or(seed, |i| i.seed);
        let mut w = World::new(seed, world_rand_seed);
        if server {
            w.server = Some(Default::default());
        }
        w.storage = storage;
        let is_new = existing.is_none();
        if let Some(info) = existing {
            w.spawn = info.spawn;
            w.world_time = info.time;
            w.info = info;
            w.init_weather_and_light();
        } else {
            if let Some(d) = dir {
                w.info.name = d.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| "world".into());
            }
            w.init_weather_and_light();
            w.find_initial_spawn();
        }
        if server {
            w.prepare_server_spawn_area();
        } else {
            w.preload_spawn_area();
        }
        // Both are unseeded in vanilla (`new Random().nextInt()` and a World.rand draw made
        // before anything else); take them from a side generator so World.rand stays put.
        let mut side = crate::random::JavaRandom::new(world_rand_seed ^ 0x5DEECE66D);
        w.update_lcg = side.next_int_unbounded();
        w.sound_counter = side.next_int(12000);
        if is_new && !server {
            // Minecraft.changeWorld saves a freshly generated world straight away
            // (the server only saves when it shuts down).
            w.save_world(true);
        }
        Ok((w, is_new))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightMode {
    Each,
    End,
    None,
}

/// Load chunks x-major over a rectangle (matches the oracle's `grid` mode).
pub fn grid(w: &mut World, x0: i32, z0: i32, x1: i32, z1: i32, light: LightMode) {
    for cx in x0..=x1 {
        for cz in z0..=z1 {
            w.load_chunk(cx, cz);
            if light == LightMode::Each {
                w.drain_lighting();
            }
        }
    }
    if light == LightMode::End {
        w.drain_lighting();
    }
}
