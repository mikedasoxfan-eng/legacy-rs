//! Block hooks reachable during world generation: `onBlockAdded`, `onBlockRemoval`,
//! `onNeighborBlockChange`, `updateTick` and item drops, dispatched by block id.
//!
//! Hooks for blocks generation never places panic, so a missing port shows up
//! loudly instead of silently diverging.

use super::tile::{ItemStack, TileEntity};
use super::{DroppedItem, RandSel, World};
use crate::block::{self, mat, material_of, opaque_cube_lookup, MAT_LAVA, MAT_WATER};

// Item ids (shiftedIndex) used by drops.
const ITEM_SEEDS: i32 = 295;
const ITEM_REED: i32 = 338;
const ITEM_FLINT: i32 = 318;
const ITEM_COAL: i32 = 263;
const ITEM_DIAMOND: i32 = 264;
const ITEM_DYE: i32 = 351;
const ITEM_REDSTONE: i32 = 331;
const ITEM_CLAY: i32 = 337;
const ITEM_SNOWBALL: i32 = 332;

fn fluid_material(id: u8) -> u8 {
    if id == block::WATER_MOVING || id == block::WATER_STILL {
        MAT_WATER
    } else {
        MAT_LAVA
    }
}

fn tick_rate(id: u8) -> i32 {
    match id {
        block::WATER_MOVING | block::WATER_STILL => 5,
        block::LAVA_MOVING | block::LAVA_STILL => 30,
        block::SAND | block::GRAVEL => 3,
        _ => 10,
    }
}

// ---------------------------------------------------------------- dispatch

pub fn on_block_added(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    match id {
        block::WATER_MOVING | block::LAVA_MOVING => {
            check_for_harden(w, id, x, y, z);
            if w.get_block_id(x, y, z) == id {
                w.schedule_block_update(x, y, z, id, tick_rate(id));
            }
        }
        block::WATER_STILL | block::LAVA_STILL => check_for_harden(w, id, x, y, z),
        block::SAND | block::GRAVEL => w.schedule_block_update(x, y, z, id, tick_rate(id)),
        block::FIRE => {
            if w.get_block_id(x, y - 1, z) != block::OBSIDIAN || !try_to_create_portal(w, x, y, z) {
                if !w.is_block_normal_cube(x, y - 1, z) && !fire_has_fuel(w, x, y, z) {
                    w.set_block_with_notify(x, y, z, block::AIR);
                } else {
                    w.schedule_block_update(x, y, z, id, 40);
                }
            }
        }
        block::CHEST | block::MOB_SPAWNER => w.set_block_tile_entity(x, y, z, TileEntity::for_block(id)),
        _ => assert_plain(id, "onBlockAdded"),
    }
}

pub fn on_block_removal(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    match id {
        block::LEAVES => {
            if w.check_chunks_exist(x - 2, y - 2, z - 2, x + 2, y + 2, z + 2) {
                for dx in -1..=1 {
                    for dy in -1..=1 {
                        for dz in -1..=1 {
                            if w.get_block_id(x + dx, y + dy, z + dz) == block::LEAVES {
                                let m = w.get_block_metadata(x + dx, y + dy, z + dz);
                                w.set_block_metadata(x + dx, y + dy, z + dz, m | 8);
                            }
                        }
                    }
                }
            }
        }
        block::LOG => {
            if w.check_chunks_exist(x - 5, y - 5, z - 5, x + 5, y + 5, z + 5) {
                for dx in -4..=4 {
                    for dy in -4..=4 {
                        for dz in -4..=4 {
                            if w.get_block_id(x + dx, y + dy, z + dz) == block::LEAVES {
                                let m = w.get_block_metadata(x + dx, y + dy, z + dz);
                                if m & 8 == 0 {
                                    w.set_block_metadata(x + dx, y + dy, z + dz, m | 8);
                                }
                            }
                        }
                    }
                }
            }
        }
        // BlockChest scatters its contents with its own private Random (entities only),
        // then BlockContainer drops the tile entity.
        block::CHEST | block::MOB_SPAWNER => w.remove_block_tile_entity(x, y, z),
        _ => assert_plain(id, "onBlockRemoval"),
    }
}

pub fn on_neighbor_block_change(w: &mut World, id: u8, x: i32, y: i32, z: i32, _changed: u8) {
    match id {
        block::FIRE => {
            if !w.is_block_normal_cube(x, y - 1, z) && !fire_has_fuel(w, x, y, z) {
                w.set_block_with_notify(x, y, z, block::AIR);
            }
        }
        block::PORTAL => portal_neighbor_change(w, x, y, z),
        block::WATER_MOVING | block::LAVA_MOVING => check_for_harden(w, id, x, y, z),
        block::WATER_STILL | block::LAVA_STILL => {
            check_for_harden(w, id, x, y, z);
            if w.get_block_id(x, y, z) == id {
                // BlockStationary.func_30004_j: become flowing without notifying.
                let meta = w.get_block_metadata(x, y, z);
                w.editing_blocks = true;
                w.set_block_and_metadata(x, y, z, id - 1, meta);
                w.schedule_block_update(x, y, z, id - 1, tick_rate(id));
                w.editing_blocks = false;
            }
        }
        block::SAND | block::GRAVEL => w.schedule_block_update(x, y, z, id, tick_rate(id)),
        block::PLANT_YELLOW
        | block::PLANT_RED
        | block::MUSHROOM_BROWN
        | block::MUSHROOM_RED
        | block::TALL_GRASS
        | block::DEAD_BUSH
        | block::SAPLING => check_flower(w, id, x, y, z),
        block::REED => {
            if !reed_can_place(w, x, y, z) {
                let meta = w.get_block_metadata(x, y, z);
                drop_block_as_item(w, id, x, y, z, meta);
                w.set_block_with_notify(x, y, z, block::AIR);
            }
        }
        block::CACTUS => {
            if !cactus_can_stay(w, x, y, z) {
                let meta = w.get_block_metadata(x, y, z);
                drop_block_as_item(w, id, x, y, z, meta);
                w.set_block_with_notify(x, y, z, block::AIR);
            }
        }
        block::SNOW => {
            if !snow_can_place(w, x, y, z) {
                let meta = w.get_block_metadata(x, y, z);
                drop_block_as_item(w, id, x, y, z, meta);
                w.set_block_with_notify(x, y, z, block::AIR);
            }
        }
        _ => assert_plain(id, "onNeighborBlockChange"),
    }
}

pub fn update_tick(w: &mut World, id: u8, x: i32, y: i32, z: i32, rs: RandSel) {
    match id {
        block::WATER_MOVING | block::LAVA_MOVING => flowing_update_tick(w, id, x, y, z, rs),
        block::SAND | block::GRAVEL => try_to_fall(w, id, x, y, z),
        block::PLANT_YELLOW
        | block::PLANT_RED
        | block::TALL_GRASS
        | block::DEAD_BUSH => check_flower(w, id, x, y, z),
        _ => panic!("updateTick for block {id} ({}) not ported", block::props(id).name),
    }
}

/// Blocks whose hooks are all no-ops in b1.7.3 (or only touch the renderer).
fn assert_plain(id: u8, hook: &str) {
    let plain = matches!(
        id,
        block::STONE
            | block::GRASS
            | block::DIRT
            | block::COBBLESTONE
            | block::BEDROCK
            | block::NETHERRACK
            | block::SOUL_SAND
            | block::GLOWSTONE
            | block::FIRE
            | block::PORTAL
            | block::ORE_GOLD
            | block::ORE_IRON
            | block::ORE_COAL
            | block::ORE_LAPIS
            | block::ORE_DIAMOND
            | block::ORE_REDSTONE
            | block::SANDSTONE
            | block::MOSSY_COBBLESTONE
            | block::OBSIDIAN
            | block::ICE
            | block::CLAY
            | block::PUMPKIN
            | block::LOG
            | block::LEAVES
            | block::CACTUS
            | block::REED
            | block::SNOW
            | block::TALL_GRASS
            | block::DEAD_BUSH
            | block::PLANT_YELLOW
            | block::PLANT_RED
            | block::MUSHROOM_BROWN
            | block::MUSHROOM_RED
            | block::WATER_MOVING
            | block::WATER_STILL
            | block::LAVA_MOVING
            | block::LAVA_STILL
            | block::SAND
            | block::GRAVEL
            | block::CHEST
            | block::MOB_SPAWNER
    );
    assert!(plain, "{hook} for block {id} ({}) not ported", block::props(id).name);
}

// ---------------------------------------------------------------- fluids

/// `BlockFluid.checkForHarden`
fn check_for_harden(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    if w.get_block_id(x, y, z) != id || fluid_material(id) != MAT_LAVA {
        return;
    }
    let touches_water = w.material_is(x, y, z - 1, MAT_WATER)
        || w.material_is(x, y, z + 1, MAT_WATER)
        || w.material_is(x - 1, y, z, MAT_WATER)
        || w.material_is(x + 1, y, z, MAT_WATER)
        || w.material_is(x, y + 1, z, MAT_WATER);
    if touches_water {
        let meta = w.get_block_metadata(x, y, z);
        if meta == 0 {
            w.set_block_with_notify(x, y, z, block::OBSIDIAN);
        } else if meta <= 4 {
            w.set_block_with_notify(x, y, z, block::COBBLESTONE);
        }
        trigger_lava_mix_effects(w);
    }
}

/// `triggerLavaMixEffects`: only the sound pitch touches World.rand.
fn trigger_lava_mix_effects(w: &mut World) {
    let _ = w.rand.next_float();
    let _ = w.rand.next_float();
}

fn get_flow_decay(w: &mut World, fluid_mat: u8, x: i32, y: i32, z: i32) -> i32 {
    if w.get_block_material(x, y, z) != fluid_mat {
        -1
    } else {
        w.get_block_metadata(x, y, z)
    }
}

fn flow_slot(id: u8) -> usize {
    if id == block::WATER_MOVING {
        0
    } else {
        1
    }
}

/// `BlockFlowing.getSmallestFlowDecay`
fn smallest_flow_decay(w: &mut World, id: u8, x: i32, y: i32, z: i32, cur: i32) -> i32 {
    let mut d = get_flow_decay(w, fluid_material(id), x, y, z);
    if d < 0 {
        return cur;
    }
    if d == 0 {
        w.flow[flow_slot(id)].num_adjacent_sources += 1;
    }
    if d >= 8 {
        d = 0;
    }
    if cur >= 0 && d >= cur {
        cur
    } else {
        d
    }
}

/// `BlockFlowing.blockBlocksFlow`
fn block_blocks_flow(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    let id = w.get_block_id(x, y, z);
    if matches!(id, block::DOOR_WOOD | block::DOOR_STEEL | block::SIGN_POST | block::LADDER | block::REED) {
        return true;
    }
    if id == block::AIR {
        return false;
    }
    mat(material_of(id)).blocks_movement
}

/// `BlockFlowing.liquidCanDisplaceBlock`
fn liquid_can_displace(w: &mut World, id: u8, x: i32, y: i32, z: i32) -> bool {
    let m = w.get_block_material(x, y, z);
    if m == fluid_material(id) || m == MAT_LAVA {
        false
    } else {
        !block_blocks_flow(w, x, y, z)
    }
}

/// `BlockFlowing.updateTick`
fn flowing_update_tick(w: &mut World, id: u8, x: i32, y: i32, z: i32, rs: RandSel) {
    let fm = fluid_material(id);
    let mut decay = get_flow_decay(w, fm, x, y, z);
    let step = if fm == MAT_LAVA && !w.dimension.is_hell_world() { 2 } else { 1 };
    let mut keep_flowing = true;
    if decay > 0 {
        w.flow[flow_slot(id)].num_adjacent_sources = 0;
        let mut s = smallest_flow_decay(w, id, x - 1, y, z, -100);
        s = smallest_flow_decay(w, id, x + 1, y, z, s);
        s = smallest_flow_decay(w, id, x, y, z - 1, s);
        s = smallest_flow_decay(w, id, x, y, z + 1, s);
        let mut next = s + step;
        if next >= 8 || s < 0 {
            next = -1;
        }
        if get_flow_decay(w, fm, x, y + 1, z) >= 0 {
            let above = get_flow_decay(w, fm, x, y + 1, z);
            next = if above >= 8 { above } else { above + 8 };
        }
        if w.flow[flow_slot(id)].num_adjacent_sources >= 2 && fm == MAT_WATER {
            if w.material_is_solid(x, y - 1, z) {
                next = 0;
            } else if w.get_block_material(x, y - 1, z) == fm && w.get_block_metadata(x, y, z) == 0 {
                next = 0;
            }
        }
        if fm == MAT_LAVA && decay < 8 && next < 8 && next > decay && w.rng(rs).next_int(4) != 0 {
            next = decay;
            keep_flowing = false;
        }
        if next != decay {
            decay = next;
            if next < 0 {
                w.set_block_with_notify(x, y, z, block::AIR);
            } else {
                w.set_block_metadata_with_notify(x, y, z, next);
                w.schedule_block_update(x, y, z, id, tick_rate(id));
                w.notify_blocks_of_neighbor_change(x, y, z, id);
            }
        } else if keep_flowing {
            make_stationary(w, id, x, y, z);
        }
    } else {
        make_stationary(w, id, x, y, z);
    }

    if liquid_can_displace(w, id, x, y - 1, z) {
        if decay >= 8 {
            w.set_block_and_metadata_with_notify(x, y - 1, z, id, decay);
        } else {
            w.set_block_and_metadata_with_notify(x, y - 1, z, id, decay + 8);
        }
    } else if decay >= 0 && (decay == 0 || block_blocks_flow(w, x, y - 1, z)) {
        optimal_flow_directions(w, id, x, y, z);
        let mut spread = decay + step;
        if decay >= 8 {
            spread = 1;
        }
        if spread >= 8 {
            return;
        }
        // Re-read the shared array after each flow: re-entrant ticks overwrite it.
        let slot = flow_slot(id);
        if w.flow[slot].optimal[0] {
            flow_into_block(w, id, x - 1, y, z, spread);
        }
        if w.flow[slot].optimal[1] {
            flow_into_block(w, id, x + 1, y, z, spread);
        }
        if w.flow[slot].optimal[2] {
            flow_into_block(w, id, x, y, z - 1, spread);
        }
        if w.flow[slot].optimal[3] {
            flow_into_block(w, id, x, y, z + 1, spread);
        }
    }
}

/// `BlockFlowing.func_30003_j`
fn make_stationary(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    let meta = w.get_block_metadata(x, y, z);
    w.set_block_and_metadata(x, y, z, id + 1, meta);
}

fn flow_into_block(w: &mut World, id: u8, x: i32, y: i32, z: i32, decay: i32) {
    if liquid_can_displace(w, id, x, y, z) {
        let cur = w.get_block_id(x, y, z);
        if cur > 0 {
            if fluid_material(id) == MAT_LAVA {
                trigger_lava_mix_effects(w);
            } else {
                let meta = w.get_block_metadata(x, y, z);
                drop_block_as_item(w, cur, x, y, z, meta);
            }
        }
        w.set_block_and_metadata_with_notify(x, y, z, id, decay);
    }
}

fn neighbor(dir: usize, x: i32, z: i32) -> (i32, i32) {
    match dir {
        0 => (x - 1, z),
        1 => (x + 1, z),
        2 => (x, z - 1),
        _ => (x, z + 1),
    }
}

/// `BlockFlowing.calculateFlowCost`
fn calculate_flow_cost(w: &mut World, id: u8, x: i32, y: i32, z: i32, depth: i32, from: usize) -> i32 {
    let fm = fluid_material(id);
    let mut best = 1000;
    for dir in 0..4usize {
        if (dir == 0 && from == 1) || (dir == 1 && from == 0) || (dir == 2 && from == 3) || (dir == 3 && from == 2) {
            continue;
        }
        let (nx, nz) = neighbor(dir, x, z);
        if !block_blocks_flow(w, nx, y, nz)
            && (w.get_block_material(nx, y, nz) != fm || w.get_block_metadata(nx, y, nz) != 0)
        {
            if !block_blocks_flow(w, nx, y - 1, nz) {
                return depth;
            }
            if depth < 4 {
                let c = calculate_flow_cost(w, id, nx, y, nz, depth + 1, dir);
                if c < best {
                    best = c;
                }
            }
        }
    }
    best
}

/// `BlockFlowing.getOptimalFlowDirections` (writes the shared per-block state).
fn optimal_flow_directions(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    let fm = fluid_material(id);
    let slot = flow_slot(id);
    for dir in 0..4usize {
        w.flow[slot].cost[dir] = 1000;
        let (nx, nz) = neighbor(dir, x, z);
        if !block_blocks_flow(w, nx, y, nz)
            && (w.get_block_material(nx, y, nz) != fm || w.get_block_metadata(nx, y, nz) != 0)
        {
            if !block_blocks_flow(w, nx, y - 1, nz) {
                w.flow[slot].cost[dir] = 0;
            } else {
                let c = calculate_flow_cost(w, id, nx, y, nz, 1, dir);
                w.flow[slot].cost[dir] = c;
            }
        }
    }
    let costs = w.flow[slot].cost;
    let min = costs.iter().copied().min().unwrap();
    for dir in 0..4 {
        w.flow[slot].optimal[dir] = costs[dir] == min;
    }
}

// ---------------------------------------------------------------- sand

fn can_fall_below(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    let id = w.get_block_id(x, y, z);
    if id == block::AIR || id == block::FIRE {
        return true;
    }
    let m = material_of(id);
    m == MAT_WATER || m == MAT_LAVA
}

/// `BlockSand.tryToFall`
fn try_to_fall(w: &mut World, id: u8, x: i32, mut y: i32, z: i32) {
    if can_fall_below(w, x, y - 1, z) && y >= 0 {
        let r = 32;
        if !w.sand_fall_instantly && w.check_chunks_exist(x - r, y - r, z - r, x + r, y + r, z + r) {
            // Spawns EntityFallingSand; the block stays until the entity ticks.
            if w.chunk_exists(x >> 4, z >> 4) {
                w.falling_sand.push((x, y, z, id));
            }
        } else {
            w.set_block_with_notify(x, y, z, block::AIR);
            while can_fall_below(w, x, y - 1, z) && y > 0 {
                y -= 1;
            }
            if y > 0 {
                w.set_block_with_notify(x, y, z, id);
            }
        }
    }
}

// ---------------------------------------------------------------- plants

/// `BlockFlower.canThisPlantGrowOnThisBlockID` and overrides.
fn plant_grows_on(id: u8, below: u8) -> bool {
    match id {
        block::MUSHROOM_BROWN | block::MUSHROOM_RED => opaque_cube_lookup(below),
        block::DEAD_BUSH => below == block::SAND,
        _ => below == block::GRASS || below == block::DIRT || below == block::TILLED_FIELD,
    }
}

/// `canBlockStay` for the BlockFlower family.
pub fn flower_can_stay(w: &mut World, id: u8, x: i32, y: i32, z: i32) -> bool {
    match id {
        block::MUSHROOM_BROWN | block::MUSHROOM_RED => {
            if !(0..128).contains(&y) {
                return false;
            }
            w.get_full_block_light_value(x, y, z) < 13 && plant_grows_on(id, w.get_block_id(x, y - 1, z))
        }
        _ => {
            (w.get_full_block_light_value(x, y, z) >= 8 || w.can_block_see_the_sky(x, y, z))
                && plant_grows_on(id, w.get_block_id(x, y - 1, z))
        }
    }
}

/// `BlockFlower.func_268_h`
fn check_flower(w: &mut World, id: u8, x: i32, y: i32, z: i32) {
    if !flower_can_stay(w, id, x, y, z) {
        let meta = w.get_block_metadata(x, y, z);
        drop_block_as_item(w, id, x, y, z, meta);
        w.set_block_with_notify(x, y, z, block::AIR);
    }
}

/// `BlockReed.canPlaceBlockAt` (also its `canBlockStay`).
pub fn reed_can_place(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    let below = w.get_block_id(x, y - 1, z);
    if below == block::REED {
        return true;
    }
    if below != block::GRASS && below != block::DIRT {
        return false;
    }
    w.material_is(x - 1, y - 1, z, MAT_WATER)
        || w.material_is(x + 1, y - 1, z, MAT_WATER)
        || w.material_is(x, y - 1, z - 1, MAT_WATER)
        || w.material_is(x, y - 1, z + 1, MAT_WATER)
}

/// `BlockCactus.canBlockStay`
pub fn cactus_can_stay(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    if w.material_is_solid(x - 1, y, z)
        || w.material_is_solid(x + 1, y, z)
        || w.material_is_solid(x, y, z - 1)
        || w.material_is_solid(x, y, z + 1)
    {
        return false;
    }
    let below = w.get_block_id(x, y - 1, z);
    below == block::CACTUS || below == block::SAND
}

/// `BlockSnow.canPlaceBlockAt`
fn snow_can_place(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    let below = w.get_block_id(x, y - 1, z);
    if below != block::AIR && w.is_opaque_cube_instance(below) {
        mat(w.get_block_material(x, y - 1, z)).blocks_movement
    } else {
        false
    }
}

/// `BlockPumpkin.canPlaceBlockAt`
pub fn pumpkin_can_place(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    let id = w.get_block_id(x, y, z);
    (id == block::AIR || mat(material_of(id)).ground_cover) && w.is_block_normal_cube(x, y - 1, z)
}

// ---------------------------------------------------------------- drops

fn quantity_dropped(w: &mut World, id: u8) -> i32 {
    match id {
        block::LEAVES => {
            if w.rand.next_int(20) == 0 {
                1
            } else {
                0
            }
        }
        block::SNOW | block::ICE | block::FIRE | block::PORTAL => 0,
        block::ORE_LAPIS => 4 + w.rand.next_int(5),
        block::ORE_REDSTONE => 4 + w.rand.next_int(2),
        block::CLAY => 4,
        _ => 1,
    }
}

fn id_dropped(w: &mut World, id: u8) -> i32 {
    match id {
        block::TALL_GRASS => {
            if w.rand.next_int(8) == 0 {
                ITEM_SEEDS
            } else {
                -1
            }
        }
        block::DEAD_BUSH | block::MOB_SPAWNER => -1,
        block::GRAVEL => {
            if w.rand.next_int(10) == 0 {
                ITEM_FLINT
            } else {
                block::GRAVEL as i32
            }
        }
        block::STONE => block::COBBLESTONE as i32,
        block::GRASS => block::DIRT as i32,
        block::LEAVES => block::SAPLING as i32,
        block::REED => ITEM_REED,
        block::ORE_COAL => ITEM_COAL,
        block::ORE_DIAMOND => ITEM_DIAMOND,
        block::ORE_LAPIS => ITEM_DYE,
        block::ORE_REDSTONE => ITEM_REDSTONE,
        block::CLAY => ITEM_CLAY,
        block::SNOW => ITEM_SNOWBALL,
        _ => id as i32,
    }
}

/// `Block.dropBlockAsItem` → `dropBlockAsItemWithChance(..., 1.0)`. Only the
/// World.rand draws matter for terrain; the entity is recorded for inspection.
pub fn drop_block_as_item(w: &mut World, id: u8, x: i32, y: i32, z: i32, meta: i32) {
    let n = quantity_dropped(w, id);
    for _ in 0..n {
        if w.rand.next_float() <= 1.0 {
            let item = id_dropped(w, id);
            if item > 0 {
                let damage = if id == block::LOG { meta } else { 0 };
                let s = 0.7f32;
                let ox = (w.rand.next_float() * s) as f64 + (1.0 - s) as f64 * 0.5;
                let oy = (w.rand.next_float() * s) as f64 + (1.0 - s) as f64 * 0.5;
                let oz = (w.rand.next_float() * s) as f64 + (1.0 - s) as f64 * 0.5;
                let (px, pz) = (x as f64 + ox, z as f64 + oz);
                if w.chunk_exists(crate::math::floor_double(px / 16.0), crate::math::floor_double(pz / 16.0)) {
                    w.dropped_items.push(DroppedItem { x: px, y: y as f64 + oy, z: pz, item: ItemStack::new(item, 1, damage) });
                }
            }
        }
    }
}

// ---------------------------------------------------------------- fire / portals

/// `BlockFire.chanceToEncourageFire`, initialized by `initializeBlock`.
fn fire_encouragement(id: u8) -> i32 {
    match id {
        5 | 85 | 53 | 17 => 5, // planks, fence, wood stairs, log
        18 | 47 | 35 => 30,    // leaves, bookshelf, wool
        46 => 15,              // TNT
        31 => 60,              // tall grass
        _ => 0,
    }
}

/// `BlockFire.func_263_h`; query order is observable through chunk loads.
fn fire_has_fuel(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    fire_encouragement(w.get_block_id(x + 1, y, z)) > 0
        || fire_encouragement(w.get_block_id(x - 1, y, z)) > 0
        || fire_encouragement(w.get_block_id(x, y - 1, z)) > 0
        || fire_encouragement(w.get_block_id(x, y + 1, z)) > 0
        || fire_encouragement(w.get_block_id(x, y, z - 1)) > 0
        || fire_encouragement(w.get_block_id(x, y, z + 1)) > 0
}

/// `BlockPortal.tryToCreatePortal`
fn try_to_create_portal(w: &mut World, mut x: i32, y: i32, mut z: i32) -> bool {
    let mut dx = 0;
    let mut dz = 0;
    if w.get_block_id(x - 1, y, z) == block::OBSIDIAN || w.get_block_id(x + 1, y, z) == block::OBSIDIAN {
        dx = 1;
    }
    if w.get_block_id(x, y, z - 1) == block::OBSIDIAN || w.get_block_id(x, y, z + 1) == block::OBSIDIAN {
        dz = 1;
    }
    if dx == dz {
        return false;
    }
    if w.get_block_id(x - dx, y, z - dz) == block::AIR {
        x -= dx;
        z -= dz;
    }
    for i in -1..=2 {
        for j in -1..=3 {
            let edge = i == -1 || i == 2 || j == -1 || j == 3;
            if i != -1 && i != 2 || j != -1 && j != 3 {
                let id = w.get_block_id(x + dx * i, y + j, z + dz * i);
                if edge {
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
            w.set_block_with_notify(x + dx * i, y + j, z + dz * i, block::PORTAL);
        }
    }
    w.editing_blocks = false;
    true
}

/// `BlockPortal.onNeighborBlockChange`
fn portal_neighbor_change(w: &mut World, x: i32, y: i32, z: i32) {
    let mut dx = 0;
    let mut dz = 1;
    if w.get_block_id(x - 1, y, z) == block::PORTAL || w.get_block_id(x + 1, y, z) == block::PORTAL {
        dx = 1;
        dz = 0;
    }
    let mut bottom = y;
    while w.get_block_id(x, bottom - 1, z) == block::PORTAL {
        bottom -= 1;
    }
    if w.get_block_id(x, bottom - 1, z) != block::OBSIDIAN {
        w.set_block_with_notify(x, y, z, block::AIR);
    } else {
        let mut height = 1;
        while height < 4 && w.get_block_id(x, bottom + height, z) == block::PORTAL {
            height += 1;
        }
        if height == 3 && w.get_block_id(x, bottom + height, z) == block::OBSIDIAN {
            let along_x = w.get_block_id(x - 1, y, z) == block::PORTAL || w.get_block_id(x + 1, y, z) == block::PORTAL;
            let along_z = w.get_block_id(x, y, z - 1) == block::PORTAL || w.get_block_id(x, y, z + 1) == block::PORTAL;
            if along_x && along_z {
                w.set_block_with_notify(x, y, z, block::AIR);
            } else if (w.get_block_id(x + dx, y, z + dz) != block::OBSIDIAN
                || w.get_block_id(x - dx, y, z - dz) != block::PORTAL)
                && (w.get_block_id(x - dx, y, z - dz) != block::OBSIDIAN
                    || w.get_block_id(x + dx, y, z + dz) != block::PORTAL)
            {
                w.set_block_with_notify(x, y, z, block::AIR);
            }
        } else {
            w.set_block_with_notify(x, y, z, block::AIR);
        }
    }
}
