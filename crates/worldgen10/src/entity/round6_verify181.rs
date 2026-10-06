//! Round 6 original-bytecode checks. See Mode_entityround6_181.java.
use super::{potion::PotionEffect, Entity};
use crate::{
    nbt::{Compound, Tag},
    world::chunk::idx,
    World,
};
fn i(o: &mut Vec<u8>, n: i32) {
    o.extend(n.to_le_bytes());
}
fn effects(e: &Entity, o: &mut Vec<u8>) {
    let n = e.to_nbt().unwrap();
    i(o, n.list("ActiveEffects").len() as i32);
    for t in n.list("ActiveEffects") {
        if let Tag::Compound(n) = t {
            for v in [
                n.byte("Id") as i32,
                n.byte("Amplifier") as i32,
                n.int("Duration"),
            ] {
                i(o, v);
            }
        }
    }
}
fn raw(w: &mut World, x: i32, y: i32, z: i32, b: u8) {
    w.chunk_mut(x >> 4, z >> 4).blocks[idx(x & 15, y, z & 15)] = b;
}
fn door_scene(w: &mut World, y: i32, id: u8, scene: i32) {
    fn put(w: &mut World, x: i32, y: i32, z: i32, b: u8) {
        if (0..128).contains(&y) {
            raw(w, x, y, z, b);
        }
    }
    put(w, 0, y, 0, if scene == 0 { 0 } else { 1 });
    if scene == 2 {
        put(w, 0, y + 2, 0, 1);
    }
    if scene == 3 {
        put(w, 0, y + 1, 1, id);
        put(w, 1, y + 1, 0, id);
    }
    if scene == 4 {
        put(w, -1, y + 1, 0, 1);
        put(w, -1, y + 2, 0, 1);
    }
    if scene == 5 {
        put(w, 0, y + 1, 0, 106);
        put(w, 0, y + 2, 0, 31);
    }
}
fn door_snapshot(w: &mut World, y: i32, o: &mut Vec<u8>) {
    for x in -1..=1 {
        for z in -1..=1 {
            for yy in y..=y + 3 {
                o.push(w.get_block_id(x, yy, z));
                i(o, w.get_block_metadata(x, yy, z));
            }
        }
    }
    o.push(w.editing_blocks as u8);
    crate::world::events::write_events(&w.drain_events(), o);
}
pub fn replay(kind: &str) -> Vec<u8> {
    let mut o = Vec::new();
    match kind {
        "livingdamage6" => {
            use super::{
                damage::{DamageKind, DamageSource},
                EntityKind,
            };
            for source in 0..5 {
                for immunity in [0, 10, 11, 20] {
                    for damage in [i32::MIN, -1, 0, 1, 7, i32::MAX] {
                        let mut w = super::verify181::world();
                        let mut a = if source == 1 {
                            Entity::player(&mut w, "Player")
                        } else {
                            Entity::living(
                                &mut w,
                                if source == 2 || source == 3 {
                                    "Wolf"
                                } else {
                                    "Zombie"
                                },
                            )
                            .unwrap()
                        };
                        a.set_position(-3.5, 65.0, 4.5);
                        if source == 3 {
                            if let EntityKind::Wolf(s) = &mut a.kind {
                                s.flags |= 4;
                            }
                        }
                        let id = a.entity_id;
                        w.loaded_entity_list.push(id);
                        if source == 1 {
                            w.players.push((-3.5, 65.0, 4.5));
                        }
                        w.entities.insert(id, a);
                        let mut e = Entity::living(&mut w, "Cow").unwrap();
                        e.set_position(-3.5, 65.0, 4.5);
                        e.living.health = 7;
                        e.living.hearts_life = immunity;
                        e.living.last_damage = -1;
                        let s = if source == 0 {
                            DamageSource::new(DamageKind::Generic)
                        } else {
                            DamageSource::entity(DamageKind::Mob, Some(id))
                        };
                        for _ in 0..2 {
                            o.push(e.attack_from(&mut w, s, damage) as u8);
                            super::verify181::state(&mut w, &e, &mut o);
                            i(&mut o, e.living.recently_hit);
                            i(&mut o, e.living.attacking_player.unwrap_or(-1));
                            o.push(e.living.dying as u8);
                        }
                    }
                }
            }
        }
        "doorhelper6" => {
            for id in [64, 71] {
                for scene in 0..6 {
                    for dir in [i32::MIN, -1, 0, 1, 2, 3, 4, i32::MAX] {
                        let mut w = super::verify181::world();
                        door_scene(&mut w, 63, id, scene);
                        super::player::placement::place_door181(&mut w, 0, 64, 0, dir, id);
                        door_snapshot(&mut w, 63, &mut o);
                    }
                }
            }
        }
        "dooritem6" => {
            for id in [324, 330] {
                for scene in 0..6 {
                    for y in [-1, 0, 63, 125, 126, 127, 128] {
                        for side in [-1, 0, 1, 2, 5, 6] {
                            for yaw in [-450.0, -90.0, 0.0, 90.0, 180.0, 450.0] {
                                for count in [i32::MIN, 0, 1] {
                                    let mut w = super::verify181::world();
                                    let p = Entity::player(&mut w, "Player");
                                    let mut p = p;
                                    p.yaw = yaw;
                                    door_scene(&mut w, y, if id == 324 { 64 } else { 71 }, scene);
                                    let mut stack = super::player::ItemStack::new(id, count, 0);
                                    o.push(p.place_door(&mut w, &mut stack, 0, y, 0, side) as u8);
                                    i(&mut o, stack.count);
                                    door_snapshot(&mut w, y, &mut o);
                                }
                            }
                        }
                    }
                }
            }
        }
        "dyeproperties6" => {
            use super::player::dye;
            fn text(o: &mut Vec<u8>, s: &str) {
                i(o, s.len() as i32);
                o.extend(s.as_bytes());
            }
            for damage in [
                i32::MIN,
                -32768,
                -17,
                -16,
                -9,
                -8,
                -1,
                0,
                1,
                7,
                8,
                15,
                16,
                17,
                127,
                32767,
                i32::MAX,
            ] {
                i(&mut o, dye::icon(damage));
                let value = dye::name(damage);
                text(&mut o, value.as_ref().map(String::as_str).unwrap_or(""));
                text(&mut o, value.err().unwrap_or(""));
            }
            for c in dye::DYE_COLORS {
                i(&mut o, c);
            }
            for c in dye::DYE_COLOR_NAMES {
                text(&mut o, c);
            }
        }
        "dyeblocks6" => {
            for block in [0, 1, 2, 6, 39, 40, 59, 104, 105] {
                for scene in 0..3 {
                    for seed in 0..4 {
                        for damage in [-1, 0, 15, 16] {
                            for count in [i32::MIN, 0, 1] {
                                for remote in [false, true] {
                                    let mut w = super::verify181::world();
                                    w.multiplayer_world = remote;
                                    let mut p = Entity::player(&mut w, "Player");
                                    for x in -8..=8 {
                                        for z in -8..=8 {
                                            raw(
                                                &mut w,
                                                x,
                                                63,
                                                z,
                                                if matches!(block, 59 | 104 | 105) {
                                                    60
                                                } else {
                                                    2
                                                },
                                            );
                                        }
                                    }
                                    raw(&mut w, 0, 64, 0, block);
                                    w.chunk_mut(0, 0).data.set(0, 64, 0, seed * 2);
                                    if scene == 1 {
                                        raw(&mut w, 0, 65, 0, 1);
                                    }
                                    if scene == 2 {
                                        for x in -8..=8 {
                                            for z in -8..=8 {
                                                raw(&mut w, x, 65, z, 1);
                                            }
                                        }
                                    }
                                    w.rand.set_seed(seed as i64);
                                    w.item_random.set_seed(600);
                                    let mut stack =
                                        super::player::ItemStack::new(351, count, damage);
                                    o.push(p.item_use_on_block181(&mut w, &mut stack, 0, 64, 0, 1)
                                        as u8);
                                    i(&mut o, stack.count);
                                    o.extend(w.rand.state().to_le_bytes());
                                    o.extend(w.item_random.state().to_le_bytes());
                                    o.extend(w.math_random.state().to_le_bytes());
                                    for x in -8..=8 {
                                        for z in -8..=8 {
                                            for y in 62..=80 {
                                                o.push(w.get_block_id(x, y, z));
                                                o.push(w.get_block_metadata(x, y, z) as u8);
                                            }
                                        }
                                    }
                                    crate::world::events::write_events(&w.drain_events(), &mut o);
                                }
                            }
                        }
                    }
                }
            }
        }
        "living6" => {
            for name in ["Cow", "Zombie", "Skeleton", "Squid"] {
                for scene in 0..3 {
                    for air in [-20, -19, 0, 300] {
                        for breathing in [false, true] {
                            for health in [0, 7] {
                                let mut w = super::verify181::world();
                                let mut e = Entity::living(&mut w, name).unwrap();
                                e.set_position(-3.5, 65.0, 4.5);
                                e.air = air;
                                e.living.health = health;
                                e.fire = 71;
                                e.living.death_time = 18;
                                e.living.recently_hit = 3;
                                for x in -5..=-2 {
                                    for y in 64..=69 {
                                        for z in 3..=6 {
                                            if scene > 0 {
                                                raw(
                                                    &mut w,
                                                    x,
                                                    y,
                                                    z,
                                                    if scene == 1 { 9 } else { 1 },
                                                );
                                            }
                                        }
                                    }
                                }
                                if breathing {
                                    e.add_potion_effect(PotionEffect::new(13, 2, 0));
                                }
                                for _ in 0..4 {
                                    e.living_entity_update(&mut w);
                                    super::verify181::state(&mut w, &e, &mut o);
                                    i(&mut o, e.living.recently_hit);
                                    i(&mut o, w.loaded_entity_list.len() as i32);
                                    for id in w.loaded_entity_list.clone() {
                                        let a = w.entities[&id].clone();
                                        super::verify181::state(&mut w, &a, &mut o);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        "livingnbt6" => {
            for ids in [
                vec![19, 1, 17, 2, 10, 13, 6, 7],
                vec![17, 1, 19, 2],
                vec![1, 17, 2, 19],
                vec![13, 10, 7, 6, 2, 1, 19, 17],
            ] {
                let mut w = super::verify181::world();
                let mut e = Entity::living(&mut w, "Cow").unwrap();
                for id in ids {
                    e.add_potion_effect(PotionEffect::new(id, 100 + id, id % 3));
                    effects(&e, &mut o);
                }
                e.add_potion_effect(PotionEffect::new(1, 3, 5));
                effects(&e, &mut o);
                // Release fixture calls EntityLiving.damageEntity; the removed
                // Beta API used to delete hunger here.
                let damage = e.armor_damage(super::damage::DamageSource::new(super::damage::DamageKind::Generic), 17);
                e.living.health -= e.resistance_damage(damage);
                effects(&e, &mut o);
                let mut n = e.to_nbt().unwrap();
                let mut list = Vec::new();
                for (id, amp, duration) in
                    [(19, 3, 99), (1, 5, 17), (19, -1, 0), (17, 2, 25), (1, 0, 5)]
                {
                    let mut t = Compound::new();
                    t.put("Id", Tag::Byte(id));
                    t.put("Amplifier", Tag::Byte(amp));
                    t.put("Duration", Tag::Int(duration));
                    list.push(Tag::Compound(t));
                }
                n.put("ActiveEffects", Tag::List(10, list));
                e.read_existing_nbt(&mut w, &n).unwrap();
                effects(&e, &mut o);
                let a = Entity::from_nbt(&mut w, &n).unwrap();
                effects(&a, &mut o);
            }
        }
        "sleep6" => {
            for scene in 0..7 {
                for seed in 0..64 {
                    let mut w = super::verify181::world();
                    for c in w.chunks.values_mut() {
                        c.sky_light.0.fill(if scene == 3 { 255 } else { 0 });
                    }
                    if scene == 1 {
                        for x in -32..=31 {
                            for z in -32..=31 {
                                raw(&mut w, x, 64, z, 1);
                                raw(&mut w, x, 65, z, 1);
                            }
                        }
                    }
                    if scene == 2 {
                        for x in -32..=31 {
                            for z in -32..=31 {
                                raw(&mut w, x, 63, z, 0);
                            }
                        }
                    }
                    let mut p = Entity::player(&mut w, "Player");
                    p.set_position(0.5, 64.0, 0.5);
                    p.player_state_mut().unwrap().sleeping = true;
                    p.player_state_mut().unwrap().bed = Some((0, 64, 0));
                    if scene == 4 || scene == 5 {
                        raw(&mut w, 0, 64, 0, 26);
                        raw(&mut w, 0, 64, 1, 26);
                        w.chunk_mut(0, 0)
                            .data
                            .set(0, 64, 0, if scene == 4 { 4 } else { 6 });
                    }
                    w.entity_joined_world(p);
                    if scene == 6 {
                        let mut q = Entity::player(&mut w, "Player");
                        q.set_position(8.5, 64.0, 8.5);
                        q.player_state_mut().unwrap().sleeping = true;
                        q.player_state_mut().unwrap().bed = Some((8, 64, 8));
                        w.entity_joined_world(q);
                    }
                    w.rand.set_seed(seed);
                    // performSleepSpawning was removed in 1.0: nothing spawns.
                    o.push(0);
                    for id in w.loaded_entity_list.clone() {
                        let e = w.entities[&id].clone();
                        super::verify181::state(&mut w, &e, &mut o);
                    }
                    i(&mut o, w.loaded_entity_list.len() as i32);
                }
            }
        }
        _ => panic!("unknown round 6 mode {kind}"),
    }
    o
}

/// Chunk-key identity hashes are external VM inputs. All traversal/spawning
/// decisions still come from the literal Rust algorithm and original Java code.
pub fn spawn_inputs_replay(expected: &[u8]) -> Vec<u8> {
    use super::EntityKind;
    let count = i32::from_le_bytes(expected[..4].try_into().unwrap()) as usize;
    let start = 4 + count * 4;
    let mut identities = expected[4..start]
        .chunks_exact(4)
        .map(|b| i32::from_le_bytes(b.try_into().unwrap()));
    let mut o = expected[..start].to_vec();
    for scenario in 0..4 {
        for seed in 0..8 {
            for players in 1..=2 {
                use crate::world::chunk::{idx, Chunk};
                let mut w = super::verify181::world();
                for x in -14..=14 {
                    for z in -14..=14 {
                        let mut c =
                            Chunk::new(x, z, vec![0; 32768].into_boxed_slice().try_into().unwrap());
                        c.loaded = true;
                        c.populated = true;
                        c.height_map.fill(64);
                        c.sky_light.0.fill(if scenario == 0 { 0 } else { 255 });
                        for bx in 0..16 {
                            for bz in 0..16 {
                                c.blocks[idx(bx, 63, bz)] = if scenario == 3 { 1 } else { 2 };
                                if scenario == 3 {
                                    for y in 64..80 {
                                        c.blocks[idx(bx, y, bz)] = 9;
                                    }
                                }
                            }
                        }
                        w.chunks.insert((x, z), Box::new(c));
                    }
                }
                w.skylight_subtracted = if scenario == 0 { 11 } else { 0 };
                w.spawn = (0, 64, 0);
                w.rand.set_seed(seed);
                let p = Entity::player(&mut w, "Player");
                w.entities.insert(p.entity_id, p);
                w.players.push((0.5, 64.0, 0.5));
                if players == 2 {
                    let p = Entity::player(&mut w, "Player");
                    w.entities.insert(p.entity_id, p);
                    w.players.push((-16.01, 64.0, 16.01));
                }
                for _ in 0..2 {
                    i(
                        &mut o,
                        w.perform_spawning_with_identity(scenario == 0, scenario != 0, |_, _| {
                            identities.next().expect("missing recorded JVM identity")
                        }),
                    );
                    o.extend(w.rand.state().to_le_bytes());
                    o.extend(w.math_random.state().to_le_bytes());
                    i(&mut o, w.loaded_entity_list.len() as i32);
                    for id in w.loaded_entity_list.clone() {
                        let e = w.entities[&id].clone();
                        super::verify181::state(&mut w, &e, &mut o);
                        if let EntityKind::Sheep { fleece_color, .. } = e.kind {
                            i(&mut o, fleece_color as i32);
                        }
                    }
                }
            }
        }
    }
    assert!(
        identities.next().is_none(),
        "unused recorded JVM identities"
    );
    o
}

pub fn block_items_replay(expected: &[u8]) -> Vec<u8> {
    let count = i32::from_le_bytes(expected[..4].try_into().unwrap()) as usize;
    let start = 4 + count * 4;
    let mut o = expected[..start].to_vec();
    for bytes in expected[4..start].chunks_exact(4) {
        let id = i32::from_le_bytes(bytes.try_into().unwrap());
        assert!(super::player::registry::item(id).is_some());
        for scene in 0..6 {
            for y in [-1, 0, 64, 126, 127, 128] {
                for side in [-1, 0, 1, 2, 3, 4, 5, 6] {
                    let mut w = super::verify181::world();
                    let mut p = Entity::player(&mut w, "Player");
                    p.set_position(8.5, 64.0, 8.5);
                    p.yaw = 35.0;
                    p.pitch = -17.0;
                    for x in -2..=2 {
                        for z in -2..=2 {
                            for yy in [y - 1, y - 2] {
                                if (0..128).contains(&yy) {
                                    raw(&mut w, x, yy, z, 1);
                                }
                            }
                        }
                    }
                    if (0..128).contains(&y) {
                        raw(&mut w, 0, y, 0, [0, 78, 106, 1, 9, 0][scene as usize]);
                    }
                    if scene == 5 {
                        let mut pig = Entity::living(&mut w, "Pig").unwrap();
                        pig.set_position(0.5, (y + 1) as f64, 0.5);
                        w.entity_joined_world(pig);
                    }
                    let mut stack = super::player::ItemStack::new(
                        id,
                        if scene == 4 {
                            0
                        } else if scene == 5 {
                            i32::MIN
                        } else {
                            1
                        },
                        if scene == 1 {
                            -1
                        } else if scene == 2 {
                            15
                        } else {
                            0
                        },
                    );
                    o.push(p.place_block_item(&mut w, &mut stack, 0, y, 0, side) as u8);
                    i(&mut o, stack.count);
                    o.extend(w.rand.state().to_le_bytes());
                    o.extend(w.math_random.state().to_le_bytes());
                    for x in -1..=1 {
                        for z in -1..=1 {
                            for yy in y - 2..=y + 2 {
                                o.push(w.get_block_id(x, yy, z));
                                i(&mut o, w.get_block_metadata(x, yy, z));
                            }
                        }
                    }
                    crate::world::events::write_events(&w.drain_events(), &mut o);
                }
            }
        }
    }
    o
}
