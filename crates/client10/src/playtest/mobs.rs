//! Play-test scenarios: mobs (see ../playtest.rs for how scenarios work).
#[allow(unused_imports)]
use super::{give, ground, settle, spawn};
#[allow(unused_imports)]
use worldgen::World;

/// This file's scenarios by name (looked up by `--setup <name>`).
pub const SCENARIOS: &[(&str, super::Scenario)] = &[
    ("mobs_blaze", blaze),
    ("mobs_magma_cube", magma_cube),
    ("mobs_mooshroom", mooshroom),
    ("mobs_mooshroom_stew", mooshroom_stew),
    ("mobs_snow_golem", snow_golem),
    ("mobs_villager", villager),
    ("mobs_enderman_silverfish_cavespider", trio),
    ("mobs_breeding", breeding),
    ("mobs_wolf_tame", wolf_tame),
    ("mobs_nether_mobs", nether_mobs),
    ("mobs_fortress", fortress),
    ("mobs_spawners", spawners),
    ("mobs_killbox", killbox),
    ("mobs_all_entities_soak", all_entities_soak),
];

/// A flat grass arena around the player's ground block: x -8..=8, z -3..=14, three blocks of
/// dirt under a grass top at `top`, everything above (trees, hills) cleared to `top + 8`.
fn arena(w: &mut World, x: i32, top: i32, z: i32) {
    for dx in -8..=8 {
        for dz in -3..=14 {
            for dy in -3..=-1 {
                w.set_block_and_metadata(x + dx, top + dy, z + dz, 3, 0);
            }
            w.set_block_and_metadata(x + dx, top, z + dz, 2, 0);
            for dy in 1..=8 {
                w.set_block_and_metadata(x + dx, top + dy, z + dz, 0, 0);
            }
        }
    }
}

/// A Blaze 5 blocks south of the player, hovering over flat ground: it should float and
/// throw fireballs at the player (EntityBlaze.updateEntityActionState).
fn blaze(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    spawn(w, "Blaze", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 5.5);
}

/// A big Magma Cube (size 4) 5 blocks south of the player on flat ground, plus a small one.
fn magma_cube(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    let id = spawn(w, "LavaSlime", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 5.5);
    if let Some(e) = w.entities.get_mut(&id) {
        e.set_slime_size(4);
    }
    spawn(w, "LavaSlime", x as f64 + 3.5, top as f64 + 1.0, z as f64 + 6.5);
}

/// A fence ring (1.5 high: 1.0-era animals hop 1-high walls, but not fences; the 1.0 selection box is
/// only 1.0 high so right-clicks still reach over it) enclosing x-1..=x+1, z+2..=z+3 with the
/// given floor block, directly in front of the player so a right-click can reach them.
fn pen(w: &mut World, x: i32, top: i32, z: i32, floor: u8) {
    for dx in -2i32..=2 {
        for dz in 1..=4 {
            let wall = dx.abs() == 2 || dz == 1 || dz == 4;
            if wall {
                w.set_block_and_metadata(x + dx, top + 1, z + dz, 85, 0);
            } else {
                w.set_block_and_metadata(x + dx, top, z + dz, floor, 0);
            }
        }
    }
}

/// A Mooshroom on mycelium in a pen in front of the player, shears in hand: right-click turns
/// it into a cow and drops red mushrooms (EntityMooshroom.interact).
fn mooshroom(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    pen(w, x, top, z, 110);
    give(w, player, 0, 359, 1, 0);
    spawn(w, "MushroomCow", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 2.5);
}

/// Two stacked snow blocks 2 blocks south of the player, who stands two blocks higher on a stone
/// pillar with a pumpkin in hand: right-clicking the top snow block puts the pumpkin on top and
/// BlockPumpkin.onBlockAdded turns the three blocks into a SnowMan.
fn snow_golem(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    for dy in 1..=2 {
        w.set_block_and_metadata(x, top + dy, z, 1, 0);
        w.set_block_and_metadata(x, top + dy, z + 2, 80, 0);
    }
    let e = w.entities.get_mut(&player).unwrap();
    e.set_location_and_angles(x as f64 + 0.5, top as f64 + 3.0, z as f64 + 0.5, 0.0, 0.0);
    give(w, player, 0, 86, 1, 0);
}

/// A Villager 5 blocks south of the player in the open arena (wanders for the whole test).
fn villager(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    spawn(w, "Villager", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 5.5);
}

/// An Enderman, a Silverfish and a Cave Spider in the arena, in front of the player.
fn trio(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    spawn(w, "Enderman", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 7.5);
    spawn(w, "Silverfish", x as f64 - 1.5, top as f64 + 1.0, z as f64 + 4.5);
    spawn(w, "CaveSpider", x as f64 + 2.5, top as f64 + 1.0, z as f64 + 4.5);
}

/// The player stands on a one-block stone pillar (feet at `top + 2`), so a right-click can
/// reach down over the fences of a small pen next to it. Returns nothing; the pen cells are
/// (x.., z + 1), fenced on every side that is not the pillar.
fn pillar_pen(w: &mut World, player: i32, x: i32, top: i32, z: i32, cells: i32) {
    w.set_block_and_metadata(x, top + 1, z, 1, 0);
    for dx in -1..=cells {
        for dz in 0..=2 {
            let interior = dz == 1 && (0..cells).contains(&dx);
            if !interior && !(dz == 0 && dx == 0) && !(dz == 0 && dx == -1) {
                w.set_block_and_metadata(x + dx, top + 1, z + dz, 85, 0);
            }
        }
    }
    for dx in 0..cells {
        w.set_block_and_metadata(x + dx, top, z + 1, 2, 0);
    }
    let e = w.entities.get_mut(&player).unwrap();
    e.set_location_and_angles(x as f64 + 0.5, top as f64 + 2.0, z as f64 + 0.5, 0.0, 0.0);
}

/// Two cows in adjacent one-block pen cells next to the player, 2 wheat in hand: feeding both
/// puts them in love and (EntityAnimal.updateEntityActionState, 60 ticks) a calf appears.
fn breeding(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    pillar_pen(w, player, x, top, z, 2);
    give(w, player, 0, 296, 2, 0);
    spawn(w, "Cow", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 1.5);
    spawn(w, "Cow", x as f64 + 1.5, top as f64 + 1.0, z as f64 + 1.5);
}

/// A wild Wolf in a one-block pen next to the player, 10 bones in hand: each right-click has a
/// 1 in 3 chance to tame it (EntityWolf.interact).
fn wolf_tame(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    pillar_pen(w, player, x, top, z, 1);
    give(w, player, 0, 352, 10, 0);
    spawn(w, "Wolf", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 1.5);
}

/// A lit Nether portal 2 blocks south of the player (obsidian frame 4 wide x 5 tall, a 2x3 portal
/// inside, sunk level with the ground, as BlockPortal.tryToCreatePortal builds it, metadata 0),
/// 2 blocks in front of the player: walk in and stand still (the portal needs ~80 ticks inside).
fn nether_mobs(w: &mut World, player: i32) {
    portal_scene(w, player);
}

fn portal_scene(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    let pz = z + 2;
    for dx in -1..=2 {
        for dy in 0..=4 {
            let edge = dx == -1 || dx == 2 || dy == 0 || dy == 4;
            if edge {
                w.set_block_and_metadata(x + dx, top + dy, pz, 49, 0);
            } else {
                w.set_block_and_metadata(x + dx, top + dy, pz, 90, 0);
            }
        }
    }
}

/// Every entity of `registry::ENTRIES` that `registry::create` can make, standing in a grid in the
/// arena in front of the player (rows 3 blocks apart from 4 to 13 south, columns 3 blocks apart).
/// The Ender Dragon is skipped: it is a boss that only lives in the End.
fn all_entities_soak(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    let mut n = 0;
    for entry in worldgen::entity::registry::ENTRIES {
        if entry.name == "EnderDragon" {
            continue;
        }
        let Some(mut e) = worldgen::entity::registry::create(w, entry.name) else {
            eprintln!("playtest: soak skips {} (registry::create makes none)", entry.name);
            continue;
        };
        let (col, row) = (n % 5, n / 5);
        n += 1;
        e.set_location_and_angles(x as f64 - 6.0 + 3.0 * col as f64 + 0.5, top as f64 + 1.0, z as f64 + 4.5 + 3.0 * row as f64, 180.0, 0.0);
        w.entity_joined_world(e);
    }
    eprintln!("playtest: soak spawned {n} entities");
}

/// Seed 7 has a Nether fortress whose start chunk is (1, 4) (MapGenNetherBridge.canSpawnStructureAtCoords:
/// found with the Java random), i.e. Nether blocks x 18..36, z 66..84. The player is moved to the
/// Overworld spot 8x that (150, 550) and given the same lit portal as `mobs_nether_mobs`, so the
/// portal puts them next to the fortress (Teleporter scale 1/8).
fn fortress(w: &mut World, player: i32) {
    let e = w.entities.get_mut(&player).unwrap();
    e.set_location_and_angles(150.5, 100.0, 550.5, 0.0, 0.0);
    portal_scene(w, player);
}

/// Three monster spawner blocks (Blaze, Cave Spider, Silverfish) in a closed, roofed (dark) stone
/// chamber around the player: monsters only spawn in the dark (EntityMob.getCanSpawnHere), the spawner
/// needs a player within 16 blocks (TileEntityMobSpawner.updateEntity), first spawn after ~20 ticks.
fn spawners(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    for dx in -6..=6 {
        for dz in -3..=10 {
            let wall = dx == -6 || dx == 6 || dz == -3 || dz == 10;
            for dy in 1..=4 {
                let block = if wall || dy == 4 { 4 } else { 0 };
                w.set_block_and_metadata(x + dx, top + dy, z + dz, block, 0);
            }
        }
    }
    for (dx, mob) in [(-3, "Blaze"), (0, "CaveSpider"), (3, "Silverfish")] {
        w.set_block_and_metadata(x + dx, top + 1, z + 6, 52, 0);
        if let Some(worldgen::world::tile::TileEntity::MobSpawner { mob: m, .. }) = w.get_block_tile_entity(x + dx, top + 1, z + 6) {
            *m = mob.into();
        }
    }
    w.drain_lighting();
}

/// A diamond sword in hand and four weakened (3 health) monsters standing 2 to 3 blocks ahead: an
/// Enderman, a Zombie Pigman, a Blaze and a size-2 Magma Cube. Hitting them exercises the attack
/// packet path, the death animation, drops (blaze rod, magma cream, ender pearl, gold) and pickup.
fn killbox(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    give(w, player, 0, 276, 1, 0);
    let ids = [
        spawn(w, "Enderman", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 2.5),
        spawn(w, "PigZombie", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 3.5),
        spawn(w, "Blaze", x as f64 + 0.5, top as f64 + 1.5, z as f64 + 4.5),
        spawn(w, "LavaSlime", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 5.5),
    ];
    for (i, id) in ids.into_iter().enumerate() {
        if let Some(e) = w.entities.get_mut(&id) {
            e.living.health = 3;
            if i == 3 {
                e.set_slime_size(2);
                e.living.health = 3;
            }
        }
    }
}

/// The same pen with an empty bowl (281) in hand: right-clicking the Mooshroom turns the bowl
/// into mushroom stew (EntityMooshroom.interact) and the Mooshroom stays.
fn mooshroom_stew(w: &mut World, player: i32) {
    let (x, top, z) = ground(w, player);
    arena(w, x, top, z);
    pen(w, x, top, z, 110);
    give(w, player, 0, 281, 1, 0);
    spawn(w, "MushroomCow", x as f64 + 0.5, top as f64 + 1.0, z as f64 + 2.5);
}
