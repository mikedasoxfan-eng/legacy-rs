//! Branch matrices paired with Mode_entitybranches181 (original bytecode).
use super::{
    damage::{DamageKind, DamageSource},
    living::Target,
    Entity, EntityKind, Vec3D,
};
fn i(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_le_bytes());
}
fn d(o: &mut Vec<u8>, v: f64) {
    o.extend(
        (if v.is_nan() {
            0x7ff8000000000000
        } else {
            v.to_bits()
        })
        .to_le_bytes(),
    );
}
fn text(o: &mut Vec<u8>, v: &str) {
    i(o, v.len() as i32);
    o.extend(v.as_bytes());
}
fn state(w: &mut crate::World, e: &Entity, o: &mut Vec<u8>) {
    super::verify181::state(w, e, o);
}
pub fn replay(kind: &str) -> Vec<u8> {
    let mut o = Vec::new();
    match kind {
        "factorybranches" => {
            for id in -1..101 {
                let mut w = super::verify181::world();
                let e = super::registry::create_by_id(&mut w, id);
                o.push(e.is_some() as u8);
                let Some(mut e) = e else { continue };
                text(&mut o, e.kind.name());
                i(&mut o, super::registry::by_kind(&e.kind).unwrap().id);
                state(&mut w, &e, &mut o);
                let named = super::registry::create(&mut w, e.kind.name()).unwrap();
                state(&mut w, &named, &mut o);
                e.set_location_and_angles(-3.5, 70.25, 4.5, 395.0, -377.0);
                e.motion = Vec3D::new(11.0, -0.25, -10.0);
                e.fire = 71;
                e.set_air(137);
                e.on_ground = true;
                if let EntityKind::Item { item, .. } = &mut e.kind {
                    *item = crate::world::tile::ItemStack::new(35, 7, 12);
                }
                if e.kind.is_living() {
                    e.living.health = 7;
                    e.living.hurt_time = 4;
                    e.living.death_time = 3;
                    e.living.attack_time = 2;
                    e.add_potion_effect(super::potion::PotionEffect::new(19, 137, 2));
                }
                if let EntityKind::Enderman(s) = &mut e.kind {
                    s.carried = 35;
                    s.carried_data = 12;
                }
                let loaded = Entity::from_nbt(&mut w, &e.to_nbt().unwrap()).unwrap();
                state(&mut w, &loaded, &mut o);
            }
            for name in [
                "Mob",
                "Monster",
                "Player",
                "Fireball",
                "ThrownEgg",
                "FishingHook",
                "LightningBolt",
                "unknown",
            ] {
                let mut w = super::verify181::world();
                o.push(super::registry::create(&mut w, name).is_some() as u8);
            }
        }
        "spawnorder" => {
            let mut set = crate::world::java_hash::JavaHashSet::default();
            for _ in 0..3 {
                set.clear();
                for x in -8i32..=8 {
                    for z in -8i32..=8 {
                        let hash = super::spawning::spawn_identity(x, z);
                        set.insert_with_identity(x ^ z, hash, (x, z));
                    }
                }
                for x in (-8i32..=8).rev() {
                    for z in (-8i32..=8).rev() {
                        o.push(set.insert_with_identity(
                            x ^ z,
                            super::spawning::spawn_identity(x, z),
                            (x, z),
                        ) as u8);
                    }
                }
                i(&mut o, set.len() as i32);
                for (x, z) in set.iter() {
                    i(&mut o, x);
                    i(&mut o, z);
                    i(&mut o, super::spawning::spawn_identity(x, z));
                }
            }
        }
        "spawnbranches" => {
            for scenario in 0..4 {
                for seed in 0..16 {
                    use crate::world::chunk::{idx, Chunk};
                    let mut w = super::verify181::world();
                    for x in -10..=10 {
                        for z in -10..=10 {
                            let mut c = Chunk::new(
                                x,
                                z,
                                vec![0; 32768].into_boxed_slice().try_into().unwrap(),
                            );
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
                    for _ in 0..2 {
                        i(&mut o, w.perform_spawning(scenario == 0, scenario != 0));
                        o.extend(w.rand.state().to_le_bytes());
                        o.extend(w.math_random.state().to_le_bytes());
                        i(&mut o, w.loaded_entity_list.len() as i32);
                        for id in w.loaded_entity_list.clone() {
                            let e = w.entities[&id].clone();
                            state(&mut w, &e, &mut o);
                            if let EntityKind::Sheep { fleece_color, .. } = e.kind {
                                i(&mut o, fleece_color as i32);
                            }
                        }
                    }
                }
            }
        }
        "slimebranches" => {
            for size in [1, 2, 3, 4, 8, 127, 128, 257] {
                for health in [-1, 0, 1] {
                    let mut w = super::verify181::world();
                    let mut e = Entity::living(&mut w, "Slime").unwrap();
                    e.set_slime_size(size);
                    e.living.health = health;
                    e.set_location_and_angles(-3.5, 70.25, 4.5, 35.0, 0.0);
                    e.set_dead(&mut w);
                    state(&mut w, &e, &mut o);
                    if let EntityKind::Slime { size, .. } = e.kind {
                        i(&mut o, size);
                    }
                    for id in w.loaded_entity_list.clone() {
                        let c = w.entities[&id].clone();
                        state(&mut w, &c, &mut o);
                        if let EntityKind::Slime { size, .. } = c.kind {
                            i(&mut o, size);
                        }
                    }
                }
            }
        }
        "skeletonbranches" => {
            for cooldown in [0, 1, 60] {
                for distance in [0.0, 5.0, 9.99, 10.0, 12.0] {
                    for yaw in [0.0, 35.0, 179.0] {
                        let mut w = super::verify181::world();
                        let mut e = Entity::living(&mut w, "Skeleton").unwrap();
                        let mut p = Entity::living(&mut w, "Pig").unwrap();
                        e.set_location_and_angles(-3.5, 70.25, 4.5, yaw, -17.0);
                        p.set_position(0.5, 71.125, 7.25);
                        e.living.attack_time = cooldown;
                        let id = p.entity_id;
                        w.entities.insert(id, p);
                        e.attack_target(&mut w, Target::Entity(id), distance, true);
                        state(&mut w, &e, &mut o);
                        o.push(e.living.has_attacked as u8);
                        for id in w.loaded_entity_list.clone() {
                            let c = w.entities[&id].clone();
                            state(&mut w, &c, &mut o);
                        }
                    }
                }
            }
        }
        "wolfbranches" => {
            for seed in 0..12 {
                for flags in [0, 2, 4, 5, 6] {
                    for item in [-1, 352, 319, 320, 363, 364, 365, 366, 367, 260] {
                        for health in [0, 7, 20] {
                            let mut w = super::verify181::world();
                            let mut e = Entity::living(&mut w, "Wolf").unwrap();
                            let mut p = Entity::player(&mut w, "Player");
                            e.set_location_and_angles(-3.5, 64.0, 4.5, 35.0, 0.0);
                            e.rand.set_seed(seed);
                            if let EntityKind::Wolf(s) = &mut e.kind {
                                s.flags = flags;
                                s.owner = if seed % 2 == 0 { "pLaYeR" } else { "Other" }.into();
                                s.watched_health = health;
                            }
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
                            if let EntityKind::Wolf(s) = &e.kind {
                                i(&mut o, s.flags as i32);
                                text(&mut o, &s.owner);
                                i(&mut o, s.watched_health);
                            }
                            let held = p.held_item();
                            i(&mut o, held.map_or(-1, |s| s.id));
                            i(&mut o, held.map_or(0, |s| s.count));
                        }
                    }
                }
            }
        }
        "vehiclebranches" => {
            for kind in -1..3 {
                for damage in [-1, 0, 1, 4, 5, 100, i32::MAX] {
                    for dead in [false, true] {
                        let mut w = super::verify181::world();
                        let mut e = if kind < 0 {
                            Entity::boat(&mut w, -3.5, 68.0, 4.5)
                        } else {
                            Entity::minecart(&mut w, -3.5, 68.0, 4.5, kind)
                        };
                        e.dead = dead;
                        if let EntityKind::Minecart(s) = &mut e.kind {
                            s.set_stack(0, Some(super::player::ItemStack::new(35, 64, 7)));
                            s.set_stack(26, Some(super::player::ItemStack::new(263, 9, 0)));
                        }
                        for _ in 0..3 {
                            o.push(e.attack_from(
                                &mut w,
                                DamageSource::new(DamageKind::Generic),
                                damage,
                            ) as u8);
                            state(&mut w, &e, &mut o);
                            match &e.kind {
                                EntityKind::Boat(s) => {
                                    for v in [s.damage, s.time_since_hit, s.rock_direction] {
                                        i(&mut o, v);
                                    }
                                }
                                EntityKind::Minecart(s) => {
                                    for v in [s.damage, s.time_since_hit, s.rock_direction] {
                                        i(&mut o, v);
                                    }
                                    for slot in [0, 26] {
                                        i(&mut o, s.items[slot].unwrap().count);
                                    }
                                }
                                _ => unreachable!(),
                            }
                            for id in &w.loaded_entity_list {
                                let c = &w.entities[id];
                                for v in [c.motion.x, c.motion.y, c.motion.z] {
                                    d(&mut o, v);
                                }
                                if let EntityKind::Item { item, .. } = &c.kind {
                                    for v in [item.id as i32, item.count, item.damage as i32] {
                                        i(&mut o, v);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        "fireballbranches" => {
            for source in 0..4 {
                for yaw in [0.0, 35.0, 179.0, -270.0] {
                    for pitch in [-90.0, -17.0, 0.0, 65.0, 90.0] {
                        let mut w = super::verify181::world();
                        let mut e = Entity::fireball(
                            &mut w,
                            Vec3D::new(-3.5, 85.0, 4.5),
                            Vec3D::new(0.4, 0.2, 1.0),
                        );
                        let mut p = if source == 1 {
                            Entity::living(&mut w, "Pig").unwrap()
                        } else {
                            Entity::player(&mut w, "Player")
                        };
                        p.yaw = yaw;
                        p.pitch = pitch;
                        let id = p.entity_id;
                        w.entities.insert(id, p);
                        let arrow = Entity::arrow(&mut w, 0.0, 0.0, 0.0);
                        let damage = match source {
                            0 => DamageSource::new(DamageKind::Generic),
                            1 => DamageSource::entity(DamageKind::Mob, Some(id)),
                            _ => DamageSource::indirect(
                                DamageKind::Arrow,
                                arrow.entity_id,
                                if source == 2 { Some(id) } else { None },
                            ),
                        };
                        o.push(e.attack_from(&mut w, damage, 4) as u8);
                        state(&mut w, &e, &mut o);
                        if let EntityKind::Fireball(s) = e.kind {
                            for v in [s.acceleration.x, s.acceleration.y, s.acceleration.z] {
                                d(&mut o, v);
                            }
                        }
                    }
                }
            }
        }
        _ => panic!("unknown entity branch fixture: {kind}"),
    }
    o
}
