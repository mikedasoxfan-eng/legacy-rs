//! Round-seven probes use production callbacks and original-bytecode fixtures.
use super::*;
use crate::container::ItemStack;
use crate::world::tile::TileEntity;

fn stack(o: &mut Vec<u8>, s: Option<ItemStack>) {
    o.push(s.is_some() as u8);
    if let Some(s) = s {
        for n in [s.id, s.count, s.damage] {
            i(o, n);
        }
    }
}
fn furnace_state(o: &mut Vec<u8>, w: &mut World, x: i32, z: i32, old: Option<u64>) {
    i(o, w.get_block_id(x, 64, z) as i32);
    i(o, w.get_block_metadata(x, 64, z));
    let tile = w.get_block_tile_entity(x, 64, z).cloned();
    let current = w
        .chunk(x >> 4, z >> 4)
        .tile_entity_ids
        .get(&(x & 15, 64, z & 15))
        .copied();
    o.push(matches!(tile, Some(TileEntity::Furnace(_))) as u8);
    o.push((current == old) as u8);
    if let Some(TileEntity::Furnace(f)) = tile {
        for s in f.items {
            stack(o, s);
        }
    }
    if let Some(old) = old {
        o.push(w.tile_objects[&old].invalid as u8);
        let Some(TileEntity::Furnace(f)) = w.tile_object(old) else {
            panic!("retained furnace");
        };
        for s in f.items {
            stack(o, s);
        }
        i(o, w.tile_objects[&old].cached_metadata);
        i(o, w.tile_objects[&old].cached_block.unwrap_or(0) as i32);
    }
    i(o, w.loaded_tile_entities.len() as i32);
    i(o, w.pending_tile_entities.len() as i32);
    super::super::events::write_events(&w.drain_events(), o);
}
pub(super) fn furnace(o: &mut Vec<u8>) {
    use crate::entity::Entity;
    for id in [61, 62] {
        for remote in [false, true] {
            for m in 0..16 {
                for mask in 0..16 {
                    let mut w = fixture(true);
                    for (k, (dx, dz)) in [(0, -1), (0, 1), (-1, 0), (1, 0)].into_iter().enumerate()
                    {
                        raw(
                            &mut w,
                            8 + dx,
                            64,
                            8 + dz,
                            if mask & (1 << k) != 0 { 1 } else { 0 },
                            0,
                        );
                    }
                    raw(&mut w, 8, 64, 8, id, m);
                    w.multiplayer_world = remote;
                    behavior::on_block_added(&mut w, id, 8, 64, 8);
                    furnace_state(o, &mut w, 8, 8, None);
                }
            }
        }
    }
    for id in [61, 62] {
        for remote in [false, true] {
            for invalid in [false, true] {
                for hook in 0..2 {
                    let mut w = fixture(true);
                    raw(&mut w, 8, 64, 8, id, 15);
                    w.get_block_tile_entity(8, 64, 8);
                    let old = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
                    w.set_machine_stack181(old, 0, Some(ItemStack::new(15, 32, 2)));
                    if invalid {
                        w.invalidate_tile181(old);
                    }
                    w.multiplayer_world = remote;
                    if hook == 0 {
                        behavior::on_block_added(&mut w, id, 8, 64, 8);
                    } else {
                        let mut p = Entity::player(&mut w, "oracle");
                        o.push(p.activate_block(&mut w, 8, 64, 8) as u8);
                        i(
                            o,
                            p.player_state()
                                .unwrap()
                                .events
                                .iter()
                                .filter(|e| {
                                    matches!(
                                        e,
                                        crate::entity::player::PlayerEvent::OpenContainer { .. }
                                    )
                                })
                                .count() as i32,
                        );
                    }
                    furnace_state(o, &mut w, 8, 8, Some(old));
                }
            }
        }
    }
    for id in [61, 62] {
        for remote in [false, true] {
            for yaw in [
                f32::NEG_INFINITY,
                -f32::MAX,
                -720.0,
                -135.0,
                -45.0,
                -0.0,
                0.0,
                44.999,
                45.0,
                90.0,
                135.0,
                180.0,
                225.0,
                270.0,
                315.0,
                720.0,
                f32::MAX,
                f32::INFINITY,
                f32::NAN,
            ] {
                let mut w = fixture(true);
                let mut p = Entity::player(&mut w, "oracle");
                p.yaw = yaw;
                raw(&mut w, 8, 64, 8, id, 15);
                w.multiplayer_world = remote;
                p.block_placed(&mut w, id, 8, 64, 8, 1);
                furnace_state(o, &mut w, 8, 8, None);
            }
        }
    }
    for id in [61, 62] {
        for m in 0..16 {
            for x in [-30000000, -17, 8, 16777217, 30000000] {
                let mut w = fixture(true);
                w.chunks.insert(
                    (x >> 4, 0),
                    Box::new(Chunk::new(x >> 4, 0, vec![0; 32768].into_boxed_slice())),
                );
                let mut r = crate::JavaRandom::new(567);
                raw(&mut w, x, 64, 8, id, m);
                for _ in 0..8 {
                    w.block_display_tick(id, x, 64, 8, &mut r);
                    l(o, r.state());
                    super::super::events::write_events(&w.drain_events(), o);
                }
            }
        }
    }
    for id in [61, 62] {
        for lit in [false, true] {
            for scanning in [false, true] {
                for remote in [false, true] {
                    for x in [-24, 8, 24] {
                        let mut w = fixture(true);
                        raw(&mut w, x, 64, 8, id, 5);
                        w.get_block_tile_entity(x, 64, 8);
                        let old = w.chunk(x >> 4, 0).tile_entity_ids[&(x & 15, 64, 8)];
                        w.set_machine_stack181(old, 0, Some(ItemStack::new(15, 32, 2)));
                        w.tile_metadata181(old);
                        w.tile_block181(old);
                        w.scanning_tiles = scanning;
                        w.multiplayer_world = remote;
                        w.swap_furnace_block(lit, x, 64, 8);
                        furnace_state(o, &mut w, x, 8, Some(old));
                    }
                }
            }
        }
    }
}

fn machine_snapshot(o: &mut Vec<u8>, w: &World, id: u64) {
    let size = if matches!(w.tile_object(id), Some(TileEntity::Furnace(_))) {
        3
    } else {
        9
    };
    for slot in 0..size {
        stack(o, w.machine_stack181(id, slot));
    }
    i(o, w.tile_objects[&id].cached_metadata);
    i(o, w.tile_objects[&id].cached_block.unwrap_or(0) as i32);
    o.push(w.tile_objects[&id].invalid as u8);
    o.push(w.chunk(0, 0).modified as u8);
}
fn machine_nbt(o: &mut Vec<u8>, w: &World, id: u64) {
    let n = match w.tile_object(id).unwrap() {
        TileEntity::Furnace(f) => f.to_nbt(8, 64, 8),
        TileEntity::Dispenser(items) => {
            crate::container::tiles::inventory_nbt("Trap", 8, 64, 8, items.as_ref())
        }
        _ => unreachable!(),
    };
    let mut bytes = Vec::new();
    crate::nbt::write_root(&mut bytes, &n).unwrap();
    i(o, bytes.len() as i32);
    o.extend(bytes);
}
pub(super) fn inventory(o: &mut Vec<u8>) {
    use crate::{container::tiles::Furnace, entity::Vec3D};
    for block in [61, 23] {
        let mut w = fixture(true);
        raw(&mut w, 8, 64, 8, block, 3);
        w.get_block_tile_entity(8, 64, 8);
        let id = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
        let size = if block == 61 { 3 } else { 9 };
        i(o, size);
        i(o, 64);
        let name = if block == 61 { "Furnace" } else { "Trap" };
        o.extend((name.len() as u16).to_be_bytes());
        o.extend(name.as_bytes());
        let mut r = crate::JavaRandom::new(1729);
        for n in 0..400 {
            let slot = r.next_int(size) as usize;
            let op = r.next_int(4);
            let amount = r.next_int(150) - 30;
            raw(&mut w, 8, 64, 8, block, n & 15);
            w.chunk_mut(0, 0).modified = false;
            match op {
                0 => w.set_machine_stack181(
                    id,
                    slot,
                    Some(ItemStack::new(if n % 2 == 0 { 15 } else { 263 }, amount, n)),
                ),
                1 => {
                    let s = w.decrement_machine181(id, slot, amount);
                    stack(o, s);
                }
                2 => w.set_machine_stack181(id, slot, None),
                _ => w.tile_inventory_changed181(id),
            }
            machine_snapshot(o, &w, id);
        }
        machine_nbt(o, &w, id);
        for c in 0..16 {
            let mut tag = Compound::new();
            tag.put("BurnTime", Tag::Short((c * 5000 - 32768) as i16));
            tag.put("CookTime", Tag::Short((32767 - c * 5000) as i16));
            let mut items = Vec::new();
            for k in 0..20 {
                let mut s = Compound::new();
                s.put("Slot", Tag::Byte((k - 3) as i8));
                s.put(
                    "id",
                    Tag::Short([15, 263, 12345, 500, 0, 5, 327, 351][((k + c) & 7) as usize]),
                );
                s.put("Count", Tag::Byte((c * 17 + k) as i8));
                s.put("Damage", Tag::Short((c * 5000 + k) as i16));
                items.push(Tag::Compound(s));
            }
            tag.put("Items", Tag::List(10, items));
            let te = if block == 61 {
                TileEntity::Furnace(Furnace::from_nbt(&tag))
            } else {
                let mut items = Box::new([None; 9]);
                crate::container::tiles::read_items(&tag, items.as_mut());
                TileEntity::Dispenser(items)
            };
            w.update_tile_object(id, te);
            machine_nbt(o, &w, id);
            machine_snapshot(o, &w, id);
            if let Some(TileEntity::Furnace(f)) = w.tile_object(id) {
                for n in [f.burn, f.current_fuel, f.cook] {
                    i(o, n);
                }
            }
        }
        for replaced in [false, true] {
            for invalid in [false, true] {
                for x in [0.5, 8.5, 16.5, 16.50001, f64::NAN, f64::INFINITY] {
                    w.tile_objects.get_mut(&id).unwrap().invalid = false;
                    w.set_tile_object(id);
                    if replaced {
                        w.set_block_tile_entity(8, 64, 8, TileEntity::for_block(block));
                    }
                    if invalid {
                        w.invalidate_tile181(id);
                    }
                    o.push(w.machine_can_interact181(id, Vec3D::new(x, 64.5, 8.5)) as u8);
                }
            }
        }
    }
    let values = [i32::MIN, -65536, -1, 0, 1, 199, 200, 32767, i32::MAX];
    for burn in values {
        for cook in values {
            for current in values {
                for scale in values {
                    let mut f = Furnace {
                        burn,
                        cook,
                        current_fuel: current,
                        ..Default::default()
                    };
                    i(o, f.cook_scaled(scale));
                    i(o, f.burn_scaled(scale));
                    i(o, f.current_fuel);
                    o.push((f.burn > 0) as u8);
                }
            }
        }
    }
    for id in 0..2258 {
        if crate::entity::player::registry::item(id).is_some() {
            i(
                o,
                crate::container::recipes::CraftingManager::instance()
                    .fuel(Some(ItemStack::new(id, -1, 32767))),
            );
        }
    }
    for remote in [false, true] {
        for count in [i32::MIN, -1, 0, 1, 64, i32::MAX] {
            for cook in values {
                for burn in [0, 1, 100] {
                    for output in [-1, 0, 63, 64, 65] {
                        let mut w = fixture(true);
                        raw(&mut w, 8, 64, 8, 61, 5);
                        w.get_block_tile_entity(8, 64, 8);
                        let id = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
                        let mut f = Furnace {
                            burn,
                            cook,
                            ..Default::default()
                        };
                        f.items[0] = Some(ItemStack::new(15, count, 0));
                        if output != -1 {
                            f.items[2] = Some(ItemStack::new(265, output, 0));
                        }
                        o.push(f.can_smelt() as u8);
                        w.update_tile_object(id, TileEntity::Furnace(f));
                        w.multiplayer_world = remote;
                        w.chunk_mut(0, 0).modified = false;
                        w.tick_tile_entity(id);
                        machine_snapshot(o, &w, id);
                        let Some(TileEntity::Furnace(f)) = w.tile_object(id) else {
                            panic!()
                        };
                        for n in [f.burn, f.current_fuel, f.cook] {
                            i(o, n);
                        }
                    }
                }
            }
        }
    }
}
fn tile_type(tile: Option<&TileEntity>) -> u8 {
    match tile {
        Some(TileEntity::BrewingStand(_))=>9,Some(TileEntity::EnchantmentTable(_))=>10,Some(TileEntity::EndPortal)=>11,
        Some(TileEntity::Dispenser(_)) => 1,
        Some(TileEntity::Note(_)) => 2,
        Some(TileEntity::MobSpawner { .. }) => 3,
        Some(TileEntity::Chest(_)) => 4,
        Some(TileEntity::Furnace(_)) => 5,
        Some(TileEntity::Sign(_)) => 6,
        Some(TileEntity::RecordPlayer { .. }) => 7,
        Some(TileEntity::Piston(_)) => 8,
        None => 0,
    }
}
pub(super) fn container(o: &mut Vec<u8>) {
    for block in [23, 25, 36, 52, 54, 61, 62, 63, 68, 84] {
        o.push(if block == 36 {
            0
        } else {
            tile_type(Some(&TileEntity::for_block(block)))
        });
        for remote in [false, true] {
            for invalid in [false, true] {
                for hook in 0..3 {
                    for event in [i32::MIN, 0, 1, 2, i32::MAX] {
                        for value in [i32::MIN, -1, 0, 1, i32::MAX] {
                            let mut w = fixture(true);
                            raw(&mut w, 8, 64, 8, block, 3);
                            w.get_block_tile_entity(8, 64, 8);
                            let old = w.chunk(0, 0).tile_entity_ids.get(&(8, 64, 8)).copied();
                            if invalid {
                                if let Some(old) = old {
                                    w.invalidate_tile181(old);
                                }
                            }
                            w.multiplayer_world = remote;
                            match hook {
                                0 => w.container_added181(block, 8, 64, 8),
                                1 => w.remove_block_tile_entity(8, 64, 8),
                                _ => w.container_play181(8, 64, 8, event, value),
                            }
                            let tile = w.get_block_tile_entity(8, 64, 8).cloned();
                            let current = w.chunk(0, 0).tile_entity_ids.get(&(8, 64, 8)).copied();
                            o.push(tile_type(tile.as_ref()));
                            o.push((current == old) as u8);
                            if let Some(old) = old {
                                o.push(w.tile_objects[&old].invalid as u8);
                            }
                            i(o, w.loaded_tile_entities.len() as i32);
                            i(
                                o,
                                current
                                    .and_then(|id| w.chest_animations.get(&id).map(|a| a.users))
                                    .unwrap_or(0),
                            );
                            i(
                                o,
                                old.and_then(|id| w.chest_animations.get(&id).map(|a| a.users))
                                    .unwrap_or(0),
                            );
                        }
                    }
                }
            }
        }
    }
}
pub(super) fn ore(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    let mut p = crate::entity::Entity::player(&mut w, "oracle");
    let sides = [
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
        (1, 0, 0),
        (-1, 0, 0),
    ];
    for id in [73, 74] {
        for remote in [false, true] {
            for m in [0, 15] {
                for mask in 0..64 {
                    for hook in 0..5 {
                        for (k, (dx, dy, dz)) in sides.into_iter().enumerate() {
                            raw(
                                &mut w,
                                8 + dx,
                                64 + dy,
                                8 + dz,
                                if mask & (1 << k) != 0 { 1 } else { 0 },
                                0,
                            );
                        }
                        raw(&mut w, 8, 64, 8, id, m);
                        w.multiplayer_world = remote;
                        round6::seed(&mut w);
                        w.loaded_entity_list.clear();
                        w.entities.clear();
                        let mut r = crate::JavaRandom::new(567);
                        match hook {
                            0 => p.block_clicked(&mut w, id, 8, 64, 8),
                            1 => p.on_entity_walking(&mut w, id, 8, 64, 8),
                            2 => o.push(p.activate_block(&mut w, 8, 64, 8) as u8),
                            3 => behavior::update_tick(&mut w, id, 8, 64, 8, RandSel::World),
                            _ => w.block_display_tick(id, 8, 64, 8, &mut r),
                        }
                        l(o, r.state());
                        round6::result(o, &mut w);
                    }
                }
            }
        }
    }
}
pub(super) fn thin(o: &mut Vec<u8>) {
    let mut w = fixture(false);
    for id in [37, 38] {
        for remote in [false, true] {
            for floor in 0..=109 {
                for light in 0..16 {
                    for m in [0, 15] {
                        for hook in 0..2 {
                            raw(&mut w, 8, 63, 8, floor, 0);
                            raw(&mut w, 8, 64, 8, id, m);
                            w.chunk_mut(0, 0).block_light.set(8, 64, 8, light);
                            w.multiplayer_world = remote;
                            w.loaded_entity_list.clear();
                            w.entities.clear();
                            round6::seed(&mut w);
                            o.push(behavior::flower_can_stay(&mut w, id, 8, 64, 8) as u8);
                            if hook == 0 {
                                behavior::on_neighbor_block_change(&mut w, id, 8, 64, 8, 1);
                            } else {
                                behavior::update_tick(&mut w, id, 8, 64, 8, RandSel::World);
                            }
                            round6::result(o, &mut w);
                        }
                    }
                }
            }
        }
    }
    for remote in [false, true] {
        for m in 0..16 {
            for light in 0..16 {
                raw(&mut w, 8, 63, 8, 1, 0);
                raw(&mut w, 8, 64, 8, 80, m);
                w.chunk_mut(0, 0).block_light.set(8, 64, 8, light);
                w.multiplayer_world = remote;
                w.loaded_entity_list.clear();
                w.entities.clear();
                round6::seed(&mut w);
                behavior::update_tick(&mut w, 80, 8, 64, 8, RandSel::World);
                round6::result(o, &mut w);
            }
        }
    }
    for kind in 0..4 {
        for initial in [false, true] {
            for fall in [-1.0, 0.0, 20.0, f32::NAN, f32::INFINITY] {
                let mut e = if kind < 3 {
                    crate::entity::Entity::living(&mut w, ["Pig", "Spider", "CaveSpider"][kind])
                        .unwrap()
                } else {
                    crate::entity::Entity::item(
                        &mut w,
                        8.0,
                        64.0,
                        8.0,
                        crate::world::tile::ItemStack::new(1, 1, 0),
                    )
                };
                e.in_web = initial;
                e.fall_distance = fall;
                e.motion = crate::entity::Vec3D::new(0.125, -0.25, 0.5);
                e.on_entity_collided_with_block(&mut w, 30, 8, 64, 8);
                o.push(e.in_web as u8);
                f181(o, e.fall_distance);
                for v in [e.motion.x, e.motion.y, e.motion.z] {
                    o.extend(v.to_bits().to_le_bytes());
                }
            }
        }
    }
    let mut w = fixture(true);
    raw(&mut w, 8, 64, 8, 25, 0);
    for instrument in [i32::MIN, -1, 0, 1, 2, 3, 4, 5, i32::MAX] {
        for note in [i32::MIN, -1000, -128, -1, 0, 12, 24, 127, 1000, i32::MAX] {
            w.play_note_at(8, 64, 8, instrument, note);
            super::super::events::write_events(&w.drain_events(), o);
        }
    }
}
