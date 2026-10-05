//! Focused original-bytecode replays for the world-blocks port.
use super::{
    behavior,
    chunk::{idx, Chunk},
    RandSel,
};
use crate::{
    entity::AxisAlignedBB as BB,
    nbt::{Compound, Tag},
    storage::LevelInfo,
    World,
};

fn i(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_le_bytes());
}
fn l(o: &mut Vec<u8>, v: i64) {
    o.extend(v.to_le_bytes());
}
fn b(o: &mut Vec<u8>, bb: Option<BB>) {
    o.push(bb.is_some() as u8);
    if let Some(bb) = bb {
        for v in [bb.min_x, bb.min_y, bb.min_z, bb.max_x, bb.max_y, bb.max_z] {
            o.extend(v.to_bits().to_le_bytes());
        }
    }
}
fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8, m: i32) {
    let c = w.chunk_mut(x >> 4, z >> 4);
    c.blocks[idx(x & 15, y, z & 15)] = id;
    c.data.set(x & 15, y, z & 15, m);
}
fn fixture(bright: bool) -> World {
    let mut w = World::new(0, 300);
    for cx in -3..=3 {
        for cz in -3..=3 {
            let mut c = Chunk::new(
                cx,
                cz,
                vec![0; 32768].into_boxed_slice().try_into().unwrap(),
            );
            for x in 0..16 {
                for z in 0..16 {
                    c.blocks[idx(x, 63, z)] = 1;
                    if !bright {
                        c.blocks[idx(x, 80, z)] = 1;
                    }
                }
            }
            c.height_map.fill(if bright { 64 } else { 81 });
            if bright {
                c.sky_light.0.fill(255);
            }
            c.loaded = true;
            c.populated = true;
            w.chunks.insert((cx, cz), Box::new(c));
        }
    }
    w
}
pub fn bytes(kind: &str) -> Vec<u8> {
    let mut o = Vec::new();
    match kind {
        "plants" => plants(&mut o),
        "geometry" => geometry(&mut o),
        "drops" => drops(&mut o),
        "saves" => saves(&mut o),
        "lights" => lights(&mut o),
        "chests" => chests(&mut o),
        "helpers" => helpers(&mut o),
        "fluids" => fluids(&mut o),
        "teleporter" => teleporter(&mut o),
        "items" => items(&mut o),
        "actions" => actions(&mut o),
        "use" => item_use(&mut o),
        "useticks" => item_use_ticks(&mut o),
        "mushrooms" => mushrooms(&mut o),
        "chestlife" => chest_life(&mut o),
        "math" => math_helpers(&mut o),
        "storage" => storage_helpers(&mut o),
        "visual" => block_visual(&mut o),
        "colors" => block_colors(&mut o),
        "spawn" => spawn_cases(&mut o),
        "blocking" => blocking_cases(&mut o),
        "relocation" => relocation_cases(&mut o),
        "foodstats" => foodstats_cases(&mut o),
        "silverfish" => silverfish(&mut o),
        "registry" => o.extend_from_slice(include_bytes!("../entity/player/registry.bin")),
        _ => panic!("unknown worldblocks181 suite {kind}"),
    };
    o
}
fn plants(o: &mut Vec<u8>) {
    for id in [104u8, 105, 106, 39, 40, 18, 60] {
        for config in 0..16 {
            let mut w = fixture(matches!(id, 104 | 105 | 18 | 60));
            w.rand.set_seed(1729 + config as i64 * 31 + id as i64);
            for x in 3..=13 {
                for z in 3..=13 {
                    raw(
                        &mut w,
                        x,
                        63,
                        z,
                        if matches!(id, 104 | 105 | 60) { 60 } else { 1 },
                        config & 1,
                    );
                }
            }
            raw(&mut w, 8, 64, 8, id, config);
            if matches!(id, 104 | 105) {
                if config & 2 != 0 {
                    raw(&mut w, 7, 64, 8, id, 7);
                }
                if config & 4 != 0 {
                    raw(&mut w, 8, 64, 7, id, 7);
                }
                if config & 8 != 0 {
                    raw(&mut w, 9, 64, 9, id, 7);
                }
            }
            if id == 106 {
                for (k, (dx, dz)) in [(0, 1), (-1, 0), (0, -1), (1, 0)].into_iter().enumerate() {
                    if config & (1 << k) != 0 {
                        for y in 62..=68 {
                            raw(&mut w, 8 + dx, y, 8 + dz, 1, 0);
                        }
                    }
                }
                raw(&mut w, 8, 65, 8, if config == 0 { 1 } else { 0 }, 0);
                if config == 15 {
                    for x in 4..=7 {
                        raw(&mut w, x, 64, 4, 106, 1);
                    }
                }
            }
            if matches!(id, 39 | 40) {
                if config & 1 != 0 {
                    for x in 4..=7 {
                        raw(&mut w, x, 64, 4, id, 0);
                    }
                }
                if config & 2 != 0 {
                    raw(&mut w, 9, 64, 8, 1, 0);
                }
            }
            if id == 18 && config & 1 != 0 {
                raw(&mut w, 8, 64, 9, 17, 0);
            }
            if id == 60 {
                raw(&mut w, 8, 63, 8, 1, 0);
                if config < 4 {
                    raw(&mut w, 8, 65, 8, [0, 59, 104, 105][config as usize], 0);
                }
                if config & 4 != 0 {
                    raw(&mut w, 10, 64, 8, 9, 0);
                }
            }
            for _ in 0..128 {
                if w.get_block_id(8, 64, 8) == id {
                    behavior::update_tick(&mut w, id, 8, 64, 8, RandSel::World);
                }
                l(o, w.rand.state());
                for x in 3..=13 {
                    for z in 3..=13 {
                        for y in 62..=68 {
                            o.push(w.get_block_id(x, y, z));
                            o.push(w.get_block_metadata(x, y, z) as u8);
                        }
                    }
                }
            }
        }
    }
}
fn geometry(o: &mut Vec<u8>) {
    let mut w = fixture(false);
    for id in 97..=109 {
        for meta in 0..16 {
            for mask in 0..16 {
                for kind in 0..3 {
                    raw(&mut w, 8, 64, 8, id, meta);
                    for (k, (dx, dz)) in [(0, 1), (-1, 0), (0, -1), (1, 0)].into_iter().enumerate()
                    {
                        raw(
                            &mut w,
                            8 + dx,
                            64,
                            8 + dz,
                            if mask & (1 << k) == 0 {
                                0
                            } else {
                                match kind {
                                    0 => 1,
                                    1 => 20,
                                    _ => id,
                                }
                            },
                            0,
                        );
                    }
                    raw(&mut w, 8, 65, 8, if mask == 0 { 1 } else { 0 }, 0);
                    let boxes = if matches!(id, 101 | 102) {
                        let boxes = w.pane_boxes(id, 8, 64, 8);
                        for bb in &boxes {
                            w.block_bounds[id as usize] = *bb;
                        }
                        boxes
                            .into_iter()
                            .map(|bb| bb.translated(8.0, 64.0, 8.0))
                            .collect()
                    } else {
                        w.get_colliding_bounding_boxes(
                            None,
                            BB::new(8.0, 64.0, 8.0, 9.0, 66.0, 9.0),
                        )
                    };
                    // The query includes the block above. Java calls the target block directly.
                    let boxes: Vec<_> = boxes.into_iter().filter(|bb| bb.min_y < 65.0).collect();
                    i(o, boxes.len() as i32);
                    for bb in boxes {
                        b(o, Some(bb));
                    }
                    b(o, w.block_collision_box(id, 8, 64, 8));
                    w.set_block_bounds_based_on_state(id, 8, 64, 8);
                    b(o, Some(w.block_bounds[id as usize]));
                }
            }
        }
    }
    for id in 0..=109 {
        raw(&mut w, 8, 64, 8, id, 0);
        o.push(w.fence_connects181(8, 64, 8) as u8);
    }
}
fn drops(o: &mut Vec<u8>) {
    let mut w = fixture(false);
    w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
    w.math_random.set_seed(12345);
    for id in [
        53, 67, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
    ] {
        for meta in 0..16 {
            for seed in 0..32 {
                w.loaded_entity_list.clear();
                w.rand.set_seed(seed);
                behavior::drop_block_with_chance(&mut w, id, 8, 64, 8, meta, 0.75);
                l(o, w.rand.state());
                i(o, w.loaded_entity_list.len() as i32);
                for eid in &w.loaded_entity_list {
                    let e = &w.entities[eid];
                    if let crate::entity::EntityKind::Item { item: stack, .. } = &e.kind {
                        i(o, stack.id as i32);
                        i(o, stack.count as i32);
                        i(o, stack.damage as i32);
                        for v in [e.pos.x, e.pos.y, e.pos.z] {
                            o.extend(v.to_bits().to_le_bytes());
                        }
                    } else {
                        panic!("non-item drop");
                    }
                }
            }
        }
    }
}
fn saves(o: &mut Vec<u8>) {
    for game in -1..=2 {
        for features in -1..=1 {
            for player in 0..2 {
                let mut n = Compound::new();
                n.put("RandomSeed", Tag::Long(-4172144997902289642));
                n.put("GameType", Tag::Int(game));
                if features >= 0 {
                    n.put("MapFeatures", Tag::Byte((features != 0) as i8));
                }
                for (key, v) in [
                    ("SpawnX", -22),
                    ("SpawnY", 64),
                    ("SpawnZ", 900),
                    ("version", 19132),
                    ("rainTime", 71),
                    ("thunderTime", 17),
                ] {
                    n.put(key, Tag::Int(v));
                }
                n.put("Time", Tag::Long(23999));
                n.put("SizeOnDisk", Tag::Long(123456789012));
                n.put("raining", Tag::Byte(1));
                if player != 0 {
                    let mut p = Compound::new();
                    p.put("Dimension", Tag::Int(-1));
                    n.put("Player", Tag::Compound(p));
                }
                let info = LevelInfo::from_nbt(&n).clone();
                let encoded = crate::nbt::write_gzip(&info.to_nbt()).unwrap();
                let n = crate::nbt::read_gzip(&encoded).unwrap();
                let info = LevelInfo::from_nbt(&n);
                l(o, info.seed);
                i(o, info.game_type);
                o.push(info.map_features as u8);
                i(o, info.spawn.0);
                i(o, info.spawn.1);
                i(o, info.spawn.2);
                l(o, info.time);
                l(o, info.size_on_disk);
                i(o, info.dimension());
                i(o, n.int("version"));
                i(o, n.int("rainTime"));
                i(o, n.int("thunderTime"));
                o.push(n.boolean("raining") as u8);
                o.push(n.boolean("thundering") as u8);
                o.push(n.has("GameType") as u8);
                o.push(n.has("MapFeatures") as u8);
            }
        }
    }
}
fn lights(o: &mut Vec<u8>) {
    use super::{chunk::LightKind, Dimension};
    let mut w = fixture(false);
    for c in w.chunks.values_mut() {
        c.init_height_and_sky();
    }
    let mut r = crate::JavaRandom::new(1007);
    let ids = [0, 1, 18, 20, 89, 98, 101, 102, 103, 107, 109];
    for t in 0..80 {
        let x = r.next_int(24) - 4;
        let y = 64 + r.next_int(22);
        let z = r.next_int(24) - 4;
        let id = ids[r.next_int(ids.len() as i32) as usize];
        let m = r.next_int(16);
        o.push(w.set_block_and_metadata(x, y, z, id, m) as u8);
        if t % 8 == 0 {
            w.recheck_skylight181(x >> 4, z >> 4);
        }
        for bx in -8..=24 {
            for bz in -8..=24 {
                for by in 59..=88 {
                    o.push(w.get_block_id(bx, by, bz));
                    o.push(w.get_block_metadata(bx, by, bz) as u8);
                    o.push(w.get_saved_light_value(LightKind::Sky, bx, by, bz) as u8);
                    o.push(w.get_saved_light_value(LightKind::Block, bx, by, bz) as u8);
                }
            }
        }
        for cx in -1..=1 {
            for cz in -1..=1 {
                let c = w.chunk(cx, cz);
                o.extend(c.height_map);
                o.extend(c.skylight_occlusion.map(|v| v as u8));
            }
        }
        i(o, w.packed_light181(x, y, z, t % 16));
    }
    for dimension in [Dimension::Overworld, Dimension::Nether] {
        for f in dimension.light_brightness_table() {
            o.extend(f.to_bits().to_le_bytes());
        }
    }
}

fn chests(o: &mut Vec<u8>) {
    for pair in 0..5 {
        for mask in 0..16 {
            for meta in 0..6 {
                for yaw in [-405f32, -90.0, 0.0, 90.0, 180.0, 270.0, 405.0] {
                    let mut w = fixture(true);
                    raw(&mut w, 8, 64, 8, 54, 0);
                    let (dx, dz) = [(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)][pair];
                    if pair != 0 {
                        raw(&mut w, 8 + dx, 64, 8 + dz, 54, meta);
                    }
                    for (k, (sx, sz)) in [(0, -1), (0, 1), (-1, 0), (1, 0)].into_iter().enumerate()
                    {
                        if w.get_block_id(8 + sx, 64, 8 + sz) != 54 {
                            raw(
                                &mut w,
                                8 + sx,
                                64,
                                8 + sz,
                                if mask & (1 << k) != 0 { 1 } else { 0 },
                                0,
                            );
                        }
                    }
                    w.orient_chest181(8, 64, 8);
                    i(o, w.get_block_metadata(8, 64, 8));
                    w.chest_placed181(8, 64, 8, yaw);
                    i(o, w.get_block_metadata(8, 64, 8));
                    i(o, w.get_block_metadata(8 + dx, 64, 8 + dz));
                }
            }
        }
    }
    for pair in 0..5 {
        let mut w = fixture(true);
        w.rand.set_seed(4242);
        w.set_block(8, 64, 8, 54);
        let (dx, dz) = [(0, 0), (0, -1), (0, 1), (-1, 0), (1, 0)][pair];
        if pair != 0 {
            w.set_block(8 + dx, 64, 8 + dz, 54);
        }
        let id = w.chest_id181(8, 64, 8).unwrap();
        for t in 0..160 {
            if [0, 50, 75, 110].contains(&t) {
                w.chest_use181(8, 64, 8, true);
            }
            if [20, 30, 65, 95, 140].contains(&t) {
                w.chest_use181(8, 64, 8, false);
            }
            if t == 35 {
                w.play_note_at(8, 64, 8, 1, 0);
            }
            if t == 100 {
                w.chest_neighbor181(8, 64, 8);
            }
            w.tick_chest181(id);
            let a = &w.chest_animations[&id];
            o.push(a.adjacency_checked as u8);
            for adjacent in a.adjacent {
                o.push(adjacent.is_some() as u8);
            }
            o.extend(a.lid.to_bits().to_le_bytes());
            o.extend(a.previous_lid.to_bits().to_le_bytes());
            i(o, a.users);
            i(o, a.ticks);
            l(o, w.rand.state());
            let sounds: Vec<_> = w
                .drain_events()
                .into_iter()
                .filter(|e| matches!(e, super::events::WorldEvent::Sound { .. }))
                .collect();
            i(o, sounds.len() as i32);
            for e in sounds {
                if let super::events::WorldEvent::Sound {
                    name,
                    position,
                    volume,
                    pitch,
                } = e
                {
                    o.push((name.as_deref() == Some("random.door_open")) as u8);
                    for d in [position.x, position.y, position.z] {
                        o.extend(d.to_bits().to_le_bytes());
                    }
                    o.extend(volume.to_bits().to_le_bytes());
                    o.extend(pitch.to_bits().to_le_bytes());
                }
            }
        }
    }
}

fn helpers(o: &mut Vec<u8>) {
    for v in [
        f64::NAN,
        f64::NEG_INFINITY,
        f64::INFINITY,
        -1e30,
        1e30,
        -9223372036854775808.0,
        9223372036854775808.0,
        -2147483649.0,
        2147483648.0,
        -123.5,
        -1.0,
        -0.1,
        -0.0,
        0.0,
        0.1,
        1.0,
        123.5,
    ] {
        l(o, crate::math::floor_long(v));
        i(o, crate::math::floor_double(v));
        i(o, crate::math::floor_float(v as f32));
    }
    for v in [i32::MIN, i32::MIN + 1, -100, -1, 0, 1, 100, i32::MAX] {
        i(o, crate::math::abs_int(v));
    }
    let mut r = crate::JavaRandom::new(6543);
    for _ in 0..1000 {
        for (min, max) in [
            (-999, 2000),
            (0, 1),
            (1, 1),
            (5, -5),
            (-7, -6),
            (2147483600, 2147483647),
        ] {
            i(o, crate::math::next_int_inclusive(&mut r, min, max));
            l(o, r.state());
        }
    }
    for bits in 4..=8 {
        let mut a = super::chunk::NibbleArray181::new(256 << bits, bits);
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..1 << bits {
                    a.set(x, y, z, x * 100 + z * 10 + y - 200);
                }
            }
        }
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..1 << bits {
                    o.push(a.get(x, y, z) as u8);
                }
            }
        }
        o.extend(&a.data);
        let a = super::chunk::NibbleArray181::from_data(a.data, bits);
        i(o, a.get(15, 0, 15));
    }
    let mut w = fixture(true);
    for id in [0, 1, 8, 18, 30, 78, 101, 102, 104, 106] {
        for y in [0, 1, 62, 64, 127] {
            let c = w.chunk_mut(0, 0);
            for yy in 0..128 {
                c.blocks[idx(8, yy, 8)] = 0;
            }
            c.precipitation_height[136] = -999;
            raw(&mut w, 8, y, 8, id, 0);
            i(o, w.precipitation_height181(8, 8));
            w.set_block(8, y, 8, 0);
            i(o, w.precipitation_height181(8, 8));
            w.set_block(8, 127, 8, 1);
            i(o, w.precipitation_height181(8, 8));
            w.set_block(8, 127, 8, 1);
            i(o, w.precipitation_height181(8, 8));
        }
    }
}

fn fluids(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    let mut r = crate::JavaRandom::new(567);
    for id in [8, 9, 10, 11] {
        for m in 0..16 {
            for above in [0, 1, 8, 79] {
                raw(&mut w, 8, 64, 8, id, m);
                raw(&mut w, 8, 65, 8, above, 0);
                for _ in 0..200 {
                    w.block_display_tick(id, 8, 64, 8, &mut r);
                    l(o, r.state());
                    let events = w.drain_events();
                    i(o, events.len() as i32);
                    for e in events {
                        let (kind, name, p, values) = match e {
                            super::events::WorldEvent::Sound {
                                name,
                                position,
                                volume,
                                pitch,
                            } => (
                                0,
                                name.unwrap(),
                                position,
                                vec![volume as f64, pitch as f64],
                            ),
                            super::events::WorldEvent::Particle {
                                name,
                                position,
                                velocity,
                            } => (1, name, position, vec![velocity.x, velocity.y, velocity.z]),
                            _ => panic!("unexpected display event"),
                        };
                        o.push(kind);
                        i(o, name.len() as i32);
                        o.extend(name.as_bytes());
                        for d in [p.x, p.y, p.z] {
                            o.extend(d.to_bits().to_le_bytes());
                        }
                        for d in values {
                            o.extend(d.to_bits().to_le_bytes());
                        }
                    }
                }
                i(o, w.fluid_packed_light181(8, 64, 8));
                for side in 0..6 {
                    let material = crate::block::material_of(id);
                    o.push(w.fluid_side_solid181(material, 8, 65, 8, side) as u8);
                }
            }
        }
    }
}

fn teleporter(o: &mut Vec<u8>) {
    for config in 0..6 {
        let mut w = fixture(true);
        for cx in -9..=9 {
            for cz in -9..=9 {
                let mut c = Chunk::new(
                    cx,
                    cz,
                    vec![0; 32768].into_boxed_slice().try_into().unwrap(),
                );
                for x in 0..16 {
                    for z in 0..16 {
                        c.blocks[idx(x, 63, z)] = if config == 2 {
                            0
                        } else if config == 3 {
                            9
                        } else {
                            1
                        };
                    }
                }
                c.sky_light.0.fill(255);
                c.height_map.fill(64);
                c.loaded = true;
                c.populated = true;
                w.chunks.insert((cx, cz), Box::new(c));
            }
        }
        w.rand.set_seed(19);
        if config >= 4 {
            for y in 64..=66 {
                for v in 0..2 {
                    raw(
                        &mut w,
                        if config == 4 { 5 + v } else { 5 },
                        y,
                        if config == 4 { 6 } else { 6 + v },
                        90,
                        0,
                    );
                }
            }
        }
        let pos = (8.25, if config == 1 { 120.0 } else { 70.0 }, 8.75);
        let mut r = crate::JavaRandom::new(77 + config);
        let p = w.teleport_position(pos, &mut r).unwrap();
        for d in [p.0, p.1, p.2] {
            o.extend(d.to_bits().to_le_bytes());
        }
        l(o, r.state());
        l(o, w.rand.state());
        for x in -12..=28 {
            for z in -12..=28 {
                for y in 35..=90 {
                    o.push(w.get_block_id(x, y, z));
                    o.push(w.get_block_metadata(x, y, z) as u8);
                }
            }
        }
    }
}

fn items(o: &mut Vec<u8>) {
    use crate::entity::{player::ItemStack, Entity};
    for id in [295, 361, 362] {
        for side in 0..6 {
            for soil in [1, 60] {
                for above in [0, 1] {
                    let mut w = fixture(true);
                    let mut p = Entity::player(&mut w, "oracle");
                    raw(&mut w, 8, 64, 8, soil, 0);
                    raw(&mut w, 8, 65, 8, above, 0);
                    p.player_state_mut().unwrap().inventory.main[0] =
                        Some(ItemStack::new(id, 4, 0));
                    o.push(p.use_on_block(&mut w, 8, 64, 8, side) as u8);
                    i(o, p.held_item().unwrap().count);
                    o.push(w.get_block_id(8, 65, 8));
                    i(o, w.get_block_metadata(8, 65, 8));
                }
            }
        }
    }
    for meta in 0..16 {
        for yaw in [-450f32, -90.0, 0.0, 90.0, 180.0, 270.0, 450.0] {
            let mut w = fixture(true);
            let mut p = Entity::player(&mut w, "oracle");
            p.yaw = yaw;
            raw(&mut w, 8, 64, 8, 107, meta);
            o.push(p.activate_block(&mut w, 8, 64, 8) as u8);
            i(o, w.get_block_metadata(8, 64, 8));
            p.block_placed(&mut w, 107, 8, 64, 8, 1);
            i(o, w.get_block_metadata(8, 64, 8));
        }
    }
    for id in [59, 104, 105] {
        for meta in 0..8 {
            let mut w = fixture(true);
            let mut p = Entity::player(&mut w, "oracle");
            raw(&mut w, 8, 63, 8, 60, 0);
            raw(&mut w, 8, 64, 8, id, meta);
            p.player_state_mut().unwrap().inventory.main[0] = Some(ItemStack::new(351, 4, 15));
            o.push(p.use_on_block(&mut w, 8, 64, 8, 1) as u8);
            i(o, p.held_item().unwrap().count);
            i(o, w.get_block_metadata(8, 64, 8));
        }
    }
    for id in [
        0, 1, 97, 109, 260, 351, 359, 360, 361, 362, 367, 2256, 2257, 2258, 12345, 30000,
    ] {
        for count in [-128, -1, 0, 1, 127] {
            let mut n = Compound::new();
            n.put("id", Tag::Short(id));
            n.put("Count", Tag::Byte(count));
            n.put("Damage", Tag::Short(-123));
            let stack = ItemStack::try_from_nbt181(&n);
            o.push(stack.is_some() as u8);
            if let Some(s) = stack {
                i(o, s.id);
                i(o, s.count);
                i(o, s.damage);
            }
        }
    }
    for id in [18, 106] {
        for meta in 0..16 {
            let mut w = fixture(true);
            let mut p = Entity::player(&mut w, "oracle");
            w.rand.set_seed(42);
            w.math_random.set_seed(123);
            w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(100);
            p.player_state_mut().unwrap().inventory.main[0] = Some(ItemStack::new(359, 1, 0));
            p.harvest_block(&mut w, id, 8, 64, 8, meta);
            l(o, w.rand.state());
            i(o, w.loaded_entity_list.len() as i32);
            for eid in &w.loaded_entity_list {
                let e = &w.entities[eid];
                if let crate::entity::EntityKind::Item { item, .. } = &e.kind {
                    i(o, item.id as i32);
                    i(o, item.count as i32);
                    i(o, item.damage as i32);
                    for d in [e.pos.x, e.pos.y, e.pos.z] {
                        o.extend(d.to_bits().to_le_bytes());
                    }
                }
            }
        }
    }
}

fn actions(o: &mut Vec<u8>) {
    use crate::entity::player::registry;
    for id in 0..=2257 {
        if registry::item(id).is_some() {
            i(o, id);
            i(o, registry::use_action(id) as i32);
            i(o, registry::use_duration(id));
        }
    }
    for elapsed in -20..=200 {
        let charge = registry::bow_charge(72000 - elapsed);
        o.push(charge.is_some() as u8);
        if let Some(f) = charge {
            o.push((f == 1.0) as u8);
        }
    }
}

pub fn regions_write181(path: &std::path::Path) -> Vec<u8> {
    use std::io::Write;
    if path.exists() {
        std::fs::remove_file(path).unwrap();
    }
    let mut region = crate::storage::RegionFile::open(path).unwrap();
    let mut o = Vec::new();
    i(&mut o, region.take_size_delta());
    let mut rng = crate::JavaRandom::new(881);
    for t in 0..36 {
        let len = [0, 1, 4090, 8190, 15000, 65536][t % 6];
        let data: Vec<_> = (0..len).map(|_| rng.next_int(256) as u8).collect();
        let mut encoder =
            flate2::write::ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(&data).unwrap();
        region
            .write_chunk((t % 4) as i32, (t % 3) as i32, &encoder.finish().unwrap())
            .unwrap();
        i(&mut o, region.take_size_delta());
    }
    o
}
pub fn regions_read181(path: &std::path::Path) -> Vec<u8> {
    let mut region = crate::storage::RegionFile::open(path).unwrap();
    let mut o = Vec::new();
    for x in -1..=32 {
        for z in -1..=32 {
            let data = region.read_chunk(x, z);
            i(&mut o, data.as_ref().map_or(-1, |d| d.len() as i32));
            if let Some(data) = data {
                o.extend(data);
            }
        }
    }
    o
}

fn use_state(o:&mut Vec<u8>,p:&crate::entity::Entity) {
    let s=p.player_state().unwrap();o.push(p.is_using_item() as u8);i(o,s.item_use.map_or(0,|u|u.remaining));o.push(p.is_blocking() as u8);
    let stack=p.held_item();o.push(stack.is_some() as u8);if let Some(v)=stack {i(o,v.id);i(o,v.count);i(o,v.damage);}
    i(o,s.food.level);o.extend(s.food.saturation.to_le_bytes());o.extend(s.food.exhaustion.to_le_bytes());i(o,s.food.timer);i(o,p.living.health);
    for id in [10,17] {let e=s.food_effects.iter().find(|e|e.id==id);o.push(e.is_some() as u8);if let Some(e)=e {i(o,e.duration);i(o,e.amplifier);}}
}
fn item_use(o:&mut Vec<u8>) {
    use crate::entity::{Entity,EntityKind,EntitySeedSource,player::ItemStack};
    for id in [260,282,297,319,320,322,349,350,357,360,363,364,365,366,367,267,261] {for level in [0,19,20] {for seed in 0..8 {
        let mut w=fixture(true);let mut p=Entity::player(&mut w,"Player");p.rand.set_seed(123);w.rand.set_seed(seed);w.math_random.set_seed(456);
        p.set_position(8.25,66.0,8.75);p.yaw=31.0;p.pitch=-17.0;
        let s=p.player_state_mut().unwrap();s.food.level=level;s.inventory.main[0]=Some(ItemStack::new(id,3,0));s.inventory.main[1]=Some(ItemStack::new(262,64,0));
        p.use_item(&mut w);use_state(o,&p);
        if id!=267 && id!=261 {p.finish_item_use(&mut w);}else{p.stop_item_use(&mut w);}
        use_state(o,&p);l(o,w.rand.state() as i64);l(o,p.rand.state() as i64);l(o,w.math_random.state() as i64);crate::world::events::write_events(&w.drain_events(),o);
    }}}
    for elapsed in -20..=80 {
        let mut w=fixture(true);w.entity_seed_source=EntitySeedSource::Deterministic(100);let mut p=Entity::player(&mut w,"Player");
        p.set_position(8.25,66.0,8.75);p.yaw=31.0;p.pitch=-17.0;w.item_random.set_seed(123);
        let s=p.player_state_mut().unwrap();s.inventory.main[0]=Some(ItemStack::new(261,1,0));s.inventory.main[1]=Some(ItemStack::new(262,64,0));
        p.start_item_use(p.held_item().unwrap());p.player_state_mut().unwrap().item_use.as_mut().unwrap().remaining=72000-elapsed;p.stop_item_use(&mut w);
        i(o,p.player_state().unwrap().inventory.main[1].unwrap().count);i(o,w.entities.len() as i32);l(o,w.item_random.state() as i64);
        if let Some(e)=w.entities.values().next() {let EntityKind::Arrow(s)=&e.kind else{panic!()};o.push(s.critical as u8);for v in [e.pos.x,e.pos.y,e.pos.z,e.motion.x,e.motion.y,e.motion.z]{o.extend(v.to_le_bytes());}o.extend(e.yaw.to_le_bytes());o.extend(e.pitch.to_le_bytes());l(o,e.rand.state() as i64);}
        crate::world::events::write_events(&w.drain_events(),o);
    }
    for level in [0,6,17,18,20] {for difficulty in 0..=3 {
        let mut w=fixture(true);w.difficulty=difficulty;let mut p=Entity::player(&mut w,"Player");p.living.health=12;
        let s=p.player_state_mut().unwrap();s.food.level=level;s.food.saturation=1.25;
        for t in 0..240 {let s=p.player_state_mut().unwrap();if t%7==0{s.food.exhaust(0.73);}if t%61==0{s.food.add(1,0.3);}p.tick_food(&mut w);use_state(o,&p);}
    }}
}

fn mushrooms(o:&mut Vec<u8>) {
    use crate::entity::{Entity,player::ItemStack};
    for id in [39,40] {for config in 0..10 {for seed in 0..16 {
        let mut w=fixture(true);let y=if config==7 {122}else if config==8 {123}else{64};
        for x in 3..=13 {for z in 3..=13 {raw(&mut w,x,y-1,z,match config {1=>1,2=>60,3=>0,4=>2,_=>3},0);}}
        raw(&mut w,8,y,8,id,5);
        if config==5 {raw(&mut w,9,y+1,8,1,0);}
        if config==6 {for yy in y+1..128.min(y+8) {raw(&mut w,9,yy,8,18,0);}}
        w.rand.set_seed(seed);let mut p=Entity::player(&mut w,"Player");p.player_state_mut().unwrap().inventory.main[0]=Some(ItemStack::new(351,4,15));
        o.push(p.use_on_block(&mut w,8,y,8,1) as u8);i(o,p.held_item().unwrap().count);l(o,w.rand.state() as i64);
        for x in 3..=13 {for z in 3..=13 {for yy in y-1..=127.min(y+9) {o.push(w.get_block_id(x,yy,z));o.push(w.get_block_metadata(x,yy,z) as u8);}}}
    }}}
    for kind in -1..=1 {for seed in 0..32 {
        let mut w=fixture(true);w.rand.set_seed(seed);o.push(super::plants181::big_mushroom(&mut w,8,64,8,kind,RandSel::World) as u8);l(o,w.rand.state() as i64);
        for x in 4..=12 {for z in 4..=12 {for y in 63..=74 {o.push(w.get_block_id(x,y,z));o.push(w.get_block_metadata(x,y,z) as u8);}}}
    }}
}

fn chest_stack(o:&mut Vec<u8>,s:Option<super::tile::ItemStack>) {o.push(s.is_some() as u8);if let Some(s)=s {i(o,s.id as i32);i(o,s.count);i(o,s.damage as i32);}}
fn chest_cache(o:&mut Vec<u8>,w:&mut World,id:u64) {i(o,w.tile_metadata181(id));i(o,w.tile_block181(id).unwrap_or(0) as i32);o.push(w.tile_objects[&id].invalid as u8);o.push(w.chest_animations[&id].adjacency_checked as u8);}
fn chest_life(o:&mut Vec<u8>) {
    use super::tile::{TileEntity,ItemStack};
    let mut w=fixture(true);raw(&mut w,8,64,8,54,3);let id=w.chest_id181(8,64,8).unwrap();let mut r=crate::JavaRandom::new(1729);
    for n in 0..400 {let slot=r.next_int(36) as usize;let op=r.next_int(3);let amount=r.next_int(150)-30;
        if op==0 {w.set_chest_stack181(id,slot,Some(ItemStack::new(if n%2==0 {1}else{260},amount,n)));}
        if op==1 {let s=w.decrement_chest181(id,slot,amount);chest_stack(o,s);}
        for k in 0..36 {chest_stack(o,w.chest_stack181(id,k));}chest_cache(o,&mut w,id);
    }
    let n=crate::storage::chunk_to_nbt(w.chunk(0,0),0);let c=crate::storage::chunk_from_nbt(&n).unwrap();
    let TileEntity::Chest(items)=&c.tile_entities[&(8,64,8)] else{panic!()};i(o,27);for s in items.iter(){chest_stack(o,*s);}o.push((items.len()==27) as u8);
    w.reset_tile_cache181(id);chest_cache(o,&mut w,id);raw(&mut w,8,64,8,1,7);chest_cache(o,&mut w,id);w.tile_inventory_changed181(id);chest_cache(o,&mut w,id);w.reset_tile_cache181(id);chest_cache(o,&mut w,id);
    raw(&mut w,8,64,8,54,2);w.reset_tile_cache181(id);w.check_chest_adjacency181(id);chest_cache(o,&mut w,id);
    w.set_block_metadata(8,64,8,19);chest_cache(o,&mut w,id);w.reset_tile_cache181(id);w.check_chest_adjacency181(id);w.tile_objects.get_mut(&id).unwrap().invalid=true;chest_cache(o,&mut w,id);w.tile_objects.get_mut(&id).unwrap().invalid=false;chest_cache(o,&mut w,id);
}

fn math_helpers(o:&mut Vec<u8>) {
    use crate::math::*;
    fn f(o:&mut Vec<u8>,v:f32){o.extend((if v.is_nan(){0x7fc00000}else{v.to_bits()}).to_le_bytes());}
    fn d(o:&mut Vec<u8>,v:f64){o.extend((if v.is_nan(){0x7ff8000000000000}else{v.to_bits()}).to_le_bytes());}
    let mut r=crate::JavaRandom::new(1729);
    for _ in 0..20000 {
        let a=f64::from_bits(r.next_long() as u64);let b=f32::from_bits(r.next_int_unbounded() as u32);let c=f64::from_bits(r.next_long() as u64);
        for v in [sin(b),cos(b),sqrt_float(b),sqrt_double(a)] {f(o,v);}
        i(o,floor_float(b));i(o,floor_double(a));l(o,floor_long(a));f(o,abs_float(b));d(o,abs_max(a,c));
        let a=r.next_int_unbounded();let mut b=r.next_int(255)-127;if b==0{b=1;}i(o,abs_int(a));i(o,bucket_int(a,b));
    }
    for a in [i32::MIN,-32,-1,0,1,32,i32::MAX] {for b in [-32,-1,1,2,16,32,i32::MAX] {i(o,bucket_int(a,b));}}
    for s in [None,Some(""),Some("a"),Some(" "),Some("\0")] {o.push(string_null_or_empty(s) as u8);}
    for min in -20..=20 {for max in -20..=20 {let mut r=crate::JavaRandom::new((min*31+max) as i64);i(o,next_int_inclusive(&mut r,min,max));l(o,r.state() as i64);}}
}

fn storage_helpers(o:&mut Vec<u8>) {
    use std::{fs,path::Path};use crate::storage181::*;
    fn snap(o:&mut Vec<u8>,p:&Path) {let n=read_uncompressed(p).unwrap();o.push(n.is_some() as u8);if let Some(n)=n {i(o,n.int("value"));l(o,n.long("long"));i(o,n.string("text").len() as i32);let b=n.bytes("bytes").unwrap();i(o,b.len() as i32);o.extend(b);l(o,std::fs::metadata(p).unwrap().len() as i64);}}
    let dir=Path::new("target/wb181-storage-rust");fs::create_dir_all(dir).unwrap();let p=dir.join("plain.dat");let tmp=dir.join("plain.dat_tmp");let _=fs::remove_file(&p);let _=fs::remove_file(&tmp);snap(o,&p);
    for k in 0..32 {let mut n=Compound::new();n.put("value",Tag::Int(k*997-1000));n.put("long",Tag::Long(-123456789012345+k as i64));n.put("text",Tag::String(format!("plain NBT {k}")));n.put("bytes",Tag::ByteArray((0..k*127).map(|j|(j*31+k) as u8).collect()));
        if k%2==0 {write_uncompressed(&p,&n).unwrap();}else{fs::write(&tmp,[]).unwrap();safe_write_uncompressed(&p,&n).unwrap();}snap(o,&p);o.push(tmp.exists() as u8);
    }
    let bad=dir.join("bad.dat");fs::write(&bad,[1,0,0,7]).unwrap();o.push(read_uncompressed(&bad).is_err() as u8);fs::remove_file(bad).unwrap();
    let target=dir.join("nonempty");fs::create_dir_all(&target).unwrap();let child=target.join("child");fs::write(&child,[]).unwrap();o.push(safe_write_uncompressed(&target,&Compound::new()).is_err() as u8);o.push(target.exists() as u8);let tmp2=dir.join("nonempty_tmp");o.push(tmp2.exists() as u8);
    fs::remove_file(child).unwrap();fs::remove_dir(target).unwrap();fs::remove_file(tmp2).unwrap();fs::remove_file(p).unwrap();let _=fs::remove_file(tmp);fs::remove_dir(dir).unwrap();
}
fn silverfish(o:&mut Vec<u8>) {
    use crate::entity::EntitySeedSource;
    for seed in 0..64 {
        let mut w=fixture(true);w.entity_seed_source=EntitySeedSource::Deterministic(seed);w.math_random.set_seed(123);w.silverfish_destroyed181(8,64,8);i(o,w.entities.len() as i32);let e=w.entities.values().next().unwrap();
        for v in [e.pos.x,e.pos.y,e.pos.z] {o.extend(v.to_le_bytes());}o.extend(e.width.to_le_bytes());o.extend(e.height.to_le_bytes());i(o,e.living.health);l(o,e.rand.state() as i64);l(o,w.math_random.state() as i64);
        let events=w.drain_events();i(o,events.len() as i32);for e in events {if let super::events::WorldEvent::Particle{position:p,velocity:v,..}=e {for a in [p.x,p.y,p.z,v.x,v.y,v.z] {o.extend(a.to_le_bytes());}}else{panic!("non-particle")}}
    }
    for id in 0..=109 {o.push(matches!(id,1|4|98) as u8);i(o,match id {4=>1,98=>2,_=>0});}
}

fn item_use_ticks(o:&mut Vec<u8>) {
    use crate::entity::{Entity,player::ItemStack};
    for id in [260,282,322,365,367,267,261] {for op in 0..5 {
        let mut w=fixture(true);let mut p=Entity::player(&mut w,"Player");p.player_state_mut().unwrap().food.level=12;w.rand.set_seed(0);
        let s=p.player_state_mut().unwrap();s.inventory.main[0]=Some(ItemStack::new(id,3,0));s.inventory.main[1]=Some(ItemStack::new(262,64,0));p.use_item(&mut w);
        for tick in 0..40 {
            if tick==8 {let s=p.player_state_mut().unwrap();match op {1=>s.inventory.main[0]=Some(ItemStack::new(id,3,0)),2=>{s.inventory.main[2]=s.inventory.main[0];s.inventory.current_item=2;},4=>s.inventory.main[0].as_mut().unwrap().count=2,_=>{}}if op==3 {p.stop_item_use(&mut w);}}
            p.tick_item_use(&mut w);p.tick_food_effects(&mut w);p.tick_food(&mut w);use_state(o,&p);
        }
    }}
}

fn block_colors(o:&mut Vec<u8>) {
    use crate::block::{props,visual181::*};
    let grass:Vec<_>=(0..65536).map(|k|(k*997)&0xffffff).collect();let foliage:Vec<_>=(0..65536).map(|k|(k*311+0x102030)&0xffffff).collect();
    for id in 1..=109u8 {if !props(id).exists{continue;}for meta in 0..16 {i(o,render_color(id,meta,&grass));
        for x in [-30000001,-17,-1,0,8,17,30000000] {for z in [-17,-1,0,8,17] {i(o,color_multiplier(id,meta as i32,x,64,z,&grass,&foliage,|x,z|((x.wrapping_mul(31).wrapping_add(z.wrapping_mul(17))&15) as f32/15.0,(x.wrapping_mul(7).wrapping_sub(z.wrapping_mul(11))&15) as f32/15.0)));}}
    }}
    let mut w=fixture(true);for id in [104,105] {for meta in 0..16 {for mask in 0..16 {raw(&mut w,8,64,8,id,meta);let fruit=if id==104 {86}else{103};for (k,(dx,dz)) in [(-1,0),(1,0),(0,-1),(0,1)].into_iter().enumerate(){raw(&mut w,8+dx,64,8+dz,if mask&(1<<k)!=0{fruit}else{0},0);}i(o,super::plants181::stem_direction(&mut w,id,8,64,8));}}}
}

fn spawn_grid(p:i32,x:i32,z:i32,w:i32,h:i32)->Vec<i32> {
    (0..w*h).map(|k|match p {0=>0,1=>4,_=>x.wrapping_add(k%w).wrapping_mul(31).wrapping_add(z.wrapping_add(k/w).wrapping_mul(17))&7}).collect()
}
fn spawn_cases(o:&mut Vec<u8>) {
    use super::spawn181::*;use crate::{JavaRandom,block};
    for p in 0..3 {for seed in 0..32 {for radius in [0,1,4,17,256] {
        let x=(-17i32-radius)>>2;let z=(8-radius)>>2;let width=((-17+radius)>>2)-x+1;let height=((8+radius)>>2)-z+1;
        let mut r=JavaRandom::new(seed);let q=biome_position(-17,8,radius,&spawn_grid(p,x,z,width,height),&[4,6,5],&mut r);
        o.push(q.is_some() as u8);if let Some((x,z))=q {i(o,x);i(o,z);}l(o,r.state());
    }}}
    for p in 0..3 {for seed in 0..16 {for mask in [0,15,255,-1] {
        let q=initial_position(seed,&spawn_grid(p,-64,-64,129,129),|x,z|mask!=-1&&(x.wrapping_mul(3).wrapping_add(z.wrapping_mul(7))&mask)==0);
        i(o,q.0);i(o,q.1);i(o,q.2);l(o,JavaRandom::new(123).state());o.push(0);
    }}}
    for dimension in 0..3 {for id in 0..110u8 {o.push(match dimension {0=>id==block::GRASS,1=>id!=0&&id!=7&&block::opaque_cube_lookup(id),_=>id!=0&&block::mat(block::material_of(id)).blocks_movement} as u8);}}
}


fn blocking_cases(o:&mut Vec<u8>) {
    use crate::entity::{Entity,player::ItemStack};
    for armor in 0..4 {for blocking in 0..2 {for bypass in 0..2 {for amount in 0..=40 {
        let mut w=fixture(true);let mut p=Entity::player(&mut w,"Player");let sword=ItemStack::new(267,1,0);p.player_state_mut().unwrap().inventory.main[0]=Some(sword);if blocking!=0{p.start_item_use(sword);}
        if armor!=0{for k in 0..4{p.player_state_mut().unwrap().inventory.armor[k]=Some(ItemStack::new(313-k as i32,1,if armor==1{0}else if armor==2{50}else{500}));}}
        p.damage_living_source181(amount,bypass!=0,if bypass!=0{0.0}else{0.3});i(o,p.living.health);let s=p.player_state().unwrap();i(o,s.damage_remainder);o.extend(s.food.exhaustion.to_bits().to_le_bytes());
        for v in s.inventory.armor {o.push(v.is_some() as u8);if let Some(v)=v {i(o,v.id);i(o,v.count);i(o,v.damage);}}
    }}}}
}

fn relocation_cases(o:&mut Vec<u8>) {
    let mut storage=crate::storage::Storage::open(std::path::Path::new("tools/oracle181/out/wb181-relocation-java")).unwrap();
    for k in 0..10 {let c=storage.load_chunk(k,0);o.push(c.is_some() as u8);if let Some(c)=c {i(o,c.x);i(o,c.z);o.extend(c.blocks.iter());o.extend(c.data.0.iter());o.extend(c.sky_light.0.iter());o.extend(c.block_light.0.iter());o.extend(c.height_map.iter());}}
}

fn foodstats_cases(o:&mut Vec<u8>) {
    use crate::{JavaRandom,entity::player::use181::FoodStats};
    fn state(o:&mut Vec<u8>,f:&FoodStats) {let mut n=Compound::new();f.write(&mut n);i(o,n.int("foodLevel"));i(o,n.int("foodTickTimer"));i(o,f.previous_level);for x in [f.saturation,f.exhaustion] {o.extend(if x.is_nan(){0x7fc00000u32}else{x.to_bits()}.to_le_bytes());}}
    let mut r=JavaRandom::new(123);for k in 0..4096 {let mut f=FoodStats::default();let mut n=Compound::new();let level=r.next_int_unbounded();let timer=r.next_int_unbounded();let sat=f32::from_bits(r.next_int_unbounded() as u32);let ex=f32::from_bits(r.next_int_unbounded() as u32);
        if k&7!=0 {n.put("foodLevel",Tag::Int(level));n.put("foodTickTimer",Tag::Int(timer));n.put("foodSaturationLevel",Tag::Float(sat));n.put("foodExhaustionLevel",Tag::Float(ex));}f.read(&n);state(o,&f);
        for _ in 0..4{let amount=r.next_int_unbounded();let factor=f32::from_bits(r.next_int_unbounded() as u32);let exhaust=f32::from_bits(r.next_int_unbounded() as u32);f.add(amount,factor);f.exhaust(exhaust);state(o,&f);}
    }
}

fn block_visual(o:&mut Vec<u8>) {
    use crate::block::visual181::*;let grass=vec![0;65536];
    for fancy in [false,true] {for id in 0..=255u8 {
        i(o,render_type(id));for side in 0..6{i(o,texture_from_side(id,side,fancy));}
        for meta in 0..16 {i(o,render_color(id,meta,&grass));for side in 0..6{i(o,texture_result(id,side,meta,fancy).unwrap_or(i32::MIN));}}
    }}
    let stacks=creative_stacks();i(o,stacks.len() as i32);for s in stacks {i(o,s.id);i(o,s.count);i(o,s.damage);}
}
