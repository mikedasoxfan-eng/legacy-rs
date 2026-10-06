//! Original-bytecode structure serialization (big endian, Java writeUTF for ASCII names).
use super::{
    mineshaft::{Kind, MineshaftPiece},
    BoundingBox as B, Piece,
};
use crate::{
    world::{chunk::Chunk, tile::TileEntity},
    JavaRandom, World,
};
fn int(out: &mut Vec<u8>, n: i32) {
    out.extend(n.to_be_bytes());
}
fn text(out: &mut Vec<u8>, s: &str) {
    out.extend((s.len() as u16).to_be_bytes());
    out.extend(s.as_bytes());
}
fn bounds(out: &mut Vec<u8>, b: B) {
    for n in b.values() {
        int(out, n);
    }
}
fn pieces(out: &mut Vec<u8>, ps: &[MineshaftPiece], b: B) {
    bounds(out, b);
    int(out, ps.len() as i32);
    for p in ps {
        text(out, p.name());
        int(out, p.component.depth);
        int(out, p.component.orientation);
        bounds(out, p.component.bounds);
        match &p.kind {
            Kind::Room { entrances } => {
                int(out, entrances.len() as i32);
                for &e in entrances {
                    bounds(out, e);
                }
            }
            Kind::Corridor {
                rails,
                spiders,
                spawner,
                sections,
            } => {
                out.extend([*rails as u8, *spiders as u8, *spawner as u8]);
                int(out, *sections);
            }
            Kind::Cross { direction, tall } => {
                int(out, *direction);
                out.push(*tall as u8);
            }
            Kind::Stairs => {}
        }
    }
}
pub fn dump(mode: &str, seed: i64) -> Vec<u8> {
    if mode == "terrain-nbt" {
        return terrain_nbt(seed);
    }
    if let Some(mode) = mode.strip_prefix("chunkcache-") {
        return crate::world::cache_verify181::dump_cache(mode, seed);
    }
    if let Some(mode) = mode.strip_prefix("provider-") {
        return provider(mode, seed);
    }
    if let Some(mode) = mode.strip_prefix("stronghold-") {
        return stronghold(mode, seed);
    }
    if mode.starts_with("village") {
        return village(mode, seed);
    }
    let mut out = Vec::new();
    let mut r = JavaRandom::new(seed);
    if mode == "layout" {
        for i in 0..16 {
            let (ps, b) = MineshaftPiece::start(&mut r, i % 4 - 2, i / 4 - 2);
            pieces(&mut out, &ps, b);
            out.extend(r.state().to_be_bytes());
        }
    } else if mode == "map" {
        let mut g = super::MineshaftGenerator::default();
        for [x, z] in [
            [0, 0],
            [40, -30],
            [-40, 30],
            [784425, 0],
            [-3, -784425],
            [40, -30],
        ] {
            out.extend(g.generate(seed, x, z).to_be_bytes());
            int(&mut out, g.starts.len() as i32);
            for key in g.keys() {
                out.extend(key.to_be_bytes());
                let (ps, b) = &g.starts[&key];
                pieces(&mut out, ps, *b);
            }
        }
    } else if mode == "blocks" {
        let (mut ps, b) = MineshaftPiece::start(&mut r, 0, 0);
        pieces(&mut out, &ps, b);
        out.extend(r.state().to_be_bytes());
        let mut w = World::new(0, 300);
        w.sand_fall_instantly = true;
        for cx in -7..=7 {
            for cz in -7..=7 {
                let mut bs = Box::new([0; 32768]);
                for x in 0..16 {
                    for z in 0..16 {
                        for y in 0..64 {
                            bs[x << 11 | z << 7 | y] = 1;
                        }
                    }
                }
                let mut c = Chunk::new(cx, cz, bs);
                c.populated = true;
                c.loaded = true;
                c.init_height_and_sky();
                w.chunks.insert((cx, cz), Box::new(c));
            }
        }
        for cx in -7..7 {
            for cz in -7..7 {
                let x = cx * 16 + 8;
                let z = cz * 16 + 8;
                let clip = B::region(x, z, x + 15, z + 15);
                if b.intersects(clip) {
                    ps.retain_mut(|p| {
                        !p.bounds().intersects(clip) || p.generate(&mut w, &mut r, clip)
                    });
                }
            }
        }
        pieces(&mut out, &ps, b);
        out.extend(r.state().to_be_bytes());
        out.extend(w.rand.state().to_be_bytes());
        for cx in -7..=7 {
            for cz in -7..=7 {
                let c = w.chunk(cx, cz);
                out.extend(&c.blocks[..]);
                out.extend(&c.data.0[..]);
            }
        }
        tiles(&mut out, &mut w, -112, 127);
    } else {
        panic!("unknown structure fixture {mode}");
    }
    out
}
fn terrain_nbt(seed: i64) -> Vec<u8> {
    use crate::nbt::{Compound, Tag};
    let mut keys: Vec<String> = [
        "xPos",
        "zPos",
        "LastUpdate",
        "Blocks",
        "Data",
        "SkyLight",
        "BlockLight",
        "HeightMap",
        "TerrainPopulated",
        "Entities",
        "TileEntities",
        "id",
        "x",
        "y",
        "z",
        "Items",
        "Slot",
        "Count",
        "Damage",
        "EntityId",
        "Delay",
        "Pos",
        "Motion",
        "Rotation",
        "FallDistance",
        "Fire",
        "Air",
        "OnGround",
        "Health",
        "HurtTime",
        "DeathTime",
        "AttackTime",
        "Sheared",
        "Color",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect();
    for i in 0..64 {
        let mut s = String::new();
        for b in 0..6 {
            s.push_str(if i & (1 << b) == 0 { "Aa" } else { "BB" });
        }
        keys.push(s);
    }
    let mut r = JavaRandom::new(seed);
    let mut out = Vec::new();
    for _ in 0..8 {
        let mut order = keys.clone();
        for i in (2..=order.len()).rev() {
            let j = r.next_int(i as i32) as usize;
            order.swap(i - 1, j);
        }
        let mut c = Compound::new();
        for (i, k) in order.iter().enumerate() {
            c.put(k, Tag::Int(i as i32));
        }
        for k in order.iter().take(12) {
            c.put(k, Tag::Int(r.next_int_unbounded()));
        }
        c.put("Empty", Tag::List(10, vec![]));
        let mut root = Compound::new();
        root.put("Level", Tag::Compound(c));
        crate::nbt::write_root(&mut out, &root).unwrap();
    }
    out.extend(r.state().to_be_bytes());
    out
}
fn provider(mode: &str, seed: i64) -> Vec<u8> {
    let mut out = Vec::new();
    let mut r = JavaRandom::new(seed);
    if mode == "longset" {
        let mut s = super::long_map_order::LongMapOrder::default();
        let mut pool = Vec::new();
        for i in -80..=80 {
            pool.push(super::ChunkCoordIntPair { x: i, z: i }.packed());
        }
        for _ in 0..80 {
            pool.push(r.next_long());
        }
        for i in 0..2400 {
            let k = pool[if i < 161 {
                i
            } else {
                r.next_int(pool.len() as i32) as usize
            }];
            let add = i < 161 || r.next_bool();
            let hash = (k ^ ((k as u64 >> 32) as i64)) as i32;
            out.push(if add {
                s.insert(hash, k)
            } else {
                s.remove(hash, k)
            } as u8);
            int(&mut out, s.len() as i32);
            for k in s.iter() {
                out.extend(k.to_be_bytes());
            }
        }
        while let Some(k) = s.first() {
            s.remove((k ^ ((k as u64 >> 32) as i64)) as i32, k);
            out.extend(k.to_be_bytes());
            int(&mut out, s.len() as i32);
            for k in s.iter() {
                out.extend(k.to_be_bytes());
            }
        }
        out.extend(r.state().to_be_bytes());
        return out;
    }
    let mut w = World::new(0, 300);
    w.spawn = (0, 64, 0);
    for i in -80..=80 {
        let mut c = Chunk::new(i, i, Box::new([0; 32768]));
        c.populated = true;
        c.loaded = true;
        w.chunks.insert((i, i), Box::new(c));
        w.chunk_list.push((i, i));
    }
    if mode == "saves" {
        for &(x, z) in &w.chunk_list.clone() {
            let c = w.chunk_mut(x, z);
            c.has_entities = r.next_bool();
            c.never_save = r.next_int(7) == 0;
            c.modified = r.next_bool();
            c.last_save_time = r.next_int(1000) as i64;
        }
        for time in [0, 599, 600, 1000, i64::MAX, i64::MIN] {
            w.world_time = time;
            for _ in 0..3 {
                out.push(w.save_chunks(false) as u8);
                provider_dump(&mut out, &w);
            }
            out.push(w.save_chunks(true) as u8);
            provider_dump(&mut out, &w);
        }
        out.extend(r.state().to_be_bytes());
        return out;
    }
    for i in 0..70 {
        let x = if i < 12 {
            0.0
        } else if i < 32 {
            900.0
        } else {
            -900.0
        };
        w.players = vec![(x, if i % 3 == 0 { 400.0 } else { 64.0 }, x)];
        if i == 4 {
            for (x, z) in w.chunk_list.clone() {
                w.provider_drop_chunk(x, z);
            }
            w.provider_load_chunk(42, 42);
            w.provide_chunk(-42, -42);
        }
        if i == 10 {
            w.spawn = (880, 64, 880);
        }
        if i == 20 {
            w.spawn = (-880, 64, -880);
        }
        if i == 35 {
            for (x, z) in [(1875004, 0), (-1875005, 0), (0, 1875004), (0, -1875005)] {
                w.provider_load_chunk(x, z);
                out.push(!w.chunk_exists(x, z) as u8);
                out.push(w.chunk_exists(x, z) as u8);
            }
        }
        w.unload_oldest_chunks();
        provider_dump(&mut out, &w);
    }
    out
}
fn provider_dump(out: &mut Vec<u8>, w: &World) {
    int(out, w.provider.scan_cursor as i32);
    int(out, w.chunk_list.len() as i32);
    for &(x, z) in &w.chunk_list {
        let c = w.chunk(x, z);
        int(out, x);
        int(out, z);
        for b in [
            c.loaded,
            c.populated,
            c.modified,
            c.never_save,
            c.has_entities,
        ] {
            out.push(b as u8);
        }
        out.extend(c.last_save_time.to_be_bytes());
    }
    let dropped = w.provider.dropped();
    int(out, dropped.len() as i32);
    for k in dropped {
        out.extend(k.to_be_bytes());
    }
    text(
        out,
        &format!(
            "ServerChunkCache: {} Drop: {}",
            w.chunks.len(),
            w.provider.dropped.len()
        ),
    );
}
fn stronghold_piece(out: &mut Vec<u8>, p: &super::stronghold::StrongholdPiece) {
    text(out, p.name());
    int(out, p.component.depth);
    int(out, p.component.orientation);
    bounds(out, p.bounds());
    int(out, p.door as i32);
    for f in p.flags {
        out.push(f as u8);
    }
    int(out, p.style);
    int(out, p.length);
}
fn stronghold_pieces(out: &mut Vec<u8>, s: &super::stronghold::StrongholdStart) {
    bounds(out, s.bounds);
    int(out, s.pieces.len() as i32);
    for p in &s.pieces {
        stronghold_piece(out, p);
    }
}
fn stronghold_solid(w: &mut World) {
    for cx in -8..=8 {
        for cz in -8..=8 {
            let mut bs = Box::new([0; 32768]);
            for x in 0..16 {
                for z in 0..16 {
                    for y in 0..64 {
                        bs[x << 11 | z << 7 | y] = 1;
                    }
                }
            }
            let mut c = Chunk::new(cx, cz, bs);
            c.populated = true;
            c.loaded = true;
            c.init_height_and_sky();
            w.chunks.insert((cx, cz), Box::new(c));
        }
    }
}
fn stronghold(mode: &str, seed: i64) -> Vec<u8> {
    use super::stronghold::{StrongholdGenerator, StrongholdPiece, StrongholdStart};
    let mut out = Vec::new();
    let mut r = JavaRandom::new(seed);
    match mode {
        "grid" => {
            let mut w = World::new(seed, 300);
            let candidates = super::locations::strongholds(
                seed,
                &w.terrain.biome_source,
                &mut JavaRandom::new(0),
            );
            for (cx, cz) in candidates {
                for x in cx - 3..=cx + 3 {
                    for z in cz - 3..=cz + 3 {
                        w.load_chunk(x, z);
                    }
                }
            }
            out.extend(w.prand().state().to_be_bytes());
            out.extend(w.rand.state().to_be_bytes());
            int(&mut out, w.chunks.len() as i32);
            let mut keys: Vec<_> = w.chunks.keys().copied().collect();
            keys.sort();
            for (x, z) in keys {
                let c = w.chunk(x, z);
                int(&mut out, x);
                int(&mut out, z);
                out.push(c.populated as u8);
                out.extend(&c.blocks[..]);
                out.extend(&c.data.0[..]);
                out.extend(&c.sky_light.0[..]);
                out.extend(&c.block_light.0[..]);
                out.extend(&c.height_map[..]);
            }
            int(&mut out, w.strongholds.starts.len() as i32);
            for k in w.strongholds.keys() {
                out.extend(k.to_be_bytes());
                stronghold_pieces(&mut out, &w.strongholds.starts[&k]);
            }
        }
        "layout" => {
            for i in 0..16 {
                let s = StrongholdStart::new(&mut r, i % 4 - 2, i / 4 - 2);
                stronghold_pieces(&mut out, &s);
                out.extend(r.state().to_be_bytes());
            }
        }
        "map" => {
            let biomes = crate::BiomeSource::new(seed);
            let candidates = super::locations::strongholds(seed, &biomes, &mut JavaRandom::new(0));
            let mut g = StrongholdGenerator::default();
            let mut qs = vec![
                [0, 0],
                [40, -30],
                [-40, 30],
                [784425, 0],
                [-3, -784425],
                [40, -30],
            ];
            for (x, z) in candidates {
                qs.push([x, z]);
            }
            for (x, z) in candidates {
                qs.push([x + 7, z - 7]);
            }
            for [x, z] in qs {
                out.extend(g.generate(seed, x, z, &biomes).to_be_bytes());
                for (x, z) in g.candidates.unwrap() {
                    int(&mut out, x);
                    int(&mut out, z);
                }
                int(&mut out, g.starts.len() as i32);
                for key in g.keys() {
                    out.extend(key.to_be_bytes());
                    stronghold_pieces(&mut out, &g.starts[&key]);
                }
            }
        }
        "blocks" | "wet" => {
            let mut s = StrongholdStart::new(&mut r, 0, 0);
            stronghold_pieces(&mut out, &s);
            out.extend(r.state().to_be_bytes());
            let mut w = World::new(0, 300);
            w.sand_fall_instantly = true;
            stronghold_solid(&mut w);
            if mode == "wet" {
                for x in -80..=80 {
                    for z in -80..=80 {
                        w.chunk_mut(x >> 4, z >> 4).blocks
                            [((x & 15) << 11 | (z & 15) << 7 | 25) as usize] = 9;
                    }
                }
            }
            for cx in -7..7 {
                for cz in -7..7 {
                    let x = cx * 16 + 8;
                    let z = cz * 16 + 8;
                    let clip = B::region(x, z, x + 15, z + 15);
                    if s.bounds.intersects(clip) {
                        s.populate(&mut w, &mut r, clip);
                    }
                }
            }
            stronghold_pieces(&mut out, &s);
            out.extend(r.state().to_be_bytes());
            out.extend(w.rand.state().to_be_bytes());
            for cx in -7..=7 {
                for cz in -7..=7 {
                    let c = w.chunk(cx, cz);
                    out.extend(&c.blocks[..]);
                    out.extend(&c.data.0[..]);
                }
            }
            tiles(&mut out, &mut w, -112, 127);
        }
        "pieces" => {
            let sizes = [
                [5, 5, 7],
                [9, 5, 11],
                [5, 5, 5],
                [5, 5, 5],
                [11, 7, 11],
                [5, 11, 8],
                [5, 11, 5],
                [10, 9, 11],
                [14, 11, 15],
                [5, 5, 3],
                [14, 6, 15],
            ];
            let mut w = World::new(0, 300);
            w.sand_fall_instantly = true;
            for (k, [sx, sy, sz]) in sizes.into_iter().enumerate() {
                for d in 0..4 {
                    stronghold_solid(&mut w);
                    w.rand.set_seed(300);
                    let mut p = StrongholdPiece::new(
                        if k == 10 { 8 } else { k },
                        7,
                        B::oriented(0, 30, 0, 0, 0, 0, sx, sy, sz, d),
                        d,
                        &mut r,
                    );
                    stronghold_piece(&mut out, &p);
                    out.extend(r.state().to_be_bytes());
                    out.push(p.generate(&mut w, &mut r, B::new([-32, 0, -32, 6, 127, 32])) as u8);
                    out.push(p.generate(&mut w, &mut r, B::new([7, 0, -32, 32, 127, 32])) as u8);
                    stronghold_piece(&mut out, &p);
                    out.extend(r.state().to_be_bytes());
                    out.extend(w.rand.state().to_be_bytes());
                    for cx in -2..=2 {
                        for cz in -2..=2 {
                            let c = w.chunk(cx, cz);
                            out.extend(&c.blocks[..]);
                            out.extend(&c.data.0[..]);
                        }
                    }
                    tiles(&mut out, &mut w, -32, 47);
                }
            }
        }
        _ => panic!("unknown stronghold fixture {mode}"),
    }
    out
}
fn village_pieces(out: &mut Vec<u8>, s: &super::village::VillageStart) {
    bounds(out, s.bounds);
    int(out, s.pieces.len() as i32);
    for p in &s.pieces {
        text(out, p.name());
        int(out, p.component.depth);
        int(out, p.component.orientation);
        bounds(out, p.bounds());
        int(out, p.height);
        out.push(p.flag as u8);
        int(out, p.style);
        int(out, p.length);
    }
}
fn village(mode: &str, seed: i64) -> Vec<u8> {
    use super::village::{VillageGenerator, VillageStart};
    let mut out = Vec::new();
    let mut r = JavaRandom::new(seed);
    let biomes = crate::BiomeSource::structure_fixture_plains();
    if mode == "village-layout" {
        for i in 0..16 {
            let s = VillageStart::new(&biomes, &mut r, i % 4 - 2, i / 4 - 2);
            village_pieces(&mut out, &s);
            out.push(s.sizeable as u8);
            out.extend(r.state().to_be_bytes());
        }
    } else if mode == "village-map" {
        let mut g = VillageGenerator::default();
        let biomes = crate::BiomeSource::new(seed);
        let mut wr = JavaRandom::new(300);
        let mut queries = vec![
            [0, 0],
            [40, -30],
            [-40, 30],
            [784425, 0],
            [-3, -784425],
            [40, -30],
        ];
        for x in -3..=3 {
            for z in -3..=3 {
                let mut candidate = JavaRandom::new(
                    (x as i64)
                        .wrapping_mul(341873128712)
                        .wrapping_add((z as i64).wrapping_mul(132897987541))
                        .wrapping_add(seed)
                        .wrapping_add(10387312),
                );
                queries.push([
                    x * 32 + candidate.next_int(24),
                    z * 32 + candidate.next_int(24),
                ]);
            }
        }
        for [x, z] in queries {
            out.extend(g.generate(seed, x, z, &biomes, &mut wr).to_be_bytes());
            out.extend(wr.state().to_be_bytes());
            int(&mut out, g.starts.len() as i32);
            for key in g.keys() {
                out.extend(key.to_be_bytes());
                let s = &g.starts[&key];
                village_pieces(&mut out, s);
                out.push(s.sizeable as u8);
            }
        }
    } else {
        let mut s = VillageStart::new(&biomes, &mut r, 0, 0);
        village_pieces(&mut out, &s);
        out.extend(r.state().to_be_bytes());
        let mut w = World::new(0, 300);
        w.sand_fall_instantly = true;
        for cx in -7..=7 {
            for cz in -7..=7 {
                let mut bs = Box::new([0; 32768]);
                for x in 0..16 {
                    for z in 0..16 {
                        for y in 0..64 {
                            bs[x << 11 | z << 7 | y] = if y < 61 {
                                1
                            } else if y < 63 {
                                3
                            } else {
                                2
                            };
                        }
                    }
                }
                let mut c = Chunk::new(cx, cz, bs);
                c.populated = true;
                c.loaded = true;
                c.init_height_and_sky();
                w.chunks.insert((cx, cz), Box::new(c));
            }
        }
        for cx in -7..7 {
            for cz in -7..7 {
                let x = cx * 16 + 8;
                let z = cz * 16 + 8;
                let clip = B::region(x, z, x + 15, z + 15);
                if s.bounds.intersects(clip) {
                    s.pieces.retain_mut(|p| {
                        !p.bounds().intersects(clip) || p.generate(&mut w, &mut r, clip)
                    });
                }
            }
        }
        village_pieces(&mut out, &s);
        out.extend(r.state().to_be_bytes());
        out.extend(w.rand.state().to_be_bytes());
        for cx in -7..=7 {
            for cz in -7..=7 {
                let c = w.chunk(cx, cz);
                out.extend(&c.blocks[..]);
                out.extend(&c.data.0[..]);
            }
        }
        tiles(&mut out, &mut w, -112, 127);
    }
    out
}
pub fn tiles(out: &mut Vec<u8>, w: &mut World, min: i32, max: i32) {
    for x in min..=max {
        for z in min..=max {
            for y in 0..128 {
                let id = w.get_block_id(x, y, z);
                if id != 54 && id != 52 {
                    continue;
                }
                for n in [x, y, z, id as i32] {
                    int(out, n);
                }
                match w.get_block_tile_entity(x, y, z).unwrap() {
                    TileEntity::Chest(slots) => {
                        for s in slots.iter().take(27) { // getSizeInventory() is 27 (the backing array may be 36)
                            out.push(s.is_some() as u8);
                            if let Some(s) = s {
                                int(out, s.id as i32);
                                int(out, s.count as i32);
                                int(out, s.damage as i32);
                            }
                        }
                    }
                    TileEntity::MobSpawner { mob, delay } => {
                        text(out, mob);
                        int(out, *delay);
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
    int(out, i32::MIN);
}
pub fn component(seed: i64) -> Vec<u8> {
    use super::{component::Component, BlockSelector, StructurePieceTreasure as T};
    let mut out = Vec::new();
    let mut r = JavaRandom::new(seed);
    for d in -1..=4 {
        let p = Component {
            bounds: B::new([0, 64, 0, 10, 80, 10]),
            orientation: d,
            depth: 7,
        };
        for x in -5..=5 {
            for z in -5..=5 {
                let (x, y, z) = p.position(x, x, z);
                for n in [x, y, z] {
                    int(&mut out, n);
                }
            }
        }
        for id in 0..110 {
            for m in -1..17 {
                int(&mut out, p.rotate(id, m));
            }
        }
        let mut w = World::new(0, 300);
        w.sand_fall_instantly = true;
        for cx in -3..=3 {
            for cz in -3..=3 {
                let mut bs = Box::new([0; 32768]);
                for x in 0..16 {
                    for z in 0..16 {
                        for y in 0..=63 {
                            bs[x << 11 | z << 7 | y] = if y < 61 {
                                1
                            } else if y < 63 {
                                3
                            } else {
                                2
                            };
                        }
                    }
                }
                let mut c = Chunk::new(cx, cz, bs);
                c.populated = true;
                c.loaded = true;
                c.init_height_and_sky();
                w.chunks.insert((cx, cz), Box::new(c));
            }
        }
        let clip = B::new([-4, 0, -4, 14, 127, 14]);
        out.push(p.liquids(&mut w, clip) as u8);
        w.chunk_mut(0, 0).blocks[63] = 9;
        out.push(p.liquids(&mut w, clip) as u8);
        w.chunk_mut(0, 0).blocks[63] = 2;
        p.fill(&mut w, clip, [0, 0, 0, 10, 8, 10], 1, 0, false);
        p.fill_stones(
            &mut w,
            clip,
            [0, 0, 0, 10, 6, 10],
            true,
            &mut r,
            &mut BlockSelector::default(),
        );
        p.random_fill(&mut w, clip, &mut r, 0.7, [0, 1, 0, 10, 4, 10], 5, 0, false);
        p.random_put(&mut w, clip, &mut r, 1.0, 5, 2, 3, 3, 3);
        p.dome(&mut w, clip, [0, 1, 0, 10, 5, 10], 0, false);
        p.clear_up(&mut w, clip, 2, 0, 2);
        p.fill_down(&mut w, clip, 4, 0, 3, -1, 3);
        p.fill(&mut w, clip, [5, -1, 5, 7, -1, 7], 1, 1, false);
        p.door(&mut w, clip, 6, 0, 6, p.rotate(64, 1));
        p.chest(
            &mut w,
            clip,
            &mut r,
            8,
            0,
            8,
            &[
                T {
                    item: 257,
                    metadata: 0,
                    min: 2,
                    max: 4,
                    weight: 1,
                },
                T {
                    item: 265,
                    metadata: 2,
                    min: 3,
                    max: 8,
                    weight: 3,
                },
            ],
            12,
        );
        int(&mut out, p.get(&mut w, clip, 6, 0, 6));
        int(&mut out, p.get(&mut w, clip, 99, 0, 99));
        out.extend(r.state().to_be_bytes());
        out.extend(w.rand.state().to_be_bytes());
        for cx in -3..=3 {
            for cz in -3..=3 {
                let c = w.chunk(cx, cz);
                out.extend(&c.blocks[..]);
                out.extend(&c.data.0[..]);
            }
        }
        tiles(&mut out, &mut w, -48, 63);
    }
    out
}
