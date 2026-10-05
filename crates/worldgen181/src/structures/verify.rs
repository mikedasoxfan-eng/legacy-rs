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
                        for s in slots.iter() {
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
