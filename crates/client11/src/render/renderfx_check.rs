//! renderfx round 1: raw-bit comparison against Mode_render_simple10, the
//! original 1.0 simple entity renderers (mob pass callbacks and their lightmap
//! coordinates, RenderXPOrb, RenderPainting, RenderFallingSand).
use super::{
    access::Access,
    blocks::Colorizers,
    entity::{
        fixed::{Fixed, Op},
        living::{MobKind, RenderState},
        objects::{self, ObjectKind, ObjectState},
        raw_living181,
    },
    weather::ClientRandom,
};
use std::cell::RefCell;
use worldgen::{
    world::chunk::{idx, Chunk},
    World,
};

struct Dump {
    bytes: Vec<u8>,
    at: usize,
    checks: usize,
    bad: usize,
}
impl Dump {
    fn take<const N: usize>(&mut self) -> [u8; N] {
        let r = self.bytes[self.at..self.at + N].try_into().expect("truncated renderfx dump");
        self.at += N;
        r
    }
    fn int(&mut self) -> i32 {
        i32::from_le_bytes(self.take())
    }
    fn long(&mut self) -> i64 {
        i64::from_le_bytes(self.take())
    }
    fn float(&mut self) -> f32 {
        f32::from_bits(self.int() as u32)
    }
    fn double(&mut self) -> f64 {
        f64::from_bits(self.long() as u64)
    }
    fn equal(&mut self, j: u64, r: u64, path: &str) {
        self.checks += 1;
        if j != r {
            if self.bad < 16 {
                eprintln!("{path}: Java {j:016x}, Rust {r:016x}");
            }
            self.bad += 1;
        }
    }
    fn f(&mut self, r: f32, path: &str) {
        let j = self.int() as u32;
        self.equal(j as u64, r.to_bits() as u64, path);
    }
    fn ops(&mut self, ops: &[Op], path: &str) {
        let n = self.int() as usize;
        self.equal(n as u64, ops.len() as u64, &format!("{path}: operation count"));
        if n != ops.len() {
            panic!("{path}: operation count Java {n}, Rust {}", ops.len());
        }
        for op in ops {
            let code = self.int() as u32;
            self.equal(code as u64, op.code as u64, &format!("{path}: operation"));
            for a in op.args {
                self.f(a, path);
            }
        }
    }
    /// Draw10 records with the lightmap coordinates in force at each draw, then
    /// the final GL state and lightmap.
    fn draws(&mut self, g: &Fixed, lightmaps: &[[f32; 2]], last: [f32; 2], path: &str) {
        let n = self.int() as usize;
        if n != g.draws.len() {
            let mut runs: Vec<(u32, u32, [u32; 4], usize)> = Vec::new();
            for v in &g.draws {
                let key = (v.texture, v.flags, v.material.map(f32::to_bits));
                match runs.last_mut() { Some(r) if (r.0, r.1, r.2) == key => r.3 += 1, _ => runs.push((key.0, key.1, key.2, 1)) }
            }
            eprintln!("{path}: Rust draws (texture, flags, material, count): {runs:?}");
        }
        assert_eq!(n, g.draws.len(), "{path}: draw count");
        for (i, v) in g.draws.iter().enumerate() {
            let path = format!("{path}.draw[{i}]");
            self.ops(&v.ops, &path);
            for actual in [v.cull as u32, v.flags, v.texture, v.src, v.dst, v.depth, v.mode] {
                let expected = self.int() as u32;
                self.equal(expected as u64, actual as u64, &path);
            }
            self.f(v.alpha_cutoff, &path);
            for value in v.material {
                self.f(value, &path);
            }
            assert_eq!(self.int() as usize, v.vertices.len(), "{path}: vertices");
            for vertex in &v.vertices {
                for actual in [vertex.normal, vertex.color] {
                    let expected = self.int() as u32;
                    self.equal(expected as u64, actual as u64, &path);
                }
                for f in vertex.pos.into_iter().chain(vertex.uv) {
                    self.f(f, &path);
                }
            }
            for value in lightmaps[i] {
                self.f(value, &format!("{path}.lightmap"));
            }
        }
        self.ops(&g.ops, path);
        for actual in [g.flags, g.src, g.dst, g.depth] {
            let expected = self.int() as u32;
            self.equal(expected as u64, actual as u64, path);
        }
        self.f(g.alpha_cutoff, path);
        for value in g.material {
            self.f(value, path);
        }
        for value in last {
            self.f(value, &format!("{path}.lightmap"));
        }
    }
}

fn coordinates(packed: i32) -> [f32; 2] {
    [(packed % 65536) as f32, (packed / 65536) as f32]
}

/// `--renderfx-simple-check`: one dump of Mode_render_simple10.
pub fn simple_check(path: &str) -> bool {
    let mut d = Dump { bytes: std::fs::read(path).expect("read renderfx dump"), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.int(), 0x58465231);
    assert_eq!(d.int(), 1);
    let kind = d.int();
    let name = match kind {
        0..=7 => {
            mobs(&mut d, kind);
            ["RenderSheep", "RenderPig", "RenderCreeper", "RenderSpider", "RenderSpider (cave spider)", "RenderSlime", "RenderEnderman", "RenderSilverfish"][kind as usize]
        }
        8 => {
            orbs(&mut d);
            "RenderXPOrb"
        }
        9 => {
            paintings(&mut d);
            "RenderPainting"
        }
        10 => {
            falling(&mut d);
            "RenderFallingSand"
        }
        _ => panic!("renderfx kind {kind}"),
    };
    assert_eq!(d.at, d.bytes.len(), "renderfx dump length");
    println!(
        "{name} {}: {} raw-bit comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

fn mobs(d: &mut Dump, kind: i32) {
    let mob = [
        MobKind::Sheep,
        MobKind::Pig,
        MobKind::Creeper,
        MobKind::Spider,
        MobKind::CaveSpider,
        MobKind::Slime,
        MobKind::Enderman,
        MobKind::Silverfish,
    ][kind as usize];
    let shadow = [0.7f32, 0.7, 0.5, 1.0, 1.0, 0.25, 0.5, 0.3][kind as usize];
    d.f(shadow, "shadow size");
    for c in 0..64 {
        let mut s = RenderState::new(mob);
        s.position = std::array::from_fn(|_| d.double());
        let [partial, body, yaw, pitch, amount, swing, attack, flash, squish0, squish1] =
            std::array::from_fn(|_| d.float());
        let [ticks, hurt, death, light, start, a, b, child, attacking] = std::array::from_fn(|_| d.int());
        let seed = d.long();
        s.partial = partial;
        s.body_yaw = body;
        s.yaw = yaw;
        s.pitch = pitch;
        s.limb_amount = amount;
        s.limb_swing = swing - amount * (1.0 - partial);
        s.attack = attack;
        s.ticks = ticks;
        s.age = ticks as f32 + partial;
        s.hurt_time = hurt;
        s.death_time = death;
        s.overlay_brightness = 0.73;
        s.is_child = child != 0;
        match mob {
            MobKind::Sheep => {
                s.fleece_color = a as usize;
                s.sheared = b != 0;
            }
            MobKind::Pig => s.saddled = a != 0,
            MobKind::Creeper => {
                s.powered = a != 0;
                s.creeper_flash = flash;
            }
            MobKind::Slime => {
                s.slime_size = a;
                s.slime_squish = squish0 + (squish1 - squish0) * partial;
            }
            MobKind::Enderman => {
                s.carried_block = a as u8;
                s.carried_data = b;
                s.carrying = a > 0;
                s.screaming = attacking != 0;
                if s.screaming {
                    // RenderEnderman.rnd: two Gaussians scaled by 0.02 (the fixture seeds it).
                    let mut r = ClientRandom::new(seed);
                    let x = r.gaussian() * 0.02;
                    let z = r.gaussian() * 0.02;
                    s.scream_jitter = [x, z];
                }
            }
            _ => {}
        }
        let g = raw_living181::render(&s, [0.0; 3], false);
        // RenderManager sets the entity's own lightmap coordinates before doRender
        // (`start` here); the eye passes switch to 61680 and the carried block back
        // to the entity's packed brightness.
        let packed = raw_living181::packed_brightnesses(&g.draws, light);
        let first = g.draws.iter().position(|v| matches!(v.texture, 1 | 41 | 44 | 71)).unwrap_or(g.draws.len());
        let lightmaps: Vec<_> = packed
            .iter()
            .enumerate()
            .map(|(i, &v)| coordinates(if i < first { start } else { v }))
            .collect();
        let last = lightmaps.last().copied().unwrap_or(coordinates(start));
        d.draws(&g, &lightmaps, last, &format!("mob {kind} case {c}"));
    }
}

fn orbs(d: &mut Dump) {
    for c in 0..64 {
        let value = d.int();
        let color = d.int();
        let packed = d.int();
        let position = std::array::from_fn(|_| d.double());
        let view = [d.float(), d.float()];
        let partial = d.float();
        // getEntityBrightness is 0.37 in the fixture: 1.0 must not read it.
        let s = ObjectState { kind: ObjectKind::XPOrb { value, ticks: color }, position, partial, brightness: 0.37 };
        let g = objects::render(&s, [0.0; 3], view);
        // The orb sets its own packed brightness, the value RenderManager.renderEntity
        // already uses for every entity draw.
        let lightmaps = vec![coordinates(packed); g.draws.len()];
        d.draws(&g, &lightmaps, coordinates(packed), &format!("xp orb case {c}"));
    }
}

fn paintings(d: &mut Dump) {
    let n = d.int();
    let mut w = worldgen::decoration_verify::flat_world();
    for case in 0..n {
        let art = d.int() as usize;
        let dir = d.int();
        let mut e = worldgen::entity::Entity::painting_base(&mut w);
        if let worldgen::entity::EntityKind::Painting(s) = &mut e.kind {
            s.art = art;
            s.tile = [0, 67, 0];
        }
        e.set_painting_direction(dir);
        let a = worldgen::entity::painting::ARTS[art];
        let lights: Vec<i32> = (0..a.width / 16 * (a.height / 16)).map(|_| d.int()).collect();
        let g = crate::render::painting::render(&e, [-0.25, 10.5, 3.75], &lights);
        let last = g.draw_lightmaps.last().copied().unwrap_or([-1.0; 2]);
        d.draws(&g, &g.draw_lightmaps.clone(), last, &format!("painting {case}"));
    }
}

fn falling(d: &mut Dump) {
    let count = d.int();
    let assets = crate::assets::load().expect("Minecraft assets");
    let col = Colorizers { grass: assets.grass_color, foliage: assets.foliage_color };
    let mut world = World::new(0, 300);
    world.skylight_subtracted = 3;
    for cx in -3..=3 {
        for cz in -3..=3 {
            let mut c = Chunk::new(cx, cz, Box::new([0; 32768]));
            for x in 0..16 {
                for z in 0..16 {
                    c.blocks[idx(x, 63, z)] = 1;
                    if (-2..=1).contains(&cx) && (-2..=1).contains(&cz) {
                        for y in 0..128 {
                            let (wx, wz) = (cx * 16 + x, cz * 16 + z);
                            c.sky_light.set(x, y, z, (wx * 7 + y * 3 + wz * 5) & 15);
                            c.block_light.set(x, y, z, (wx * 3 + y * 11 + wz) & 15);
                        }
                    }
                }
            }
            world.chunks.insert((cx, cz), Box::new(c));
        }
    }
    for x in -3i32..=3 {
        for z in -3i32..=3 {
            if (x * 5 + z * 3) & 3 == 0 {
                let id = [1u8, 20, 12, 49][((x + z + 8) % 4) as usize];
                let y = 70 + ((x + z) & 1);
                let c = world.chunks.get_mut(&(x >> 4, z >> 4)).unwrap();
                c.blocks[idx(x & 15, y, z & 15)] = id;
            }
        }
    }
    let biomes = RefCell::new(worldgen::BiomeSource::new(0));
    for case in 0..count {
        let id = d.int() as u8;
        let ao = d.int() != 0;
        let pos: [f64; 3] = std::array::from_fn(|_| d.double());
        let _relative: [f64; 3] = std::array::from_fn(|_| d.double());
        let n = d.int() as usize;
        let packed_flag = d.int();
        let cell = pos.map(worldgen::math::floor_double);
        let verts: Vec<[u32; 7]> = if id == crate::render::falling::DRAGON_EGG {
            let mut access = Access::new(&world, cell[0] >> 4, cell[2] >> 4, 3, &biomes);
            access.fancy = true;
            crate::render::falling::egg_vertices(&access, &col, ao, pos)
                .into_iter()
                .map(|v| [v.pos[0].to_bits(), v.pos[1].to_bits(), v.pos[2].to_bits(), v.uv[0].to_bits(), v.uv[1].to_bits(), u32::from_le_bytes(v.color), v.light])
                .collect()
        } else {
            let light = world.packed_light181(cell[0], cell[1], cell[2], worldgen::block::light_value(id)) as u32;
            let mut g = Fixed::new(1.0);
            super::entity::inventory::falling(&mut g, id);
            g.draws
                .iter()
                .flat_map(|v| v.vertices.iter())
                .map(|v| [v.pos[0].to_bits(), v.pos[1].to_bits(), v.pos[2].to_bits(), v.uv[0].to_bits(), v.uv[1].to_bits(), v.color, light])
                .collect()
        };
        d.equal(packed_flag as u64, 1, &format!("falling {case} id {id}: packed brightness enabled"));
        if n != verts.len() {
            eprintln!("falling {case} id {id} ao {ao}: Java {n} vertices, Rust {}", verts.len());
            d.bad += 1;
        }
        for v in 0..n {
            let expected: [u32; 7] = std::array::from_fn(|_| d.int() as u32);
            if let Some(r) = verts.get(v) {
                for k in 0..7 {
                    d.equal(expected[k] as u64, r[k] as u64, &format!("falling {case} id {id} ao {ao} vertex {v} word {k}"));
                }
            }
        }
    }
}
