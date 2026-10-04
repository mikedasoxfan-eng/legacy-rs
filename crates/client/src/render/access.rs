//! `ChunkCache`: the read-only world view `RenderBlocks` uses for one chunk,
//! covering it and its 8 neighbours.

use super::{brightness_table, rprops};
use std::cell::RefCell;
use worldgen::block::{self, material_of};
use worldgen::world::chunk::Chunk;
use worldgen::{BiomeSource, World};

pub struct Access<'a> {
    pub world: &'a World,
    pub fancy: bool,
    cx0: i32,
    cz0: i32,
    chunks: [[Option<&'a Chunk>; 3]; 3],
    /// `World.skylightSubtracted`
    pub sky_sub: i32,
    table: [f32; 16],
    /// Shared climate sampler for biome tints (`func_4069_a(x, z, 1, 1)`).
    biomes: &'a RefCell<BiomeSource>,
    clim: RefCell<Vec<worldgen::Biome>>,
}

impl<'a> Access<'a> {
    pub fn new(world: &'a World, cx: i32, cz: i32, sky_sub: i32, biomes: &'a RefCell<BiomeSource>) -> Self {
        let mut chunks = [[None; 3]; 3];
        for (i, row) in chunks.iter_mut().enumerate() {
            for (j, c) in row.iter_mut().enumerate() {
                *c = world.chunks.get(&(cx - 1 + i as i32, cz - 1 + j as i32)).map(|b| &**b);
            }
        }
        Access {
            world,
            fancy: true,
            cx0: cx - 1,
            cz0: cz - 1,
            chunks,
            sky_sub,
            table: brightness_table(),
            biomes,
            clim: RefCell::new(Vec::new()),
        }
    }

    #[inline]
    fn chunk(&self, x: i32, z: i32) -> Option<&'a Chunk> {
        let i = (x >> 4) - self.cx0;
        let j = (z >> 4) - self.cz0;
        if (0..3).contains(&i) && (0..3).contains(&j) {
            self.chunks[i as usize][j as usize]
        } else {
            None
        }
    }

    #[inline]
    pub fn block(&self, x: i32, y: i32, z: i32) -> u8 {
        if !(0..128).contains(&y) {
            return 0;
        }
        self.chunk(x, z).map_or(0, |c| c.block(x & 15, y, z & 15))
    }

    #[inline]
    pub fn meta(&self, x: i32, y: i32, z: i32) -> i32 {
        if !(0..128).contains(&y) {
            return 0;
        }
        self.chunk(x, z).map_or(0, |c| c.data.get(x & 15, y, z & 15))
    }

    #[inline]
    pub fn material(&self, x: i32, y: i32, z: i32) -> u8 {
        material_of(self.block(x, y, z))
    }

    /// `isBlockOpaqueCube` (instance `isOpaqueCube()`, Fancy graphics).
    #[inline]
    pub fn is_opaque_cube(&self, x: i32, y: i32, z: i32) -> bool {
        let id = self.block(x, y, z);
        rprops(id).exists && (rprops(id).opaque || id == block::LEAVES && !self.fancy)
    }

    /// `getLightValueExt(x, y, z, true)`
    pub fn light_value(&self, x: i32, y: i32, z: i32) -> i32 {
        self.light_value_ext(x, y, z, true)
    }

    fn light_value_ext(&self, x: i32, y: i32, z: i32, special: bool) -> i32 {
        if !(x >= -32000000 && z >= -32000000 && x < 32000000 && z <= 32000000) {
            return 15;
        }
        if special {
            let id = self.block(x, y, z);
            // stairSingle, tilledField, stairCompactPlanks, stairCompactCobblestone
            if matches!(id, 44 | 60 | 53 | 67) {
                let mut v = self.light_value_ext(x, y + 1, z, false);
                for n in [
                    self.light_value_ext(x + 1, y, z, false),
                    self.light_value_ext(x - 1, y, z, false),
                    self.light_value_ext(x, y, z + 1, false),
                    self.light_value_ext(x, y, z - 1, false),
                ] {
                    if n > v {
                        v = n;
                    }
                }
                return v;
            }
        }
        if y < 0 {
            return 0;
        }
        if y >= 128 {
            return (15 - self.sky_sub).max(0);
        }
        match self.chunk(x, z) {
            Some(c) => c.block_light_value(x & 15, y, z & 15, self.sky_sub),
            None => 0,
        }
    }

    /// `getBrightness(x, y, z, minLight)`
    #[inline]
    pub fn brightness(&self, x: i32, y: i32, z: i32, min_light: i32) -> f32 {
        let mut l = self.light_value(x, y, z);
        if l < min_light {
            l = min_light;
        }
        self.table[l as usize]
    }

    /// `getLightBrightness(x, y, z)`
    #[inline]
    pub fn light_brightness(&self, x: i32, y: i32, z: i32) -> f32 {
        self.table[self.light_value(x, y, z) as usize]
    }

    /// Temperature and humidity at a column, as `func_4069_a(x, z, 1, 1)` leaves them.
    pub fn climate(&self, x: i32, z: i32) -> (f64, f64) {
        let mut bs = self.biomes.borrow_mut();
        let mut out = self.clim.borrow_mut();
        bs.biomes(&mut out, x, z, 1, 1);
        (bs.temperature[0], bs.humidity[0])
    }

    pub fn is_air(&self, x: i32, y: i32, z: i32) -> bool {
        self.block(x, y, z) == block::AIR
    }
}
