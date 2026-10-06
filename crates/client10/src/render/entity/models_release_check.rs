//! Raw-bit comparison of original 1.0 model constructor and animation bytecode.
use super::{
    geometry::PositionTextureVertex,
    models::{Model, ModelKind, WolfAnimation},
};

struct Dump {
    bytes: Vec<u8>,
    at: usize,
    checks: usize,
    bad: usize,
}
impl Dump {
    fn take<const N: usize>(&mut self) -> [u8; N] {
        let result = self.bytes[self.at..self.at + N]
            .try_into()
            .expect("truncated model dump");
        self.at += N;
        result
    }
    fn int(&mut self) -> i32 {
        i32::from_le_bytes(self.take())
    }
    fn float(&mut self) -> f32 {
        f32::from_bits(self.int() as u32)
    }
    fn string(&mut self) -> String {
        let n = self.int() as usize;
        let s = String::from_utf8(self.bytes[self.at..self.at + n].to_vec()).unwrap();
        self.at += n;
        s
    }
    fn equal(&mut self, j: u64, r: u64, path: &str) {
        self.checks += 1;
        if j != r {
            if self.bad < 12 {
                eprintln!("{path}: Java {j:016x}, Rust {r:016x}");
            }
            self.bad += 1;
        }
    }
    fn f(&mut self, r: f32, path: &str) {
        let j = self.int() as u32;
        self.equal(j as u64, r.to_bits() as u64, path);
    }
    fn vertex(&mut self, r: PositionTextureVertex, path: &str) {
        for v in r.pos {
            let j = u64::from_le_bytes(self.take());
            self.equal(j, v.to_bits(), path);
        }
        for v in r.uv {
            self.f(v, path);
        }
    }
    fn part(&mut self, p: &super::geometry::ModelRenderer, path: &str) {
        for v in p.texture_size {
            self.f(v, path);
        }
        assert_eq!(
            self.string(),
            p.box_name.as_deref().unwrap_or(""),
            "{path}: box name"
        );
        for v in [
            p.rotation_point_x,
            p.rotation_point_y,
            p.rotation_point_z,
            p.rotate_angle_x,
            p.rotate_angle_y,
            p.rotate_angle_z,
        ] {
            self.f(v, path);
        }
        assert_eq!(
            self.int(),
            p.mirror as i32 | (p.show_model as i32) << 1 | (p.hidden as i32) << 2,
            "{path}: flags"
        );
        assert_eq!(self.int() as usize, p.cube_list.len(), "{path}: cube count");
        for cube in &p.cube_list {
            for v in cube.raw_bounds {
                self.f(v, path);
            }
            assert_eq!(
                self.string(),
                cube.name.as_deref().unwrap_or(""),
                "{path}: cube name"
            );
            for v in cube.corners {
                self.vertex(v, path);
            }
            assert_eq!(cube.faces.len(), 6);
            for q in &cube.faces {
                for v in q.vertices {
                    self.vertex(v, path);
                }
            }
        }
    }
    fn ops(&mut self, r: &[super::fixed::Op], path: &str) {
        assert_eq!(self.int() as usize, r.len(), "{path}: operation count");
        for op in r {
            assert_eq!(self.int() as u32, op.code, "{path}: operation");
            for a in op.args {
                self.f(a, path);
            }
        }
    }
    fn draws(&mut self, g: &super::fixed::Fixed, path: &str) {
        assert_eq!(self.int() as usize, g.draws.len(), "{path}: draw count");
        for (i, draw) in g.draws.iter().enumerate() {
            let path = format!("{path}.draw[{i}]");
            self.ops(&draw.ops, &path);
            assert_eq!(self.int() as u8, draw.cull, "{path}: culling");
            let normal = self.int() as u32;
            assert_eq!(
                self.int() as usize,
                draw.vertices.len(),
                "{path}: vertex count"
            );
            for vertex in &draw.vertices {
                self.equal(normal as u64, vertex.normal as u64, &path);
                for f in vertex.pos.into_iter().chain(vertex.uv) {
                    self.f(f, &path);
                }
            }
        }
        self.ops(&g.ops, path);
    }
    fn full_draws(&mut self, g: &super::fixed::Fixed, path: &str) {
        assert_eq!(self.int() as usize, g.draws.len(), "{path}: draw count");
        for (i, v) in g.draws.iter().enumerate() {
            let path = format!("{path}.draw[{i}]");
            self.ops(&v.ops, &path);
            for actual in [
                v.cull as u32,
                v.flags,
                v.texture,
                v.src,
                v.dst,
                v.depth,
                v.mode,
            ] {
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
    }
}

pub fn check(path: &str) -> bool {
    use ModelKind::*;
    let kinds = [
        Biped,
        Zombie,
        Skeleton,
        Quadruped,
        Pig,
        Cow,
        Chicken,
        Wolf,
        Dragon,
        Villager,
        SnowMan,
        Blaze,
        MagmaCube,
        Book,
        EnderCrystal,
    ];
    let mut d = Dump {
        bytes: std::fs::read(path).expect("read release models"),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.int(), 0x30314d52);
    assert_eq!(d.int(), 2);
    let count = d.int();
    assert_eq!(count, 80, "complete constructor/pose matrix");
    let mut group = -1;
    let mut m = Model::new(Biped);
    for c in 0..count {
        let next_group = d.int();
        let kind = kinds[d.int() as usize];
        let inflate = d.float();
        let offset = d.float();
        let legs = d.int();
        let pose = std::array::from_fn(|_| d.float());
        let attack = d.float();
        let flags = d.int();
        let partial = d.float();
        let left = d.int();
        let right = d.int();
        let bow = d.int() != 0;
        let interested = std::array::from_fn(|_| d.float());
        let shake = std::array::from_fn(|_| d.float());
        if next_group != group {
            m = Model::with_options(kind, inflate, offset, legs);
            group = next_group;
        }
        m.on_ground = attack;
        m.is_riding = flags & 1 != 0;
        m.is_sneak = flags & 2 != 0;
        m.is_child = flags & 8 != 0;
        m.held_left_pose = left;
        m.held_right_pose = right;
        m.aimed_bow = bow;
        if kind == Wolf {
            m.set_living_animations(
                pose[0],
                pose[1],
                partial,
                &WolfAnimation {
                    angry: flags & 2 != 0,
                    sitting: flags & 4 != 0,
                    interested,
                    shake,
                },
            );
        }
        if kind == MagmaCube {
            m.magma_animation(shake[0] - 0.7, shake[1] - 0.7, partial);
        }
        m.set_rotation_angles(pose);
        assert_eq!(d.int(), m.texture_size[0]);
        assert_eq!(d.int(), m.texture_size[1]);
        assert_eq!(d.int(), m.model_version());
        assert_eq!(
            d.int() as usize,
            m.registered.len(),
            "case {c}: registered parts"
        );
        for i in 0..m.registered.len() {
            let p = m.registered_part(i);
            let path = format!("case {c} {kind:?}.{}[{i}]", m.registered[i]);
            d.part(p, &path);
        }
        assert_eq!(d.int() as usize, m.registered.len());
        for i in 0..m.registered.len() {
            let children: Vec<_> = m
                .child_links
                .iter()
                .filter(|(parent, _)| parent == &m.registered[i])
                .map(|(_, child)| m.registered.iter().position(|name| name == child).unwrap())
                .collect();
            assert_eq!(
                d.int() as usize,
                children.len(),
                "case {c}: children of {}",
                m.registered[i]
            );
            for child in children {
                assert_eq!(d.int() as usize, child);
            }
        }
    }
    assert_eq!(d.at, d.bytes.len(), "trailing oracle data");
    println!(
        "MODELS10 {}: {count} cases, {} exact raw-bit comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

pub fn draw_check(path: &str) -> bool {
    use ModelKind::*;
    let kinds = [
        Biped,
        Zombie,
        Skeleton,
        Quadruped,
        Pig,
        Cow,
        Chicken,
        Wolf,
        Dragon,
        Villager,
        SnowMan,
        Blaze,
        MagmaCube,
        Book,
        EnderCrystal,
    ];
    let mut d = Dump {
        bytes: std::fs::read(path).expect("read release model draws"),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.int(), 0x3031444d);
    assert_eq!(d.int(), 2);
    assert_eq!(d.int(), 32);
    let mut group = -1;
    let mut m = Model::new(Biped);
    for c in 0..32 {
        let next_group = d.int();
        let kind = kinds[d.int() as usize];
        let pose = std::array::from_fn(|_| d.float());
        let flags = d.int();
        let partial = d.float();
        if next_group != group {
            m = Model::with_options(
                kind,
                next_group as f32 * 0.25,
                (next_group - 2) as f32,
                6 + next_group * 3,
            );
            group = next_group;
        }
        let case = c % 8;
        m.is_child = flags & 1 != 0;
        m.on_ground = if case == 0 {
            -10000.0
        } else {
            case as f32 * 0.125
        };
        m.is_riding = flags & 2 != 0;
        m.held_left_pose = case % 4;
        m.held_right_pose = case / 2;
        m.aimed_bow = flags & 4 != 0;
        m.is_sneak = flags & 1 != 0;
        let mut g = super::fixed::Fixed::new(1.0);
        if kind == Dragon {
            let animation = d.float();
            let history = std::array::from_fn(|_| {
                std::array::from_fn(|_| f64::from_bits(u64::from_le_bytes(d.take())))
            });
            m.dragon_render(&mut g, animation, &history, pose[5]);
        } else if kind == EnderCrystal {
            m.crystal_render(&mut g, pose[1], pose[2], pose[5]);
        } else {
            m.set_rotation_angles(pose);
            super::special::model_scaled(&mut g, &m, pose[5]);
        }
        d.draws(&g, &format!("case {c} {kind:?}, partial {partial}"));
    }
    assert_eq!(d.at, d.bytes.len());
    println!(
        "MODEL DRAWS10 {}: 32 sequential cases, {} exact raw-bit comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

pub fn geometry_check(path: &str) -> bool {
    use super::{fixed::Fixed, geometry::ModelRenderer};
    let mut d = Dump {
        bytes: std::fs::read(path).expect("read model foundation dump"),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.int(), 0x3031474d);
    assert_eq!(d.int(), 2);
    let mut m = Model::new(ModelKind::Villager);
    m.set_texture_offset("named.first", -17, 37);
    m.set_texture_offset("named.first", -56, 88);
    let uv = m.get_texture_offset("named.first").unwrap();
    assert_eq!(d.int(), uv.coordinates()[0]);
    assert_eq!(d.int(), uv.coordinates()[1]);
    assert_eq!(d.int(), m.get_texture_offset("absent").is_none() as i32);
    let mut p = ModelRenderer::named("named", [128, 64]);
    p.add_named_box("first", uv, -1.5, -0.0, 2.25, 0, 7, 3);
    p.mirror = true;
    p.set_texture_offset(3, -7)
        .add_box(-2.5, -3.25, -4.75, 9, 0, 5, -0.35);
    let mut a = ModelRenderer::new(19, 31);
    a.set_texture_size(128, 64)
        .add_box(-3.0, 2.0, -1.0, 4, 6, 2, 0.5);
    a.set_rotation_point(0.25, -0.5, 2.0);
    let mut b = ModelRenderer::new(0, 0);
    b.set_texture_size(256, 256)
        .set_texture_offset(-56, 144)
        .add_box(-56.0, 0.0, 2.0, 56, 0, 56, 0.0);
    b.set_rotation_point(-2.0, 3.0, 4.0);
    b.rotate_angle_y = 0.75;
    for (part, path) in [(&p, "parent"), (&a, "child a"), (&b, "child b")] {
        d.part(part, path);
    }
    p.add_child(a);
    p.add_child(b);
    assert_eq!(d.int(), 24);
    for c in 0..24 {
        p.show_model = c % 7 != 3;
        p.hidden = c % 7 == 4;
        p.set_rotation_point(
            if c % 4 == 0 { 0.0 } else { -0.25 * c as f32 },
            if c % 4 == 0 { 0.0 } else { 1.5 },
            if c % 4 == 0 { 0.0 } else { -2.0 },
        );
        p.rotate_angle_x = if c % 4 == 0 { 0.0 } else { 0.13 * c as f32 };
        p.rotate_angle_y = if c % 4 == 0 { 0.0 } else { -0.17 };
        p.rotate_angle_z = if c % 4 == 0 { 0.0 } else { 0.31 };
        if c == 6 {
            p.set_texture_offset(18, 19)
                .add_box(-1.0, -2.0, -3.0, 1, 2, 3, 0.0);
            p.children[0].add_box(0.0, 1.0, 2.0, 0, 2, 3, 0.0);
        }
        let scale = [0.125, 0.0625, 0.25][c % 3];
        let mode = match c % 3 {
            0 => 2,
            1 => 0,
            _ => 1,
        };
        let mut g = Fixed::new(1.0);
        p.render_fixed(&mut g, scale, mode);
        d.draws(&g, &format!("ModelRenderer case {c}"));
    }
    assert_eq!(d.at, d.bytes.len());
    println!(
        "MODEL GEOMETRY10 {}: {} raw-bit comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

pub fn tile_check(path: &str) -> bool {
    use super::{
        objects::{self, ObjectKind, ObjectState},
        render_release::{self, BookState},
    };
    let mut d = Dump {
        bytes: std::fs::read(path).expect("read tile renderer dump"),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.int(), 0x30315452);
    assert_eq!(d.int(), 1);
    let kind = d.int();
    assert_eq!(d.int(), 64);
    for c in 0..64 {
        let relative = std::array::from_fn(|_| f64::from_bits(u64::from_le_bytes(d.take())));
        let partial = d.float();
        let ticks = d.int();
        let g = if kind == 0 {
            let state = BookState {
                ticks,
                rotation: std::array::from_fn(|_| d.float()),
                pages: std::array::from_fn(|_| d.float()),
                opening: std::array::from_fn(|_| d.float()),
            };
            render_release::book(state, relative, partial)
        } else {
            assert_eq!(kind, 1);
            objects::render(
                &ObjectState {
                    kind: ObjectKind::EnderCrystal { rotation: ticks },
                    position: relative,
                    partial,
                    brightness: 1.0,
                },
                [0.0; 3],
                [0.0; 2],
            )
        };
        assert_eq!(
            d.string(),
            if kind == 0 {
                "/item/book.png"
            } else {
                "/mob/enderdragon/crystal.png"
            }
        );
        assert!(g
            .draws
            .iter()
            .all(|draw| draw.texture == if kind == 0 { 75 } else { 74 }));
        d.draws(&g, &format!("renderer {kind} case {c}"));
    }
    assert_eq!(d.at, d.bytes.len());
    println!(
        "TILE RENDERER10 {}: 64 cases, {} raw-bit comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

pub fn effects_check(path: &str) -> bool {
    use super::{
        fixed::Fixed,
        objects::{self, ObjectKind, ObjectState},
        render_release,
    };
    let mut d = Dump {
        bytes: std::fs::read(path).expect("read renderer effects dump"),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.int(), 0x30314552);
    assert_eq!(d.int(), 1);
    let kind = d.int();
    assert_eq!(d.int(), 64);
    for c in 0..64 {
        let g = if kind != 1 {
            let icon = d.int();
            let damage = d.int();
            let position = std::array::from_fn(|_| f64::from_bits(u64::from_le_bytes(d.take())));
            let view = std::array::from_fn(|_| d.float());
            let partial = d.float();
            let scale = if kind == 2 { d.float() } else { 0.5 };
            objects::render(
                &ObjectState {
                    kind: if icon == 154 {
                        ObjectKind::Potion { damage }
                    } else {
                        ObjectKind::Sprite { icon, scale }
                    },
                    position,
                    partial,
                    brightness: 1.0,
                },
                [0.0; 3],
                view,
            )
        } else {
            assert_eq!(kind, 1);
            let ticks = d.int();
            let partial = d.float();
            let mut g = Fixed::new(1.0);
            if ticks > 0 {
                render_release::death_rays(&mut g, ticks, partial);
            }
            g
        };
        d.full_draws(&g, &format!("effects {kind} case {c}"));
    }
    assert_eq!(d.at, d.bytes.len());
    println!(
        "RENDER EFFECTS10 {}: 64 cases, {} raw-bit comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

pub fn living_check(path: &str) -> bool {
    use super::{
        living::{MobKind, RenderState},
        raw_living181,
    };
    let mut d = Dump {
        bytes: std::fs::read(path).expect("read living renderer dump"),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.int(), 0x30314c52);
    assert_eq!(d.int(), 1);
    let kind = d.int();
    assert_eq!(d.int(), 64);
    d.f(
        if kind == 1 {
            0.25
        } else if kind == 3 {
            0.7
        } else {
            0.5
        },
        "shadow size",
    );
    for c in 0..64 {
        let mut s = RenderState::new(
            [
                MobKind::Blaze,
                MobKind::MagmaCube,
                MobKind::Villager,
                MobKind::Mooshroom,
                MobKind::SnowMan,
            ][kind as usize],
        );
        s.position = std::array::from_fn(|_| f64::from_bits(u64::from_le_bytes(d.take())));
        s.partial = d.float();
        s.body_yaw = d.float();
        s.yaw = d.float();
        s.pitch = d.float();
        s.limb_amount = d.float();
        let swing = d.float();
        s.attack = d.float();
        let previous = d.float();
        let current = d.float();
        s.ticks = d.int();
        s.hurt_time = d.int();
        s.death_time = d.int();
        s.slime_size = d.int();
        s.profession = d.int();
        s.is_child = d.int() != 0;
        s.limb_swing = swing - s.limb_amount * (1.0 - s.partial);
        s.age = s.ticks as f32 + s.partial;
        s.slime_squish = previous + (current - previous) * s.partial;
        s.overlay_brightness = 0.73;
        let g = raw_living181::render(&s, [0.0; 3], false);
        d.full_draws(&g, &format!("living {kind} case {c}"));
    }
    assert_eq!(d.at, d.bytes.len());
    println!(
        "LIVING RENDERER10 {}: 64 cases, {} raw-bit comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

pub fn beam_check(path: &str) -> bool {
    use super::{
        fixed::Fixed,
        living::{MobKind, RenderState},
        render_release,
    };
    let mut d = Dump {
        bytes: std::fs::read(path).expect("read dragon beam dump"),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.int(), 0x30314244);
    assert_eq!(d.int(), 1);
    assert_eq!(d.int(), 64);
    for c in 0..64 {
        let mut point = || std::array::from_fn(|_| f64::from_bits(u64::from_le_bytes(d.take())));
        let mut s = RenderState::new(MobKind::Dragon);
        s.position = point();
        s.previous_position = point();
        s.current_position = point();
        let crystal = point();
        let rotation = d.int();
        s.ticks = d.int();
        s.partial = d.float();
        let mut g = Fixed::new(1.0);
        render_release::dragon_beam(&mut g, &s, [0.0; 3], crystal, rotation);
        d.full_draws(&g, &format!("dragon beam {c}"));
    }
    assert_eq!(d.at, d.bytes.len());
    println!(
        "DRAGON BEAM10 {}: 64 cases, {} raw-bit comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

pub fn dragon_check(path: &str) -> bool {
    use super::{
        living::{MobKind, RenderState},
        render_release,
    };
    let mut d = Dump {
        bytes: std::fs::read(path).expect("read dragon renderer dump"),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.int(), 0x30314452);
    assert_eq!(d.int(), 1);
    assert_eq!(d.int(), 32);
    for c in 0..32 {
        let mut s = RenderState::new(MobKind::Dragon);
        s.position = std::array::from_fn(|_| f64::from_bits(u64::from_le_bytes(d.take())));
        s.partial = d.float();
        let previous_animation = d.float();
        let animation = d.float();
        s.health = d.int();
        s.death_time = d.int();
        s.hurt_time = d.int();
        let death_ticks = d.int();
        let history = std::array::from_fn(|_| {
            std::array::from_fn(|_| f64::from_bits(u64::from_le_bytes(d.take())))
        });
        s.dragon = Some(Box::new(worldgen::entity::dragon::DragonState {
            previous_animation,
            animation,
            history,
            history_index: 37,
            death_ticks,
            ..Default::default()
        }));
        s.overlay_brightness = 0.73;
        let g = render_release::dragon(&s, [0.0; 3], false);
        d.full_draws(&g, &format!("dragon {c}"));
    }
    assert_eq!(d.at, d.bytes.len());
    println!(
        "DRAGON RENDERER10 {}: 32 cases, {} raw-bit comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

pub fn boss_check(path: &str) -> bool {
    let mut d = Dump {
        bytes: std::fs::read(path).expect("read boss overlay dump"),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.int(), 0x30314842);
    assert_eq!(d.int(), 1);
    assert_eq!(d.int(), 64);
    for c in 0..64 {
        let width = d.int();
        let font_width = d.int();
        let health = d.int();
        let present = d.int() != 0;
        let plan = super::render_release::boss_plan(width, font_width, health);
        assert_eq!(
            d.int() as usize,
            if present { plan.rectangles.len() } else { 0 }
        );
        if present {
            for rectangle in plan.rectangles {
                for actual in rectangle {
                    let expected = d.int();
                    d.equal(
                        expected as u32 as u64,
                        actual as u32 as u64,
                        &format!("boss {c}"),
                    );
                }
            }
        }
        assert_eq!(d.int(), present as i32);
        if present {
            assert_eq!(d.string(), "Boss health");
            for actual in plan.text {
                assert_eq!(d.int(), actual);
            }
        }
        assert_eq!(d.int(), 1, "original boss reference consumed once");
    }
    assert_eq!(d.at, d.bytes.len());
    println!(
        "BOSS OVERLAY10 {}: 64 cases, {} exact comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

pub fn registry_check(path: &str) -> bool {
    use super::{manager, tile_manager};
    let mut d = Dump {
        bytes: std::fs::read(path).expect("read rendering registry dump"),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.int(), 0x30315252);
    assert_eq!(d.int(), 1);
    assert_eq!(d.int() as usize, manager::RENDERERS.len());
    let mut registered = std::collections::BTreeSet::new();
    for _ in manager::RENDERERS {
        let class = d.string();
        assert!(
            registered.insert(class.clone()),
            "duplicate original renderer"
        );
        let renderer = manager::RENDERERS
            .iter()
            .find(|r| r.entity_class == class)
            .expect("original entity registration present");
        assert_eq!(d.string(), renderer.renderer_class);
        d.f(renderer.shadow_size, &class);
        d.f(renderer.shadow_opacity, &class);
    }
    assert_eq!(
        d.string(),
        manager::RENDERERS[11].renderer_class,
        "PigZombie inherits Zombie"
    );
    assert_eq!(d.int() as usize, tile_manager::RENDERERS.len());
    registered.clear();
    for _ in tile_manager::RENDERERS {
        let class = d.string();
        assert!(registered.insert(class.clone()));
        let renderer = tile_manager::RENDERERS
            .iter()
            .find(|(name, _)| *name == class)
            .expect("original tile registration present");
        assert_eq!(d.string(), renderer.1);
    }
    assert_eq!(d.at, d.bytes.len());
    println!(
        "RENDER REGISTRIES10 {}: all entities and tiles, {} raw-bit comparisons, {} mismatches",
        if d.bad == 0 { "EXACT" } else { "DIFFER" },
        d.checks,
        d.bad
    );
    d.bad == 0
}

/// The release inventory delta, using the same production RenderBlocks adapter.
pub fn block_inventory_check(path: &str) -> bool {
    let mut d = Dump { bytes: std::fs::read(path).expect("block inventory dump"), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.int(), 0x30314942); assert_eq!(d.int(), 1);
    let count = d.int();
    for case in 0..count {
        let id=d.int() as u8; let meta=d.int(); let brightness=d.float(); let tint=d.int()!=0;
        let mut gl=super::fixed::Fixed::new(1.0);
        super::inventory::inventory(&mut gl,id,meta,brightness,tint);
        d.full_draws(&gl,&format!("inventory[{case}] id {id} meta {meta}"));
    }
    assert_eq!(d.at,d.bytes.len(),"inventory EOF");
    println!("BLOCK INVENTORY10 {}: {count} cases, {} raw-bit comparisons, {} mismatches",if d.bad==0 {"EXACT"}else{"DIFFER"},d.checks,d.bad);
    d.bad==0
}

/// Full sky draws and final GL state, with world color/time values supplied by the fixture.
pub fn global_sky_check(path: &str) -> bool {
    let mut d = Dump { bytes: std::fs::read(path).expect("sky dump"), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.int(), 0x30314752); assert_eq!(d.int(), 1);
    let count = d.int();
    for case in 0..count {
        let dimension=d.int(); let anaglyph=d.int()!=0; let angle=d.float(); let rain=d.float(); let stars=d.float();
        let phase=d.int(); let height=f64::from_bits(u64::from_le_bytes(d.take())); let rgb=std::array::from_fn(|_|d.float());
        let gl=crate::render::sky_release_check::draw(dimension,anaglyph,angle,rain,stars,phase,height,rgb);
        d.full_draws(&gl,&format!("sky[{case}]"));
    }
    assert_eq!(d.at,d.bytes.len(),"sky EOF");
    println!("GLOBAL SKY10 {}: {count} cases, {} raw-bit comparisons, {} mismatches",if d.bad==0 {"EXACT"}else{"DIFFER"},d.checks,d.bad);
    d.bad==0
}
