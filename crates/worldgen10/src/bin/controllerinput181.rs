use worldgen::{
    World,
    entity::{
        Entity, MovingObjectPosition as Hit,
        player::{controller::PlayerControllerSP, inventory::ItemStack},
    },
};
fn i(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_le_bytes());
}
fn f(o: &mut Vec<u8>, v: f32) {
    o.extend(v.to_le_bytes());
}
fn stack(o: &mut Vec<u8>, s: Option<ItemStack>) {
    for v in s.map_or([-1, 0, 0, 0], |s| [s.id, s.count, s.damage, s.animations]) {
        i(o, v);
    }
}
fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8, meta: i32) {
    let c = w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap();
    let at = worldgen::world::chunk::idx(x & 15, y, z & 15);
    c.blocks[at] = id;
    c.data.set(x & 15, y, z & 15, meta);
}
fn state(
    w: &mut World,
    p: &mut Entity,
    c: &PlayerControllerSP,
    creative: bool,
    result: i32,
    o: &mut Vec<u8>,
) {
    i(o, result);
    if creative {
        i(o, c.block_hit_wait);
    } else {
        for v in [
            c.cur_block_damage,
            c.prev_block_damage,
            c.step_sound_counter,
        ] {
            f(o, v);
        }
        for v in [c.target.0, c.target.1, c.target.2, c.block_hit_wait] {
            i(o, v);
        }
    }
    for y in 64..=65 {
        for x in 3..=5 {
            for z in 3..=5 {
                i(o, w.get_block_id(x, y, z) as i32);
                i(o, w.get_block_metadata(x, y, z));
            }
        }
    }
    for s in p.player_state().unwrap().inventory.main {
        stack(o, s);
    }
    f(o, p.player_state().unwrap().food.read().exhaustion);
    let log = std::mem::take(&mut p.player_state_mut().unwrap().stat_log);
    i(o, log.len() as i32);
    for (id, n) in log {
        i(o, id);
        i(o, n);
    }
    o.extend(w.rand.state().to_le_bytes());
    o.extend(w.item_random.state().to_le_bytes());
    worldgen::world::events::write_events(&w.drain_events(), o);
    i(o, w.loaded_entity_list.len() as i32);
    for id in w.loaded_entity_list.clone() {
        let e = w.entities[&id].clone();
        worldgen::entity::player::verify181::dropped_entity_snapshot(w, &e, o);
    }
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut out = Vec::new();
    for creative in [false, true] {
        let mut w = worldgen::entity::verify181::world();
        w.item_random = worldgen::JavaRandom::new(605);
        let mut p = Entity::player(&mut w, "Player");
        p.on_ground = true;
        p.set_location_and_angles(0.5, 64.0, 0.5, 0.0, 0.0);
        p.set_creative(creative);
        let id = p.entity_id;
        w.entities.insert(id, p);
        for t in 0..1400 {
            let phase = t % 70;
            if phase == 0 {
                let p = w.entities.get_mut(&id).unwrap();
                p.clear_item_use();
                let s = p.player_state_mut().unwrap();
                s.inventory.current_item = 0;
                s.inventory.main[0] = Some(ItemStack::new(
                    [278, 359, 267, 5, 260, 261, 282][t as usize / 70 % 7],
                    [1, 3, 64, 0, -1][t as usize / 70 % 5],
                    0,
                ));
                s.inventory.main[8] = Some(ItemStack::new(262, 0, 0));
                s.food.write().level = 10;
                raw(
                    &mut w,
                    4,
                    64,
                    4,
                    [1, 3, 18, 31, 106][t as usize / 70 % 5],
                    t / 70 % 4,
                );
                raw(&mut w, 4, 65, 4, 0, 0);
            }
            let hit = if phase % 11 < 8 {
                Some(Hit::Block {
                    x: 4,
                    y: 64,
                    z: 4,
                    side: 1,
                    hit: worldgen::entity::Vec3D::new(4.0, 65.0, 4.0),
                })
            } else {
                None
            };
            w.apply_player_tick_input(
                id,
                worldgen::entity::player::api::PlayerTickInput {
                    target: worldgen::entity::player::api::PlayerTarget::ObjectMouseOver(hit),
                    left_held: phase < 17 || phase >= 55,
                    right_held: phase >= 20 && phase < 48,
                    attack_presses: if phase == 0 {
                        2
                    } else if phase == 55 {
                        1
                    } else {
                        0
                    },
                    use_presses: if phase == 20 {
                        2
                    } else if phase == 30 {
                        1
                    } else {
                        0
                    },
                    pick_presses: if phase == 25 || phase == 50 { 1 } else { 0 },
                    ..Default::default()
                },
                t,
            );
            let mut p = w.entities.remove(&id).unwrap();
            let c = p.player_state().unwrap().controller.clone();
            let mut frame = Vec::new();
            state(&mut w, &mut p, &c, creative, -1, &mut frame);
            let s = p.player_state().unwrap();
            for n in [
                s.left_click_counter,
                s.right_click_delay,
                s.swing_ticks,
                s.item_use_ticks,
                s.inventory.current_item,
                s.pending_attack_presses,
                s.pending_use_presses,
                s.pending_pick_presses,
            ] {
                i(&mut frame, n);
            }
            frame.push(s.is_swinging as u8);
            stack(&mut frame, s.item_in_use);
            i(&mut out, frame.len() as i32);
            out.extend(frame);
            w.entities.insert(id, p);
        }
    }
    std::fs::write(&args[1], &out).unwrap();
    let expected = std::fs::read(&args[2]).unwrap();
    if out != expected {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "CONTROLLER INPUT DIFFERS at byte {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        std::process::exit(1);
    }
    println!(
        "CONTROLLER INPUT EXACT: 2800 original click/held/use/queued-input transitions ({} bytes)",
        out.len()
    );
}
