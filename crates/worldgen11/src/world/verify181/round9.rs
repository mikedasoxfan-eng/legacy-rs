//! Round-nine direct original-bytecode class probes.
use super::*;
pub(super) fn bed(o: &mut Vec<u8>) {
    use crate::entity::{player::SleepStatus, Entity, Vec3D};
    use crate::world::beds181::{BedActor181, BED_OFFSETS181};
    let mut w = fixture(true);
    for remote in [false, true] {
        for hell in [false, true] {
            for m in 0..16 {
                for config in 0..16 {
                    for status in [
                        SleepStatus::Ok,
                        SleepStatus::NotPossibleHere,
                        SleepStatus::NotPossibleNow,
                        SleepStatus::TooFarAway,
                        SleepStatus::OtherProblem,
                        SleepStatus::NotSafe, // 1.0 EnumStatus.values() has six entries
                    ] {
                        for x in 6..=10 {
                            for y in 62..=65 {
                                for z in 6..=10 {
                                    raw(&mut w, x, y, z, 0, 0);
                                }
                            }
                        }
                        raw(&mut w, 8, 63, 8, 1, 0);
                        raw(&mut w, 8, 64, 8, 26, m);
                        let (dx, dz) = BED_OFFSETS181[(m & 3) as usize];
                        if config & 1 != 0 {
                            raw(&mut w, 8 + dx, 64, 8 + dz, 26, 8 | (m & 7));
                        }
                        if config & 2 != 0 {
                            raw(&mut w, 8 - dx, 64, 8 - dz, 26, m & 7);
                        }
                        w.player_entities.clear();
                        w.entities.clear();
                        let mut other = Entity::player(&mut w, "other");
                        other.player_state_mut().unwrap().sleeping = true;
                        other.player_state_mut().unwrap().bed = Some((
                            if m & 8 != 0 { 8 } else { 8 + dx },
                            64,
                            if m & 8 != 0 { 8 } else { 8 + dz },
                        ));
                        if config & 4 != 0 {
                            w.player_entities.push(Some(other.entity_id));
                        }
                        w.entities.insert(other.entity_id, other);
                        let actor = BedActor181 {
                            id: -900,
                            sleeping: config & 8 != 0,
                            bed: Some((
                                if m & 8 != 0 { 8 } else { 8 + dx },
                                64,
                                if m & 8 != 0 { 8 } else { 8 + dz },
                            )),
                        };
                        if config & 8 != 0 {
                            w.player_entities.push(Some(actor.id));
                        }
                        w.dimension = if hell {
                            crate::world::Dimension::Nether
                        } else {
                            crate::world::Dimension::Overworld
                        };
                        w.multiplayer_world = remote;
                        w.loaded_entity_list.clear();
                        round6::seed(&mut w);
                        let mut calls = Vec::new();
                        let mut blasts = Vec::new();
                        let chat = w.activate_bed181(
                            8,
                            64,
                            8,
                            Some(actor),
                            |_, x, y, z| {
                                calls.push([x, y, z]);
                                status
                            },
                            |_, pos| {
                                blasts.push(pos);
                            },
                        );
                        o.push(1);
                        round6::result(o, &mut w);
                        i(o, calls.len() as i32);
                        for call in calls {
                            for n in call {
                                i(o, n);
                            }
                        }
                        i(o, blasts.len() as i32);
                        for Vec3D { x, y, z } in blasts {
                            for v in [x, y, z, 5.0, 1.0] {
                                o.extend(v.to_bits().to_le_bytes());
                            }
                        }
                        i(o, chat.is_some() as i32);
                        if let Some(s) = chat {
                            i(o, s.len() as i32);
                            o.extend(s.as_bytes());
                        }
                    }
                }
            }
        }
    }
    for remote in [false, true] {
        for m in 0..16 {
            for paired in [0, 1, 26] {
                for x in 7..=9 {
                    for z in 7..=9 {
                        raw(&mut w, x, 64, z, 0, 0);
                    }
                }
                raw(&mut w, 8, 64, 8, 26, m);
                let (dx, dz) = BED_OFFSETS181[(m & 3) as usize];
                raw(
                    &mut w,
                    8 + if m & 8 != 0 { -dx } else { dx },
                    64,
                    8 + if m & 8 != 0 { -dz } else { dz },
                    paired,
                    0,
                );
                w.multiplayer_world = remote;
                w.loaded_entity_list.clear();
                round6::seed(&mut w);
                behavior::on_neighbor_block_change(&mut w, 26, 8, 64, 8, 1);
                round6::result(o, &mut w);
            }
        }
    }
    for m in 0..16 {
        for occupied in [false, true] {
            raw(&mut w, 8, 64, 8, 26, m);
            w.set_bed_occupied181(8, 64, 8, occupied);
            i(o, w.get_block_metadata(8, 64, 8));
        }
    }
    for m in 0..16 {
        for mask in 0..16 {
            for skip in [i32::MIN, -1, 0, 1, 8, 9, 17, 18, i32::MAX] {
                for x in 6..=10 {
                    for z in 6..=10 {
                        raw(
                            &mut w,
                            x,
                            63,
                            z,
                            if (x + z) & 1 == 0 && mask & 1 != 0 {
                                0
                            } else {
                                1
                            },
                            0,
                        );
                        raw(
                            &mut w,
                            x,
                            64,
                            z,
                            if mask & 2 != 0 && x == 8 { 1 } else { 0 },
                            0,
                        );
                        raw(
                            &mut w,
                            x,
                            65,
                            z,
                            if mask & 4 != 0 && z == 8 { 1 } else { 0 },
                            0,
                        );
                    }
                }
                raw(&mut w, 8, 64, 8, 26, m);
                let coords = Entity::bed_exit(&mut w, 8, 64, 8, skip);
                o.push(coords.is_some() as u8);
                if let Some((x, y, z)) = coords {
                    for n in [x, y, z] {
                        i(o, n);
                    }
                }
            }
        }
    }
    for m in 0..16 {
        raw(&mut w, 8, 64, 8, 26, m);
        w.block_bounds[26] = BB::new(-0.25, 0.0, 0.25, 1.25, 1.0, 0.75);
        b(o, Some(w.block_bounds[26].translated(8.0, 64.0, 8.0)));
        b(o, w.block_collision_box(26, 8, 64, 8));
        w.set_block_bounds_based_on_state(26, 8, 64, 8);
        b(o, Some(w.block_bounds[26]));
    }
    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for matching_first in [false, true] {
        w.dimension = crate::world::Dimension::Overworld;
        w.multiplayer_world = false;
        w.player_entities.clear();
        w.entities.clear();
        let mut other = Entity::player(&mut w, "other");
        other.player_state_mut().unwrap().sleeping = true;
        other.player_state_mut().unwrap().bed = Some((8, 64, 8));
        let actor = BedActor181 {
            id: -900,
            sleeping: true,
            bed: None,
        };
        let ids = if matching_first {
            [other.entity_id, actor.id]
        } else {
            [actor.id, other.entity_id]
        };
        w.player_entities.extend(ids.map(Some));
        w.entities.insert(other.entity_id, other);
        raw(&mut w, 8, 64, 8, 26, 12);
        w.loaded_entity_list.clear();
        round6::seed(&mut w);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            w.activate_bed181(
                8,
                64,
                8,
                Some(actor),
                |_, _, _, _| panic!("unexpected sleep"),
                |_, _| panic!("unexpected explosion"),
            )
        }));
        o.push(result.is_ok() as u8);
        round6::result(o, &mut w);
    }
    std::panic::set_hook(old_hook);
}
pub(super) fn extension(o: &mut Vec<u8>) {
    use crate::world::pistons::DIRECTIONS;
    let mut w = fixture(true);
    let values = [
        i32::MIN,
        -100,
        -1,
        0,
        1,
        2,
        3,
        4,
        5,
        6,
        7,
        8,
        15,
        16,
        100,
        i32::MAX,
    ];
    let old_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    for texture in values {
        w.set_piston_extension_texture181(texture);
        for m in values {
            for side in values {
                match w.piston_extension_texture181(side, m) {
                    Ok(t) => {
                        o.push(1);
                        i(o, t);
                    }
                    Err(()) => o.push(0),
                }
            }
        }
        w.clear_piston_extension_texture181();
        for side in 0..6 {
            i(o, w.piston_extension_texture181(side, side).unwrap());
        }
    }
    for remote in [false, true] {
        for face in 0..6 {
            for base in [0, 1, 29, 33] {
                for bm in 0..16 {
                    for hook in 0..2 {
                        for x in 6..=10 {
                            for y in 61..=67 {
                                for z in 6..=10 {
                                    raw(&mut w, x, y, z, 0, 0);
                                }
                            }
                        }
                        let (dx, dy, dz) = DIRECTIONS[face];
                        raw(&mut w, 8 - dx, 64 - dy, 8 - dz, base, bm);
                        raw(&mut w, 8, 64, 8, 34, face as i32 | 8);
                        w.multiplayer_world = remote;
                        w.loaded_entity_list.clear();
                        round6::seed(&mut w);
                        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                            if hook == 0 {
                                behavior::on_block_removal(&mut w, 34, 8, 64, 8);
                            } else {
                                behavior::on_neighbor_block_change(&mut w, 34, 8, 64, 8, 1);
                            }
                        }));
                        o.push(result.is_ok() as u8);
                        round6::result(o, &mut w);
                    }
                }
            }
        }
    }
    std::panic::set_hook(old_hook);
    w.chunks.insert(
        (16777216 >> 4, 0),
        Box::new(Chunk::new(16777216 >> 4, 0, Box::new([0; 32768]))),
    );
    for x in [8, -17, 16777216] {
        for m in 0..16 {
            for shape in 0..2 {
                raw(&mut w, x, 64, 8, 34, m);
                w.block_bounds[34] = BB::new(
                    if shape == 0 { 0.25 } else { -0.25 },
                    0.0,
                    0.25,
                    if shape == 0 { 0.75 } else { 1.25 },
                    1.0,
                    0.75,
                );
                b(o, Some(w.piston_extension_selected_box181(x, 64, 8)));
                b(o, w.block_collision_box(34, x, 64, 8));
                b(o, Some(w.block_bounds[34]));
                w.set_block_bounds_based_on_state(34, x, 64, 8);
                b(o, Some(w.block_bounds[34]));
                for offset in [-0.5, 0.0, 0.25, 0.5, 0.75, 1.0, 1.5] {
                    let boxes = w.piston_extension_boxes181(
                        x,
                        64,
                        8,
                        BB::new(
                            x as f64 + offset,
                            64.0 + offset,
                            8.0 + offset,
                            x as f64 + offset + 0.25,
                            64.0 + offset + 0.25,
                            8.0 + offset + 0.25,
                        ),
                    );
                    i(o, boxes.len() as i32);
                    for bb in boxes {
                        b(o, Some(bb));
                    }
                    b(o, Some(w.block_bounds[34]));
                }
            }
        }
    }
}
pub(super) fn door(o: &mut Vec<u8>) {
    use crate::block::visual181;
    use crate::entity::{Entity, MovingObjectPosition, Vec3D};
    let mut w = fixture(true);
    w.record_block_update_ranges181 = true;
    let mut p = Entity::player(&mut w, "oracle");
    let values = [
        i32::MIN,
        -100,
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
        100,
        i32::MAX,
    ];
    for id in [64, 71] {
        for m in values {
            i(o, visual181::door_state181(m));
            o.push((m & 4 != 0) as u8);
            w.set_door_rotation181(id, m);
            b(o, Some(w.block_bounds[id as usize]));
            for side in values {
                i(o, visual181::door_texture181(id, side, m));
            }
        }
        for remote in [false, true] {
            for m in 0..16 {
                for config in 0..16 {
                    for hook in 0..5 {
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
                                if config & 1 != 0 {
                                    id
                                } else {
                                    0
                                }
                            } else {
                                if config & 1 != 0 {
                                    1
                                } else {
                                    0
                                }
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
                        w.loaded_entity_list.clear();
                        round6::seed(&mut w);
                        w.block_update_ranges181.clear();
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
                            3 => o.push(p.activate_block(&mut w, 8, 64, 8) as u8),
                            _ => p.block_clicked(&mut w, id, 8, 64, 8),
                        }
                        round6::result(o, &mut w);
                        ranges(o, &mut w);
                    }
                }
            }
        }
        w.chunks.insert(
            (16777216 >> 4, 0),
            Box::new(Chunk::new(16777216 >> 4, 0, Box::new([0; 32768]))),
        );
        for x in [8, -17, 16777216] {
            for m in 0..16 {
                raw(&mut w, x, 64, 8, id, m);
                w.set_door_rotation181(id, -1);
                b(o, Some(w.door_selected_box181(id, x, 64, 8)));
                b(o, Some(w.block_bounds[id as usize]));
                b(o, w.block_collision_box(id, x, 64, 8));
                b(o, Some(w.block_bounds[id as usize]));
                for axis in 0..3 {
                    for offset in [0.0, 0.1875, 0.5, 0.8125, 1.0] {
                        for reverse in [false, true] {
                            let mut s = [x as f64 + offset, 64.0 + offset, 8.0 + offset];
                            let mut e = s;
                            s[axis] = [x as f64, 64.0, 8.0][axis] - 1.0;
                            e[axis] = [x as f64, 64.0, 8.0][axis] + 2.0;
                            let start = Vec3D::new(s[0], s[1], s[2]);
                            let end = Vec3D::new(e[0], e[1], e[2]);
                            let hit = w.collision_ray_trace(
                                x,
                                64,
                                8,
                                if reverse { end } else { start },
                                if reverse { start } else { end },
                                false,
                                false,
                            );
                            o.push(hit.is_some() as u8);
                            if let Some(MovingObjectPosition::Block { x, y, z, side, hit }) = hit {
                                for n in [x, y, z, side] {
                                    i(o, n);
                                }
                                for v in [hit.x, hit.y, hit.z] {
                                    o.extend(v.to_bits().to_le_bytes());
                                }
                            }
                            b(o, Some(w.block_bounds[id as usize]));
                        }
                    }
                }
            }
        }
    }
}
fn queue(o: &mut Vec<u8>, w: &World) {
    i(o, w.tick_order.len() as i32);
    for &(time, _, key) in &w.tick_order {
        for v in [key.x, key.y, key.z, key.block] {
            i(o, v);
        }
        l(o, time);
    }
}
pub(super) fn plate(o: &mut Vec<u8>) {
    use crate::entity::Entity;
    let mut w = fixture(true);
    w.record_block_update_ranges181 = true;
    for id in [70, 72] {
        w.set_plate_item_bounds181(id);
        b(o, Some(w.block_bounds[id as usize]));
        for m in 0..16 {
            raw(&mut w, 8, 64, 8, id, m);
            w.set_block_bounds_based_on_state(id, 8, 64, 8);
            b(o, Some(w.block_bounds[id as usize]));
            for side in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, i32::MAX] {
                o.push(w.block_power(8, 64, 8, side, false) as u8);
                o.push(w.block_power(8, 64, 8, side, true) as u8);
            }
        }
        for remote in [false, true] {
            for m in 0..16 {
                for kind in 0..4 {
                    for position in 0..9 {
                        for dead in [false, true] {
                            for hook in 0..3 {
                                w.loaded_entity_list.clear();
                                w.entities.clear();
                                w.players.clear();
                                w.player_entities.clear();
                                for c in w.chunks.values_mut() {
                                    for list in &mut c.entity_lists {
                                        list.clear();
                                    }
                                }
                                for x in 7..=9 {
                                    for y in 62..=65 {
                                        for z in 7..=9 {
                                            raw(&mut w, x, y, z, 0, 0);
                                        }
                                    }
                                }
                                raw(&mut w, 8, 63, 8, 1, 0);
                                raw(&mut w, 8, 64, 8, id, m);
                                w.multiplayer_world = remote;
                                round6::seed(&mut w);
                                w.tick_order.clear();
                                w.pending_ticks.clear();
                                w.block_update_ranges181.clear();
                                let mut e = if kind == 2 {
                                    Entity::player(&mut w, "oracle")
                                } else {
                                    crate::entity::registry::create(
                                        &mut w,
                                        ["Pig", "Item", "Player", "Snowball"][kind],
                                    )
                                    .unwrap()
                                };
                                let ps = [
                                    [8.5, 64.0, 8.5],
                                    [7.5, 64.0, 8.5],
                                    [9.5, 64.0, 8.5],
                                    [8.5, 64.0, 7.5],
                                    [8.5, 64.0, 9.5],
                                    [8.5, 63.0, 8.5],
                                    [8.5, 64.25, 8.5],
                                    [8.5, 64.249999, 8.5],
                                    [8.5, 64.5, 8.5],
                                ][position];
                                e.set_position(ps[0], ps[1], ps[2]);
                                w.entity_joined_world(e.clone());
                                e.dead = dead;
                                w.entities.get_mut(&e.entity_id).unwrap().dead = dead;
                                if hook == 0 {
                                    e.on_entity_collided_with_block(&mut w, id, 8, 64, 8);
                                } else if hook == 1 {
                                    behavior::update_tick(&mut w, id, 8, 64, 8, RandSel::World);
                                } else {
                                    behavior::on_block_removal(&mut w, id, 8, 64, 8);
                                }
                                for x in 7..=9 {
                                    for y in 62..=65 {
                                        for z in 7..=9 {
                                            o.push(w.get_block_id(x, y, z));
                                            o.push(w.get_block_metadata(x, y, z) as u8);
                                        }
                                    }
                                }
                                l(o, w.rand.state());
                                crate::world::events::write_events(&w.drain_events(), o);
                                queue(o, &w);
                                ranges(o, &mut w);
                            }
                        }
                    }
                }
            }
        }
    }
}
pub(super) fn ranges(o: &mut Vec<u8>, w: &mut World) {
    let ranges = w.drain_block_update_ranges181();
    i(o, ranges.len() as i32);
    for range in ranges {
        for n in range {
            i(o, n);
        }
    }
}
pub(super) fn cake(o: &mut Vec<u8>) {
    use crate::block::visual181;
    let mut w = fixture(true);
    w.record_block_update_ranges181 = true;
    let mut p = crate::entity::Entity::player(&mut w, "oracle");
    let values = [
        i32::MIN,
        -100,
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
        100,
        i32::MAX,
    ];
    w.set_cake_item_bounds181();
    b(o, Some(w.block_bounds[92]));
    for m in values {
        b(o, Some(visual181::cake_bounds181(m)));
        for side in values {
            i(o, visual181::cake_texture181(side, 0));
            i(o, visual181::cake_texture181(side, m));
        }
        w.rand.set_seed(91);
        i(o, behavior::id_dropped(&mut w, 92, m));
        i(o, behavior::quantity_dropped(&mut w, 92));
        l(o, w.rand.state());
    }
    for floor in 0..=109 {
        for target in 0..=109 {
            raw(&mut w, 8, 63, 8, floor, 0);
            raw(&mut w, 8, 64, 8, target, 0);
            o.push(crate::entity::player::placement::can_place_at(&mut w, 92, 8, 64, 8, 1) as u8);
            o.push(crate::block::mat(w.get_block_material(8, 63, 8)).is_solid as u8);
        }
    }
    for remote in [false, true] {
        for floor in 0..=109 {
            for m in 0..16 {
                raw(&mut w, 8, 63, 8, floor, 0);
                raw(&mut w, 8, 64, 8, 92, m);
                w.multiplayer_world = remote;
                w.loaded_entity_list.clear();
                round6::seed(&mut w);
                w.block_update_ranges181.clear();
                behavior::on_neighbor_block_change(&mut w, 92, 8, 64, 8, 1);
                round6::result(o, &mut w);
                ranges(o, &mut w);
            }
        }
    }
    for remote in [false, true] {
        for m in 0..16 {
            for food in [0, 19, 20] {
                for creative in [false, true] {
                    for hook in 0..2 {
                        raw(&mut w, 8, 63, 8, 1, 0);
                        raw(&mut w, 8, 64, 8, 92, m);
                        w.multiplayer_world = remote;
                        p.player_state_mut().unwrap().food.write().level = food;
                        p.player_state_mut().unwrap().food.write().saturation = 4.5;
                        p.player_state_mut().unwrap().capabilities.disable_damage = creative;
                        w.loaded_entity_list.clear();
                        round6::seed(&mut w);
                        w.block_update_ranges181.clear();
                        if hook == 0 {
                            p.block_clicked(&mut w, 92, 8, 64, 8);
                        } else {
                            o.push(p.activate_block(&mut w, 8, 64, 8) as u8);
                        }
                        i(o, p.player_state().unwrap().food.read().level);
                        o.extend(
                            p.player_state()
                                .unwrap()
                                .food.read()
                                .saturation
                                .to_bits()
                                .to_le_bytes(),
                        );
                        round6::result(o, &mut w);
                        ranges(o, &mut w);
                    }
                }
            }
        }
    }
    for x in [8, -17, 16777216, -16777217, i32::MIN, i32::MAX] {
        for y in [-1, 64, 127, 128, 16777216] {
            for m in 0..16 {
                if (0..128).contains(&y) {
                    w.chunks.insert(
                        (x >> 4, 0),
                        Box::new(Chunk::new(x >> 4, 0, Box::new([0; 32768]))),
                    );
                    raw(&mut w, x, y, 8, 92, m);
                }
                w.set_cake_item_bounds181();
                b(o, w.block_collision_box(92, x, y, 8));
                b(o, Some(w.block_bounds[92]));
                b(o, Some(w.cake_selected_box181(x, y, 8)));
                b(o, Some(w.block_bounds[92]));
            }
        }
    }
}
