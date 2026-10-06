//! Round 3: original player object identities, End requests/travel and direct path cases.
use super::{
    player::{food::FoodAction, ItemStack, PlayerEvent},
    Entity, Vec3D,
};
use crate::{
    nbt::{Compound, Tag},
    world::Dimension,
    JavaRandom, World,
};
fn i(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_le_bytes());
}
fn f(o: &mut Vec<u8>, v: f32) {
    o.extend(v.to_bits().to_le_bytes());
}
fn d(o: &mut Vec<u8>, v: f64) {
    o.extend(v.to_bits().to_le_bytes());
}
fn l(o: &mut Vec<u8>, v: i64) {
    o.extend(v.to_le_bytes());
}
fn text(o: &mut Vec<u8>, v: Option<&str>) {
    if let Some(v) = v {
        i(o, v.len() as i32);
        o.extend(v.as_bytes());
    } else {
        i(o, -1);
    }
}
fn player(o: &mut Vec<u8>, p: &Entity) {
    let s = p.player_state().unwrap();
    let food = s.food.snapshot();
    i(o, p.living.health);
    f(o, s.experience.points);
    i(o, s.experience.level);
    i(o, s.experience.total);
    i(o, s.score);
    for v in [food.level, food.tick_timer, food.prev_level] {
        i(o, v);
    }
    f(o, food.saturation);
    f(o, food.exhaustion);
    i(o, s.inventory.current_item);
    for stack in s.inventory.main.iter().chain(&s.inventory.armor) {
        i(o, stack.map_or(-1, |s| s.id));
        i(o, stack.map_or(0, |s| s.count));
        i(o, stack.map_or(0, |s| s.damage));
    }
    i(o, s.dimension);
    i(o, s.capabilities.flying as i32);
    i(o, s.capabilities.creative as i32);
}
fn copy() -> Vec<u8> {
    let mut o = vec![];
    for scene in 0..24 {
        let mut w = super::verify181::world();
        let mut a = Entity::player(&mut w, "Player");
        let mut b = Entity::player(&mut w, "Player");
        let mut c = Entity::player(&mut w, "Player");
        a.living.health = scene % 20 + 1;
        let s = a.player_state_mut().unwrap();
        s.experience.points = scene as f32 * 0.03125;
        s.experience.level = scene;
        s.experience.total = scene * 113;
        s.score = scene * 37;
        s.dimension = 1;
        s.capabilities.flying = true;
        s.capabilities.creative = true;
        s.inventory.current_item = scene % 9;
        s.inventory.main[scene as usize] = Some(ItemStack::new(35, scene + 1, scene % 16));
        s.inventory.armor[(scene % 4) as usize] =
            Some(ItemStack::new(310 + scene % 4, 1, scene * 3));
        let mut n = Compound::new();
        n.put("foodLevel", Tag::Int(scene % 21));
        n.put("foodTickTimer", Tag::Int(scene * 7));
        n.put("foodSaturationLevel", Tag::Float(scene as f32 * 0.25));
        n.put("foodExhaustionLevel", Tag::Float(scene as f32 * 0.5));
        s.food.read_nbt(&n);
        b.player_state_mut().unwrap().inventory.current_item = 8;
        b.copy_player_from(&a);
        c.copy_player_from(&b);
        i(
            &mut o,
            a.player_state()
                .unwrap()
                .food
                .same_object(&b.player_state().unwrap().food) as i32,
        );
        i(
            &mut o,
            b.player_state()
                .unwrap()
                .food
                .same_object(&c.player_state().unwrap().food) as i32,
        );
        i(
            &mut o,
            (!a.player_state().unwrap().inventory.main[scene as usize]
                .unwrap()
                .same_object(b.player_state().unwrap().inventory.main[scene as usize].unwrap()))
                as i32,
        );
        b.player_state_mut().unwrap().inventory.main[scene as usize]
            .as_mut()
            .unwrap()
            .count += 5;
        for tick in 0..240 {
            let p = match tick % 3 {
                0 => &mut a,
                1 => &mut b,
                _ => &mut c,
            };
            if tick % 17 == 0 {
                p.player_state().unwrap().food.add_food(2, 0.3);
            }
            if tick % 7 == 0 {
                p.player_state().unwrap().food.add_exhaustion(3.25);
            }
            w.difficulty = (tick / 60) % 4;
            let action = p
                .player_state()
                .unwrap()
                .food
                .tick(w.difficulty, p.living.health);
            match action {
                FoodAction::Heal => p.heal(1),
                FoodAction::Starve => {
                    p.attack_from(
                        &mut w,
                        super::damage::DamageSource::new(super::damage::DamageKind::Starve),
                        1,
                    );
                }
                FoodAction::None => {}
            }
            if tick == 100 {
                n.put("foodLevel", Tag::Int(0));
                n.put("foodSaturationLevel", Tag::Float(0.0));
                n.put("foodExhaustionLevel", Tag::Float(0.0));
                n.put("foodTickTimer", Tag::Int(79));
                b.player_state().unwrap().food.read_nbt(&n);
            }
            player(&mut o, &a);
            player(&mut o, &b);
            player(&mut o, &c);
        }
    }
    o
}
fn end_events() -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..16 {
        for remote in [false, true] {
            for from in [-1, 0, 1] {
                for to in [-1, 0, 1, 2] {
                    let mut w = super::verify181::world();
                    w.multiplayer_world = remote;
                    let mut p = Entity::player(&mut w, "Player");
                    p.player_state_mut().unwrap().dimension = from;
                    p.rand = JavaRandom::new(seed);
                    p.enter_end(&mut w, to);
                    let s = p.player_state().unwrap();
                    let requested = s
                        .events
                        .iter()
                        .find_map(|e| {
                            if let PlayerEvent::ChangeDimensionTo { dimension } = e {
                                Some(*dimension)
                            } else {
                                None
                            }
                        })
                        .unwrap_or(-999);
                    i(&mut o, requested);
                    text(
                        &mut o,
                        s.events
                            .iter()
                            .any(|e| matches!(e, PlayerEvent::WinGame))
                            .then_some("GuiWinGame"),
                    );
                    i(&mut o, s.stat_log.len() as i32);
                    for (id, amount) in &s.stat_log {
                        i(&mut o, *id);
                        i(&mut o, *amount);
                    }
                    l(&mut o, p.rand.state());
                    crate::world::events::write_events(&w.drain_events(), &mut o);
                }
            }
        }
    }
    o
}
fn magic() -> Vec<u8> {
    let mut o = vec![];
    for scene in 0..32 {
        let mut w = super::verify181::world();
        let mut p = Entity::player(&mut w, "Player");
        let mut target = if scene % 2 == 0 {
            Entity::living(&mut w, "Zombie").unwrap()
        } else {
            Entity::boat(&mut w, 0.0, 0.0, 0.0)
        };
        target.set_position(
            scene as f64 * 0.25 - 4.0,
            64.0 + scene as f64 * 0.125,
            scene as f64 * 0.5,
        );
        p.magic_critical_hit(&target);
        let PlayerEvent::MagicCriticalHit { target: request } =
            p.player_state().unwrap().events.last().unwrap()
        else {
            panic!("missing magic request")
        };
        text(&mut o, Some("magicCrit"));
        i(&mut o, (request.entity_id == target.entity_id) as i32);
        i(&mut o, request.entity_id);
        for v in [request.pos.x, request.pos.y, request.pos.z] {
            d(&mut o, v);
        }
    }
    o
}
fn travel() -> Vec<u8> {
    let mut o = vec![];
    for from in [0, -1, 1] {
        let mut w = super::verify181::world();
        let mut p = Entity::player(&mut w, "Player");
        p.player_state_mut().unwrap().dimension = from;
        p.set_location_and_angles(-3.75, 71.125, 4.875, 37.0, -12.0);
        p.motion = Vec3D::new(0.25, -0.125, 0.75);
        p.player_state_mut().unwrap().inventory.main[0] = Some(ItemStack::new(264, 13, 0));
        let id = p.entity_id;
        w.entity_joined_world(p);
        let dimension = if from == 1 {
            Dimension::Overworld
        } else {
            Dimension::End
        };
        let mut title = None;
        let mut w = w
            .use_portal_to(id, dimension, 700, 800, &mut |step| {
                if let crate::scenario::LoadStep::Title(s) = step {
                    title = Some(s.to_owned());
                }
            })
            .unwrap();
        text(&mut o, title.as_deref());
        let p = w.entities.get(&id).unwrap();
        player(&mut o, p);
        for v in [
            p.pos.x,
            p.pos.y,
            p.pos.z,
            p.motion.x,
            p.motion.y,
            p.motion.z,
            p.bounding_box.min_y,
        ] {
            d(&mut o, v);
        }
        f(&mut o, p.yaw);
        f(&mut o, p.pitch);
        if from != 1 {
            for x in 98..=102 {
                for y in 51..=54 {
                    for z in -2..=2 {
                        i(&mut o, w.get_block_id(x, y, z) as i32);
                    }
                }
            }
        }
    }
    o
}
fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8, meta: u8) {
    w.provide_chunk(x >> 4, z >> 4);
    let c = w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap();
    c.blocks[crate::world::chunk::idx(x & 15, y, z & 15)] = id;
    c.data.set(x & 15, y, z & 15, meta as i32);
}
fn paths() -> Vec<u8> {
    let mut o = vec![];
    for scene in 0..10 {
        for range in [0.0, 1.0, 4.0, 8.0, 16.0, 24.0] {
            for mob in ["Zombie", "Spider", "Giant"] {
                let mut w = super::verify181::world();
                let mut e = Entity::living(&mut w, mob).unwrap();
                e.set_position(-3.5, 64.0, 4.5);
                for z in -4..=14 {
                    for y in 64..=67 {
                        let block = match scene {
                            1 | 8 => 1,
                            2 => 9,
                            3 => 11,
                            4 => 85,
                            5 => 64,
                            6 => 71,
                            7 => 96,
                            _ => 0,
                        };
                        if scene != 8 || z != 5 {
                            raw(&mut w, 0, y, z, block, 0);
                        }
                    }
                }
                if scene == 9 {
                    for x in -2..=2 {
                        for z in 2..=7 {
                            raw(&mut w, x, 63, z, 0, 0);
                        }
                    }
                }
                let path = w.entity_path_to_xyz(&e, 5, 64, 6, range);
                i(&mut o, path.is_some() as i32);
                if let Some(path) = path {
                    i(&mut o, path.points.len() as i32);
                    for point in path.points {
                        for v in point {
                            i(&mut o, v);
                        }
                    }
                }
                l(&mut o, e.rand.state());
            }
        }
    }
    o
}
fn respawn() -> Vec<u8> {
    let mut o = vec![];
    for creative in [false, true] {
        for keep in [false, true] {
            let mut w = super::verify181::world();
            w.spawn = (0, 64, 0);
            w.info.spawn = w.spawn;
            w.info.game_type = creative as i32;
            for x in -10..=10 {
                for z in -10..=10 {
                    if !w.chunks.contains_key(&(x, z)) {
                        let mut c = crate::world::chunk::Chunk::new(
                            x,
                            z,
                            vec![0; 32768].into_boxed_slice().try_into().unwrap(),
                        );
                        c.loaded = true;
                        c.populated = true;
                        for bx in 0..16 {
                            for bz in 0..16 {
                                c.blocks[crate::world::chunk::idx(bx, 63, bz)] = 1;
                            }
                        }
                        w.chunks.insert((x, z), Box::new(c));
                    }
                }
            }
            let mut p = Entity::player(&mut w, "Player");
            p.set_position(0.5, 70.0, 0.5);
            p.living.health = 7;
            let s = p.player_state_mut().unwrap();
            s.experience.points = 0.625;
            s.experience.level = 13;
            s.experience.total = 321;
            s.score = 456;
            s.inventory.current_item = 7;
            s.inventory.main[7] = Some(ItemStack::new(264, 13, 0));
            s.inventory.armor[3] = Some(ItemStack::new(310, 1, 33));
            let mut n = Compound::new();
            n.put("foodLevel", Tag::Int(9));
            n.put("foodSaturationLevel", Tag::Float(2.5));
            n.put("foodExhaustionLevel", Tag::Float(3.75));
            n.put("foodTickTimer", Tag::Int(37));
            s.food.read_nbt(&n);
            let id = p.entity_id;
            let retained = p.clone();
            w.entity_joined_world(p);
            w.respawn_player_with_inventory(id, keep, &mut |_| {});
            let q = w.entities.get(&id).unwrap();
            player(&mut o, q);
            i(
                &mut o,
                retained
                    .player_state()
                    .unwrap()
                    .food
                    .same_object(&q.player_state().unwrap().food) as i32,
            );
            i(&mut o, (id == q.entity_id) as i32);
            for v in [
                q.pos.x, q.pos.y, q.pos.z, q.motion.x, q.motion.y, q.motion.z,
            ] {
                d(&mut o, v);
            }
            f(&mut o, q.yaw);
            f(&mut o, q.pitch);
            q.player_state().unwrap().food.add_food(3, 0.1);
            player(&mut o, &retained);
            player(&mut o, q);
        }
    }
    o
}
fn falls(mob: &str) -> Vec<u8> {
    let mut o = vec![];
    for seed in 0..4 {
        for distance in [-1.0, 0.0, 3.0, 3.01, 4.0, 8.0, 30.0] {
            for mounted in [false, true] {
                let mut w = super::verify181::world();
                let mut e = Entity::living(&mut w, mob).unwrap();
                e.set_position(-3.5, 64.0, 4.5);
                e.rand.set_seed(seed + 1000);
                if mounted {
                    let mut rider = Entity::living(&mut w, "Zombie").unwrap();
                    rider.set_position(-3.5, 66.0, 4.5);
                    rider.riding = Some(e.entity_id);
                    e.ridden_by = Some(rider.entity_id);
                    w.entity_joined_world(rider);
                }
                e.fall(&mut w, distance);
                super::round2_verify10::entity(&e, &mut o);
                super::round2_verify10::world_state(&mut w, &mut o);
            }
        }
    }
    o
}
pub fn run(kind: &str) -> Vec<u8> {
    if let Some(mob) = kind.strip_prefix("fall-") {
        return falls(mob);
    }
    if let Some(kind) = kind.strip_prefix("covered-") {
        return super::round2_verify10::run(kind);
    }
    match kind {
        "player-copy-alias" => copy(),
        "playersp-end-events" => end_events(),
        "player-end-travel" => travel(),
        "playersp-magic-request" => magic(),
        "pathfinder-obstacles" => paths(),
        "player-respawn-copy" => respawn(),
        _ => panic!("unknown check {kind}"),
    }
}
