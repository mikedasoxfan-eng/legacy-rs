//! Raw vertex comparison against TileEntityRendererPiston, including fractional
//! translations, short/long rods, retraction bases, colors and atlas UVs.
use super::{access::Access, blocks::Colorizers, pistons::Snapshot};
use std::cell::RefCell;
use worldgen::{
    world::{
        chunk::{idx, Chunk},
        pistons::Piston,
    },
    World,
};
struct Input {
    data: Vec<u8>,
    at: usize,
}
impl Input {
    fn u(&mut self) -> u32 {
        let v = u32::from_le_bytes(self.data[self.at..self.at + 4].try_into().unwrap());
        self.at += 4;
        v
    }
    fn f(&mut self) -> f32 {
        f32::from_bits(self.u())
    }
    fn d(&mut self) -> f64 {
        let v = u64::from_le_bytes(self.data[self.at..self.at + 8].try_into().unwrap());
        self.at += 8;
        f64::from_bits(v)
    }
}
pub fn check(path: &str) -> bool {
    let mut input = Input {
        data: std::fs::read(path).expect("piston renderer dump"),
        at: 8,
    };
    assert_eq!(&input.data[..8], b"B181PST1");
    let count = input.u();
    let assets = crate::assets::load().expect("Minecraft assets");
    let col = Colorizers {
        grass: assets.grass_color,
        foliage: assets.foliage_color,
    };
    let mut world = World::new(0, 300);
    world.skylight_subtracted = 3;
    for cx in -3..=3 {
        for cz in -3..=3 {
            let mut c = Chunk::new(cx, cz, Box::new([0; 32768]));
            for x in 0..16 {
                for z in 0..16 {
                    c.blocks[idx(x, 63, z)] = 1;
                    for y in 0..128 {
                        let (wx, wz) = (cx * 16 + x, cz * 16 + z);
                        c.sky_light.set(x, y, z, (wx * 7 + y * 3 + wz * 5) & 15);
                        c.block_light.set(x, y, z, (wx * 3 + y * 11 + wz) & 15);
                    }
                }
            }
            world.chunks.insert((cx, cz), Box::new(c));
        }
    }
    let biomes = RefCell::new(worldgen::BiomeSource::new(0));
    let mut bad = 0;
    let mut words = 0;
    for case in 0..count {
        let (id, meta, facing, extending, head) = (
            input.u() as u8,
            input.u() as i32,
            input.u() as i32,
            input.u() != 0,
            input.u() != 0,
        );
        let (progress, last_progress, partial) = (input.f(), input.f(), input.f());
        let position = [input.u() as i32, input.u() as i32, input.u() as i32];
        let (ao, fancy) = (input.u() != 0, input.u() != 0);
        let render_position = [input.d(), input.d(), input.d()];
        let [x, y, z] = position;
        let c = world.chunks.get_mut(&(x >> 4, z >> 4)).unwrap();
        c.blocks[idx(x & 15, y, z & 15)] = 36;
        c.data.set(x & 15, y, z & 15, meta);
        let mut access = Access::new(&world, x >> 4, z >> 4, 3, &biomes);
        access.fancy = fancy;
        let mut p = Piston::new(id, meta, facing, extending, head);
        p.progress = progress;
        p.last_progress = last_progress;
        let snapshot = Snapshot::new(&access, &col, ao, position, p);
        let verts = snapshot.vertices(partial, render_position);
        let n = input.u() as usize;
        if n != verts.len() {
            if bad < 10 {
                eprintln!("case {case} id {id} facing {facing} progress {progress}/{last_progress}/{partial}: java {n} vertices rust {}",verts.len());
            }
            bad += 1;
        }
        for v in 0..n {
            let expected = [
                input.u(),
                input.u(),
                input.u(),
                input.u(),
                input.u(),
                input.u(),
                input.u(),
            ];
            if let Some(r) = verts.get(v) {
                let actual = [
                    r.pos[0].to_bits(),
                    r.pos[1].to_bits(),
                    r.pos[2].to_bits(),
                    r.uv[0].to_bits(),
                    r.uv[1].to_bits(),
                    u32::from_le_bytes(r.color),
                    r.light,
                ];
                for k in 0..7 {
                    words += 1;
                    if actual[k] != expected[k] {
                        if bad < 10 {
                            eprintln!("case {case} id {id}/{meta} facing {facing} head {head} ext {extending} progress {progress}/{last_progress}/{partial} ao {ao} fancy {fancy} vertex {v} field {k}: java {:08x}, rust {:08x}",expected[k],actual[k]);
                        }
                        bad += 1;
                    }
                }
            }
        }
    }
    assert_eq!(input.at, input.data.len());
    println!("piston renderer: {count} cases, {words} vertex words, {bad} mismatches");
    bad == 0
}
