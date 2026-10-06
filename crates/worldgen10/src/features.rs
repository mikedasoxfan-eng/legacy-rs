//! The `WorldGen*` feature generators used by overworld population, ported
//! line for line. Population uses `w.prand()`; saplings use their supplied RNG.

use crate::block::{self, mat, opaque_cube_lookup, MAT_LAVA, MAT_WATER};
use crate::features181::swamp_tree;
use crate::jmath;
use crate::math::{self, PI_F32};
use crate::random::JavaRandom;
use crate::world::behavior;
use crate::world::chunk::LightKind;
use crate::world::tile::{ItemStack, TileEntity};
use crate::world::{RandSel, World};

// ---------------------------------------------------------------- lakes

/// `WorldGenLakes`
pub fn lakes(w: &mut World, liquid: u8, x: i32, y: i32, z: i32) -> bool {
    let x = x - 8;
    let z = z - 8;
    let mut y = y;
    while y > 0 && w.is_air_block(x, y, z) {
        y -= 1;
    }
    y -= 4;
    let mut shape = [false; 2048];
    let blobs = w.prand().next_int(4) + 4;
    for _ in 0..blobs {
        let r = w.prand();
        let sx = r.next_double() * 6.0 + 3.0;
        let sy = r.next_double() * 4.0 + 2.0;
        let sz = r.next_double() * 6.0 + 3.0;
        let cx = r.next_double() * (16.0 - sx - 2.0) + 1.0 + sx / 2.0;
        let cy = r.next_double() * (8.0 - sy - 4.0) + 2.0 + sy / 2.0;
        let cz = r.next_double() * (16.0 - sz - 2.0) + 1.0 + sz / 2.0;
        for i in 1..15 {
            for j in 1..15 {
                for k in 1..7 {
                    let dx = (i as f64 - cx) / (sx / 2.0);
                    let dy = (k as f64 - cy) / (sy / 2.0);
                    let dz = (j as f64 - cz) / (sz / 2.0);
                    if dx * dx + dy * dy + dz * dz < 1.0 {
                        shape[(i * 16 + j) * 8 + k] = true;
                    }
                }
            }
        }
    }

    let s = |i: usize, j: usize, k: usize| shape[(i * 16 + j) * 8 + k];
    let border = |i: usize, j: usize, k: usize| {
        !s(i, j, k)
            && (i < 15 && s(i + 1, j, k)
                || i > 0 && s(i - 1, j, k)
                || j < 15 && s(i, j + 1, k)
                || j > 0 && s(i, j - 1, k)
                || k < 7 && s(i, j, k + 1)
                || k > 0 && s(i, j, k - 1))
    };

    for i in 0..16usize {
        for j in 0..16usize {
            for k in 0..8usize {
                if border(i, j, k) {
                    let (bx, by, bz) = (x + i as i32, y + k as i32, z + j as i32);
                    let m = w.get_block_material(bx, by, bz);
                    if k >= 4 && mat(m).is_liquid {
                        return false;
                    }
                    if k < 4 && !mat(m).is_solid && w.get_block_id(bx, by, bz) != liquid {
                        return false;
                    }
                }
            }
        }
    }
    for i in 0..16usize {
        for j in 0..16usize {
            for k in 0..8usize {
                if s(i, j, k) {
                    let id = if k >= 4 { block::AIR } else { liquid };
                    w.set_block(x + i as i32, y + k as i32, z + j as i32, id);
                }
            }
        }
    }
    for i in 0..16usize {
        for j in 0..16usize {
            for k in 4..8usize {
                let (bx, by, bz) = (x + i as i32, y + k as i32, z + j as i32);
                if s(i, j, k)
                    && w.get_block_id(bx, by - 1, bz) == block::DIRT
                    && w.get_saved_light_value(LightKind::Sky, bx, by, bz) > 0
                {
                    w.set_block(bx, by - 1, bz, block::GRASS);
                }
            }
        }
    }
    if block::material_of(liquid) == MAT_LAVA {
        for i in 0..16usize {
            for j in 0..16usize {
                for k in 0..8usize {
                    let (bx, by, bz) = (x + i as i32, y + k as i32, z + j as i32);
                    if border(i, j, k)
                        && (k < 4 || w.prand().next_int(2) != 0)
                        && w.material_is_solid(bx, by, bz)
                    {
                        w.set_block(bx, by, bz, block::STONE);
                    }
                }
            }
        }
    }
    true
}

// ---------------------------------------------------------------- dungeons

/// `WorldGenDungeons`
pub fn dungeon(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    let h = 3;
    let rx = w.prand().next_int(2) + 2;
    let rz = w.prand().next_int(2) + 2;
    let mut openings = 0;
    for bx in x - rx - 1..=x + rx + 1 {
        for by in y - 1..=y + h + 1 {
            for bz in z - rz - 1..=z + rz + 1 {
                let solid = w.material_is_solid(bx, by, bz);
                if by == y - 1 && !solid {
                    return false;
                }
                if by == y + h + 1 && !solid {
                    return false;
                }
                if (bx == x - rx - 1 || bx == x + rx + 1 || bz == z - rz - 1 || bz == z + rz + 1)
                    && by == y
                    && w.is_air_block(bx, by, bz)
                    && w.is_air_block(bx, by + 1, bz)
                {
                    openings += 1;
                }
            }
        }
    }
    if !(1..=5).contains(&openings) {
        return false;
    }

    for bx in x - rx - 1..=x + rx + 1 {
        for by in (y - 1..=y + h).rev() {
            for bz in z - rz - 1..=z + rz + 1 {
                if bx != x - rx - 1
                    && by != y - 1
                    && bz != z - rz - 1
                    && bx != x + rx + 1
                    && by != y + h + 1
                    && bz != z + rz + 1
                {
                    w.set_block_with_notify(bx, by, bz, block::AIR);
                } else if by >= 0 && !w.material_is_solid(bx, by - 1, bz) {
                    w.set_block_with_notify(bx, by, bz, block::AIR);
                } else if w.material_is_solid(bx, by, bz) {
                    if by == y - 1 && w.prand().next_int(4) != 0 {
                        w.set_block_with_notify(bx, by, bz, block::MOSSY_COBBLESTONE);
                    } else {
                        w.set_block_with_notify(bx, by, bz, block::COBBLESTONE);
                    }
                }
            }
        }
    }

    'chests: for _ in 0..2 {
        for _ in 0..3 {
            let cx = x + w.prand().next_int(rx * 2 + 1) - rx;
            let cz = z + w.prand().next_int(rz * 2 + 1) - rz;
            if w.is_air_block(cx, y, cz) {
                let mut walls = 0;
                if w.material_is_solid(cx - 1, y, cz) {
                    walls += 1;
                }
                if w.material_is_solid(cx + 1, y, cz) {
                    walls += 1;
                }
                if w.material_is_solid(cx, y, cz - 1) {
                    walls += 1;
                }
                if w.material_is_solid(cx, y, cz + 1) {
                    walls += 1;
                }
                if walls == 1 {
                    w.set_block_with_notify(cx, y, cz, block::CHEST);
                    for _ in 0..8 {
                        if let Some(item) = pick_loot(w.prand()) {
                            let slot = w.prand().next_int(27) as usize;
                            if let Some(TileEntity::Chest(slots)) =
                                w.get_block_tile_entity(cx, y, cz)
                            {
                                slots[slot] = Some(item);
                            }
                        }
                    }
                    continue 'chests;
                }
            }
        }
    }

    w.set_block_with_notify(x, y, z, block::MOB_SPAWNER);
    let mob = pick_mob(w.prand());
    if let Some(TileEntity::MobSpawner { mob: m, .. }) = w.get_block_tile_entity(x, y, z) {
        *m = mob.to_string();
    }
    true
}

fn pick_loot(r: &mut JavaRandom) -> Option<ItemStack> {
    let one = |id: i32| Some(ItemStack::new(id, 1, 0));
    match r.next_int(11) {
        0 => one(329),                                                              // saddle
        1 => Some(ItemStack::new(265, r.next_int(4) + 1, 0)),                       // iron ingot
        2 => one(297),                                                              // bread
        3 => Some(ItemStack::new(296, r.next_int(4) + 1, 0)),                       // wheat
        4 => Some(ItemStack::new(289, r.next_int(4) + 1, 0)),                       // gunpowder
        5 => Some(ItemStack::new(287, r.next_int(4) + 1, 0)),                       // string
        6 => one(325),                                                              // bucket
        7 if r.next_int(100) == 0 => one(322),                                      // golden apple
        8 if r.next_int(2) == 0 => Some(ItemStack::new(331, r.next_int(4) + 1, 0)), // redstone
        9 if r.next_int(10) == 0 => one(2256 + r.next_int(2)), // record 13 / cat
        10 => Some(ItemStack::new(351, 1, 3)),                 // cocoa beans
        _ => None,
    }
}

fn pick_mob(r: &mut JavaRandom) -> &'static str {
    match r.next_int(4) {
        0 => "Skeleton",
        1 | 2 => "Zombie",
        3 => "Spider",
        _ => "",
    }
}

// ---------------------------------------------------------------- ore-like blobs

/// `WorldGenClay` (replaces sand under water).
pub fn clay(w: &mut World, n: i32, x: i32, y: i32, z: i32) -> bool {
    if w.get_block_material(x, y, z) != MAT_WATER {
        return false;
    }
    disc(w, n, x, y, z, block::CLAY, 1, block::CLAY);
    true
}

/// `WorldGenSand`, also used for gravel (the decorator accidentally calls sand twice).
pub fn sand(w: &mut World, n: i32, id: u8, x: i32, y: i32, z: i32) -> bool {
    if w.get_block_material(x, y, z) != MAT_WATER {
        return false;
    }
    disc(w, n, x, y, z, id, 2, block::GRASS);
    true
}
#[allow(clippy::too_many_arguments)]
fn disc(w: &mut World, n: i32, x: i32, y: i32, z: i32, id: u8, dy: i32, replace: u8) {
    let radius = w.prand().next_int(n - 2) + 2;
    for bx in x - radius..=x + radius {
        for bz in z - radius..=z + radius {
            if (bx - x) * (bx - x) + (bz - z) * (bz - z) <= radius * radius {
                for by in y - dy..=y + dy {
                    let b = w.get_block_id(bx, by, bz);
                    if b == block::DIRT || b == replace {
                        w.set_block(bx, by, bz, id);
                    }
                }
            }
        }
    }
}

/// `WorldGenMinable`
pub fn minable(w: &mut World, ore: u8, n: i32, x: i32, y: i32, z: i32) -> bool {
    blob(w, n, x, y, z, false, block::STONE, ore);
    true
}

/// Shared body of WorldGenClay/WorldGenMinable. They differ in loop structure
/// (clay has no early-outs) and that clay checks after the sphere test.
#[allow(clippy::too_many_arguments)]
fn blob(w: &mut World, n: i32, x: i32, y: i32, z: i32, clay_loops: bool, target: u8, place: u8) {
    let angle = w.prand().next_float() * PI_F32;
    let nf = n as f32;
    let x0 = ((x + 8) as f32 + math::sin(angle) * nf / 8.0) as f64;
    let x1 = ((x + 8) as f32 - math::sin(angle) * nf / 8.0) as f64;
    let z0 = ((z + 8) as f32 + math::cos(angle) * nf / 8.0) as f64;
    let z1 = ((z + 8) as f32 - math::cos(angle) * nf / 8.0) as f64;
    let y0 = (y + w.prand().next_int(3) - 2) as f64;
    let y1 = (y + w.prand().next_int(3) - 2) as f64;
    for i in 0..=n {
        let cx = x0 + (x1 - x0) * i as f64 / n as f64;
        let cy = y0 + (y1 - y0) * i as f64 / n as f64;
        let cz = z0 + (z1 - z0) * i as f64 / n as f64;
        let size = w.prand().next_double() * n as f64 / 16.0;
        let rh = (math::sin(i as f32 * PI_F32 / nf) + 1.0) as f64 * size + 1.0;
        let rv = (math::sin(i as f32 * PI_F32 / nf) + 1.0) as f64 * size + 1.0;
        let bx0 = math::floor_double(cx - rh / 2.0);
        let by0 = math::floor_double(cy - rv / 2.0);
        let bz0 = math::floor_double(cz - rh / 2.0);
        let bx1 = math::floor_double(cx + rh / 2.0);
        let by1 = math::floor_double(cy + rv / 2.0);
        let bz1 = math::floor_double(cz + rh / 2.0);
        if clay_loops {
            for bx in bx0..=bx1 {
                for by in by0..=by1 {
                    for bz in bz0..=bz1 {
                        let dx = (bx as f64 + 0.5 - cx) / (rh / 2.0);
                        let dy = (by as f64 + 0.5 - cy) / (rv / 2.0);
                        let dz = (bz as f64 + 0.5 - cz) / (rh / 2.0);
                        if dx * dx + dy * dy + dz * dz < 1.0 && w.get_block_id(bx, by, bz) == target
                        {
                            w.set_block(bx, by, bz, place);
                        }
                    }
                }
            }
        } else {
            for bx in bx0..=bx1 {
                let dx = (bx as f64 + 0.5 - cx) / (rh / 2.0);
                if dx * dx < 1.0 {
                    for by in by0..=by1 {
                        let dy = (by as f64 + 0.5 - cy) / (rv / 2.0);
                        if dx * dx + dy * dy < 1.0 {
                            for bz in bz0..=bz1 {
                                let dz = (bz as f64 + 0.5 - cz) / (rh / 2.0);
                                if dx * dx + dy * dy + dz * dz < 1.0
                                    && w.get_block_id(bx, by, bz) == target
                                {
                                    w.set_block(bx, by, bz, place);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// ---------------------------------------------------------------- plants

#[inline]
fn scatter(w: &mut World, x: i32, y: i32, z: i32, xz: i32, dy: i32) -> (i32, i32, i32) {
    let r = w.prand();
    let px = x + r.next_int(xz) - r.next_int(xz);
    let py = y + r.next_int(dy) - r.next_int(dy);
    let pz = z + r.next_int(xz) - r.next_int(xz);
    (px, py, pz)
}

/// `WorldGenFlowers` (flowers and mushrooms)
pub fn flowers(w: &mut World, id: u8, x: i32, y: i32, z: i32) -> bool {
    for _ in 0..64 {
        let (px, py, pz) = scatter(w, x, y, z, 8, 4);
        if w.is_air_block(px, py, pz) && behavior::flower_can_stay(w, id, px, py, pz) {
            w.set_block(px, py, pz, id);
        }
    }
    true
}

/// `WorldGenTallGrass`
pub fn tall_grass(w: &mut World, id: u8, meta: i32, x: i32, mut y: i32, z: i32) -> bool {
    loop {
        let b = w.get_block_id(x, y, z);
        if (b != block::AIR && b != block::LEAVES) || y <= 0 {
            break;
        }
        y -= 1;
    }
    for _ in 0..128 {
        let (px, py, pz) = scatter(w, x, y, z, 8, 4);
        if w.is_air_block(px, py, pz) && behavior::flower_can_stay(w, id, px, py, pz) {
            w.set_block_and_metadata(px, py, pz, id, meta);
        }
    }
    true
}

/// `WorldGenDeadBush`
pub fn dead_bush(w: &mut World, id: u8, x: i32, mut y: i32, z: i32) -> bool {
    loop {
        let b = w.get_block_id(x, y, z);
        if (b != block::AIR && b != block::LEAVES) || y <= 0 {
            break;
        }
        y -= 1;
    }
    for _ in 0..4 {
        let (px, py, pz) = scatter(w, x, y, z, 8, 4);
        if w.is_air_block(px, py, pz) && behavior::flower_can_stay(w, id, px, py, pz) {
            w.set_block(px, py, pz, id);
        }
    }
    true
}

/// `WorldGenReed`
pub fn reed(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    for _ in 0..20 {
        let r = w.prand();
        let px = x + r.next_int(4) - r.next_int(4);
        let pz = z + r.next_int(4) - r.next_int(4);
        if w.is_air_block(px, y, pz)
            && (w.material_is(px - 1, y - 1, pz, MAT_WATER)
                || w.material_is(px + 1, y - 1, pz, MAT_WATER)
                || w.material_is(px, y - 1, pz - 1, MAT_WATER)
                || w.material_is(px, y - 1, pz + 1, MAT_WATER))
        {
            let r = w.prand();
            let b = r.next_int(3) + 1;
            let h = 2 + r.next_int(b);
            for i in 0..h {
                if behavior::reed_can_place(w, px, y + i, pz) {
                    w.set_block(px, y + i, pz, block::REED);
                }
            }
        }
    }
    true
}

/// `WorldGenPumpkin`
pub fn pumpkin(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    for _ in 0..64 {
        let (px, py, pz) = scatter(w, x, y, z, 8, 4);
        if w.is_air_block(px, py, pz)
            && w.get_block_id(px, py - 1, pz) == block::GRASS
            && behavior::pumpkin_can_place(w, px, py, pz)
        {
            let facing = w.prand().next_int(4);
            w.set_block_and_metadata(px, py, pz, block::PUMPKIN, facing);
        }
    }
    true
}

/// `WorldGenCactus`
pub fn cactus(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    for _ in 0..10 {
        let (px, py, pz) = scatter(w, x, y, z, 8, 4);
        if w.is_air_block(px, py, pz) {
            let r = w.prand();
            let b = r.next_int(3) + 1;
            let h = 1 + r.next_int(b);
            for i in 0..h {
                if behavior::cactus_can_stay(w, px, py + i, pz) {
                    w.set_block(px, py + i, pz, block::CACTUS);
                }
            }
        }
    }
    true
}

/// `WorldGenLiquids` (springs): place a source and tick it once immediately.
pub fn liquid_spring(w: &mut World, id: u8, x: i32, y: i32, z: i32) -> bool {
    if w.get_block_id(x, y + 1, z) != block::STONE {
        return false;
    }
    if w.get_block_id(x, y - 1, z) != block::STONE {
        return false;
    }
    if w.get_block_id(x, y, z) != block::AIR && w.get_block_id(x, y, z) != block::STONE {
        return false;
    }
    let mut stone = 0;
    if w.get_block_id(x - 1, y, z) == block::STONE {
        stone += 1;
    }
    if w.get_block_id(x + 1, y, z) == block::STONE {
        stone += 1;
    }
    if w.get_block_id(x, y, z - 1) == block::STONE {
        stone += 1;
    }
    if w.get_block_id(x, y, z + 1) == block::STONE {
        stone += 1;
    }
    let mut air = 0;
    if w.is_air_block(x - 1, y, z) {
        air += 1;
    }
    if w.is_air_block(x + 1, y, z) {
        air += 1;
    }
    if w.is_air_block(x, y, z - 1) {
        air += 1;
    }
    if w.is_air_block(x, y, z + 1) {
        air += 1;
    }
    if stone == 3 && air == 1 {
        w.set_block_with_notify(x, y, z, id);
        w.immediate_updates = true;
        behavior::update_tick(w, id, x, y, z, RandSel::Populate);
        w.immediate_updates = false;
    }
    true
}

// ---------------------------------------------------------------- trees

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeKind {
    Oak,     // WorldGenTrees
    Birch,   // WorldGenForest
    BigOak,  // WorldGenBigTree
    Spruce1, // WorldGenTaiga1
    Spruce2, // WorldGenTaiga2
    Swamp,   // WorldGenSwamp
}

pub fn tree(w: &mut World, kind: TreeKind, x: i32, y: i32, z: i32) -> bool {
    match kind {
        TreeKind::Oak => simple_tree(w, x, y, z, 4, 0, RandSel::Populate, false),
        TreeKind::Birch => simple_tree(w, x, y, z, 5, 2, RandSel::Populate, false),
        TreeKind::BigOak => BigTree::new().generate(w, x, y, z),
        TreeKind::Spruce1 => taiga1(w, x, y, z, RandSel::Populate),
        TreeKind::Spruce2 => taiga2(w, x, y, z, RandSel::Populate, false),
        TreeKind::Swamp => swamp_tree(w, x, y, z),
    }
}

/// BlockSapling creates a fresh generator without populate's func_517_a(1,1,1).
/// Keep the supplied RNG separate from generators invoked by nested chunk loads.
pub fn sapling_tree(w: &mut World, kind: TreeKind, x: i32, y: i32, z: i32, rs: RandSel) -> bool {
    match kind {
        TreeKind::Oak => simple_tree(w, x, y, z, 4, 0, rs, true),
        TreeKind::Birch => simple_tree(w, x, y, z, 5, 2, rs, true),
        TreeKind::BigOak => {
            let mut tree = BigTree::unscaled();
            tree.notify = true;
            tree.generate_with_random(w, x, y, z, rs)
        },
        TreeKind::Spruce2 => taiga2(w, x, y, z, rs, true),
        _ => unreachable!("BlockSapling tree kind"),
    }
}

/// Shared trunk/clearance check of the four small tree generators.
/// `outer_breaks` mirrors whether the y loop also stops on failure (taiga variants).
fn space_clear(
    w: &mut World,
    x: i32,
    y: i32,
    z: i32,
    height: i32,
    radius_at: impl Fn(i32) -> i32,
    outer_breaks: bool,
) -> bool {
    let mut ok = true;
    let mut by = y;
    while by <= y + 1 + height && (ok || !outer_breaks) {
        let r = radius_at(by);
        let mut bx = x - r;
        while bx <= x + r && ok {
            let mut bz = z - r;
            while bz <= z + r && ok {
                if (0..128).contains(&by) {
                    let id = w.get_block_id(bx, by, bz);
                    if id != block::AIR && id != block::LEAVES {
                        ok = false;
                    }
                } else {
                    ok = false;
                }
                bz += 1;
            }
            bx += 1;
        }
        by += 1;
    }
    ok
}

fn set_tree_block(w: &mut World, x: i32, y: i32, z: i32, id: u8, meta: i32, notify: bool) {
    if notify {
        w.set_block_and_metadata_with_notify(x, y, z, id, meta);
    } else if meta == 0 {
        w.set_block(x, y, z, id);
    } else {
        w.set_block_and_metadata(x, y, z, id, meta);
    }
}

/// `WorldGenTrees` (meta 0, base height 4) and `WorldGenForest` (birch, meta 2, base height 5).
fn simple_tree(w: &mut World, x: i32, y: i32, z: i32, base: i32, meta: i32, rs: RandSel, notify: bool) -> bool {
    let height = w.rng(rs).next_int(3) + base;
    if y < 1 || y + height + 1 > 128 {
        return false;
    }
    let clear = space_clear(
        w,
        x,
        y,
        z,
        height,
        |by| {
            let mut r = 1;
            if by == y {
                r = 0;
            }
            if by >= y + 1 + height - 2 {
                r = 2;
            }
            r
        },
        false,
    );
    if !clear {
        return false;
    }
    let ground = w.get_block_id(x, y - 1, z);
    if !((ground == block::GRASS || ground == block::DIRT) && y < 128 - height - 1) {
        return false;
    }
    w.set_block(x, y - 1, z, block::DIRT);
    for ly in y - 3 + height..=y + height {
        let dy = ly - (y + height);
        let r = 1 - dy / 2;
        for lx in x - r..=x + r {
            let dx = lx - x;
            for lz in z - r..=z + r {
                let dz = lz - z;
                if (dx.abs() != r || dz.abs() != r || w.rng(rs).next_int(2) != 0 && dy != 0)
                    && !opaque_cube_lookup(w.get_block_id(lx, ly, lz))
                {
                    set_tree_block(w, lx, ly, lz, block::LEAVES, meta, notify);
                }
            }
        }
    }
    for i in 0..height {
        let id = w.get_block_id(x, y + i, z);
        if id == block::AIR || id == block::LEAVES {
            set_tree_block(w, x, y + i, z, block::LOG, meta, notify);
        }
    }
    true
}

/// `WorldGenTaiga1` (tall, sparse spruce)
fn taiga1(w: &mut World, x: i32, y: i32, z: i32, rs: RandSel) -> bool {
    let r = w.rng(rs);
    let height = r.next_int(5) + 7;
    let bare = height - r.next_int(2) - 3;
    let crown = height - bare;
    let max_r = 1 + r.next_int(crown + 1);
    if y < 1 || y + height + 1 > 128 {
        return false;
    }
    let clear = space_clear(
        w,
        x,
        y,
        z,
        height,
        |by| if by - y < bare { 0 } else { max_r },
        true,
    );
    if !clear {
        return false;
    }
    let ground = w.get_block_id(x, y - 1, z);
    if !((ground == block::GRASS || ground == block::DIRT) && y < 128 - height - 1) {
        return false;
    }
    w.set_block(x, y - 1, z, block::DIRT);
    let mut rad = 0;
    let mut ly = y + height;
    while ly >= y + bare {
        for lx in x - rad..=x + rad {
            let dx = lx - x;
            for lz in z - rad..=z + rad {
                let dz = lz - z;
                if (dx.abs() != rad || dz.abs() != rad || rad <= 0)
                    && !opaque_cube_lookup(w.get_block_id(lx, ly, lz))
                {
                    w.set_block_and_metadata(lx, ly, lz, block::LEAVES, 1);
                }
            }
        }
        if rad >= 1 && ly == y + bare + 1 {
            rad -= 1;
        } else if rad < max_r {
            rad += 1;
        }
        ly -= 1;
    }
    for i in 0..height - 1 {
        let id = w.get_block_id(x, y + i, z);
        if id == block::AIR || id == block::LEAVES {
            w.set_block_and_metadata(x, y + i, z, block::LOG, 1);
        }
    }
    true
}

/// `WorldGenTaiga2` (bushy spruce)
fn taiga2(w: &mut World, x: i32, y: i32, z: i32, rs: RandSel, notify: bool) -> bool {
    let r = w.rng(rs);
    let height = r.next_int(4) + 6;
    let bare = 1 + r.next_int(2);
    let crown = height - bare;
    let max_r = 2 + r.next_int(2);
    if y < 1 || y + height + 1 > 128 {
        return false;
    }
    let clear = space_clear(
        w,
        x,
        y,
        z,
        height,
        |by| if by - y < bare { 0 } else { max_r },
        true,
    );
    if !clear {
        return false;
    }
    let ground = w.get_block_id(x, y - 1, z);
    if !((ground == block::GRASS || ground == block::DIRT) && y < 128 - height - 1) {
        return false;
    }
    w.set_block(x, y - 1, z, block::DIRT);
    let mut rad = w.rng(rs).next_int(2);
    let mut limit = 1;
    let mut reset = 0;
    for i in 0..=crown {
        let ly = y + height - i;
        for lx in x - rad..=x + rad {
            let dx = lx - x;
            for lz in z - rad..=z + rad {
                let dz = lz - z;
                if (dx.abs() != rad || dz.abs() != rad || rad <= 0)
                    && !opaque_cube_lookup(w.get_block_id(lx, ly, lz))
                {
                    set_tree_block(w, lx, ly, lz, block::LEAVES, 1, notify);
                }
            }
        }
        if rad >= limit {
            rad = reset;
            reset = 1;
            limit += 1;
            if limit > max_r {
                limit = max_r;
            }
        } else {
            rad += 1;
        }
    }
    let top_gap = w.rng(rs).next_int(3);
    for i in 0..height - top_gap {
        let id = w.get_block_id(x, y + i, z);
        if id == block::AIR || id == block::LEAVES {
            set_tree_block(w, x, y + i, z, block::LOG, 1, notify);
        }
    }
    true
}

/// `WorldGenBigTree`
pub struct BigTree {
    notify: bool,
    rand: JavaRandom,
    base: [i32; 3],
    height_limit: i32,
    height: i32,
    height_attenuation: f64,
    branch_slope: f64,
    scale_width: f64,
    leaf_density: f64,
    trunk_size: i32,
    height_limit_limit: i32,
    leaf_distance_limit: i32,
    leaf_nodes: Vec<[i32; 4]>,
}

const OTHER_COORD_PAIRS: [usize; 6] = [2, 0, 0, 1, 2, 1];

/// Java `Math.pow(v, 2.0)`, which is exactly `v * v` for every input BigTree uses
/// (checked by `compare math`).
#[inline]
fn pow2(v: f64) -> f64 {
    v * v
}

impl BigTree {
    pub fn new() -> Self {
        let mut t = Self::unscaled();
        // populate() calls func_517_a(1.0, 1.0, 1.0) on every tree generator.
        t.set_scale(1.0, 1.0, 1.0);
        t
    }

    fn unscaled() -> Self {
        BigTree {
            notify: false,
            rand: JavaRandom::new(0),
            base: [0; 3],
            height_limit: 0,
            height: 0,
            height_attenuation: 0.618,
            branch_slope: 0.381,
            scale_width: 1.0,
            leaf_density: 1.0,
            trunk_size: 1,
            height_limit_limit: 12,
            leaf_distance_limit: 4,
            leaf_nodes: Vec::new(),
        }
    }

    /// `func_517_a`
    pub fn set_scale(&mut self, height: f64, width: f64, leaves: f64) {
        self.height_limit_limit = (height * 12.0) as i32;
        if height > 0.5 {
            self.leaf_distance_limit = 5;
        }
        self.scale_width = width;
        self.leaf_density = leaves;
    }

    pub fn generate(&mut self, w: &mut World, x: i32, y: i32, z: i32) -> bool {
        self.generate_with_random(w, x, y, z, RandSel::Populate)
    }

    fn generate_with_random(&mut self, w: &mut World, x: i32, y: i32, z: i32, rs: RandSel) -> bool {
        let seed = w.rng(rs).next_long();
        self.rand.set_seed(seed);
        self.base = [x, y, z];
        if self.height_limit == 0 {
            self.height_limit = 5 + self.rand.next_int(self.height_limit_limit);
        }
        if !self.valid_location(w) {
            return false;
        }
        self.generate_leaf_node_list(w);
        self.generate_leaves(w);
        self.generate_trunk(w);
        self.generate_leaf_node_bases(w);
        true
    }

    /// `func_521_a`
    fn generate_leaf_node_list(&mut self, w: &mut World) {
        self.height = (self.height_limit as f64 * self.height_attenuation) as i32;
        if self.height >= self.height_limit {
            self.height = self.height_limit - 1;
        }
        let mut per_layer =
            (1.382 + pow2(self.leaf_density * self.height_limit as f64 / 13.0)) as i32;
        if per_layer < 1 {
            per_layer = 1;
        }
        let mut nodes: Vec<[i32; 4]> = Vec::with_capacity((per_layer * self.height_limit) as usize);
        let mut layer_y = self.base[1] + self.height_limit - self.leaf_distance_limit;
        let trunk_top = self.base[1] + self.height;
        let mut rel = layer_y - self.base[1];
        nodes.push([self.base[0], layer_y, self.base[2], trunk_top]);
        layer_y -= 1;
        while rel >= 0 {
            let size = self.layer_size(rel);
            if size < 0.0 {
                layer_y -= 1;
                rel -= 1;
                continue;
            }
            let offset = 0.5;
            for _ in 0..per_layer {
                let dist = self.scale_width * size as f64 * (self.rand.next_float() as f64 + 0.328);
                let angle = self.rand.next_float() as f64 * 2.0 * 3.14159;
                let nx = math::floor_double(
                    dist * jmath::java_sin(angle) + self.base[0] as f64 + offset,
                );
                let nz = math::floor_double(
                    dist * jmath::java_cos(angle) + self.base[2] as f64 + offset,
                );
                let start = [nx, layer_y, nz];
                let end = [nx, layer_y + self.leaf_distance_limit, nz];
                if self.check_block_line(w, start, end) == -1 {
                    let mut branch_base = [self.base[0], self.base[1], self.base[2]];
                    let horiz = (pow2((self.base[0] - start[0]).abs() as f64)
                        + pow2((self.base[2] - start[2]).abs() as f64))
                    .sqrt();
                    let drop = horiz * self.branch_slope;
                    if start[1] as f64 - drop > trunk_top as f64 {
                        branch_base[1] = trunk_top;
                    } else {
                        branch_base[1] = (start[1] as f64 - drop) as i32;
                    }
                    if self.check_block_line(w, branch_base, start) == -1 {
                        nodes.push([nx, layer_y, nz, branch_base[1]]);
                    }
                }
            }
            layer_y -= 1;
            rel -= 1;
        }
        self.leaf_nodes = nodes;
    }

    /// `func_523_a`
    #[allow(clippy::too_many_arguments)]
    fn gen_tree_layer(
        &self,
        w: &mut World,
        x: i32,
        y: i32,
        z: i32,
        size: f32,
        axis: usize,
        id: u8,
    ) {
        let r = (size as f64 + 0.618) as i32;
        let a1 = OTHER_COORD_PAIRS[axis];
        let a2 = OTHER_COORD_PAIRS[axis + 3];
        let center = [x, y, z];
        let mut p = [0i32; 3];
        p[axis] = center[axis];
        for i in -r..=r {
            p[a1] = center[a1] + i;
            let mut j = -r;
            while j <= r {
                let d = (pow2(i.abs() as f64 + 0.5) + pow2(j.abs() as f64 + 0.5)).sqrt();
                if d > size as f64 {
                    j += 1;
                } else {
                    p[a2] = center[a2] + j;
                    let b = w.get_block_id(p[0], p[1], p[2]);
                    if b != block::AIR && b != block::LEAVES {
                        j += 1;
                    } else {
                        set_tree_block(w, p[0], p[1], p[2], id, 0, self.notify);
                        j += 1;
                    }
                }
            }
        }
    }

    /// `func_528_a`
    fn layer_size(&self, y: i32) -> f32 {
        if (y as f64) < (self.height_limit as f32) as f64 * 0.3 {
            return -1.618;
        }
        let half = self.height_limit as f32 / 2.0;
        let d = self.height_limit as f32 / 2.0 - y as f32;
        let mut s = if d == 0.0 {
            half
        } else if d.abs() >= half {
            0.0
        } else {
            (pow2(half.abs() as f64) - pow2(d.abs() as f64)).sqrt() as f32
        };
        s *= 0.5;
        s
    }

    /// `func_526_b`
    fn leaf_size(&self, y: i32) -> f32 {
        if y >= 0 && y < self.leaf_distance_limit {
            if y != 0 && y != self.leaf_distance_limit - 1 {
                3.0
            } else {
                2.0
            }
        } else {
            -1.0
        }
    }

    /// `func_520_a`
    fn generate_leaf_node(&self, w: &mut World, x: i32, y: i32, z: i32) {
        for ly in y..y + self.leaf_distance_limit {
            let s = self.leaf_size(ly - y);
            self.gen_tree_layer(w, x, ly, z, s, 1, block::LEAVES);
        }
    }

    /// `func_522_a`
    fn place_block_line(&self, w: &mut World, a: [i32; 3], b: [i32; 3], id: u8) {
        let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let mut major = 0usize;
        for i in 0..3 {
            if d[i].abs() > d[major].abs() {
                major = i;
            }
        }
        if d[major] == 0 {
            return;
        }
        let a1 = OTHER_COORD_PAIRS[major];
        let a2 = OTHER_COORD_PAIRS[major + 3];
        let step = if d[major] > 0 { 1 } else { -1 };
        let r1 = d[a1] as f64 / d[major] as f64;
        let r2 = d[a2] as f64 / d[major] as f64;
        let mut p = [0i32; 3];
        let mut i = 0;
        let end = d[major] + step;
        while i != end {
            p[major] = math::floor_double((a[major] + i) as f64 + 0.5);
            p[a1] = math::floor_double(a[a1] as f64 + i as f64 * r1 + 0.5);
            p[a2] = math::floor_double(a[a2] as f64 + i as f64 * r2 + 0.5);
            set_tree_block(w, p[0], p[1], p[2], id, 0, self.notify);
            i += step;
        }
    }

    /// `func_518_b`
    fn generate_leaves(&self, w: &mut World) {
        for n in &self.leaf_nodes {
            self.generate_leaf_node(w, n[0], n[1], n[2]);
        }
    }

    /// `func_527_c`
    fn leaf_node_needs_base(&self, y: i32) -> bool {
        y as f64 >= self.height_limit as f64 * 0.2
    }

    /// `func_529_c`
    fn generate_trunk(&self, w: &mut World) {
        let mut a = [self.base[0], self.base[1], self.base[2]];
        let mut b = [self.base[0], self.base[1] + self.height, self.base[2]];
        self.place_block_line(w, a, b, block::LOG);
        if self.trunk_size == 2 {
            a[0] += 1;
            b[0] += 1;
            self.place_block_line(w, a, b, block::LOG);
            a[2] += 1;
            b[2] += 1;
            self.place_block_line(w, a, b, block::LOG);
            a[0] -= 1;
            b[0] -= 1;
            self.place_block_line(w, a, b, block::LOG);
        }
    }

    /// `func_525_d`
    fn generate_leaf_node_bases(&self, w: &mut World) {
        let mut base = [self.base[0], self.base[1], self.base[2]];
        for n in &self.leaf_nodes {
            let tip = [n[0], n[1], n[2]];
            base[1] = n[3];
            if self.leaf_node_needs_base(base[1] - self.base[1]) {
                self.place_block_line(w, base, tip, block::LOG);
            }
        }
    }

    /// `func_524_a`: -1 if the line is clear, else the distance to the first obstruction.
    fn check_block_line(&self, w: &mut World, a: [i32; 3], b: [i32; 3]) -> i32 {
        let d = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
        let mut major = 0usize;
        for i in 0..3 {
            if d[i].abs() > d[major].abs() {
                major = i;
            }
        }
        if d[major] == 0 {
            return -1;
        }
        let a1 = OTHER_COORD_PAIRS[major];
        let a2 = OTHER_COORD_PAIRS[major + 3];
        let step = if d[major] > 0 { 1 } else { -1 };
        let r1 = d[a1] as f64 / d[major] as f64;
        let r2 = d[a2] as f64 / d[major] as f64;
        let mut p = [0i32; 3];
        let mut i = 0;
        let end = d[major] + step;
        while i != end {
            p[major] = a[major] + i;
            p[a1] = math::floor_double(a[a1] as f64 + i as f64 * r1);
            p[a2] = math::floor_double(a[a2] as f64 + i as f64 * r2);
            let b = w.get_block_id(p[0], p[1], p[2]);
            if b != block::AIR && b != block::LEAVES {
                break;
            }
            i += step;
        }
        if i == end {
            -1
        } else {
            i.abs()
        }
    }

    /// `func_519_e`
    fn valid_location(&mut self, w: &mut World) -> bool {
        let a = self.base;
        let b = [
            self.base[0],
            self.base[1] + self.height_limit - 1,
            self.base[2],
        ];
        let ground = w.get_block_id(self.base[0], self.base[1] - 1, self.base[2]);
        if ground != block::GRASS && ground != block::DIRT {
            return false;
        }
        let d = self.check_block_line(w, a, b);
        if d == -1 {
            true
        } else if d < 6 {
            false
        } else {
            self.height_limit = d;
            true
        }
    }
}

impl Default for BigTree {
    fn default() -> Self {
        Self::new()
    }
}
