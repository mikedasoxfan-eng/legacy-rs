//! Fixtures shared with Mode_features181. Only results, metadata and RNG states are compared.
use crate::{
    features::{self, TreeKind},
    features181,
    world::{chunk::Chunk, World},
    JavaRandom,
};
pub fn starts(seed: i64) -> Vec<u8> {
    use crate::structures::{BoundingBox as B, Piece, StructureStart};
    struct Fixture {
        id: i32,
        bounds: B,
    }
    impl Piece for Fixture {
        fn bounds(&self) -> B {
            self.bounds
        }
        fn bounds_mut(&mut self) -> &mut B {
            &mut self.bounds
        }
        fn component_type(&self) -> i32 {
            self.id
        }
        fn generate(&mut self, _w: &mut World, r: &mut JavaRandom, _clip: B) -> bool {
            r.next_bool()
        }
    }
    let mut r = JavaRandom::new(seed);
    let mut w = World::new(0, 300);
    let mut out = Vec::new();
    fn ints(o: &mut Vec<u8>, v: &[i32]) {
        for n in v {
            o.extend_from_slice(&n.to_be_bytes());
        }
    }
    for k in 0..64 {
        let mut s = StructureStart::default();
        for id in 0..4 {
            let x = r.next_int(32) - 16;
            let y = r.next_int(32);
            let z = r.next_int(32) - 16;
            let dx = r.next_int(8);
            let dy = r.next_int(8);
            let dz = r.next_int(8);
            s.pieces.push(Box::new(Fixture {
                id,
                bounds: B::new([x, y, z, x + dx, y + dy, z + dz]),
            }));
        }
        s.update_bounding_box();
        ints(&mut out, &s.bounding_box.unwrap().values());
        s.move_below_sea_level(&mut r, k % 17);
        ints(&mut out, &s.bounding_box.unwrap().values());
        out.push(s.sizeable() as u8);
        s.generate(&mut w, &mut r, B::new([-8, 0, -8, 8, 127, 8]));
        ints(&mut out, &[s.pieces.len() as i32]);
        for p in &s.pieces {
            ints(&mut out, &[p.component_type()]);
            ints(&mut out, &p.bounds().values());
        }
        out.extend_from_slice(&r.state().to_be_bytes());
    }
    out
}
pub fn cache(seed: i64) -> Vec<u8> {
    let mut out = Vec::new();
    for scenario in 0..3 {
        let mut m = crate::BiomeSource::new(seed);
        m.biome_at(-32, 16);
        m.raw_temperature(0, 0);
        m.raw_rainfall(64, -48);
        let now = 100000;
        m.cache_fixture_times(
            if scenario == 0 {
                now - 10000
            } else if scenario == 1 {
                now
            } else {
                now + 10000
            },
            &[
                ((-2, 1), now - 40000),
                ((0, 0), now - 1000),
                ((4, -3), now + 100000),
            ],
        );
        m.cleanup_cache_at(now);
        let keys = m.cache_snapshot();
        out.extend_from_slice(&(keys.len() as i32).to_be_bytes());
        for (x, z) in keys {
            out.extend_from_slice(&x.to_be_bytes());
            out.extend_from_slice(&z.to_be_bytes());
        }
    }
    for id in [8, 9] {
        let mut m = if id == 8 {
            crate::BiomeSource::hell()
        } else {
            crate::BiomeSource::sky()
        };
        let mut r = JavaRandom::new(seed);
        for radius in [0, 1, 16, 256] {
            for include in [true, false] {
                let allowed = [crate::Biome::from_id(if include { id } else { 4 })];
                let p = m.find_position(-17, 31, radius, &allowed, &mut r);
                out.push(p.is_some() as u8);
                if let Some((x, z)) = p {
                    out.extend_from_slice(&x.to_be_bytes());
                    out.extend_from_slice(&z.to_be_bytes());
                }
                out.extend_from_slice(&r.state().to_be_bytes());
                out.push(m.viable(-17, 31, radius, &allowed) as u8);
            }
        }
        let mut t = Vec::new();
        m.temperatures(&mut t, -21, 35, 7, 9);
        for v in t {
            out.extend_from_slice(&(v as f32).to_be_bytes());
        }
        for v in m.rainfalls(-21, 35, 7, 9) {
            out.extend_from_slice(&(v as f32).to_be_bytes());
        }
        out.extend(m.block_biomes(-21, 35, 7, 9).into_iter().map(|b| b as u8));
    }
    out
}
pub fn utilities(seed: i64) -> Vec<u8> {
    use crate::{
        structures::BoundingBox as B,
        weighted::{self, WeightedRandomChoice as C},
    };
    let mut r = JavaRandom::new(seed);
    let mut out = Vec::new();
    fn ints(o: &mut Vec<u8>, v: &[i32]) {
        for n in v {
            o.extend_from_slice(&n.to_be_bytes());
        }
    }
    for k in 0..512 {
        let mut v = [0; 10];
        for n in &mut v {
            *n = r.next_int_unbounded();
        }
        v[9] = k % 6 - 1;
        let mut b = B::oriented(v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9]);
        ints(&mut out, &b.values());
        let c = B::new(v[..6].try_into().unwrap());
        out.extend([
            b.intersects(c) as u8,
            b.intersects_region(v[0], v[2], v[3], v[5]) as u8,
            b.contains(v[6], v[7], v[8]) as u8,
        ]);
        b.encompass(c);
        b.offset(v[6], v[7], v[8]);
        ints(&mut out, &b.values());
        ints(&mut out, &b.sizes());
    }
    ints(&mut out, &B::empty().values());
    ints(&mut out, &B::region(-13, 31, 44, 55).values());
    for id in 0..10 {
        let b = crate::Biome::from_id(id);
        out.extend_from_slice(&(b.name().len() as u16).to_be_bytes());
        out.extend_from_slice(b.name().as_bytes());
        ints(&mut out, &[b.color(), b.foliage_color()]);
        out.extend([b.top_block(), b.filler_block()]);
        for n in [b.heights().0, b.heights().1, b.temperature(), b.rainfall()] {
            out.extend_from_slice(&n.to_be_bytes());
        }
        out.extend([b.enable_snow() as u8, b.enable_rain() as u8]);
        for list in [
            weighted::creatures(b),
            weighted::monsters(b),
            weighted::water_creatures(b),
        ] {
            ints(&mut out, &[list.len() as i32]);
            for e in list {
                out.extend_from_slice(&((if e.entity == "LavaSlime" { "MagmaCube" } else { e.entity }).len() as u16).to_be_bytes());
                out.extend_from_slice((if e.entity == "LavaSlime" { "MagmaCube" } else { e.entity }).as_bytes());
                ints(&mut out, &[e.weight, e.min, e.max]);
            }
        }
    }
    for weights in [
        vec![12, 10, 10, 8],
        vec![0, 1, 0, 9],
        vec![-3, 8, 2],
        vec![i32::MAX, i32::MAX, 5],
    ] {
        let choices: Vec<_> = weights.into_iter().map(|weight| C { weight }).collect();
        ints(
            &mut out,
            &[weighted::total(&choices), weighted::total(&choices)],
        );
        for _ in 0..128 {
            let chosen = weighted::choose_with_total(&mut r, &choices, 100);
            let index = chosen
                .map(|c| choices.iter().position(|e| std::ptr::eq(e, c)).unwrap() as i32)
                .unwrap_or(-1);
            ints(&mut out, &[index]);
        }
    }
    out.extend_from_slice(&r.state().to_be_bytes());
    let mut stones = crate::structures::BlockSelector::default();
    for k in 0..256 {
        let x = r.next_int_unbounded();
        let z = r.next_int_unbounded();
        let p = crate::structures::ChunkCoordIntPair { x, z };
        out.extend_from_slice(&p.packed().to_be_bytes());
        ints(&mut out, &[p.java_hash()]);
        out.push((p == crate::structures::ChunkCoordIntPair { x, z }) as u8);
        stones.stronghold_stone(&mut r, x, k, z, k % 3 != 0);
        ints(&mut out, &[stones.block, stones.metadata]);
        let weight = r.next_int_unbounded();
        let limit = r.next_int(5);
        let spawned = r.next_int(8);
        let w = crate::structures::PieceWeight {
            piece_type: "ComponentStrongholdLibrary",
            weight,
            limit,
            spawned,
        };
        let good = w.can_spawn(k % 10);
        out.extend([
            good as u8,
            good as u8,
            good as u8,
            good as u8,
            w.can_spawn_library(k % 10) as u8,
        ]);
        let t = crate::structures::StructurePieceTreasure {
            item: x,
            metadata: z,
            min: k,
            max: limit,
            weight,
        };
        ints(&mut out, &[t.item, t.metadata, t.min, t.max, t.weight]);
    }
    for d in [
        crate::structures::Door::Opening,
        crate::structures::Door::WoodDoor,
        crate::structures::Door::Grates,
        crate::structures::Door::IronDoor,
    ] {
        ints(&mut out, &[d as i32, d.switch_index()]);
    }
    out.extend_from_slice(&r.state().to_be_bytes());
    out
}
pub fn biomes(seed: i64) -> Vec<u8> {
    crate::intcache::CACHE.with(|c| *c.borrow_mut() = Default::default());
    let mut m = crate::BiomeSource::new(seed);
    let mut out = Vec::new();
    for [x, z, w, h] in [
        [0, 0, 16, 16],
        [-19, 31, 17, 13],
        [-1, -1, 1, 1],
        [512, -1024, 32, 16],
        [-33, -41, 9, 7],
    ] {
        out.extend(m.block_biomes(x, z, w, h).into_iter().map(|b| b as u8));
        let mut t = Vec::new();
        m.temperatures(&mut t, x, z, w, h);
        for v in t {
            out.extend_from_slice(&(v as f32).to_be_bytes());
        }
        for v in m.rainfalls(x, z, w, h) {
            out.extend_from_slice(&(v as f32).to_be_bytes());
        }
        out.extend(m.generation_biomes(x, z, w, h).into_iter().map(|b| b as u8));
    }
    let allowed = [
        crate::Biome::Forest,
        crate::Biome::Swampland,
        crate::Biome::Taiga,
    ];
    let mut rand = JavaRandom::new(seed);
    for radius in [4, 64, 256, 16, 0] {
        let p = m.find_position10(-17, 31, radius, &allowed, &mut rand);
        out.push(p.is_err() as u8);
        let p = p.unwrap_or(None);
        out.push(p.is_some() as u8);
        if let Some((x, z)) = p {
            out.extend_from_slice(&x.to_be_bytes());
            out.extend_from_slice(&z.to_be_bytes());
        }
        out.extend_from_slice(&rand.state().to_be_bytes());
        out.push(m.viable(-17, 31, radius, &allowed) as u8);
    }
    for [x, z] in [[-21, 79], [-21, 79], [1025, -512], [0, 0]] {
        out.push(m.biome_at(x, z) as u8);
        out.extend_from_slice(&(m.raw_temperature(x, z) as f32).to_be_bytes());
    }
    out
}
pub fn feature(name: &str, seed: i64, fixture: i32) -> Vec<u8> {
    let mut w = if name == "sky" {
        World::with_dimension(seed, 300, crate::world::Dimension::Sky)
    } else {
        World::new(0, 300)
    };
    w.sand_fall_instantly = true;
    for cx in -3..=3 {
        for cz in -3..=3 {
            let mut blocks = vec![0; 32768];
            for x in 0..16 {
                for z in 0..16 {
                    for y in 0..=63 {
                        blocks[(x << 11) | (z << 7) | y] = if y < 61 {
                            1
                        } else if y < 63 {
                            3
                        } else {
                            2
                        };
                    }
                }
            }
            let mut c = Chunk::new(cx, cz, blocks.into_boxed_slice().try_into().unwrap());
            c.populated = true;
            c.loaded = true;
            w.chunks.insert((cx, cz), Box::new(c));
        }
    }
    if fixture == 1 {
        for x in 3..=13 {
            for z in 3..=13 {
                let c = w.chunk_mut(x >> 4, z >> 4);
                c.blocks[((x & 15) << 11 | (z & 15) << 7 | 64) as usize] = 9;
            }
        }
    }
    if fixture == 2 {
        w.chunk_mut(0, 0).blocks[8 << 11 | 8 << 7 | 66] = 1;
    }
    if fixture == 4 {
        for x in 10..=13 {
            for y in 32..=33 {
                w.chunk_mut(0, 0).blocks[x << 11 | 8 << 7 | y] = 0;
            }
        }
    }
    for cx in -3..=3 {
        for cz in -3..=3 {
            w.chunk_mut(cx, cz).init_height_and_sky();
        }
    }
    if name != "sky" { *w.prand() = JavaRandom::new(seed); }
    let y = if name == "ore" || name == "dungeon" {
        32
    } else if fixture == 3 {
        123
    } else {
        64
    };
    let result = if name == "sky" {
        crate::end::populate(&mut w, 0, 0);
        true
    } else if let Some(id) = name.strip_prefix("animals") {
        crate::populate::spawn_population(
            &mut w,
            crate::Biome::from_id(id.parse().unwrap()),
            8,
            8,
            16,
            16,
        );
        true
    } else if let Some(id) = name.strip_prefix("decorate") {
        crate::populate::decorate(&mut w, crate::Biome::from_id(id.parse().unwrap()), 0, 0);
        true
    } else {
        match name {
            "oak" => features::tree(&mut w, TreeKind::Oak, 8, y, 8),
            "birch" => features::tree(&mut w, TreeKind::Birch, 8, y, 8),
            "taiga1" => features::tree(&mut w, TreeKind::Spruce1, 8, y, 8),
            "taiga2" => features::tree(&mut w, TreeKind::Spruce2, 8, y, 8),
            "big" => features::tree(&mut w, TreeKind::BigOak, 8, y, 8),
            "swamp" => features181::swamp_tree(&mut w, 8, y, 8),
            "brown" => features181::big_mushroom(&mut w, Some(0), 8, y, 8),
            "red" => features181::big_mushroom(&mut w, Some(1), 8, y, 8),
            "clay" => features::clay(&mut w, 4, 8, y, 8),
            "sand" => features::sand(&mut w, 7, 12, 8, y, 8),
            "gravel" => features::sand(&mut w, 6, 13, 8, y, 8),
            "ore" => features::minable(&mut w, 14, 8, 8, y, 8),
            "lake" => features::lakes(&mut w, 9, 8, y, 8),
            "lava" => features::lakes(&mut w, 11, 8, y, 8),
            "dungeon" | "dungeontiles" => features::dungeon(&mut w, 8, 32, 8),
            _ => panic!("unknown feature {name}"),
        }
    };
    let mut out = vec![result as u8];
    out.extend_from_slice(&w.prand().state().to_be_bytes());
    out.extend_from_slice(&w.rand.state().to_be_bytes());
    for cx in -3..=3 {
        for cz in -3..=3 {
            let c = w.chunk(cx, cz);
            out.extend_from_slice(&c.blocks[..]);
            out.extend_from_slice(&c.data.0[..]);
        }
    }
    let entities: Vec<_> = w.loaded_entity_list.iter().map(|id| &w.entities[id]).collect();
    out.extend_from_slice(&(entities.len() as i32).to_be_bytes());
    for e in entities {
        let name = e.kind.name();
        out.extend_from_slice(&(name.len() as u16).to_be_bytes());
        out.extend_from_slice(name.as_bytes());
        for v in [e.pos.x, e.pos.y, e.pos.z] {
            out.extend_from_slice(&v.to_be_bytes());
        }
        out.extend_from_slice(&e.yaw.to_be_bytes());
        let color = if let crate::entity::EntityKind::Sheep { fleece_color, .. } = e.kind {
            fleece_color as i32
        } else {
            -1
        };
        out.extend_from_slice(&color.to_be_bytes());
    }
    if name == "dungeontiles" {
        crate::structures::verify::tiles(&mut out, &mut w, -48, 63);
    }
    out
}
pub fn round2(mode: &str, seed: i64) -> Vec<u8> {
    match mode {
        "lifetime" => [seed, 3, 4, 5]
            .into_iter()
            .flat_map(|s| feature("decorate4", s, 0))
            .collect(),
        "dungeon" => feature("dungeontiles", seed, 4),
        "sky" => feature("sky", seed, 0),
        "locations" => {
            let mut out = Vec::new();
            let mut r = JavaRandom::new(0);
            let biomes = crate::BiomeSource::new(seed);
            for (x, z) in crate::structures::locations::strongholds(seed, &biomes, &mut r) {
                out.extend(x.to_be_bytes());
                out.extend(z.to_be_bytes());
            }
            out.extend(r.state().to_be_bytes());
            out
        }
        "skygrid" => {
            let mut out = Vec::new();
            let mut w = World::with_dimension(seed, 300, crate::world::Dimension::Sky);
            for x in -2..=2 {
                for z in -2..=2 {
                    w.load_chunk(x, z);
                }
            }
            out.extend(w.prand().state().to_be_bytes());
            out.extend(w.rand.state().to_be_bytes());
            out.extend((w.chunks.len() as i32).to_be_bytes());
            let mut keys: Vec<_> = w.chunks.keys().copied().collect();
            keys.sort();
            for (x, z) in keys {
                let c = w.chunk(x, z);
                out.extend(x.to_be_bytes());
                out.extend(z.to_be_bytes());
                out.push(c.populated as u8);
                out.extend(&c.blocks[..]);
                out.extend(&c.data.0[..]);
                out.extend(&c.sky_light.0[..]);
                out.extend(&c.block_light.0[..]);
                out.extend(&c.height_map[..]);
            }
            out
        }
        "colors" => {
            let mut out = Vec::new();
            for id in 0..16 {
                for t in [-100.0, -3.0, -1.0, 0.0, 0.5, 1.0, 3.0, 100.0] {
                    out.extend(crate::Biome::from_id(id).sky_color_by_temp(t).to_be_bytes());
                }
            }
            for time in [-1, 0, 6000, 12000, 18000, 24000] {
                for partial in [0.0, 0.5, 1.0] {
                    out.extend(
                        crate::world::Dimension::Sky
                            .celestial_angle(time, partial)
                            .to_be_bytes(),
                    );
                }
            }
            out
        }
        _ => panic!("unknown round2 fixture"),
    }
}
