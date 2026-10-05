//! `updateTick` bodies of blocks that tick randomly (or are scheduled by those):
//! grass, leaves, fire, still lava, mushrooms, reeds, cactus, snow, ice, glowing
//! redstone ore, plus fire's placement hooks and nether portal frames.

use super::chunk::LightKind;
use super::{behavior, RandSel, World};
use crate::block::{self, mat, material_of};

pub use crate::block::{NETHERRACK, PORTAL};
pub const SNOW_BLOCK: u8 = 80;
pub const ORE_REDSTONE_GLOWING: u8 = 74;
pub const TNT: u8 = 46;

/// `BlockFire.chanceToEncourageFire` / `abilityToCatchFire` (`initializeBlock`).
fn fire_rates(id: u8) -> (i32, i32) {
    match id {
        5 | 85 | 53 => (5, 20), // planks, fence, wooden stairs
        block::LOG => (5, 5),
        block::LEAVES => (30, 60),
        47 => (30, 20), // bookshelf
        TNT => (15, 100),
        block::TALL_GRASS => (60, 100),
        35 => (30, 60), // cloth
        _ => (0, 0),
    }
}

/// `BlockGrass.updateTick`
pub fn grass(w: &mut World, x: i32, y: i32, z: i32, rs: RandSel) {
    let above = w.get_block_id(x, y + 1, z);
    if w.get_block_light_value(x, y + 1, z) < 4 && block::light_opacity(above) > 2 {
        if w.rng(rs).next_int(4) != 0 {
            return;
        }
        w.set_block_with_notify(x, y, z, block::DIRT);
    } else if w.get_block_light_value(x, y + 1, z) >= 9 {
        let r = w.rng(rs);
        let nx = x + r.next_int(3) - 1;
        let ny = y + r.next_int(5) - 3;
        let nz = z + r.next_int(3) - 1;
        let above = w.get_block_id(nx, ny + 1, nz);
        if w.get_block_id(nx, ny, nz) == block::DIRT
            && w.get_block_light_value(nx, ny + 1, nz) >= 4
            && block::light_opacity(above) <= 2
        {
            w.set_block_with_notify(nx, ny, nz, block::GRASS);
        }
    }
}

/// `BlockLeaves.updateTick`: decay leaves flagged by a removed log once no log is within
/// 4 steps through leaves.
pub fn leaves(w: &mut World, x: i32, y: i32, z: i32) {
    let meta = w.get_block_metadata(x, y, z);
    if meta & 8 == 0 || meta & 4 != 0 {
        return;
    }
    let r = 4;
    let size = 32i32;
    let sq = size * size;
    let half = size / 2;
    let mut a = w
        .leaf_scratch
        .take()
        .unwrap_or_else(|| vec![0; (size * size * size) as usize]);
    let at =
        |dx: i32, dy: i32, dz: i32| ((dx + half) * sq + (dy + half) * size + dz + half) as usize;
    if w.check_chunks_exist(
        x - r - 1,
        y - r - 1,
        z - r - 1,
        x + r + 1,
        y + r + 1,
        z + r + 1,
    ) {
        for dx in -r..=r {
            for dy in -r..=r {
                for dz in -r..=r {
                    let id = w.get_block_id(x + dx, y + dy, z + dz);
                    a[at(dx, dy, dz)] = if id == block::LOG {
                        0
                    } else if id == block::LEAVES {
                        -2
                    } else {
                        -1
                    };
                }
            }
        }
        for dist in 1..=4 {
            for dx in -r..=r {
                for dy in -r..=r {
                    for dz in -r..=r {
                        if a[at(dx, dy, dz)] == dist - 1 {
                            for (ox, oy, oz) in [
                                (-1, 0, 0),
                                (1, 0, 0),
                                (0, -1, 0),
                                (0, 1, 0),
                                (0, 0, -1),
                                (0, 0, 1),
                            ] {
                                let i = at(dx + ox, dy + oy, dz + oz);
                                if a[i] == -2 {
                                    a[i] = dist;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let center = a[at(0, 0, 0)];
    w.leaf_scratch = Some(a);
    if center >= 0 {
        w.set_block_metadata(x, y, z, meta & !8);
    } else {
        let m = w.get_block_metadata(x, y, z);
        behavior::drop_block_as_item(w, block::LEAVES, x, y, z, m);
        w.set_block_with_notify(x, y, z, block::AIR);
    }
}

/// `BlockStationary.updateTick` for lava: occasionally set fire above it.
pub fn still_lava(w: &mut World, mut x: i32, mut y: i32, mut z: i32, rs: RandSel) {
    let n = w.rng(rs).next_int(3);
    for _ in 0..n {
        x += w.rng(rs).next_int(3) - 1;
        y += 1;
        z += w.rng(rs).next_int(3) - 1;
        let id = w.get_block_id(x, y, z);
        if id == block::AIR {
            let burns =
                |w: &mut World, x: i32, y: i32, z: i32| mat(w.get_block_material(x, y, z)).burns;
            if burns(w, x - 1, y, z)
                || burns(w, x + 1, y, z)
                || burns(w, x, y, z - 1)
                || burns(w, x, y, z + 1)
                || burns(w, x, y - 1, z)
                || burns(w, x, y + 1, z)
            {
                w.set_block_with_notify(x, y, z, block::FIRE);
                return;
            }
        } else if mat(material_of(id)).blocks_movement {
            return;
        }
    }
}

pub(crate) fn can_catch_fire(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    fire_rates(w.get_block_id(x, y, z)).0 > 0
}

/// `BlockFire.func_263_h`: any flammable neighbour.
fn fire_has_fuel(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    can_catch_fire(w, x + 1, y, z)
        || can_catch_fire(w, x - 1, y, z)
        || can_catch_fire(w, x, y - 1, z)
        || can_catch_fire(w, x, y + 1, z)
        || can_catch_fire(w, x, y, z - 1)
        || can_catch_fire(w, x, y, z + 1)
}

/// `BlockFire.canPlaceBlockAt`
pub(crate) fn fire_can_place(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    w.is_block_normal_cube(x, y - 1, z) || fire_has_fuel(w, x, y, z)
}

/// `BlockFire.onBlockAdded`
pub fn fire_added(w: &mut World, x: i32, y: i32, z: i32) {
    if w.get_block_id(x, y - 1, z) != block::OBSIDIAN || !try_to_create_portal(w, x, y, z) {
        if !w.is_block_normal_cube(x, y - 1, z) && !fire_has_fuel(w, x, y, z) {
            w.set_block_with_notify(x, y, z, block::AIR);
        } else {
            w.schedule_block_update(x, y, z, block::FIRE, 40);
        }
    }
}

/// `BlockFire.onNeighborBlockChange`
pub fn fire_neighbor(w: &mut World, x: i32, y: i32, z: i32) {
    if !w.is_block_normal_cube(x, y - 1, z) && !fire_has_fuel(w, x, y, z) {
        w.set_block_with_notify(x, y, z, block::AIR);
    }
}

/// `BlockFire.updateTick`
pub fn fire(w: &mut World, x: i32, y: i32, z: i32, rs: RandSel) {
    let on_netherrack = w.get_block_id(x, y - 1, z) == NETHERRACK;
    if !fire_can_place(w, x, y, z) {
        w.set_block_with_notify(x, y, z, block::AIR);
    }
    let rained_out = !on_netherrack
        && w.is_raining()
        && (w.can_block_be_rained_on(x, y, z)
            || w.can_block_be_rained_on(x - 1, y, z)
            || w.can_block_be_rained_on(x + 1, y, z)
            || w.can_block_be_rained_on(x, y, z - 1)
            || w.can_block_be_rained_on(x, y, z + 1));
    if rained_out {
        w.set_block_with_notify(x, y, z, block::AIR);
        return;
    }
    let age = w.get_block_metadata(x, y, z);
    if age < 15 {
        let inc = w.rng(rs).next_int(3) / 2;
        w.set_block_metadata(x, y, z, age + inc);
    }
    w.schedule_block_update(x, y, z, block::FIRE, 40);
    if !on_netherrack && !fire_has_fuel(w, x, y, z) {
        if !w.is_block_normal_cube(x, y - 1, z) || age > 3 {
            w.set_block_with_notify(x, y, z, block::AIR);
        }
    } else if !on_netherrack
        && !can_catch_fire(w, x, y - 1, z)
        && age == 15
        && w.rng(rs).next_int(4) == 0
    {
        w.set_block_with_notify(x, y, z, block::AIR);
    } else {
        try_to_catch_fire(w, x + 1, y, z, 300, rs, age);
        try_to_catch_fire(w, x - 1, y, z, 300, rs, age);
        try_to_catch_fire(w, x, y - 1, z, 250, rs, age);
        try_to_catch_fire(w, x, y + 1, z, 250, rs, age);
        try_to_catch_fire(w, x, y, z - 1, 300, rs, age);
        try_to_catch_fire(w, x, y, z + 1, 300, rs, age);
        for bx in x - 1..=x + 1 {
            for bz in z - 1..=z + 1 {
                for by in y - 1..=y + 4 {
                    if bx == x && by == y && bz == z {
                        continue;
                    }
                    let mut chance = 100;
                    if by > y + 1 {
                        chance += (by - (y + 1)) * 100;
                    }
                    let enc = neighbors_encourage_fire(w, bx, by, bz);
                    if enc > 0 {
                        let spread = (enc + 40) / (age + 30);
                        // (sic) the x-1 rain check uses the fire's own z.
                        if spread > 0
                            && w.rng(rs).next_int(chance) <= spread
                            && (!w.is_raining() || !w.can_block_be_rained_on(bx, by, bz))
                            && !w.can_block_be_rained_on(bx - 1, by, z)
                            && !w.can_block_be_rained_on(bx + 1, by, bz)
                            && !w.can_block_be_rained_on(bx, by, bz - 1)
                            && !w.can_block_be_rained_on(bx, by, bz + 1)
                        {
                            let mut a = age + w.rng(rs).next_int(5) / 4;
                            if a > 15 {
                                a = 15;
                            }
                            w.set_block_and_metadata_with_notify(bx, by, bz, block::FIRE, a);
                        }
                    }
                }
            }
        }
    }
}

/// `BlockFire.tryToCatchBlockOnFire`
fn try_to_catch_fire(w: &mut World, x: i32, y: i32, z: i32, chance: i32, rs: RandSel, age: i32) {
    let catch = fire_rates(w.get_block_id(x, y, z)).1;
    if w.rng(rs).next_int(chance) < catch {
        let is_tnt = w.get_block_id(x, y, z) == TNT;
        if w.rng(rs).next_int(age + 10) < 5 && !w.can_block_be_rained_on(x, y, z) {
            let mut a = age + w.rng(rs).next_int(5) / 4;
            if a > 15 {
                a = 15;
            }
            w.set_block_and_metadata_with_notify(x, y, z, block::FIRE, a);
        } else {
            w.set_block_with_notify(x, y, z, block::AIR);
        }
        if is_tnt {
            w.ignite_tnt(x, y, z);
        }
    }
}

/// `BlockFire.getChanceOfNeighborsEncouragingFire`
fn neighbors_encourage_fire(w: &mut World, x: i32, y: i32, z: i32) -> i32 {
    if !w.is_air_block(x, y, z) {
        return 0;
    }
    let mut c = 0;
    for (dx, dy, dz) in [
        (1, 0, 0),
        (-1, 0, 0),
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
    ] {
        let v = fire_rates(w.get_block_id(x + dx, y + dy, z + dz)).0;
        if v > c {
            c = v;
        }
    }
    c
}

/// `BlockPortal.tryToCreatePortal`
pub fn try_to_create_portal(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    let mut ox = 0;
    let mut oz = 0;
    if w.get_block_id(x - 1, y, z) == block::OBSIDIAN
        || w.get_block_id(x + 1, y, z) == block::OBSIDIAN
    {
        ox = 1;
    }
    if w.get_block_id(x, y, z - 1) == block::OBSIDIAN
        || w.get_block_id(x, y, z + 1) == block::OBSIDIAN
    {
        oz = 1;
    }
    if ox == oz {
        return false;
    }
    let (mut x, mut z) = (x, z);
    if w.get_block_id(x - ox, y, z - oz) == block::AIR {
        x -= ox;
        z -= oz;
    }
    for i in -1..=2 {
        for j in -1..=3 {
            let frame = i == -1 || i == 2 || j == -1 || j == 3;
            if (i != -1 && i != 2) || (j != -1 && j != 3) {
                let id = w.get_block_id(x + ox * i, y + j, z + oz * i);
                if frame {
                    if id != block::OBSIDIAN {
                        return false;
                    }
                } else if id != block::AIR && id != block::FIRE {
                    return false;
                }
            }
        }
    }
    w.editing_blocks = true;
    for i in 0..2 {
        for j in 0..3 {
            w.set_block_with_notify(x + ox * i, y + j, z + oz * i, PORTAL);
        }
    }
    w.editing_blocks = false;
    true
}

/// `BlockPortal.onNeighborBlockChange`: break a portal whose frame is incomplete.
pub fn portal_neighbor(w: &mut World, x: i32, y: i32, z: i32) {
    let (mut ox, mut oz) = (0, 1);
    if w.get_block_id(x - 1, y, z) == PORTAL || w.get_block_id(x + 1, y, z) == PORTAL {
        ox = 1;
        oz = 0;
    }
    let mut by = y;
    while w.get_block_id(x, by - 1, z) == PORTAL {
        by -= 1;
    }
    if w.get_block_id(x, by - 1, z) != block::OBSIDIAN {
        w.set_block_with_notify(x, y, z, block::AIR);
        return;
    }
    let mut h = 1;
    while h < 4 && w.get_block_id(x, by + h, z) == PORTAL {
        h += 1;
    }
    if h == 3 && w.get_block_id(x, by + h, z) == block::OBSIDIAN {
        let along_x =
            w.get_block_id(x - 1, y, z) == PORTAL || w.get_block_id(x + 1, y, z) == PORTAL;
        let along_z =
            w.get_block_id(x, y, z - 1) == PORTAL || w.get_block_id(x, y, z + 1) == PORTAL;
        if along_x && along_z {
            w.set_block_with_notify(x, y, z, block::AIR);
        } else if (w.get_block_id(x + ox, y, z + oz) != block::OBSIDIAN
            || w.get_block_id(x - ox, y, z - oz) != PORTAL)
            && (w.get_block_id(x - ox, y, z - oz) != block::OBSIDIAN
                || w.get_block_id(x + ox, y, z + oz) != PORTAL)
        {
            w.set_block_with_notify(x, y, z, block::AIR);
        }
    } else {
        w.set_block_with_notify(x, y, z, block::AIR);
    }
}

/// `BlockMushroom.updateTick`: rare spread to a nearby dark spot.
pub fn mushroom(w: &mut World, id: u8, mut x: i32, mut y: i32, mut z: i32, rs: RandSel) {
    if w.rng(rs).next_int(100) != 0 { return; }
    let mut remaining = 5;
    for bx in x - 4..=x + 4 { for bz in z - 4..=z + 4 { for by in y - 1..=y + 1 {
        if w.get_block_id(bx, by, bz) == id {
            remaining -= 1;
            if remaining <= 0 { return; }
        }
    }}}
    let mut nx = x + w.rng(rs).next_int(3) - 1;
    let mut ny = y + w.rng(rs).next_int(2) - w.rng(rs).next_int(2);
    let mut nz = z + w.rng(rs).next_int(3) - 1;
    for _ in 0..4 {
        if w.is_air_block(nx, ny, nz) && behavior::flower_can_stay(w, id, nx, ny, nz) { x=nx; y=ny; z=nz; }
        nx = x + w.rng(rs).next_int(3) - 1;
        ny = y + w.rng(rs).next_int(2) - w.rng(rs).next_int(2);
        nz = z + w.rng(rs).next_int(3) - 1;
    }
    if w.is_air_block(nx, ny, nz) && behavior::flower_can_stay(w, id, nx, ny, nz) { w.set_block_with_notify(nx, ny, nz, id); }
}

/// `BlockReed.updateTick` / `BlockCactus.updateTick`: grow up to three tall.
pub fn grow_column(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    if !w.is_air_block(x, y + 1, z) {
        return;
    }
    let mut h = 1;
    while w.get_block_id(x, y - h, z) == id {
        h += 1;
    }
    if h < 3 {
        let m = w.get_block_metadata(x, y, z);
        if m == 15 {
            w.set_block_with_notify(x, y + 1, z, id);
            w.set_block_metadata_with_notify(x, y, z, 0);
        } else {
            w.set_block_metadata_with_notify(x, y, z, m + 1);
        }
    }
}

/// `BlockSnow.updateTick` (layer) / `BlockSnowBlock.updateTick`: melt near bright block light.
pub fn melt_snow(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    if w.get_saved_light_value(LightKind::Block, x, y, z) > 11 {
        let m = w.get_block_metadata(x, y, z);
        behavior::drop_block_as_item(w, id, x, y, z, m);
        w.set_block_with_notify(x, y, z, block::AIR);
    }
}

/// `BlockIce.updateTick`
pub fn melt_ice(w: &mut World, x: i32, y: i32, z: i32) {
    if w.get_saved_light_value(LightKind::Block, x, y, z) > 11 - block::light_opacity(block::ICE) {
        let m = w.get_block_metadata(x, y, z);
        behavior::drop_block_as_item(w, block::ICE, x, y, z, m);
        w.set_block_with_notify(x, y, z, block::WATER_STILL);
    }
}

/// The rest of `BlockSapling.updateTick` (after BlockFlower's check).
pub fn sapling(w: &mut World, x: i32, y: i32, z: i32, rs: RandSel) {
    if w.get_block_light_value(x, y + 1, z) >= 9 && w.rng(rs).next_int(30) == 0 {
        let meta = w.get_block_metadata(x, y, z);
        if meta & 8 == 0 {
            w.set_block_metadata_with_notify(x, y, z, meta | 8);
        } else {
            grow_tree(w, x, y, z, rs);
        }
    }
}

/// `BlockSapling.growTree(world, x, y, z, rand)`: the tree generators run on `rand`
/// (World.rand from ticks and bone meal); a tree that doesn't fit puts the sapling back.
pub fn grow_tree(w: &mut World, x: i32, y: i32, z: i32, rs: RandSel) {
    use crate::features::TreeKind::*;
    let meta = w.get_block_metadata(x, y, z) & 3;
    w.set_block(x, y, z, 0);
    let kind = match meta {
        1 => Spruce2,
        2 => Birch,
        _ => {
            if w.rng(rs).next_int(10) == 0 {
                BigOak
            } else {
                Oak
            }
        }
    };
    let grown = crate::features::sapling_tree(w, kind, x, y, z, rs);
    if !grown {
        w.set_block_and_metadata(x, y, z, block::SAPLING, meta);
    }
}
