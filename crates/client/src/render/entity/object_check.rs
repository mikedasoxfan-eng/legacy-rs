//! Raw-bit comparison of original renderer draws and GL inputs.
use super::{
    fixed::Draw,
    objects::{ObjectKind, ObjectState},
};
struct Input {
    data: Vec<u8>,
    at: usize,
    checks: usize,
    bad: usize,
}
impl Input {
    fn bytes<const N: usize>(&mut self) -> [u8; N] {
        let r = self.data[self.at..self.at + N].try_into().unwrap();
        self.at += N;
        r
    }
    fn u(&mut self) -> u32 {
        u32::from_le_bytes(self.bytes())
    }
    fn f(&mut self) -> f32 {
        f32::from_bits(self.u())
    }
    fn d(&mut self) -> f64 {
        f64::from_bits(u64::from_le_bytes(self.bytes()))
    }
    fn eq(&mut self, r: u32, path: &str) {
        let j = self.u();
        self.checks += 1;
        if j != r {
            if self.bad < 16 {
                eprintln!("{path}: java {j:08x}, rust {r:08x}");
            }
            self.bad += 1;
        }
    }
    fn draw(&mut self, r: &Draw, path: &str) {
        self.eq(r.texture, &format!("{path}.texture"));
        for (i, v) in r.material.iter().chain(r.normal.iter()).enumerate() {
            self.eq(v.to_bits(), &format!("{path}.color/normal[{i}]"));
        }
        for (name, v) in [("flags", r.flags), ("src", r.src), ("dst", r.dst), ("depth", r.depth)] {
            self.eq(v, &format!("{path}.{name}"));
        }
        let ops = self.u() as usize;
        assert_eq!(ops, r.ops.len(), "{path} matrix op count");
        for (i, o) in r.ops.iter().enumerate() {
            self.eq(o.code, &format!("{path}.op[{i}]"));
            for (k, v) in o.args.iter().enumerate() {
                self.eq(v.to_bits(), &format!("{path}.op[{i}][{k}]"));
            }
        }
        let n = self.u() as usize;
        assert_eq!(n, r.vertices.len(), "{path} vertex count");
        for (i, v) in r.vertices.iter().enumerate() {
            for (k, v) in v.pos.iter().chain(v.uv.iter()).enumerate() {
                self.eq(v.to_bits(), &format!("{path}.vertex[{i}][{k}]"));
            }
            self.eq(v.color, &format!("{path}.vertex[{i}].rgba"));
            self.eq(v.normal, &format!("{path}.vertex[{i}].normal"));
        }
    }
}
pub fn check(path: &str) -> bool {
    let mut d = Input { data: std::fs::read(path).unwrap(), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.u(), u32::from_le_bytes(*b"R173"));
    let version=d.u();
    assert!(matches!(version,1|2));
    let n = d.u();
    let mut draws = 0;
    for case in 0..n {
        let kind = d.u();
        let id = d.u() as i16;
        let count = d.u() as u8;
        let damage = d.u() as i16;
        let age = d.u() as i32;
        let hover = d.f();
        let fuse = d.u() as i32;
        let shake = d.u() as i32;
        let partial = d.f();
        let brightness = d.f();
        let view = [d.f(), d.f()];
        let yaw = d.f();
        let pitch = d.f();
        let position = [d.d(), d.d(), d.d()];
        let light = std::array::from_fn(|_| d.f());
        let original_kind = kind;
        let kind = match kind {
            0 => ObjectKind::Item { item: worldgen::world::tile::ItemStack { id, count, damage }, age, hover },
            1 => ObjectKind::Falling { block: id as u8, light },
            2 => ObjectKind::Tnt { fuse },
            3 => ObjectKind::Arrow { yaw, pitch, shake, visible: true },
            4 => ObjectKind::Sprite { icon: 14, scale: 2.0 },
            5 => ObjectKind::Sprite { icon: 14, scale: 0.5 },
            6 | 7 => ObjectKind::Item { item: worldgen::world::tile::ItemStack { id, count, damage }, age, hover },
            _ => panic!("kind"),
        };
        let s = ObjectState { kind, position, partial, brightness };
        let r = if original_kind == 6 || original_kind == 7 {
            let item = worldgen::world::tile::ItemStack { id, count, damage };
            if original_kind == 6 {
                super::gui_item::geometry(&item, 12, 9, true)
            } else {
                super::gui_item::durability(&item, 12, 9)
            }
        } else {
            crate::world_effects::Effects::object_draws(&s, [0.0; 3], view, version==1)
        };
        let nd = d.u() as usize;
        assert_eq!(nd, r.draws.len(), "case {case} id {id}: draw count");
        draws += nd;
        let before = d.bad;
        for (i, r) in r.draws.iter().enumerate() {
            d.draw(r, &format!("case {case} id {id} draw {i}"));
        }
        if d.bad > before {
            eprintln!("case {case} id {id}: {} differences", d.bad - before);
        }
    }
    assert_eq!(d.at, d.data.len(), "trailing renderer bytes");
    println!("renderers: {n} cases, {draws} draws, {} bit comparisons, {} differ", d.checks, d.bad);
    d.bad == 0
}

pub fn vehicle_check(path:&str)->bool {
    use super::vehicles::{self,RailSnapshot};
    use worldgen::entity::Entity;
    let mut d=Input {data:std::fs::read(path).unwrap(),at:0,checks:0,bad:0};
    assert_eq!(d.u(),u32::from_le_bytes(*b"V173"));assert_eq!(d.u(),1);let n=d.u();
    let mut w=worldgen::World::new(0,300);let mut draws=0;
    for case in 0..n {
        let t=d.u() as i32;let meta=d.u() as i32;let hit=d.u() as i32;let damage=d.u() as i32;let dir=d.u() as i32;
        let partial=d.f();let brightness=d.f();let prev_yaw=d.f();let yaw=d.f();let prev_pitch=d.f();let pitch=d.f();
        let last:[f64;3]=std::array::from_fn(|_|d.d());let pos:[f64;3]=std::array::from_fn(|_|d.d());let rel:[f64;3]=std::array::from_fn(|_|d.d());
        let mut e=if t<0 {Entity::boat(&mut w,pos[0],pos[1],pos[2])} else {Entity::minecart(&mut w,pos[0],pos[1],pos[2],t)};
        e.set_position(pos[0],pos[1],pos[2]);e.last_tick_pos=worldgen::entity::Vec3D::new(last[0],last[1],last[2]);
        e.prev_yaw=prev_yaw;e.yaw=yaw;e.prev_pitch=prev_pitch;e.pitch=pitch;
        let mut s=vehicles::state(&e,partial).unwrap();s.time_since_hit=hit;s.damage=damage;s.rock_direction=dir;
        let rails=RailSnapshot::capture(&e,|x,y,z|if meta>=0 && (x,y,z)==(0,70,-3) {(66,meta)} else {(0,0)});
        s.track=rails.sample(s.position);
        let mut r=super::fixed::Fixed::new(brightness);vehicles::render(&mut r,s,rel,partial,brightness);
        let nd=d.u() as usize;assert_eq!(nd,r.draws.len(),"vehicle case {case} draw count");draws+=nd;
        for (i,r) in r.draws.iter().enumerate() {d.draw(r,&format!("vehicle case {case} draw {i}"));}
    }
    assert_eq!(d.at,d.data.len());println!("vehicles: {n} cases, {draws} draws, {} raw-bit comparisons, {} differences",d.checks,d.bad);
    d.bad==0
}

pub fn effects_check(path: &str) -> bool {
    use super::effects::Surface;
    let mut d = Input { data: std::fs::read(path).unwrap(), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.u(), u32::from_le_bytes(*b"E173"));
    assert_eq!(d.u(), 1);
    let n = d.u();
    let mut world = worldgen::World::new(0, 1);
    for case in 0..n {
        let k = d.u();
        let effect = d.u();
        let partial = d.f();
        let width = d.f();
        let height = d.f();
        let min_y = d.d();
        let pos = [d.d(), d.d(), d.d()];
        let prev = [d.d(), d.d(), d.d()];
        let camera = [d.d(), d.d(), d.d()];
        let yaw = d.f();
        let mut surfaces = Vec::new();
        let ns = d.u();
        for _ in 0..ns {
            let xyz = [d.u() as i32, d.u() as i32, d.u() as i32];
            let block = d.u() as u8;
            let brightness = d.f();
            surfaces.push(Surface { xyz, block, brightness });
        }
        let mut e = worldgen::entity::Entity::item(
            &mut world,
            pos[0],
            pos[1],
            pos[2],
            worldgen::world::tile::ItemStack::new(1, 1, 0),
        );
        e.width = width;
        e.height = height;
        e.bounding_box.min_y = min_y;
        e.last_tick_pos = worldgen::entity::Vec3D::new(prev[0], prev[1], prev[2]);
        e.fire = 10;
        e.kind = match k {
            0 => worldgen::entity::EntityKind::Item {
                item: worldgen::world::tile::ItemStack::new(1, 1, 0),
                age: 0,
                pickup_delay: 0,
                health: 5,
                hover_start: 0.0,
                updates: 0,
            },
            1 => worldgen::entity::EntityKind::Zombie,
            2 => worldgen::entity::EntityKind::Cow,
            3 => worldgen::entity::EntityKind::Spider,
            4 => worldgen::entity::EntityKind::Ghast(Default::default()),
            5 => worldgen::entity::EntityKind::Giant,
            _ => unreachable!(),
        };
        let r = if effect == 0 {
            super::effects::shadow(&e, &surfaces, partial, camera)
        } else {
            super::effects::fire(&e, partial, camera, yaw)
        };
        let nd = d.u() as usize;
        assert_eq!(nd, r.draws.len(), "case {case}: draws");
        for (i, r) in r.draws.iter().enumerate() {
            d.draw(r, &format!("effects {case} draw {i}"));
        }
    }
    assert_eq!(d.at, d.data.len());
    println!("effects: {n} cases, {} bit comparisons, {} differ", d.checks, d.bad);
    d.bad == 0
}

pub fn living_check(path: &str) -> bool {
    use super::living::{MobKind, RenderState};
    let mut d = Input { data: std::fs::read(path).unwrap(), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.u(), u32::from_le_bytes(*b"L173"));
    assert_eq!(d.u(), 1);
    let n = d.u();
    for case in 0..n {
        let kind = d.u();
        let c = d.u() as i32;
        let partial = d.f();
        let brightness = d.f();
        let mut s = RenderState::new(match kind {
            0 => MobKind::Ghast,
            1 => MobKind::PigZombie,
            2 => MobKind::Giant,
            _ => unreachable!(),
        });
        s.partial = partial;
        s.brightness = brightness;
        s.position = [-0.25, 10.5, 3.75];
        s.body_yaw = -33.0 + (17.0 - -33.0) * partial;
        s.yaw = -75.0 + (35.0 - -75.0) * partial;
        s.pitch = 25.0 + (-12.0 - 25.0) * partial;
        s.limb_amount = 0.2 + (0.9 - 0.2) * partial;
        s.limb_swing = 3.75 - 0.9 * (1.0 - partial);
        s.age = (c * 127) as f32 + partial;
        s.attack = 0.15 + (0.65 - 0.15) * partial;
        s.hurt_time = if c % 4 == 1 { 3 } else { 0 };
        s.death_time = if c % 4 == 2 { c } else { 0 };
        s.ghast_attack = (c - 5) as f32 + 10.0 * partial;
        s.ghast_attacking = c % 2 != 0;
        let r = super::special::render(&s, [0.0; 3], c >= 8);
        let nd = d.u() as usize;
        assert_eq!(nd, r.draws.len(), "living case {case}: draws");
        for (i, r) in r.draws.iter().enumerate() {
            d.draw(r, &format!("living case {case} draw {i}"));
        }
    }
    assert_eq!(d.at, d.data.len());
    println!("living renderers: {n} cases, {} bit comparisons, {} differ", d.checks, d.bad);
    d.bad == 0
}
