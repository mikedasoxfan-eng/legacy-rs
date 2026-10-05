//! Focused entity oracle, deliberately independent of terrain/world dump formats.
use super::{
    damage::{DamageKind, DamageSource},
    potion::PotionEffect,
    Entity, EntityKind, Vec3D,
};
use crate::{
    world::chunk::{idx, Chunk},
    World,
};
fn i(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_le_bytes());
}
fn l(o: &mut Vec<u8>, v: i64) {
    o.extend(v.to_le_bytes());
}
fn f(o: &mut Vec<u8>, v: f32) {
    o.extend((if v.is_nan() { 0x7fc00000 } else { v.to_bits() }).to_le_bytes());
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
fn text(o: &mut Vec<u8>, s: &str) {
    i(o, s.len() as i32);
    o.extend(s.as_bytes());
}
pub fn world() -> World {
    let mut w = World::new(0, 300);
    w.set_entity_determinism(100, 200, 0);
    w.difficulty = 2;
    w.skylight_subtracted = 11;
    w.world_time = 18000;
    for x in -3..=3 {
        for z in -3..=3 {
            let mut c = Chunk::new(x, z, vec![0; 32768].into_boxed_slice().try_into().unwrap());
            c.loaded = true;
            c.populated = true;
            c.sky_light.0.fill(255);
            c.height_map.fill(64);
            for bx in 0..16 {
                for bz in 0..16 {
                    c.blocks[idx(bx, 63, bz)] = 1;
                }
            }
            w.chunks.insert((x, z), Box::new(c));
        }
    }
    w
}
pub(crate) fn state(w: &mut World, e: &Entity, o: &mut Vec<u8>) {
    for v in [
        e.pos.x,
        e.pos.y,
        e.pos.z,
        e.motion.x,
        e.motion.y,
        e.motion.z,
        e.prev_pos.x,
        e.prev_pos.y,
        e.prev_pos.z,
        e.bounding_box.min_x,
        e.bounding_box.min_y,
        e.bounding_box.min_z,
        e.bounding_box.max_x,
        e.bounding_box.max_y,
        e.bounding_box.max_z,
    ] {
        d(o, v);
    }
    for v in [
        e.yaw,
        e.pitch,
        e.prev_yaw,
        e.prev_pitch,
        e.width,
        e.height,
        e.y_offset,
        e.y_size,
        e.step_height,
        e.fall_distance,
        e.distance_walked,
    ] {
        f(o, v);
    }
    for v in [e.ticks_existed, e.fire, e.air] {
        i(o, v);
    }
    for v in [
        e.on_ground,
        e.dead,
        e.in_water,
        e.been_attacked,
        e.velocity_changed,
    ] {
        o.push(v as u8);
    }
    if e.kind.is_living() {
        let a = &e.living;
        for v in [
            a.health,
            a.prev_health,
            a.hearts_life,
            a.hurt_time,
            a.max_hurt_time,
            a.death_time,
            a.attack_time,
            a.entity_age,
            a.last_damage,
            a.experience_value,
            a.arrows_stuck,
            a.arrow_timer,
        ] {
            i(o, v);
        }
        for v in [
            a.attacked_at_yaw,
            a.render_yaw_offset,
            a.prev_render_yaw_offset,
            a.prev_limb_amount,
            a.limb_amount,
            a.limb_swing,
            a.move_strafing,
            a.move_forward,
            a.random_yaw_velocity,
            a.move_speed,
            a.ground_acceleration,
            a.air_acceleration,
        ] {
            f(o, v);
        }
        o.push(a.is_jumping as u8);
        let mut effects = a.effects.clone();
        effects.sort_by_key(|e| e.id);
        i(o, effects.len() as i32);
        for e in effects {
            i(o, e.id);
            i(o, e.duration);
            i(o, e.amplifier);
        }
        if !matches!(
            e.kind,
            EntityKind::Player(..) | EntityKind::Ghast(..) | EntityKind::Slime { .. }
        ) {
            i(o, a.flee_ticks);
            i(
                o,
                a.target.and_then(|t| w.target_entity_id(t)).unwrap_or(-1),
            );
        }
    }
    match &e.kind {
        EntityKind::Enderman(s) => {
            i(o, s.carried as i32);
            i(o, s.carried_data as i32);
            o.push(s.screaming as u8);
            i(o, s.teleport_ticks);
            i(o, s.stare_ticks);
        }
        EntityKind::Silverfish { ally_timer } => i(o, *ally_timer),
        EntityKind::XPOrb(s) => {
            for v in [
                s.ticks,
                s.age,
                s.pickup_delay,
                s.health,
                s.value,
                super::xp::texture_index(s.value),
            ] {
                i(o, v);
            }
            f(o, s.phase);
        }
        EntityKind::Arrow(s) => {
            o.push(s.critical as u8);
            i(o, s.ticks_ground);
            i(o, s.ticks_air);
        }
        _ => {}
    }
    l(o, e.rand.state());
    l(o, w.rand.state());
    l(o, w.math_random.state());
    i(o, w.loaded_entity_list.len() as i32);
    for id in &w.loaded_entity_list {
        let e = &w.entities[id];
        if let Some(entry) = super::registry::by_kind(&e.kind) { text(o,entry.name); } else { i(o,-1); }
        d(o, e.pos.x);
        d(o, e.pos.y);
        d(o, e.pos.z);
        if let EntityKind::XPOrb(s) = &e.kind {
            i(o, s.value);
        }
    }
    crate::world::events::write_events(&w.drain_events(), o);
}
pub fn rules() -> Vec<u8> {
    let mut o = vec![];
    for kind in [
        DamageKind::InFire,
        DamageKind::OnFire,
        DamageKind::Lava,
        DamageKind::InWall,
        DamageKind::Drown,
        DamageKind::Starve,
        DamageKind::Cactus,
        DamageKind::Fall,
        DamageKind::OutOfWorld,
        DamageKind::Generic,
        DamageKind::Explosion,
        DamageKind::Magic,
        DamageKind::Mob,
        DamageKind::Arrow,
    ] {
        let s = DamageSource::new(kind);
        text(&mut o, s.name());
        o.push(s.bypasses_armor() as u8);
        o.push(s.harms_creative() as u8);
        f(&mut o, s.exhaustion());
    }
    for id in 1..=19 {
        for amp in -1..36 {
            for ticks in [-1, 0, 1, 2, 7, 12, 24, 25, 50, 100, 999] {
                o.push(super::potion::is_ready(id, ticks, amp) as u8);
            }
        }
    }
    for amp in 0..5 {
        for newer in -1..6 {
            let mut p = PotionEffect::new(19, 200, amp);
            p.combine(PotionEffect::new(19, 7, newer));
            i(&mut o, p.id);
            i(&mut o, p.duration);
            i(&mut o, p.amplifier);
        }
    }
    for id in 0..256 {
        o.push(super::mobs181::enderman_can_carry(id as u8) as u8);
    }
    for value in -2..10000 {
        i(&mut o, super::xp::split_xp(value));
        i(&mut o, super::xp::texture_index(value));
    }
    let mut w = world();
    let mut e = Entity::living(&mut w, "Enderman").unwrap();
    if let EntityKind::Enderman(s) = &mut e.kind {
        s.carried = 35;
        s.carried_data = 7;
    }
    let n = e.to_nbt().unwrap();
    i(&mut o, n.short("carried") as i32);
    i(&mut o, n.short("carriedData") as i32);
    e.read_existing_nbt(&mut w, &n).unwrap();
    if let EntityKind::Enderman(s) = &e.kind {
        i(&mut o, s.carried as i32);
        i(&mut o, s.carried_data as i32);
    }
    for bi in 0..10 {
        for cat in 0..3 {
            let list = super::spawning::spawn_list181(bi, cat);
            i(&mut o, list.len() as i32);
            for (name, weight, min, max) in list {
                text(&mut o, name);
                i(&mut o, weight);
                i(&mut o, min);
                i(&mut o, max);
            }
        }
    }
    let mut w = world();
    let mut player = Entity::player(&mut w, "Player");
    for value in [1, 3, 7, 17, 37, 73, 149, 307, 617, 1237, 2477, 10000, -2] {
        for delay in [0, 1] {
            for cooldown in [0, 1] {
                let mut orb = Entity::xp_orb(&mut w, Vec3D::new(-3.5, 68.0, 4.5), value);
                if let EntityKind::XPOrb(state) = &mut orb.kind {
                    state.pickup_delay = delay;
                }
                player.player_state_mut().unwrap().xp_pickup_cooldown = cooldown;
                orb.pickup_xp_orb(&mut w, &mut player);
                let p = player.player_state().unwrap();
                for v in [
                    p.experience.points,
                    p.experience.level,
                    p.experience.total,
                    p.xp_pickup_cooldown,
                ] {
                    i(&mut o, v);
                }
                o.push(orb.dead as u8);
                l(&mut o, orb.rand.state());
                crate::world::events::write_events(&w.drain_events(), &mut o);
            }
        }
    }
    o
}
pub fn population() -> Vec<u8> {
    let mut out = vec![];
    for biome in 0..10 {
        for seed in 4096..4128 {
            let mut w = world();
            let mut random = crate::JavaRandom::new(seed);
            w.spawn_population_animals(biome, -8, -7, 16, 12, &mut random);
            l(&mut out, random.state());
            l(&mut out, w.rand.state());
            l(&mut out, w.math_random.state());
            i(&mut out, w.loaded_entity_list.len() as i32);
            for id in &w.loaded_entity_list {
                let e = &w.entities[id];
                text(&mut out, e.kind.name());
                for v in [e.pos.x, e.pos.y, e.pos.z] {
                    d(&mut out, v);
                }
                f(&mut out, e.yaw);
                l(&mut out, e.rand.state());
                if let EntityKind::Sheep { fleece_color, .. } = e.kind {
                    i(&mut out, fleece_color as i32);
                }
            }
        }
    }
    out
}
pub fn replay(kind: &str, ticks: i32) -> Vec<u8> {
    if kind == "explosion" { return super::combat_verify181::explosion_replay(); }
    if kind == "combat" { return super::combat_verify181::replay(); }
    if matches!(kind, "watchers" | "coordinates" | "light" | "registry") { return super::api_verify181::replay(kind); }
    if kind == "rules" {
        return rules();
    }
    if kind == "population" {
        return population();
    }
    let mut w = world();
    let mut e = match kind {
        "XPOrb" | "attract" => Entity::xp_orb(&mut w, Vec3D::new(-3.5, 68.0, 4.5), 149),
        "Boat" => Entity::boat(&mut w, -3.5, 68.0, 4.5),
        "Minecart" => Entity::minecart(&mut w, -3.5, 68.0, 4.5, 0),
        "Item" => Entity::item(
            &mut w,
            -3.5,
            68.0,
            4.5,
            crate::world::tile::ItemStack::new(263, 1, 0),
        ),
        "Fireball" => Entity::fireball(
            &mut w,
            Vec3D::new(-3.5, 68.0, 4.5),
            Vec3D::new(0.4, 0.2, 1.0),
        ),
        "Snowball" | "Egg" => {
            let mut e = Entity::throwable_at(&mut w, -3.5, 68.0, 4.5, kind == "Egg");
            e.set_arrow_heading(0.4, 0.2, 1.0, 1.5, 1.0);
            e
        }
        "Lightning" => Entity::lightning(&mut w, -3.5, 64.0, 4.5),
        "Arrow" => {
            let mut e = Entity::arrow(&mut w, -3.5, 68.0, 4.5);
            e.set_arrow_heading(0.4, 0.2, 1.0, 1.5, 1.0);
            if let EntityKind::Arrow(s) = &mut e.kind {
                s.critical = true;
            }
            e
        }
        _ => {
            let mut e = Entity::living(
                &mut w,
                if kind.starts_with("kill-") {
                    &kind[5..]
                } else if kind == "poison" {
                    "Zombie"
                } else if kind == "teleport"
                    || kind == "carry"
                    || kind == "indirect"
                    || kind.starts_with("stare")
                {
                    "Enderman"
                } else if kind == "allies" {
                    "Silverfish"
                } else if kind == "potions" || kind == "damage" {
                    "Pig"
                } else {
                    kind
                },
            )
            .unwrap();
            e.set_location_and_angles(-3.5, 64.0, 4.5, 35.0, 0.0);
            e
        }
    };
    if kind == "potions" {
        e.living.health = 8;
        for p in [
            [1, 80, 1],
            [2, 120, 0],
            [10, 150, 0],
            [19, 180, 0],
            [6, 3, 0],
            [7, 2, 0],
        ] {
            e.add_potion_effect(PotionEffect::new(p[0], p[1], p[2]));
        }
    }
    if kind == "carry" {
        for x in -8..=8 {
            for z in -8..=8 {
                w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap().blocks[idx(x & 15, 64, z & 15)] = 37;
            }
        }
    }
    if kind == "allies" {
        for x in -7..=7 {
            for z in -7..=7 {
                w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap().blocks[idx(x & 15, 63, z & 15)] = 97;
            }
        }
    }
    let mut killer = None;
    if kind.starts_with("kill-") || kind == "attract" || kind.starts_with("stare") {
        let mut p = Entity::player(&mut w, "Player");
        p.entity_id = 10000;
        p.set_position(0.5, 65.62, 0.5);
        if kind.starts_with("stare") {
            p.yaw = 45.0;
            p.pitch = 0.5;
            if kind == "starepumpkin" {
                p.player_state_mut().unwrap().inventory.armor[3] =
                    Some(crate::world::tile::ItemStack::new(86, 1, 0).into());
            }
        }
        w.players.push((0.5, 65.62, 0.5));
        w.player_entities.push(Some(10000));
        w.entities.insert(10000, p);
        killer = Some(10000);
    }
    if kind == "poison" || kind == "allies" {
        let mut spider = Entity::living(&mut w, "CaveSpider").unwrap();
        spider.set_position(-3.5, 64.0, 3.5);
        w.entities.insert(spider.entity_id, spider.clone());
        if e.attack_from(
            &mut w,
            DamageSource::entity(DamageKind::Mob, Some(spider.entity_id)),
            2,
        ) && kind == "poison"
        {
            e.add_potion_effect(PotionEffect::new(19, 140, 0));
        }
    }
    let mut out = vec![];
    state(&mut w, &e, &mut out);
    for t in 0..ticks {
        if kind == "indirect" && t == 0 {
            let arrow = Entity::arrow(&mut w, 0.0, 0.0, 0.0);
            out.push(e.attack_from(
                &mut w,
                DamageSource::indirect(DamageKind::Arrow, arrow.entity_id, None),
                4,
            ) as u8);
        }
        if kind.starts_with("kill-") && t == 0 {
            e.attack_from(
                &mut w,
                DamageSource::entity(DamageKind::Player, killer),
                1000,
            );
        }
        if kind == "teleport" && t % 25 == 0 {
            out.push(e.enderman_teleport(
                &mut w,
                Vec3D::new(if t == 25 { 1000.0 } else { -4.5 }, 70.75, 5.5),
            ) as u8);
        }
        if kind == "damage" && t % 40 == 0 {
            e.attack_from(&mut w, DamageSource::new(DamageKind::Generic), 3);
        }
        if kind == "poison" {
            w.entities.insert(e.entity_id, e.clone());
        }
        e.on_update(&mut w);
        if kind == "poison" {
            w.entities.remove(&e.entity_id);
        }
        state(&mut w, &e, &mut out);
    }
    out
}
