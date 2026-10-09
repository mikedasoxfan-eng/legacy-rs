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
/// Modes whose oracle output starts with the recorded tick-entry identity hashes.
fn replays_tick_identities(mode: &str) -> bool {
    matches!(mode, "ticks" | "hellticks" | "regionbytes" | "hellregionbytes")
}
static TICK_IDENTITIES: std::sync::Mutex<Option<std::collections::VecDeque<i32>>> =
    std::sync::Mutex::new(None);
fn replay_world(seed: i64, dimension: crate::world::Dimension) -> crate::World {
    let mut w = crate::World::with_dimension(seed, 300, dimension);
    w.tick_identity_replay10 = TICK_IDENTITIES.lock().unwrap().take();
    w
}
/// `dump` for modes that replay VM identity hashes recorded at the start of `expected`.
pub fn dump_expected(mode: &str, seed: i64, expected: &[u8]) -> Vec<u8> {
    if !replays_tick_identities(mode) {
        return dump(mode, seed);
    }
    let n = i32::from_be_bytes(expected[..4].try_into().unwrap()) as usize;
    let prefix = &expected[..4 + 4 * n];
    *TICK_IDENTITIES.lock().unwrap() = Some(
        prefix[4..]
            .chunks(4)
            .map(|c| i32::from_be_bytes(c.try_into().unwrap()))
            .collect(),
    );
    let mut o = prefix.to_vec();
    o.extend(dump(mode, seed));
    o
}
pub fn dump(mode: &str, seed: i64) -> Vec<u8> {
    let mut o = vec![];
    match mode {
        "nbt" => nbt(&mut o, seed),
        "colors" => colors(&mut o, seed),
        "nearest" => nearest(&mut o, seed),
        "intcache" => intcache(&mut o),
        "gzipbytes" => gzipbytes(&mut o, seed),
        "fortressspawn" => fortressspawn(&mut o, seed),
        "saveformat" => saveformat(&mut o, seed),
        "regionbytes" => regionbytes(&mut o, seed, crate::world::Dimension::Overworld),
        "hellregionbytes" => regionbytes(&mut o, seed, crate::world::Dimension::Nether),
        "misc" => misc(&mut o),
        "providerapi" => providerapi(&mut o, seed),
        "worldinfo" => worldinfo(&mut o, seed),
        "savefolders" => savefolders(&mut o),
        "notify" => notify(&mut o),
        "endteleport" => endteleport(&mut o),
        "population" => population(&mut o, seed, crate::world::Dimension::Overworld),
        "hellpopulation" => population(&mut o, seed, crate::world::Dimension::Nether),
        "waterlily" => waterlily(&mut o, seed),
        "ticks" => ticks(&mut o, seed, crate::world::Dimension::Overworld),
        "hellticks" => ticks(&mut o, seed, crate::world::Dimension::Nether),
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
        o.push(dimension.is_alternate_dimension() as u8);
        o.push(dimension.can_respawn_here() as u8);
        o.push(dimension.is_surface_world() as u8);
        o.extend(dimension.cloud_height().to_be_bytes());
        for time in [i64::MIN, -24001, -1, 0, 24000, 192000, i64::MAX] {
            i(o, dimension.moon_phase(time));
        }
        for _ in 0..3 {
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
        for _ in 0..3 {
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
    // Large payloads cross the 32 KiB window and the 16 KiB symbol-buffer block limit.
    for n in 0..10 {
        let len = if n == 9 { 0 } else { 1 + r.next_int(120000) as usize };
        let mut b = vec![0u8; len];
        for j in 0..len {
            b[j] = match n % 3 {
                0 => r.next_int_unbounded() as u8,
                1 => {
                    if j > 0 && r.next_int(5) != 0 {
                        let back = r.next_int(if j < 40000 { 300 } else { 40000 }) as i64;
                        b[(j as i64 - 1 - back).max(0) as usize]
                    } else {
                        r.next_int(16) as u8
                    }
                }
                _ => {
                    if r.next_int(8) == 0 {
                        r.next_int(3) as u8
                    } else {
                        0
                    }
                }
            };
        }
        let mut c = Compound::new();
        c.put("Blocks", Tag::ByteArray(b));
        c.put("xPos", Tag::Int(n));
        let bytes = nbt::write_gzip(&c).unwrap();
        i(o, bytes.len() as i32);
        o.extend(bytes);
    }
}
fn write_spawn_list(o: &mut Vec<u8>, list: &[crate::weighted::SpawnListEntry]) {
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
/// MapGenNetherBridge.canSpawnStructureAtCoords.
fn fortress_candidate(seed: i64, x: i32, z: i32) -> bool {
    let (rx, rz) = (x >> 4, z >> 4);
    let mut r = JavaRandom::new(((rx ^ (rz << 4)) as i64) ^ seed);
    r.next_int_unbounded();
    r.next_int(3) == 0
        && x == (rx << 4).wrapping_add(4).wrapping_add(r.next_int(8))
        && z == (rz << 4).wrapping_add(4).wrapping_add(r.next_int(8))
}
fn fortressspawn(o: &mut Vec<u8>, seed: i64) {
    let mut w = crate::World::with_dimension(seed, 300, crate::world::Dimension::Nether);
    w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(500);
    w.next_entity_id = 100;
    let (mut bx, mut bz, mut best) = (0, 0, i32::MAX);
    for cx in -40..40i32 {
        for cz in -40..40i32 {
            if cx.abs() + cz.abs() < best && fortress_candidate(seed, cx, cz) {
                best = cx.abs() + cz.abs();
                bx = cx;
                bz = cz;
            }
        }
    }
    i(o, bx);
    i(o, bz);
    for x in bx - 3..=bx + 3 {
        for z in bz - 3..=bz + 3 {
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
    let mut hits = 0;
    for x in (bx * 16 - 96..=bx * 16 + 112).step_by(5) {
        for z in (bz * 16 - 96..=bz * 16 + 112).step_by(5) {
            for y in [34, 40, 48, 56, 64, 72, 80, 90] {
                let list = w.provider_spawn_list10(0, x, y, z);
                let inside = list.first().is_some_and(|e| e.entity == "Blaze");
                hits += inside as i32;
                o.push(inside as u8);
                write_spawn_list(o, &list);
            }
        }
    }
    i(o, hits);
    println!("fortress chunk {bx},{bz}: {hits} query points inside");
    for x in bx - 3..=bx + 3 {
        for z in bz - 3..=bz + 3 {
            let c = w.chunk(x, z);
            o.push(c.populated as u8);
            o.extend(&*c.blocks);
            o.extend(&*c.data.0);
        }
    }
}
/// Region files with the wall-clock timestamp table cleared.
fn region_files(o: &mut Vec<u8>, dir: &std::path::Path, dimension: crate::world::Dimension) {
    use crate::world::Dimension;
    let region = dir.join(match dimension {
        Dimension::Overworld => "region",
        Dimension::Nether => "DIM-1/region",
        Dimension::End => "DIM1/region",
    });
    let mut names: Vec<String> = std::fs::read_dir(&region)
        .unwrap()
        .map(|e| e.unwrap().file_name().into_string().unwrap())
        .collect();
    names.sort();
    i(o, names.len() as i32);
    for name in names {
        let mut bytes = std::fs::read(region.join(&name)).unwrap();
        for b in bytes.iter_mut().take(8192).skip(4096) {
            *b = 0;
        }
        nbt::write_utf(o, &name).unwrap();
        i(o, bytes.len() as i32);
        o.extend(bytes);
    }
}
fn regionbytes(o: &mut Vec<u8>, seed: i64, dimension: crate::world::Dimension) {
    use crate::world::Dimension;
    let dir = std::env::current_dir()
        .unwrap()
        .join("target/r10/world4/regionbytes-rust");
    if dir.exists() {
        std::fs::remove_dir_all(&dir).unwrap();
    }
    let mut w = replay_world(seed, dimension);
    w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(500);
    w.next_entity_id = 100;
    let storage = crate::storage::Storage::open(&dir).unwrap();
    w.storage = Some(if dimension == Dimension::Overworld {
        storage
    } else {
        storage.for_dimension(dimension).unwrap()
    });
    for x in -2..=2 {
        for z in -2..=2 {
            w.load_chunk(x, z);
        }
    }
    w.save_world(true);
    crate::threaded_io10::FILE_IO.flush();
    region_files(o, &dir, dimension);
    let mut r = JavaRandom::new(seed);
    for _ in 0..3 {
        for x in -2..=2 {
            for z in -2..=2 {
                let kind = r.next_int(4);
                if kind == 0 {
                    continue;
                }
                let c = w.chunk_mut(x, z);
                for b in c.blocks.iter_mut() {
                    if kind == 1 {
                        if r.next_int(6) == 0 {
                            *b = (1 + r.next_int(90)) as u8;
                        }
                    } else if kind == 2 {
                        *b = 0;
                    } else if r.next_int(3) == 0 {
                        *b = r.next_int(100) as u8;
                    }
                }
                c.modified = true;
            }
        }
        w.save_world(true);
        crate::threaded_io10::FILE_IO.flush();
        region_files(o, &dir, dimension);
    }
    crate::region_cache181::clear();
    region_files(o, &dir, dimension);
    drop(w);
    std::fs::remove_dir_all(&dir).unwrap();
}
fn ticks(o: &mut Vec<u8>, seed: i64, dimension: crate::world::Dimension) {
    let mut w = replay_world(seed, dimension);
    w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(500);
    w.next_entity_id = 100;
    for x in -2..=2 {
        for z in -2..=2 {
            w.load_chunk(x, z);
        }
    }
    let tree: Vec<_> = w.tick_order.iter().map(|&(t, _, k)| (k, t)).collect();
    let set: Vec<_> = w
        .scheduled_tick_set10
        .iter()
        .map(|k| (k, w.pending_ticks[&k].0))
        .collect();
    for list in [tree, set] {
        i(o, list.len() as i32);
        for (k, t) in list {
            for n in [k.x, k.y, k.z, k.block] {
                i(o, n);
            }
            o.extend(t.to_be_bytes());
        }
    }
    for x in -2..=2 {
        for z in -2..=2 {
            i(o, w.pending_chunk_ticks(x, z, false).map_or(-1, |l| l.len() as i32));
        }
    }
}
fn saveformat(o: &mut Vec<u8>, seed: i64) {
    let root = std::env::current_dir()
        .unwrap()
        .join("target/r10/world4/saveformat-rust");
    let _ = std::fs::remove_dir_all(&root);
    let format = crate::save_format_old181::SaveFormatOld181::new(&root);
    let mut r = JavaRandom::new(seed);
    for n in 1..=6 {
        let dir = root.join(format!("World{n}"));
        std::fs::create_dir_all(&dir).unwrap();
        let mut data = Compound::new();
        data.put("RandomSeed", Tag::Long(r.next_long()));
        data.put("GameType", Tag::Int(r.next_int(3) - 1));
        data.put("LastPlayed", Tag::Long(r.next_int(4) as i64 * 1000));
        data.put("SizeOnDisk", Tag::Long(r.next_int(1 << 20) as i64));
        if n % 3 != 0 {
            data.put("hardcore", Tag::Byte((n % 3 == 1) as i8));
        }
        data.put("LevelName", Tag::String(format!("name\u{e9}{n}")));
        let mut tag = Compound::new();
        tag.put("Data", Tag::Compound(data));
        std::fs::write(dir.join("level.dat"), nbt::write_gzip(&tag).unwrap()).unwrap();
    }
    let mut worlds = format.worlds();
    // Collections.sort: a stable sort by compareTo.
    worlds.sort_by(|a, b| a.compare(b).cmp(&0));
    i(o, worlds.len() as i32);
    for w in worlds {
        nbt::write_utf(o, &w.file_name).unwrap();
        nbt::write_utf(o, &w.display_name).unwrap();
        o.extend(w.last_played.to_be_bytes());
        i(o, w.game_type);
        o.push(w.requires_conversion as u8);
        o.push(w.hardcore as u8);
    }
    std::fs::remove_dir_all(&root).unwrap();
}
