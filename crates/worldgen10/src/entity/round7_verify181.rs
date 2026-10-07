//! Round 7 original-bytecode item boundary matrices.
use super::{
    player::{ItemStack, PlayerEvent},
    Entity,
};
use crate::{world::chunk::idx, World};
fn i(o: &mut Vec<u8>, n: i32) {
    o.extend(n.to_le_bytes());
}
fn text(o: &mut Vec<u8>, s: &str) {
    i(o, s.len() as i32);
    o.extend(s.as_bytes());
}
fn raw(w: &mut World, x: i32, y: i32, z: i32, b: u8) {
    if (0..128).contains(&y) {
        w.chunk_mut(x >> 4, z >> 4).blocks[idx(x & 15, y, z & 15)] = b;
    }
}
fn scene(w: &mut World, y: i32, scene: i32) {
    for x in -2..=2 {
        for z in -2..=2 {
            raw(
                w,
                x,
                y,
                z,
                match scene {
                    0 => 0,
                    1 => 1,
                    2 => 20,
                    3 => 44,
                    4 => 9,
                    _ => 1,
                },
            );
        }
    }
    if scene == 5 {
        raw(w, 0, y + 1, 0, 1);
    }
    if scene == 6 {
        for x in -2..=2 {
            for z in -2..=2 {
                if x != 0 || z != 0 {
                    raw(w, x, y + 1, z, 1);
                }
            }
        }
    }
    if scene == 7 {
        raw(w, 0, y + 1, 0, 106);
    }
}
fn bucket_player(p: &mut Entity, config: i32) {
    p.set_position(3.25, 66.5, -2.75);
    p.prev_pos = super::Vec3D::new(-1.25, 61.25, 4.5);
    p.yaw = 35.0;
    p.prev_yaw = -17.0;
    p.pitch = 27.0;
    p.prev_pitch = -65.0;
    if config == 1 {
        p.prev_pos = super::Vec3D::new(1e16, -1e16, 1e16);
        p.prev_yaw = 1e20;
        p.prev_pitch = -1e20;
    }
    if config == 2 {
        p.y_offset = -0.75;
        p.yaw = -450.0;
        p.pitch = -90.0;
    }
}
fn bucket_snapshot(
    w: &mut World,
    original: ItemStack,
    result: ItemStack,
    writes: &[[i32; 6]],
    ray: (super::Vec3D, super::Vec3D),
    liquids: bool,
    o: &mut Vec<u8>,
) {
    o.push(original.same_object(result) as u8);
    for v in [result.id, result.count, result.damage] {
        i(o, v);
    }
    i(o, writes.len() as i32);
    for write in writes {
        for v in write {
            i(o, *v);
        }
    }
    for v in [ray.0, ray.1] {
        for d in [v.x, v.y, v.z] {
            o.extend(d.to_bits().to_le_bytes());
        }
    }
    o.push(liquids as u8);
    o.extend(w.rand.state().to_le_bytes());
    o.extend(w.math_random.state().to_le_bytes());
    for x in -1..=1 {
        for z in -1..=1 {
            for y in 63..=65 {
                o.push(w.get_block_id(x, y, z));
                i(o, w.get_block_metadata(x, y, z));
            }
        }
    }
    crate::world::events::write_events(&w.drain_events(), o);
}
fn bucket_replay(kind: &str) -> Vec<u8> {
    use super::{MovingObjectPosition, Vec3D};
    let mut o = Vec::new();
    if kind == "buckethit7" {
        for id in [325, 326, 327, 335] {
            for hit in -1..3 {
                for block in [0, 1, 8, 9, 10, 11, 31, 106] {
                    for meta in [0, 1, 15] {
                        for side in [-1, 0, 1, 2, 3, 4, 5, 6] {
                            for config in 0..3 {
                                for creative in [false, true] {
                                    for hell in [false, true] {
                                        for remote in [false, true] {
                                            let mut w = super::verify181::world();
                                            w.multiplayer_world = remote;
                                            if hell {
                                                w.dimension = crate::world::Dimension::Nether;
                                            }
                                            let mut p = Entity::player(&mut w, "Player");
                                            bucket_player(&mut p, config);
                                            p.player_state_mut().unwrap().capabilities.creative =
                                                creative;
                                            raw(&mut w, 0, 64, 0, block);
                                            w.chunk_mut(0, 0).data.set(0, 64, 0, meta);
                                            let hit = if hit < 0 {
                                                None
                                            } else if hit == 0 {
                                                Some(MovingObjectPosition::Block {
                                                    x: 0,
                                                    y: 64,
                                                    z: 0,
                                                    side,
                                                    hit: Vec3D::new(0.0, 64.0, 0.0),
                                                })
                                            } else {
                                                let e = Entity::living(
                                                    &mut w,
                                                    if hit == 1 { "Cow" } else { "Pig" },
                                                )
                                                .unwrap();
                                                let id = e.entity_id;
                                                w.entities.insert(id, e);
                                                Some(MovingObjectPosition::Entity {
                                                    id,
                                                    hit: Vec3D::default(),
                                                })
                                            };
                                            let original = ItemStack::new(
                                                id,
                                                if config == 1 {
                                                    i32::MIN
                                                } else if config == 2 {
                                                    0
                                                } else {
                                                    1
                                                },
                                                -32769,
                                            );
                                            let full = match id {
                                                326 => 8,
                                                327 => 10,
                                                335 => -1,
                                                _ => 0,
                                            };
                                            // 1.0 ItemBucketMilk.onItemRightClick only calls
                                            // setItemInUse: no ray, no writes, stack returned as is.
                                            let milk = id == 335;
                                            let ray = if milk {
                                                (Vec3D::default(), Vec3D::default())
                                            } else {
                                                p.bucket_ray181()
                                            };
                                            let writes = std::cell::RefCell::new(Vec::new());
                                            let result = if milk { original } else { p.bucket_hit_with_writes181(
                                                &mut w,
                                                original,
                                                full,
                                                ray.0,
                                                hit,
                                                |_, x, y, z| {
                                                    writes.borrow_mut().push([0, x, y, z, 0, 0])
                                                },
                                                |_, x, y, z, id| {
                                                    writes.borrow_mut().push([1, x, y, z, id, 0])
                                                },
                                            ) };
                                            bucket_snapshot(
                                                &mut w,
                                                original,
                                                result,
                                                &writes.into_inner(),
                                                ray,
                                                full == 0,
                                                &mut o,
                                            );
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    } else {
        for id in [325, 326, 327, 335] {
            for block in [0, 1, 8, 9, 10, 11, 31, 106] {
                for meta in [0, 1, 15] {
                    for yaw in [0.0, 90.0, 180.0, 270.0, 395.0] {
                        for pitch in [-90.0, 0.0, 45.0, 90.0] {
                            for hell in [false, true] {
                                for creative in [false, true] {
                                    let mut w = super::verify181::world();
                                    if hell {
                                        w.dimension = crate::world::Dimension::Nether;
                                    }
                                    let mut p = Entity::player(&mut w, "Player");
                                    p.set_position(0.5, 65.0, 0.5);
                                    p.prev_pos = Vec3D::new(-0.25, 64.0, 0.25);
                                    p.yaw = yaw;
                                    p.prev_yaw = -17.0;
                                    p.pitch = pitch;
                                    p.prev_pitch = -65.0;
                                    p.player_state_mut().unwrap().capabilities.creative = creative;
                                    for x in -1..=1 {
                                        for z in -1..=1 {
                                            raw(&mut w, x, 64, z, block);
                                            w.chunk_mut(x >> 4, z >> 4).data.set(
                                                x & 15,
                                                64,
                                                z & 15,
                                                meta,
                                            );
                                        }
                                    }
                                    let original = ItemStack::new(id, 1, 17);
                                    let full = match id {
                                        326 => 8,
                                        327 => 10,
                                        335 => -1,
                                        _ => 0,
                                    };
                                    // 1.0 ItemBucketMilk.onItemRightClick: no ray, stack unchanged.
                                    let (ray, result) = if id == 335 {
                                        ((Vec3D::default(), Vec3D::default()), original)
                                    } else {
                                        (p.bucket_ray181(), p.bucket_right_click181(&mut w, original, full))
                                    };
                                    bucket_snapshot(
                                        &mut w,
                                        original,
                                        result,
                                        &[],
                                        ray,
                                        full == 0,
                                        &mut o,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    o
}
pub fn replay(kind: &str) -> Vec<u8> {
    let mut o = Vec::new();
    if kind == "vineconstructor7" {
        for block in [253u8, 254u8] {
            for subtypes in [false, true] {
                for color in [i32::MIN, -1, 0, 4764952, i32::MAX] {
                    for damage in [
                        i32::MIN,
                        -65537,
                        -32768,
                        -17,
                        -1,
                        0,
                        1,
                        2,
                        3,
                        15,
                        16,
                        17,
                        32767,
                        65536,
                        i32::MAX,
                    ] {
                        let item = super::player::registry::VineItem::new(block, subtypes);
                        i(&mut o, 64);
                        i(&mut o, 0);
                        o.push(item.subtypes as u8);
                        i(&mut o, item.placed_metadata(damage));
                        let mut calls = Vec::new();
                        let icon = item.icon(damage, |block, side, damage| {
                            calls.push([block as i32, side, damage]);
                            (block as i32)
                                .wrapping_mul(1009)
                                .wrapping_add(side.wrapping_mul(31))
                                .wrapping_add(damage.wrapping_mul(17))
                        });
                        let result_color = item.color(damage, |block, damage| {
                            calls.push([block as i32, -1, damage]);
                            color
                        });
                        i(&mut o, icon);
                        i(&mut o, result_color);
                        i(&mut o, calls.len() as i32);
                        for call in calls {
                            for v in call {
                                i(&mut o, v);
                            }
                        }
                    }
                }
            }
        }
        return o;
    }
    if kind == "logconstructor7" {
        for _placement in [1, 17, 31, 98] {
            for block in [Some(253u8), Some(254u8), None] {
                for damage in [
                    i32::MIN,
                    -65537,
                    -32768,
                    -17,
                    -1,
                    0,
                    1,
                    2,
                    3,
                    15,
                    16,
                    17,
                    32767,
                    65536,
                    i32::MAX,
                ] {
                    let item = super::player::registry::LogItem::new(block);
                    i(&mut o, 64);
                    i(&mut o, 0);
                    o.push(1);
                    i(&mut o, item.placed_metadata(damage));
                    let mut calls = Vec::new();
                    let result = item.icon(damage, |block, side, damage| {
                        calls.push([block as i32, side, damage]);
                        (block as i32)
                            .wrapping_mul(1009)
                            .wrapping_add(side.wrapping_mul(31))
                            .wrapping_add(damage.wrapping_mul(17))
                    });
                    i(&mut o, result.unwrap_or(0));
                    text(&mut o, result.err().unwrap_or(""));
                    i(&mut o, calls.len() as i32);
                    for call in calls {
                        for v in call {
                            i(&mut o, v);
                        }
                    }
                }
            }
        }
        return o;
    }
    if matches!(kind, "buckethit7" | "bucketray7") {
        return bucket_replay(kind);
    }
    if kind == "metadata7" {
        use super::player::registry;
        for id in [44, 35, 18, 17, 98, 31, 106] {
            let item = registry::item(id).unwrap();
            i(&mut o, item.max_stack);
            i(&mut o, item.durability);
            o.push(item.subtypes as u8);
            for fancy in [false, true] {
                for damage in (-32768..=32767).chain([
                    i32::MIN,
                    -65537,
                    -65536,
                    -32769,
                    32768,
                    65535,
                    65536,
                    65537,
                    i32::MAX,
                ]) {
                    let (icon, color) = registry::block_item_properties(id, damage, fancy).unwrap();
                    i(&mut o, icon);
                    i(&mut o, color as i32);
                    i(&mut o, registry::placed_block_metadata(id, damage));
                    // Mode_itemmetadata7_10 writes getItemNameIS only for ItemSlab (44) and
                    // ItemCloth (35); other ItemBlock subclasses (ItemColored 31) emit no name.
                    if matches!(id, 44 | 35) {
                        if let Some(name) = registry::block_item_name(id, damage) {
                            text(&mut o, &name);
                        }
                    }
                }
            }
        }
        return o;
    }
    let id = if kind == "beditem7" { 355 } else { 323 };
    for sc in 0..8 {
        for y in [-2, -1, 0, 63, 125, 126, 127, 128] {
            for side in [-1, 0, 1, 2, 3, 4, 5, 6] {
                for yaw in [-450.0, -90.0, 0.0, 90.0, 180.0, 450.0] {
                    for count in [i32::MIN, -1, 0, 1, i32::MAX] {
                        for remote in [false, true] {
                            let mut w = super::verify181::world();
                            w.multiplayer_world = remote;
                            let mut p = Entity::player(&mut w, "Player");
                            p.yaw = yaw;
                            scene(&mut w, y, sc);
                            let mut pig = Entity::living(&mut w, "Pig").unwrap();
                            pig.set_position(0.5, (y + 1) as f64, 0.5);
                            w.entity_joined_world(pig);
                            let mut stack = ItemStack::new(id, count, 0);
                            o.push(p.item_use_on_block181(&mut w, &mut stack, 0, y, 0, side) as u8);
                            i(&mut o, stack.count);
                            for x in -2..=2 {
                                for z in -2..=2 {
                                    for yy in y - 1..=y + 2 {
                                        o.push(w.get_block_id(x, yy, z));
                                        i(&mut o, w.get_block_metadata(x, yy, z));
                                    }
                                }
                            }
                            let edits: Vec<_> = p
                                .player_state()
                                .unwrap()
                                .events
                                .iter()
                                .filter_map(|e| {
                                    if let PlayerEvent::EditSign { position, sign, .. } = e {
                                        Some((*position, sign))
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            i(&mut o, edits.len() as i32);
                            if let Some((pos, sign)) = edits.last() {
                                for v in [pos.0, pos.1, pos.2] {
                                    i(&mut o, v);
                                }
                                for s in &sign.text {
                                    text(&mut o, s);
                                }
                                o.push(sign.editable as u8);
                            }
                            o.extend(w.rand.state().to_le_bytes());
                            crate::world::events::write_events(&w.drain_events(), &mut o);
                        }
                    }
                }
            }
        }
    }
    o
}
