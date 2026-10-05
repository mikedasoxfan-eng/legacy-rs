//! Original-bytecode round 5 branch matrices, paired with Mode_entityround5_181.
use super::{
    damage::{DamageKind, DamageSource},
    living::Target,
    Entity, EntityKind,
};
use crate::{world::chunk::idx, World};
fn i(o: &mut Vec<u8>, n: i32) {
    o.extend(n.to_le_bytes());
}
fn d(o: &mut Vec<u8>, n: f64) {
    o.extend(n.to_bits().to_le_bytes());
}
fn f(o: &mut Vec<u8>, n: f32) {
    o.extend(n.to_bits().to_le_bytes());
}
fn state(w: &mut World, e: &Entity, o: &mut Vec<u8>) {
    super::verify181::state(w, e, o);
    i(o, e.riding.unwrap_or(-1));
    i(o, e.ridden_by.unwrap_or(-1));
    if let EntityKind::Wolf(s) = &e.kind {
        i(o, s.flags as i8 as i32);
        i(o, s.watched_health);
        for b in [s.interested, s.shaking, s.shake_active] {
            o.push(b as u8);
        }
        for n in [s.interest, s.prev_interest, s.shake_time, s.prev_shake_time] {
            f(o, n);
        }
    }
    if e.kind.is_living()
        && !matches!(
            e.kind,
            EntityKind::Player(..) | EntityKind::Ghast(..) | EntityKind::Slime { .. }
        )
    {
        i(
            o,
            e.living.path.as_ref().map_or(-1, |p| p.points.len() as i32),
        );
        if let Some(p) = &e.living.path {
            for point in &p.points {
                for n in point {
                    i(o, *n);
                }
            }
        }
    }
}
fn create(w: &mut World, name: &str) -> Entity {
    match name {
        "Player" => Entity::player(w, "Player"),
        "Boat" => Entity::boat_base(w),
        s if s.starts_with("Minecart") => {
            Entity::minecart(w, 0.0, 0.0, 0.0, s[8..].parse().unwrap())
        }
        _ => Entity::living(w, name).unwrap(),
    }
}
fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8) {
    w.chunk_mut(x >> 4, z >> 4).blocks[idx(x & 15, y, z & 15)] = id;
}
fn watched(e: &Entity, o: &mut Vec<u8>) {
    let bytes = e.data_watcher.write().unwrap();
    i(o, bytes.len() as i32);
    o.extend(bytes);
    o.push(e.data_watcher.object_changed as u8);
}
pub fn replay(kind: &str) -> Vec<u8> {
    let mut o = Vec::new();
    match kind {
        "recorditem5" => {
            for record in [2256, 2257] {
                for remote in [false, true] {
                    for block in [0, 1, 84] {
                        for meta in [0, 1, 15] {
                            for count in [i32::MIN, -1, 0, 1, 2, i32::MAX] {
                                for side in [-1, 0, 1, 2, 5, 6] {
                                    let mut w = super::verify181::world();
                                    w.multiplayer_world = remote;
                                    let mut p = create(&mut w, "Player");
                                    raw(&mut w, -3, 65, 4, block);
                                    w.chunk_mut(-3 >> 4, 4 >> 4).data.set(
                                        -3 & 15,
                                        65,
                                        4 & 15,
                                        meta,
                                    );
                                    let mut stack = super::player::ItemStack::new(record, count, 0);
                                    o.push(
                                        p.item_use_on_block181(&mut w, &mut stack, -3, 65, 4, side)
                                            as u8,
                                    );
                                    i(&mut o, stack.count);
                                    i(&mut o, w.get_block_metadata(-3, 65, 4));
                                    let stored = match w.get_block_tile_entity(-3, 65, 4) {
                                        Some(crate::world::tile::TileEntity::RecordPlayer {
                                            record,
                                        }) => *record,
                                        _ => -1,
                                    };
                                    i(&mut o, stored);
                                    state(&mut w, &p, &mut o);
                                }
                            }
                        }
                    }
                }
            }
        }

        "paintingitem5" => {
            for side in [-1, 0, 1, 2, 3, 4, 5, 6] {
                for remote in [false, true] {
                    for scene in 0..3 {
                        for y in [-1, 0, 63, 64, 127, 128] {
                            for count in [i32::MIN, -1, 0, 1, 2, i32::MAX] {
                                let mut w = super::verify181::world();
                                w.multiplayer_world = remote;
                                let p = create(&mut w, "Player");
                                if scene > 0 {
                                    for x in -7..=1 {
                                        for z in 0..=8 {
                                            for yy in 0..128 {
                                                if (scene == 1 && z == 4) || (scene == 2 && x == -3)
                                                {
                                                    raw(&mut w, x, yy, z, 1);
                                                }
                                            }
                                        }
                                    }
                                }
                                let mut stack = super::player::ItemStack::new(321, count, 0);
                                o.push(p.place_painting(&mut w, &mut stack, -3, y, 4, side) as u8);
                                i(&mut o, stack.count);
                                state(&mut w, &p, &mut o);
                                for id in w.loaded_entity_list.clone() {
                                    let e = w.entities[&id].clone();
                                    state(&mut w, &e, &mut o);
                                    if let EntityKind::Painting(s) = e.kind {
                                        i(&mut o, s.art as i32);
                                        i(&mut o, s.direction);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        "vehicleriders5" => {
            for kind in -1..3 {
                for name in ["Player", "Pig", "Boat"] {
                    for remote in [false, true] {
                        for dead in [false, true] {
                            for scene in 0..3 {
                                for speed in [-0.4, 0.0, 0.4] {
                                    let mut w = super::verify181::world();
                                    w.multiplayer_world = remote;
                                    for x in -20..=20 {
                                        for z in 2..=7 {
                                            if scene == 1 {
                                                for y in 64..68 {
                                                    raw(&mut w, x, y, z, 9);
                                                }
                                            }
                                            if scene == 2 {
                                                raw(&mut w, x, 64, z, 66);
                                                w.chunk_mut(x >> 4, z >> 4).data.set(
                                                    x & 15,
                                                    64,
                                                    z & 15,
                                                    1,
                                                );
                                            }
                                        }
                                    }
                                    let mut a = if kind < 0 {
                                        Entity::boat(&mut w, -3.5, 65.0, 4.5)
                                    } else {
                                        Entity::minecart(&mut w, -3.5, 65.0, 4.5, kind)
                                    };
                                    let mut r = create(&mut w, name);
                                    r.dead = dead;
                                    r.motion.x = 0.3;
                                    r.motion.z = -0.2;
                                    r.riding = Some(a.entity_id);
                                    a.ridden_by = Some(r.entity_id);
                                    a.motion.x = speed;
                                    a.motion.z = -speed / 2.0;
                                    a.yaw = 35.0;
                                    let rid = r.entity_id;
                                    w.entities.insert(rid, r);
                                    for _ in 0..12 {
                                        a.on_update(&mut w);
                                        let mut r = w.entities.remove(&rid).unwrap();
                                        if a.ridden_by.is_some() {
                                            a.update_rider_position(&mut r);
                                        }
                                        state(&mut w, &a, &mut o);
                                        state(&mut w, &r, &mut o);
                                        w.entities.insert(rid, r);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        "dyesheep5" => {
            for name in ["Sheep", "Pig", "Wolf"] {
                for remote in [false, true] {
                    for sheared in [false, true] {
                        for color in [0, 1, 7, 15] {
                            for damage in [i32::MIN, -17, -1, 0, 1, 7, 15, 16, i32::MAX] {
                                for count in [i32::MIN, -1, 0, 1, 2, i32::MAX] {
                                    let mut w = super::verify181::world();
                                    w.multiplayer_world = remote;
                                    let mut e = create(&mut w, name);
                                    if let EntityKind::Sheep {
                                        sheared: s,
                                        fleece_color: c,
                                    } = &mut e.kind
                                    {
                                        *s = sheared;
                                        *c = color;
                                    }
                                    e.sync_mob_watcher();
                                    let mut stack =
                                        super::player::ItemStack::new(351, count, damage);
                                    e.dye_sheep(&mut stack);
                                    state(&mut w, &e, &mut o);
                                    watched(&e, &mut o);
                                    i(&mut o, stack.id);
                                    i(&mut o, stack.count);
                                    i(&mut o, stack.damage);
                                }
                            }
                        }
                    }
                }
            }
        }

        "potionexpiry5" => {
            for remote in [false, true] {
                for id in [1, 2, 6, 7, 10, 13, 17, 19] {
                    for duration in [-1, 0, 1, 2, 25] {
                        for amplifier in [0, 1, 31] {
                            let mut w = super::verify181::world();
                            w.multiplayer_world = remote;
                            let mut e = create(&mut w, "Pig");
                            e.living.health = 7;
                            e.add_potion_effect(super::potion::PotionEffect::new(
                                id, duration, amplifier,
                            ));
                            for _ in 0..4 {
                                e.tick_potion_effects(&mut w);
                                state(&mut w, &e, &mut o);
                            }
                        }
                    }
                }
            }
        }

        "watchers5" => {
            for name in [
                "Item",
                "XPOrb",
                "Arrow",
                "Snowball",
                "Painting",
                "Creeper",
                "Skeleton",
                "Spider",
                "Giant",
                "Zombie",
                "Slime",
                "PigZombie",
                "Enderman",
                "CaveSpider",
                "Silverfish",
                "Ghast",
                "Pig",
                "Sheep",
                "Cow",
                "Chicken",
                "Squid",
                "Wolf",
                "Boat",
                "Minecart",
            ] {
                let mut w = super::verify181::world();
                let mut e = super::registry::create(&mut w, name).unwrap();
                watched(&e, &mut o);
                for bit in [-1, 0, 1, 2, 3, 4, 7, 8, 31, 32, 33, 35] {
                    for value in [true, false] {
                        e.set_entity_flag(bit, value);
                        watched(&e, &mut o);
                        for n in 0..5 {
                            o.push(e.entity_flag(n) as u8);
                        }
                    }
                }
                use super::data_watcher::{Value, WatchableObject};
                e.apply_watcher_list(&[
                    WatchableObject::new(0, Value::Byte(27)),
                    WatchableObject::new(29, Value::Int(123)),
                ]);
                watched(&e, &mut o);
                for n in 0..5 {
                    o.push(e.entity_flag(n) as u8);
                }
            }
            for remote in [false, true] {
                for fire in [-1, 0, 1, 20] {
                    for riding in [false, true] {
                        let mut w = super::verify181::world();
                        w.multiplayer_world = remote;
                        let mut e = create(&mut w, "Pig");
                        e.fire = fire;
                        e.set_position(-3.5, 85.25, 4.5);
                        if riding {
                            let rider = create(&mut w, "Pig");
                            e.riding = Some(rider.entity_id);
                        }
                        e.set_entity_flag(3, true);
                        e.set_entity_flag(1, true);
                        for _ in 0..3 {
                            e.on_entity_update(&mut w);
                            watched(&e, &mut o);
                            i(&mut o, e.fire);
                            i(&mut o, e.air);
                        }
                    }
                }
            }
        }
        "wolfremote5" => {
            for remote in [false, true] {
                for seed in 0..6 {
                    for flags in [0, 2, 4, 5, 6] {
                        for item in [-1, 352, 319, 320, 363, 364, 365, 366, 367, 260] {
                            for health in [0, 7, 20] {
                                let mut w = super::verify181::world();
                                w.multiplayer_world = remote;
                                let mut e = create(&mut w, "Wolf");
                                let mut p = create(&mut w, "Player");
                                e.rand.set_seed(seed);
                                e.set_position(-3.5, 64.0, 4.5);
                                if let EntityKind::Wolf(s) = &mut e.kind {
                                    s.flags = flags;
                                    s.owner = if seed % 2 == 0 { "pLaYeR" } else { "Other" }.into();
                                    s.watched_health = health;
                                }
                                e.sync_mob_watcher();
                                e.living.health = health;
                                e.living.is_jumping = true;
                                if item >= 0 {
                                    p.player_state_mut().unwrap().inventory.main[0] = Some(
                                        super::player::ItemStack::new(item, seed as i32 % 2 + 1, 0),
                                    );
                                }
                                let id = e.entity_id;
                                w.entities.insert(id, e);
                                o.push(p.use_on_entity(&mut w, id) as u8);
                                let e = w.entities.remove(&id).unwrap();
                                state(&mut w, &e, &mut o);
                                watched(&e, &mut o);
                                let stack = p.held_item();
                                i(&mut o, stack.map_or(-1, |v| v.id));
                                i(&mut o, stack.map_or(0, |v| v.count));
                            }
                        }
                    }
                }
            }
        }
        "health5" => {
            for name in [
                "Pig",
                "Wolf",
                "Zombie",
                "Skeleton",
                "Creeper",
                "Squid",
                "Slime",
                "Enderman",
                "Silverfish",
            ] {
                for remote in [false, true] {
                    for health in [0, 7, 20] {
                        for status in [0, 2, 3, 6, 7, 8] {
                            let mut w = super::verify181::world();
                            w.multiplayer_world = remote;
                            let mut e = create(&mut w, name);
                            e.set_position(-3.5, 70.25, 4.5);
                            e.living.health = health;
                            e.living.attacked_at_yaw = 47.0;
                            e.handle_health_update(&mut w, status);
                            state(&mut w, &e, &mut o);
                            e.perform_hurt_animation();
                            state(&mut w, &e, &mut o);
                        }
                    }
                }
            }
            for health in [-1, 0, 1, 19, 20, 100, i32::MAX] {
                for amount in [-20, -1, 0, 1, 20, i32::MAX, i32::MIN] {
                    let mut w = super::verify181::world();
                    let mut e = create(&mut w, "Pig");
                    e.living.health = health;
                    e.living.hearts_life = 47;
                    e.heal(amount);
                    state(&mut w, &e, &mut o);
                }
            }
        }
        "wolfattack5" => {
            for flags in [0, 4] {
                for seed in 0..24 {
                    for distance in [0.0, 1.49, 1.5, 2.0, 2.01, 5.99, 6.0] {
                        for ground in [false, true] {
                            for dy in [0, 3] {
                                let mut w = super::verify181::world();
                                let mut e = create(&mut w, "Wolf");
                                let mut p = create(&mut w, "Pig");
                                e.set_location_and_angles(-3.5, 70.25, 4.5, 35.0, 0.0);
                                p.set_position(0.5, 70.25 + dy as f64, 7.25);
                                let id = p.entity_id;
                                w.entity_joined_world(p);
                                if let EntityKind::Wolf(s) = &mut e.kind {
                                    s.flags = flags;
                                }
                                e.rand.set_seed(seed);
                                e.on_ground = ground;
                                e.motion.x = 0.125;
                                e.motion.z = -0.375;
                                e.living.attack_time = 17;
                                e.attack_target(&mut w, Target::Entity(id), distance, true);
                                state(&mut w, &e, &mut o);
                                let p = w.entities[&id].clone();
                                state(&mut w, &p, &mut o);
                            }
                        }
                    }
                }
            }
        }
        "wolffollow5" => {
            for distance in [5.0, 5.01, 12.0, 12.01, 25.0] {
                for scene in 0..4 {
                    let mut w = super::verify181::world();
                    let mut e = create(&mut w, "Wolf");
                    let mut target = create(&mut w, "Pig");
                    e.set_location_and_angles(-3.5, 64.0, 4.5, 35.0, -17.0);
                    target.set_position(-3.5 + distance, 64.0, 4.5);
                    if scene == 1 || scene == 3 {
                        for x in -5..=24 {
                            for z in 2..=7 {
                                for y in 64..=66 {
                                    raw(&mut w, x, y, z, if scene == 1 { 1 } else { 9 });
                                }
                            }
                        }
                    }
                    if scene == 2 {
                        for x in -6..=28 {
                            for z in 0..=8 {
                                raw(&mut w, x, 63, z, 0);
                            }
                        }
                    }
                    e.follow_owner(&mut w, target.pos, distance as f32);
                    state(&mut w, &e, &mut o);
                }
            }
            for remote in [false, true] {
                for item in [-1, 352, 319, 320, 363, 364, 365, 366, 367, 260] {
                    let mut w = super::verify181::world();
                    w.multiplayer_world = remote;
                    let mut e = create(&mut w, "Wolf");
                    let mut p = create(&mut w, "Player");
                    e.network.multiplayer_entity = true;
                    e.living.health = 7;
                    if let EntityKind::Wolf(s) = &mut e.kind {
                        s.flags |= 4;
                    }
                    e.living.current_target = Some(Target::Entity(p.entity_id));
                    if item >= 0 {
                        p.player_state_mut().unwrap().inventory.main[0] =
                            Some(super::player::ItemStack::new(item, 1, 0));
                    }
                    w.entities.insert(p.entity_id, p);
                    e.living_update(&mut w);
                    state(&mut w, &e, &mut o);
                }
            }
        }
        "vehiclecollision5" => {
            for kind in -1..3 {
                for name in [
                    "Pig",
                    "Player",
                    "Boat",
                    "Minecart0",
                    "Minecart1",
                    "Minecart2",
                ] {
                    for remote in [false, true] {
                        for offset in [0.0, 0.009, 0.01, 0.5, 4.0] {
                            for speed in [0.0, 0.1, 0.4] {
                                for reduction in [0.0, 0.5, 1.0] {
                                    let mut w = super::verify181::world();
                                    w.multiplayer_world = remote;
                                    let mut a = if kind < 0 {
                                        Entity::boat(&mut w, -3.5, 68.0, 4.5)
                                    } else {
                                        Entity::minecart(&mut w, -3.5, 68.0, 4.5, kind)
                                    };
                                    let mut b = create(&mut w, name);
                                    b.set_position(-3.5 + offset, 68.0, 4.5 + offset);
                                    b.prev_pos.x = 2.5;
                                    b.motion.x = -0.125;
                                    b.motion.z = 0.25;
                                    a.motion.x = speed;
                                    a.motion.z = speed / 2.0;
                                    a.collision_reduction = reduction;
                                    a.collide_entity(&mut w, &mut b);
                                    state(&mut w, &a, &mut o);
                                    state(&mut w, &b, &mut o);
                                }
                            }
                        }
                    }
                }
            }
        }
        "vehicleinteract5" => {
            for kind in -1..3 {
                for remote in [false, true] {
                    for occupied in 0..4 {
                        for count in [-1, 0, 1, 2] {
                            for item in [-1, 263, 1] {
                                let mut w = super::verify181::world();
                                w.multiplayer_world = remote;
                                let mut a = if kind < 0 {
                                    Entity::boat(&mut w, -3.5, 68.0, 4.5)
                                } else {
                                    Entity::minecart(&mut w, -3.5, 68.0, 4.5, kind)
                                };
                                let mut p = create(&mut w, "Player");
                                p.set_position(0.5, 68.0, 0.5);
                                if item >= 0 {
                                    p.player_state_mut().unwrap().inventory.main[0] =
                                        Some(super::player::ItemStack::new(item, count, 0));
                                }
                                if let EntityKind::Minecart(s) = &mut a.kind {
                                    if kind == 2 {
                                        s.fuel = i32::MAX - 100;
                                    }
                                }
                                if occupied > 0 {
                                    let mut r = if occupied == 1 {
                                        p.clone()
                                    } else {
                                        create(&mut w, if occupied == 2 { "Player" } else { "Pig" })
                                    };
                                    a.ridden_by = Some(r.entity_id);
                                    r.riding = Some(a.entity_id);
                                    if occupied == 1 {
                                        p.riding = r.riding;
                                    }
                                    w.entity_joined_world(r);
                                }
                                let id = a.entity_id;
                                w.entities.insert(id, a);
                                o.push(p.use_on_entity(&mut w, id) as u8);
                                let a = w.entities.remove(&id).unwrap();
                                state(&mut w, &a, &mut o);
                                state(&mut w, &p, &mut o);
                                i(&mut o,p.player_state().unwrap().events.iter().filter(|v|matches!(v,super::player::PlayerEvent::OpenVehicleInventory{..})).count() as i32);
                                let s = p.held_item();
                                i(&mut o, s.map_or(-1, |v| v.id));
                                i(&mut o, s.map_or(0, |v| v.count));
                                if let EntityKind::Minecart(s) = a.kind {
                                    i(&mut o, s.fuel);
                                    d(&mut o, s.push_x);
                                    d(&mut o, s.push_z);
                                }
                            }
                        }
                    }
                }
            }
        }
        "skeletondeath5" => {
            for source in 0..5 {
                for distance in [0.0, 49.99, 50.0, 50.01, 100.0] {
                    for remote in [false, true] {
                        let mut w = super::verify181::world();
                        w.multiplayer_world = remote;
                        let mut e = create(&mut w, "Skeleton");
                        let mut p = create(&mut w, "Player");
                        p.set_position(e.pos.x + distance, 70.0, e.pos.z);
                        let pid = p.entity_id;
                        w.entities.insert(pid, p);
                        let arrow = Entity::arrow(&mut w, 0.0, 0.0, 0.0);
                        let s = match source {
                            0 => DamageSource::new(DamageKind::Generic),
                            1 => DamageSource::entity(DamageKind::Player, Some(pid)),
                            2 => DamageSource::indirect(
                                DamageKind::Arrow,
                                arrow.entity_id,
                                Some(pid),
                            ),
                            3 => DamageSource::indirect(DamageKind::Arrow, arrow.entity_id, None),
                            _ => DamageSource::entity(DamageKind::Mob, Some(arrow.entity_id)),
                        };
                        i(&mut o, s.attacker.unwrap_or(-1));
                        i(&mut o, s.attacker.unwrap_or(-1));
                        e.on_living_death(&mut w, s);
                        state(&mut w, &e, &mut o);
                        i(&mut o, 0);
                    }
                }
            }
        }
        _ => panic!("unknown round 5 case {kind}"),
    }
    o
}
