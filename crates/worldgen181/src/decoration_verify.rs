//! Original-bytecode replays for decorations and fishing.
use crate::{
    JavaRandom, World,
    entity::{Entity, EntitySeedSource},
    world::{
        chunk::{Chunk, idx},
        tile::TileEntity,
    },
};
fn i(o: &mut Vec<u8>, v: i32) {
    o.extend_from_slice(&v.to_le_bytes());
}
fn text(o: &mut Vec<u8>, s: &str) {
    i(o, s.len() as i32);
    o.extend_from_slice(s.as_bytes());
}
fn painting_state(o: &mut Vec<u8>, p: &Entity, w: &mut World) {
    let crate::entity::EntityKind::Painting(s) = &p.kind else {
        unreachable!()
    };
    for v in [
        s.art as i32,
        s.direction,
        s.tile[0],
        s.tile[1],
        s.tile[2],
        s.check_counter,
    ] {
        i(o, v);
    }
    for v in [
        p.pos.x,
        p.pos.y,
        p.pos.z,
        p.bounding_box.min_x,
        p.bounding_box.min_y,
        p.bounding_box.min_z,
        p.bounding_box.max_x,
        p.bounding_box.max_y,
        p.bounding_box.max_z,
    ] {
        o.extend_from_slice(&v.to_bits().to_le_bytes());
    }
    for v in [p.yaw, p.prev_yaw] {
        o.extend_from_slice(&v.to_bits().to_le_bytes());
    }
    for v in [
        p.dead as i32,
        p.been_attacked as i32,
        p.painting_valid_surface(w) as i32,
    ] {
        i(o, v);
    }
    o.extend_from_slice(&p.rand.state().to_le_bytes());
    i(o, w.loaded_entity_list.len() as i32);
}
fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8) {
    w.chunk_mut(x >> 4, z >> 4).blocks[idx(x & 15, y, z & 15)] = id;
}
fn map_state(o: &mut Vec<u8>, m: &crate::maps::MapData, stack: &crate::entity::player::ItemStack) {
    for v in [
        stack.damage,
        m.center[0],
        m.center[1],
        m.dimension as i32,
        m.scale as i32,
        m.counter,
        m.dirty as i32,
    ] {
        i(o, v);
    }
    o.extend_from_slice(&m.colors);
    i(o, m.icons.len() as i32);
    for icon in &m.icons {
        o.extend(icon.iter().map(|v| *v as u8));
    }
    i(o, m.players.len() as i32);
    for info in &m.players {
        for v in info.min.iter().chain(&info.max) {
            i(o, *v);
        }
    }
}
pub fn map_world() -> World {
    let mut w = flat_world();
    let blocks = [1, 2, 3, 12, 9, 11, 18, 20, 35, 41, 42, 79, 80, 82, 17, 78];
    for cx in -10..=10 {
        for cz in -10..=10 {
            let mut c = Chunk::new(
                cx,
                cz,
                vec![0; 32768].into_boxed_slice().try_into().unwrap(),
            );
            c.loaded = true;
            c.populated = true;
            for x in 0..16 {
                for z in 0..16 {
                    let wx = cx * 16 + x;
                    let wz = cz * 16 + z;
                    let h = 60 + ((wx * 3 + wz * 5) & 7);
                    let id = blocks[((wx / 8 + wz / 8) & 15) as usize];
                    c.blocks[idx(x, 55, z)] = 1;
                    for y in 56..h {
                        c.blocks[idx(x, y, z)] = 3;
                    }
                    c.blocks[idx(x, h, z)] = id;
                    if id == 9 || id == 11 {
                        for y in h - ((wx + wz) & 3)..h {
                            c.blocks[idx(x, y, z)] = id;
                        }
                    }
                    if (wx + wz & 7) == 0 {
                        c.blocks[idx(x, h + 1, z)] = 20;
                    }
                    c.height_map[(z << 4 | x) as usize] = (h + 2) as u8;
                }
            }
            w.chunks.insert((cx, cz), Box::new(c));
        }
    }
    w
}
pub fn maps() -> Vec<u8> {
    use crate::{
        entity::player::ItemStack,
        maps::MapData,
        nbt::{Compound, Tag},
    };
    let mut o = Vec::new();
    for id in 0..256 {
        i(&mut o, crate::maps::block_color(id as u8) as i32);
    }
    let mut w = map_world();
    let mut p = Entity::player(&mut w, "Player");
    for dimension in [0, -1] {
        for scale in 0..5 {
            w.dimension = if dimension == 0 {
                crate::world::Dimension::Overworld
            } else {
                crate::world::Dimension::Nether
            };
            p.player_state_mut().unwrap().dimension = dimension;
            let stack = ItemStack::new(358, 1, 100 + scale);
            p.player_state_mut().unwrap().inventory.main[0] = Some(stack);
            p.set_position(-13.25, 70.0, 19.75);
            let mut m = MapData {
                center: [-13, 19],
                dimension: dimension as i8,
                scale: scale as i8,
                ..Default::default()
            };
            for tick in 0..48 {
                p.yaw = tick as f32 * 37.25 - 400.0;
                p.set_position(
                    -13.25 + (tick % 9 - 4) as f64 * 2.125,
                    70.0,
                    19.75 + (tick % 7 - 3) as f64 * 3.125,
                );
                m.update_markers(&w, &p, &stack);
                if tick % 11 != 0 {
                    m.update_terrain(&mut w, &p);
                }
                map_state(&mut o, &m, &stack);
            }
            let copy = MapData::from_nbt(&m.to_nbt()).unwrap();
            map_state(&mut o, &copy, &stack);
        }
    }
    for size in [16, 64, 128] {
        for scale in [-1, 0, 3, 5] {
            let mut n = Compound::new();
            n.put("dimension", Tag::Byte(-1));
            n.put("xCenter", Tag::Int(-123));
            n.put("zCenter", Tag::Int(256));
            n.put("scale", Tag::Byte(scale));
            n.put("width", Tag::Short(size));
            n.put("height", Tag::Short(size));
            n.put(
                "colors",
                Tag::ByteArray(
                    (0..size as usize * size as usize)
                        .map(|i| (i % 56) as u8)
                        .collect(),
                ),
            );
            let m = MapData::from_nbt(&n).unwrap();
            map_state(&mut o, &m, &ItemStack::new(358, 1, 0));
        }
    }
    let mut markers = MapData {
        scale: 0,
        ..Default::default()
    };
    let held = ItemStack::new(358, 1, 100);
    p.player_state_mut().unwrap().inventory.main[0] = Some(held);
    p.player_state_mut().unwrap().dimension = 0;
    let mut second = Entity::player(&mut w, "Player");
    second.player_state_mut().unwrap().inventory.armor[0] = Some(held);
    second.set_position(-63.75, 70.0, 63.75);
    second.player_state_mut().unwrap().dimension = 0;
    second.yaw = 179.0;
    for t in 0..12 {
        p.set_position(
            if t == 6 {
                64.01
            } else {
                -64.0 + t as f64 * 0.25
            },
            70.0,
            -63.75,
        );
        p.yaw = -370.0 + t as f32 * 31.25;
        w.entities.insert(second.entity_id, second.clone());
        markers.update_markers(&w, &p, &held);
        w.entities.insert(p.entity_id, p.clone());
        markers.update_markers(&w, &second, &held);
        map_state(&mut o, &markers, &held);
        if t == 7 {
            p.dead = true;
        }
        if t == 9 {
            second.player_state_mut().unwrap().inventory.armor[0] = None;
        }
    }
    p.dead = false;
    let mut w = flat_world();
    let mut p = Entity::player(&mut w, "Player");
    p.set_position(-13.25, 70.0, 19.75);
    w.map_id = 32766;
    for c in 0..4 {
        let mut stack = ItemStack::new(358, 1, 32767);
        if c % 2 == 0 {
            w.create_map(&mut stack, &p);
        } else {
            w.get_map(&mut stack);
        }
        map_state(&mut o, &w.maps.data[&stack.damage], &stack);
    }
    o
}
pub fn paintings() -> Vec<u8> {
    let mut o = Vec::new();
    let widths = [8, 1, 2, 1, 2, 4, 4, 4];
    let heights = [8, 1, 1, 2, 2, 2, 3, 4];
    for scene in 0..12 {
        for dir in 0..4 {
            for seed in 0..8 {
                let mut w = flat_world();
                w.entity_seed_source = EntitySeedSource::Deterministic(100 + seed);
                let width = widths[scene % 8];
                let height = heights[scene % 8];
                let material = match scene {
                    8 => 20,
                    9 => 18,
                    10 => 44,
                    11 => 0,
                    _ => 1,
                };
                for a in 0..width {
                    for b in 0..height {
                        let x = if dir == 0 || dir == 2 {
                            a - width / 2
                        } else {
                            0
                        };
                        let z = if dir == 0 || dir == 2 {
                            0
                        } else {
                            a - width / 2
                        };
                        raw(&mut w, x, 67 + b - height / 2, z, material);
                    }
                }
                let mut p = Entity::painting(&mut w, 0, 67, 0, dir);
                painting_state(&mut o, &p, &mut w);
                let n = p.to_nbt().unwrap();
                let q = Entity::from_nbt(&mut w, &n).unwrap();
                painting_state(&mut o, &q, &mut w);
                if scene % 3 == 0 {
                    w.entity_joined_world(p.clone());
                    let overlap = Entity::painting(&mut w, 0, 67, 0, dir);
                    painting_state(&mut o, &overlap, &mut w);
                }
                for y in 64..73 {
                    for a in -4..=4 {
                        raw(
                            &mut w,
                            if dir == 0 || dir == 2 { a } else { 0 },
                            y,
                            if dir == 0 || dir == 2 { 0 } else { a },
                            0,
                        );
                    }
                }
                for _ in 0..103 {
                    p.on_update(&mut w);
                    painting_state(&mut o, &p, &mut w);
                }
                p.attack(&mut w, None, 0);
                painting_state(&mut o, &p, &mut w);
            }
        }
    }
    for art in 0..25 {
        for dir in 0..4 {
            for action in 0..4 {
                let mut w = flat_world();
                let mut p = Entity::painting_base(&mut w);
                if let crate::entity::EntityKind::Painting(s) = &mut p.kind {
                    s.art = if action == 3 { 0 } else { art };
                    s.tile = [0, 67, 0];
                }
                p.set_painting_direction(dir);
                painting_state(&mut o, &p, &mut w);
                for _ in 0..2 {
                    match action {
                        0 => {
                            p.attack(&mut w, None, 0);
                        }
                        1 => p.move_entity(&mut w, 0.125, 0.0, -0.25),
                        2 => p.add_velocity(&mut w, 0.0, 0.25, 0.0),
                        _ => p.move_entity(&mut w, 0.0, 0.0, 0.0),
                    }
                    painting_state(&mut o, &p, &mut w);
                    i(&mut o, w.loaded_entity_list.len() as i32);
                    for id in &w.loaded_entity_list {
                        let e = &w.entities[id];
                        for v in [
                            e.pos.x, e.pos.y, e.pos.z, e.motion.x, e.motion.y, e.motion.z,
                        ] {
                            o.extend(v.to_bits().to_le_bytes());
                        }
                        o.extend(e.rand.state().to_le_bytes());
                    }
                    o.extend(w.math_random.state().to_le_bytes());
                }
            }
        }
    }
    o
}
fn sign(o: &mut Vec<u8>, s: Option<&crate::sign::Sign>) {
    i(o, s.is_some() as i32);
    if let Some(s) = s {
        for s in &s.text {
            text(o, s);
        }
    }
}
pub fn flat_world() -> World {
    let mut w = World::new(0, 300);
    w.entity_seed_source = EntitySeedSource::Deterministic(100);
    w.math_random = JavaRandom::new(200);
    for x in -3..=3 {
        for z in -3..=3 {
            let mut b = vec![0u8; 32768];
            for bx in 0..16 {
                for bz in 0..16 {
                    b[idx(bx, 63, bz)] = 1;
                }
            }
            let mut c = Chunk::new(x, z, b.into_boxed_slice().try_into().unwrap());
            c.populated = true;
            c.loaded = true;
            w.chunks.insert((x, z), Box::new(c));
        }
    }
    w
}
pub fn signs(allowed: &str) -> Vec<u8> {
    let mut w = flat_world();
    let mut p = Entity::player(&mut w, "Player");
    let mut o = Vec::new();
    for support in [1, 20, 18, 44, 85, 0, 9] {
        for side in 0..6 {
            for yaw in [
                -720.0, -181.0, -180.0, -168.75, -0.1, 0.0, 11.25, 179.99, 180.0, 359.99, 720.0,
            ] {
                w.chunk_mut(0, 0).blocks[idx(0, 64, 0)] = support;
                p.yaw = yaw;
                p.player_state_mut().unwrap().events.clear();
                let (x, y, z) = crate::entity::player::placement::offset(0, 64, 0, side);
                if y != 63 {
                    w.set_block(x, y, z, 0);
                }
                let mut stack = crate::entity::player::ItemStack::new(323, 1, 0);
                let used = p.place_sign(&mut w, &mut stack, 0, 64, 0, side);
                for v in [
                    used as i32,
                    stack.count,
                    p.player_state().unwrap().events.len() as i32,
                    w.get_block_id(x, y, z) as i32,
                    w.get_block_metadata(x, y, z),
                ] {
                    i(&mut o, v);
                }
                let s = if matches!(w.get_block_id(x, y, z), 63 | 68) {
                    match w.get_block_tile_entity(x, y, z) {
                        Some(TileEntity::Sign(s)) => Some(s),
                        _ => None,
                    }
                } else {
                    None
                };
                sign(&mut o, s.as_deref());
                w.set_block_with_notify(0, 64, 0, 0);
                i(&mut o, w.get_block_id(x, y, z) as i32);
                o.extend_from_slice(&w.rand.state().to_le_bytes());
                w.set_block(x, y, z, 0);
                if y == 63 {
                    w.chunk_mut(x >> 4, z >> 4).blocks[idx(x & 15, y, z & 15)] = 1;
                }
            }
        }
    }
    let mut editor = crate::sign::Editor::default();
    let mut r = JavaRandom::new(1729);
    for _ in 0..2000 {
        let key = [200, 208, 28, 14, 1, 0, 0, 0, 0][r.next_int(9) as usize];
        let c = char::from_u32(r.next_int(256) as u32).unwrap();
        editor.key_typed(c, key, allowed.contains(c));
        editor.update_counter += 1;
        i(&mut o, editor.edit_line as i32);
        i(&mut o, editor.update_counter);
        sign(&mut o, Some(&editor.sign));
    }
    let mut n = crate::nbt::Compound::new();
    for j in 1..=4 {
        n.put(
            &format!("Text{j}"),
            crate::nbt::Tag::String("01234567890123456789012345".into()),
        );
    }
    sign(&mut o, Some(&crate::sign::Sign::from_nbt(&n)));
    o
}

pub fn map_files(java_dir: &std::path::Path, rust_dir: &std::path::Path) -> Vec<u8> {
    use crate::{
        entity::player::ItemStack,
        maps::MapStorage,
        nbt::{self, Tag},
    };
    let mut store = MapStorage::default();
    let mut o = Vec::new();
    for id in [0, 7, 32767] {
        assert!(store.load(Some(java_dir), id));
        map_state(&mut o, &store.data[&id], &ItemStack::new(358, 1, id));
        store.data.get_mut(&id).unwrap().dirty = true;
    }
    let mut w = flat_world();
    w.storage = Some(crate::storage::Storage::open(rust_dir).unwrap());
    w.maps = store;
    w.save_level();
    assert!(w.maps.data.values().all(|m| !m.dirty));
    let bytes = std::fs::read(java_dir.join("data/idcounts.dat")).unwrap();
    let mut counts = nbt::read_root(&mut &bytes[..]).unwrap();
    // The Java reader already allocated the rollover ID; reproduce that allocation from its predecessor.
    assert_eq!(counts.short("map"), -32768);
    counts.put("map", Tag::Short(32767));
    let mut bytes = Vec::new();
    nbt::write_root(&mut bytes, &counts).unwrap();
    std::fs::write(rust_dir.join("data/idcounts.dat"), bytes).unwrap();
    let p = Entity::player(&mut w, "Player");
    let mut stack = ItemStack::new(358, 1, 0);
    w.create_map(&mut stack, &p);
    i(&mut o, stack.damage);
    assert_eq!(w.map_id, -32768);
    // Rewind just the allocation so the original reader exercises the same transition.
    w.map_id = 32767;
    w.save_map_count();
    o
}

fn fish_state(
    o: &mut Vec<u8>,
    f: &Entity,
    p: &Entity,
    rod: &crate::entity::player::ItemStack,
    w: &mut World,
) {
    use crate::entity::{EntityKind, player::PlayerEvent};
    let EntityKind::Fish(s) = &f.kind else {
        unreachable!()
    };
    for v in [
        f.pos.x,
        f.pos.y,
        f.pos.z,
        f.prev_pos.x,
        f.prev_pos.y,
        f.prev_pos.z,
        f.motion.x,
        f.motion.y,
        f.motion.z,
        f.bounding_box.min_x,
        f.bounding_box.min_y,
        f.bounding_box.min_z,
        f.bounding_box.max_x,
        f.bounding_box.max_y,
        f.bounding_box.max_z,
    ] {
        o.extend(v.to_bits().to_le_bytes());
    }
    for v in [f.yaw, f.pitch, f.prev_yaw, f.prev_pitch, f.fall_distance] {
        o.extend(v.to_bits().to_le_bytes());
    }
    let player = w.entities.get(&p.entity_id).unwrap();
    let ps = player.player_state().unwrap();
    let caught = ps
        .events
        .iter()
        .filter_map(|e| {
            if let PlayerEvent::Stat { id: 2025, amount: count } = e {
                Some(*count)
            } else {
                None
            }
        })
        .sum();
    for v in [
        f.dead as i32,
        f.on_ground as i32,
        f.collided_horizontally as i32,
        f.in_water as i32,
        f.ticks_existed,
        s.arrow.shake,
        s.arrow.ticks_ground,
        s.arrow.ticks_air,
        s.catchable,
        s.arrow.in_ground as i32,
        s.hooked.is_some() as i32,
        ps.fish_entity.is_some() as i32,
        rod.damage,
        rod.count,
        caught,
        ps.events
            .iter()
            .filter_map(|e| {
                if let PlayerEvent::Stat {
                    id: 16974170,
                    amount: count,
                } = e
                {
                    Some(*count)
                } else {
                    None
                }
            })
            .sum(),
    ] {
        i(o, v);
    }
    for v in [f.rand.state(), w.rand.state(), w.math_random.state()] {
        o.extend(v.to_le_bytes());
    }
    let other: Vec<_> = w
        .loaded_entity_list
        .iter()
        .filter_map(|id| w.entities.get(id))
        .filter(|e| e.entity_id != p.entity_id && e.entity_id != f.entity_id)
        .collect();
    i(o, other.len() as i32);
    for e in other {
        let id = if let EntityKind::Item { item, .. } = &e.kind {
            item.id as i32
        } else {
            0
        };
        i(o, id);
        for v in [
            e.pos.x, e.pos.y, e.pos.z, e.motion.x, e.motion.y, e.motion.z,
        ] {
            o.extend(v.to_bits().to_le_bytes());
        }
        o.extend(e.rand.state().to_le_bytes());
    }
    crate::world::events::write_events(&w.drain_events(), o);
}
pub fn fishing() -> Vec<u8> {
    use crate::entity::{EntityKind, Vec3D, player::ItemStack};
    let mut o = Vec::new();
    for scene in 0..10 {
        for seed in 0..3 {
            let mut w = flat_world();
            let mut p = Entity::player(&mut w, "Player");
            p.set_position(0.5, 66.0, 0.5);
            p.yaw = if scene < 3 { scene as f32 * 73.25 } else { 0.0 };
            p.pitch = if scene < 3 { 35.0 } else { 0.0 };
            let mut rod = ItemStack::new(346, 1, if scene == 9 { 64 } else { 0 });
            p.player_state_mut().unwrap().inventory.main[0] = Some(rod);
            if scene < 3 || scene == 6 || scene == 7 {
                for x in -18..=18 {
                    for z in -18..=18 {
                        for y in 64..=65 {
                            raw(&mut w, x, y, z, 9);
                        }
                    }
                }
            }
            if scene == 1 {
                w.info.raining = true;
                w.raining_strength = 1.0;
            }
            w.entity_seed_source = EntitySeedSource::Deterministic(100 + seed);
            w.math_random = JavaRandom::new(200);
            w.item_random = JavaRandom::new(500);
            w.events.clear();
            p.use_fishing_rod(&mut w, &mut rod);
            let mut id = p.player_state().unwrap().fish_entity.unwrap();
            w.entity_joined_world(p.clone());
            let mut f = w.entities.remove(&id).unwrap();
            if scene == 3 || scene == 4 {
                let mut pig = Entity::living(&mut w, "Pig").unwrap();
                pig.set_position(0.5, 65.75, 3.5);
                w.entity_joined_world(pig);
            }
            if scene == 5 || scene == 9 {
                if let EntityKind::Fish(s) = &mut f.kind {
                    s.arrow.in_ground = true;
                    s.arrow.shake = 7;
                }
            }
            if scene == 8 {
                f.set_fish_velocity(Vec3D::new(0.25, -0.125, 0.5));
                f.interpolate_fish(Vec3D::new(-4.0, 68.0, 7.0), 400.0, -30.0, 5);
            }
            fish_state(&mut o, &f, &p, &rod, &mut w);
            for tick in 0..1600 {
                if scene == 6 && tick == 50 {
                    w.entities
                        .get_mut(&p.entity_id)
                        .unwrap()
                        .set_position(40.0, 66.0, 0.5);
                }
                if scene == 7 && tick == 50 {
                    w.entities
                        .get_mut(&p.entity_id)
                        .unwrap()
                        .player_state_mut()
                        .unwrap()
                        .inventory
                        .current_item = 1;
                }
                if scene == 4 && tick == 10 {
                    if let EntityKind::Fish(s) = &f.kind {
                        if let Some(id) = s.hooked {
                            w.entities.get_mut(&id).unwrap().dead = true;
                        }
                    }
                }
                if !f.dead {
                    f.on_update(&mut w);
                }
                let EntityKind::Fish(s) = &f.kind else {
                    unreachable!()
                };
                if !f.dead
                    && ((s.catchable > 0 && tick > 20)
                        || (scene == 3 && s.hooked.is_some())
                        || (scene == 9 && tick == 10))
                {
                    let mut owner = w.entities.remove(&p.entity_id).unwrap();
                    w.entities.insert(id, f);
                    owner.use_fishing_rod(&mut w, &mut rod);
                    f = w.entities.remove(&id).unwrap();
                    w.entities.insert(p.entity_id, owner);
                }
                if scene == 5 && tick == 1200 {
                    w.loaded_entity_list.retain(|v| *v != p.entity_id);
                    w.entities.insert(id, f);
                    w.update_entities();
                    f = w
                        .entities
                        .remove(&id)
                        .expect("angler must retain the expired hook");
                }
                if scene == 5 && (tick == 1250 || tick == 1251) {
                    let mut owner = w.entities.remove(&p.entity_id).unwrap();
                    let previous = f.clone();
                    w.entities.insert(id, f);
                    owner.use_fishing_rod(&mut w, &mut rod);
                    if tick == 1251 {
                        id = owner.player_state().unwrap().fish_entity.unwrap();
                    }
                    f = if tick == 1250 {
                        w.entities.remove(&id).unwrap_or_else(|| {
                            let mut old = previous;
                            old.dead = true;
                            old
                        })
                    } else {
                        w.entities.remove(&id).unwrap()
                    };
                    w.entities.insert(p.entity_id, owner);
                }
                fish_state(&mut o, &f, &p, &rod, &mut w);
            }
        }
    }
    o
}
