//! Comparison with `Oracle models`: doubles and floats are compared by raw bits.

use super::geometry::PositionTextureVertex;
use super::models::{Model, ModelKind, WolfAnimation};

struct Dump {
    data: Vec<u8>,
    at: usize,
    checks: usize,
    bad: usize,
}
impl Dump {
    fn bytes<const N: usize>(&mut self) -> [u8; N] {
        let b = self.data[self.at..self.at + N].try_into().expect("truncated model dump");
        self.at += N;
        b
    }
    fn u32(&mut self) -> u32 {
        u32::from_le_bytes(self.bytes())
    }
    fn f32(&mut self) -> f32 {
        f32::from_bits(self.u32())
    }
    fn string(&mut self) -> String {
        let n = self.u32() as usize;
        let s = std::str::from_utf8(&self.data[self.at..self.at + n]).expect("part name").to_owned();
        self.at += n;
        s
    }
    fn equal(&mut self, java: u64, rust: u64, path: &str) {
        self.checks += 1;
        if java != rust {
            if self.bad < 12 {
                eprintln!("{path}: java {java:016x}, rust {rust:016x}");
            }
            self.bad += 1;
        }
    }
    fn vertex(&mut self, r: PositionTextureVertex, path: &str) {
        for (i, v) in r.pos.into_iter().enumerate() {
            let j = u64::from_le_bytes(self.bytes());
            self.equal(j, v.to_bits(), &format!("{path}.pos[{i}]"));
        }
        for (i, v) in r.uv.into_iter().enumerate() {
            let j = self.u32();
            self.equal(j as u64, v.to_bits() as u64, &format!("{path}.uv[{i}]"));
        }
    }
}

pub fn model_check(path: &str) -> bool {
    use ModelKind::*;
    let kinds = [
        Biped, Zombie, Skeleton, Quadruped, Pig, Cow, SheepFur, Sheep, Chicken, Spider, Creeper, SlimeOuter,
        SlimeInner, Squid, Wolf, Ghast, Enderman, Silverfish, Chest, LargeChest,
    ];
    let mut d = Dump { data: std::fs::read(path).expect("read models dump"), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.u32(), u32::from_le_bytes(*b"M173"), "not a model dump");
    assert_eq!(d.u32(), 2, "model dump version");
    let n = d.u32();
    assert!(n > 0, "empty model dump");
    let mut current = u32::MAX;
    let mut model = Model::new(Biped);
    let mut seen = [0usize; 20];
    let mut parts = 0;
    for case in 0..n {
        let group = d.u32();
        let kind = d.u32() as usize;
        let inflate = d.f32();
        let offset = d.f32();
        let legs = d.u32() as i32;
        let pose = std::array::from_fn(|_| d.f32());
        let attack = d.f32();
        let flags = d.u32();
        let partial = d.f32();
        let interested = std::array::from_fn(|_| d.f32());
        let shake = std::array::from_fn(|_| d.f32());
        if group != current {
            model = Model::with_options(kinds[kind], inflate, offset, legs);
            current = group;
        }
        seen[kind] += 1;
        model.on_ground = attack;
        model.is_riding = flags & 1 != 0;
        model.is_sneak = flags & 2 != 0;
        model.held_left = flags & 4 != 0;
        model.held_right = flags & 8 != 0;
        model.set_living_animations(
            pose[0],
            pose[1],
            partial,
            &WolfAnimation { angry: flags & 16 != 0, sitting: flags & 32 != 0, interested, shake },
        );
        model.carrying=flags & 4 != 0; model.screaming=flags & 8 != 0;
        model.set_rotation_angles(pose);
        let np = d.u32() as usize;
        assert_eq!(np, model.parts.len(), "case {case}: part count");
        parts += np;
        for _ in 0..np {
            let name = d.string();
            let path = format!("case {case} {:?}.{name}", kinds[kind]);
            let r = model.parts.get(&name).unwrap_or_else(||panic!("case {case} {:?}: missing {name}",kinds[kind]));
            for (f, v) in [
                ("rotationPointX", r.rotation_point_x),
                ("rotationPointY", r.rotation_point_y),
                ("rotationPointZ", r.rotation_point_z),
                ("rotateAngleX", r.rotate_angle_x),
                ("rotateAngleY", r.rotate_angle_y),
                ("rotateAngleZ", r.rotate_angle_z),
            ] {
                let j = d.u32();
                d.equal(j as u64, v.to_bits() as u64, &format!("{path}.{f}"));
            }
            let j = d.u32();
            let flags = r.mirror as u32 | (r.show_model as u32) << 1 | (r.hidden as u32) << 2;
            d.equal(j as u64, flags as u64, &format!("{path}.flags"));
            assert_eq!(d.u32(), 8, "corners");
            for (i, v) in r.corners.iter().enumerate() {
                d.vertex(*v, &format!("{path}.corners[{i}]"));
            }
            assert_eq!(d.u32() as usize, r.faces.len(), "quads");
            for (i, q) in r.faces.iter().enumerate() {
                assert_eq!(d.u32(), 4, "quad vertices");
                for (k, v) in q.vertices.iter().enumerate() {
                    d.vertex(*v, &format!("{path}.faces[{i}][{k}]"));
                }
                for (k, v) in q.normal().to_array().into_iter().enumerate() {
                    let j = d.u32();
                    d.equal(j as u64, v.to_bits() as u64, &format!("{path}.faces[{i}].normal[{k}]"));
                }
                let j = d.u32();
                d.equal(j as u64, super::living::packed_normal(q) as u64, &format!("{path}.faces[{i}].packedNormal"));
            }
        }
    }
    assert_eq!(d.at, d.data.len(), "trailing model dump bytes");
    assert!(seen.iter().all(|n| *n > 0), "oracle must cover every model");
    println!("models: {n} poses, {parts} parts, {} bit comparisons, {} differ", d.checks, d.bad);
    d.bad == 0
}
