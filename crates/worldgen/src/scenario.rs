//! Chunk-loading sequences from the vanilla client, so population (which depends
//! on load order) can be reproduced and checked against the oracle.

use crate::block;
use crate::world::World;

impl World {
    /// `World.getInitialSpawnLocation`: random walk until the top block is sand.
    pub fn find_initial_spawn(&mut self) {
        let mut x = 0;
        let mut z = 0;
        while !self.can_coordinate_be_spawn(x, z) {
            x += self.rand.next_int(64) - self.rand.next_int(64);
            z += self.rand.next_int(64) - self.rand.next_int(64);
        }
        self.spawn = (x, 64, z);
    }

    /// `WorldProvider.canCoordinateBeSpawn`
    fn can_coordinate_be_spawn(&mut self, x: i32, z: i32) -> bool {
        self.get_first_uncovered_block(x, z) == block::SAND
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
