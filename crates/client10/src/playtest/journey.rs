//! Play-test scenarios: journey (see ../playtest.rs for how scenarios work): the Nether and
//! the End by their real portals, the dragon, the credits, death and respawn across dimensions.
//!
//! The End is reached the way the game does it (a lit End portal, `Minecraft.usePortal(1)`);
//! what a test needs there (a wounded or dying dragon, a walkway) is placed by the scenario's
//! arrival part (`ARRIVALS`), which the worker runs once the player is in the new world.
use super::{give, ground, settle};
use worldgen::entity::EntityKind;
use worldgen::World;

/// This file's scenarios by name (looked up by `--setup <name>`).
pub const SCENARIOS: &[(&str, super::Scenario)] = &[
    ("end_idle", end_trip),
    ("end_view", end_view),
    ("dragon_exit", dragon_exit),
    ("end_death", end_trip),
    ("nether_round_trip", nether_round_trip),
    ("hardcore_death", hardcore_death),
];

/// Second parts, run when the player arrives in another dimension (`playtest::arrived`).
pub const ARRIVALS: &[(&str, super::Arrival)] = &[
    ("end_idle", end_idle_arrived),
    ("end_view", end_view_arrived),
    ("dragon_exit", dragon_exit_arrived),
    ("end_death", end_death_arrived),
    ("nether_round_trip", nether_arrived),
];

/// A lit End portal (12 eyed frames around a 3x3 pool) sunk into the ground with the player
/// standing on the middle portal block: the portal has no collision box, so the player drops
/// into it on the first tick and `BlockEndPortal.onEntityCollidedWithBlock` sends it to the End.
fn end_portal_under(w: &mut World, player: i32) -> (i32, i32, i32) {
    use worldgen::block::{END_PORTAL, END_PORTAL_FRAME, STONE};
    let (x, top, z) = ground(w, player);
    for dx in -3..=3 {
        for dz in -3..=3 {
            for dy in -2..=0 {
                w.set_block_and_metadata(x + dx, top + dy, z + dz, STONE, 0);
            }
            for dy in 1..=3 {
                w.set_block_and_metadata(x + dx, top + dy, z + dz, 0, 0);
            }
        }
    }
    // Facing: 0 south(+Z), 1 west(-X), 2 north(-Z), 3 east(+X); each frame faces the middle.
    for i in -1..=1 {
        w.set_block_and_metadata(x + i, top, z - 2, END_PORTAL_FRAME, 0 | 4);
        w.set_block_and_metadata(x + i, top, z + 2, END_PORTAL_FRAME, 2 | 4);
        w.set_block_and_metadata(x - 2, top, z + i, END_PORTAL_FRAME, 3 | 4);
        w.set_block_and_metadata(x + 2, top, z + i, END_PORTAL_FRAME, 1 | 4);
    }
    for dx in -1..=1 {
        for dz in -1..=1 {
            w.set_block_and_metadata(x + dx, top, z + dz, END_PORTAL, 0);
        }
    }
    (x, top, z)
}

/// The End by its portal, carrying a few items to check they survive (sword, pearls, a potion).
fn end_trip(w: &mut World, player: i32) {
    end_portal_under(w, player);
    give(w, player, 2, 373, 1, 8194);
    give(w, player, 1, 368, 16, 0);
    give(w, player, 0, 276, 1, 0);
}

/// The End by its portal with empty hands, for comparing the view with the original
/// (tools/oracle10/journey/endshot.sh takes the same views from the real 1.0 client).
fn end_view(w: &mut World, player: i32) {
    end_portal_under(w, player);
}

/// Faces the island (yaw 90, level), as the oracle's first view.
fn end_view_arrived(w: &mut World, player: i32, dimension: i32) {
    report_arrival(w, player, dimension);
    if dimension != 1 {
        return;
    }
    let e = w.entities.get_mut(&player).unwrap();
    let (p, feet) = (e.pos, e.pos.y - e.y_offset as f64);
    e.set_location_and_angles(p.x, feet, p.z, 90.0, 0.0);
}

/// The dragon, if the End's decorator has spawned it (BiomeEndDecorator, chunk 0, 0).
fn dragon(w: &World) -> Option<i32> {
    w.entities.iter().find(|(_, e)| matches!(e.kind, EntityKind::Dragon(..))).map(|(id, _)| *id)
}

fn report_arrival(w: &mut World, player: i32, dimension: i32) {
    let p = w.entities[&player].pos;
    eprintln!("journey: player in dimension {dimension} at {:.2} {:.2} {:.2}", p.x, p.y, p.z);
    if let Some(id) = dragon(w) {
        let d = &w.entities[&id];
        let crystals = w.entities.values().filter(|e| matches!(e.kind, EntityKind::EnderCrystal { .. })).count();
        eprintln!("journey: dragon {id} at {:.1} {:.1} {:.1} health {}, {crystals} crystals", d.pos.x, d.pos.y, d.pos.z, d.living.health);
    }
}

/// In the End: the dragon has been hurt before (150 of 200), so its crystals visibly heal it
/// (EntityDragon.func_41011_ay: +1 every 10 ticks while a crystal is linked).
fn end_idle_arrived(w: &mut World, player: i32, dimension: i32) {
    report_arrival(w, player, dimension);
    if dimension != 1 {
        return;
    }
    if let Some(id) = dragon(w) {
        w.entities.get_mut(&id).unwrap().living.health = 150;
    }
}

/// Dying in the End: the player arrives with one heart and a half and walks off the platform.
fn end_death_arrived(w: &mut World, player: i32, dimension: i32) {
    report_arrival(w, player, dimension);
    if dimension == 1 {
        w.entities.get_mut(&player).unwrap().living.health = 3;
    }
}

/// Where the dragon dies: the exit portal (EntityDragon.func_41012_a) is built around its
/// column at y 64, with the pool's middle 6 blocks west (-X) of the arrival platform's centre.
const EXIT: (i32, i32) = (94, 0);

/// The dragon fight's last blow, then the walk into the exit portal.
fn dragon_exit(w: &mut World, player: i32) {
    end_trip(w, player);
}

/// In the End: an end stone walkway at y 64 (the exit portal's level) from the player east of
/// the dragon's death column; the player stands on it facing west (the arrival yaw, 90). The
/// dragon, worn down to 1 health, is moved over that column and takes the player's final hit to
/// the head: EntityDragon.onDeathUpdate (func_40120_m_) then rises for 200 ticks, drops its XP
/// orbs and builds the exit portal with the egg on top.
fn dragon_exit_arrived(w: &mut World, player: i32, dimension: i32) {
    use worldgen::entity::damage::{DamageKind, DamageSource};
    report_arrival(w, player, dimension);
    if dimension != 1 {
        return;
    }
    let (ex, ez) = EXIT;
    settle(w, ex, ez, 2);
    for x in ex + 4..=ex + 8 {
        for z in ez - 1..=ez + 1 {
            w.set_block_and_metadata(x, 64, z, worldgen::block::END_STONE, 0);
            for y in 65..=67 {
                w.set_block_and_metadata(x, y, z, 0, 0);
            }
        }
    }
    {
        let p = w.entities.get_mut(&player).unwrap();
        p.set_location_and_angles(ex as f64 + 7.5, 65.0, ez as f64 + 0.5, 90.0, 0.0);
        p.motion = worldgen::entity::Vec3D { x: 0.0, y: 0.0, z: 0.0 };
        p.fall_distance = 0.0;
    }
    let Some(id) = dragon(w) else {
        eprintln!("journey: no dragon in the End");
        return;
    };
    let mut d = w.entities.remove(&id).unwrap();
    d.set_position(ex as f64 + 0.5, 80.0, ez as f64 + 0.5);
    d.living.health = 1;
    let killed = d.attack_dragon_part(w, true, DamageSource::entity(DamageKind::Player, Some(player)), 10);
    eprintln!("journey: dragon hit {killed}, health {}", d.living.health);
    w.entities.insert(id, d);
}

/// A lit Nether portal 3 blocks south of the player: a 4x5 obsidian frame across X with the
/// 2x3 portal (block 90) inside, set into a stone pad. Walking south (W) enters it.
fn nether_round_trip(w: &mut World, player: i32) {
    use worldgen::block::{OBSIDIAN, STONE};
    let (x, top, z) = ground(w, player);
    let pz = z + 3;
    for dx in -3..=4 {
        for dz in -1..=6 {
            for dy in -2..=0 {
                w.set_block_and_metadata(x + dx, top + dy, z + dz, STONE, 0);
            }
            for dy in 1..=6 {
                w.set_block_and_metadata(x + dx, top + dy, z + dz, 0, 0);
            }
        }
    }
    // The frame's bottom row is sunk into the pad, so the player walks straight in.
    for dx in -1..=2 {
        w.set_block_and_metadata(x + dx, top, pz, OBSIDIAN, 0);
        w.set_block_and_metadata(x + dx, top + 4, pz, OBSIDIAN, 0);
    }
    for dy in 1..=3 {
        w.set_block_and_metadata(x - 1, top + dy, pz, OBSIDIAN, 0);
        w.set_block_and_metadata(x + 2, top + dy, pz, OBSIDIAN, 0);
        w.set_block_and_metadata(x, top + dy, pz, 90, 0);
        w.set_block_and_metadata(x + 1, top + dy, pz, 90, 0);
    }
}

/// In the Nether the player stands in the portal the Teleporter built, which may run along X or
/// along Z; the player turns (as with the mouse) to face out of it across the portal's plane, toward
/// the side with room to stand, so the script can step out and back in along a straight line.
fn nether_arrived(w: &mut World, player: i32, dimension: i32) {
    report_arrival(w, player, dimension);
    if dimension != -1 {
        return;
    }
    let fl = worldgen::math::floor_double;
    let p = w.entities[&player].pos;
    let (x, y, z) = (fl(p.x), fl(p.y - 1.62 + 0.01), fl(p.z));
    let frame = |id: u8| id == 90 || id == worldgen::block::OBSIDIAN;
    // The portal's plane runs along X when its neighbours along X are portal or frame.
    let along_x = frame(w.get_block_id(x - 1, y, z)) || frame(w.get_block_id(x + 1, y, z));
    let open = |w: &mut World, dx: i32, dz: i32| w.get_block_id(x + dx, y, z + dz) == 0 && w.get_block_id(x + dx, y + 1, z + dz) == 0
        && w.get_block_id(x + dx, y - 1, z + dz) != 0;
    // Yaw 0 faces +Z, 90 -X, 180 -Z, 270 +X.
    let yaw = if along_x { if open(w, 0, 1) { 0.0 } else { 180.0 } } else if open(w, 1, 0) { 270.0 } else { 90.0 };
    let e = w.entities.get_mut(&player).unwrap();
    let feet = p.y - e.y_offset as f64;
    e.set_location_and_angles(p.x, feet, p.z, yaw, 0.0);
    eprintln!("journey: Nether portal runs along {}, facing yaw {yaw}", if along_x { "X" } else { "Z" });
}

/// A hardcore world (WorldInfo.hardcore, as GuiCreateWorld's Hardcore mode sets it): the
/// player, down to two hearts, drops from 20 blocks up and dies of the fall.
fn hardcore_death(w: &mut World, player: i32) {
    w.info.hardcore = true;
    let (x, top, z) = ground(w, player);
    let p = w.entities.get_mut(&player).unwrap();
    p.living.health = 4;
    p.set_location_and_angles(x as f64 + 0.5, top as f64 + 21.0, z as f64 + 0.5, 0.0, 0.0);
}

/// Crash audit for the paths 1.0-only content reaches in the renderer (both crashes found so far
/// were a missing case on such a path): every block id 0..=122 with every metadata 0..=15 is
/// meshed, packed among arbitrary neighbours and alone, in the Overworld, the Nether and the End,
/// fancy and fast, with and without smooth lighting; every entity the 1.0 EntityList creates, plus
/// the ones it doesn't (XP orbs, lightning, items of every id, falling blocks, thrown potions, the
/// dying dragon) goes through the renderer's CPU side.
#[cfg(test)]
mod audit {
    use crate::render::blocks::Colorizers;
    use crate::render::video::VideoSettings;
    use std::cell::RefCell;
    use worldgen::entity::{Entity, EntityKind, Vec3D};
    use worldgen::world::Dimension;
    use worldgen::{BiomeSource, World};

    /// EntityRenderer.updateFogColor in the End at render distance Far, before the eye-brightness
    /// fade: WorldProviderEnd.getFogColor (0x8080A0 * 0.15) pulled 29% toward the black sky
    /// (BiomeGenEnd.getSkyColorByTemp is 0). The original printed 0.05314, 0.05314, 0.06642 after
    /// its fade (tools/oracle10/journey/endshot.sh), the port's End used the Overworld's fog.
    #[test]
    fn end_fog_color_is_world_provider_end_s() {
        let f = crate::render::sky::end_fog_color(0.0, 0, 0.0, 0.0, 0, 0.0);
        let k = 1.0 - (0.25f64).powf(0.25);
        let want = [128.0 / 255.0 * 0.15 * (1.0 - k), 128.0 / 255.0 * 0.15 * (1.0 - k), 160.0 / 255.0 * 0.15 * (1.0 - k)];
        for i in 0..3 {
            assert!((f[i] as f64 - want[i]).abs() < 1e-6, "{f:?} {want:?}");
        }
        // The fade the original applied when it printed its value is within a percent of 1.
        assert!((f[0] - 0.053136144).abs() < 0.0005 && (f[2] - 0.06642018).abs() < 0.0005);
    }

    fn colorizers() -> Colorizers {
        Colorizers { grass: vec![0xff60a040; 65536], foliage: vec![0xff408030; 65536] }
    }

    #[test]
    fn every_block_and_metadata_meshes() {
        let col = colorizers();
        for dimension in [Dimension::Overworld, Dimension::Nether, Dimension::End] {
            let mut w = World::with_dimension(42, 42, dimension);
            for cx in -1..=3 {
                for cz in -1..=2 {
                    w.load_chunk(cx, cz);
                }
            }
            let all: Vec<(u8, i32)> = (0..=122u8).flat_map(|id| (0..16).map(move |m| (id, m))).collect();
            // Straight into the chunk arrays (no onBlockAdded: a powered piston with facing 6 or 7
            // throws in the original's simulation too; this is about the renderer).
            let mut put = |w: &mut World, x: i32, y: i32, z: i32, id: u8, meta: i32| {
                let c = w.chunk_mut(x >> 4, z >> 4);
                c.blocks[worldgen::world::chunk::idx(x & 15, y, z & 15)] = id;
                c.data.set(x & 15, y, z & 15, meta);
            };
            // Packed: chunk (0, 0), neighbours are whatever comes next.
            for (i, &(id, meta)) in all.iter().enumerate() {
                let i = i as i32;
                put(&mut w, i & 15, 2 + (i >> 8), (i >> 4) & 15, id, meta);
            }
            // Alone: chunks (1, 0) and (2, 0), every other cell, with air around.
            for (i, &(id, meta)) in all.iter().enumerate() {
                let i = i as i32;
                let (x, z, y) = (16 + (i & 15) * 2, (i >> 4 & 7) * 2, 20 + (i >> 7) * 2);
                put(&mut w, x, y, z, id, meta);
            }
            let biomes = RefCell::new(match dimension {
                Dimension::Overworld => BiomeSource::new(42),
                Dimension::Nether => BiomeSource::hell(),
                _ => BiomeSource::sky(),
            });
            for (fancy, ao) in [(true, true), (false, false), (true, false)] {
                let video = VideoSettings { fancy, ao, ..Default::default() };
                w.leaves_opaque = !fancy;
                for cx in 0..=2 {
                    let (opaque, translucent) = crate::worker::mesh_chunk(&w, cx, 0, w.skylight_subtracted, &biomes, &col, video);
                    assert!(!opaque.is_empty() || !translucent.is_empty(), "{dimension:?} chunk {cx} empty");
                }
            }
        }
    }

    /// Run one entity through what RenderGlobal.renderEntities does for it on the CPU side.
    fn render(e: &Entity) {
        let _ = crate::render::entity::manager::renderer_for(e);
        for partial in [0.0, 0.5, 1.0] {
            if let Some(s) = crate::render::entity::bridge::render_state(e, partial, 0.8) {
                let _ = crate::render::entity::raw_living181::render(&s, [0.0, 64.0, 0.0], false);
            } else if let Some(s) = crate::render::entity::bridge::object_state(e, partial, 0.8, [0.8; 7]) {
                for fancy in [true, false] {
                    let _ = crate::world_effects::Effects::object_draws(&s, [0.0, 64.0, 0.0], [30.0, 10.0], fancy);
                }
            }
            let _ = crate::render::entity::effects::shadow_with_camera(e, &[], partial, [0.0, 64.0, 0.0], [0.0, 64.0, 0.0]);
            let _ = crate::render::entity::effects::fire(e, partial, [0.0, 64.0, 0.0], 10.0);
        }
    }

    #[test]
    fn every_entity_renders() {
        let mut w = World::with_dimension(42, 42, Dimension::End);
        w.load_chunk(0, 0);
        let mut n = 0;
        let mut place = |w: &mut World, mut e: Entity| {
            e.set_location_and_angles(2.5, 70.0, 3.5, 45.0, 10.0);
            e.on_update(w);
            render(&e);
            e.ticks_existed += 7;
            render(&e);
        };
        for entry in worldgen::entity::registry::ENTRIES {
            if let Some(e) = worldgen::entity::registry::create(&mut w, entry.name) {
                place(&mut w, e);
                n += 1;
            }
        }
        assert!(n >= 37, "{n} entities");
        // The dragon dying, and its crystals.
        let mut dragon = worldgen::entity::registry::create(&mut w, "EnderDragon").unwrap();
        dragon.set_location_and_angles(0.0, 90.0, 0.0, 0.0, 0.0);
        dragon.living.health = 0;
        for _ in 0..60 {
            dragon.on_update(&mut w);
        }
        if let EntityKind::Dragon(s) = &dragon.kind {
            assert!(s.death_ticks > 0, "the dragon's death animation runs");
        }
        render(&dragon);
        for value in [1, 3, 7, 17, 37, 73, 149, 307, 617, 1237, 2477] {
            { let e = Entity::xp_orb(&mut w, Vec3D::new(0.0, 70.0, 0.0), value); place(&mut w, e); }
        }
        { let e = Entity::lightning(&mut w, 0.0, 70.0, 0.0); place(&mut w, e); }
        for id in 1..=122u8 {
            if worldgen::block::props(id).exists {
                { let e = Entity::falling_sand(&mut w, 0.0, 70.0, 0.0, id); place(&mut w, e); }
            }
        }
        let items: Vec<i32> = (1..=122).chain(256..=400).chain(2256..=2266)
            .filter(|&id| worldgen::entity::player::registry::item(id).is_some())
            .collect();
        for &id in &items {
            for damage in [0, 1, 3, 15] {
                let stack = worldgen::entity::player::ItemStack::new(id, 1 + damage, damage);
                // RenderItem.drawItemIntoGui and the durability bar (hotbar, inventory).
                let tile: worldgen::world::tile::ItemStack = stack.into();
                let _ = crate::render::entity::gui_item::geometry(&tile, 10, 10, true);
                let _ = crate::render::entity::gui_item::durability(&tile, 10, 10);
                { let e = Entity::item(&mut w, 0.0, 70.0, 0.0, stack.into()); place(&mut w, e); }
            }
        }
        let thrower = worldgen::entity::registry::create(&mut w, "Zombie").unwrap();
        for damage in [0, 16, 8193, 8194, 8197, 8201, 8227, 16388, 16392, 16421, 32767] {
            { let e = worldgen::entity::Entity::potion_projectile(&mut w, &thrower, damage); place(&mut w, e); }
            let stack = worldgen::entity::player::ItemStack::new(373, 1, damage);
            { let e = Entity::item(&mut w, 0.0, 70.0, 0.0, stack.into()); place(&mut w, e); }
        }
    }
}
