//! Whole-stream state and vertex-bit comparison against original EntityFX bytecode.
use super::{
    particles::{rotation, Kind, Particle},
    weather::{Column, Snapshot},
};
use worldgen::{Biome, JavaRandom};
struct Reader {
    data: Vec<u8>,
    at: usize,
}
impl Reader {
    fn bytes<const N: usize>(&mut self) -> [u8; N] {
        let r = self.data[self.at..self.at + N].try_into().unwrap();
        self.at += N;
        r
    }
    fn i(&mut self) -> i32 {
        i32::from_le_bytes(self.bytes())
    }
    fn l(&mut self) -> i64 {
        i64::from_le_bytes(self.bytes())
    }
    fn d(&mut self) -> f64 {
        f64::from_bits(self.l() as u64)
    }
    fn f(&mut self) -> f32 {
        f32::from_bits(self.i() as u32)
    }
}
fn state(p: &Particle, m: &JavaRandom) -> Vec<u8> {
    let mut o = Vec::new();
    for v in p
        .pos
        .into_iter()
        .chain(p.prev)
        .chain(p.motion)
        .chain(p.bounds)
    {
        o.extend(
            (if v.is_nan() {
                f64::NAN.to_bits()
            } else {
                v.to_bits()
            })
            .to_le_bytes(),
        );
    }
    for v in p.color.into_iter().chain([
        p.scale,
        p.gravity,
        p.jitter[0],
        p.jitter[1],
        p.width,
        p.height,
        p.y_offset,
    ]) {
        o.extend(
            (if v.is_nan() {
                f32::NAN.to_bits()
            } else {
                v.to_bits()
            })
            .to_le_bytes(),
        );
    }
    for v in [p.texture, p.age, p.max_age, p.layer() as i32, p.timer] {
        o.extend(v.to_le_bytes());
    }
    for v in [p.dead, p.on_ground, p.no_clip, p.in_web] {
        o.push(v as u8);
    }
    o.extend(p.rand.state().to_le_bytes());
    o.extend(m.state().to_le_bytes());
    o
}
pub fn check(path: &str) -> bool {
    let mut d = Reader {
        data: std::fs::read(path).unwrap(),
        at: 0,
    };
    let magic = d.bytes::<8>();
    let light = magic == *b"B181PRT2";
    assert!(light || magic == *b"B173PRT1");
    let mut s = Snapshot::default();
    let width = d.i();
    for _ in 0..width * width {
        let (x, z, top) = (d.i(), d.i(), d.i());
        let blocks = d.bytes();
        let meta = d.bytes();
        let brightness = std::array::from_fn(|_| d.f());
        let packed_light = if light { std::array::from_fn(|_| d.i()) } else { [0;129] };
        s.columns.push(Column {
            rain: false,
            snow: false,
            x,
            z,
            top,
            blocks,
            meta,
            brightness,
            packed_light,
            biome: Biome::Plains,
            particle_rain: true,
        });
    }
    let cases = d.i();
    let mut bad = 0;
    let mut vertices = 0;
    for case in 0..cases {
        let id = d.i();
        let seed = d.l();
        let mathseed = d.l();
        let pos = std::array::from_fn(|_| d.d());
        let velocity = std::array::from_fn(|_| d.d());
        let block = d.i() as u8;
        let metadata = d.i();
        let frames = d.i();
        let kind = match id {
            0 => Kind::Base,
            1 => Kind::Smoke,
            2 => Kind::LargeSmoke,
            3 => Kind::Flame,
            4 => Kind::Explode,
            5 => Kind::Splash,
            6 => Kind::Rain,
            7 => Kind::Bubble,
            8 => Kind::Portal,
            9 => Kind::Note,
            10 => Kind::RedDust,
            11 => Kind::Heart,
            12 => Kind::Lava,
            13 => Kind::Snowball,
            14 => Kind::Slime,
            15 => Kind::SnowShovel,
            16 => Kind::Digging { block, metadata },
            17 => Kind::Footstep,
            18 => Kind::Crit,
            19 => Kind::MagicCrit,
            20 => Kind::Cloud,
            21 => Kind::Aura,
            22 => Kind::Suspend,
            23 => Kind::LargeExplode,
            _ => panic!(),
        };
        let mut m = JavaRandom::new(mathseed);
        let mut p = Particle::new_in(kind, pos, velocity, &mut m, seed, &s);
        if id == 20 {
            p.cloud_player = Some([0.5, 64.1, 1.5, 64.1 - 1.62f32 as f64, 0.0]);
        }
        if id == 16 && case % 8 == 7 {
            p.multiply_velocity(0.2);
            p.multiply_scale(0.6);
        }
        for tick in 0..frames {
            let rust = state(&p, &m);
            let start = d.at;
            let end = start + rust.len();
            let expected = &d.data[start..end];
            if rust != expected {
                if bad < 12 {
                    let offset = rust.iter().zip(expected).position(|(a, b)| a != b).unwrap();
                    eprintln!("particle {kind:?} variant {} tick {tick} state byte {offset}: rust {:02x?}, java {:02x?}",case%8,&rust[offset..(offset+8).min(rust.len())],&expected[offset..(offset+8).min(expected.len())]);
                }
                bad += 1;
            }
            d.at = end;
            for partial in [0.0, 0.25, 0.75, 1.0] {
                if light {
                    let java = d.i();
                    let rust = p.packed_brightness(&s, partial);
                    if rust != java { if bad < 12 { eprintln!("particle {kind:?} tick {tick} light {rust:08x}/{java:08x}"); } bad += 1; }
                }
                let vs = p.vertices(&s, [0.125, 65.0, -0.25], rotation(37.25, -21.5), partial);
                if light { assert_eq!(d.i() as usize, vs.len(), "particle vertex count"); }
                for v in vs {
                    let mut fields = Vec::new();
                    for n in v.pos.into_iter().chain(v.uv) {
                        fields.push(n.to_bits());
                    }
                    let c = v.color.map(|v| (v * 255.0).round() as u32);
                    fields.push(c[0] | c[1] << 8 | c[2] << 16 | c[3] << 24);
                    for (j, r) in fields.into_iter().enumerate() {
                        let java = d.i() as u32;
                        if r != java {
                            if bad < 12 {
                                eprintln!("particle {kind:?} variant {} tick {tick} partial {partial} vertex field {j}: rust {r:08x}, java {java:08x}",case%8);
                            }
                            bad += 1;
                        }
                    }
                    vertices += 1;
                }
            }
            if !p.dead {
                p.tick(&s, &mut m);
            }
        }
    }
    let pickups = d.i();
    let mut transforms = 0;
    for case in 0..pickups {
        let seed = d.l();
        let mathseed = d.l();
        let source = std::array::from_fn(|_| d.d());
        let motion = std::array::from_fn(|_| d.d());
        let last = std::array::from_fn(|_| d.d());
        let now = std::array::from_fn(|_| d.d());
        let mut m = JavaRandom::new(mathseed);
        let mut p = Particle::new(Kind::Pickup, source, motion, &mut m, seed);
        for tick in 0..4 {
            let rust = state(&p, &m);
            if rust != d.data[d.at..d.at + rust.len()] {
                if bad < 12 {
                    eprintln!("pickup {case} tick {tick} state differs");
                }
                bad += 1;
            }
            d.at += rust.len();
            for partial in [0.0, 0.25, 0.75, 1.0] {
                let pos = crate::world_effects::Effects::pickup_position(
                    source, last, now, p.timer, partial,
                );
                let actual = std::array::from_fn::<_, 3, _>(|i| {
                    (pos[i] - [0.125, 65.0, -0.25][i]) as f32 as f64
                });
                for v in actual {
                    let java = d.d();
                    if v.to_bits() != java.to_bits() {
                        if bad < 12 {
                            eprintln!("pickup {case} tick {tick} position differs {v} {java}");
                        }
                        bad += 1;
                    }
                }
                let brightness = 1.0f32;
                if brightness.to_bits() != d.f().to_bits() {
                    bad += 1;
                }
                if light && p.packed_brightness(&s,partial) != d.i() { bad += 1; }
                transforms += 1;
            }
            if !p.dead {
                p.tick(&s, &mut m);
            }
        }
    }
    println!("pickup checks: {pickups} cases, {transforms} transforms");
    assert_eq!(d.at, d.data.len());
    println!("particle checks: {cases} cases, {vertices} vertices, {bad} differing fields");
    bad == 0
}

pub fn effects_check(path: &str) -> bool {
    use crate::{render::particles::EffectRenderer, world_effects::Effects};
    let a = crate::assets::load().unwrap();
    let mut e = Effects::new(0);
    e.colors = Some(crate::render::blocks::Colorizers {
        grass: a.grass_color,
        foliage: a.foliage_color,
    });
    let mut d = Reader {
        data: std::fs::read(path).unwrap(),
        at: 0,
    };
    assert_eq!(d.bytes::<8>(), *b"B173EFX1");
    let cases = d.i();
    let mut bad = 0;
    let mut vertices = 0;
    for _ in 0..cases {
        let block = d.i() as u8;
        let meta = d.i();
        e.fancy = d.i() != 0;
        let bounds = std::array::from_fn(|_| d.d());
        let tint = d.i() as u32;
        let actual = e.tint(block, meta, [8, 64, 8]);
        if actual != tint & 0xffffff {
            if bad < 8 {
                eprintln!("block {block} meta {meta} tint differs {actual:06x}/{tint:06x}");
            }
            bad += 1;
        }
        let width = d.i();
        let mut terrain = Snapshot::default();
        for _ in 0..width * width {
            let (x, z, top) = (d.i(), d.i(), d.i());
            let blocks = d.bytes();
            let meta = d.bytes();
            let brightness = std::array::from_fn(|_| d.f());
            terrain.columns.push(Column {
                rain: false,
                snow: false,
                    x,
                z,
                top,
                blocks,
                meta,
                brightness,
                packed_light: [0;129],
                biome: Biome::Plains,
                particle_rain: true,
            });
        }
        e.renderer = EffectRenderer::new(0);
        e.renderer.next_seed = 800;
        e.renderer.math_random.set_seed(900);
        e.renderer.rand.set_seed(700);
        e.terrain = terrain;
        e.fragments(block, meta, [8, 64, 8], None);
        for side in 0..6 {
            e.fragments(block, meta, [8, 64, 8], Some((side, bounds)));
        }
        for tick in 0..20 {
            if e.renderer.rand.state() != d.l() {
                bad += 1;
            }
            let count = d.i();
            if count as usize != e.renderer.layers[1].len() {
                panic!("fragment count differs");
            }
            for p in &mut e.renderer.layers[1] {
                let bytes = state(p, &e.renderer.math_random);
                if bytes != d.data[d.at..d.at + bytes.len()] {
                    if bad < 8 {
                        let byte = bytes.iter().zip(&d.data[d.at..]).position(|(a, b)| a != b);
                        eprintln!("block {block} meta {meta} tick {tick} fragment state differs at {byte:?}");
                    }
                    bad += 1;
                }
                d.at += bytes.len();
                for v in p.vertices(&e.terrain, [8.0, 65.0, 8.0], [1.0, 1.0, 0.0, 0.0, 0.0], 0.5) {
                    let c = v.color.map(|v| (v * 255.0).round() as u32);
                    let color = c[0] | c[1] << 8 | c[2] << 16 | c[3] << 24;
                    for actual in v
                        .pos
                        .into_iter()
                        .chain(v.uv)
                        .map(f32::to_bits)
                        .chain([color])
                    {
                        if actual != d.i() as u32 {
                            bad += 1;
                        }
                    }
                    vertices += 1;
                }
            }
            e.renderer.tick(&e.terrain, [8.0, 65.0, 8.0]);
        }
    }
    e.renderer = EffectRenderer::new(0);
    e.renderer.next_seed = 1100;
    e.renderer.math_random.set_seed(1200);
    for kind in [
        Kind::Lava,
        Kind::Smoke,
        Kind::Digging {
            block: 1,
            metadata: 0,
        },
        Kind::Slime,
        Kind::Footstep,
        Kind::Rain,
    ] {
        let p = e.renderer.create(kind, [8.5, 66.1, 8.5], [0.1, -0.1, 0.2]);
        e.renderer.add(p);
    }
    for tick in 0..30 {
        for layer in 0..4 {
            let n = d.i() as usize;
            assert_eq!(
                n,
                e.renderer.layers[layer].len(),
                "tick {tick} layer {layer} count"
            );
            for p in &e.renderer.layers[layer] {
                let bytes = state(p, &e.renderer.math_random);
                if bytes != d.data[d.at..d.at + bytes.len()] {
                    if bad < 8 {
                        let byte = bytes.iter().zip(&d.data[d.at..]).position(|(a, b)| a != b);
                        eprintln!("mixed effects tick {tick} layer {layer} state byte {byte:?}");
                    }
                    bad += 1;
                }
                d.at += bytes.len();
            }
        }
        e.renderer.tick(&e.terrain, [8.0, 65.0, 8.0]);
    }
    let mut r = EffectRenderer::new(0);
    for i in 0..4002 {
        let p = r.create(Kind::Base, [i as f64, 70.0, 0.0], [0.0; 3]);
        r.add(p);
    }
    assert_eq!(d.i() as usize, r.len());
    assert_eq!(d.d(), r.layers[0][0].pos[0]);
    assert_eq!(d.d(), r.layers[0].last().unwrap().pos[0]);
    let names = [
        "bubble",
        "smoke",
        "note",
        "portal",
        "explode",
        "flame",
        "lava",
        "footstep",
        "splash",
        "largesmoke",
        "reddust",
        "snowballpoof",
        "snowshovel",
        "slime",
        "heart",
        "unknown",
    ];
    let distances = [
        0.0,
        16.0,
        f64::from_bits(16.0f64.to_bits() + 1),
        -16.0,
        f64::NAN,
        f64::INFINITY,
    ];
    assert_eq!(d.i() as usize, names.len() * distances.len());
    for name in names {
        for distance in distances {
            let mut r = EffectRenderer::new(0);
            r.next_seed = 1000;
            r.math_random.set_seed(2000);
            r.spawn(
                name,
                [8.5 + distance, 65.0, 8.5],
                [0.1, -0.1, 0.2],
                [8.5, 65.0, 8.5],
            );
            for layer in &r.layers {
                assert_eq!(
                    d.i() as usize,
                    layer.len(),
                    "RenderGlobal gate {name}, distance {distance}"
                );
                for p in layer {
                    let bytes = state(p, &r.math_random);
                    if bytes != d.data[d.at..d.at + bytes.len()] {
                        eprintln!("RenderGlobal gate {name}, distance {distance} state differs");
                        bad += 1;
                    }
                    d.at += bytes.len();
                }
            }
            if r.math_random.state() != d.l() {
                bad += 1;
            }
        }
    }
    assert_eq!(d.at, d.data.len());
    println!("EffectRenderer checks: {cases} block/metadata cases, {vertices} vertices, {bad} differing fields");
    bad == 0
}

pub fn emitter_check(path: &str) -> bool {
    let mut r = Reader { data: std::fs::read(path).unwrap(), at: 0 };
    let cases = r.i(); let mut bad = 0;
    for case in 0..cases {
        let kind = if r.i() == 0 { Kind::HugeExplode } else { Kind::CritEmitter };
        let seed = r.l(); let mathseed = r.l();
        let mut w = worldgen::entity::verify181::world();
        let mut target = worldgen::entity::Entity::living(&mut w, "Pig").unwrap();
        target.set_position(-3.5,64.1,1.5); target.motion = worldgen::entity::Vec3D::new(0.25,-0.1,0.5);
        let pos = if kind == Kind::CritEmitter { [target.pos.x, target.bounding_box.min_y+(target.height/2.0) as f64,target.pos.z] } else { [-3.5,64.1,1.5] };
        let motion = if kind == Kind::CritEmitter { [0.25,-0.1,0.5] } else { [0.0;3] };
        let mut m = JavaRandom::new(mathseed);
        let terrain = Snapshot::default();
        let mut p = Particle::new(kind,pos,motion,&mut m,seed);
        if kind == Kind::CritEmitter { p.pickup=Some(Box::new(target.clone()));p.tick(&terrain,&mut m); }
        for tick in 0..10 {
            let mut bytes = state(&p,&m);
            bytes.extend((p.emitted.len() as i32).to_le_bytes());
            for (name,pos,motion) in std::mem::take(&mut p.emitted) {
                bytes.push(1);bytes.extend((name.len() as i32).to_le_bytes());bytes.extend(name.as_bytes());
                for v in pos.into_iter().chain(motion) {bytes.extend(v.to_bits().to_le_bytes());}
            }
            if bytes != r.data[r.at..r.at+bytes.len()] { if bad<12 {eprintln!("emitter {case} tick {tick} differs at {:?}",bytes.iter().zip(&r.data[r.at..]).position(|(a,b)|a!=b));} bad+=1; }
            r.at += bytes.len();
            target.set_position(target.pos.x+0.125,target.pos.y-0.03125,target.pos.z+0.25);
            p.pickup=Some(Box::new(target.clone()));
            if !p.dead {p.tick(&terrain,&mut m);}
        }
    }
    assert_eq!(r.at,r.data.len());
    println!("particle emitters: {cases} cases, {bad} differences");bad==0
}

pub fn lifecycle_check(path: &str) -> bool {
    use super::particles::EffectRenderer;
    let mut expected = Reader { data: std::fs::read(path).unwrap(), at: 0 };
    let mut renderer = EffectRenderer::new(0);
    renderer.next_seed = 100;
    renderer.math_random.set_seed(200);
    let terrain = Snapshot::default();
    let mut cases = 0;
    fn snapshot(r: &EffectRenderer, d: &mut Reader) {
        for layer in &r.layers {
            assert_eq!(d.i() as usize, layer.len(), "particle layer size");
            for p in layer {
                assert_eq!(d.d().to_bits(), p.pos[0].to_bits(), "particle insertion/tick order");
                assert_eq!(d.i(), p.age, "particle age");
                assert_eq!(d.bytes::<1>()[0] != 0, p.dead, "particle dead flag");
                assert_eq!(d.l(), p.rand.state(), "particle RNG");
            }
        }
        assert_eq!(d.i() as usize, r.len(), "statistics excludes lit layer");
        assert_eq!(d.l(), r.math_random.state(), "Math.random");
    }
    for _cycle in 0..3 {
        for layer in 0..4 {
            for j in 0..4003 {
                let kind = [Kind::Base, Kind::Digging { block: 1, metadata: 0 }, Kind::Snowball, Kind::Footstep][layer];
                let mut p = renderer.create(kind, [0.0,85.0,0.0], [0.0;3]);
                p.pos[0] = j as f64 * 0.001;
                p.max_age = j % 4;
                p.dead = j % 7 == 0;
                renderer.add(p);
            }
        }
        snapshot(&renderer, &mut expected); cases += 1;
        for _ in 0..5 { renderer.tick(&terrain, [0.0,85.0,0.0]); snapshot(&renderer, &mut expected); cases += 1; }
        renderer.clear(); snapshot(&renderer, &mut expected); cases += 1;
    }
    assert_eq!(expected.at, expected.data.len());
    println!("EXACT: EffectRenderer lifecycle, {cases} snapshots, 48,036 insertions across four layers");
    true
}

/// Complete named factory matrix, including admission boundaries and error RNG.
pub fn factory_check(path: &str) -> bool {
    use super::particles::EffectRenderer;
    let mut actual = Vec::new();
    let names = ["bubble","suspended","depthsuspend","townaura","crit","magicCrit","smoke","note","portal","explode","flame","lava","footstep","splash","largesmoke","cloud","reddust","snowballpoof","snowshovel","slime","heart","largeexplode","hugeexplosion","iconcrack_260","iconcrack_+267","iconcrack_351","tilecrack_1","tilecrack_18","tilecrack_35","tilecrack_98","unknown","iconcrack_foo","tilecrack_","tilecrack_300","tilecrack_111","iconcrack_111"];
    for name in names { for offset in [0.0,16.0,16.0001,1000.0,f64::NAN] {
        let mut renderer=EffectRenderer::new(0);renderer.next_seed=100;renderer.math_random.set_seed(200);
        let error=renderer.try_spawn(name,[-3.5,85.25,4.5],[0.125,-0.1,0.2],[-3.5+offset,85.25,4.5]).err().unwrap_or("");
        actual.extend((error.len() as i32).to_le_bytes());actual.extend(error.as_bytes());
        for layer in &renderer.layers { actual.extend((layer.len() as i32).to_le_bytes());for p in layer{actual.extend(state(p,&renderer.math_random));} }
        actual.extend(renderer.math_random.state().to_le_bytes());actual.extend((renderer.len() as i32).to_le_bytes());
    }}
    let expected=std::fs::read(path).unwrap();
    if actual!=expected {let offset=actual.iter().zip(&expected).position(|(a,b)|a!=b).unwrap_or(actual.len().min(expected.len()));eprintln!("particle factory DIFF byte {offset}: Java {}, Rust {}",expected.len(),actual.len());std::fs::write(std::path::Path::new(path).with_extension("rust.bin"),actual).unwrap();return false;}
    println!("EXACT: named particle factory, {} cases, {} bytes",names.len()*5,actual.len());true
}
