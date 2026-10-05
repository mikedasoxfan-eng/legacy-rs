//! `ChunkProviderGenerate.populate`: decorates the 16x16 area offset by +8,+8
//! from chunk (cx, cz). Uses the generator's shared RNG, which nested chunk
//! loads reseed mid-way exactly as in the original.

use crate::biome::Biome;
use crate::block;
use crate::features::{self, TreeKind};
use crate::world::World;

pub fn populate(w: &mut World, cx: i32, cz: i32) {
    w.sand_fall_instantly = true;
    let x0 = cx * 16;
    let z0 = cz * 16;
    let biome = w.terrain.biome_source.biome_at(x0 + 16, z0 + 16);
    let seed = w.seed;
    let r = w.prand();
    r.set_seed(seed);
    let mx = r.next_long() / 2 * 2 + 1;
    let mz = r.next_long() / 2 * 2 + 1;
    r.set_seed((cx as i64).wrapping_mul(mx).wrapping_add((cz as i64).wrapping_mul(mz)) ^ seed);

    if w.prand().next_int(4) == 0 {
        let x = x0 + w.prand().next_int(16) + 8;
        let y = w.prand().next_int(128);
        let z = z0 + w.prand().next_int(16) + 8;
        features::lakes(w, block::WATER_STILL, x, y, z);
    }
    if w.prand().next_int(8) == 0 {
        let x = x0 + w.prand().next_int(16) + 8;
        let b = w.prand().next_int(120) + 8;
        let y = w.prand().next_int(b);
        let z = z0 + w.prand().next_int(16) + 8;
        if y < 64 || w.prand().next_int(10) == 0 {
            features::lakes(w, block::LAVA_STILL, x, y, z);
        }
    }
    for _ in 0..8 {
        let x = x0 + w.prand().next_int(16) + 8;
        let y = w.prand().next_int(128);
        let z = z0 + w.prand().next_int(16) + 8;
        features::dungeon(w, x, y, z);
    }
    for _ in 0..10 {
        let (x, y, z) = ore_pos(w, x0, z0, 128);
        features::clay(w, 32, x, y, z);
    }
    let ores: [(u8, i32, i32, i32); 6] = [
        (block::DIRT, 32, 20, 128),
        (block::GRAVEL, 32, 10, 128),
        (block::ORE_COAL, 16, 20, 128),
        (block::ORE_IRON, 8, 20, 64),
        (block::ORE_GOLD, 8, 2, 32),
        (block::ORE_REDSTONE, 7, 8, 16),
    ];
    for (ore, size, count, max_y) in ores {
        for _ in 0..count {
            let (x, y, z) = ore_pos(w, x0, z0, max_y);
            features::minable(w, ore, size, x, y, z);
        }
    }
    {
        let (x, y, z) = ore_pos(w, x0, z0, 16);
        features::minable(w, block::ORE_DIAMOND, 7, x, y, z);
    }
    {
        let x = x0 + w.prand().next_int(16);
        let y = w.prand().next_int(16) + w.prand().next_int(16);
        let z = z0 + w.prand().next_int(16);
        features::minable(w, block::ORE_LAPIS, 6, x, y, z);
    }

    // Trees
    let s = 0.5;
    let noise = w.terrain.forest.sample_2d(x0 as f64 * s, z0 as f64 * s);
    let density = ((noise / 8.0 + w.prand().next_double() * 4.0 + 4.0) / 3.0) as i32;
    let mut trees = 0;
    if w.prand().next_int(10) == 0 {
        trees += 1;
    }
    match biome {
        Biome::Forest | Biome::Rainforest | Biome::Taiga => trees += density + 5,
        Biome::SeasonalForest => trees += density + 2,
        Biome::Desert | Biome::Tundra | Biome::Plains => trees -= 20,
        _ => {}
    }
    for _ in 0..trees {
        let x = x0 + w.prand().next_int(16) + 8;
        let z = z0 + w.prand().next_int(16) + 8;
        let kind = tree_for_biome(w, biome);
        let y = w.get_height_value(x, z);
        features::tree(w, kind, x, y, z);
    }

    // Yellow flowers
    let n = match biome {
        Biome::Forest | Biome::Taiga => 2,
        Biome::SeasonalForest => 4,
        Biome::Plains => 3,
        _ => 0,
    };
    for _ in 0..n {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::flowers(w, block::PLANT_YELLOW, x, y, z);
    }

    // Tall grass / ferns
    let n = match biome {
        Biome::Forest | Biome::SeasonalForest => 2,
        Biome::Rainforest | Biome::Plains => 10,
        Biome::Taiga => 1,
        _ => 0,
    };
    for _ in 0..n {
        let mut meta = 1;
        if biome == Biome::Rainforest && w.prand().next_int(3) != 0 {
            meta = 2;
        }
        let (x, y, z) = surface_pos(w, x0, z0);
        features::tall_grass(w, block::TALL_GRASS, meta, x, y, z);
    }

    let n = if biome == Biome::Desert { 2 } else { 0 };
    for _ in 0..n {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::dead_bush(w, block::DEAD_BUSH, x, y, z);
    }
    if w.prand().next_int(2) == 0 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::flowers(w, block::PLANT_RED, x, y, z);
    }
    if w.prand().next_int(4) == 0 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::flowers(w, block::MUSHROOM_BROWN, x, y, z);
    }
    if w.prand().next_int(8) == 0 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::flowers(w, block::MUSHROOM_RED, x, y, z);
    }
    for _ in 0..10 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::reed(w, x, y, z);
    }
    if w.prand().next_int(32) == 0 {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::pumpkin(w, x, y, z);
    }
    let n = if biome == Biome::Desert { 10 } else { 0 };
    for _ in 0..n {
        let (x, y, z) = surface_pos(w, x0, z0);
        features::cactus(w, x, y, z);
    }
    for _ in 0..50 {
        let x = x0 + w.prand().next_int(16) + 8;
        let b = w.prand().next_int(120) + 8;
        let y = w.prand().next_int(b);
        let z = z0 + w.prand().next_int(16) + 8;
        features::liquid_spring(w, block::WATER_MOVING, x, y, z);
    }
    for _ in 0..20 {
        let x = x0 + w.prand().next_int(16) + 8;
        let b = w.prand().next_int(112) + 8;
        let b = w.prand().next_int(b) + 8;
        let y = w.prand().next_int(b);
        let z = z0 + w.prand().next_int(16) + 8;
        features::liquid_spring(w, block::LAVA_MOVING, x, y, z);
    }

    // Snow
    let mut temps = Vec::new();
    w.terrain.biome_source.temperatures(&mut temps, x0 + 8, z0 + 8, 16, 16);
    for x in x0 + 8..x0 + 8 + 16 {
        for z in z0 + 8..z0 + 8 + 16 {
            let (i, j) = (x - (x0 + 8), z - (z0 + 8));
            let y = w.find_top_solid_block(x, z);
            let t = temps[(i * 16 + j) as usize] - (y - 64) as f64 / 64.0 * 0.3;
            if t < 0.5 && y > 0 && y < 128 && w.is_air_block(x, y, z) {
                let below = w.get_block_material(x, y - 1, z);
                if block::mat(below).blocks_movement && below != block::MAT_ICE {
                    w.set_block_with_notify(x, y, z, block::SNOW);
                }
            }
        }
    }
    w.sand_fall_instantly = false;
}

/// x/z within the chunk (no +8 offset), y below `max_y`; used by clay and ores.
fn ore_pos(w: &mut World, x0: i32, z0: i32, max_y: i32) -> (i32, i32, i32) {
    let r = w.prand();
    let x = x0 + r.next_int(16);
    let y = r.next_int(max_y);
    let z = z0 + r.next_int(16);
    (x, y, z)
}

/// x/z offset by +8, y anywhere; used by plants.
fn surface_pos(w: &mut World, x0: i32, z0: i32) -> (i32, i32, i32) {
    let r = w.prand();
    let x = x0 + r.next_int(16) + 8;
    let y = r.next_int(128);
    let z = z0 + r.next_int(16) + 8;
    (x, y, z)
}

/// `BiomeGenBase.getRandomWorldGenForTrees` and overrides.
fn tree_for_biome(w: &mut World, biome: Biome) -> TreeKind {
    let r = w.prand();
    match biome {
        Biome::Forest => {
            if r.next_int(5) == 0 {
                TreeKind::Birch
            } else if r.next_int(3) == 0 {
                TreeKind::BigOak
            } else {
                TreeKind::Oak
            }
        }
        Biome::Rainforest => {
            if r.next_int(3) == 0 {
                TreeKind::BigOak
            } else {
                TreeKind::Oak
            }
        }
        Biome::Taiga => {
            if r.next_int(3) == 0 {
                TreeKind::Spruce1
            } else {
                TreeKind::Spruce2
            }
        }
        _ => {
            if r.next_int(10) == 0 {
                TreeKind::BigOak
            } else {
                TreeKind::Oak
            }
        }
    }
}
