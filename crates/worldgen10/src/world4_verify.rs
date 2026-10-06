//! Original-bytecode fixtures for the Release world round-four suite.
use crate::{
    nbt::{self, Compound, NamedTag10, Tag},
    Biome, BiomeSource, JavaRandom,
};
const KEYS: [&str; 16] = [
    "xPos",
    "zPos",
    "Blocks",
    "Data",
    "Aa",
    "BB",
    "Health",
    "Items",
    "Pos",
    "Motion",
    "Level",
    "Player",
    "name",
    "\0",
    "\u{e9}",
    "\u{1f600}",
];
fn i(o: &mut Vec<u8>, n: i32) {
    o.extend(n.to_be_bytes());
}
fn tag(r: &mut JavaRandom, ty: i32, depth: i32) -> Tag {
    match ty {
        1 => Tag::Byte(r.next_int_unbounded() as i8),
        2 => Tag::Short(r.next_int_unbounded() as i16),
        3 => Tag::Int(r.next_int_unbounded()),
        4 => Tag::Long(r.next_long()),
        5 => Tag::Float(f32::from_bits(r.next_int_unbounded() as u32)),
        6 => Tag::Double(f64::from_bits(r.next_long() as u64)),
        7 => {
            let mut b = vec![0; r.next_int(65) as usize];
            for chunk in b.chunks_mut(4) {
                let v = r.next_int_unbounded().to_le_bytes();
                chunk.copy_from_slice(&v[..chunk.len()]);
            }
            Tag::ByteArray(b)
        }
        8 => Tag::String(format!(
            "{}{}",
            KEYS[r.next_int(16) as usize],
            KEYS[r.next_int(16) as usize]
        )),
        9 => {
            let ty = 1 + r.next_int(if depth > 0 { 10 } else { 8 });
            let n = r.next_int(9);
            Tag::List(ty as u8, (0..n).map(|_| tag(r, ty, depth - 1)).collect())
        }
        10 => {
            let mut c = Compound::new();
            let n = 1 + r.next_int(20);
            for _ in 0..n {
                let k = KEYS[r.next_int(16) as usize];
                let ty = 1 + r.next_int(if depth > 0 { 10 } else { 8 });
                c.put(k, tag(r, ty, depth - 1));
            }
            Tag::Compound(c)
        }
        _ => unreachable!(),
    }
}
fn nbt(o: &mut Vec<u8>, seed: i64) {
    let mut r = JavaRandom::new(seed);
    o.push(0);
    for ty in 1..=10 {
        for n in 0..32 {
            let t = NamedTag10::new(Some(KEYS[n % 16]), Some(tag(&mut r, ty, 3)));
            let mut bytes = vec![];
            t.write(&mut bytes).unwrap();
            o.extend(&bytes);
            t.copy10().write(o).unwrap();
            NamedTag10::read(&mut &bytes[..]).unwrap().write(o).unwrap();
            if let Some(Tag::Compound(c)) = &t.payload {
                for _ in 0..2 {
                    let gz = nbt::write_gzip(c).unwrap();
                    let c = nbt::read_gzip(&gz).unwrap();
                    nbt::write_root(o, &c).unwrap();
                }
            }
        }
    }
    o.extend(r.state().to_be_bytes());
}
fn colors(o: &mut Vec<u8>, seed: i64) {
    let source = BiomeSource::new(seed);
    let grass: Vec<u32> = (0..65536i32)
        .map(|i| ((i.wrapping_mul(1234567) ^ 0x654321) & 0xffffff) as u32)
        .collect();
    let foliage: Vec<u32> = (0..65536i32)
        .map(|i| ((i.wrapping_mul(7654321) ^ 0x123456) & 0xffffff) as u32)
        .collect();
    for id in 0..16 {
        for x in [-4097, -17, -1, 0, 8, 31, 2048] {
            for z in [-33, 0, 17] {
                for y in [0, 64, 127] {
                    let biome = Biome::from_id(id);
                    i(o, biome.grass_color10(&source, x, y, z, &grass) as i32);
                    i(o, biome.foliage_color10(&source, x, y, z, &foliage) as i32);
                }
            }
        }
    }
}
fn nearest(o: &mut Vec<u8>, seed: i64) {
    let source = BiomeSource::new(seed);
    let mut g = crate::structures::stronghold::StrongholdGenerator::default();
    for stage in 0..3 {
        if stage == 1 {
            for (x, z) in g.candidates.unwrap() {
                g.generate(seed, x, z, &source);
            }
        }
        for [x, y, z] in [
            [0, 64, 0],
            [100, 0, -200],
            [-512, 127, 256],
            [30000000, 65, -30000000],
            [i32::MIN, i32::MAX, i32::MAX],
        ] {
            let p = g.nearest(seed, &source, x, y, z);
            o.push(p.is_some() as u8);
            if let Some(p) = p {
                for n in p {
                    i(o, n);
                }
            }
        }
    }
}
fn intcache(o: &mut Vec<u8>) {
    let mut c = crate::intcache::IntCache::default();
    for round in 0..8 {
        c.reset();
        for n in [1, 256, 257, 300, 33, 512, 260, 17] {
            let (mut a, allocation) = c.allocate(n);
            i(o, a.len() as i32);
            for &v in &a {
                i(o, v);
            }
            for (j, v) in a[..n].iter_mut().enumerate() {
                *v = round * 1000 + n as i32 + j as i32;
            }
            c.finish(&a, allocation);
        }
    }
}
pub fn dump(mode: &str, seed: i64) -> Vec<u8> {
    let mut o = vec![];
    match mode {
        "nbt" => nbt(&mut o, seed),
        "colors" => colors(&mut o, seed),
        "nearest" => nearest(&mut o, seed),
        "intcache" => intcache(&mut o),
        "gzipbytes" => gzipbytes(&mut o, seed),
        "sharedsave" => sharedsave(&mut o),
        "misc" => misc(&mut o),
        "providerapi" => providerapi(&mut o, seed),
        "worldinfo" => worldinfo(&mut o, seed),
        "savefolders" => savefolders(&mut o),
        "notify" => notify(&mut o),
        "endteleport" => endteleport(&mut o),
        "population" => population(&mut o, seed, crate::world::Dimension::Overworld),
        "hellpopulation" => population(&mut o, seed, crate::world::Dimension::Nether),
        "waterlily" => waterlily(&mut o, seed),
        _ => panic!("unknown world4 mode {mode}"),
    }
    o
}
fn population(o: &mut Vec<u8>, seed: i64, dimension: crate::world::Dimension) {
    let mut w = crate::World::with_dimension(seed, 300, dimension);
    w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(500);
    w.next_entity_id = 100;
    for x in -2..=2 {
        for z in -2..=2 {
            w.load_chunk(x, z);
        }
    }
    o.extend(w.prand().state().to_be_bytes());
    o.extend(w.rand.state().to_be_bytes());
    let crate::entity::EntitySeedSource::Deterministic(next) = w.entity_seed_source else {
        unreachable!()
    };
    o.extend(next.to_be_bytes());
    i(o, w.next_entity_id);
    o.push(w.sand_fall_instantly as u8);
    let mut keys: Vec<_> = w.chunks.keys().copied().collect();
    keys.sort();
    i(o, keys.len() as i32);
    for (x, z) in keys {
        let c = w.chunk(x, z);
        i(o, x);
        i(o, z);
        o.push(c.populated as u8);
        o.extend(&*c.blocks);
        o.extend(&*c.data.0);
        o.extend(&*c.sky_light.0);
        o.extend(&*c.block_light.0);
        o.extend(&*c.height_map);
    }
    i(o, w.loaded_entity_list.len() as i32);
    for id in &w.loaded_entity_list {
        let e = &w.entities[id];
        i(o, e.entity_id);
        crate::nbt::write_utf(o, e.kind.name()).unwrap();
        o.extend(e.pos.x.to_be_bytes());
        o.extend(e.pos.y.to_be_bytes());
        o.extend(e.pos.z.to_be_bytes());
        o.extend(e.yaw.to_be_bytes());
        o.extend(e.rand.state().to_be_bytes());
    }
}
fn waterlily(o: &mut Vec<u8>, seed: i64) {
    use crate::world::{
        chunk::{idx, Chunk},
        Dimension,
    };
    for fixture in 0..8 {
        let mut w = crate::World::with_dimension(0, 300, Dimension::Overworld);
        for x in -3..=3 {
            for z in -3..=3 {
                let mut c = Chunk::new(x, z, vec![0; 32768].into_boxed_slice());
                c.loaded = true;
                c.populated = true;
                w.chunks.insert((x, z), Box::new(c));
            }
        }
        for x in -12..=28 {
            for z in -12..=28 {
                let c = w.chunk_mut(x >> 4, z >> 4);
                let id = [9, 8, 9, 8, 1, 9, 9, 9][fixture];
                c.blocks[idx(x & 15, 63, z & 15)] = id;
                c.data.set(
                    x & 15,
                    63,
                    z & 15,
                    if fixture == 2 || fixture == 3 { 1 } else { 0 },
                );
                if fixture == 5 {
                    c.blocks[idx(x & 15, 64, z & 15)] = 1;
                }
            }
        }
        w.prand().set_seed(seed);
        let result = crate::features181::waterlily(
            &mut w,
            8,
            if fixture == 6 {
                127
            } else if fixture == 7 {
                0
            } else {
                64
            },
            8,
        );
        o.push(result as u8);
        o.extend(w.prand().state().to_be_bytes());
        o.extend(w.rand.state().to_be_bytes());
        for x in -1..=1 {
            for z in -1..=1 {
                let c = w.chunk(x, z);
                o.extend(&*c.blocks);
                o.extend(&*c.data.0);
            }
        }
    }
}
fn misc(o: &mut Vec<u8>) {
    use crate::entity::player::{
        registry::{use_action, ItemUseAction},
        ItemStack,
    };
    for (a, name) in [
        (ItemUseAction::None, "none"),
        (ItemUseAction::Eat, "eat"),
        (ItemUseAction::Drink, "drink"),
        (ItemUseAction::Block, "block"),
        (ItemUseAction::Bow, "bow"),
    ] {
        i(o, a as i32);
        nbt::write_utf(o, name).unwrap();
    }
    for id in [1, 260, 261, 267, 282, 319, 322, 335, 373, 375, 376, 382] {
        i(o, use_action(id) as i32);
    }
    // Empty3's original classfile contains no fields, methods or constructors.
    i(o, 0);
    i(o, 0);
    i(o, 0);
    for round in 0..32 {
        let a = ItemStack::new(276, 1, 0);
        let b = ItemStack::new(267, 1, 0);
        a.add_enchantment(16, round + 1);
        a.add_enchantment(17, 3);
        a.add_enchantment(16, 2);
        b.add_enchantment(19, 4);
        let mut trace = vec![];
        crate::enchantment::apply_modifier_array(
            &mut |e: &crate::enchantment::Enchantment, level| {
                trace.extend([
                    e.id,
                    level,
                    std::ptr::eq(e, crate::enchantment::get_ref(e.id).unwrap()) as i32,
                ]);
            },
            &[None, Some(a), Some(ItemStack::new(1, 1, 0)), Some(b)],
        );
        i(o, trace.len() as i32);
        for n in trace {
            i(o, n);
        }
    }
}
fn providerapi(o: &mut Vec<u8>, seed: i64) {
    use crate::world::Dimension;
    for dimension in [Dimension::Nether, Dimension::Overworld, Dimension::End] {
        let mut w = crate::World::with_dimension(seed, 300, dimension);
        i(o, dimension.id());
        o.push(dimension.has_no_sky() as u8);
        o.push(dimension.is_hell_world() as u8);
        o.push(dimension.can_respawn_here() as u8);
        o.push(dimension.is_surface_world() as u8);
        o.extend(dimension.cloud_height().to_be_bytes());
        for time in [i64::MIN, -24001, -1, 0, 24000, 192000, i64::MAX] {
            i(o, dimension.moon_phase(time));
        }
        for _ in 0..2 {
            for [x, y, z] in [[0, 64, 0], [-17, 1, 31], [1024, 127, -512]] {
                for category in 0..3 {
                    let list = w.provider_spawn_list10(category, x, y, z);
                    i(o, list.len() as i32);
                    for e in list {
                        nbt::write_utf(
                            o,
                            match e.entity {
                                "LavaSlime" => "MagmaCube",
                                "MushroomCow" => "Mooshroom",
                                name => name,
                            },
                        )
                        .unwrap();
                        for n in [e.weight, e.min, e.max] {
                            i(o, n);
                        }
                    }
                }
            }
        }
        for _ in 0..2 {
            for name in ["Stronghold", "stronghold", "Village", ""] {
                let p = w.closest_structure10(name, 0, 64, 0);
                o.push(p.is_some() as u8);
                if let Some(p) = p {
                    for n in p {
                        i(o, n);
                    }
                }
            }
        }
    }
    o.push(1); // Unsupported dimensions have no provider.
}
fn worldinfo(o: &mut Vec<u8>, seed: i64) {
    for fixture in 0..12 {
        let mut c = Compound::new();
        c.put("RandomSeed", Tag::Long(seed));
        c.put("GameType", Tag::Int(fixture % 2));
        if fixture % 3 != 0 {
            c.put("hardcore", Tag::Byte((fixture % 3 == 2) as i8));
        }
        if fixture % 4 != 0 {
            c.put("MapFeatures", Tag::Byte((fixture % 4 == 2) as i8));
        }
        c.put("LevelName", Tag::String("World \u{e9}\u{1f600}".into()));
        c.put("Time", Tag::Long(-i64::from(fixture)));
        c.put("version", Tag::Int(19132));
        let mut player = Compound::new();
        player.put("Dimension", Tag::Int([0, -1, 1][fixture as usize % 3]));
        c.put("Player", Tag::Compound(player));
        let info = crate::storage::LevelInfo::from_nbt(&c);
        for info in [&info, &info.clone()] {
            o.push(info.hardcore as u8);
            o.push(info.map_features as u8);
            i(o, info.dimension());
            let mut result = info.to_nbt();
            result.put("LastPlayed", Tag::Long(0));
            nbt::write_root(o, &result).unwrap();
        }
    }
}
fn savefolders(o: &mut Vec<u8>) {
    use crate::{storage::Storage, world::Dimension};
    let root = std::path::Path::new("target/r10/world4/savefolders-rust");
    if root.exists() {
        std::fs::remove_dir_all(root).unwrap();
    }
    std::fs::create_dir_all(root).unwrap();
    for region in [false, true] {
        for name in ["world", "World \u{e9}\u{1f600}"] {
            let dir = root.join(name);
            let s = if region {
                Storage::open_with_players(&dir, true)
            } else {
                Storage::open_legacy_with_players(&dir, true)
            }
            .unwrap();
            nbt::write_utf(o, &s.save_name10()).unwrap();
            for dimension in [Dimension::Overworld, Dimension::Nether, Dimension::End] {
                let loader = s.for_dimension(dimension).unwrap();
                o.push(loader.uses_threaded_chunks10() as u8);
                o.push(
                    dir.join(match dimension {
                        Dimension::Nether => "DIM-1",
                        Dimension::End => "DIM1",
                        _ => "",
                    })
                    .is_dir() as u8,
                );
            }
            crate::threaded_io10::FILE_IO.flush();
            o.push(dir.join("data").is_dir() as u8);
            o.push(dir.join("players").is_dir() as u8);
        }
    }
    std::fs::remove_dir_all(root).unwrap();
}
fn notify(o: &mut Vec<u8>) {
    for notify in [false, true] {
        let mut w = crate::world3_verify::flat(crate::world::Dimension::Overworld);
        for [id, meta] in [[1, 7], [1, 3], [17, 2], [17, 2], [0, 5], [1, 0]] {
            crate::features::generator_place10(&mut w, notify, 8, 64, 8, id as u8, meta);
            i(o, w.get_block_id(8, 64, 8) as i32);
            i(o, w.get_block_metadata(8, 64, 8));
            o.extend(w.rand.state().to_be_bytes());
        }
    }
}
fn endteleport(o: &mut Vec<u8>) {
    use crate::entity::{Entity, EntitySeedSource, Vec3D};
    for [x, y, z] in [
        [8.25, 70., 8.75],
        [-0.25, 64.5, -16.1],
        [32., 50., 0.],
        [8., 1., 8.],
        [8., 128., 8.],
    ] {
        let mut w = crate::world3_verify::flat(crate::world::Dimension::End);
        w.entity_seed_source = EntitySeedSource::Deterministic(500);
        let mut e = Entity::living(&mut w, "Pig").unwrap();
        e.set_position(x, y, z);
        e.yaw = 123.;
        e.pitch = -20.;
        e.motion = Vec3D::new(1., 2., 3.);
        let id = e.entity_id;
        w.entities.insert(id, e);
        let mut r = JavaRandom::new(42);
        w.teleport_to_portal(id, &mut r);
        let e = &w.entities[&id];
        for v in [
            e.pos.x, e.pos.y, e.pos.z, e.motion.x, e.motion.y, e.motion.z,
        ] {
            o.extend(v.to_be_bytes());
        }
        o.extend(e.yaw.to_be_bytes());
        o.extend(e.pitch.to_be_bytes());
        o.extend(r.state().to_be_bytes());
        o.extend(w.rand.state().to_be_bytes());
        for x in -3..=3 {
            for z in -3..=3 {
                let c = w.chunk(x, z);
                o.extend(&*c.blocks);
                o.extend(&*c.data.0);
            }
        }
    }
}
fn gzipbytes(o: &mut Vec<u8>, seed: i64) {
    let mut r = JavaRandom::new(seed);
    for _ in 0..16 {
        let Tag::Compound(c) = tag(&mut r, 10, 3) else {
            unreachable!()
        };
        let bytes = nbt::write_gzip(&c).unwrap();
        i(o, bytes.len() as i32);
        o.extend(bytes);
    }
}
fn sharedsave(o: &mut Vec<u8>) {
    use crate::{
        threaded_io10::{IThreadedFileIO, ThreadedChunkLoader, FILE_IO},
        world::chunk::LightKind,
    };
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
    let (started, wait) = mpsc::channel();
    let (release, blocked) = mpsc::channel();
    FILE_IO.enqueue(Arc::new(Barrier {
        started,
        release: Mutex::new(blocked),
    }));
    wait.recv().unwrap();
    let dir = std::path::Path::new("target/r10/world4/sharedsave-rust");
    std::fs::create_dir_all(dir).unwrap();
    let loader = ThreadedChunkLoader::new(dir);
    let mut w = crate::world3_verify::flat(crate::world::Dimension::Overworld);
    let mut root = Compound::new();
    root.put(
        "Level",
        Tag::Compound(crate::storage::chunk_to_nbt(w.chunk(0, 0), 0)),
    );
    loader.save(0, 0, root);
    w.set_block_and_metadata(8, 64, 8, 17, 2);
    w.set_block_metadata(8, 64, 8, 7);
    w.chunk_mut(0, 0).set_light(LightKind::Sky, 8, 64, 8, 12);
    w.chunk_mut(0, 0).set_light(LightKind::Block, 8, 64, 8, 3);
    let root = loader.load_pending(0, 0).unwrap();
    let level = root.compound("Level").unwrap();
    for name in ["Blocks", "Data", "SkyLight", "BlockLight", "HeightMap"] {
        o.extend(level.bytes(name).unwrap());
    }
    loader.write_next();
    release.send(()).unwrap();
    FILE_IO.flush();
    let bytes = crate::region_cache181::read_chunk(dir, 0, 0)
        .unwrap()
        .unwrap();
    let root = nbt::read_root(&mut &bytes[..]).unwrap();
    let level = root.compound("Level").unwrap();
    for name in ["Blocks", "Data", "SkyLight", "BlockLight", "HeightMap"] {
        o.extend(level.bytes(name).unwrap());
    }
    crate::region_cache181::clear();
    std::fs::remove_dir_all(dir).unwrap();
}
