//! Replay inputs recorded beside the original Minecraft.runTick results.
use crate::{
    gui::settings::{GameSettings, KeyBinding},
    minecraft_input::{self, Events, Host, KeyEvent, MouseEvent},
    worker::MinecraftHost,
};
use worldgen::entity::{player::inventory::ItemStack, Entity, MovingObjectPosition as Hit, Vec3D};
fn i(out: &mut Vec<u8>, n: i32) {
    out.extend(n.to_le_bytes());
}
struct Reader {
    bytes: Vec<u8>,
    at: usize,
}
impl Reader {
    fn int(&mut self) -> i32 {
        let n = i32::from_le_bytes(self.bytes[self.at..self.at + 4].try_into().unwrap());
        self.at += 4;
        n
    }
    fn boolean(&mut self) -> bool {
        let b = self.bytes[self.at] != 0;
        self.at += 1;
        b
    }
    fn bytes(&mut self, n: usize) -> Vec<u8> {
        let b = self.bytes[self.at..self.at + n].to_vec();
        self.at += n;
        b
    }
    fn double(&mut self) -> f64 {
        f64::from_le_bytes(self.bytes(8).try_into().unwrap())
    }
}

pub fn check_preload(path: &str) -> bool {
    use worldgen::scenario::{preload_grid, PreloadCall};
    let mut input = Reader {
        bytes: std::fs::read(path).unwrap(),
        at: 0,
    };
    let mut differences = 0;
    for case in 0..128 {
        let mut x = input.int();
        let mut z = input.int();
        let player = input.int() != 0;
        let loading = input.int() != 0;
        let reduced = input.int() != 0;
        let provider = input.int() != 0;
        let px = input.double();
        let pz = input.double();
        if player {
            x = px as i32;
            z = pz as i32;
        }
        let len = input.int() as usize;
        let expected = input.bytes(len * 4);
        let mut actual = Vec::new();
        let mut remaining = 0;
        if loading {
            i(&mut actual, 0);
            i(&mut actual, 1);
        }
        preload_grid(x, z, reduced, loading, &mut |call| match call {
            PreloadCall::Progress(n) => {
                i(&mut actual, 2);
                i(&mut actual, n);
                false
            }
            PreloadCall::Block(x, z) => {
                for n in [3, x, 64, z] {
                    i(&mut actual, n);
                }
                remaining = (x ^ z) & 3;
                false
            }
            PreloadCall::Lighting => {
                let more = remaining > 0;
                remaining -= 1;
                i(&mut actual, 4);
                i(&mut actual, more as i32);
                more
            }
        });
        if !reduced {
            if loading {
                i(&mut actual, 6);
            }
            i(&mut actual, 5);
        }
        if provider {
            for n in [7, x >> 4, 7, z >> 4] {
                i(&mut actual, n);
            }
        }
        if actual != expected {
            differences += 1;
            eprintln!("preloadWorld case {case} differs");
        }
    }
    assert_eq!(input.at, input.bytes.len());
    println!(
        "MINECRAFT PRELOAD {}: 128 cases, {differences} differences",
        if differences == 0 { "EXACT" } else { "DIFFERS" }
    );
    differences == 0
}

pub fn check_debug(path: &str) -> bool {
    let mut input = Reader {
        bytes: std::fs::read(path).unwrap(),
        at: 0,
    };
    let mut differences = 0;
    for _ in 0..10000 {
        let mut counts = [0usize; 4];
        for n in &mut counts {
            *n = input.int() as usize;
        }
        let entities = input.int() as usize;
        let len = input.int() as usize;
        let expected = input.bytes(len);
        let actual = crate::minecraft_input::debug_entities(counts[..3].iter().sum(), entities);
        if actual.as_bytes() != expected {
            differences += 1;
        }
    }
    assert_eq!(input.at, input.bytes.len());
    println!(
        "MINECRAFT DEBUG ENTITIES {}: 10000 cases, {differences} differences",
        if differences == 0 { "EXACT" } else { "DIFFERS" }
    );
    differences == 0
}
fn raw(w: &mut worldgen::World, x: i32, y: i32, z: i32, id: u8) {
    let c = w.chunks.get_mut(&(x >> 4, z >> 4)).unwrap();
    c.blocks[worldgen::world::chunk::idx(x & 15, y, z & 15)] = id;
    c.data.set(x & 15, y, z & 15, 0);
}
pub fn check(path: &str) -> bool {
    let mut input = Reader {
        bytes: std::fs::read(path).expect("runTick oracle fixture"),
        at: 0,
    };
    let mut count = 0;
    let mut differences = 0;
    let cases = input.int();
    assert_eq!(cases, 26);
    for _ in 0..cases {
        let creative = input.int() != 0;
        let scenario = input.int();
        let ticks = input.int();
        let mut world = worldgen::entity::verify181::world();
        world.item_random = worldgen::JavaRandom::new(605);
        let mut player = Entity::player(&mut world, "Player");
        player.on_ground = true;
        player.set_location_and_angles(0.5, 64.0, 0.5, 0.0, 0.0);
        player.set_creative(creative);
        let mut settings = GameSettings::default();
        KeyBinding::release_all();
        settings.smooth_camera_zoom = scenario % 2 == 0;
        if scenario == 6 {
            for (name, code) in [("key.attack", 33), ("key.use", 2), ("key.pickItem", 16)] {
                settings
                    .key_bindings
                    .iter_mut()
                    .find(|k| k.description == name)
                    .unwrap()
                    .set_code(code);
            }
            KeyBinding::rebuild_registry();
        }
        let s = player.player_state_mut().unwrap();
        s.inventory.main = [None; 36];
        s.inventory.armor = [None; 4];
        for slot in 0..9 {
            s.inventory.main[slot] = Some(ItemStack::new(
                match scenario {
                    1 => 267,
                    9 => 260,
                    10 => 261,
                    _ => 278,
                },
                8,
                0,
            ));
        }
        s.inventory.main[8] = Some(ItemStack::new(262, 64, 0));
        s.food.write().level = 10;
        let controller = std::mem::take(&mut s.controller);
        world.multiplayer_world = scenario == 8;
        let mut host = MinecraftHost {
            world: &mut world,
            player,
            controller,
            target: None,
            screen: false,
            focused: scenario != 4,
            multiplayer: scenario == 8,
            allow_input: false,
            actions: vec![],
            drops: 0,
            clicks: [0; 6],
        };
        let (mut world_ticks, mut updates, mut joins, mut display, mut join_counter) =
            (0, 0, 0, 0, 0);
        let (mut mobs, mut animals) = (false, false);
        for tick in 0..ticks {
            let close = input.int() != 0;
            let allow_screen = input.int() != 0;
            let fresh = input.boolean();
            let paused = input.boolean();
            let target = input.boolean();
            let difficulty = input.int();
            let mut flash = input.int();
            settings.difficulty = difficulty;
            if close {
                host.screen = false;
                host.focused = true;
                *host.left_counter() = 10000;
                host.allow_input = false;
            }
            if allow_screen {
                host.screen = true;
                host.allow_input = true;
                if host.focused {
                    KeyBinding::release_all();
                    host.focused = false;
                }
            }
            let mut events = Events::default();
            let n = input.int();
            for _ in 0..n {
                events.mouse.push(MouseEvent {
                    button: input.int(),
                    pressed: input.int() != 0,
                    wheel: input.int(),
                    fresh,
                });
            }
            let n = input.int();
            for _ in 0..n {
                events.keys.push(KeyEvent {
                    code: input.int(),
                    pressed: input.int() != 0,
                });
            }
            let n = input.int();
            for _ in 0..n {
                events.down.push(input.int());
            }
            let expected_len = input.int() as usize;
            let expected = input.bytes(expected_len);
            raw(host.world, 4, 64, 4, if scenario == 12 { 58 } else { 1 });
            raw(host.world, 4, 65, 4, 0);
            host.target = target.then_some(Hit::Block {
                x: 4,
                y: 64,
                z: 4,
                side: 1,
                hit: Vec3D::new(4.0, 65.0, 4.0),
            });
            if !paused {
                host.controller.update_for_mode(creative);
            }
            minecraft_input::run_tick(&mut settings, &mut host, events);
            let policy = minecraft_input::tick_policy(
                &mut join_counter,
                difficulty,
                paused,
                host.multiplayer,
            );
            if policy.join {
                joins += 1;
            }
            if policy.entities {
                if flash > 0 {
                    flash -= 1;
                }
                updates += 1;
            }
            if policy.display {
                display += 1;
            }
            if policy.world {
                mobs = difficulty > 0;
                animals = true;
                world_ticks += 1;
            }
            let mut actual = Vec::new();
            i(&mut actual, host.screen as i32);
            actual.push(host.focused as u8);
            i(&mut actual, *host.left_counter());
            i(&mut actual, *host.right_counter());
            let s = host.player.player_state().unwrap();
            for n in [s.inventory.current_item, host.drops, s.swing_ticks] {
                i(&mut actual, n);
            }
            actual.push(s.is_swinging as u8);
            actual.push(s.item_in_use.is_some() as u8);
            i(&mut actual, s.item_use_ticks);
            for stack in s.inventory.main {
                for n in stack.map_or([-1, 0, 0, 0], |s| [s.id, s.count, s.damage, s.animations]) {
                    i(&mut actual, n);
                }
            }
            for calls in host.clicks {
                i(&mut actual, calls);
            }
            for binding in &settings.key_bindings {
                actual.push(binding.pressed() as u8);
                i(&mut actual, binding.press_count());
            }
            for b in [
                settings.hide_gui,
                settings.show_debug_info,
                settings.third_person_view != 0,
            ] {
                actual.push(b as u8);
            }
            i(&mut actual, settings.render_distance);
            actual.extend(settings.smooth_camera_zoom_amount.to_le_bytes());
            i(&mut actual, host.actions.len() as i32);
            for action in &host.actions {
                i(&mut actual, *action as i32);
            }
            host.actions.clear();
            for n in [
                policy.difficulty,
                flash,
                world_ticks,
                updates,
                joins,
                display,
            ] {
                i(&mut actual, n);
            }
            actual.push(mobs as u8);
            actual.push(animals as u8);
            if creative {
                i(&mut actual, host.controller.block_hit_wait);
            } else {
                for f in [
                    host.controller.cur_block_damage,
                    host.controller.prev_block_damage,
                    host.controller.step_sound_counter,
                ] {
                    actual.extend(f.to_le_bytes());
                }
                for n in [
                    host.controller.target.0,
                    host.controller.target.1,
                    host.controller.target.2,
                    host.controller.block_hit_wait,
                ] {
                    i(&mut actual, n);
                }
            }
            if expected != actual {
                differences += 1;
                if differences < 6 {
                    let at = actual
                        .iter()
                        .zip(&expected)
                        .position(|(a, b)| a != b)
                        .unwrap_or(actual.len().min(expected.len()));
                    let left = *host.left_counter();
                    let right = *host.right_counter();
                    eprintln!("runTick {:?} scenario {scenario} tick {tick}: byte {at}, sizes {}/{}, left/right {}/{}, screen {}",creative,actual.len(),expected.len(),left,right,host.screen);
                    std::fs::write(
                        format!("target/runtick181-{scenario}-{tick}-rust.bin"),
                        &actual,
                    )
                    .unwrap();
                    std::fs::write(
                        format!("target/runtick181-{scenario}-{tick}-java.bin"),
                        &expected,
                    )
                    .unwrap();
                }
            }
            count += 1;
        }
    }
    assert_eq!(input.at, input.bytes.len(), "trailing runTick fixture data");
    println!(
        "MINECRAFT RUNTICK {}: {count} ticks, {differences} differences",
        if differences == 0 { "EXACT" } else { "DIFFERS" }
    );
    differences == 0
}
