//! Deterministic fixtures for Mode_r10world3, compared as complete byte streams.
use crate::{
    hashmap10::*,
    world::{
        cache181::ChunkCache181,
        chunk::{Chunk, LightKind},
        Dimension,
    },
    JavaRandom, World,
};
fn i(o: &mut Vec<u8>, n: i32) {
    o.extend(n.to_be_bytes());
}
fn l(o: &mut Vec<u8>, n: i64) {
    o.extend(n.to_be_bytes());
}
fn b(o: &mut Vec<u8>, n: bool) {
    o.push(n as u8);
}
fn d(o: &mut Vec<u8>, n: f64) {
    o.extend(
        if n.is_nan() {
            0x7ff8000000000000u64
        } else {
            n.to_bits()
        }
        .to_be_bytes(),
    );
}
fn f(o: &mut Vec<u8>, n: f32) {
    o.extend(
        if n.is_nan() {
            0x7fc00000u32
        } else {
            n.to_bits()
        }
        .to_be_bytes(),
    );
}
fn text(o: &mut Vec<u8>, n: &str) {
    crate::nbt::write_utf(o, n).unwrap();
}
fn value(o: &mut Vec<u8>, n: Option<i32>) {
    b(o, n.is_some());
    if let Some(n) = n {
        i(o, n);
    }
}
pub(crate) fn flat(dimension: Dimension) -> World {
    let mut w = World::with_dimension(0, 300, dimension);
    for x in -3..=3 {
        for z in -3..=3 {
            let mut c = Chunk::new(x, z, vec![0; 32768].into_boxed_slice());
            c.loaded = true;
            c.populated = true;
            c.has_no_sky10 = dimension.has_no_sky();
            w.chunks.insert((x, z), Box::new(c));
        }
    }
    w
}
pub fn dump(kind: &str, seed: i64) -> Vec<u8> {
    let mut o = Vec::new();
    match kind {
        "worldheight" => world_height(&mut o),
        "intmap" => int_map(&mut o, seed),
        "longmap" => long_map(&mut o, seed),
        "profiler" => profiler(&mut o),
        "endterrain" => end_terrain(&mut o, seed),
        "endpopulation" | "spikes" => end_population(&mut o, seed, kind),
        "endprovider" => end_provider(&mut o),
        "path" => path(&mut o),
        "biomes" => biomes(&mut o, seed),
        "threaded" => threaded(&mut o),
        "misc" => misc(&mut o, seed),
        "guard" => guard(&mut o),
        name if name.starts_with("village") => village(&mut o, seed, name[7..].parse().unwrap()),
        "fortresslayout" => fortress_layout(&mut o, seed),
        "fortressmap" => fortress_map(&mut o, seed),
        "portal" | "chest" => {
            structure_placement(&mut o, seed, if kind == "portal" { 12 } else { 11 }, true)
        }
        name if name.starts_with("fortress") => {
            structure_placement(&mut o, seed, name[8..].parse().unwrap(), false)
        }
        _ => panic!("unknown round-3 fixture {kind}"),
    }
    o
}
fn world_height(o: &mut Vec<u8>) {
    for dimension in [Dimension::Overworld, Dimension::Nether, Dimension::End] {
        let mut w = flat(dimension);
        w.skylight_subtracted = 7;
        for x in [-1875000, 1874999] {
            w.chunks.insert(
                (x, 0),
                Box::new(Chunk::new(x, 0, vec![0; 32768].into_boxed_slice())),
            );
        }
        for x in 0..16 {
            for z in 0..16 {
                for y in 0..128 {
                    let c = w.chunk_mut(0, 0);
                    c.blocks[crate::world::chunk::idx(x, y, z)] = if y % 7 == 0 { 1 } else { 0 };
                    c.data.set(x, y, z, y % 16);
                    c.sky_light.set(x, y, z, y % 16);
                    c.block_light.set(x, y, z, (y + 3) % 16);
                }
            }
        }
        for height in [128, 64, 32, 1, 0, -1] {
            w.height10 = height;
            i(o, w.height10);
            {
                let c = ChunkCache181::new(&mut w, -16, -16, 31, 31);
                i(o, c.height());
            }
            for y in [-100, -1, 0, 1, 31, 32, 63, 64, 126, 127, 128, 300] {
                i(o, w.get_block_id(8, y, 8) as i32);
                i(o, w.get_block_metadata(8, y, 8));
                b(o, w.block_exists(8, y, 8));
                b(o, w.can_block_see_the_sky(8, y, 8));
                b(o, w.check_chunks_exist(0, y, 0, 15, y, 15));
                i(o, w.get_saved_light_value(LightKind::Sky, 8, y, 8));
                i(o, w.get_saved_light_value(LightKind::Block, 8, y, 8));
                i(o, w.saved_light_with_neighbors181(LightKind::Sky, 8, y, 8));
                i(
                    o,
                    w.saved_light_with_neighbors181(LightKind::Block, 8, y, 8),
                );
                let c = ChunkCache181::new(&mut w, -16, -16, 31, 31);
                i(o, c.block_id(8, y, 8) as i32);
                i(o, c.metadata(8, y, 8));
                i(o, c.light_value(8, y, 8));
                i(o, c.saved_light(LightKind::Sky, 8, y, 8));
                i(o, c.saved_light(LightKind::Block, 8, y, 8));
            }
            for x in [-30000001, -30000000, 29999999, 30000000] {
                i(o, w.get_block_id(x, 64, 8) as i32);
                i(o, w.get_saved_light_value(LightKind::Sky, x, 64, 8));
                b(o, w.block_exists(x, 128, 8));
            }
        }
    }
}
fn int_entry(o: &mut Vec<u8>, e: Option<&IntHashMapEntry<i32>>) {
    b(o, e.is_some());
    let Some(e) = e else { return };
    i(o, e.key);
    value(o, e.value);
    i(o, e.hash);
    i(o, e.hash_code());
    let mut other = e.clone();
    other.next = None;
    b(o, *e == other);
    other.key ^= 1;
    b(o, *e == other);
    text(o, &e.to_string());
}
fn int_snapshot(o: &mut Vec<u8>, m: &IntHashMap<i32>, key: i32) {
    i(o, m.slots.len() as i32);
    i(o, m.count as i32);
    i(o, m.threshold as i32);
    i(o, m.version_stamp);
    let mut keys: Vec<_> = m.keys.iter().collect();
    keys.sort();
    i(o, keys.len() as i32);
    for key in keys {
        i(o, key);
    }
    b(o, m.contains(key));
    value(o, m.lookup(key).copied());
    int_entry(o, m.lookup_entry(key).map(|n| &m.entries[n]));
    for &slot in &m.slots {
        let mut ids = Vec::new();
        let mut next = slot;
        while let Some(n) = next {
            ids.push(n);
            next = m.entries[n].next;
        }
        i(o, ids.len() as i32);
        for n in ids {
            int_entry(o, Some(&m.entries[n]));
        }
    }
}
fn int_map(o: &mut Vec<u8>, seed: i64) {
    let mut r = JavaRandom::new(seed);
    let mut m = IntHashMap::default();
    for n in 0..320 {
        let key = if n < 40 {
            -1
        } else if n < 120 {
            n - 80
        } else {
            r.next_int(64) - 32
        };
        let val = if n % 7 == 0 {
            None
        } else {
            Some(r.next_int_unbounded())
        };
        if n == 240 {
            m.clear_map();
        } else if n < 160 || n % 4 == 0 {
            m.add_key(key, val);
        } else if n % 4 == 1 {
            value(o, m.remove_object(key));
        } else if n % 4 == 2 {
            let id = m.remove_entry(key);
            int_entry(o, id.map(|n| &m.entries[n]));
        }
        int_snapshot(o, &m, key);
    }
    l(o, r.state());
}
fn long_entry(o: &mut Vec<u8>, e: Option<&LongHashMapEntry<i32>>) {
    b(o, e.is_some());
    let Some(e) = e else { return };
    l(o, e.key);
    value(o, e.value);
    i(o, e.hash);
    i(o, e.hash_code());
    let mut other = e.clone();
    other.next = None;
    b(o, *e == other);
    other.key ^= 1;
    b(o, *e == other);
    text(o, &e.to_string());
}
fn long_snapshot(o: &mut Vec<u8>, m: &LongHashMap<i32>, key: i64) {
    i(o, m.slots.len() as i32);
    i(o, m.count as i32);
    i(o, m.threshold as i32);
    i(o, m.version_stamp);
    b(o, m.contains(key));
    value(o, m.lookup(key).copied());
    long_entry(o, m.lookup_entry(key).map(|n| &m.entries[n]));
    for &slot in &m.slots {
        let mut ids = Vec::new();
        let mut next = slot;
        while let Some(n) = next {
            ids.push(n);
            next = m.entries[n].next;
        }
        i(o, ids.len() as i32);
        for n in ids {
            long_entry(o, Some(&m.entries[n]));
        }
    }
}
fn long_map(o: &mut Vec<u8>, seed: i64) {
    let mut r = JavaRandom::new(seed);
    let mut m = LongHashMap::default();
    for n in 0..320 {
        let key = if n < 40 {
            i64::MIN + 123
        } else if n < 120 {
            ((n - 80) as i64).wrapping_shl(32) ^ ((n - 80) as i64)
        } else {
            (r.next_int(64) - 32) as i64
        };
        let val = if n % 7 == 0 {
            None
        } else {
            Some(r.next_int_unbounded())
        };
        if n < 160 || n % 4 == 0 {
            m.add(key, val);
        } else if n % 4 == 1 {
            value(o, m.remove(key));
        } else if n % 4 == 2 {
            let id = m.remove_entry(key);
            long_entry(o, id.map(|n| &m.entries[n]));
        }
        long_snapshot(o, &m, key);
    }
    l(o, r.state());
}
fn end_terrain(o: &mut Vec<u8>, seed: i64) {
    let mut g = crate::end::EndGenerator::new(seed);
    l(o, g.rand_mut().state());
    for [x, z] in [
        [0, 0],
        [-1, 1],
        [3, 4],
        [6, 0],
        [-8, -7],
        [100, 100],
        [784425, -3],
        [i32::MAX, i32::MIN],
    ] {
        let raw = g.generate(x, z);
        let mut c = Chunk::new(x, z, raw.blocks);
        c.init_height_and_sky_for_dimension(Dimension::End);
        o.extend(&*c.blocks);
        o.extend(&*c.height_map);
        o.extend(&*c.sky_light.0);
        let density = g.density(x.wrapping_mul(2), z.wrapping_mul(2));
        i(o, density.len() as i32);
        for n in density {
            d(o, n);
        }
        l(o, g.rand_mut().state());
    }
}
fn end_population(o: &mut Vec<u8>, seed: i64, kind: &str) {
    for fixture in 0..5 {
        let mut w = flat(Dimension::End);
        w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(500);
        w.next_entity_id = 100;
        if fixture != 4 {
            for x in -3..=3 {
                for z in -3..=3 {
                    for bx in 0..16 {
                        for bz in 0..16 {
                            w.chunk_mut(x, z).blocks[crate::world::chunk::idx(bx, 63, bz)] = 121;
                        }
                    }
                }
            }
        }
        if fixture == 1 {
            w.chunk_mut(0, 0).blocks[crate::world::chunk::idx(8, 64, 8)] = 1;
        }
        if fixture == 2 {
            w.chunk_mut(0, 0).blocks[crate::world::chunk::idx(8, 63, 8)] = 1;
        }
        if fixture == 3 {
            w.chunk_mut(0, 0).blocks[crate::world::chunk::idx(5, 63, 8)] = 0;
        }
        w.rand.set_seed(seed);
        let result = if kind == "spikes" {
            w.population_uses_world10 = true;
            crate::end::generate_spike(&mut w, 8, 64, 8)
        } else {
            crate::end::populate(&mut w, if fixture == 4 { 1 } else { 0 }, 0);
            true
        };
        b(o, result);
        b(o, w.sand_fall_instantly);
        l(o, w.rand.state());
        let crate::entity::EntitySeedSource::Deterministic(next_seed) = w.entity_seed_source else {
            unreachable!()
        };
        l(o, next_seed);
        i(o, w.next_entity_id);
        for x in -1..=2 {
            for z in -1..=2 {
                let c = w.chunk(x, z);
                o.extend(&*c.blocks);
                o.extend(&*c.data.0);
            }
        }
        i(o, w.loaded_entity_list.len() as i32);
        for id in &w.loaded_entity_list {
            let e = &w.entities[id];
            i(o, e.entity_id);
            text(
                o,
                if matches!(e.kind, crate::entity::EntityKind::Dragon(_)) {
                    "EnderDragon"
                } else {
                    "EnderCrystal"
                },
            );
            d(o, e.pos.x);
            d(o, e.pos.y);
            d(o, e.pos.z);
            f(o, e.yaw);
            l(o, e.rand.state());
            b(o, e.added_to_chunk);
            i(o, e.chunk_x);
            i(o, e.chunk_y);
            i(o, e.chunk_z);
            if let crate::entity::EntityKind::Dragon(state) = &e.kind {
                i(o, state.parts.len() as i32);
                for id in state.parts {
                    let p = &w.entities[&id];
                    i(o, p.entity_id);
                    l(o, p.rand.state());
                }
            } else {
                i(o, 0);
            }
        }
    }
}
fn end_provider(o: &mut Vec<u8>) {
    let mut w = flat(Dimension::End);
    let p = w.dimension;
    i(o, p.id());
    b(o, p.has_no_sky());
    b(o, p.is_hell_world());
    b(o, p.can_respawn_here());
    b(o, p.is_surface_world());
    f(o, p.cloud_height());
    let (x, y, z) = p.fixed_spawn().unwrap();
    i(o, x);
    i(o, y);
    i(o, z);
    for t in [i64::MIN, -24001, -1, 0, 23999, i64::MAX] {
        for partial in [-1.0, 0.0, 0.25, 1.0, f32::NAN] {
            f(o, p.celestial_angle(t, partial));
            b(o, true);
            let fog = p.end_fog_color(partial).unwrap();
            d(o, fog.x);
            d(o, fog.y);
            d(o, fog.z);
        }
    }
    for id in [0, 1, 2, 8, 9, 18, 49, 78, 121] {
        w.chunk_mut(0, 0).blocks[crate::world::chunk::idx(8, 63, 8)] = id;
        b(o, w.can_coordinate_be_spawn(8, 8));
    }
}
fn profiler(o: &mut Vec<u8>) {
    use crate::profiler::Profiler;
    let names = [
        "root",
        "root.a",
        "root.b",
        "root.deep.x",
        "root..edge",
        "root.😀",
        "root.\u{e000}",
        "zero",
        "zero.a",
    ];
    let times = [
        [100, 20, 20, 9, 3, 2, 2, 0, 0],
        [0; 9],
        [i64::MAX, i64::MAX, -1, 3, 10, 6, 7, -1, -1],
        [10000001, 1, 2, 3, 4, 5, 6, 7, 8],
    ];
    let mut p = Profiler::default();
    b(o, p.results("root").is_none());
    p.start_at("ignored", 0);
    p.end_at(1);
    for row in times {
        p.times.clear();
        for (name, n) in names.iter().zip(row) {
            p.times.insert((*name).into(), n);
        }
        p.enabled = true;
        for section in ["root", "zero", "missing", "", "root.deep"] {
            let result = p.results(section).unwrap();
            i(o, result.len() as i32);
            for r in result {
                text(o, &r.name);
                d(o, r.local_percent);
                d(o, r.root_percent);
                i(o, r.color());
            }
            for name in names {
                l(o, p.times[name]);
            }
        }
    }
    p.clear();
    i(o, p.times.len() as i32);
}
fn bounds(o: &mut Vec<u8>, bounds: crate::structures::BoundingBox) {
    for n in bounds.values() {
        i(o, n);
    }
}
fn guard(o: &mut Vec<u8>) {
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(|_| {}));
    fn rejected(biome: crate::Biome) -> String {
        let e = std::panic::catch_unwind(|| crate::decorator10::begin(biome))
            .expect_err("duplicate decoration");
        if let Some(message) = e.downcast_ref::<String>() {
            message.clone()
        } else {
            e.downcast_ref::<&str>().unwrap().to_string()
        }
    }
    for id in 0..16 {
        let biome = crate::Biome::from_id(id);
        let r = JavaRandom::new(42);
        crate::decorator10::begin(biome);
        text(o, &rejected(biome));
        let result = std::thread::spawn(move || rejected(biome)).join().unwrap();
        text(o, &result);
        b(o, crate::decorator10::active10(biome));
        l(o, r.state());
        text(o, &rejected(biome));
        crate::decorator10::finish(biome);
        b(o, crate::decorator10::active10(biome));
    }
    std::panic::set_hook(hook);
}
fn misc(o: &mut Vec<u8>, seed: i64) {
    use crate::direction10::{Direction as D, Facing as F};
    for row in [
        &D::OFFSETS_X[..],
        &D::OFFSETS_Z,
        &D::HEAD_INVISIBLE_FACE,
        &D::SIDE_TO_DIRECTION,
        &D::FOOT_INVISIBLE_FACE_REMAP,
        &D::CLOCKWISE,
        &D::COUNTERCLOCKWISE,
    ] {
        for n in row {
            i(o, *n);
        }
    }
    for row in D::BED_DIRECTION {
        for n in row {
            i(o, n);
        }
    }
    for row in [F::OPPOSITE, F::OFFSETS_X, F::OFFSETS_Y, F::OFFSETS_Z] {
        for n in row {
            i(o, n);
        }
    }
    for name in ["", "map_0", "\u{1f600}", "\u{e000}"] {
        let mut s = crate::saved_data10::SavedDataState10::new(name.into());
        text(o, s.map_name());
        b(o, s.is_dirty());
        s.mark_dirty();
        b(o, s.is_dirty());
        s.set_dirty(false);
        b(o, s.is_dirty());
        s.set_dirty(true);
        b(o, s.is_dirty());
        for hardcore in [false, true] {
            let v = crate::storage181::SaveFormatComparator181 {
                file_name: name.into(),
                display_name: "display".into(),
                last_played: seed,
                size_on_disk: 0,
                game_type: 3,
                requires_conversion: true,
                hardcore,
            };
            b(o, v.hardcore);
        }
    }
    let mut r = JavaRandom::new(seed);
    for _ in 0..1024 {
        let mut v = [0; 6];
        for n in &mut v {
            *n = r.next_int_unbounded();
        }
        let p = crate::structures::ChunkCoordIntPair { x: v[0], z: v[1] };
        for n in p.center(v[2]) {
            i(o, n);
        }
        for n in crate::structures::BoundingBox::new(v).center() {
            i(o, n);
        }
    }
    l(o, r.state());
}
fn village(o: &mut Vec<u8>, seed: i64, kind: usize) {
    use crate::structures::{village::VillagePiece, BoundingBox as B, Piece};
    const SIZES: [[i32; 3]; 9] = [
        [5, 6, 5],
        [5, 12, 9],
        [9, 9, 6],
        [4, 6, 5],
        [9, 7, 11],
        [13, 4, 9],
        [7, 4, 9],
        [10, 6, 7],
        [9, 7, 12],
    ];
    let mut r = JavaRandom::new(seed);
    for dir in 0..4 {
        let mut w = flat(Dimension::Overworld);
        w.sand_fall_instantly = true;
        w.next_entity_id = 100;
        w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(500);
        for x in -3..=3 {
            for z in -3..=3 {
                let c = w.chunk_mut(x, z);
                for bx in 0..16 {
                    for bz in 0..16 {
                        for y in 0..64 {
                            c.blocks[crate::world::chunk::idx(bx, y, bz)] = if y < 61 {
                                1
                            } else if y < 63 {
                                3
                            } else {
                                2
                            };
                        }
                    }
                }
                c.init_height_and_sky();
            }
        }
        let [sx, sy, sz] = SIZES[kind];
        let bb = B::oriented(0, 50, 0, 0, 0, 0, sx, sy, sz, dir);
        let mut p = VillagePiece::new(kind, 7, bb, dir, &mut r);
        l(o, r.state());
        for clip in [
            B::new([-32, 0, -32, 2, 127, 47]),
            B::new([3, 0, -32, 47, 127, 47]),
            B::new([-32, 0, -32, 47, 127, 47]),
        ] {
            b(o, p.generate(&mut w, &mut r, clip));
            l(o, r.state());
            l(o, w.rand.state());
            i(o, p.villagers10);
            bounds(o, p.component.bounds);
            i(o, w.next_entity_id);
            let crate::entity::EntitySeedSource::Deterministic(next) = w.entity_seed_source else {
                unreachable!()
            };
            l(o, next);
            i(o, w.loaded_entity_list.len() as i32);
            for id in &w.loaded_entity_list {
                let e = &w.entities[id];
                i(o, e.entity_id);
                let crate::entity::EntityKind::Villager { profession } = e.kind else {
                    unreachable!()
                };
                i(o, profession);
                d(o, e.pos.x);
                d(o, e.pos.y);
                d(o, e.pos.z);
                f(o, e.yaw);
                l(o, e.rand.state());
                b(o, e.added_to_chunk);
                i(o, e.chunk_x);
                i(o, e.chunk_y);
                i(o, e.chunk_z);
            }
            for x in -2..=2 {
                for z in -2..=2 {
                    let c = w.chunk(x, z);
                    o.extend(&*c.blocks);
                    o.extend(&*c.data.0);
                }
            }
            crate::structures::verify::tiles(o, &mut w, -32, 47);
        }
    }
}
fn threaded(o: &mut Vec<u8>) {
    use crate::threaded_io10::{IThreadedFileIO, ThreadedChunkLoader, FILE_IO};
    use std::sync::{mpsc, Arc, Mutex};
    struct Barrier {
        started: mpsc::Sender<()>,
        release: Mutex<mpsc::Receiver<()>>,
    }
    impl IThreadedFileIO for Barrier {
        fn write_next(&self) -> bool {
            self.started.send(()).unwrap();
            self.release.lock().unwrap().recv().unwrap();
            false
        }
    }
    FILE_IO.flush();
    let initial = FILE_IO.counters10();
    let (started, wait) = mpsc::channel();
    let (release, blocked) = mpsc::channel();
    let barrier = Arc::new(Barrier {
        started,
        release: Mutex::new(blocked),
    });
    FILE_IO.enqueue(barrier.clone());
    wait.recv().unwrap();
    FILE_IO.enqueue(barrier.clone());
    let dir = std::env::current_dir()
        .unwrap()
        .join("target/r10/world3/threaded-rust");
    std::fs::create_dir_all(&dir).unwrap();
    let loader = ThreadedChunkLoader::new(&dir);
    for (x, z, v) in [
        (1, 0, 10),
        (2, -1, 20),
        (1, 0, 30),
        (-1, i32::MIN, 40),
        (2, -1, 50),
    ] {
        let mut root = crate::nbt::Compound::new();
        root.put("value", crate::nbt::Tag::Int(v));
        loader.save(x, z, root);
        let entries = loader.queued_snapshot10();
        i(o, entries.len() as i32);
        for p in entries {
            i(o, p.x);
            i(o, p.z);
            i(o, p.root.int("value"));
        }
        for (x, z) in [(1, 0), (2, -1), (-1, i32::MIN)] {
            let p = loader.load_pending(x, z);
            b(o, p.is_some());
            if let Some(p) = p {
                i(o, p.int("value"));
            }
        }
        let (queued, complete) = FILE_IO.counters10();
        l(o, queued - initial.0);
        l(o, complete - initial.1);
    }
    for _ in 0..4 {
        b(o, loader.write_next());
        i(o, loader.queued_snapshot10().len() as i32);
    }
    release.send(()).unwrap();
    FILE_IO.flush();
    let (queued, complete) = FILE_IO.counters10();
    l(o, queued - initial.0);
    l(o, complete - initial.1);
    for (x, z) in [(1, 0), (2, -1), (-1, i32::MIN)] {
        let bytes = crate::region_cache181::read_chunk(&dir, x, z)
            .unwrap()
            .unwrap();
        let root = crate::nbt::read_root(&mut &bytes[..]).unwrap();
        i(o, root.int("value"));
        b(o, loader.load_pending(x, z).is_some());
    }
}
fn biomes(o: &mut Vec<u8>, seed: i64) {
    let mut w = flat(Dimension::Overworld);
    w.rand.set_seed(seed);
    w.population_uses_world10 = true;
    for id in 0..16 {
        let biome = crate::Biome::from_id(id);
        text(o, biome.name());
        i(o, biome.color());
        i(o, biome.foliage_color());
        o.push(biome.top_block());
        o.push(biome.filler_block());
        for n in [
            biome.heights().0,
            biome.heights().1,
            biome.temperature(),
            biome.rainfall(),
        ] {
            f(o, n);
        }
        b(o, biome.enable_snow());
        b(o, biome.enable_rain());
        i(o, biome.water_color10());
        i(o, (biome.temperature() * 65536.0) as i32);
        i(o, (biome.rainfall() * 65536.0) as i32);
        f(o, 0.1);
        let s = crate::decorator10::DecoratorSettings::for_biome(biome);
        for n in [
            s.trees,
            s.flowers,
            s.grass,
            s.dead_bush,
            s.mushrooms,
            s.reeds,
            s.cacti,
            s.waterlilies,
            s.big_mushrooms,
            s.sand,
            s.sand2,
            s.clay,
        ] {
            i(o, n);
        }
        b(o, s.generate_fluid);
        for list in [
            crate::weighted::creatures(biome),
            crate::weighted::monsters(biome),
            crate::weighted::water_creatures(biome),
        ] {
            i(o, list.len() as i32);
            for e in list {
                text(
                    o,
                    match e.entity {
                        "LavaSlime" => "MagmaCube",
                        "MushroomCow" => "Mooshroom",
                        name => name,
                    },
                );
                for n in [e.weight, e.min, e.max] {
                    i(o, n);
                }
            }
        }
        for _ in 0..128 {
            text(
                o,
                match crate::populate::tree_for_biome(&mut w, biome) {
                    crate::features::TreeKind::Oak => "WorldGenTrees",
                    crate::features::TreeKind::BigOak => "WorldGenBigTree",
                    crate::features::TreeKind::Birch => "WorldGenForest",
                    crate::features::TreeKind::Spruce1 => "WorldGenTaiga1",
                    crate::features::TreeKind::Spruce2 => "WorldGenTaiga2",
                    crate::features::TreeKind::Swamp => "WorldGenSwamp",
                },
            );
            l(o, w.rand.state());
        }
    }
}
fn path(o: &mut Vec<u8>) {
    for (scenario, wall) in [0, 1, 8, 10, 64, 71, 53, 85, 9].into_iter().enumerate() {
        let mut w = flat(Dimension::Overworld);
        for x in -48..64 {
            for z in -48..64 {
                w.chunk_mut(x >> 4, z >> 4).blocks[crate::world::chunk::idx(x & 15, 63, z & 15)] =
                    1;
            }
        }
        for z in -12..=12 {
            if z != 2 {
                for y in 64..=66 {
                    let c = w.chunk_mut(0, z >> 4);
                    c.blocks[crate::world::chunk::idx(0, y, z & 15)] = wall;
                    c.data
                        .set(0, y, z & 15, if scenario % 2 == 0 { 4 } else { 0 });
                }
            }
        }
        if scenario == 6 {
            for x in 1..=12 {
                for z in -5..=5 {
                    w.chunk_mut(x >> 4, z >> 4).blocks
                        [crate::world::chunk::idx(x & 15, 64, z & 15)] = 1;
                }
            }
        }
        if scenario == 7 {
            for x in -10..=10 {
                for z in -10..=10 {
                    w.chunk_mut(x >> 4, z >> 4).blocks
                        [crate::world::chunk::idx(x & 15, 67, z & 15)] = 1;
                }
            }
        }
        for width in [0.6, 0.9, 1.4, 2.0] {
            for start in [-8, -3, 0, 8] {
                for target in [-12, 5, 12] {
                    let mut e = crate::entity::Entity::item(
                        &mut w,
                        0.0,
                        0.0,
                        0.0,
                        crate::world::tile::ItemStack::new(1, 1, 0),
                    );
                    e.width = width;
                    e.height = 1.8;
                    e.y_offset = 0.0;
                    e.set_position(start as f64 + 0.5, 64.0, 0.5);
                    let p = w.entity_path_to_xyz(&e, target, 64, 4, 16.0);
                    b(o, p.is_some());
                    if let Some(mut p) = p {
                        i(o, p.points.len() as i32);
                        for q in &p.points {
                            for n in q {
                                i(o, *n);
                            }
                        }
                        while let Some(v) = p.position(e.width) {
                            d(o, v.x);
                            d(o, v.y);
                            d(o, v.z);
                            p.index += 1;
                        }
                    }
                }
            }
        }
        l(o, w.rand.state());
    }
}
fn fortress_piece(o: &mut Vec<u8>, p: &crate::structures::nether10::NetherPiece) {
    i(o, p.kind as i32);
    i(o, p.component.orientation);
    i(o, p.component.depth);
    bounds(o, p.component.bounds);
    i(o, p.end_seed);
    b(o, p.has_spawner());
}
fn fortress_layout(o: &mut Vec<u8>, seed: i64) {
    let mut r = JavaRandom::new(seed);
    for j in 0..16 {
        let s = crate::structures::nether10::NetherStart::new(&mut r, j % 4 - 2, j / 4 - 2);
        bounds(o, s.bounds);
        i(o, s.pieces.len() as i32);
        for p in &s.pieces {
            fortress_piece(o, p);
        }
        l(o, r.state());
    }
}
fn fortress_map(o: &mut Vec<u8>, seed: i64) {
    let mut g = crate::structures::nether10::NetherBridgeGenerator::default();
    for [x, z] in [
        [0, 0],
        [4, 4],
        [-8, -8],
        [40, -30],
        [-40, 30],
        [784425, -3],
        [0, 0],
    ] {
        g.generate(seed, x, z);
        i(o, g.starts.len() as i32);
        for key in g.keys() {
            l(o, key);
            let s = &g.starts[&key];
            bounds(o, s.bounds);
            i(o, s.pieces.len() as i32);
            for p in &s.pieces {
                fortress_piece(o, p);
            }
        }
    }
}
fn structure_placement(o: &mut Vec<u8>, seed: i64, kind: usize, stronghold: bool) {
    use crate::structures::{BoundingBox as B, Piece};
    const SIZES: [[i32; 3]; 14] = [
        [5, 10, 19],
        [19, 10, 19],
        [7, 9, 7],
        [7, 11, 7],
        [7, 8, 9],
        [13, 14, 13],
        [5, 7, 5],
        [5, 7, 5],
        [5, 7, 5],
        [5, 7, 5],
        [5, 14, 10],
        [9, 7, 9],
        [13, 14, 13],
        [5, 10, 8],
    ];
    let mut r = JavaRandom::new(seed);
    for dir in 0..4 {
        let dimension = if stronghold {
            Dimension::Overworld
        } else {
            Dimension::Nether
        };
        let mut w = flat(dimension);
        w.sand_fall_instantly = true;
        for x in -3..=3 {
            for z in -3..=3 {
                for bx in 0..16 {
                    for bz in 0..16 {
                        for y in 0..if stronghold { 64 } else { 32 } {
                            w.chunk_mut(x, z).blocks[crate::world::chunk::idx(bx, y, bz)] = 1;
                        }
                    }
                }
                w.chunk_mut(x, z)
                    .init_height_and_sky_for_dimension(dimension);
            }
        }
        let [sx, sy, sz] = if stronghold {
            if kind == 12 {
                [11, 8, 16]
            } else {
                [5, 5, 7]
            }
        } else {
            SIZES[kind]
        };
        let bb = B::oriented(0, 50, 0, 0, 0, 0, sx, sy, sz, dir);
        let mut fort = if !stronghold {
            Some(crate::structures::nether10::NetherPiece::fixture(
                kind, bb, dir, &mut r,
            ))
        } else {
            None
        };
        let mut strong = if stronghold {
            Some(crate::structures::stronghold::StrongholdPiece::new(
                kind, 7, bb, dir, &mut r,
            ))
        } else {
            None
        };
        l(o, r.state());
        for clip in [
            B::new([-32, 0, -32, 6, 127, 47]),
            B::new([7, 0, -32, 47, 127, 47]),
            B::new([-32, 0, -32, 47, 127, 47]),
        ] {
            let result = if let Some(p) = &mut fort {
                p.generate(&mut w, &mut r, clip)
            } else {
                strong.as_mut().unwrap().generate(&mut w, &mut r, clip)
            };
            b(o, result);
            l(o, r.state());
            l(o, w.rand.state());
            if let Some(p) = &fort {
                fortress_piece(o, p);
            } else {
                b(o, strong.as_ref().unwrap().flags[0]);
            }
            for x in -2..=2 {
                for z in -2..=2 {
                    let c = w.chunk(x, z);
                    o.extend(&*c.blocks);
                    o.extend(&*c.data.0);
                }
            }
            crate::structures::verify::tiles(o, &mut w, -32, 47);
        }
    }
}
