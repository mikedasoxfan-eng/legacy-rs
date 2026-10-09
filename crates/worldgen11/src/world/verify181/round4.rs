//! Round-four probes invoke production block/item handlers against flat fixtures.
use super::*;
use crate::entity::{player::ItemStack, Entity, EntityKind};

fn cells(o: &mut Vec<u8>, w: &mut World, lo: i32, hi: i32, bottom: i32, top: i32) {
    for x in lo..=hi {
        for y in bottom..=top {
            for z in lo..=hi {
                o.push(w.get_block_id(x, y, z));
                o.push(w.get_block_metadata(x, y, z) as u8);
            }
        }
    }
}

pub(super) fn placement(o: &mut Vec<u8>) {
    for id in [
        1, 6, 17, 18, 35, 44, 98, 106, 107, 108, 109, 338, 354, 356, 331, 259, 290, 291, 292, 293,
        294, 324, 330, 355, 323, 295, 361, 362, 351,
    ] {
        for side in 0..6 {
            for config in 0..12 {
                let mut w = fixture(true);
                w.rand.set_seed(42);
                w.item_random.set_seed(123);
                w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
                w.math_random.set_seed(456);
                let mut p = Entity::player(&mut w, "oracle");
                p.yaw = -450.0 + 90.0 * config as f32;
                raw(
                    &mut w,
                    8,
                    64,
                    8,
                    [0, 1, 2, 3, 60, 78, 106, 31, 8, 9, 51, 1][config as usize],
                    config & 15,
                );
                if config >= 10 {
                    for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        raw(&mut w, 8 + dx, 64, 8 + dz, 1, 0);
                    }
                }
                let mut stack = ItemStack::new(
                    id,
                    [0, 1, 4, -1][config as usize % 4],
                    if id == 351 { 15 } else { config & 3 },
                );
                o.push(p.item_use_on_block181(&mut w, &mut stack, 8, 64, 8, side) as u8);
                i(o, stack.id);
                i(o, stack.count);
                i(o, stack.damage);
                cells(o, &mut w, 6, 10, 63, 67);
                l(o, w.rand.state());
                l(o, w.item_random.state());
                super::super::events::write_events(&w.drain_events(), o);
            }
        }
    }
}

pub(super) fn fire(o: &mut Vec<u8>) {
    for config in 0..24 {
        let mut w = fixture(true);
        w.rand.set_seed(777 + config);
        w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
        w.math_random.set_seed(123);
        for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            for y in 64..=67 {
                raw(
                    &mut w,
                    8 + dx,
                    y,
                    8 + dz,
                    if config & 1 == 0 { 106 } else { 5 },
                    config as i32 & 15,
                );
            }
        }
        if config & 2 != 0 {
            raw(&mut w, 8, 63, 8, 87, 0);
        }
        raw(&mut w, 8, 64, 8, 51, config as i32 % 16);
        for _ in 0..80 {
            if w.get_block_id(8, 64, 8) == 51 {
                behavior::update_tick(&mut w, 51, 8, 64, 8, RandSel::World);
            }
            cells(o, &mut w, 5, 11, 62, 69);
            l(o, w.rand.state());
            i(o, w.pending_ticks.len() as i32);
        }
    }
}

pub(super) fn harvest(o: &mut Vec<u8>) {
    for id in [18, 31, 106] {
        for meta in 0..16 {
            for tool in [0, 359] {
                let mut w = fixture(true);
                let mut p = Entity::player(&mut w, "oracle");
                w.rand.set_seed(42);
                w.math_random.set_seed(123);
                w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
                if tool != 0 {
                    p.player_state_mut().unwrap().inventory.main[0] =
                        Some(ItemStack::new(tool, 1, 0));
                }
                p.harvest_block(&mut w, id, 8, 64, 8, meta);
                l(o, w.rand.state());
                i(o, w.loaded_entity_list.len() as i32);
                for eid in &w.loaded_entity_list {
                    let e = &w.entities[eid];
                    if let EntityKind::Item { item, .. } = &e.kind {
                        i(o, item.id as i32);
                        i(o, item.count);
                        i(o, item.damage as i32);
                        for d in [e.pos.x, e.pos.y, e.pos.z] {
                            o.extend(d.to_bits().to_le_bytes());
                        }
                    }
                }
            }
        }
    }
}

pub(super) fn sounds(o: &mut Vec<u8>) {
    for id in 1..=109 {
        if crate::block::props(id).exists {
            i(o, id as i32);
            for sound in [
                super::super::events::step_sound(id),
                super::super::events::break_sound(id),
            ] {
                i(o, sound.0.len() as i32);
                o.extend(sound.0.as_bytes());
                o.extend(sound.1.to_bits().to_le_bytes());
                o.extend(sound.2.to_bits().to_le_bytes());
            }
        }
    }
}

pub(super) fn region_cache(o: &mut Vec<u8>) {
    use crate::region_cache181 as cache;
    use std::{io::Write, path::Path};
    let root = Path::new("target/wb181-regioncache4-rust");
    cache::clear();
    let _ = std::fs::remove_dir_all(root);
    let a = root.join("a");
    let b = root.join("b");
    for directory in [&a, &a, &b, &a] {
        i(o, cache::size_delta(directory, 0, 0).unwrap());
    }
    for n in 0i32..36 {
        let (cx, cz) = ((n % 4) * 32 + n % 3, -(n % 3) * 32 + n % 5);
        let directory = if n & 1 == 0 { &a } else { &b };
        let raw: Vec<u8> = (0..(n * 47 + 1)).map(|v| v as u8).collect();
        let mut z = flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        z.write_all(&raw).unwrap();
        cache::write_chunk(directory, cx, cz, &z.finish().unwrap()).unwrap();
        i(o, cache::size_delta(directory, cx, cz).unwrap());
        let bytes = cache::read_chunk(directory, cx, cz).unwrap().unwrap();
        i(o, bytes.len() as i32);
        o.extend(bytes);
        i(o, cache::size_delta(directory, cx, cz).unwrap());
    }
    cache::clear();
    // Open an unconsumed new file first, then force the global 256-entry flush.
    i(o, cache::size_delta(&a, 9000, 0).unwrap());
    cache::with_region(&a, 9000, 0, |_| Ok(())).unwrap();
    for n in 0..256 {
        cache::with_region(&b, n * 32, 4000, |_| Ok(())).unwrap();
    }
    i(o, cache::size_delta(&a, 9000, 0).unwrap());
    i(o, cache::size_delta(&b, 255 * 32, 4000).unwrap());
    cache::clear();
    o.push(std::fs::remove_dir_all(root).is_ok() as u8);
}

pub(super) fn old_format(o: &mut Vec<u8>) {
    use crate::save_format_old181::SaveFormatOld181;
    use std::{fs, path::Path};
    let root = Path::new("target/wb181-oldformat4-rust");
    let _ = fs::remove_dir_all(root);
    let format = SaveFormatOld181::new(root);
    o.push(root.is_dir() as u8);
    i(o, format.name().len() as i32);
    o.extend(format.name().as_bytes());
    for n in 1..=7 {
        let dir = root.join(format!("World{n}"));
        fs::create_dir_all(&dir).unwrap();
        let mut data = Compound::new();
        data.put("RandomSeed", Tag::Long(n));
        data.put("GameType", Tag::Int(n as i32 - 2));
        data.put("LastPlayed", Tag::Long(n * 100));
        data.put("SizeOnDisk", Tag::Long(n * 1000));
        data.put("LevelName", Tag::String(format!("name{n}")));
        let mut tag = Compound::new();
        if n == 3 {
            tag.put("Data", Tag::Int(4));
        } else if n != 2 {
            tag.put("Data", Tag::Compound(data.clone()));
        }
        fs::write(dir.join("level.dat"), crate::nbt::write_gzip(&tag).unwrap()).unwrap();
        if n == 3 {
            let mut old = Compound::new();
            old.put("Data", Tag::Compound(data));
            fs::write(
                dir.join("level.dat_old"),
                crate::nbt::write_gzip(&old).unwrap(),
            )
            .unwrap();
        }
    }
    let worlds = format.worlds();
    i(o, worlds.len() as i32);
    for w in worlds {
        i(o, w.file_name.len() as i32);
        o.extend(w.file_name.as_bytes());
        i(o, w.display_name.len() as i32);
        o.extend(w.display_name.as_bytes());
        l(o, w.last_played);
        l(o, w.last_played);
        i(o, w.game_type);
        o.push(w.requires_conversion as u8);
    }
    for n in 1..=8 {
        let name = format!("World{n}");
        format.rename(&name, "renamed");
        let info = format.world_info(&name);
        o.push(info.is_some() as u8);
        if let Some(info) = info {
            l(o, info.seed);
            i(o, info.game_type);
            i(o, info.name.len() as i32);
            o.extend(info.name.as_bytes());
        }
        o.push(format.is_old_map_format(&name) as u8);
        o.push(format.convert_map_format(&name) as u8);
    }
    for players in [false, true] {
        let name = if players {
            "withplayers"
        } else {
            "withoutplayers"
        };
        let s = format.loader(name, players).unwrap();
        o.push(s.session_ok() as u8);
        o.push(root.join(name).join("players").exists() as u8);
        o.push(root.join(name).join("data").exists() as u8);
    }
    format.flush_cache();
    format.delete("World1");
    o.push(root.join("World1").exists() as u8);
    format.delete("absent");
    format.delete("World3");
    o.push(root.join("World3").exists() as u8);
    fs::remove_dir_all(root).unwrap();
}

pub(super) fn empty_cache(o: &mut Vec<u8>) {
    for sky in [0, 7, 15, 20] {
        let mut w = fixture(true);
        w.chunks.clear();
        w.server = Some(Default::default());
        w.skylight_subtracted = sky;
        let mut c = super::super::cache181::ChunkCache181::new(&mut w, -16, -16, 32, 32);
        for x in [-16, -1, 0, 15, 16, 32] {
            for z in [-16, -1, 0, 15, 16, 32] {
                for y in [-1, 0, 63, 127, 128] {
                    i(o, c.block_id(x, y, z) as i32);
                    i(o, c.metadata(x, y, z));
                    i(o, c.light_value(x, y, z));
                    i(o, c.packed_light(x, y, z, 7));
                    i(o, c.material(x, y, z) as i32);
                    o.push(c.air_block(x, y, z) as u8);
                    o.push(c.normal_cube(x, y, z) as u8);
                    o.push(c.tile_entity(x, y, z).is_some() as u8);
                }
            }
        }
    }
}

pub(super) fn plates(o: &mut Vec<u8>) {
    for id in [70, 72] {
        for floor in 0..=109 {
            for meta in [0, 1] {
                let mut w = fixture(true);
                w.rand.set_seed(42);
                w.math_random.set_seed(123);
                w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
                raw(&mut w, 8, 63, 8, floor, 0);
                raw(&mut w, 8, 64, 8, id, meta);
                o.push(
                    crate::entity::player::placement::can_place_at(&mut w, id, 8, 64, 8, 1) as u8,
                );
                behavior::on_neighbor_block_change(&mut w, id, 8, 64, 8, 1);
                o.push(w.get_block_id(8, 64, 8));
                i(o, w.get_block_metadata(8, 64, 8));
                l(o, w.rand.state());
                i(o, w.loaded_entity_list.len() as i32);
                super::super::events::write_events(&w.drain_events(), o);
            }
        }
    }
}

pub(super) fn shears(o: &mut Vec<u8>) {
    for id in 0..=109 {
        for damage in [0, 237, 238, 239, 32767, -1] {
            let mut w = fixture(true);
            let mut p = Entity::player(&mut w, "oracle");
            p.player_state_mut().unwrap().inventory.main[0] = Some(ItemStack::new(359, 1, damage));
            p.destroy_held_block181(id);
            let s = p.held_item();
            o.push(s.is_some() as u8);
            if let Some(s) = s {
                i(o, s.count);
                i(o, s.damage);
            }
            let stats = &p.player_state().unwrap().stat_log;
            i(o, stats.len() as i32);
            for &(stat, amount) in stats {
                i(o, stat);
                i(o, amount);
            }
        }
    }
}

pub(super) fn placement_predicates(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    for id in [
        1, 6, 31, 32, 37, 38, 39, 40, 53, 54, 64, 67, 70, 71, 72, 81, 83, 86, 91, 98, 99, 100, 101,
        102, 103, 104, 105, 106, 107, 108, 109,
    ] {
        for floor in 0..=109 {
            for target in [0, 1, 8, 9, 10, 11, 51, 78, 106] {
                for side in 0..6 {
                    raw(&mut w, 8, 63, 8, floor, 0);
                    raw(&mut w, 8, 64, 8, target, 0);
                    o.push(
                        crate::entity::player::placement::can_place(&mut w, id, 8, 64, 8, side)
                            as u8,
                    );
                }
            }
        }
    }
}

pub(super) fn growth(o: &mut Vec<u8>) {
    for id in [2, 18, 31, 32, 37, 38, 78, 79, 80, 81, 83] {
        for config in 0..16 {
            let mut w = fixture(config & 2 == 0);
            w.rand.set_seed(901 + config);
            w.math_random.set_seed(123);
            w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
            raw(
                &mut w,
                8,
                63,
                8,
                [1, 2, 3, 12, 60, 0, 79, 81, 83, 1, 2, 3, 12, 60, 0, 1][config as usize],
                0,
            );
            raw(&mut w, 8, 64, 8, id, config as i32);
            if config & 4 != 0 {
                raw(&mut w, 7, 63, 8, 9, 0);
            }
            if config & 8 != 0 {
                raw(&mut w, 9, 64, 8, 17, 0);
            }
            if config & 1 != 0 {
                w.chunk_mut(0, 0).block_light.0.fill(255);
            }
            behavior::on_neighbor_block_change(&mut w, id, 8, 64, 8, 1);
            for _ in 0..48 {
                if w.get_block_id(8, 64, 8) == id {
                    behavior::update_tick(&mut w, id, 8, 64, 8, RandSel::World);
                }
                cells(o, &mut w, 5, 11, 61, 68);
                l(o, w.rand.state());
                i(o, w.loaded_entity_list.len() as i32);
            }
        }
    }
}

pub(super) fn doors(o: &mut Vec<u8>) {
    for id in [64, 71] {
        for dir in 0..4 {
            for mask in 0..81 {
                let mut w = fixture(true);
                let mut p = Entity::player(&mut w, "oracle");
                p.yaw = (dir - 1) as f32 * 90.0;
                w.rand.set_seed(42);
                w.math_random.set_seed(123);
                w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
                let (dx, dz) = [(0, 1), (-1, 0), (0, -1), (1, 0)][dir as usize];
                let mut v = mask;
                for (sx, sy, sz) in [(-dx, 0, -dz), (-dx, 1, -dz), (dx, 0, dz), (dx, 1, dz)] {
                    raw(&mut w, 8 + sx, 64 + sy, 8 + sz, [0, 1, id][v % 3], 0);
                    v /= 3;
                }
                let mut s = ItemStack::new(if id == 64 { 324 } else { 330 }, 4, 0);
                o.push(p.place_door(&mut w, &mut s, 8, 63, 8, 1) as u8);
                i(o, s.count);
                cells(o, &mut w, 6, 10, 63, 66);
                l(o, w.rand.state());
                i(o, w.loaded_entity_list.len() as i32);
            }
        }
    }
}

fn stack(o: &mut Vec<u8>, s: Option<ItemStack>) {
    o.push(s.is_some() as u8);
    if let Some(s) = s {
        i(o, s.id);
        i(o, s.count);
        i(o, s.damage);
    }
}

pub(super) fn furnaces(o: &mut Vec<u8>) {
    use super::super::tile::TileEntity;
    use crate::container::tiles::Furnace;
    for fuel in [0, 5, 6, 17, 18, 99, 100, 103, 107, 263, 327, 368] {
        for input in [12, 15, 319, 363, 365, 1] {
            for config in 0..8 {
                let mut w = fixture(true);
                w.set_block(8, 64, 8, 61);
                let mut f = Furnace::default();
                f.items[0] = Some(ItemStack::new(input, 3, 0));
                if fuel != 0 {
                    f.items[1] = Some(ItemStack::new(fuel, 2, 0));
                }
                if config & 1 != 0 {
                    f.items[2] = crate::container::recipes::CraftingManager::instance()
                        .smelting_result(input)
                        .map(|mut s| {
                            s.count = if config & 2 == 0 { 63 } else { 64 };
                            s
                        });
                }
                f.cook = [0, 190, 199, 200, 0, 190, 199, 32767][config as usize];
                f.burn = [0, 0, 1, -1, 200, 0, 1, -1][config as usize];
                f = Furnace::from_nbt(&f.to_nbt(8, 64, 8));
                w.set_block_tile_entity(8, 64, 8, TileEntity::Furnace(f));
                let tile = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
                for _ in 0..24 {
                    w.chunk_mut(0, 0).modified = false;
                    w.tick_tile_entity(tile);
                    let TileEntity::Furnace(f) = w.tile_object(tile).unwrap() else {
                        panic!("furnace identity")
                    };
                    for s in f.items {
                        stack(o, s);
                    }
                    i(o, f.burn);
                    i(o, f.current_fuel);
                    i(o, f.cook);
                    o.push(w.tile_objects[&tile].invalid as u8);
                    o.push(w.chunk(0, 0).modified as u8);
                    o.push(w.get_block_id(8, 64, 8));
                    i(o, w.get_block_metadata(8, 64, 8));
                }
            }
        }
    }
}

pub(super) fn dispensers(o: &mut Vec<u8>) {
    use super::super::tile::TileEntity;
    for facing in 0..6 {
        for config in 0..16 {
            let mut w = fixture(true);
            w.set_block(8, 64, 8, 23);
            w.set_block_metadata(8, 64, 8, facing);
            w.rand.set_seed(42);
            w.math_random.set_seed(123);
            w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
            let mut items = Box::new([None; 9]);
            for n in 0..9 {
                if (config & (1 << (n % 4))) != 0 {
                    items[n] = Some(ItemStack::new(
                        [262, 344, 332, 4][n % 4],
                        2 + (n % 3) as i32,
                        0,
                    ));
                }
            }
            w.set_block_tile_entity(8, 64, 8, TileEntity::Dispenser(items));
            w.dispenser_random
                .insert((8, 64, 8), crate::JavaRandom::new(55));
            for _ in 0..8 {
                w.dispense_item(8, 64, 8, RandSel::World);
                let Some(TileEntity::Dispenser(items)) = w.get_block_tile_entity(8, 64, 8) else {
                    panic!("dispenser")
                };
                for s in **items {
                    stack(o, s);
                }
                l(o, w.rand.state());
                l(o, w.dispenser_random[&(8, 64, 8)].state());
                i(o, w.loaded_entity_list.len() as i32);
                for eid in &w.loaded_entity_list {
                    let e = &w.entities[eid];
                    i(
                        o,
                        match &e.kind {
                            EntityKind::Arrow(_) => 0,
                            EntityKind::Egg(_) => 1,
                            EntityKind::Snowball(_) => 2,
                            EntityKind::Item { .. } => 3,
                            _ => panic!("dispenser entity"),
                        },
                    );
                    for d in [
                        e.pos.x, e.pos.y, e.pos.z, e.motion.x, e.motion.y, e.motion.z,
                    ] {
                        o.extend(d.to_bits().to_le_bytes());
                    }
                }
                super::super::events::write_events(&w.drain_events(), o);
            }
        }
    }
}

pub(super) fn activation(o: &mut Vec<u8>) {
    for id in [23, 25, 54, 61, 62] {
        for config in 0..4 {
            let mut w = fixture(true);
            let mut p = Entity::player(&mut w, "oracle");
            w.set_block(8, 64, 8, id);
            let tile = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
            if config & 1 != 0 {
                w.tile_objects.get_mut(&tile).unwrap().invalid = true;
            }
            if config & 2 != 0 {
                raw(&mut w, 8, 65, 8, 1, 0);
            }
            w.chunk_mut(0, 0).modified = false;
            w.events.clear();
            w.rand.set_seed(42);
            o.push(p.activate_block(&mut w, 8, 64, 8) as u8);
            i(
                o,
                p.player_state()
                    .unwrap()
                    .events
                    .iter()
                    .filter(|e| {
                        matches!(e, crate::entity::player::PlayerEvent::OpenContainer { .. })
                    })
                    .count() as i32,
            );
            o.push(w.chunk(0, 0).modified as u8);
            l(o, w.rand.state());
            super::super::events::write_events(&w.drain_events(), o);
        }
    }
}

pub(super) fn records(o: &mut Vec<u8>) {
    use super::super::tile::TileEntity;
    for value in [i32::MIN, -1, 0, 1, 2256, 2257, i32::MAX] {
        let mut c = Chunk::new(0, 0, vec![0; 32768].into_boxed_slice());
        c.blocks[idx(8, 64, 8)] = 84;
        c.tile_entities
            .insert((8, 64, 8), TileEntity::RecordPlayer { record: value });
        let n = crate::storage::chunk_to_nbt(&c, 0);
        let Tag::Compound(tile) = &n.list("TileEntities")[0] else {
            panic!("record tag")
        };
        o.push(tile.has("Record") as u8);
        i(o, tile.int("Record"));
        let loaded = crate::storage::chunk_from_nbt(&n).unwrap();
        let TileEntity::RecordPlayer { record } = &loaded.tile_entities[&(8, 64, 8)] else {
            panic!("record factory")
        };
        i(o, *record);
    }
    for record in [2256, 2257] {
        for config in 0..4 {
            let mut w = fixture(true);
            let mut p = Entity::player(&mut w, "oracle");
            w.set_block(8, 64, 8, 84);
            let tile = w.chunk(0, 0).tile_entity_ids[&(8, 64, 8)];
            if config & 1 != 0 {
                w.tile_objects.get_mut(&tile).unwrap().invalid = true;
            }
            if config & 2 != 0 {
                w.set_block_metadata(8, 64, 8, 1);
            }
            w.rand.set_seed(42);
            w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
            w.math_random.set_seed(456);
            w.events.clear();
            let mut stack = ItemStack::new(record, 2, 0);
            o.push(p.item_use_on_block181(&mut w, &mut stack, 8, 64, 8, 1) as u8);
            i(o, stack.count);
            i(o, w.get_block_metadata(8, 64, 8));
            super::super::events::write_events(&w.drain_events(), o);
            w.eject_record(8, 64, 8);
            i(o, w.get_block_metadata(8, 64, 8));
            l(o, w.rand.state());
            i(o, w.loaded_entity_list.len() as i32);
            super::super::events::write_events(&w.drain_events(), o);
        }
    }
}
