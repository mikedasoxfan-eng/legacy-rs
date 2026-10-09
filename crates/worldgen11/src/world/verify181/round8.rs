//! Round-eight class completion probes. Production callbacks mirror original bytecode.
use super::*;
use crate::entity::{Entity, MovingObjectPosition, Vec3D};
pub(super) fn sign(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    for id in [63, 68] {
        o.push(matches!(
            crate::world::tile::TileEntity::for_block(id),
            crate::world::tile::TileEntity::Sign(_)
        ) as u8);
        raw(&mut w, 8, 64, 8, id, 0);
        w.container_added181(id, 8, 64, 8);
        let old = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
        w.container_added181(id, 8, 64, 8);
        o.push((w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)] != old) as u8);
        for remote in [false, true] {
            for m in 0..16 {
                for support in 0..=109 {
                    w.chunk_mut(0, 0).tile_entities.clear();
                    w.chunk_mut(0, 0).tile_entity_ids.clear();
                    w.tile_objects.clear();
                    w.loaded_tile_entities.clear();
                    for x in 7..=9 {
                        for y in 63..=65 {
                            for z in 7..=9 {
                                raw(&mut w, x, y, z, 0, 0);
                            }
                        }
                    }
                    raw(&mut w, 8, 63, 8, support, 0);
                    for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        raw(&mut w, 8 + dx, 64, 8 + dz, support, 0);
                    }
                    raw(&mut w, 8, 64, 8, id, m);
                    w.multiplayer_world = remote;
                    w.loaded_entity_list.clear();
                    round6::seed(&mut w);
                    behavior::on_block_added(&mut w, id, 8, 64, 8);
                    w.get_block_tile_entity(8, 64, 8);
                    let old = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
                    behavior::on_neighbor_block_change(&mut w, id, 8, 64, 8, 1);
                    round6::result(o, &mut w);
                    o.push(w.tile_objects[&old].invalid as u8);
                    w.get_block_tile_entity(8, 64, 8);
                    o.push((w.chunk(0, 0).tile_entity_ids.get(&(8, 64, 8)) == Some(&old)) as u8);
                }
            }
        }
    }
    w.chunks.insert(
        (16777216 >> 4, 0),
        Box::new(Chunk::new(16777216 >> 4, 0, Box::new([0; 32768]))),
    );
    for id in [63, 68] {
        for x in [8, -17, 16777216] {
            for m in 0..16 {
                for shape in 0..2 {
                    raw(&mut w, x, 64, 8, id, m);
                    w.block_bounds[id as usize] = BB::new(
                        if shape == 0 { 0.25 } else { -0.25 },
                        0.0,
                        0.25,
                        if shape == 0 { 0.75 } else { 1.25 },
                        1.0,
                        0.75,
                    );
                    let bb = w.sign_selected_box181(id, x, 64, 8);
                    b(o, Some(bb));
                    bounds(o, &w, id);
                    b(o, w.block_collision_box(id, x, 64, 8));
                    w.set_block_bounds_based_on_state(id, x, 64, 8);
                    bounds(o, &w, id);
                }
            }
        }
    }
}
pub(super) fn spawner(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    w.rand.set_seed(91);
    o.push(crate::block::opaque_cube_lookup(52) as u8);
    for m in [i32::MIN, -1, 0, 1, 2, 15, 16, i32::MAX] {
        for _ in 0..32 {
            i(o, behavior::id_dropped(&mut w, 52, m));
            i(o, behavior::quantity_dropped(&mut w, 52));
            l(o, w.rand.state());
            let crate::world::tile::TileEntity::MobSpawner { mob, delay } =
                crate::world::tile::TileEntity::for_block(52)
            else {
                panic!("spawner factory");
            };
            i(o, delay);
            i(o, mob.len() as i32);
            o.extend(mob.as_bytes());
            o.extend(0.0f64.to_bits().to_le_bytes());
            o.extend(0.0f64.to_bits().to_le_bytes());
            raw(&mut w, 8, 64, 8, 52, 0);
            w.container_added181(52, 8, 64, 8);
            let old = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
            w.container_added181(52, 8, 64, 8);
            o.push((w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)] != old) as u8);
        }
    }
}
pub(super) fn stairs(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    let mut p = Entity::player(&mut w, "oracle");
    let mut r = crate::JavaRandom::new(91);
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
    for id in [53, 67, 108, 109] {
        let model = crate::block::visual181::stairs_model181(id).unwrap();
        for side in values {
            for m in values {
                i(o, crate::block::visual181::stairs_texture181(id, side, 0));
                i(o, crate::block::visual181::stairs_texture181(id, side, m));
            }
        }
        let mut calls = Vec::new();
        let brightness =
            crate::block::visual181::base_brightness181(id, -17, 64, 8, |x, y, z, min| {
                calls.push([x, y, z, min]);
                0.625
            });
        let packed =
            crate::block::visual181::base_packed_light181(id, -17, 64, 8, |x, y, z, min| {
                calls.push([x, y, z, min]);
                x.wrapping_mul(31) ^ y.wrapping_mul(17) ^ z.wrapping_mul(7) ^ min
            });
        f181(o, brightness);
        i(o, packed);
        i(o, calls.len() as i32);
        for c in calls {
            for v in c {
                i(o, v);
            }
        }
        f181(o, crate::entity::block_table::RESISTANCE[id as usize]);
        for x in [8, -17, 16777216] {
            for _m in 0..16 {
                for shape in 0..3 {
                    w.block_bounds[model as usize] = BB::new(
                        if shape == 0 {
                            0.0
                        } else if shape == 1 {
                            -0.25
                        } else {
                            0.125
                        },
                        0.0,
                        0.0,
                        if shape == 0 {
                            1.0
                        } else if shape == 1 {
                            1.25
                        } else {
                            0.875
                        },
                        1.0,
                        1.0,
                    );
                    w.block_bounds[id as usize] = BB::new(0.25, 0.25, 0.25, 0.75, 0.75, 0.75);
                    b(o, Some(w.stairs_selected_box181(id, x, 64, 8)));
                    bounds(o, &w, id);
                }
            }
        }
        w.block_bounds[model as usize] = BB::new(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        for remote in [false, true] {
            for m in 0..16 {
                for hook in 0..6 {
                    raw(&mut w, 8, 64, 8, id, m);
                    w.multiplayer_world = remote;
                    w.loaded_entity_list.clear();
                    round6::seed(&mut w);
                    match hook {
                        0 => behavior::on_block_added(&mut w, id, 8, 64, 8),
                        1 => behavior::on_block_removal(&mut w, id, 8, 64, 8),
                        2 => p.block_clicked(&mut w, id, 8, 64, 8),
                        3 => behavior::update_tick(&mut w, id, 8, 64, 8, RandSel::World),
                        4 => o.push(p.activate_block(&mut w, 8, 64, 8) as u8),
                        _ => p.on_entity_walking(&mut w, id, 8, 64, 8),
                    }
                    round6::result(o, &mut w);
                    queue(o, &w);
                    w.block_display_tick(id, 8, 64, 8, &mut r);
                    l(o, r.state());
                    super::super::events::write_events(&w.drain_events(), o);
                }
            }
        }
        for yaw in [
            f32::NEG_INFINITY,
            -f32::MAX,
            -360.0,
            -45.0,
            0.0,
            45.0,
            180.0,
            360.0,
            f32::MAX,
            f32::INFINITY,
            f32::NAN,
        ] {
            raw(&mut w, 8, 64, 8, id, 15);
            p.yaw = yaw;
            round6::seed(&mut w);
            p.block_placed(&mut w, id, 8, 64, 8, 1);
            round6::result(o, &mut w);
        }
    }
}
pub(super) fn wire(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    let ds = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for floor in 0..=109 {
        for target in [0, 1, 31, 55] {
            raw(&mut w, 8, 63, 8, floor, 0);
            raw(&mut w, 8, 64, 8, target, 0);
            o.push(crate::entity::player::placement::can_place_at(&mut w, 55, 8, 64, 8, 1) as u8);
        }
    }
    for id in 0..=109 {
        for m in 0..16 {
            for side in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, i32::MAX] {
                raw(&mut w, 8, 64, 8, id, m);
                o.push(w.wire_neighbor(8, 64, 8, side) as u8);
            }
        }
    }
    for flag in [false, true] {
        for m in 0..16 {
            for roof in 0..2 {
                for config in 0..256 {
                    raw(&mut w, 8, 64, 8, 55, m);
                    raw(&mut w, 8, 65, 8, roof, 0);
                    let mut v = config;
                    for (dx, dz) in ds {
                        let ty = v & 3;
                        v >>= 2;
                        for dy in -1..=1 {
                            raw(&mut w, 8 + dx, 64 + dy, 8 + dz, 0, 0);
                        }
                        if ty == 1 {
                            raw(&mut w, 8 + dx, 64, 8 + dz, 55, 0);
                        }
                        if ty == 2 {
                            raw(&mut w, 8 + dx, 64, 8 + dz, 1, 0);
                            raw(&mut w, 8 + dx, 65, 8 + dz, 55, 0);
                        }
                        if ty == 3 {
                            raw(&mut w, 8 + dx, 63, 8 + dz, 55, 0);
                        }
                    }
                    w.wires_provide_power = flag;
                    o.push(w.wires_provide_power as u8);
                    for side in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, i32::MAX] {
                        o.push(w.block_power(8, 64, 8, side, false) as u8);
                        o.push(w.block_power(8, 64, 8, side, true) as u8);
                    }
                }
            }
        }
    }
    w.wires_provide_power = true;
    for remote in [false, true] {
        for m in 0..16 {
            for mask in 0..16 {
                for source in [false, true] {
                    for hook in 0..3 {
                        clear_repeater(&mut w);
                        for x in 6..=10 {
                            for y in 61..=66 {
                                for z in 6..=10 {
                                    raw(&mut w, x, y, z, if y == 62 { 1 } else { 0 }, 0);
                                }
                            }
                        }
                        raw(&mut w, 8, 63, 8, if mask == 15 { 0 } else { 1 }, 0);
                        raw(&mut w, 8, 64, 8, 55, m);
                        for (k, (dx, dz)) in ds.into_iter().enumerate() {
                            raw(&mut w, 8 + dx, 63, 8 + dz, 1, 0);
                            let up = mask & (1 << k) != 0;
                            raw(
                                &mut w,
                                8 + dx,
                                64,
                                8 + dz,
                                if up { 1 } else { 55 },
                                (m + 7) & 15,
                            );
                            if up {
                                raw(&mut w, 8 + dx, 65, 8 + dz, 55, (m + 7) & 15);
                            }
                        }
                        raw(&mut w, 8, 63, 6, 1, 0);
                        raw(&mut w, 8, 64, 6, if source { 76 } else { 0 }, 5);
                        w.multiplayer_world = remote;
                        round6::seed(&mut w);
                        match hook {
                            0 => behavior::on_block_added(&mut w, 55, 8, 64, 8),
                            1 => behavior::on_neighbor_block_change(&mut w, 55, 8, 64, 8, 76),
                            _ => behavior::on_block_removal(&mut w, 55, 8, 64, 8),
                        }
                        round6::result(o, &mut w);
                        queue(o, &w);
                    }
                }
            }
        }
    }
    w.chunks.insert(
        (16777216 >> 4, 0),
        Box::new(Chunk::new(16777216 >> 4, 0, Box::new([0; 32768]))),
    );
    let mut r = crate::JavaRandom::new(91);
    for x in [8, -17, 16777216] {
        for m in 0..16 {
            for _ in 0..16 {
                raw(&mut w, x, 64, 8, 55, m);
                round6::seed(&mut w);
                w.block_display_tick(55, x, 64, 8, &mut r);
                l(o, r.state());
                l(o, w.rand.state());
                super::super::events::write_events(&w.drain_events(), o);
            }
        }
    }
}
pub(super) fn cactus(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    for side in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, i32::MAX] {
        i(o, crate::block::visual181::cactus_texture181(side));
    }
    for x in [
        i32::MIN,
        -30000001,
        -16777217,
        -17,
        0,
        8,
        16777216,
        30000000,
        i32::MAX,
    ] {
        for y in [i32::MIN, -1, 0, 64, 127, 16777216, i32::MAX] {
            for z in [i32::MIN, -17, 8, 16777216, i32::MAX] {
                b(o, w.block_collision_box(81, x, y, z));
                b(
                    o,
                    Some(crate::block::visual181::cactus_box181(x, y, z, true)),
                );
            }
        }
    }
    for remote in [false, true] {
        for floor in 0..=109 {
            for neighbor in 0..=109 {
                for side in 0..4 {
                    w.chunk_mut(0, 0).tile_entities.clear();
                    w.chunk_mut(0, 0).tile_entity_ids.clear();
                    w.tile_objects.clear();
                    w.loaded_tile_entities.clear();
                    let ds = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dx, dz) in ds {
                        raw(&mut w, 8 + dx, 64, 8 + dz, 0, 0);
                    }
                    let (dx, dz) = ds[side];
                    raw(&mut w, 8 + dx, 64, 8 + dz, neighbor, 0);
                    raw(&mut w, 8, 63, 8, floor, 0);
                    raw(&mut w, 8, 64, 8, 81, 15);
                    w.multiplayer_world = remote;
                    w.loaded_entity_list.clear();
                    round6::seed(&mut w);
                    o.push(behavior::cactus_can_stay(&mut w, 8, 64, 8) as u8);
                    behavior::on_neighbor_block_change(&mut w, 81, 8, 64, 8, 1);
                    round6::result(o, &mut w);
                }
            }
        }
    }
    for remote in [false, true] {
        for m in 0..16 {
            for height in 1..=5 {
                for above in [0, 1, 81] {
                    for x in 7..=9 {
                        for y in 58..=69 {
                            for z in 7..=9 {
                                raw(&mut w, x, y, z, 0, 0);
                            }
                        }
                    }
                    let bottom = 65 - height;
                    raw(&mut w, 8, bottom - 1, 8, 12, 0);
                    for y in bottom..=64 {
                        raw(&mut w, 8, y, 8, 81, if y == 64 { m } else { 0 });
                    }
                    raw(&mut w, 8, 65, 8, above, 0);
                    w.multiplayer_world = remote;
                    w.loaded_entity_list.clear();
                    round6::seed(&mut w);
                    for _ in 0..32 {
                        behavior::update_tick(&mut w, 81, 8, 64, 8, RandSel::World);
                        for x in 7..=9 {
                            for y in 58..=69 {
                                for z in 7..=9 {
                                    o.push(w.get_block_id(x, y, z));
                                    o.push(w.get_block_metadata(x, y, z) as u8);
                                }
                            }
                        }
                        l(o, w.rand.state());
                        super::super::events::write_events(&w.drain_events(), o);
                    }
                }
            }
        }
    }
    for remote in [false, true] {
        for health in [-1, 1, 10, 20] {
            for immunity in [0, 11, 20] {
                w.multiplayer_world = remote;
                w.loaded_entity_list.clear();
                round6::seed(&mut w);
                let mut e = Entity::living(&mut w, "Pig").unwrap();
                e.set_position(8.0, 64.0, 8.0);
                e.living.health = health;
                e.living.hearts_life = immunity;
                e.living.last_damage = 0;
                for _ in 0..3 {
                    e.on_entity_collided_with_block(&mut w, 81, 8, 64, 8);
                    for v in [
                        e.living.health,
                        e.living.prev_health,
                        e.living.hearts_life,
                        e.living.hurt_time,
                        e.living.max_hurt_time,
                        e.living.last_damage,
                    ] {
                        i(o, v);
                    }
                    o.extend([e.been_attacked as u8, e.dead as u8]);
                    l(o, e.rand.state());
                    round6::result(o, &mut w);
                }
            }
        }
    }
}
pub(super) fn reed(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    for remote in [false, true] {
        for floor in 0..=109 {
            for mask in 0..16 {
                for m in [0, 15] {
                    for (k, (dx, dz)) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().enumerate()
                    {
                        raw(
                            &mut w,
                            8 + dx,
                            63,
                            8 + dz,
                            if mask & (1 << k) == 0 {
                                0
                            } else if k & 1 == 0 {
                                8
                            } else {
                                9
                            },
                            0,
                        );
                    }
                    raw(&mut w, 8, 63, 8, floor, 0);
                    raw(&mut w, 8, 64, 8, 83, m);
                    w.multiplayer_world = remote;
                    w.loaded_entity_list.clear();
                    round6::seed(&mut w);
                    o.push(
                        crate::entity::player::placement::can_place_at(&mut w, 83, 8, 64, 8, 1)
                            as u8,
                    );
                    o.push(behavior::reed_can_place(&mut w, 8, 64, 8) as u8);
                    behavior::on_neighbor_block_change(&mut w, 83, 8, 64, 8, 1);
                    round6::result(o, &mut w);
                }
            }
        }
    }
    for remote in [false, true] {
        for m in 0..16 {
            for height in 1..=5 {
                for above in [0, 1, 83] {
                    for x in 7..=9 {
                        for y in 58..=69 {
                            for z in 7..=9 {
                                raw(&mut w, x, y, z, 0, 0);
                            }
                        }
                    }
                    let bottom = 65 - height;
                    raw(&mut w, 8, bottom - 1, 8, 12, 0);
                    raw(&mut w, 7, bottom - 1, 8, 9, 0);
                    for y in bottom..=64 {
                        raw(&mut w, 8, y, 8, 83, if y == 64 { m } else { 0 });
                    }
                    raw(&mut w, 8, 65, 8, above, 0);
                    w.multiplayer_world = remote;
                    w.loaded_entity_list.clear();
                    round6::seed(&mut w);
                    for _ in 0..32 {
                        behavior::update_tick(&mut w, 83, 8, 64, 8, RandSel::World);
                        for x in 7..=9 {
                            for y in 58..=69 {
                                for z in 7..=9 {
                                    o.push(w.get_block_id(x, y, z));
                                    o.push(w.get_block_metadata(x, y, z) as u8);
                                }
                            }
                        }
                        l(o, w.rand.state());
                        super::super::events::write_events(&w.drain_events(), o);
                    }
                }
            }
        }
    }
}
fn bounds(o: &mut Vec<u8>, w: &World, id: u8) {
    b(o, Some(w.block_bounds[id as usize]));
}
fn hit(o: &mut Vec<u8>, h: Option<MovingObjectPosition>) {
    o.push(h.is_some() as u8);
    if let Some(MovingObjectPosition::Block { x, y, z, side, hit }) = h {
        for v in [x, y, z, side] {
            i(o, v);
        }
        for v in [hit.x, hit.y, hit.z] {
            o.extend(v.to_bits().to_le_bytes());
        }
    }
}
fn clear_repeater(w: &mut World) {
    w.tick_order.clear();
    w.pending_ticks.clear();
    w.loaded_entity_list.clear();
    for x in 7..=9 {
        for y in 63..=65 {
            for z in 7..=9 {
                raw(w, x, y, z, 0, 0);
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
pub(super) fn repeater(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    let mut p = Entity::player(&mut w, "oracle");
    for id in [93, 94] {
        for floor in 0..=109 {
            for target in [0, 1, 31, 78, 93] {
                raw(&mut w, 8, 63, 8, floor, 0);
                raw(&mut w, 8, 64, 8, target, 0);
                o.push(
                    crate::entity::player::placement::can_place_at(&mut w, id, 8, 64, 8, 1) as u8,
                );
                o.push(w.is_block_normal_cube(8, 63, 8) as u8);
            }
        }
    }
    for id in [93, 94] {
        for remote in [false, true] {
            for m in 0..16 {
                for config in 0..8 {
                    for hook in 0..6 {
                        clear_repeater(&mut w);
                        raw(&mut w, 8, 63, 8, if config & 1 == 0 { 0 } else { 1 }, 0);
                        raw(&mut w, 8, 64, 8, id, m);
                        let (dx, dz) = [(0, 1), (-1, 0), (0, -1), (1, 0)][(m & 3) as usize];
                        raw(
                            &mut w,
                            8 + dx,
                            64,
                            8 + dz,
                            [0, 55, 76, 94][config >> 1],
                            if config >> 1 == 1 {
                                15
                            } else if config >> 1 == 2 {
                                5
                            } else {
                                (m + 2) & 3
                            },
                        );
                        w.multiplayer_world = remote;
                        round6::seed(&mut w);
                        match hook {
                            0 => behavior::on_block_added(&mut w, id, 8, 64, 8),
                            1 => behavior::on_neighbor_block_change(&mut w, id, 8, 64, 8, 1),
                            2 => behavior::update_tick(&mut w, id, 8, 64, 8, RandSel::World),
                            3 => o.push(p.activate_block(&mut w, 8, 64, 8) as u8),
                            4 => {
                                p.yaw = m as f32 * 22.5 - 180.0;
                                p.block_placed(&mut w, id, 8, 64, 8, 1);
                            }
                            _ => behavior::on_block_removal(&mut w, id, 8, 64, 8),
                        }
                        // The Java callback receiver remains the old singleton after a tick swaps IDs.
                        let actual = w.get_block_id(8, 64, 8);
                        let meta = w.get_block_metadata(8, 64, 8);
                        raw(&mut w, 8, 64, 8, id, meta);
                        for side in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, i32::MAX] {
                            o.push(w.block_power(8, 64, 8, side, false) as u8);
                            o.push(w.block_power(8, 64, 8, side, true) as u8);
                        }
                        let meta = w.get_block_metadata(8, 64, 8);
                        raw(&mut w, 8, 64, 8, actual, meta);
                        round6::result(o, &mut w);
                        queue(o, &w);
                    }
                }
            }
        }
    }
    for id in [93, 94] {
        for yaw in [
            f32::NEG_INFINITY,
            -f32::MAX,
            -360.0,
            -45.0,
            0.0,
            45.0,
            180.0,
            360.0,
            f32::MAX,
            f32::INFINITY,
            f32::NAN,
        ] {
            clear_repeater(&mut w);
            raw(&mut w, 8, 63, 8, 1, 0);
            raw(&mut w, 8, 64, 8, id, 15);
            p.yaw = yaw;
            round6::seed(&mut w);
            p.block_placed(&mut w, id, 8, 64, 8, 1);
            round6::result(o, &mut w);
            queue(o, &w);
        }
    }
    w.chunks.insert(
        (16777216 >> 4, 0),
        Box::new(Chunk::new(16777216 >> 4, 0, Box::new([0; 32768]))),
    );
    let mut r = crate::JavaRandom::new(91);
    for id in [93, 94] {
        for x in [8, -17, 16777216] {
            for m in 0..16 {
                for _ in 0..16 {
                    raw(&mut w, x, 64, 8, id, m);
                    round6::seed(&mut w);
                    w.block_display_tick(id, x, 64, 8, &mut r);
                    l(o, r.state());
                    l(o, w.rand.state());
                    super::super::events::write_events(&w.drain_events(), o);
                }
            }
        }
    }
}
pub(super) fn trapdoor(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    let mut p = Entity::player(&mut w, "oracle");
    w.set_trapdoor_item_bounds181();
    bounds(o, &w, 96);
    for target in [0, 1, 96] {
        for support in 0..=109 {
            for side in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, i32::MAX] {
                for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    raw(&mut w, 8 + dx, 64, 8 + dz, support, 0);
                }
                raw(&mut w, 8, 64, 8, target, 0);
                o.push(
                    crate::entity::player::placement::can_place_at(&mut w, 96, 8, 64, 8, side)
                        as u8,
                );
            }
        }
    }
    for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        raw(&mut w, 8 + dx, 64, 8 + dz, 0, 0);
    }
    for m in [
        i32::MIN,
        -17,
        -16,
        -9,
        -8,
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
        31,
        i32::MAX,
    ] {
        w.block_bounds[96] = crate::block::visual181::trapdoor_bounds181(m);
        bounds(o, &w, 96);
        o.push((m & 4 != 0) as u8);
    }
    for remote in [false, true] {
        for m in 0..16 {
            for hook in 0..3 {
                for side in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, 6, i32::MAX] {
                    raw(&mut w, 8, 64, 8, 96, m);
                    w.multiplayer_world = remote;
                    w.loaded_entity_list.clear();
                    round6::seed(&mut w);
                    match hook {
                        0 => p.block_placed(&mut w, 96, 8, 64, 8, side),
                        1 => o.push(p.activate_block(&mut w, 8, 64, 8) as u8),
                        _ => p.block_clicked(&mut w, 96, 8, 64, 8),
                    }
                    round6::result(o, &mut w);
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
            raw(&mut w, x, 64, 8, 96, m);
            w.set_trapdoor_item_bounds181();
            w.set_block_bounds_based_on_state(96, x, 64, 8);
            b(o, Some(w.block_bounds[96].translated(x as f64, 64.0, 8.0)));
            bounds(o, &w, 96);
            let bb = w.block_collision_box(96, x, 64, 8);
            b(o, bb);
            bounds(o, &w, 96);
            for axis in 0..3 {
                for offset in [0.0, 0.0625, 0.1875, 0.5, 0.8125, 0.9375, 1.0] {
                    for reverse in [false, true] {
                        let mut s = [x as f64 + offset, 64.0 + offset, 8.0 + offset];
                        let mut e = s;
                        s[axis] = [x as f64, 64.0, 8.0][axis] - 1.0;
                        e[axis] = [x as f64, 64.0, 8.0][axis] + 2.0;
                        let s = Vec3D::new(s[0], s[1], s[2]);
                        let e = Vec3D::new(e[0], e[1], e[2]);
                        hit(
                            o,
                            w.collision_ray_trace(
                                x,
                                64,
                                8,
                                if reverse { e } else { s },
                                if reverse { s } else { e },
                                false,
                                false,
                            ),
                        );
                        bounds(o, &w, 96);
                    }
                }
            }
        }
    }
}
