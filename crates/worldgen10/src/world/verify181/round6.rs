//! Round-six probes call production world/block callbacks.
use super::*;
fn piston_callback_snapshot(o: &mut Vec<u8>, w: &mut World) {
    use crate::world::tile::TileEntity;
    result(o, w);
    for x in 7..=9 {
        for y in 63..=65 {
            for z in 7..=9 {
                let tile = w.get_block_tile_entity(x, y, z).cloned();
                o.push(matches!(tile, Some(TileEntity::Piston(_))) as u8);
                if let Some(TileEntity::Piston(p)) = tile {
                    for v in [p.block_id as i32, p.block_data, p.facing] {
                        i(o, v);
                    }
                    o.extend([p.extending as u8, p.render_head as u8]);
                    f181(o, p.progress);
                    f181(o, p.progress(0.0));
                    let id = w.chunk(x >> 4, z >> 4).tile_entity_ids[&(x & 15, y, z & 15)];
                    o.push(w.tile_objects[&id].invalid as u8);
                }
            }
        }
    }
}
pub(super) fn piston_callbacks(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    let mut p = crate::entity::Entity::player(&mut w, "oracle");
    for id in [29, 33] {
        for remote in [false, true] {
            for dx in [-3.0, -1.999, 0.0, 1.999, 2.0] {
                for dz in [-3.0, -1.999, 0.0, 1.999, 2.0] {
                    for py in [61.0, 63.0, 64.0, 65.0, 67.0, 69.0] {
                        for yaw in [-360.0, -45.0, 0.0, 45.0, 180.0, f32::INFINITY, f32::NAN] {
                            raw(&mut w, 8, 64, 8, id, 7);
                            w.multiplayer_world = remote;
                            p.pos.x = 8.0 + dx;
                            p.pos.y = py;
                            p.pos.z = 8.0 + dz;
                            p.yaw = yaw;
                            seed(&mut w);
                            p.block_placed(&mut w, id, 8, 64, 8, 1);
                            piston_callback_snapshot(o, &mut w);
                        }
                    }
                }
            }
        }
    }
    let power = [
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
        (1, 0, 0),
        (-1, 0, 0),
        (0, 0, 0),
        (0, 2, 0),
        (0, 1, -1),
        (0, 1, 1),
        (-1, 1, 0),
        (1, 1, 0),
    ];
    for id in [29, 33] {
        for remote in [false, true] {
            for ignored in [false, true] {
                for meta in [0, 1, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13] {
                    for source in -1..12 {
                        for hook in 0..2 {
                            let mut w = fixture(true);
                            raw(&mut w, 8, 64, 8, id, meta);
                            if source >= 0 {
                                let (dx, dy, dz) = power[source as usize];
                                raw(&mut w, 8 + dx, 64 + dy, 8 + dz, 76, 5);
                            }
                            w.multiplayer_world = remote;
                            w.piston_guard[(id == 29) as usize] = ignored;
                            seed(&mut w);
                            if hook == 0 {
                                behavior::on_block_added(&mut w, id, 8, 64, 8);
                            } else {
                                behavior::on_neighbor_block_change(&mut w, id, 8, 64, 8, 76);
                            }
                            piston_callback_snapshot(o, &mut w);
                        }
                    }
                }
            }
        }
    }
}
pub(super) fn base_block(o: &mut Vec<u8>) {
    use crate::block::{self, visual181};
    for id in 1..=109 {
        if !block::props(id).exists || matches!(id, 8..=11) {
            continue;
        }
        for normal in [false, true] {
            for (x, y, z) in [(8, 64, 8), (-17, -1, 31), (i32::MIN, i32::MAX, -30000001)] {
                let mut calls = Vec::new();
                let ao = visual181::ambient_occlusion181(x, y, z, |x, y, z| {
                    calls.push(vec![x, y, z]);
                    normal
                });
                f181(o, ao);
                let brightness = visual181::base_brightness181(id, x, y, z, |x, y, z, min| {
                    calls.push(vec![x, y, z, min]);
                    min as f32 * 0.125
                });
                f181(o, brightness);
                i(
                    o,
                    visual181::base_packed_light181(id, x, y, z, |x, y, z, min| {
                        calls.push(vec![x, y, z, min]);
                        x.wrapping_mul(31) ^ y.wrapping_mul(17) ^ z.wrapping_mul(7) ^ min
                    }),
                );
                if !matches!(id, 2 | 18 | 31 | 106) {
                    i(o, visual181::base_color181());
                }
                i(o, calls.len() as i32);
                for call in calls {
                    i(o, call.len() as i32);
                    for v in call {
                        i(o, v);
                    }
                }
            }
        }
    }
    let mut w = fixture(true);
    let mut p = crate::entity::Entity::player(&mut w, "oracle");
    for id in [1, 3, 4, 7, 22, 45, 48, 57, 87, 98] {
        for remote in [false, true] {
            for disabled in [false, true] {
                for meta in 0..16 {
                    for start in [-1.0, 0.0, 39.99, 40.0, f32::INFINITY, f32::NAN] {
                        w.multiplayer_world = remote;
                        let state = p.player_state_mut().unwrap();
                        state.capabilities.disable_damage = disabled;
                        state.food.exhaustion = start;
                        state.stat_log.clear();
                        w.loaded_entity_list.clear();
                        w.entities.clear();
                        seed(&mut w);
                        p.harvest_block(&mut w, id, 8, 64, 8, meta);
                        let state = p.player_state().unwrap();
                        f181(o, state.food.exhaustion);
                        i(o, state.stat_log.len() as i32);
                        for &(id, n) in &state.stat_log {
                            i(o, id);
                            i(o, n);
                        }
                        result(o, &mut w);
                    }
                }
            }
        }
    }
}
fn dispenser_snapshot(o: &mut Vec<u8>, w: &mut World) {
    use crate::world::tile::TileEntity;
    i(o, w.get_block_metadata(8, 64, 8));
    let tile = w.get_block_tile_entity(8, 64, 8).cloned();
    o.push(matches!(tile, Some(TileEntity::Dispenser(_))) as u8);
    if let Some(TileEntity::Dispenser(items)) = tile {
        for stack in *items {
            o.push(stack.is_some() as u8);
            if let Some(stack) = stack {
                for v in [stack.id as i32, stack.count, stack.damage as i32] {
                    i(o, v);
                }
            }
        }
    }
    i(o, w.tick_order.len() as i32);
    for (time, _, key) in &w.tick_order {
        for v in [key.x, key.y, key.z, key.block as i32] {
            i(o, v);
        }
        l(o, *time);
    }
    l(o, w.rand.state());
    i(o, w.loaded_entity_list.len() as i32);
    for id in &w.loaded_entity_list {
        let e = &w.entities[id];
        for v in [
            e.pos.x, e.pos.y, e.pos.z, e.motion.x, e.motion.y, e.motion.z,
        ] {
            o.extend(v.to_bits().to_le_bytes());
        }
    }
    super::super::events::write_events(&w.drain_events(), o);
}
pub(super) fn dispenser(o: &mut Vec<u8>) {
    use crate::{entity::Entity, world::tile::TileEntity};
    for remote in [false, true] {
        for meta in 0..16 {
            for mask in 0..16 {
                let mut w = fixture(true);
                for (k, (dx, dz)) in [(0, -1), (0, 1), (-1, 0), (1, 0)].into_iter().enumerate() {
                    raw(
                        &mut w,
                        8 + dx,
                        64,
                        8 + dz,
                        if mask & (1 << k) != 0 { 1 } else { 0 },
                        0,
                    );
                }
                raw(&mut w, 8, 64, 8, 23, meta);
                w.multiplayer_world = remote;
                seed(&mut w);
                behavior::on_block_added(&mut w, 23, 8, 64, 8);
                dispenser_snapshot(o, &mut w);
            }
        }
    }
    for remote in [false, true] {
        for invalid in [false, true] {
            for meta in 0..16 {
                let mut w = fixture(true);
                let mut p = Entity::player(&mut w, "oracle");
                raw(&mut w, 8, 64, 8, 23, meta);
                w.get_block_tile_entity(8, 64, 8);
                let id = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
                if invalid {
                    w.invalidate_tile181(id);
                }
                w.multiplayer_world = remote;
                seed(&mut w);
                o.push(p.activate_block(&mut w, 8, 64, 8) as u8);
                i(
                    o,
                    p.player_state()
                        .unwrap()
                        .events
                        .iter()
                        .filter(|e| {
                            matches!(e, crate::entity::player::PlayerEvent::OpenContainer { .. })
                        })
                        .count() as i32,
                );
                dispenser_snapshot(o, &mut w);
            }
        }
    }
    for remote in [false, true] {
        for invalid in [false, true] {
            let mut w = fixture(true);
            raw(&mut w, 8, 64, 8, 23, 15);
            w.get_block_tile_entity(8, 64, 8);
            let old = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
            if let Some(TileEntity::Dispenser(items)) = w.get_block_tile_entity(8, 64, 8) {
                items[0] = Some(crate::entity::player::ItemStack::new(4, 32, 2));
            }
            if invalid {
                w.invalidate_tile181(old);
            }
            w.multiplayer_world = remote;
            seed(&mut w);
            behavior::on_block_added(&mut w, 23, 8, 64, 8);
            w.get_block_tile_entity(8, 64, 8);
            o.push((w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)] != old) as u8);
            o.push(w.tile_objects[&old].invalid as u8);
            i(o, w.loaded_tile_entities.len() as i32);
            let Some(TileEntity::Dispenser(items)) = w.tile_object(old) else {
                panic!("dispenser");
            };
            let stack = items[0].unwrap();
            o.push(1);
            for v in [stack.id as i32, stack.count, stack.damage as i32] {
                i(o, v);
            }
            dispenser_snapshot(o, &mut w);
        }
    }
    for remote in [false, true] {
        for yaw in [
            f32::NEG_INFINITY,
            -f32::MAX,
            -720.0,
            -360.0,
            -135.0,
            -45.0,
            -0.0,
            0.0,
            44.999,
            45.0,
            90.0,
            135.0,
            180.0,
            225.0,
            270.0,
            315.0,
            360.0,
            720.0,
            f32::MAX,
            f32::INFINITY,
            f32::NAN,
        ] {
            let mut w = fixture(true);
            let mut p = Entity::player(&mut w, "oracle");
            p.yaw = yaw;
            raw(&mut w, 8, 64, 8, 23, 15);
            w.multiplayer_world = remote;
            seed(&mut w);
            p.block_placed(&mut w, 23, 8, 64, 8, 1);
            dispenser_snapshot(o, &mut w);
        }
    }
    for remote in [false, true] {
        for invalid in [false, true] {
            for meta in 0..16 {
                for mask in 0..4 {
                    for changed in [0, 1, 55, 76] {
                        for hook in 0..2 {
                            let mut w = fixture(true);
                            raw(&mut w, 8, 64, 8, 23, meta);
                            raw(&mut w, 7, 64, 8, if mask & 1 != 0 { 76 } else { 0 }, 5);
                            raw(&mut w, 7, 65, 8, if mask & 2 != 0 { 76 } else { 0 }, 5);
                            let mut items = Box::new([None; 9]);
                            items[0] = Some(crate::entity::player::ItemStack::new(4, 3, 2));
                            w.set_block_tile_entity(8, 64, 8, TileEntity::Dispenser(items));
                            w.dispenser_random
                                .insert((8, 64, 8), crate::JavaRandom::new(55));
                            if invalid {
                                let id = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
                                w.invalidate_tile181(id);
                            }
                            w.multiplayer_world = remote;
                            seed(&mut w);
                            if hook == 0 {
                                behavior::on_neighbor_block_change(&mut w, 23, 8, 64, 8, changed);
                            } else {
                                behavior::update_tick(&mut w, 23, 8, 64, 8, RandSel::World);
                            }
                            dispenser_snapshot(o, &mut w);
                        }
                    }
                }
            }
        }
    }
}
pub(super) fn materials(o: &mut Vec<u8>) {
    use crate::block::visual181::*;
    for name in SLAB_TYPES181 {
        i(o, name.len() as i32);
        o.extend(name.as_bytes());
    }
    for m in [
        i32::MIN,
        -2147483647,
        -65536,
        -257,
        -256,
        -17,
        -16,
        -1,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        15,
        16,
        17,
        255,
        256,
        65535,
        i32::MAX,
    ] {
        i(o, cloth_dye181(m));
        i(o, cloth_dye181(m));
        for side in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, i32::MAX] {
            for id in [35, 43, 44] {
                i(
                    o,
                    if id == 35 {
                        cloth_texture181(m)
                    } else {
                        slab_texture181(side, m)
                    },
                );
            }
        }
    }
}
pub(super) fn music(o: &mut Vec<u8>) {
    use crate::world::{notes::Note, tile::TileEntity};
    let mut w = fixture(true);
    let mut p = crate::entity::Entity::player(&mut w, "oracle");
    for remote in [false, true] {
        for m in 0..16 {
            for note in [-128, -26, -1, 0, 23, 24, 25, 126, 127] {
                for floor in [1, 12, 20, 5, 0] {
                    for roof in [0, 1] {
                        for hook in 0..5 {
                            w.multiplayer_world = false;
                            raw(&mut w, 8, 63, 8, floor, 0);
                            raw(&mut w, 8, 65, 8, roof, 0);
                            raw(&mut w, 8, 64, 8, 25, m);
                            raw(
                                &mut w,
                                7,
                                64,
                                8,
                                if hook == 3 && floor != 0 { 76 } else { 0 },
                                5,
                            );
                            w.set_block_tile_entity(
                                8,
                                64,
                                8,
                                TileEntity::Note(Note {
                                    note: note as i8,
                                    powered: m & 1 != 0,
                                }),
                            );
                            let id = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
                            w.tile_objects.get_mut(&id).unwrap().cached_metadata = -1;
                            w.multiplayer_world = remote;
                            seed(&mut w);
                            match hook {
                                0 => o.push(p.activate_block(&mut w, 8, 64, 8) as u8),
                                1 => p.block_clicked(&mut w, 25, 8, 64, 8),
                                2 => w.trigger_note(8, 64, 8),
                                3 => behavior::on_neighbor_block_change(&mut w, 25, 8, 64, 8, 76),
                                4 => w.play_note_at(8, 64, 8, floor as i32, note),
                                _ => unreachable!(),
                            }
                            if let TileEntity::Note(n) = w.tile_object(id).unwrap() {
                                i(o, n.note as i32);
                            }
                            i(o, w.tile_objects[&id].cached_metadata);
                            if let TileEntity::Note(n) = w.tile_object(id).unwrap() {
                                o.push(n.powered as u8);
                            }
                            result(o, &mut w);
                        }
                    }
                }
            }
        }
    }
    for remote in [false, true] {
        for m in 0..16 {
            for record in [0, 2256, 2257] {
                for hook in 0..4 {
                    w.multiplayer_world = false;
                    raw(&mut w, 7, 64, 8, 0, 0);
                    raw(&mut w, 8, 63, 8, 1, 0);
                    raw(&mut w, 8, 65, 8, 0, 0);
                    raw(&mut w, 8, 64, 8, 84, m);
                    w.set_block_tile_entity(8, 64, 8, TileEntity::RecordPlayer { record });
                    let id = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
                    w.tile_objects.get_mut(&id).unwrap().cached_metadata = -1;
                    w.multiplayer_world = remote;
                    w.loaded_entity_list.clear();
                    w.entities.clear();
                    seed(&mut w);
                    match hook {
                        0 => o.push(p.activate_block(&mut w, 8, 64, 8) as u8),
                        1 => w.eject_record(8, 64, 8),
                        2 => w.set_record181(8, 64, 8, record),
                        3 => behavior::on_block_removal(&mut w, 84, 8, 64, 8),
                        _ => unreachable!(),
                    }
                    if let TileEntity::RecordPlayer { record } = w.tile_object(id).unwrap() {
                        i(o, *record);
                    } else {
                        panic!()
                    }
                    i(o, w.tile_objects[&id].cached_metadata);
                    o.push(w.tile_objects[&id].invalid as u8);
                    result(o, &mut w);
                }
            }
        }
    }
}
use std::{fs, io::Write, path::Path};
const CONVERSION_COORDS: [(i32, i32, &str, &str); 7] = [
    (0, 0, "0", "0"),
    (1, 1, "1", "1"),
    (31, 31, "v", "v"),
    (-1, -1, "-1", "-1"),
    (32, 0, "w", "0"),
    (0, 32, "0", "w"),
    (1, 1, "1", "1"),
];
fn alpha_chunks(dir: &Path, dimension: i32) {
    for (n, (_, _, x, z)) in CONVERSION_COORDS.into_iter().enumerate() {
        let sub = dir.join(["0", "1", "b"][n % 3]).join("0");
        fs::create_dir_all(&sub).unwrap();
        let mut out = flate2::write::GzEncoder::new(
            fs::File::create(sub.join(format!("c.{x}.{z}.dat"))).unwrap(),
            flate2::Compression::default(),
        );
        let data: Vec<u8> = (0..128 + n * 23)
            .map(|v| (v * 37 + n * 11 + dimension as usize * 13) as u8)
            .collect();
        out.write_all(&data).unwrap();
        out.finish().unwrap();
    }
}
fn region_write(dir: &Path, x: i32, z: i32, data: &[u8]) {
    let mut encoder = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    encoder.write_all(data).unwrap();
    crate::region_cache181::write_chunk(dir, x, z, &encoder.finish().unwrap()).unwrap();
}
pub(super) fn conversion(o: &mut Vec<u8>) {
    crate::region_cache181::clear();
    let root = verification_dir("wb181-conversion6-rust");
    for config in 0..8 {
        let dir = root.join(format!("world{config}"));
        let nether = dir.join("DIM-1");
        let storage = crate::storage::Storage::open_with_players(&dir, false).unwrap();
        let mut info = LevelInfo::new(900 + config, "converted");
        info.game_type = config as i32 - 2;
        info.map_features = config & 1 != 0;
        info.version = 0;
        info.size_on_disk = 1234;
        storage.save_level(&info).unwrap();
        drop(storage);
        if config & 4 == 0 {
            alpha_chunks(&dir, 0);
        } else {
            fs::create_dir_all(dir.join("a")).unwrap();
        }
        if config & 1 != 0 {
            fs::create_dir_all(&nether).unwrap();
            if config & 4 == 0 {
                alpha_chunks(&nether, 1);
            }
        }
        if config & 2 != 0 {
            region_write(&dir, 0, 0, &[90; 167]);
        }
        let foreign = root.join("foreign");
        region_write(&foreign, config as i32, 0, &[5; 100]);
        o.push(crate::convert::is_old_map_format(&dir) as u8);
        let mut progress = Vec::new();
        o.push(crate::convert::convert_map_format(&dir, &mut |s| {
            if let crate::scenario::LoadStep::Progress(p) = s {
                progress.push(p)
            }
        }) as u8);
        i(o, progress.len() as i32);
        for p in progress {
            i(o, p);
        }
        o.push(crate::convert::is_old_map_format(&dir) as u8);
        let info = crate::storage::Storage::read_level(&dir).unwrap();
        l(o, info.seed);
        i(o, info.game_type);
        o.push(info.map_features as u8);
        i(o, info.version);
        l(o, info.size_on_disk);
        for name in [
            "players", "0", "1", "b", "a", "DIM-1/0", "DIM-1/1", "DIM-1/b",
        ] {
            o.push(dir.join(name).exists() as u8);
        }
        i(
            o,
            crate::region_cache181::size_delta(&foreign, config as i32, 0).unwrap(),
        );
        for dimension in 0..2 {
            let d = if dimension == 0 {
                dir.clone()
            } else {
                nether.clone()
            };
            for (x, z, _, _) in CONVERSION_COORDS {
                let data = crate::region_cache181::read_chunk(&d, x, z).unwrap();
                i(o, data.as_ref().map_or(-1, |v| v.len() as i32));
                if let Some(data) = data {
                    o.extend(data);
                }
                let file = fs::read(
                    d.join("region")
                        .join(format!("r.{}.{}.mcr", x >> 5, z >> 5)),
                )
                .unwrap();
                let at = ((x & 31) + (z & 31) * 32) as usize * 4;
                i(o, i32::from_be_bytes(file[at..at + 4].try_into().unwrap()));
            }
        }
        crate::region_cache181::clear();
    }
}
pub(super) fn push(o: &mut Vec<u8>) {
    use super::super::{
        pistons::{DIRECTIONS, Piston},
        tile::TileEntity,
    };
    for face in 0..6 {
        for extend in [false, true] {
            for stored in [1, 44, 85, 34, 36, 0] {
                for scene in 0..8 {
                    let mut w = fixture(true);
                    seed(&mut w);
                    raw(&mut w, 8, 72, 8, 36, face);
                    if scene & 1 != 0 {
                        for x in 6..=10 {
                            for z in 6..=10 {
                                raw(&mut w, x, 71, z, 1, 0);
                            }
                        }
                    }
                    if scene & 2 != 0 {
                        for y in 71..=74 {
                            raw(&mut w, 9, y, 8, 1, 0);
                        }
                    }
                    let mut p = Piston::new(stored, face, face, extend, false);
                    p.progress = if scene & 4 != 0 { 0.5 } else { 0.0 };
                    p.last_progress = p.progress;
                    w.set_block_tile_entity(8, 72, 8, TileEntity::Piston(p));
                    let tile = w.chunk(0, 0).tile_entity_ids[&(8, 72, 8)];
                    let (dx, dy, dz) = DIRECTIONS[face as usize];
                    let mut entities = Vec::new();
                    for j in 0..9 {
                        let offset = (j as f64 - 4.0) * 0.25;
                        let mut e = crate::entity::Entity::item(
                            &mut w,
                            8.5 + dx as f64 * offset,
                            72.5 + dy as f64 * offset,
                            8.5 + dz as f64 * offset,
                            super::super::tile::ItemStack::new(1, 1, 0),
                        );
                        e.no_clip = j == 8;
                        e.motion = crate::entity::Vec3D::new(0.1, -0.2, 0.3);
                        let id = e.entity_id;
                        w.entity_joined_world(e);
                        entities.push(id);
                    }
                    w.drain_events();
                    for _ in 0..4 {
                        let TileEntity::Piston(p) = w.tile_object(tile).unwrap().clone() else {
                            panic!()
                        };
                        w.tick_piston(tile, p);
                        let TileEntity::Piston(p) = w.tile_object(tile).unwrap() else {
                            panic!()
                        };
                        for v in [p.progress, p.last_progress] {
                            o.extend(v.to_bits().to_le_bytes());
                        }
                        o.push(w.tile_objects[&tile].invalid as u8);
                        o.push(w.get_block_id(8, 72, 8));
                        i(o, w.get_block_metadata(8, 72, 8));
                        for id in &entities {
                            let e = &w.entities[id];
                            for v in [
                                e.pos.x,
                                e.pos.y,
                                e.pos.z,
                                e.motion.x,
                                e.motion.y,
                                e.motion.z,
                                e.bounding_box.min_x,
                                e.bounding_box.min_y,
                                e.bounding_box.min_z,
                                e.bounding_box.max_x,
                                e.bounding_box.max_y,
                                e.bounding_box.max_z,
                            ] {
                                o.extend(v.to_bits().to_le_bytes());
                            }
                            o.extend([
                                e.on_ground as u8,
                                e.collided_horizontally as u8,
                                e.collided_vertically as u8,
                                e.collided as u8,
                            ]);
                            o.extend(e.fall_distance.to_bits().to_le_bytes());
                            o.extend(e.distance_walked.to_bits().to_le_bytes());
                        }
                        l(o, w.rand.state());
                        super::super::events::write_events(&w.drain_events(), o);
                    }
                }
            }
        }
    }
}
pub(super) fn unload(o: &mut Vec<u8>) {
    use super::super::tile::TileEntity;
    for block in [25, 84, 54] {
        for repeated in [false, true] {
            let mut w = fixture(true);
            seed(&mut w);
            let mut ids = Vec::new();
            for x in [8, 24] {
                raw(&mut w, x, 72, 8, block, 3);
                w.get_block_tile_entity(x, 72, 8);
                ids.push(w.chunk(x >> 4, 0).tile_entity_ids[&(x & 15, 72, 8)]);
            }
            if block == 54 {
                for id in &ids {
                    w.chest_animations.get_mut(id).unwrap().users = 1;
                }
            }
            for step in 0..7 {
                match step {
                    1 => {
                        w.unload_chunk_entities(0, 0);
                        if repeated {
                            w.unload_chunk_entities(0, 0);
                        }
                        match w.get_block_tile_entity(8, 72, 8).unwrap() {
                            TileEntity::Note(n) => n.note = 7,
                            TileEntity::RecordPlayer { record } => *record = 2256,
                            _ => {}
                        }
                    }
                    2 | 4 | 6 => w.update_entities(),
                    3 => {
                        w.chunk_mut(0, 0).loaded = true;
                        w.loaded_tile_entities.push(ids[0]);
                    }
                    5 => w.unload_chunk_entities(1, 0),
                    _ => {}
                }
                i(o, w.loaded_tile_entities.len() as i32);
                for id in &w.loaded_tile_entities {
                    let (x, y, z) = w.tile_objects[id].position;
                    for v in [x, y, z] {
                        i(o, v);
                    }
                }
                i(o, w.unloaded_tile_entities181.len() as i32);
                for id in &ids {
                    o.push(w.tile_objects[id].invalid as u8);
                    match w.tile_object(*id).unwrap() {
                        TileEntity::Note(n) => {
                            i(o, n.note as i32);
                            continue;
                        }
                        TileEntity::RecordPlayer { record } => {
                            i(o, *record);
                            continue;
                        }
                        _ => {}
                    }
                    let state = &w.chest_animations[id];
                    for v in [state.lid, state.previous_lid] {
                        o.extend(v.to_bits().to_le_bytes());
                    }
                    i(o, state.users);
                }
                l(o, w.rand.state());
                super::super::events::write_events(&w.drain_events(), o);
            }
        }
    }
}
fn cells(o: &mut Vec<u8>, w: &mut World) {
    for x in 7..=9 {
        for y in 62..=65 {
            for z in 7..=9 {
                o.push(w.get_block_id(x, y, z));
                o.push(w.get_block_metadata(x, y, z) as u8);
            }
        }
    }
}
pub(super) fn result(o: &mut Vec<u8>, w: &mut World) {
    cells(o, w);
    l(o, w.rand.state());
    i(o, w.loaded_entity_list.len() as i32);
    for id in &w.loaded_entity_list {
        let e = &w.entities[id];
        if let crate::entity::EntityKind::Item {
            item: stack,
            pickup_delay,
            ..
        } = &e.kind
        {
            i(o, stack.id as i32);
            i(o, stack.count);
            i(o, stack.damage as i32);
            i(o, *pickup_delay);
            for v in [
                e.pos.x, e.pos.y, e.pos.z, e.motion.x, e.motion.y, e.motion.z,
            ] {
                o.extend(v.to_bits().to_le_bytes());
            }
        } else {
            panic!("unexpected entity");
        }
    }
    super::super::events::write_events(&w.drain_events(), o);
}
pub(super) fn seed(w: &mut World) {
    w.rand.set_seed(42);
    w.math_random.set_seed(200);
    w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
}
pub(super) fn doors(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    let mut p = crate::entity::Entity::player(&mut w, "oracle");
    for id in [64, 71] {
        for remote in [false, true] {
            for m in 0..16 {
                for config in 0..16 {
                    for hook in 0..4 {
                        for x in 7..=9 {
                            for y in 62..=66 {
                                for z in 7..=9 {
                                    raw(&mut w, x, y, z, 0, 0);
                                }
                            }
                        }
                        raw(
                            &mut w,
                            8,
                            63,
                            8,
                            if m & 8 != 0 {
                                if config & 1 != 0 { id } else { 0 }
                            } else {
                                if config & 1 != 0 { 1 } else { 0 }
                            },
                            m & 7,
                        );
                        raw(
                            &mut w,
                            8,
                            65,
                            8,
                            if config & 2 != 0 { id } else { 0 },
                            (m & 7) | 8,
                        );
                        raw(&mut w, 7, 64, 8, if config & 4 != 0 { 76 } else { 0 }, 5);
                        raw(&mut w, 8, 64, 8, id, m);
                        w.multiplayer_world = remote;
                        seed(&mut w);
                        w.loaded_entity_list.clear();
                        w.entities.clear();
                        w.drain_events();
                        match hook {
                            0 => behavior::on_neighbor_block_change(
                                &mut w,
                                id,
                                8,
                                64,
                                8,
                                if config & 8 != 0 { 76 } else { 1 },
                            ),
                            1 | 2 => w.door_powered181(id, 8, 64, 8, hook == 2),
                            3 => {
                                o.push(p.activate_block(&mut w, 8, 64, 8) as u8);
                            }
                            _ => unreachable!(),
                        }
                        result(o, &mut w);
                    }
                }
            }
        }
    }
}
pub(super) fn slabs(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    for id in [43, 44] {
        for below in [0, 1, 43, 44] {
            for m in 0..16 {
                for n in 0..16 {
                    raw(&mut w, 8, 62, 8, 1, 0);
                    raw(&mut w, 8, 63, 8, below, n);
                    raw(&mut w, 8, 64, 8, id, m);
                    seed(&mut w);
                    behavior::on_block_added(&mut w, id, 8, 64, 8);
                    result(o, &mut w);
                }
            }
        }
    }
}
pub(super) fn attached(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    for id in [69, 77] {
        for remote in [false, true] {
            for m in 0..16 {
                for mask in 0..32 {
                    for (k, (dx, dy, dz)) in
                        [(-1, 0, 0), (1, 0, 0), (0, 0, -1), (0, 0, 1), (0, -1, 0)]
                            .into_iter()
                            .enumerate()
                    {
                        raw(
                            &mut w,
                            8 + dx,
                            64 + dy,
                            8 + dz,
                            if mask & (1 << k) != 0 { 1 } else { 0 },
                            0,
                        );
                    }
                    raw(&mut w, 8, 64, 8, id, m);
                    w.multiplayer_world = remote;
                    seed(&mut w);
                    w.loaded_entity_list.clear();
                    w.entities.clear();
                    w.drain_events();
                    behavior::on_neighbor_block_change(&mut w, id, 8, 64, 8, 1);
                    result(o, &mut w);
                }
            }
        }
    }
}
pub(super) fn drops(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    for id in 1..=109 {
        for remote in [false, true] {
            for m in 0..16 {
                for case in 0..8 {
                    w.multiplayer_world = remote;
                    seed(&mut w);
                    w.rand.set_seed(case as i64);
                    w.loaded_entity_list.clear();
                    w.entities.clear();
                    w.drain_events();
                    behavior::drop_block_with_chance(
                        &mut w,
                        id,
                        8,
                        64,
                        8,
                        m,
                        [-1.0, 0.0, 0.25, 0.75, 1.0, 2.0, f32::NAN, f32::INFINITY][case],
                    );
                    result(o, &mut w);
                }
            }
        }
    }
}
pub(super) fn trapdoors(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    for remote in [false, true] {
        for m in 0..16 {
            for support in 0..=109 {
                for change in [0, 1, 55, 76] {
                    for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        raw(&mut w, 8 + dx, 64, 8 + dz, 0, 0);
                    }
                    let (dx, dz) = [(0, 1), (0, -1), (1, 0), (-1, 0)][(m & 3) as usize];
                    raw(&mut w, 8 + dx, 64, 8 + dz, support, 0);
                    raw(&mut w, 8, 64, 8, 96, m);
                    w.multiplayer_world = remote;
                    seed(&mut w);
                    w.loaded_entity_list.clear();
                    w.entities.clear();
                    w.drain_events();
                    behavior::on_neighbor_block_change(&mut w, 96, 8, 64, 8, change);
                    result(o, &mut w);
                }
            }
        }
    }
    w.multiplayer_world = false;
    for m in 0..16 {
        for powered in [false, true] {
            for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                raw(&mut w, 8 + dx, 64, 8 + dz, 1, 0);
            }
            raw(&mut w, 8, 64, 8, 96, m);
            seed(&mut w);
            w.loaded_entity_list.clear();
            w.entities.clear();
            w.drain_events();
            w.trapdoor_powered181(8, 64, 8, powered);
            result(o, &mut w);
        }
    }
}
