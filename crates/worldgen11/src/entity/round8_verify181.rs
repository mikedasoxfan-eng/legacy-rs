//! Original-bytecode ItemMap lifecycle and lookup matrices.
use crate::{
    entity::{player::ItemStack, Entity},
    maps::{MapData, MapStorage},
    World,
};
fn i(o: &mut Vec<u8>, n: i32) {
    o.extend(n.to_le_bytes());
}
fn text(o: &mut Vec<u8>, s: &str) {
    i(o, s.len() as i32);
    o.extend(s.as_bytes());
}
fn map_state(o: &mut Vec<u8>, m: &MapData, stack: &ItemStack) {
    for n in [
        stack.damage,
        m.center[0],
        m.center[1],
        m.dimension as i32,
        m.scale as i32,
        m.counter,
        m.dirty as i32,
    ] {
        i(o, n);
    }
    o.extend(&m.colors);
    i(o, m.icons.len() as i32);
    for icon in &m.icons {
        o.extend(icon.map(|v| v as u8));
    }
    i(o, m.players.len() as i32);
    for p in &m.players {
        for n in p.min.into_iter().chain(p.max) {
            i(o, n);
        }
    }
}
fn snapshot(o: &mut Vec<u8>, w: &World, stack: &ItemStack) {
    for n in [
        stack.damage,
        stack.count,
        w.map_id,
        w.maps.data.len() as i32,
    ] {
        i(o, n);
    }
    let mut ids: Vec<_> = w.maps.data.keys().copied().collect();
    ids.sort();
    for id in ids {
        i(o, id);
        map_state(o, &w.maps.data[&id], stack);
    }
}
fn reset(w: &mut World, seed: i32) {
    w.maps = MapStorage::default();
    w.maps.counts_loaded = true;
    w.map_id = seed;
}
pub fn itemmap() -> Vec<u8> {
    let mut w = crate::decoration_verify::map_world();
    w.spawn = (57, 64, -93);
    let mut o = Vec::new();
    let mut player = Entity::player(&mut w, "Player");
    let mut pig = Entity::living(&mut w, "Pig").unwrap();
    for seed in [-32768, -1, 0, 32766, 32767] {
        for input in [-32768, -1, 0, 1, 32767, 32768, i32::MIN, i32::MAX] {
            reset(&mut w, seed);
            let mut stack = ItemStack::new(358, -7, input);
            let first = w.get_map_by_id(input as i16);
            i(&mut o, first);
            snapshot(&mut o, &w, &stack);
            let second = w.get_map_by_id(input as i16);
            i(&mut o, (first == second) as i32);
            i(&mut o, second);
            snapshot(&mut o, &w, &stack);
            w.get_map(&mut stack);
            let third = stack.damage;
            i(&mut o, (third == first) as i32);
            snapshot(&mut o, &w, &stack);
            w.get_map(&mut stack);
            i(&mut o, (third == stack.damage) as i32);
            snapshot(&mut o, &w, &stack);
        }
    }
    for dimension in [0, -1] {
        for remote in [false, true] {
            for entity_kind in 0..3 {
                for selected in [false, true] {
                    for damage in [-32768, -1, 32767, i32::MAX] {
                        for existing in [false, true] {
                            reset(&mut w, 32766);
                            w.multiplayer_world = remote;
                            w.dimension = if dimension == 0 {
                                crate::world::Dimension::Overworld
                            } else {
                                crate::world::Dimension::Nether
                            };
                            let mut stack = ItemStack::new(358, -7, damage);
                            player.dead = false;
                            player.player_state_mut().unwrap().dimension = dimension;
                            player.player_state_mut().unwrap().inventory.main[0] = Some(stack);
                            player.player_state_mut().unwrap().inventory.current_item =
                                if selected { 0 } else { 1 };
                            player.set_position(-13.25, 70.0, 19.75);
                            pig.set_position(-13.25, 70.0, 19.75);
                            if existing {
                                w.maps.data.insert(
                                    damage,
                                    MapData {
                                        center: [-13, 19],
                                        scale: 3,
                                        dimension: dimension as i8,
                                        ..Default::default()
                                    },
                                );
                            }
                            let result = if entity_kind == 0 {
                                player.update_maps(&mut w);
                                stack = player.player_state().unwrap().inventory.main[0].unwrap();
                                Ok(())
                            } else {
                                w.try_update_map_item(
                                    &mut stack,
                                    if entity_kind == 1 { Some(&pig) } else { None },
                                    selected,
                                )
                            };
                            text(&mut o, result.err().unwrap_or(""));
                            snapshot(&mut o, &w, &stack);
                        }
                    }
                }
            }
        }
    }
    w.multiplayer_world = false;
    for seed in [-32768, -1, 0, 32766, 32767] {
        reset(&mut w, seed);
        let mut stack = ItemStack::new(358, -7, i32::MAX);
        text(
            &mut o,
            w.try_create_map(&mut stack, None).err().unwrap_or(""),
        );
        snapshot(&mut o, &w, &stack);
    }
    for seed in [-32768, -1, 0, 32766, 32767] {
        for dimension in [0, -1] {
            for x in [-13.25, 0.0, 19.75] {
                reset(&mut w, seed);
                w.dimension = if dimension == 0 {
                    crate::world::Dimension::Overworld
                } else {
                    crate::world::Dimension::Nether
                };
                player.set_position(x, 70.0, -x);
                let mut stack = ItemStack::new(358, -7, i32::MAX);
                w.create_map(&mut stack, &player);
                snapshot(&mut o, &w, &stack);
                w.create_map(&mut stack, &player);
                snapshot(&mut o, &w, &stack);
            }
        }
    }
    o
}
pub fn wolf_despawn() -> Vec<u8> {
    let mut w = super::verify181::world();
    let mut o = Vec::new();
    for flags in [0, 1, 2, 4, 5, 6, 7] {
        for age in [0, 600, 601, 1000] {
            for distance in [0.0, 31.999, 32.0, 127.999, 128.0, 128.001, 1000.0] {
                for present in [false, true] {
                    w.players = if present {
                        vec![(distance, 85.0, 0.0)]
                    } else {
                        Vec::new()
                    };
                    for seed in 0..1600 {
                        let mut e = Entity::living(&mut w, "Wolf").unwrap();
                        e.set_position(0.0, 85.0, 0.0);
                        if let super::EntityKind::Wolf(s) = &mut e.kind {
                            s.flags = flags;
                        }
                        e.living.entity_age = age;
                        e.rand.set_seed(seed);
                        e.despawn(&w);
                        i(&mut o, e.living.entity_age);
                        o.push(e.dead as u8);
                        o.extend(e.rand.state().to_le_bytes());
                    }
                }
            }
        }
    }
    o
}
