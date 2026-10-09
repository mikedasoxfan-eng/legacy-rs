//! Branch matrices paired with Mode_entityround4_181, invoking original bytecode.
use super::{
    damage::{DamageKind, DamageSource},
    Entity, EntityKind, Vec3D,
};
use crate::{
    world::{chunk::idx, tile::ItemStack},
    World,
};
fn i(o: &mut Vec<u8>, n: i32) {
    o.extend(n.to_le_bytes());
}
fn state(w: &mut World, e: &Entity, o: &mut Vec<u8>) {
    super::verify181::state(w, e, o);
    match &e.kind {
        EntityKind::PigZombie { anger, sound_delay } => {
            i(o, *anger);
            i(o, *sound_delay);
        }
        EntityKind::Wolf(s) => {
            for mask in [2, 4, 1] {
                o.push((s.flags & mask != 0) as u8);
            }
        }
        EntityKind::Boat(s) => {
            for n in [
                s.damage,
                s.time_since_hit,
                s.rock_direction,
                e.network.increments,
            ] {
                i(o, n);
            }
        }
        EntityKind::Minecart(s) => {
            for n in [
                s.damage,
                s.time_since_hit,
                s.rock_direction,
                e.network.increments,
            ] {
                i(o, n);
            }
        }
        _ => {}
    }
    if e.kind.is_living() {
        i(o, e.network.increments);
    }
}
fn create(w: &mut World, name: &str) -> Entity {
    match name {
        "Fish" => Entity::fish_base(w),
        "Item" => Entity::item(w, -3.5, 68.0, 4.5, ItemStack::new(263, 1, 0)),
        "Boat" => Entity::boat(w, -3.5, 68.0, 4.5),
        "Minecart" => Entity::minecart(w, -3.5, 68.0, 4.5, 0),
        "Player" => Entity::player(w, "Player"),
        _ => Entity::living(w, name).unwrap(),
    }
}
fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8) {
    let c = w.chunk_mut(x >> 4, z >> 4);
    c.blocks[idx(x & 15, y, z & 15)] = id;
    c.data.set(x & 15, y, z & 15, 0);
}
fn loaded(w: &mut World, o: &mut Vec<u8>) {
    for id in w.loaded_entity_list.clone() {
        let e = w.entities[&id].clone();
        state(w, &e, o);
    }
}
pub fn replay(kind: &str) -> Vec<u8> {
    let mut o = Vec::new();
    match kind {
        "creeperdeath4" => {
            for source in 0..6 {
                for remote in [false, true] {
                    for seed in 0..3 {
                        let mut w = super::verify181::world();
                        w.multiplayer_world = remote;
                        let mut e = create(&mut w, "Creeper");
                        e.set_position(-3.5, 85.25, 4.5);
                        e.rand.set_seed(seed);
                        e.living.health = 4;
                        let attacker =
                            create(&mut w, if source == 2 { "Player" } else { "Skeleton" });
                        let aid = attacker.entity_id;
                        w.entity_joined_world(attacker);
                        let arrow = Entity::arrow(&mut w, 0.0, 0.0, 0.0);
                        let damage = match source {
                            0 => DamageSource::new(DamageKind::Generic),
                            1 => DamageSource::entity(DamageKind::Mob, Some(aid)),
                            2 => DamageSource::entity(DamageKind::Player, Some(aid)),
                            3 => DamageSource::indirect(
                                DamageKind::Arrow,
                                arrow.entity_id,
                                Some(aid),
                            ),
                            4 => DamageSource::indirect(DamageKind::Arrow, arrow.entity_id, None),
                            _ => {
                                let ball =
                                    Entity::base(&mut w, EntityKind::Fireball(Default::default()));
                                DamageSource::indirect(
                                    DamageKind::Fireball,
                                    ball.entity_id,
                                    Some(aid),
                                )
                            }
                        };
                        o.push(e.attack_from(&mut w, damage, 10) as u8);
                        state(&mut w, &e, &mut o);
                        i(&mut o, w.loaded_entity_list.len() as i32);
                        for id in w.loaded_entity_list.clone() {
                            let a = w.entities[&id].clone();
                            state(&mut w, &a, &mut o);
                            if let EntityKind::Item { item, .. } = a.kind {
                                for v in [item.id as i32, item.count as i32, item.damage as i32] {
                                    i(&mut o, v);
                                }
                            }
                        }
                    }
                }
            }
        }
        "sun4" => {
            for name in ["Zombie", "Skeleton", "PigZombie"] {
                for remote in [false, true] {
                    for day in [false, true] {
                        for covered in [false, true] {
                            for seed in 0..16 {
                                let mut w = super::verify181::world();
                                w.multiplayer_world = remote;
                                w.skylight_subtracted = if day { 0 } else { 11 };
                                let mut e = create(&mut w, name);
                                e.set_position(-3.5, 85.25, 4.5);
                                e.network.multiplayer_entity = true;
                                e.rand.set_seed(seed);
                                e.fire = 13;
                                if covered {
                                    raw(&mut w, -4, 88, 4, 1);
                                }
                                for _ in 0..8 {
                                    e.on_update(&mut w);
                                    state(&mut w, &e, &mut o);
                                }
                            }
                        }
                    }
                }
            }
        }
        "network4" => {
            for name in ["Item", "Zombie", "Ghast", "Boat", "Minecart", "Fish"] {
                for remote in [false, true] {
                    for count in [-4, 0, 1, 3, 7] {
                        for y in [62.5, 64.0, 70.25] {
                            for yaw in [-720.0, -179.0, 181.0, 721.0] {
                                let mut w = super::verify181::world();
                                w.multiplayer_world = remote;
                                let mut e = create(&mut w, name);
                                e.set_position(-3.5, 68.0, 4.5);
                                e.yaw = 179.0;
                                e.pitch = -179.0;
                                e.on_ground = true;
                                e.fire = 41;
                                if e.kind.is_living() {
                                    e.network.multiplayer_entity = true;
                                }
                                e.set_velocity(0.125, -0.25, 0.375);
                                e.motion = Vec3D::new(0.9, 0.8, 0.7);
                                e.set_position_and_rotation2(
                                    &mut w,
                                    Vec3D::new(-2.25, y, 5.75),
                                    yaw,
                                    541.0,
                                    count,
                                );
                                state(&mut w, &e, &mut o);
                                // A base hook has no angler: only execute its interpolation branch.
                                for _ in 0..if name == "Fish" { count.max(0) } else { 12 } {
                                    e.on_update(&mut w);
                                    state(&mut w, &e, &mut o);
                                }
                            }
                        }
                    }
                }
            }
        }
        "lightning4" => {
            for name in [
                "Pig",
                "Cow",
                "Chicken",
                "Sheep",
                "Wolf",
                "Zombie",
                "Skeleton",
                "Spider",
                "CaveSpider",
                "Creeper",
                "PigZombie",
                "Ghast",
                "Squid",
                "Slime",
                "Silverfish",
                "Player",
            ] {
                for fire in [-1, 0, 19, 20, 100] {
                    for hearts in [0, 20] {
                        let mut w = super::verify181::world();
                        let mut e = create(&mut w, name);
                        e.set_position(-3.5, 68.0, 4.5);
                        e.living.health = 20;
                        e.living.hearts_life = hearts;
                        e.fire = fire;
                        if let Some(p) = e.player_state_mut() {
                            p.inventory.armor[3] = Some(super::player::ItemStack::new(306, 1, 0));
                        }
                        let _bolt = Entity::lightning(&mut w, 0.0, 85.0, 0.0);
                        for _ in 0..3 {
                            e.struck_by_lightning(&mut w);
                            state(&mut w, &e, &mut o);
                        }
                    }
                }
            }
            for difficulty in 0..4 {
                for seed in 0..8 {
                    let mut w = super::verify181::world();
                    w.difficulty = difficulty;
                    w.entity_seed_source = super::EntitySeedSource::Deterministic(100 + seed);
                    let mut bolt = Entity::lightning(&mut w, -3.5, 64.0, 4.5);
                    for name in ["Pig", "Creeper", "Wolf", "PigZombie"] {
                        let mut e = create(&mut w, name);
                        e.set_position(-3.5, 64.0, 4.5);
                        w.entity_joined_world(e);
                    }
                    for _ in 0..40 {
                        bolt.on_update(&mut w);
                        state(&mut w, &bolt, &mut o);
                        if let EntityKind::Lightning {
                            state,
                            seed,
                            remaining,
                        } = bolt.kind
                        {
                            i(&mut o, state);
                            i(&mut o, remaining);
                            o.extend(seed.to_le_bytes());
                        }
                        i(&mut o, w.lightning_sky_flash);
                        for x in -5..=-2 {
                            for y in 63..=65 {
                                for z in 3..=6 {
                                    let id = w.get_block_id(x, y, z);
                                    i(&mut o, id as i32);
                                }
                            }
                        }
                        loaded(&mut w, &mut o);
                    }
                }
            }
        }
        "retaliation4" => {
            for name in ["PigZombie", "Wolf"] {
                for source in 0..6 {
                    for damage in [0, 4, 25] {
                        for hearts in [0, 20] {
                            for dead in [false, true] {
                                let mut w = super::verify181::world();
                                let mut e = create(&mut w, name);
                                e.set_position(-3.5, 68.0, 4.5);
                                e.living.health = 20;
                                e.living.hearts_life = hearts;
                                e.dead = dead;
                                let mut attacker =
                                    create(&mut w, if source == 2 { "Zombie" } else { "Player" });
                                attacker.set_position(0.5, 68.0, 0.5);
                                let aid = attacker.entity_id;
                                w.entity_joined_world(attacker);
                                for j in 0..5 {
                                    let mut a = create(&mut w, name);
                                    a.set_position(
                                        -3.5 + if j == 4 { 40.0 } else { j as f64 * 3.0 },
                                        68.0,
                                        4.5,
                                    );
                                    if j == 2 {
                                        if let EntityKind::Wolf(s) = &mut a.kind {
                                            s.flags |= 4;
                                        }
                                    }
                                    w.entity_joined_world(a);
                                }
                                let arrow = Entity::arrow(&mut w, 0.0, 0.0, 0.0);
                                let s = match source {
                                    0 => DamageSource::new(DamageKind::Generic),
                                    1 => DamageSource::entity(DamageKind::Player, Some(aid)),
                                    2 => DamageSource::entity(DamageKind::Mob, Some(aid)),
                                    3 => DamageSource::indirect(
                                        DamageKind::Arrow,
                                        arrow.entity_id,
                                        Some(aid),
                                    ),
                                    4 => DamageSource::indirect(
                                        DamageKind::Arrow,
                                        arrow.entity_id,
                                        None,
                                    ),
                                    _ => {
                                        let ball = Entity::base(
                                            &mut w,
                                            EntityKind::Fireball(Default::default()),
                                        );
                                        DamageSource::indirect(
                                            DamageKind::Fireball,
                                            ball.entity_id,
                                            Some(aid),
                                        )
                                    }
                                };
                                for _ in 0..3 {
                                    o.push(e.attack_from(&mut w, s, damage) as u8);
                                    state(&mut w, &e, &mut o);
                                    loaded(&mut w, &mut o);
                                }
                            }
                        }
                    }
                }
            }
        }
        "impact4" => {
            for name in ["Pig", "Ghast", "Enderman", "Creeper"] {
                for owner in 0..3 {
                    for remote in [false, true] {
                        for block in [false, true] {
                            let mut w = super::verify181::world();
                            w.multiplayer_world = remote;
                            let mut target = create(&mut w, name);
                            target.set_position(-3.5, 85.25, 5.5);
                            let tid = target.entity_id;
                            w.entity_joined_world(target);
                            let shooter = if owner == 0 {
                                None
                            } else {
                                let mut e =
                                    create(&mut w, if owner == 1 { "Zombie" } else { "Player" });
                                e.set_position(0.0, 90.0, 0.0);
                                w.entity_joined_world(e.clone());
                                Some(e)
                            };
                            let aim = Vec3D::new(0.4, 0.2, 1.0);
                            let mut ball = if let Some(e) = &shooter {
                                Entity::fireball_from_owner(&mut w, e, aim)
                            } else {
                                Entity::fireball(&mut w, Vec3D::new(-3.5, 85.25, 4.5), aim)
                            };
                            ball.set_position(-3.5, 85.25, 4.5);
                            ball.set_velocity(0.0, 0.0, 1.0);
                            if block {
                                raw(&mut w, -4, 85, 5, 1);
                            }
                            state(&mut w, &ball, &mut o);
                            ball.on_update(&mut w);
                            state(&mut w, &ball, &mut o);
                            let target = w.entities[&tid].clone();
                            state(&mut w, &target, &mut o);
                        }
                    }
                }
            }
        }
        "flying4" => {
            for medium in 0..3 {
                for ground in [false, true] {
                    for strafe in [-1.0, 0.0, 0.25] {
                        for forward in [-1.0, 0.0, 1.0] {
                            for yaw in [0.0, 35.0, 181.0] {
                                let mut w = super::verify181::world();
                                let mut e = create(&mut w, "Ghast");
                                e.set_position(-3.5, 64.0, 4.5);
                                e.yaw = yaw;
                                e.on_ground = ground;
                                e.motion = Vec3D::new(0.125, -0.25, 0.375);
                                e.prev_pos = Vec3D::new(-3.75, 64.0, 4.125);
                                if medium != 0 {
                                    for x in -6..=0 {
                                        for y in 64..=69 {
                                            for z in 2..=7 {
                                                raw(
                                                    &mut w,
                                                    x,
                                                    y,
                                                    z,
                                                    if medium == 1 { 9 } else { 11 },
                                                );
                                            }
                                        }
                                    }
                                }
                                e.in_water = medium == 1;
                                for _ in 0..4 {
                                    e.flying_heading(&mut w, strafe, forward);
                                    state(&mut w, &e, &mut o);
                                    o.push(e.is_on_ladder(&mut w) as u8);
                                }
                            }
                        }
                    }
                }
            }
        }
        "explosionblocks4" => {
            for size in [0.25, 1.0, 2.0, 4.0] {
                for scene in 0..8 {
                    for flame in [false, true] {
                        let mut w = super::verify181::world();
                        let blocks = [1, 5, 20, 35, 46, 98, 87, 2];
                        for x in -6i32..=-1 {
                            for y in 83..=86 {
                                for z in 2..=7 {
                                    let id = if scene == 7 {
                                        blocks[(x + y + z).rem_euclid(blocks.len() as i32) as usize]
                                    } else {
                                        blocks[scene]
                                    };
                                    raw(&mut w, x, y, z, id);
                                    w.chunk_mut(x >> 4, z >> 4).data.set(
                                        x & 15,
                                        y,
                                        z & 15,
                                        if id == 35 {
                                            12
                                        } else if id == 98 {
                                            2
                                        } else {
                                            0
                                        },
                                    );
                                }
                            }
                        }
                        let ex = w.new_explosion(None, Vec3D::new(-3.5, 85.25, 4.5), size, flame);
                        i(&mut o, ex.destroyed_blocks.len() as i32);
                        for (x, y, z) in ex.destroyed_blocks {
                            for v in [x, y, z] {
                                i(&mut o, v);
                            }
                        }
                        for x in -7..=0 {
                            for y in 82..=87 {
                                for z in 1..=8 {
                                    let id = w.get_block_id(x, y, z);
                                    let meta = w.get_block_metadata(x, y, z);
                                    i(&mut o, id as i32);
                                    i(&mut o, meta);
                                }
                            }
                        }
                        i(&mut o, w.loaded_entity_list.len() as i32);
                        for id in w.loaded_entity_list.clone() {
                            let e = w.entities[&id].clone();
                            state(&mut w, &e, &mut o);
                            if let EntityKind::Item { item, .. } = e.kind {
                                for v in [item.id as i32, item.count as i32, item.damage as i32] {
                                    i(&mut o, v);
                                }
                            }
                            if let EntityKind::Tnt { fuse } = e.kind {
                                i(&mut o, fuse);
                            }
                        }
                        o.extend(w.rand.state().to_le_bytes());
                        o.extend(w.math_random.state().to_le_bytes());
                        crate::world::events::write_events(&w.drain_events(), &mut o);
                    }
                }
            }
        }
        "creeper4" => {
            for powered in [false, true] {
                for distance in [2.99, 3.0, 6.99, 7.0, 10.0] {
                    for visible in [false, true] {
                        for initial in [-1, 1] {
                            for ignition in [0, 1, 29] {
                                for remote in [false, true] {
                                    let mut w = super::verify181::world();
                                    w.multiplayer_world = remote;
                                    let mut e = create(&mut w, "Creeper");
                                    e.set_position(-3.5, 85.25, 4.5);
                                    if let EntityKind::Creeper {
                                        ignition: i,
                                        state: s,
                                        powered: p,
                                        ..
                                    } = &mut e.kind
                                    {
                                        *i = ignition;
                                        *s = initial;
                                        *p = powered;
                                    }
                                    let mut p = create(&mut w, "Pig");
                                    p.set_position(-3.5, 85.25, 5.5);
                                    let pid = p.entity_id;
                                    w.entity_joined_world(p);
                                    for _ in 0..2 {
                                        e.attack_target(
                                            &mut w,
                                            super::living::Target::Entity(pid),
                                            distance,
                                            visible,
                                        );
                                        state(&mut w, &e, &mut o);
                                        if let EntityKind::Creeper {
                                            ignition, state, ..
                                        } = e.kind
                                        {
                                            i(&mut o, ignition);
                                            i(&mut o, state);
                                        }
                                        o.push(e.living.has_attacked as u8);
                                        let p = w.entities[&pid].clone();
                                        state(&mut w, &p, &mut o);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        _ => panic!("unknown round 4 case {kind}"),
    }
    o
}
