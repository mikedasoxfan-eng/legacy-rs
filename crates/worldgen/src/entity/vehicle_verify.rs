//! Scripted vehicle worlds shared with VehicleOracle.java; snapshots include RNG and effects.
use super::{
    Entity, EntityKind, Vec3D,
    player::{ItemStack, PlayerInput},
    verify::snapshot,
};
use crate::{
    World,
    world::chunk::{Chunk, idx},
};
fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8, m: i32) {
    let c = w.chunk_mut(x >> 4, z >> 4);
    c.blocks[idx(x & 15, y, z & 15)] = id;
    c.data.set(x & 15, y, z & 15, m);
}
fn cart(w: &mut World, x: f64, y: f64, z: f64, t: i32, vx: f64, vz: f64) -> i32 {
    let mut e = Entity::minecart(w, x, y, z, t);
    e.motion.x = vx;
    e.motion.z = vz;
    let id = e.entity_id;
    w.entity_joined_world(e);
    id
}
fn fixture() -> World {
    let mut w = World::new(0, 300);
    w.set_allowed_mob_spawns(false, false);
    w.set_entity_determinism(100, 200, 0);
    for x in -6..=6 {
        for z in -6..=6 {
            let mut c = Chunk::new(x, z, vec![0; 32768].into_boxed_slice().try_into().unwrap());
            for bx in 0..16 {
                for bz in 0..16 {
                    c.blocks[idx(bx, 63, bz)] = 1;
                }
            }
            c.loaded = true;
            c.populated = true;
            w.chunks.insert((x, z), Box::new(c));
            w.chunk_list.push((x, z));
        }
    }
    w
}
fn rails(w: &mut World) {
    for k in 0..16 {
        let x = -30 + (k % 4) * 18;
        let z = -30 + (k / 4) * 18;
        let id = if k < 10 {
            66
        } else if k < 14 {
            27
        } else {
            28
        };
        let m = if k < 10 {
            k
        } else if k < 12 {
            k - 10
        } else if k < 14 {
            (k - 12) | 8
        } else {
            k - 14
        };
        for dx in -6..=6 {
            for dz in -6..=6 {
                raw(w, x + dx, 64, z + dz, id, m);
            }
        }
        let id = cart(
            w,
            x as f64 + 0.3,
            64.0,
            z as f64 + 0.7,
            k % 3,
            if k % 2 == 0 { 0.43 } else { -0.43 },
            (k % 3 - 1) as f64 * 0.27,
        );
        if let EntityKind::Minecart(s) = &mut w.entities.get_mut(&id).unwrap().kind {
            if s.cart_type == 2 {
                s.push_x = 0.7;
                s.push_z = -0.3;
                s.fuel = 10;
            }
        }
    }
}
fn train(w: &mut World) {
    for x in -40..=40 {
        raw(w, x, 64, 0, 66, 1);
    }
    let id = cart(w, -8.5, 64.0, 0.5, 2, 0.2, 0.0);
    if let EntityKind::Minecart(s) = &mut w.entities.get_mut(&id).unwrap().kind {
        s.push_x = 1.0;
        s.fuel = 1200;
    }
    cart(w, -6.5, 64.0, 0.5, 0, 0.0, 0.0);
    let id = cart(w, -4.5, 64.0, 0.5, 1, 0.0, 0.0);
    if let EntityKind::Minecart(s) = &mut w.entities.get_mut(&id).unwrap().kind {
        s.items[0] = Some(ItemStack::new(35, 64, 7));
        s.items[26] = Some(ItemStack::new(264, 43, 0));
    }
    let mut e = Entity::living(w, "Pig").unwrap();
    e.set_location_and_angles(5.5, 64.0, 0.5, 0.0, 0.0);
    w.entity_joined_world(e);
    cart(w, 12.5, 64.0, 0.5, 0, -0.25, 0.0);
    cart(w, 12.5, 68.0, 10.5, 0, 1.0, -1.0);
}
fn boats(w: &mut World) {
    for x in -35..=35 {
        for z in -20..=20 {
            raw(w, x, 64, z, 9, 0);
        }
    }
    for x in 8..=20 {
        for z in -20..=20 {
            raw(w, x, 64, z, 8, (x + z) & 7);
        }
    }
    for z in -20..=20 {
        raw(w, 25, 64, z, 1, 0);
        raw(w, 25, 65, z, 1, 0);
    }
    for k in 0..8 {
        let mut b = Entity::boat(
            w,
            -20.5 + k as f64 * 6.0,
            65.0 + (k % 3) as f64 * 0.2,
            -12.5 + k as f64 * 3.0,
        );
        b.motion = Vec3D::new(
            if k % 2 == 0 { 0.38 } else { -0.17 },
            -0.04,
            (k % 3 - 1) as f64 * 0.21,
        );
        b.yaw = k as f32 * 33.7;
        w.entity_joined_world(b);
    }
    let mut a = Entity::boat(w, 0.0, 65.0, 18.0);
    a.motion.x = 0.2;
    w.entity_joined_world(a);
    let mut b = Entity::boat(w, 1.2, 65.0, 18.0);
    b.motion.x = -0.2;
    w.entity_joined_world(b);
}
fn riding(w: &mut World) {
    w.spawn = (0, 64, 0);
    let mut p = Entity::player(w, "Player");
    p.set_location_and_angles(-15.5, 64.0, 0.5, 0.0, 0.0);
    w.entity_joined_world(p);
    for x in -20..=8 {
        raw(w, x, 64, 0, if x == 0 { 28 } else { 66 }, 1);
    }
    raw(w, 8, 64, 0, 66, 7);
    for z in 1..=25 {
        raw(w, 8, 64, z, 66, 0);
    }
    let c = cart(w, -15.5, 64.0, 0.5, 0, 0.38, 0.0);
    let mut p = w.entities.remove(&0).unwrap();
    p.use_on_entity(w, c);
    w.entities.insert(0, p);
    for x in -20..=0 {
        for z in 10..=25 {
            raw(w, x, 64, z, 9, 0);
        }
    }
    let b = Entity::boat(w, -10.0, 65.0, 15.0);
    w.entity_joined_world(b);
    let c = cart(w, -10.5, 64.0, 5.5, 1, 0.0, 0.0);
    if let EntityKind::Minecart(s) = &mut w.entities.get_mut(&c).unwrap().kind {
        s.items[2] = Some(ItemStack::new(263, 64, 0));
    }
    cart(w, -7.5, 64.0, 5.5, 2, 0.0, 0.0);
    let mut p = Entity::living(w, "Pig").unwrap();
    p.kind = EntityKind::Pig { saddled: true };
    p.set_location_and_angles(-5.5, 64.0, 5.5, 0.0, 0.0);
    w.entity_joined_world(p);
}
fn placing(w: &mut World) {
    w.spawn = (0, 64, 0);
    let mut p = Entity::player(w, "Player");
    p.set_location_and_angles(0.5, 66.0, 0.5, 0.0, 90.0);
    w.entity_joined_world(p);
    for x in -8..=8 {
        raw(w, x, 64, 8, 66, 1);
        raw(w, x, 64, 0, 9, 0);
    }
}
fn longrail(w: &mut World) {
    for x in 7..=70 {
        for z in -6..=6 {
            let mut c = Chunk::new(x, z, vec![0; 32768].into_boxed_slice().try_into().unwrap());
            for bx in 0..16 {
                for bz in 0..16 {
                    c.blocks[idx(bx, 63, bz)] = 1;
                }
            }
            c.loaded = true;
            c.populated = true;
            w.chunks.insert((x, z), Box::new(c));
            w.chunk_list.push((x, z));
        }
    }
    w.spawn = (0, 64, 0);
    let mut p = Entity::player(w, "Player");
    p.set_location_and_angles(-35.5, 64.0, 0.5, 0.0, 0.0);
    w.entity_joined_world(p);
    for x in -40..=1100 {
        raw(
            w,
            x,
            64,
            0,
            if x == 0 { 28 } else { 27 },
            if x == 0 { 1 } else { 9 },
        );
    }
    let c = cart(w, -35.5, 64.0, 0.5, 0, 0.38, 0.0);
    let mut p = w.entities.remove(&0).unwrap();
    p.use_on_entity(w, c);
    w.entities.insert(0, p);
}
fn extra(w: &mut World, o: &mut Vec<u8>) {
    for id in 0..16 {
        let Some(e) = w
            .entities
            .get(&id)
            .filter(|e| matches!(e.kind, EntityKind::Minecart(..)))
            .cloned()
        else {
            continue;
        };
        for offset in [0.0, 0.3f32 as f64, -0.3f32 as f64, 1.2, -1.2] {
            let p = if offset == 0.0 {
                Entity::rail_position(w, e.pos.x, e.pos.y, e.pos.z)
            } else {
                Entity::rail_offset(w, e.pos.x, e.pos.y, e.pos.z, offset)
            };
            o.push(p.is_some() as u8);
            if let Some(p) = p {
                for v in [p.x, p.y, p.z] {
                    o.extend_from_slice(&v.to_bits().to_le_bytes());
                }
            }
        }
    }
    let mut stats: Vec<(i32, i32)> = Vec::new();
    for id in w.player_entities.clone().into_iter().flatten() {
        for event in w.take_player_events(id) {
            match event {
                super::player::PlayerEvent::Stat { name, amount } => {
                    let id = match name {
                        "distanceByMinecart" => 2006,
                        "distanceByBoat" => 2007,
                        "distanceByPig" => 2008,
                        _ => continue,
                    };
                    stats.push((id, amount));
                }
                super::player::PlayerEvent::Container(
                    crate::container::ContainerEvent::Achievement("onARail"),
                ) => stats.push((5242891, 1)),
                _ => {}
            }
        }
    }
    o.extend_from_slice(&(stats.len() as i32).to_le_bytes());
    for (id, n) in stats {
        o.extend_from_slice(&id.to_le_bytes());
        o.extend_from_slice(&n.to_le_bytes());
    }
    let mut vehicles: Vec<_> = w
        .loaded_entity_list
        .iter()
        .filter_map(|id| w.entities.get(id))
        .filter(|e| matches!(e.kind, EntityKind::Minecart(..) | EntityKind::Boat(..)))
        .collect();
    vehicles.sort_by_key(|e| e.entity_id);
    o.extend_from_slice(&(vehicles.len() as i32).to_le_bytes());
    for e in vehicles {
        o.extend_from_slice(&e.entity_id.to_le_bytes());
        nbt_bytes(e, o);
    }
}
fn nbt_bytes(e: &Entity, o: &mut Vec<u8>) {
    use crate::nbt::{Compound, Tag};
    fn sort(n: &mut Compound) {
        n.0.sort_by(|a, b| a.0.cmp(&b.0));
        for (_, t) in &mut n.0 {
            walk(t);
        }
    }
    fn walk(t: &mut Tag) {
        match t {
            Tag::Compound(n) => sort(n),
            Tag::List(_, items) => {
                for item in items {
                    walk(item);
                }
            }
            _ => {}
        }
    }
    let mut n = e.to_nbt().unwrap();
    sort(&mut n);
    let mut bytes = Vec::new();
    crate::nbt::write_root(&mut bytes, &n).unwrap();
    o.extend_from_slice(&(bytes.len() as i32).to_le_bytes());
    o.extend(bytes);
}
pub fn inventory_bytes() -> Vec<u8> {
    let mut w = fixture();
    let id = cart(&mut w, 0.5, 64.0, 0.5, 1, 0.0, 0.0);
    let mut player = Entity::player(&mut w, "Player");
    let mut rng = crate::JavaRandom::new(1729);
    let mut out = b"B173VIN1".to_vec();
    out.extend_from_slice(&10000i32.to_le_bytes());
    let items = [1, 35, 263, 264, 267, 280, 351, 332, 5, 61, 54, 328];
    for _ in 0..10000 {
        let op = rng.next_int(6);
        let slot = rng.next_int(27) as usize;
        let count = rng.next_int(100) - 10;
        let item = items[rng.next_int(items.len() as i32) as usize];
        let mut e = w.entities.remove(&id).unwrap();
        let mut result = None;
        let mut usable = false;
        match op {
            0 => {
                if let EntityKind::Minecart(s) = &mut e.kind {
                    s.set_stack(slot, Some(ItemStack::new(item, count, slot as i32)));
                }
            }
            1 => {
                if let EntityKind::Minecart(s) = &mut e.kind {
                    result = s.decrement_stack(slot, count);
                }
            }
            2 => {
                if let EntityKind::Minecart(s) = &e.kind {
                    result = s.stack_in_slot(slot);
                }
            }
            3 => {
                if let EntityKind::Minecart(s) = &mut e.kind {
                    s.cart_type = rng.next_int(3);
                    s.fuel = rng.next_int(131072) - 65536;
                    s.push_x = rng.next_double() * 4.0 - 2.0;
                    s.push_z = rng.next_double() * 4.0 - 2.0;
                }
                let n = e.to_nbt().unwrap();
                e.read_existing_nbt(&mut w, &n).unwrap();
            }
            4 => {
                player.set_position(rng.next_double() * 16.0 - 8.0, 64.0, 0.5);
                usable = e.can_interact_with_cart(&player);
            }
            _ => {
                if let EntityKind::Minecart(s) = &mut e.kind {
                    s.damage = rng.next_int(80);
                }
                e.perform_vehicle_hurt_animation();
            }
        }
        super::player::verify::write(&mut out, result);
        out.push(usable as u8);
        w.entities.insert(id, e);
        snapshot(&mut w, &mut out);
        nbt_bytes(&w.entities[&id], &mut out);
    }
    out
}
pub fn replay(scene: &str, ticks: i32) -> Vec<u8> {
    let mut w = fixture();
    match scene {
        "rails" => rails(&mut w),
        "train" | "nbt" => train(&mut w),
        "boats" => boats(&mut w),
        "longrail" => longrail(&mut w),
        "placing" => placing(&mut w),
        _ => riding(&mut w),
    };
    let mut o = b"B173VEH1".to_vec();
    o.extend_from_slice(&ticks.to_le_bytes());
    snapshot(&mut w, &mut o);
    extra(&mut w, &mut o);
    for t in 0..ticks {
        if scene == "placing" && t % 20 == 0 {
            let mut p = w.entities.remove(&0).unwrap();
            p.set_location_and_angles(0.5, 66.0, 0.5, 0.0, 90.0);
            p.motion = Vec3D::default();
            let phase = t / 20 % 5;
            let item = [328, 342, 343, 333, 333][phase as usize];
            p.player_state_mut().unwrap().inventory.main[0] = Some(ItemStack::new(item, 1, 0));
            if phase < 3 {
                p.use_on_block(&mut w, (phase - 1) * 3, 64, 8, 1);
            } else {
                raw(&mut w, 0, 65, 0, if phase == 4 { 78 } else { 0 }, 0);
                p.use_item(&mut w);
            }
            w.entities.insert(0, p);
        }
        if scene == "longrail" {
            if t == 80 {
                w.entities
                    .get_mut(&1)
                    .unwrap()
                    .set_position(1000.5, 64.5, 0.5);
            }
            let p = w.entities.get_mut(&0).unwrap();
            p.set_player_input(PlayerInput {
                forward: true,
                left: true,
                sneak: (80..100).contains(&t),
                ..Default::default()
            });
        }
        if scene == "riding" {
            let mut p = w.entities.remove(&0).unwrap();
            p.set_player_input(PlayerInput {
                forward: t % 40 < 30,
                left: t % 60 < 15,
                yaw_delta: (t % 7 - 3) as f32 * 7.3,
                pitch_delta: (t % 5 - 2) as f32 * 1.7,
                ..Default::default()
            });
            if t == 40 {
                let id = w.loaded_entity_list[1];
                p.use_on_entity(&mut w, id);
            }
            if t == 45 {
                let id = w.loaded_entity_list[2];
                p.use_on_entity(&mut w, id);
            }
            if t == 110 {
                let id = *w
                    .loaded_entity_list
                    .iter()
                    .find(|id| {
                        w.entities
                            .get(id)
                            .is_some_and(|e| matches!(e.kind, EntityKind::Pig { .. }))
                    })
                    .unwrap();
                p.mount_entity(&mut w, Some(id));
            }
            if t == 140 {
                p.mount_entity(&mut w, None);
            }
            if t == 90 {
                p.player_state_mut().unwrap().inventory.main[0] = Some(ItemStack::new(263, 3, 0));
                let id = w.loaded_entity_list[3];
                p.use_on_entity(&mut w, id);
            }
            w.entities.insert(0, p);
            if t == 80 {
                let id = w.loaded_entity_list[2];
                let mut e = w.entities.remove(&id).unwrap();
                e.attack(&mut w, Some(0), 5);
                w.entities.insert(id, e);
            }
            if t == 100 {
                let id = w.loaded_entity_list[2];
                let mut e = w.entities.remove(&id).unwrap();
                e.attack(&mut w, Some(0), 5);
                w.entities.insert(id, e);
            }
        }
        if scene == "train" && t == 80 || scene == "boats" && t == 50 {
            for id in w.loaded_entity_list.clone() {
                let mut e = w.entities.remove(&id).unwrap();
                if scene == "train" && matches!(&e.kind,EntityKind::Minecart(s) if s.cart_type==1)
                    || scene == "boats" && matches!(e.kind, EntityKind::Boat(..)) && id % 3 == 0
                {
                    e.attack(&mut w, None, 5);
                }
                w.entities.insert(id, e);
            }
        }
        w.update_entities();
        if matches!(scene, "riding" | "longrail") {
            w.world_time += 1;
            w.tick_updates(false);
        }
        if matches!(scene, "nbt" | "ridingnbt") && t % 25 == 24 {
            let pending: Vec<_> = w
                .player_entities
                .clone()
                .into_iter()
                .flatten()
                .flat_map(|id| w.take_player_events(id))
                .collect();
            let tags: Vec<_> = w
                .loaded_entity_list
                .iter()
                .filter_map(|id| {
                    let e = &w.entities[id];
                    if e.player_state().is_some() {
                        let mut n = e.player_nbt()?;
                        n.put("id", crate::nbt::Tag::String("Player".into()));
                        Some(n)
                    } else {
                        e.to_nbt()
                    }
                })
                .collect();
            w.entities.clear();
            w.loaded_entity_list.clear();
            w.players.clear();
            w.player_entities.clear();
            for c in w.chunks.values_mut() {
                for list in &mut c.entity_lists {
                    list.clear();
                }
            }
            for tag in tags {
                let e = if tag.string("id") == "Player" {
                    Entity::player_from_nbt(&mut w, &tag, "Player").unwrap()
                } else {
                    Entity::from_nbt(&mut w, &tag).unwrap()
                };
                w.entity_joined_world(e);
            }
            if let Some(id) = w.player_entities.iter().flatten().next().copied() {
                w.entities
                    .get_mut(&id)
                    .unwrap()
                    .player_state_mut()
                    .unwrap()
                    .events
                    .extend(pending);
            }
        }
        snapshot(&mut w, &mut o);
        extra(&mut w, &mut o);
    }
    super::verify::append_world(&w, &mut o);
    o
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::{ContainerCommand, ContainerEvent};
    #[test]
    fn chest_cart_window_mutates_entity_and_closes_at_distance() {
        let mut w = fixture();
        w.spawn = (0, 64, 0);
        let mut p = Entity::player(&mut w, "Player");
        p.set_location_and_angles(0.5, 64.0, 0.5, 0.0, 0.0);
        p.player_state_mut().unwrap().inventory.main[0] = Some(ItemStack::new(263, 40, 0));
        let player = p.entity_id;
        w.entity_joined_world(p);
        let cart = cart(&mut w, 0.5, 64.0, 0.5, 1, 0.0, 0.0);
        let opened = w
            .container_command(player, ContainerCommand::OpenVehicle { entity: cart })
            .unwrap();
        assert_eq!(opened.inventory_name, Some("Minecart"));
        assert_eq!(opened.contents.len(), 63);
        w.container_command(
            player,
            ContainerCommand::Click {
                window: opened.window,
                slot: 54,
                button: 0,
                shift: true,
            },
        );
        if let EntityKind::Minecart(s) = &w.entities[&cart].kind {
            assert_eq!(s.items[0], Some(ItemStack::new(263, 40, 0)));
        } else {
            panic!("cart");
        }
        assert!(w.entities[&player].held_item().is_none());
        w.container_command(
            player,
            ContainerCommand::Click {
                window: opened.window,
                slot: 0,
                button: 1,
                shift: false,
            },
        );
        w.entities
            .get_mut(&cart)
            .unwrap()
            .set_position(20.5, 64.0, 0.5);
        w.update_entities();
        assert!(w.container_snapshot(player).is_none());
        assert!(w.take_player_events(player).iter().any(|e|matches!(e,super::super::player::PlayerEvent::Container(ContainerEvent::Closed {window}) if *window==opened.window)));
        assert!(w.loaded_entity_list.iter().any(|id|matches!(&w.entities[id].kind,EntityKind::Item {item,..} if item.id==263 && item.count==20)));
    }
    #[test]
    fn passenger_can_dismount_or_break_vehicle_during_recursive_tick() {
        let mut w = fixture();
        riding(&mut w);
        w.update_entities();
        let mut p = w.entities.remove(&0).unwrap();
        p.use_on_entity(&mut w, 1);
        w.entities.insert(0, p);
        assert_eq!(w.entities[&1].ridden_by, None);
        assert_eq!(w.entities[&0].riding, None);
        let mut p = w.entities.remove(&0).unwrap();
        p.use_on_entity(&mut w, 2);
        w.entities.insert(0, p);
        let mut boat = w.entities.remove(&2).unwrap();
        boat.attack(&mut w, Some(0), 5);
        w.entities.insert(2, boat);
        w.update_entities();
        assert_eq!(w.entities[&0].riding, None);
        assert!(!w.entities.contains_key(&2));
    }
}
