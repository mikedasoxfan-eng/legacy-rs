//! Release 1.0 original-bytecode comparisons, with no tolerant float comparison.
use super::{
    damage::{DamageKind, DamageSource},
    living::Target,
    Entity, EntityKind, Vec3D,
};
use crate::World;
fn i(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_le_bytes());
}
fn l(o: &mut Vec<u8>, v: i64) {
    o.extend(v.to_le_bytes());
}
fn f(o: &mut Vec<u8>, v: f32) {
    i(o, if v.is_nan() { 0x7fc00000 } else { v.to_bits() } as i32);
}
fn d(o: &mut Vec<u8>, v: f64) {
    l(
        o,
        if v.is_nan() {
            0x7ff8000000000000
        } else {
            v.to_bits()
        } as i64,
    );
}
fn entity(e: &Entity, o: &mut Vec<u8>) {
    i(o, e.entity_id);
    for p in [e.pos, e.motion, e.prev_pos] {
        for v in [p.x, p.y, p.z] {
            d(o, v);
        }
    }
    let b = e.bounding_box;
    for v in [b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z] {
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
        e.no_clip,
        e.collided_horizontally,
        e.collided_vertically,
    ] {
        i(o, v as i32);
    }
    l(o, e.rand.state());
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
            a.last_damage,
            a.damage_remainder,
            a.recently_hit,
        ] {
            i(o, v);
        }
        f(o, a.render_yaw_offset);
        f(o, a.random_yaw_velocity);
    }
}
fn dragon(w: &mut World, e: &Entity, o: &mut Vec<u8>) {
    entity(e, o);
    let EntityKind::Dragon(s) = &e.kind else {
        panic!()
    };
    for v in [
        s.target_position.x,
        s.target_position.y,
        s.target_position.z,
    ] {
        d(o, v);
    }
    i(o, s.history_index);
    for h in s.history {
        for v in h {
            d(o, v);
        }
    }
    f(o, s.previous_animation);
    f(o, s.animation);
    i(o, s.force_new_target as i32);
    i(o, s.slowed as i32);
    i(
        o,
        s.target.and_then(|t| w.target_entity_id(t)).unwrap_or(-1),
    );
    i(o, s.healing_crystal.unwrap_or(-1));
    i(o, s.death_ticks);
    i(o, e.data_watcher.int(16).unwrap());
    for id in s.parts {
        entity(&w.entities[&id], o);
    }
    l(o, w.rand.state());
    l(o, w.math_random.state());
    let super::EntitySeedSource::Deterministic(seed) = w.entity_seed_source else {
        panic!()
    };
    l(o, seed);
    i(o, w.loaded_entity_list.len() as i32);
    for id in &w.loaded_entity_list {
        let a = &w.entities[id];
        entity(a, o);
        if let EntityKind::XPOrb(s) = &a.kind {
            i(o, s.value);
        }
    }
    crate::world::events::write_events(&w.drain_events(), o);
}
pub fn run(kind: &str) -> Vec<u8> {
    let mut o = vec![];
    if let Some(scene) = kind.strip_prefix("branches-") {
        return super::branches_verify181::replay(scene);
    }
    if let Some(scene) = kind.strip_prefix("round4-") {
        return super::round4_verify181::replay(scene);
    }
    if let Some(scene) = kind.strip_prefix("round5-") {
        return super::round5_verify181::replay(scene);
    }
    if let Some(scene) = kind.strip_prefix("player-") {
        return super::player::verify181::replay(scene, 240);
    }
    if kind == "playersp" {
        return super::player::verify181::sp_boundaries();
    }
    if kind == "explosion" {
        return super::combat_verify181::explosion_replay();
    }
    if let Some(name) = kind.strip_prefix("mob-") {
        return run_mob(name);
    }
    if kind == "mooshroom" {
        return run_mooshroom();
    }
    match kind {
        "fortressspawns" => return run_fortress_spawns(),
        "itempickup" => return run_item_pickup(),
        "sleep" => return run_sleep(),
        "fallingsand" => return run_falling_sand(),
        "inventorycopy" => return run_inventory_copy(),
        "food" => return run_food(),
        "settings" => return run_settings(),
        "crystal" => return run_crystal(),
        "smallfireball" => return run_fireball(true),
        "fireball" => return run_fireball(false),
        "potionregistry" => return run_potion_registry(),
        "damagesources" => return run_damage_sources(),
        "pathvalues" => return run_path_values(),
        "registry10" => {
            for id in -10..=255 {
                let entry = super::registry::by_id(id);
                text(&mut o, entry.map(|e| e.class));
                text(&mut o, entry.map(|e| e.name));
            }
            return o;
        }
        _ => {}
    }
    if kind == "inventory" {
        return super::player::verify::inventory_bytes();
    }
    if kind == "cartinventory" {
        return super::vehicle_verify::inventory_bytes();
    }
    if let Some(scene) = kind.strip_prefix("vehicle-") {
        return super::vehicle_verify::replay(scene, 240);
    }
    if let Some(kind) = kind.strip_prefix("api-") {
        return super::api_verify181::replay(kind);
    }
    if matches!(kind, "living6" | "livingdamage6" | "livingnbt6") {
        return super::round6_verify181::replay(kind);
    }
    if kind == "rules" {
        return super::verify181::rules();
    }
    if let Some(kind) = kind.strip_prefix("replay-") {
        return super::verify181::replay(
            kind,
            if matches!(kind, "Egg" | "Snowball") {
                24
            } else {
                240
            },
        );
    }
    if kind == "fishhook" {
        return run_fish();
    }
    if kind == "breeding" {
        return run_breeding();
    }
    if kind == "blaze" {
        return run_blaze();
    }
    if kind == "endereye" {
        return run_eye();
    }
    if kind == "playerxp" {
        return run_player_xp();
    }
    if let Some(name) = kind.strip_prefix("drops-") {
        return run_drops(name);
    }
    if matches!(kind, "egg" | "snowball" | "pearl" | "splashpotion") {
        return run_throwable(kind);
    }
    if kind == "dragonhistory" {
        let mut s = super::dragon::DragonState::default();
        for j in 0..64 {
            s.history[j] = [
                j as f64 * 173.25 - 7000.0,
                (j * j) as f64 * 0.125 - 10.0,
                j as f64 * 0.2,
            ];
        }
        for idx in [-1, 0, 1, 31, 63] {
            for hp in [0, 1, 200] {
                for n in [0, 1, 5, 10, 12, 16, 63, 64, 129] {
                    for p in [-0.5, 0.0, 0.25, 1.0, 1.5] {
                        s.history_index = idx;
                        for v in s.movement_offsets(n, p, hp) {
                            d(&mut o, v);
                        }
                    }
                }
            }
        }
        return o;
    }
    if kind == "dragondamage" {
        for source in 0..4 {
            for part in 0..7 {
                for damage in [-7, 0, 1, 3, 4, 19, 201] {
                    let mut w = super::verify181::world();
                    let mut e = Entity::living(&mut w, "EnderDragon").unwrap();
                    e.set_position(-3.5, 80.0, 4.5);
                    e.yaw = 173.25;
                    let mut a = if source == 2 {
                        Entity::player(&mut w, "Player")
                    } else {
                        Entity::living(&mut w, "Zombie").unwrap()
                    };
                    a.set_position(4.0, 78.0, -3.0);
                    let aid = a.entity_id;
                    w.entity_joined_world(a);
                    let s = match source {
                        0 => DamageSource::new(DamageKind::Generic),
                        1 => DamageSource::new(DamageKind::Explosion),
                        _ => DamageSource::entity(DamageKind::Mob, Some(aid)),
                    };
                    i(&mut o, e.attack_from(&mut w, s, damage) as i32);
                    i(
                        &mut o,
                        e.attack_dragon_part(&mut w, part == 0, s, damage) as i32,
                    );
                    dragon(&mut w, &e, &mut o);
                    let EntityKind::Dragon(state) = &e.kind else {
                        panic!()
                    };
                    let p = &w.entities[&state.parts[part]];
                    i(&mut o, p.entity_identity_matches(Some(p.entity_id)) as i32);
                    i(&mut o, p.entity_identity_matches(Some(e.entity_id)) as i32);
                }
            }
        }
        return o;
    }
    for scene in 0..if kind == "dragondeath" { 2 } else { 12 } {
        let mut w = super::verify181::world();
        let mut e = Entity::living(&mut w, "EnderDragon").unwrap();
        dragon(&mut w, &e, &mut o);
        e.set_position(-3.5, 80.0, 4.5);
        e.yaw = 179.0;
        e.pitch = 7.0;
        e.motion = Vec3D::new(0.35, -0.125, 0.6);
        if kind == "dragondeath" {
            e.living.health = 0;
            w.multiplayer_world = scene == 1;
            for _ in 0..201 {
                e.dragon_death_update(&mut w);
                dragon(&mut w, &e, &mut o);
            }
        } else {
            w.multiplayer_world = scene == 1 || scene == 2;
            if scene == 2 {
                e.set_position_and_rotation2(&mut w, Vec3D::new(8.0, 86.0, -4.0), -179.0, 31.0, 7);
            }
            if let EntityKind::Dragon(s) = &mut e.kind {
                if scene == 3 {
                    s.force_new_target = true;
                }
                if scene == 4 {
                    s.slowed = true;
                }
                if scene == 5 {
                    s.target_position = Vec3D::new(120.0, 78.0, 95.0);
                }
            }
            if scene == 6 || scene == 7 {
                let mut p = Entity::player(&mut w, "Player");
                p.set_position(4.0, 78.0, -3.0);
                if scene == 7 {
                    p.set_position(1000.0, 90.0, 1000.0);
                }
                let id = p.entity_id;
                w.entity_joined_world(p);
                if let EntityKind::Dragon(s) = &mut e.kind {
                    s.target = Some(Target::Entity(id));
                }
            }
            if scene == 8 || scene == 9 {
                let mut c = Entity::ender_crystal(&mut w, Vec3D::new(0.0, 80.0, 0.0));
                c.dead = scene == 9;
                let id = c.entity_id;
                w.entity_joined_world(c);
                e.living.health = 190;
                if let EntityKind::Dragon(s) = &mut e.kind {
                    s.healing_crystal = Some(id);
                }
            }
            if scene == 10 {
                for x in -10..=4 {
                    for y in 79..=82 {
                        for z in -4..=9 {
                            w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap().blocks
                                [crate::world::chunk::idx(x & 15, y, z & 15)] =
                                if (x + z) % 3 == 0 { 49 } else { 1 };
                        }
                    }
                }
            }
            if scene == 11 {
                e.living.health = 0;
            }
            w.rand.set_seed(300);
            for tick in 0..96 {
                e.ticks_existed = tick;
                e.dragon_living_update(&mut w);
                dragon(&mut w, &e, &mut o);
            }
        }
        for x in -10..=6 {
            for y in 63..=97 {
                for z in -5..=12 {
                    i(&mut o, w.get_block_id(x, y, z) as i32);
                    i(&mut o, w.get_block_metadata(x, y, z));
                }
            }
        }
    }
    o
}
fn fish(w: &mut World, e: &Entity, o: &mut Vec<u8>) {
    entity(e, o);
    let EntityKind::Fish(s) = &e.kind else {
        panic!()
    };
    let a = &s.arrow;
    for v in [
        a.tile[0],
        a.tile[1],
        a.tile[2],
        a.block as i32,
        a.shake,
        a.ticks_ground,
        a.ticks_air,
        s.catchable,
        s.interpolation_ticks,
    ] {
        i(o, v);
    }
    i(o, a.in_ground as i32);
    i(o, a.owner.unwrap_or(-1));
    i(o, s.hooked.unwrap_or(-1));
    for v in [
        s.target.x,
        s.target.y,
        s.target.z,
        s.target_yaw,
        s.target_pitch,
        s.velocity.x,
        s.velocity.y,
        s.velocity.z,
    ] {
        d(o, v);
    }
    i(
        o,
        a.owner
            .and_then(|id| w.entities.get(&id))
            .and_then(|e| e.player_state())
            .and_then(|p| p.fish_entity)
            .unwrap_or(-1),
    );
    l(o, w.rand.state());
    l(o, w.math_random.state());
    let super::EntitySeedSource::Deterministic(seed) = w.entity_seed_source else {
        panic!()
    };
    l(o, seed);
    i(o, w.loaded_entity_list.len() as i32);
    for id in &w.loaded_entity_list {
        entity(&w.entities[id], o);
    }
    crate::world::events::write_events(&w.drain_events(), o);
}
fn run_fish() -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..8 {
        for scene in 0..14 {
            let mut w = super::verify181::world();
            let mut p = Entity::player(&mut w, "Player");
            p.set_position(-3.5, 68.0, 4.5);
            p.yaw = (seed * 47) as f32;
            p.pitch = (seed * 9 - 35) as f32;
            p.player_state_mut().unwrap().inventory.main[0] =
                Some(super::player::ItemStack::new(346, 1, 0));
            let pid = p.entity_id;
            let mut e = Entity::fish_from_owner(&mut w, &mut p);
            w.entity_joined_world(p);
            e.rand.set_seed(seed as i64 + 1000);
            fish(&mut w, &e, &mut o);
            if scene == 1 {
                w.multiplayer_world = true;
                w.entities
                    .get_mut(&pid)
                    .unwrap()
                    .player_state_mut()
                    .unwrap()
                    .inventory
                    .main[0] = None;
            }
            if scene == 2 {
                w.entities
                    .get_mut(&pid)
                    .unwrap()
                    .player_state_mut()
                    .unwrap()
                    .inventory
                    .main[0] = None;
            }
            if scene == 3 {
                w.entities.get_mut(&pid).unwrap().living.health = 0;
            }
            if scene == 4 {
                w.entities.get_mut(&pid).unwrap().dead = true;
            }
            if scene == 5 {
                w.entities
                    .get_mut(&pid)
                    .unwrap()
                    .set_position(100.0, 68.0, 100.0);
            }
            if scene == 6 {
                e.set_velocity(0.2, -0.1, 0.3);
                e.set_position_and_rotation2(&mut w, Vec3D::new(4.0, 71.0, -2.0), -179.0, 22.0, 5);
            }
            if scene == 7 || scene == 8 {
                let mut c = Entity::living(&mut w, "Cow").unwrap();
                c.set_position(-2.0, 66.0, 4.0);
                c.dead = scene == 8;
                let id = c.entity_id;
                w.entity_joined_world(c);
                if let EntityKind::Fish(s) = &mut e.kind {
                    s.hooked = Some(id);
                }
            }
            if scene == 9 || scene == 10 {
                if let EntityKind::Fish(s) = &mut e.kind {
                    s.arrow.in_ground = true;
                    s.arrow.tile = [-3, 63, 4];
                    s.arrow.block = if scene == 9 { 1 } else { 49 };
                    s.arrow.ticks_ground = 1197;
                }
            }
            if scene == 11 || scene == 12 {
                e.set_position(-3.5, 65.0, 4.5);
                e.motion = Vec3D::default();
                for x in -8..=2 {
                    for y in 64..=68 {
                        for z in 0..=9 {
                            w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap().blocks
                                [crate::world::chunk::idx(x & 15, y, z & 15)] = 9;
                        }
                    }
                }
                if scene == 12 {
                    if let EntityKind::Fish(s) = &mut e.kind {
                        s.catchable = 5;
                    }
                }
            }
            if scene == 13 {
                let mut c = Entity::living(&mut w, "Cow").unwrap();
                c.set_position(e.pos.x, e.pos.y, e.pos.z + 1.0);
                w.entity_joined_world(c);
                e.set_velocity(0.0, 0.0, 0.7);
            }
            for _ in 0..if scene == 11 { 1000 } else { 12 } {
                e.on_update(&mut w);
                fish(&mut w, &e, &mut o);
                if e.dead {
                    break;
                }
            }
            if !e.dead {
                let mut p = w.entities.remove(&pid).unwrap();
                let result = e.catch_fish(&mut w, &mut p);
                w.entities.insert(pid, p);
                i(&mut o, result);
                fish(&mut w, &e, &mut o);
            }
            let n = e.write_fish_nbt();
            let mut copy = Entity::fish_base(&mut w);
            copy.read_fish_nbt(&n);
            let EntityKind::Fish(s) = &copy.kind else {
                panic!()
            };
            for v in [
                s.arrow.tile[0],
                s.arrow.tile[1],
                s.arrow.tile[2],
                s.arrow.block as i32,
                s.arrow.shake,
            ] {
                i(&mut o, v);
            }
            i(&mut o, s.arrow.in_ground as i32);
        }
    }
    o
}
fn throwable(w: &mut World, e: &Entity, o: &mut Vec<u8>) {
    entity(e, o);
    let s = match &e.kind {
        EntityKind::Egg(s)
        | EntityKind::Snowball(s)
        | EntityKind::EnderPearl(s)
        | EntityKind::Potion(s, _) => s,
        _ => panic!(),
    };
    for v in [
        s.tile[0],
        s.tile[1],
        s.tile[2],
        s.block as i32,
        s.shake,
        s.ticks_ground,
        s.ticks_air,
    ] {
        i(o, v);
    }
    i(o, s.in_ground as i32);
    i(o, s.owner.unwrap_or(-1));
    l(o, w.rand.state());
    l(o, w.math_random.state());
    let super::EntitySeedSource::Deterministic(seed) = w.entity_seed_source else {
        panic!()
    };
    l(o, seed);
    i(o, w.loaded_entity_list.len() as i32);
    for id in &w.loaded_entity_list {
        let a = &w.entities[id];
        entity(a, o);
        if a.kind.is_living() {
            i(
                o,
                if a.kind.is_animal() {
                    a.living.growing_age
                } else {
                    0
                },
            );
            let mut effects = a.living.effects.clone();
            effects.sort_by_key(|p| p.id);
            i(o, effects.len() as i32);
            for p in effects {
                for v in [p.id, p.duration, p.amplifier] {
                    i(o, v);
                }
            }
        }
    }
    crate::world::events::write_events(&w.drain_events(), o);
}
fn run_throwable(kind: &str) -> Vec<u8> {
    let mut o = vec![];
    let typ = match kind {
        "egg" => 0,
        "snowball" => 1,
        "pearl" => 2,
        _ => 3,
    };
    for seed in 0..128 {
        for scene in 0..9 {
            let mut w = super::verify181::world();
            let mut owner = if scene == 8 {
                Entity::living(&mut w, "Cow").unwrap()
            } else {
                Entity::player(&mut w, "Player")
            };
            owner.set_position(-3.5, 68.0, 4.5);
            owner.yaw = (seed * 17) as f32;
            owner.pitch = (seed % 85 - 40) as f32;
            let metadata =
                [16389, 16396, 16421, 16428, 16385, 16386, 16388, 16392][seed as usize % 8];
            let mut e = if scene == 0 || scene == 1 {
                match typ {
                    0 | 1 => Entity::throwable_at(&mut w, -3.5, 68.0, 4.5, typ == 0),
                    2 => Entity::pearl_at(&mut w, Vec3D::new(-3.5, 68.0, 4.5)),
                    _ => Entity::potion_at(&mut w, Vec3D::new(-3.5, 68.0, 4.5), metadata),
                }
            } else {
                match typ {
                    0 | 1 => Entity::throwable(&mut w, &owner, typ == 0),
                    2 => Entity::pearl(&mut w, &owner),
                    _ => Entity::potion_projectile(&mut w, &owner, metadata),
                }
            };
            w.entity_joined_world(owner);
            throwable(&mut w, &e, &mut o);
            e.rand.set_seed(seed as i64 + 1000);
            if scene == 1 {
                e.set_velocity(0.25, -0.5, 0.75);
            }
            if scene == 2 || scene == 3 {
                let mut s = match &e.kind {
                    EntityKind::Egg(s)
                    | EntityKind::Snowball(s)
                    | EntityKind::EnderPearl(s)
                    | EntityKind::Potion(s, _) => s.clone(),
                    _ => panic!(),
                };
                s.in_ground = true;
                s.tile = [-3, 63, 4];
                s.block = if scene == 2 { 1 } else { 49 };
                s.ticks_ground = 1197;
                e.store_throwable_state(s);
            }
            if scene == 4 {
                e.set_position(-3.5, 65.0, 4.5);
                e.set_velocity(0.1, -0.05, 0.1);
                for x in -8..=2 {
                    for y in 64..=68 {
                        for z in 0..=9 {
                            w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap().blocks
                                [crate::world::chunk::idx(x & 15, y, z & 15)] = 9;
                        }
                    }
                }
            }
            if (5..=7).contains(&scene) {
                e.set_position(-3.5, 65.0, 4.5);
                e.set_velocity(0.0, -0.8, 0.0);
                if scene == 6 {
                    let mut b = Entity::living(&mut w, "Blaze").unwrap();
                    b.set_position(-3.5, 64.0, 4.5);
                    w.entity_joined_world(b);
                }
                if scene == 7 {
                    w.multiplayer_world = true;
                }
            }
            for _ in 0..8 {
                e.on_update(&mut w);
                throwable(&mut w, &e, &mut o);
                if e.dead {
                    break;
                }
            }
            let n = e.write_throwable_nbt();
            e.read_throwable_nbt(&n);
            for name in ["xTile", "yTile", "zTile"] {
                i(&mut o, n.short(name) as i32);
            }
            for name in ["inTile", "shake", "inGround"] {
                i(&mut o, n.byte(name) as i32);
            }
            if typ == 3 {
                i(&mut o, n.int("potionValue"));
            }
        }
    }
    o
}
fn animal(w: &mut World, e: &Entity, o: &mut Vec<u8>) {
    entity(e, o);
    for v in [
        e.living.growing_age,
        e.living.in_love,
        e.living.breeding_ticks,
        e.living
            .target
            .and_then(|t| w.target_entity_id(t))
            .unwrap_or(-1),
        e.living.has_attacked as i32,
    ] {
        i(o, v);
    }
    i(
        o,
        if let EntityKind::Sheep { fleece_color, .. } = e.kind {
            fleece_color as i32
        } else {
            0
        },
    );
    l(o, w.rand.state());
    l(o, w.math_random.state());
    let super::EntitySeedSource::Deterministic(seed) = w.entity_seed_source else {
        panic!()
    };
    l(o, seed);
    i(o, w.loaded_entity_list.len() as i32);
    for id in &w.loaded_entity_list {
        let a = &w.entities[id];
        entity(a, o);
        if a.kind.is_animal() {
            i(o, a.living.growing_age);
            i(
                o,
                if let EntityKind::Sheep { fleece_color, .. } = a.kind {
                    fleece_color as i32
                } else {
                    0
                },
            );
        }
    }
    crate::world::events::write_events(&w.drain_events(), o);
}
fn run_breeding() -> Vec<u8> {
    let mut o = vec![];
    for name in ["Cow", "Pig", "Sheep", "Chicken", "MushroomCow"] {
        for scene in 0..8 {
            let mut w = super::verify181::world();
            let mut e = Entity::living(&mut w, name).unwrap();
            e.set_position(-3.5, 64.0, 4.5);
            let mut p = Entity::player(&mut w, "Player");
            p.set_position(-2.0, 65.0, 4.0);
            p.player_state_mut().unwrap().inventory.main[0] = Some(super::player::ItemStack::new(
                if scene == 1 { 1 } else { 296 },
                if scene == 2 { 1 } else { 3 },
                0,
            ));
            p.player_state_mut().unwrap().capabilities.creative = scene == 3;
            let pid = p.entity_id;
            e.set_growing_age(if scene == 4 {
                -24000
            } else if scene == 5 {
                6000
            } else {
                0
            });
            let result = e.feed_animal(&mut w, &mut p);
            i(&mut o, result as i32);
            i(&mut o, p.held_item().map_or(-1, |s| s.count));
            w.entity_joined_world(p);
            animal(&mut w, &e, &mut o);
            let mut mate = Entity::living(&mut w, name).unwrap();
            mate.set_position(-2.5, 64.0, 4.5);
            mate.living.in_love = 600;
            if let EntityKind::Sheep { fleece_color, .. } = &mut mate.kind {
                *fleece_color = 14;
            }
            if let EntityKind::Sheep { fleece_color, .. } = &mut e.kind {
                *fleece_color = 3;
            }
            if scene == 6 {
                mate.living.target = Some(Target::Entity(pid));
            }
            if scene == 7 {
                mate.set_growing_age(-20);
                e.set_growing_age(100);
                e.living.in_love = 0;
            }
            let mid = mate.entity_id;
            w.entity_joined_world(mate);
            for tick in 0..65 {
                e.animal_attack_target(
                    &mut w,
                    Target::Entity(mid),
                    if scene == 0 && tick < 3 { 4.0 } else { 2.0 },
                );
                animal(&mut w, &e, &mut o);
            }
            let n = e.to_nbt().unwrap();
            i(&mut o, n.int("Age"));
            i(&mut o, n.int("InLove"));
        }
    }
    for mushroom in [false, true] {
        let mut w = super::verify181::world();
        let mut e = Entity::living(&mut w, if mushroom { "MushroomCow" } else { "Cow" }).unwrap();
        e.set_position(0.0, 64.0, 0.0);
        e.living.in_love = 600;
        let mut mate =
            Entity::living(&mut w, if mushroom { "Cow" } else { "MushroomCow" }).unwrap();
        mate.set_position(2.0, 64.0, 2.0);
        mate.living.in_love = 600;
        w.entity_joined_world(mate);
        i(
            &mut o,
            e.animal_find_target(&w)
                .and_then(|t| w.target_entity_id(t))
                .unwrap_or(-1),
        );
    }
    o
}
fn world_state(w: &mut World, o: &mut Vec<u8>) {
    l(o, w.rand.state());
    l(o, w.math_random.state());
    let super::EntitySeedSource::Deterministic(seed) = w.entity_seed_source else {
        panic!()
    };
    l(o, seed);
    i(o, w.loaded_entity_list.len() as i32);
    for id in &w.loaded_entity_list {
        let a = &w.entities[id];
        entity(a, o);
        if let EntityKind::Item { item, .. } = &a.kind {
            for v in [item.id, item.count, item.damage as i32] {
                i(o, v);
            }
        }
        if let EntityKind::Fireball(s) | EntityKind::SmallFireball(s) = &a.kind {
            for v in [s.acceleration.x, s.acceleration.y, s.acceleration.z] {
                d(o, v);
            }
        }
    }
    crate::world::events::write_events(&w.drain_events(), o);
}
fn run_drops(name: &str) -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..16 {
        for looting in 0..5 {
            for flags in 0..4 {
                let mut w = super::verify181::world();
                let mut e = Entity::living(&mut w, name).unwrap();
                e.set_position(-3.5, 64.0, 4.5);
                e.rand.set_seed(seed + 1000);
                if flags & 2 != 0 {
                    e.set_fire_seconds(10);
                }
                entity(&e, &mut o);
                i(&mut o, e.max_health());
                i(&mut o, e.creature_attribute() as i32);
                for damage in [7, 3] {
                    let damage = e.armor_damage(DamageSource::new(DamageKind::Generic), damage);
                    e.living.health -= e.resistance_damage(damage);
                    entity(&e, &mut o);
                }
                e.drop_few_items_with_looting(&mut w, flags & 1 != 0, looting);
                entity(&e, &mut o);
                world_state(&mut w, &mut o);
            }
        }
    }
    o
}
fn blaze(w: &mut World, e: &Entity, o: &mut Vec<u8>) {
    entity(e, o);
    let EntityKind::Blaze(s) = &e.kind else {
        panic!()
    };
    f(o, s.hover_offset);
    i(o, s.height_timer);
    i(o, s.attack_step);
    i(o, e.data_watcher.byte(16).unwrap() as i32);
    i(o, e.living.has_attacked as i32);
    i(o, e.is_burning(w) as i32);
    f(o, e.brightness(w));
    i(o, e.packed_brightness(w, 0.5));
    world_state(w, o);
}
fn run_blaze() -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..8 {
        for scene in 0..5 {
            let mut w = super::verify181::world();
            let mut e = Entity::living(&mut w, "Blaze").unwrap();
            e.set_position(-3.5, 68.0, 4.5);
            e.rand.set_seed(1000 + seed);
            let mut t = Entity::living(&mut w, "Cow").unwrap();
            t.set_position(
                if scene == 1 { -3.0 } else { 4.0 },
                if scene == 2 { 79.0 } else { 68.0 },
                4.5,
            );
            let tid = t.entity_id;
            w.entity_joined_world(t);
            e.living.target = Some(Target::Entity(tid));
            if scene == 3 {
                w.multiplayer_world = true;
            }
            if scene == 4 {
                for x in -5..=-2 {
                    for y in 67..=71 {
                        for z in 3..=6 {
                            w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap().blocks
                                [crate::world::chunk::idx(x & 15, y, z & 15)] = 9;
                        }
                    }
                }
            }
            for _ in 0..180 {
                if e.living.attack_time > 0 {
                    e.living.attack_time -= 1;
                }
                e.new_mob_attack(
                    &mut w,
                    Target::Entity(tid),
                    if scene == 1 {
                        1.0
                    } else if scene == 2 {
                        31.0
                    } else {
                        8.0
                    },
                );
                blaze(&mut w, &e, &mut o);
            }
            e.living.target = None;
            for _ in 0..110 {
                e.on_update(&mut w);
                blaze(&mut w, &e, &mut o);
            }
        }
    }
    o
}
fn run_eye() -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..32 {
        for scene in 0..5 {
            let mut w = super::verify181::world();
            let mut e = Entity::ender_eye(&mut w, Vec3D::new(-3.5, 68.0, 4.5));
            e.rand.set_seed(1000 + seed);
            w.multiplayer_world = scene == 1;
            e.set_eye_target(
                if scene == 2 {
                    1000.0
                } else if scene == 3 {
                    -3.0
                } else {
                    2.0
                },
                if scene == 4 { 60 } else { 77 },
                if scene == 2 {
                    1000.0
                } else if scene == 3 {
                    4.0
                } else {
                    8.0
                },
            );
            if scene == 1 {
                e.set_velocity(0.2, -0.1, 0.3);
            }
            for _ in 0..82 {
                e.on_update(&mut w);
                entity(&e, &mut o);
                let EntityKind::EnderEye(s) = &e.kind else {
                    panic!()
                };
                for v in [s.target.x, s.target.y, s.target.z] {
                    d(&mut o, v);
                }
                i(&mut o, s.lifetime);
                i(&mut o, s.drops as i32);
                world_state(&mut w, &mut o);
                if e.dead {
                    break;
                }
            }
        }
    }
    o
}
fn run_player_xp() -> Vec<u8> {
    let mut o = vec![];
    let mut w = super::verify181::world();
    let mut p = Entity::player(&mut w, "Player");
    for level in [0, 1, 2, 10, 100, 1000] {
        for amount in [-7, 0, 1, 3, 7, 73, 10000] {
            let s = p.player_state_mut().unwrap();
            s.experience = super::player::Experience {
                points: 0.375,
                level,
                total: 17,
            };
            s.score = 29;
            i(&mut o, s.experience.bar_cap());
            p.add_experience(amount);
            let s = p.player_state_mut().unwrap();
            f(&mut o, s.experience.points);
            for v in [
                s.experience.level,
                s.experience.total,
                s.score,
                s.experience.bar_cap(),
            ] {
                i(&mut o, v);
            }
            s.experience.spend_levels(level + 3);
            i(&mut o, s.experience.level);
            f(&mut o, s.experience.points);
        }
    }
    for flags in 0..16 {
        let c = super::player::PlayerCapabilities {
            disable_damage: flags & 1 != 0,
            flying: flags & 2 != 0,
            allow_flying: flags & 4 != 0,
            creative: flags & 8 != 0,
        };
        let mut n = crate::nbt::Compound::new();
        c.write_nbt(&mut n);
        let crate::nbt::Tag::Compound(a) = n.get("abilities").unwrap() else {
            panic!()
        };
        for key in ["invulnerable", "flying", "mayfly", "instabuild"] {
            i(&mut o, a.boolean(key) as i32);
        }
        let mut loaded = super::player::PlayerCapabilities::default();
        loaded.read_nbt(&n);
        for flag in [
            loaded.disable_damage,
            loaded.flying,
            loaded.allow_flying,
            loaded.creative,
        ] {
            i(&mut o, flag as i32);
        }
    }
    o
}
fn text(o: &mut Vec<u8>, s: Option<&str>) {
    if let Some(s) = s {
        i(o, s.len() as i32);
        o.extend(s.as_bytes());
    } else {
        i(o, -1);
    }
}
fn mob_state(w: &mut World, e: &Entity, o: &mut Vec<u8>) {
    entity(e, o);
    i(o, e.max_health());
    i(o, e.creature_attribute() as i32);
    f(o, e.living.move_speed);
    f(o, e.living.ground_acceleration);
    i(o, e.living.entity_age);
    if let EntityKind::Slime {
        size,
        squish,
        prev_squish,
        ..
    }
    | EntityKind::MagmaCube {
        size,
        squish,
        prev_squish,
        ..
    } = e.kind
    {
        i(o, size);
        f(o, squish);
        f(o, e.living.slime_squish);
        f(o, prev_squish);
    }
    if let EntityKind::Villager { profession } = e.kind {
        text(o, e.villager_texture());
        i(o, profession);
    }
    i(o, e.is_burning(w) as i32);
    f(o, e.brightness(w));
    i(o, e.packed_brightness(w, 1.0));
    world_state(w, o);
}
fn run_mob(name: &str) -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..8 {
        for scene in 0..5 {
            let mut w = super::verify181::world();
            let mut e = Entity::living(&mut w, name).unwrap();
            e.set_position(-3.5, 64.0, 4.5);
            e.rand.set_seed(seed + 1000);
            w.multiplayer_world = scene == 1;
            w.difficulty = if scene == 2 { 0 } else { 2 };
            if matches!(
                e.kind,
                EntityKind::Slime { .. } | EntityKind::MagmaCube { .. }
            ) {
                e.set_slime_size([1, 2, 4, 1, 4][scene]);
            }
            if matches!(e.kind, EntityKind::Villager { .. }) {
                let mut n = e.to_nbt().unwrap();
                n.put(
                    "Profession",
                    crate::nbt::Tag::Int(if scene == 4 { 99 } else { scene as i32 }),
                );
                e.read_existing_nbt(&mut w, &n).unwrap();
            }
            if scene == 3 {
                for x in -5..=-2 {
                    for z in 3..=6 {
                        w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap().blocks
                            [crate::world::chunk::idx(x & 15, 64, z & 15)] = 9;
                    }
                }
            }
            let mut target = Entity::living(&mut w, "Zombie").unwrap();
            target.set_position(2.0, 64.0, 8.0);
            let tid = target.entity_id;
            w.entity_joined_world(target);
            mob_state(&mut w, &e, &mut o);
            for t in 0..160 {
                if matches!(e.kind, EntityKind::Snowman) && scene == 4 {
                    e.new_mob_attack(
                        &mut w,
                        Target::Entity(tid),
                        if t % 3 == 0 { 11.0 } else { 7.0 },
                    );
                }
                if t == 100 && scene == 4 {
                    e.attack_from(&mut w, DamageSource::new(DamageKind::Generic), 100);
                }
                e.on_update(&mut w);
                mob_state(&mut w, &e, &mut o);
                if e.dead {
                    break;
                }
            }
        }
    }
    o
}
fn run_mooshroom() -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..8 {
        for age in [-24000, -1, 0, 6000] {
            for item in [281, 359, 296, 0] {
                for creative in [false, true] {
                    let mut w = super::verify181::world();
                    let mut e = Entity::living(&mut w, "MushroomCow").unwrap();
                    e.set_location_and_angles(-3.5, 64.0, 4.5, 37.0, -12.0);
                    e.living.health = 7;
                    e.living.render_yaw_offset = 73.0;
                    e.set_growing_age(age);
                    e.rand.set_seed(seed + 1000);
                    let mut p = Entity::player(&mut w, "Player");
                    p.player_state_mut().unwrap().capabilities.creative = creative;
                    if item != 0 {
                        p.player_state_mut().unwrap().inventory.main[0] =
                            Some(super::player::ItemStack::new(item, 5, 0));
                    }
                    let pid = p.entity_id;
                    w.entity_joined_world(p);
                    let mut p = w.entities.remove(&pid).unwrap();
                    let eid = e.entity_id;
                    w.entities.insert(eid, e);
                    i(&mut o, p.use_on_entity(&mut w, eid) as i32);
                    e = w.entities.remove(&eid).unwrap();
                    entity(&e, &mut o);
                    let held = p.held_item();
                    for v in [
                        held.map_or(-1, |s| s.id),
                        held.map_or(0, |s| s.count),
                        held.map_or(0, |s| s.damage as i32),
                    ] {
                        i(&mut o, v);
                    }
                    w.entities.insert(pid, p);
                    world_state(&mut w, &mut o);
                }
            }
        }
    }
    o
}
fn run_fortress_spawns() -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..8 {
        let mut w = super::verify181::world();
        w.dimension = crate::world::Dimension::Nether;
        let start =
            crate::structures::nether10::NetherStart::new(&mut crate::JavaRandom::new(seed), -2, 1);
        let bounds: Vec<_> = start.pieces.iter().map(|p| p.component.bounds).collect();
        w.nether_bridges10.starts.insert(0, start);
        i(&mut o, bounds.len() as i32);
        for b in bounds {
            for v in [b.min_x, b.min_y, b.min_z, b.max_x, b.max_y, b.max_z] {
                i(&mut o, v);
            }
            for x in [b.min_x - 1, b.min_x, b.max_x, b.max_x + 1] {
                for y in [b.min_y - 1, b.min_y, b.max_y, b.max_y + 1] {
                    let z = (b.min_z + b.max_z) / 2;
                    i(&mut o, w.nether_bridges10.contains(x, y, z) as i32);
                    for category in 0..3 {
                        i(
                            &mut o,
                            w.provider_spawn_list10(category, x, y, z).len() as i32,
                        );
                        for _ in 0..3 {
                            let entry = w.choose_spawn_entry10(category, x, y, z);
                            text(
                                &mut o,
                                entry.map(|s| match s.entity {
                                    "LavaSlime" => "MagmaCube",
                                    "MushroomCow" => "Mooshroom",
                                    n => n,
                                }),
                            );
                            if let Some(s) = entry {
                                for v in [s.weight, s.min, s.max] {
                                    i(&mut o, v);
                                }
                            }
                            l(&mut o, w.rand.state());
                        }
                    }
                }
            }
        }
    }
    o
}
fn run_item_pickup() -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..8 {
        for item_id in [17, 334, 264, 369, 370, 371, 1] {
            for scene in 0..6 {
                let mut w = super::verify181::world();
                w.multiplayer_world = scene == 5;
                let mut p = Entity::player(&mut w, "Player");
                p.player_state_mut().unwrap().stat_log.clear();
                p.player_state_mut().unwrap().capabilities.creative = scene == 3;
                if scene == 1 || scene == 2 || scene == 3 {
                    p.player_state_mut()
                        .unwrap()
                        .inventory
                        .main
                        .fill(Some(super::player::ItemStack::new(1, 64, 0)));
                }
                if scene == 2 {
                    p.player_state_mut().unwrap().inventory.main[0] =
                        Some(super::player::ItemStack::new(item_id, 63, 0));
                }
                let mut e = Entity::item(
                    &mut w,
                    -3.5,
                    64.0,
                    4.5,
                    super::player::ItemStack::new(item_id, 5, 0).into(),
                );
                e.rand.set_seed(seed + 1000);
                if let EntityKind::Item { pickup_delay, .. } = &mut e.kind {
                    *pickup_delay = if scene == 4 { 1 } else { 0 };
                }
                p.player_pickup(&mut w, &mut e);
                entity(&e, &mut o);
                if let EntityKind::Item { item, .. } = e.kind {
                    i(&mut o, item.count as i32);
                }
                let s = p.player_state().unwrap();
                for stack in s.inventory.main {
                    i(&mut o, stack.map_or(-1, |v| v.id));
                    i(&mut o, stack.map_or(0, |v| v.count));
                }
                i(&mut o, s.stat_log.len() as i32);
                for &(id, count) in &s.stat_log {
                    i(&mut o, id);
                    i(&mut o, count);
                }
                let pickups: Vec<_> = s
                    .events
                    .iter()
                    .filter_map(|event| {
                        if let super::player::PlayerEvent::Pickup { entity, count, .. } = event {
                            Some((*entity, *count))
                        } else {
                            None
                        }
                    })
                    .collect();
                i(&mut o, pickups.len() as i32);
                for (id, count) in pickups {
                    i(&mut o, id);
                    i(&mut o, count);
                }
                world_state(&mut w, &mut o);
            }
        }
    }
    o
}
fn run_sleep() -> Vec<u8> {
    use super::player::SleepStatus;
    let mut o = vec![];
    for direction in 0..4 {
        for scene in 0..7 {
            for client in [false, true] {
                let mut w = super::verify181::world();
                w.multiplayer_world = client;
                let mut p = Entity::player(&mut w, "Player");
                p.set_location_and_angles(0.5, 64.0, 0.5, 0.0, 0.0);
                w.chunks.get_mut(&(0, 0)).unwrap().blocks[crate::world::chunk::idx(0, 64, 0)] = 26;
                w.chunks
                    .get_mut(&(0, 0))
                    .unwrap()
                    .data
                    .set(0, 64, 0, direction);
                if scene == 1 {
                    p.living.health = 0;
                }
                if scene == 2 {
                    p.set_position(10.0, 64.0, 0.5);
                }
                if scene == 3 {
                    w.skylight_subtracted = 0;
                }
                if scene == 4 {
                    w.dimension = crate::world::Dimension::Nether;
                }
                if scene == 5 || scene == 6 {
                    let mut mob =
                        Entity::living(&mut w, if scene == 5 { "Zombie" } else { "Cow" }).unwrap();
                    mob.set_position(2.0, 64.0, 2.0);
                    w.entity_joined_world(mob);
                }
                let status = p.sleep_in_bed(&mut w, 0, 64, 0);
                text(
                    &mut o,
                    Some(match status {
                        SleepStatus::Ok => "OK",
                        SleepStatus::OtherProblem => "OTHER_PROBLEM",
                        SleepStatus::NotPossibleHere => "NOT_POSSIBLE_HERE",
                        SleepStatus::NotPossibleNow => "NOT_POSSIBLE_NOW",
                        SleepStatus::TooFarAway => "TOO_FAR_AWAY",
                        SleepStatus::NotSafe => "NOT_SAFE",
                    }),
                );
                entity(&p, &mut o);
                i(&mut o, p.player_state().unwrap().sleeping as i32);
                world_state(&mut w, &mut o);
            }
        }
    }
    o
}
fn run_falling_sand() -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..8 {
        for scene in 0..7 {
            let mut w = super::verify181::world();
            w.multiplayer_world = scene == 2;
            let y = if scene == 3 || scene == 4 {
                64.5
            } else {
                70.25
            };
            let mut e = Entity::falling_sand(&mut w, -3.5, y, 4.5, if scene == 6 { 0 } else { 12 });
            e.rand.set_seed(seed + 1000);
            w.chunks.get_mut(&(-1, 0)).unwrap().blocks[crate::world::chunk::idx(12, y as i32, 4)] =
                if scene == 1 || scene == 2 {
                    1
                } else if scene == 4 {
                    36
                } else if scene == 5 || scene == 6 {
                    0
                } else {
                    12
                };
            if let EntityKind::FallingSand { fall_time, .. } = &mut e.kind {
                if scene == 4 {
                    *fall_time = 2;
                }
                if scene == 5 {
                    *fall_time = 100;
                }
            }
            for tick in 0..32 {
                if tick != 0 {
                    e.on_update(&mut w);
                }
                entity(&e, &mut o);
                let EntityKind::FallingSand { block, fall_time } = e.kind else {
                    panic!()
                };
                i(&mut o, block as i32);
                i(&mut o, fall_time);
                world_state(&mut w, &mut o);
                for yy in 63..=71 {
                    i(&mut o, w.get_block_id(-4, yy, 4) as i32);
                }
                if e.dead {
                    break;
                }
            }
        }
    }
    o
}
fn run_inventory_copy() -> Vec<u8> {
    use super::player::{InventoryPlayer, ItemStack};
    use crate::nbt::{Compound, Tag};
    let mut o = vec![];
    for c in 0..64 {
        let mut a = InventoryPlayer::default();
        let mut b = InventoryPlayer::default();
        a.current_item = c % 9;
        b.current_item = 8;
        for slot in 0..40 {
            if (slot + c) % 3 != 0 {
                let mut s = ItemStack::new(
                    if slot < 36 { 267 } else { 310 },
                    c + slot + 1,
                    c * 7 + slot,
                );
                s.animations = 19;
                let mut n = Compound::new();
                let mut nested = Compound::new();
                nested.put("value", Tag::Int(c * 100 + slot));
                n.put("nested", Tag::Compound(nested));
                let mut entry = Compound::new();
                entry.put("id", Tag::Short(16));
                entry.put("lvl", Tag::Short((c % 5 + 1) as i16));
                n.put("ench", Tag::List(10, vec![Tag::Compound(entry)]));
                s.set_tag(Some(n));
                if slot < 36 {
                    a.main[slot as usize] = Some(s);
                } else {
                    a.armor[(slot - 36) as usize] = Some(s);
                }
            }
        }
        b.copy_from_inventory(&a);
        for _step in 0..2 {
            i(&mut o, b.current_item);
            for slot in 0..40 {
                let source = if slot < 36 {
                    &mut a.main[slot]
                } else {
                    &mut a.armor[slot - 36]
                };
                let copy = if slot < 36 {
                    b.main[slot]
                } else {
                    b.armor[slot - 36]
                };
                i(&mut o, copy.is_some() as i32);
                if let Some(copy) = copy {
                    let source = source.as_mut().unwrap();
                    let tag = copy.tag().unwrap();
                    for v in [copy.id, copy.count, copy.damage, copy.animations] {
                        i(&mut o, v);
                    }
                    i(&mut o, copy.same_object(*source) as i32);
                    i(
                        &mut o,
                        std::sync::Arc::ptr_eq(
                            &copy.tag_handle().unwrap(),
                            &source.tag_handle().unwrap(),
                        ) as i32,
                    );
                    let Tag::Compound(nested) = tag.get("nested").unwrap() else {
                        panic!()
                    };
                    i(&mut o, nested.int("value"));
                    let Tag::Compound(entry) = &tag.list("ench")[0] else {
                        panic!()
                    };
                    i(&mut o, entry.short("lvl") as i32);
                    let handle = source.tag_handle().unwrap();
                    let mut original = handle.lock().unwrap();
                    let mut nested = Compound::new();
                    nested.put("value", Tag::Int(-c - slot as i32));
                    original.put("nested", Tag::Compound(nested));
                    let mut entry = Compound::new();
                    entry.put("id", Tag::Short(16));
                    entry.put("lvl", Tag::Short(99));
                    original.put("ench", Tag::List(10, vec![Tag::Compound(entry)]));
                    source.count = -1;
                }
            }
        }
    }
    o
}
fn run_food() -> Vec<u8> {
    use super::player::{food::FoodAction, FoodStats};
    let mut o = vec![];
    for difficulty in 0..4 {
        for initial_health in [0, 1, 10, 11, 19, 20] {
            for level in [0, 1, 17, 18, 20] {
                for saturation in [0.0, 0.5, 5.0] {
                    let mut food = FoodStats {
                        level,
                        saturation,
                        exhaustion: 40.0,
                        ..Default::default()
                    };
                    let mut health = initial_health;
                    for _tick in 0..200 {
                        let action = food.tick(difficulty, health);
                        match action {
                            FoodAction::Heal => health = (health + 1).min(20),
                            FoodAction::Starve => health -= 1,
                            _ => {}
                        }
                        for v in [food.level, food.prev_level, food.tick_timer] {
                            i(&mut o, v);
                        }
                        f(&mut o, food.saturation);
                        f(&mut o, food.exhaustion);
                        i(&mut o, health);
                        i(
                            &mut o,
                            match action {
                                FoodAction::None => 0,
                                FoodAction::Heal => 1,
                                FoodAction::Starve => 2,
                            },
                        );
                        i(&mut o, food.needs_food() as i32);
                    }
                }
            }
        }
    }
    o
}
fn run_settings() -> Vec<u8> {
    use super::player::{food::WorldSettings, PlayerCapabilities};
    let mut o = vec![];
    for seed in [0, 42, -1, i64::MIN, i64::MAX] {
        for mode in [0, 1, 2] {
            for features in [false, true] {
                for hardcore in [false, true] {
                    let s = WorldSettings::with_hardcore(seed, mode, features, hardcore);
                    l(&mut o, s.seed);
                    i(&mut o, s.game_mode);
                    i(&mut o, s.map_features as i32);
                    i(&mut o, s.hardcore as i32);
                }
            }
        }
    }
    let mut w = super::verify181::world();
    let mut p = Entity::player(&mut w, "Player");
    for flags in 0..16 {
        p.player_state_mut().unwrap().capabilities = PlayerCapabilities {
            disable_damage: flags & 1 != 0,
            flying: flags & 2 != 0,
            allow_flying: flags & 4 != 0,
            creative: flags & 8 != 0,
        };
        for creative in [true, false] {
            p.set_creative(creative);
            let c = p.player_state().unwrap().capabilities;
            for b in [c.disable_damage, c.flying, c.allow_flying, c.creative] {
                i(&mut o, b as i32);
            }
        }
    }
    o
}
fn blocks(w: &mut World, o: &mut Vec<u8>) {
    for x in -12..=8 {
        for y in 56..=80 {
            for z in -4..=16 {
                i(o, w.get_block_id(x, y, z) as i32);
                i(o, w.get_block_metadata(x, y, z));
            }
        }
    }
}
fn run_crystal() -> Vec<u8> {
    let mut o = vec![];
    for _seed in 0..4 {
        for client in [false, true] {
            let mut w = super::verify181::world();
            w.multiplayer_world = client;
            let mut e = Entity::ender_crystal(&mut w, Vec3D::new(-3.5, 64.0, 4.5));
            for tick in 0..25 {
                if tick == 21 || tick == 23 {
                    i(
                        &mut o,
                        e.attack_from(&mut w, DamageSource::new(DamageKind::Magic), -1) as i32,
                    );
                }
                if tick != 0 {
                    e.on_update(&mut w);
                }
                entity(&e, &mut o);
                let EntityKind::EnderCrystal { rotation, health } = e.kind else {
                    panic!()
                };
                i(&mut o, rotation);
                i(&mut o, health);
                i(&mut o, e.data_watcher.int(8).unwrap());
                world_state(&mut w, &mut o);
            }
            blocks(&mut w, &mut o);
        }
    }
    o
}
fn fireball_state(w: &mut World, e: &Entity, o: &mut Vec<u8>) {
    entity(e, o);
    let s = match &e.kind {
        EntityKind::Fireball(s) | EntityKind::SmallFireball(s) => s,
        _ => panic!(),
    };
    let a = &s.arrow;
    for v in [
        a.tile[0],
        a.tile[1],
        a.tile[2],
        a.block as i32,
        a.ticks_ground,
        a.ticks_air,
    ] {
        i(o, v);
    }
    i(o, a.in_ground as i32);
    i(o, a.owner.unwrap_or(-1));
    for v in [s.acceleration.x, s.acceleration.y, s.acceleration.z] {
        d(o, v);
    }
    i(o, e.can_be_collided_with() as i32);
    f(o, e.brightness(w));
    i(o, e.packed_brightness(w, 1.0));
    world_state(w, o);
}
fn run_fireball(small: bool) -> Vec<u8> {
    use super::MovingObjectPosition as Hit;
    let mut o = vec![];
    for seed in 0..8 {
        for scene in 0..12 {
            let mut w = super::verify181::world();
            let mut owner = Entity::living(&mut w, "Cow").unwrap();
            owner.set_location_and_angles(-3.5, 68.0, 4.5, 27.0, -13.0);
            let oid = owner.entity_id;
            w.entity_joined_world(owner.clone());
            let mut e = if small {
                Entity::small_fireball_from_owner(&mut w, &owner, Vec3D::new(0.4, 0.2, 1.0))
            } else {
                Entity::fireball_from_owner(&mut w, &owner, Vec3D::new(0.4, 0.2, 1.0))
            };
            e.rand.set_seed(seed + 1000);
            w.multiplayer_world = scene == 1;
            fireball_state(&mut w, &e, &mut o);
            let s = match &e.kind {
                EntityKind::Fireball(s) | EntityKind::SmallFireball(s) => s.clone(),
                _ => panic!(),
            };
            if (2..8).contains(&scene) {
                w.chunks.get_mut(&(-1, 0)).unwrap().blocks[crate::world::chunk::idx(13, 67, 4)] = 1;
                e.fireball_impact(
                    &mut w,
                    Hit::Block {
                        x: -3,
                        y: 67,
                        z: 4,
                        side: scene - 2,
                        hit: Vec3D::new(-3.0, 67.0, 4.0),
                    },
                    &s,
                );
            } else if scene == 8 || scene == 9 {
                let mut target =
                    Entity::living(&mut w, if scene == 8 { "Cow" } else { "Blaze" }).unwrap();
                target.set_position(-2.0, 68.0, 5.0);
                let id = target.entity_id;
                w.entity_joined_world(target);
                e.fireball_impact(
                    &mut w,
                    Hit::Entity {
                        id,
                        hit: Vec3D::default(),
                    },
                    &s,
                );
            } else if scene == 10 {
                w.entities.get_mut(&oid).unwrap().dead = true;
            } else if scene == 11 {
                i(
                    &mut o,
                    e.attack_from(&mut w, DamageSource::entity(DamageKind::Mob, Some(oid)), 1)
                        as i32,
                );
            }
            for _tick in 0..24 {
                e.on_update(&mut w);
                fireball_state(&mut w, &e, &mut o);
                if e.dead {
                    break;
                }
            }
            if (2..10).contains(&scene) {
                blocks(&mut w, &mut o);
            }
        }
    }
    o
}
fn run_potion_registry() -> Vec<u8> {
    use super::potion::*;
    let mut o = vec![];
    for id in 0..32 {
        let p = by_id(id);
        i(&mut o, p.is_some() as i32);
        if let Some(p) = p {
            i(&mut o, p.id);
            text(&mut o, Some(p.name));
            for v in [p.harmful as i32, p.color, p.icon, (p.icon >= 0) as i32] {
                i(&mut o, v);
            }
            d(&mut o, p.effectiveness);
            i(&mut o, p.usable as i32);
            i(&mut o, matches!(id, HEAL | HARM) as i32);
        }
    }
    for id in 1..=19 {
        for amp in [-1, 0, 1, 2, 31, 32] {
            for duration in [-100, -1, 0, 1, 20, 1200, 123456] {
                let e = PotionEffect::new(id, duration, amp);
                i(&mut o, e.hash_code());
                i(&mut o, (e == e.clone()) as i32);
                text(&mut o, Some(&e.description()));
                text(&mut o, Some(&duration_string(e)));
                i(&mut o, e.ready() as i32);
            }
        }
    }
    for name in ["Cow", "Zombie", "Spider"] {
        for id in [HEAL, HARM] {
            for amp in 0..4 {
                for strength in [0.0, 0.005, 0.1, 0.5, 1.0, 1.5] {
                    for has_owner in [false, true] {
                        let mut w = super::verify181::world();
                        let mut e = Entity::living(&mut w, name).unwrap();
                        e.set_position(-3.5, 64.0, 4.5);
                        e.living.health = 5;
                        let mut owner = Entity::living(&mut w, "Cow").unwrap();
                        owner.set_position(4.0, 64.0, 4.0);
                        let owner_id = owner.entity_id;
                        w.entity_joined_world(owner);
                        e.apply_instant_potion(
                            &mut w,
                            id,
                            has_owner.then_some(owner_id),
                            amp,
                            strength,
                        );
                        entity(&e, &mut o);
                        world_state(&mut w, &mut o);
                    }
                }
            }
        }
    }
    o
}
fn run_damage_sources() -> Vec<u8> {
    use DamageKind::*;
    let mut o = vec![];
    let mut sources: Vec<_> = [
        InFire, OnFire, Lava, InWall, Drown, Starve, Cactus, Fall, OutOfWorld, Generic, Explosion,
        Magic,
    ]
    .into_iter()
    .map(DamageSource::new)
    .collect();
    for owner in [false, true] {
        sources.extend([
            DamageSource::entity(Mob, owner.then_some(0)),
            DamageSource::entity(Player, owner.then_some(1)),
            DamageSource::indirect(Arrow, 2, owner.then_some(0)),
            DamageSource::indirect(Fireball, 3, owner.then_some(0)),
            DamageSource::indirect(Thrown, 2, owner.then_some(0)),
            DamageSource::indirect(IndirectMagic, 2, owner.then_some(0)),
        ]);
    }
    for s in sources {
        text(&mut o, Some(s.name()));
        for v in [
            s.bypasses_armor(),
            s.is_fire(),
            s.is_projectile(),
            s.harms_creative(),
        ] {
            i(&mut o, v as i32);
        }
        f(&mut o, s.exhaustion());
        i(&mut o, s.attacker.unwrap_or(-1));
        i(&mut o, s.attacker.unwrap_or(-1));
    }
    o
}
fn run_path_values() -> Vec<u8> {
    use super::path::{ChunkPosition, PathEntity};
    let mut o = vec![];
    for x in [
        f64::NEG_INFINITY,
        -2147483649.0,
        -1.25,
        -1.0,
        -0.0,
        0.0,
        0.99,
        2147483648.0,
        f64::INFINITY,
        f64::NAN,
    ] {
        let p = ChunkPosition::from_vec(Vec3D::new(x, x + 0.125, -x));
        for v in [p.x, p.y, p.z, p.hash_code(), (p == p.clone()) as i32] {
            i(&mut o, v);
        }
    }
    for width in [
        -0.99,
        0.0,
        0.3,
        0.9,
        1.0,
        2.9,
        16.0,
        f32::NAN,
        f32::INFINITY,
    ] {
        let mut p = PathEntity {
            points: vec![[-3, 64, 7], [0, 65, -9], [1000, -1, 2]],
            index: 0,
        };
        i(&mut o, p.points.len() as i32);
        for _ in 0..3 {
            i(&mut o, (p.index >= p.points.len()) as i32);
            let v = p.position(width).unwrap();
            for n in [v.x, v.y, v.z] {
                d(&mut o, n);
            }
            p.index += 1;
        }
        i(&mut o, (p.index >= p.points.len()) as i32);
    }
    o
}
