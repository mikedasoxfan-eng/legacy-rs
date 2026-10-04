//! `ChunkProviderHell.populate` and the Nether `WorldGen*` features.
//! Unlike the overworld, population never reseeds `hellRNG`.

use crate::block;
use crate::features;
use crate::world::{behavior, RandSel, World};

pub fn populate(w: &mut World, cx: i32, cz: i32) {
    w.sand_fall_instantly = true;
    let x0 = cx * 16;
    let z0 = cz * 16;
    for _ in 0..8 {
        let (x, y, z) = position(w, x0, z0, 120, 4);
        hell_lava(w, x, y, z);
    }
    let bound = w.prand().next_int(10) + 1;
    let count = w.prand().next_int(bound) + 1;
    for _ in 0..count {
        let (x, y, z) = position(w, x0, z0, 120, 4);
        fire(w, x, y, z);
    }
    let bound = w.prand().next_int(10) + 1;
    let count = w.prand().next_int(bound);
    for _ in 0..count {
        let (x, y, z) = position(w, x0, z0, 120, 4);
        glow_stone(w, x, y, z);
    }
    for _ in 0..10 {
        let (x, y, z) = position(w, x0, z0, 128, 0);
        glow_stone(w, x, y, z);
    }
    // nextInt(1) still advances the RNG.
    if w.prand().next_int(1) == 0 {
        let (x, y, z) = position(w, x0, z0, 128, 0);
        features::flowers(w, block::MUSHROOM_BROWN, x, y, z);
    }
    if w.prand().next_int(1) == 0 {
        let (x, y, z) = position(w, x0, z0, 128, 0);
        features::flowers(w, block::MUSHROOM_RED, x, y, z);
    }
    w.sand_fall_instantly = false;
}

fn position(w: &mut World, x0: i32, z0: i32, bound: i32, offset: i32) -> (i32, i32, i32) {
    let x = x0 + w.prand().next_int(16) + 8;
    let y = w.prand().next_int(bound) + offset;
    let z = z0 + w.prand().next_int(16) + 8;
    (x, y, z)
}

/// `WorldGenHellLava.generate`
pub fn hell_lava(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    if w.get_block_id(x, y + 1, z) != block::NETHERRACK {
        return false;
    }
    if w.get_block_id(x, y, z) != block::AIR && w.get_block_id(x, y, z) != block::NETHERRACK {
        return false;
    }
    let neighbors = [(x - 1, y, z), (x + 1, y, z), (x, y, z - 1), (x, y, z + 1), (x, y - 1, z)];
    let mut rock = 0;
    for (nx, ny, nz) in neighbors {
        if w.get_block_id(nx, ny, nz) == block::NETHERRACK {
            rock += 1;
        }
    }
    let mut air = 0;
    for (nx, ny, nz) in neighbors {
        if w.is_air_block(nx, ny, nz) {
            air += 1;
        }
    }
    if rock == 4 && air == 1 {
        w.set_block_with_notify(x, y, z, block::LAVA_MOVING);
        w.immediate_updates = true;
        behavior::update_tick(w, block::LAVA_MOVING, x, y, z, RandSel::Populate);
        w.immediate_updates = false;
    }
    true
}

/// `WorldGenFire.generate`
pub fn fire(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    for _ in 0..64 {
        let bx = x + w.prand().next_int(8) - w.prand().next_int(8);
        let by = y + w.prand().next_int(4) - w.prand().next_int(4);
        let bz = z + w.prand().next_int(8) - w.prand().next_int(8);
        if w.is_air_block(bx, by, bz) && w.get_block_id(bx, by - 1, bz) == block::NETHERRACK {
            w.set_block_with_notify(bx, by, bz, block::FIRE);
        }
    }
    true
}

/// `WorldGenGlowStone1.generate` and `WorldGenGlowStone2.generate` have identical bodies.
pub fn glow_stone(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    if !w.is_air_block(x, y, z) {
        return false;
    }
    if w.get_block_id(x, y + 1, z) != block::NETHERRACK {
        return false;
    }
    w.set_block_with_notify(x, y, z, block::GLOWSTONE);
    for _ in 0..1500 {
        let bx = x + w.prand().next_int(8) - w.prand().next_int(8);
        let by = y - w.prand().next_int(12);
        let bz = z + w.prand().next_int(8) - w.prand().next_int(8);
        if w.get_block_id(bx, by, bz) == block::AIR {
            let mut count = 0;
            for (nx, ny, nz) in [
                (bx - 1, by, bz),
                (bx + 1, by, bz),
                (bx, by - 1, bz),
                (bx, by + 1, bz),
                (bx, by, bz - 1),
                (bx, by, bz + 1),
            ] {
                if w.get_block_id(nx, ny, nz) == block::GLOWSTONE {
                    count += 1;
                }
            }
            if count == 1 {
                w.set_block_with_notify(bx, by, bz, block::GLOWSTONE);
            }
        }
    }
    true
}
