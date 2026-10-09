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
        for (name, v) in [
            ("flags", r.flags),
            ("src", r.src),
            ("dst", r.dst),
            ("depth", r.depth),
        ] {
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
    let mut d = Input {
        data: std::fs::read(path).unwrap(),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.u(), u32::from_le_bytes(*b"R173"));
    let version = d.u();
    assert!(matches!(version, 1 | 2));
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
            0 => ObjectKind::Item {
                item: worldgen::entity::player::ItemStack::new(id as i32,count as i32,damage as i32),
                age,
                hover,
            },
            1 => ObjectKind::Falling {
                block: id as u8,
                light,
            },
            2 => ObjectKind::Tnt { fuse },
            3 => ObjectKind::Arrow {
                yaw,
                pitch,
                shake,
                visible: true,
            },
            8 => ObjectKind::XPOrb {
                value: id as i32,
                ticks: age,
            },
            // 1.0 RenderFireball draws the fire charge icon (46) regardless of the entity.
            4 => ObjectKind::Sprite {
                icon: 46,
                scale: 2.0,
            },
            5 => ObjectKind::Sprite {
                icon: 14,
                scale: 0.5,
            },
            6 | 7 => ObjectKind::Item {
                item: worldgen::entity::player::ItemStack::new(id as i32,count as i32,damage as i32),
                age,
                hover,
            },
            _ => panic!("kind"),
        };
        let s = ObjectState {
            kind,
            position,
            partial,
            brightness,
        };
        let r = if original_kind == 6 || original_kind == 7 {
            let item = worldgen::world::tile::ItemStack::new(id as i32,count as i32,damage as i32);
            if original_kind == 6 {
                super::gui_item::geometry(&item, 12, 9, true)
            } else {
                super::gui_item::durability(&item, 12, 9)
            }
        } else {
            crate::world_effects::Effects::object_draws(&s, [0.0; 3], view, version == 1)
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
    println!(
        "renderers: {n} cases, {draws} draws, {} bit comparisons, {} differ",
        d.checks, d.bad
    );
    d.bad == 0
}

pub fn sign_check(path: &str) -> bool {
    let mut d = Input {
        data: std::fs::read(path).unwrap(),
        at: 0,
        checks: 0,
        bad: 0,
    };
    let n = d.u();
    for case in 0..n {
        let block = d.u() as u8;
        let metadata = d.u() as i32;
        let edited = d.u() as i32;
        let sign = worldgen::sign::Sign {
            text: [
                "Hello!".into(),
                "012345678901234".into(),
                "a W i".into(),
                "".into(),
            ],
            line_being_edited: edited,
            editable: true,
        };
        let g = crate::render::sign::render(&sign, block, metadata, [-0.25, 10.5, 3.75], 0.73);
        assert_eq!(d.u() as usize, g.draws.len(), "sign {case} draw count");
        for (i, r) in g.draws.iter().enumerate() {
            d.draw(r, &format!("sign {case} draw {i}"));
        }
    }
    assert_eq!(d.at, d.data.len());
    println!("sign renderers: {} values, {} differences", d.checks, d.bad);
    d.bad == 0
}

#[cfg(test)]
mod sign_font11_tests {
    use super::*;
    #[test]
    #[ignore="requires original NORMAL_PACK11_FIXTURE"]
    fn original_normal_packing() {
        let bytes=std::fs::read(std::env::var("NORMAL_PACK11_FIXTURE").unwrap()).unwrap();
        let words:Vec<_>=bytes.chunks_exact(4).map(|word|u32::from_le_bytes(word.try_into().unwrap())).collect();
        assert_eq!(words.len(),1+words[0] as usize*5);
        for (index,record) in words[1..].chunks_exact(5).enumerate() {
            let normal=glam::Vec3::new(f32::from_bits(record[0]),f32::from_bits(record[1]),f32::from_bits(record[2]));
            assert_eq!(super::super::living::pack_normal(normal),record[3],"vector {index}: {normal:?}");
            let mut fixed=super::super::fixed::Fixed::new(1.0);fixed.set_normal(normal.x,normal.y,normal.z);
            assert_eq!(fixed.vertex_normal,Some(record[3]));assert_eq!(record[4],1);
        }
        println!("NORMAL PACK EXACT: {} original Tessellator vectors",words[0]);
    }
    fn phase(input: &mut Input, draws: &[Draw], case: &str) {
        let default=input.u();
        let pages:Vec<_>=(0..256).map(|_|input.u()).collect();
        assert_eq!(input.u() as usize,draws.len(),"{case} draw count");
        for (index,draw) in draws.iter().enumerate() {
            let texture=u32::from_le_bytes(input.data[input.at..input.at+4].try_into().unwrap());
            let logical=if texture==default {14} else if let Some(page)=pages.iter().position(|&id|id!=0 && id==texture) {0x8000_0000|page as u32} else {texture};
            // Normalize allocator identities only; retain glyph page identity.
            input.data[input.at..input.at+4].copy_from_slice(&logical.to_le_bytes());
            input.draw(draw,&format!("{case} draw {index}"));
        }
    }
    #[test]
    #[ignore="requires original SIGN_FONT11_FIXTURE and installed 1.1 assets"]
    fn original_sign_then_gui_shared_font() {
        let mut input=Input{data:std::fs::read(std::env::var("SIGN_FONT11_FIXTURE").unwrap()).unwrap(),at:0,checks:0,bad:0};
        assert_eq!(input.u(),0x53463131);let cases=input.u();
        let mut jar=crate::assets::Jar::open().unwrap();
        let mut font=crate::gui::font::FontRenderer::new(&jar.image("font/default.png").unwrap(),14,&jar.text("font.txt").unwrap());
        for case in 0..cases {
            font.unicode=input.u()!=0;font.bidi=input.u()!=0;
            let (block,metadata,edited)=(input.u() as u8,input.u() as i32,input.u() as i32);
            let seed=i64::from_le_bytes(input.bytes());
            let text=std::array::from_fn(|_|{let n=input.u();String::from_utf16(&(0..n).map(|_|input.u() as u16).collect::<Vec<_>>()).unwrap()});
            let sign=worldgen::sign::Sign{text,line_being_edited:edited,editable:true};
            *font.random.lock().unwrap()=worldgen::JavaRandom::new(seed);
            let world_font=font.world_snapshot();
            assert!(std::sync::Arc::ptr_eq(&font.random,&world_font.random));
            let world=crate::render::sign::render_with_font(&sign,block,metadata,[-0.25,10.5,3.75],0.73,&world_font);
            phase(&mut input,&world.draws,&format!("sign {case}"));
            let mut gui=super::super::fixed::Fixed::new(1.0);
            font.draw_fixed(&mut gui,"§kABC אב",7,11,0x7fabcdef);
            phase(&mut input,&gui.draws,&format!("GUI after sign {case}"));
            assert_eq!(font.random.lock().unwrap().next_long(),i64::from_le_bytes(input.bytes()),"shared RNG case {case}");
        }
        assert_eq!(input.at,input.data.len());assert_eq!(input.bad,0);
        println!("SIGN FONT EXACT: {cases} world/GUI sequences, {} bit comparisons",input.checks);
    }
}
pub fn painting_check(path: &str) -> bool {
    let mut d = Input {
        data: std::fs::read(path).unwrap(),
        at: 0,
        checks: 0,
        bad: 0,
    };
    let n = d.u();
    let mut w = worldgen::decoration_verify::flat_world();
    for case in 0..n {
        let art = d.u() as usize;
        let dir = d.u() as i32;
        let mut e = worldgen::entity::Entity::painting_base(&mut w);
        if let worldgen::entity::EntityKind::Painting(s) = &mut e.kind {
            s.art = art;
            s.tile = [0, 67, 0];
        }
        e.set_painting_direction(dir);
        let a = worldgen::entity::painting::ARTS[art];
        // The legacy dump records scalar tile brightness; 1.0 draws with glColor 1
        // and the lightmap (checked by the renderfx painting fixture).
        let tiles = (a.width / 16 * (a.height / 16)) as usize;
        for _ in 0..tiles { d.f(); }
        let g = crate::render::painting::render(&e, [-0.25, 10.5, 3.75], &vec![0; tiles]);
        assert_eq!(d.u() as usize, g.draws.len(), "painting {case} draw count");
        for (i, r) in g.draws.iter().enumerate() {
            d.draw(r, &format!("painting {case} draw {i}"));
        }
    }
    assert_eq!(d.at, d.data.len());
    println!(
        "painting renderers: {} values, {} differences",
        d.checks, d.bad
    );
    d.bad == 0
}
pub fn vehicle_check(path: &str) -> bool {
    use super::vehicles::{self, RailSnapshot};
    use worldgen::entity::Entity;
    let mut d = Input {
        data: std::fs::read(path).unwrap(),
        at: 0,
        checks: 0,
        bad: 0,
    };
    assert_eq!(d.u(), u32::from_le_bytes(*b"V173"));
    let version = d.u();
    assert!(version == 1 || version == 2);
    let n = d.u();
    let mut w = worldgen::World::new(0, 300);
    let mut draws = 0;
    for case in 0..n {
        let t = d.u() as i32;
        let meta = d.u() as i32;
        let hit = d.u() as i32;
        let damage = d.u() as i32;
        let dir = d.u() as i32;
        let entity_id = if version >= 2 { Some(d.u() as i32) } else { None };
        let partial = d.f();
        let brightness = d.f();
        let prev_yaw = d.f();
        let yaw = d.f();
        let prev_pitch = d.f();
        let pitch = d.f();
        let last: [f64; 3] = std::array::from_fn(|_| d.d());
        let pos: [f64; 3] = std::array::from_fn(|_| d.d());
        let rel: [f64; 3] = std::array::from_fn(|_| d.d());
        let mut e = if t < 0 {
            Entity::boat(&mut w, pos[0], pos[1], pos[2])
        } else {
            Entity::minecart(&mut w, pos[0], pos[1], pos[2], t)
        };
        e.set_position(pos[0], pos[1], pos[2]);
        if let Some(id) = entity_id { e.entity_id = id; }
        e.last_tick_pos = worldgen::entity::Vec3D::new(last[0], last[1], last[2]);
        e.prev_yaw = prev_yaw;
        e.yaw = yaw;
        e.prev_pitch = prev_pitch;
        e.pitch = pitch;
        let mut s = vehicles::state(&e, partial).unwrap();
        s.time_since_hit = hit;
        s.damage = damage;
        s.rock_direction = dir;
        let rails = RailSnapshot::capture(&e, |x, y, z| {
            if meta >= 0 && (x, y, z) == (0, 70, -3) {
                (66, meta)
            } else {
                (0, 0)
            }
        });
        s.track = rails.sample(s.position);
        let mut r = super::fixed::Fixed::new(brightness);
        vehicles::render(&mut r, s, rel, partial, brightness);
        let nd = d.u() as usize;
        assert_eq!(nd, r.draws.len(), "vehicle case {case} draw count");
        draws += nd;
        for (i, r) in r.draws.iter().enumerate() {
            d.draw(r, &format!("vehicle case {case} draw {i}"));
        }
    }
    assert_eq!(d.at, d.data.len());
    println!(
        "vehicles: {n} cases, {draws} draws, {} raw-bit comparisons, {} differences",
        d.checks, d.bad
    );
    d.bad == 0
}

pub fn effects_check(path: &str) -> bool {
    use super::effects::Surface;
    let mut d = Input {
        data: std::fs::read(path).unwrap(),
        at: 0,
        checks: 0,
        bad: 0,
    };
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
            surfaces.push(Surface {
                xyz,
                block,
                brightness,
            });
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
    println!(
        "effects: {n} cases, {} bit comparisons, {} differ",
        d.checks, d.bad
    );
    d.bad == 0
}

pub fn living_check(path: &str) -> bool {
    use super::living::{MobKind, RenderState};
    let mut d = Input {
        data: std::fs::read(path).unwrap(),
        at: 0,
        checks: 0,
        bad: 0,
    };
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
    println!(
        "living renderers: {n} cases, {} bit comparisons, {} differ",
        d.checks, d.bad
    );
    d.bad == 0
}

pub fn fish_check(path: &str) -> bool {
    let mut d = Input {
        data: std::fs::read(path).unwrap(),
        at: 0,
        checks: 0,
        bad: 0,
    };
    let n = d.u();
    let mut w = worldgen::decoration_verify::flat_world();
    let mut p = worldgen::entity::Entity::player(&mut w, "Player");
    let mut f = worldgen::entity::Entity::fish_base(&mut w);
    f.prev_pos = worldgen::entity::Vec3D::new(-1.5, 65.25, 2.75);
    f.set_position(0.25, 64.5, -0.75);
    p.prev_pos = worldgen::entity::Vec3D::new(1.5, 66.25, 3.75);
    p.set_position(-0.25, 65.5, 0.75);
    for case in 0..n {
        let third = d.u() != 0;
        let yaw = d.f();
        let swing = d.f();
        let partial = d.f();
        p.prev_yaw = yaw - 17.0;
        p.yaw = yaw;
        p.prev_pitch = -23.0;
        p.pitch = 37.0;
        p.living.prev_render_yaw_offset = yaw + 11.0;
        p.living.render_yaw_offset = yaw - 7.0;
        p.living.prev_swing_progress = swing;
        p.living.swing_progress = (swing + 0.2) % 1.0;
        let g = crate::render::fishing::render(
            &f,
            Some(&p),
            [-0.25, 10.5, 3.75],
            partial,
            0.73,
            [37.0, -17.0],
            third,
        );
        assert_eq!(d.u() as usize, g.draws.len());
        assert_eq!(g.draws[1].mode, 3);
        assert_eq!(g.draws[1].batch().vertices.len(), 17);
        for (i, r) in g.draws.iter().enumerate() {
            d.draw(r, &format!("fish {case} draw {i}"));
        }
    }
    assert_eq!(d.at, d.data.len());
    println!("fish renderers: {} values, {} differences", d.checks, d.bad);
    d.bad == 0
}

pub fn chest_check(path: &str) -> bool {
    let mut d = Input { data: std::fs::read(path).unwrap(), at: 0, checks: 0, bad: 0 };
    let cases = d.u();
    for case in 0..cases {
        let adj = d.u() as i32;
        let metadata = d.u() as i32;
        let lid = d.f(); let prev_lid = d.f(); let partial = d.f();
        let g = if adj < 0 { super::chest::item() } else { super::chest::render(&super::chest::ChestState {
            position: [-0.25,10.5,3.75], metadata, adjacent: std::array::from_fn(|i| adj & (1<<i) != 0), lid, prev_lid, partial,
        }) };
        assert_eq!(d.u() as usize, g.draws.len(), "chest {case} draw count");
        for (i,r) in g.draws.iter().enumerate() { d.draw(r, &format!("chest {case} draw {i}")); }
    }
    assert_eq!(d.at, d.data.len());
    println!("chest renderers: {cases} cases, {} values, {} differences", d.checks, d.bad);
    d.bad == 0
}

pub fn new_living_check(path:&str)->bool {
 use super::living::{MobKind,RenderState};
 let mut d=Input{data:std::fs::read(path).unwrap(),at:0,checks:0,bad:0};let n=d.u();
 for case in 0..n {let kind=d.u();let c=d.u() as i32;let arrows=d.u() as i32;let partial=d.f();let brightness=d.f();let packed=d.u() as i32;
  let mut s=RenderState::new(match kind {0=>MobKind::Enderman,1=>MobKind::Silverfish,2=>MobKind::Spider,3=>MobKind::CaveSpider,4=>MobKind::Zombie,5=>MobKind::Skeleton,6=>MobKind::Pig,7=>MobKind::Cow,8=>MobKind::Sheep,9=>MobKind::Chicken,10=>MobKind::Squid,11=>MobKind::Wolf,12=>MobKind::Creeper,13=>MobKind::Ghast,14=>MobKind::PigZombie,15=>MobKind::Giant,16=>MobKind::Slime,17=>MobKind::Zombie,_=>panic!("unknown mob kind {kind}")});
  if kind==17 {let id=d.u() as i32;let metadata=d.u() as i32;s.held_item=None;s.held_stack=Some(worldgen::entity::player::ItemStack::new(id,1,metadata));}
  s.entity_id=c*337;s.arrow_count=arrows;s.partial=partial;s.brightness=1.0;s.overlay_brightness=brightness;s.position=[-0.25,10.5,3.75];
  s.body_yaw=-33.0+(17.0 - -33.0)*partial;s.yaw=-75.0+(35.0 - -75.0)*partial;s.pitch=25.0+(-12.0-25.0)*partial;
  s.limb_amount=0.2+(0.9-0.2)*partial;s.limb_swing=3.75-0.9*(1.0-partial);s.age=(c*127) as f32+partial;s.attack=0.15+(0.65-0.15)*partial;
  s.hurt_time=if c%4==1 {3}else{0};s.death_time=if c%4==2 {c}else{0};
  if kind==0 {s.carried_block=match c%3 {0=>0,1=>1,_=>35};s.carried_data=c%16;s.carrying=s.carried_block>0;s.screaming=c&1!=0;if s.screaming {let mut r=crate::render::weather::ClientRandom::new((42+c) as i64);s.scream_jitter=[r.gaussian()*0.02,r.gaussian()*0.02];}}
  s.saddled=c&1!=0;s.sheared=c&1!=0;s.fleece_color=c as usize;
  s.chicken_flap=0.2+(1.9-0.2)*partial;s.chicken_flap_amount=0.15+(0.8-0.15)*partial;
  s.squid_pitch=-17.0+(35.0 - -17.0)*partial;s.squid_yaw=12.0+(-45.0-12.0)*partial;s.tentacle_angle=0.3+(1.1-0.3)*partial;
  s.wolf.angry=c%8&2!=0;s.wolf.sitting=c%8&1!=0;s.wolf_tamed=c%8&4!=0;s.wolf_health=c+4;s.wolf.interested=[0.2,0.8];s.wolf.shake=[0.1,0.6];s.powered=c&1!=0;s.creeper_flash=(c as f32+3.0*partial)/28.0;
  s.ghast_attack=(c-5) as f32+10.0*partial;s.ghast_attacking=c%2!=0;s.slime_size=1<<(c%3);s.slime_squish=0.2+(0.8-0.2)*partial;
  let g=super::raw_living181::render(&s,[0.0;3],c>=8);assert_eq!(d.u() as usize,g.draws.len(),"new living {case} draw count");for (i,r) in g.draws.iter().enumerate(){d.draw(r,&format!("new living {case} draw {i}"));}
  let lights=super::raw_living181::packed_brightnesses(&g.draws,packed);assert_eq!(d.u() as usize,lights.len());for (i,v) in lights.into_iter().enumerate(){d.eq(v as u32,&format!("new living {case} light {i}"));}
 }
 assert_eq!(d.at,d.data.len());println!("new living renderers: {n} cases, {} values, {} differences",d.checks,d.bad);d.bad==0
}

pub fn bed_check(path:&str)->bool {
 use super::bed_model::*;let mut d=Input{data:std::fs::read(path).unwrap(),at:0,checks:0,bad:0};
 for table in [&X[..],&Z[..],&HEAD_INVISIBLE_FACE[..],&FACE_DIRECTION[..],&FOOT_INVISIBLE_FACE_REMAP[..],&NEXT[..],&PREVIOUS[..]] {for v in table {d.eq(*v as u32,"ModelBed table");}}
 for table in BED_DIRECTION {for v in table {d.eq(v as u32,"ModelBed direction");}}
 assert_eq!(d.at,d.data.len());println!("ModelBed: {} values, {} differences",d.checks,d.bad);d.bad==0
}

pub fn player_living_check(path: &str) -> bool {
    use worldgen::entity::{Entity, EntityKind, player::ItemStack};
    let mut d=Input{data:std::fs::read(path).unwrap(),at:0,checks:0,bad:0};let header=d.u();let round6=header==0x18160006;let count=if round6 {d.u()}else{header};
    let mut renderer=super::player::Renderer::default();let mut w=worldgen::entity::verify181::world();
    for case in 0..count {
        let c=d.u() as i32;let material=d.u() as i32;let id=d.u() as i32;let arrows=d.u() as i32;let using=d.u()!=0;let partial=d.f();let brightness=d.f();
        let mut e=Entity::player(&mut w,"Player");e.entity_id=c*337;e.living.arrows_stuck=arrows;e.set_position(-0.25,10.5,3.75);e.last_tick_pos=e.pos;
        e.living.prev_render_yaw_offset=-33.0;e.living.render_yaw_offset=17.0;e.prev_yaw=-75.0;e.yaw=35.0;e.prev_pitch=25.0;e.pitch=-12.0;
        e.living.prev_limb_amount=0.2;e.living.limb_amount=0.9;e.living.limb_swing=3.75;e.ticks_existed=c*127;e.living.prev_swing_progress=0.15;e.living.swing_progress=0.65;
        e.living.hurt_time=if c%4==1 {3}else{0};e.living.death_time=if c%4==2 {c}else{0};e.sneaking=c%3==0;e.riding=if c%2!=0 {Some(100000)}else{None};
        let p=e.player_state_mut().unwrap();p.inventory.main[0]=Some(ItemStack::new(id,1,0));for j in 0..4 {p.inventory.armor[j]=if material==5 {None}else{Some(ItemStack::new(298+material*4+3-j as i32,1,0))};}p.item_use_ticks=if using {20}else{0};
        let (local,bed)=if round6 {let local=d.u()!=0;let bed=d.u() as i32;let sleep=d.u()!=0;e.living.health=d.u() as i32;let p=e.player_state_mut().unwrap();p.sleeping=sleep;p.bed_offset=worldgen::entity::Vec3D::new(0.1f32 as f64,-0.2f32 as f64,0.3f32 as f64);(local,bed)}else{(false,0)};
        let g=renderer.render_lit(&e,partial,[0.0;3],1.0,brightness,bed,local);assert_eq!(d.u() as usize,g.draws.len(),"player {case} draw count");for (i,r) in g.draws.iter().enumerate(){d.draw(r,&format!("player {case} draw {i}"));}
        assert!(matches!(e.kind,EntityKind::Player(..)));
    }
    assert_eq!(d.at,d.data.len());println!("player renderers: {count} cases, {} values, {} differences",d.checks,d.bad);d.bad==0
}

pub fn arrow_check(path:&str)->bool {
 let mut d=Input{data:std::fs::read(path).unwrap(),at:0,checks:0,bad:0};let n=d.u();
 for case in 0..n {let yaw=d.f();let pitch=d.f();let shake=d.u() as i32;let partial=d.f();let g=super::objects::render(&ObjectState{kind:ObjectKind::Arrow{yaw,pitch,shake,visible:false},position:[-0.25,10.5,3.75],partial,brightness:1.0},[0.0;3],[0.0;2]);assert_eq!(d.u() as usize,g.draws.len());for (i,r) in g.draws.iter().enumerate(){d.draw(r,&format!("arrow {case} draw {i}"));}}
 assert_eq!(d.at,d.data.len());println!("arrows: {n} cases, {} values, {} differences",d.checks,d.bad);d.bad==0
}

/// Round 9: the real manager registry's shadow configuration and sprite bridges.
pub fn manager_visual_check(path: &str) -> bool {
    use worldgen::entity::{Entity, EntityKind, Vec3D};
    let mut d = Input { data: std::fs::read(path).unwrap(), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.u(), 0x3938314d); assert_eq!(d.u(), 1);
    let n = d.u();
    let mut w = worldgen::entity::verify181::world();
    for case in 0..n {
        let kind = d.u(); let partial = d.f(); let fancy = d.u() != 0;
        let mut e = match kind {
            0 => Entity::living(&mut w, "CaveSpider").unwrap(),
            1 => Entity::living(&mut w, "Silverfish").unwrap(),
            2 => Entity::xp_orb_base(&mut w),
            3 | 4 => { let mut e = worldgen::entity::registry::create(&mut w, "Snowball").unwrap(); if kind == 4 {e.kind = EntityKind::Egg(Default::default());} e },
            _ => unreachable!(),
        };
        e.width = d.f(); e.height = d.f(); e.bounding_box.min_y = d.d();
        e.pos = Vec3D::new(d.d(), d.d(), d.d()); e.last_tick_pos = Vec3D::new(d.d(), d.d(), d.d());
        let origin = std::array::from_fn(|_| d.d()); let camera = std::array::from_fn(|_| d.d());
        let view = [d.f(), d.f()];
        let ns = d.u(); let mut surfaces = Vec::new();
        for _ in 0..ns { surfaces.push(super::effects::Surface {xyz: [d.u() as i32, d.u() as i32, d.u() as i32], block: d.u() as u8, brightness: d.f()}); }
        let g = if kind < 3 {
            if fancy {super::effects::shadow_with_camera(&e, &surfaces, partial, origin, camera)} else {super::fixed::Fixed::new(1.0)}
        } else {
            let state = super::bridge::object_state(&e, partial, 1.0, [1.0; 7]).expect("thrown-item bridge");
            super::objects::render(&state, origin, view)
        };
        let nd = d.u() as usize; assert_eq!(nd, g.draws.len(), "manager visuals {case}: draws");
        for (j, draw) in g.draws.iter().enumerate() { d.draw(draw, &format!("manager visuals {case}, draw {j}")); }
    }
    assert_eq!(d.at, d.data.len()); println!("manager visuals: {n} cases, {} comparisons, {} differences", d.checks, d.bad); d.bad == 0
}

/// Release 1.0 RenderBiped held items (potion second pass, glint) against
/// Mode_render_biped4_10; texture matrices come first so the glint clock can be replayed.
pub fn biped_release_check(path: &str) -> bool {
    use super::living::{MobKind, RenderState};
    let mut d = Input { data: std::fs::read(path).unwrap(), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.u(), 0x34425052);
    let n = d.u();
    let mut glints = 0;
    for case in 0..n {
        let c = d.u() as i32;
        let partial = d.f();
        let brightness = d.f();
        let packed = d.u() as i32;
        let (id, metadata, ench) = (d.u() as i32, d.u() as i32, d.u() != 0);
        let lists: Vec<Vec<super::fixed::Op>> = (0..d.u())
            .map(|_| (0..d.u()).map(|_| super::fixed::Op { code: d.u(), args: std::array::from_fn(|_| d.f()) }).collect())
            .collect();
        let mut times = [0.0f32; 2];
        for ops in &lists {
            if ops.len() == 3 && ops[0].code == 1 && ops[0].args[0] == 0.125 && ops[1].code == 0 && ops[2].code == 2 {
                if ops[2].args[0] == -50.0 { times[0] = ops[1].args[0]; } else if ops[2].args[0] == 10.0 { times[1] = -ops[1].args[0]; }
            }
        }
        let mut s = RenderState::new(MobKind::Zombie);
        let stack = worldgen::entity::player::ItemStack::new(id, 1, metadata);
        if ench { stack.add_enchantment(16, 2); }
        s.held_item = None;
        s.held_stack = Some(stack);
        s.entity_id = c * 337; s.partial = partial; s.brightness = 1.0; s.overlay_brightness = brightness; s.position = [-0.25, 10.5, 3.75];
        s.body_yaw = -33.0 + (17.0 - -33.0) * partial; s.yaw = -75.0 + (35.0 - -75.0) * partial; s.pitch = 25.0 + (-12.0 - 25.0) * partial;
        s.limb_amount = 0.2 + (0.9 - 0.2) * partial; s.limb_swing = 3.75 - 0.9 * (1.0 - partial); s.age = (c * 127) as f32 + partial; s.attack = 0.15 + (0.65 - 0.15) * partial;
        s.hurt_time = if c % 4 == 1 { 3 } else { 0 }; s.death_time = if c % 4 == 2 { c } else { 0 };
        crate::render::first_person::override_glint_times(Some(times));
        let g = super::raw_living181::render(&s, [0.0; 3], c >= 8);
        crate::render::first_person::override_glint_times(None);
        glints += g.draws.iter().filter(|x| x.texture == 76).count();
        let nd = d.u() as usize;
        d.checks += 1;
        if nd != g.draws.len() { d.bad += 1; eprintln!("biped {case} ({id}:{metadata}) draw count: java {nd}, rust {}", g.draws.len()); return false; }
        for (i, r) in g.draws.iter().enumerate() { d.draw(r, &format!("biped {case} draw {i}")); }
        let lights = super::raw_living181::packed_brightnesses(&g.draws, packed);
        assert_eq!(d.u() as usize, lights.len());
        for (i, v) in lights.into_iter().enumerate() { d.eq(v as u32, &format!("biped {case} light {i}")); }
        d.checks += 1;
        if lists.len() != g.draws.len() { d.bad += 1; eprintln!("biped {case}: texture-matrix lists {}", lists.len()); continue; }
        for (i, (j, r)) in lists.iter().zip(&g.draws).enumerate() {
            d.checks += 1;
            let same = j.len() == r.texture_ops.len() && j.iter().zip(&r.texture_ops).all(|(a, b)| a.code == b.code && a.args.map(f32::to_bits) == b.args.map(f32::to_bits));
            if !same { d.bad += 1; if d.bad < 16 { eprintln!("biped {case} draw {i} texture matrix: java {j:?}, rust {:?}", r.texture_ops); } }
        }
    }
    assert_eq!(d.at, d.data.len());
    println!("RENDER BIPED10 {}: {n} cases, {glints} glint draws, {} values, {} differences", if d.bad == 0 { "EXACT" } else { "DIFFER" }, d.checks, d.bad);
    d.bad == 0
}

#[test]
#[ignore = "requires original 1.1 sheep renderer fixture"]
fn sheep_live_bridge_models_and_renderer_match_original_11() {
    use super::models::{Model, ModelKind};
    use worldgen::entity::{Entity, EntityKind};
    let path = std::env::var("SHEEP_RENDER11_FIXTURE").unwrap();
    let mut d = Input { data: std::fs::read(path).unwrap(), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.u(), 0x534d3131);
    let n = d.u();
    let mut world = worldgen::World::new(0, 0);
    let mut body = Model::new(ModelKind::Sheep);
    let mut wool = Model::new(ModelKind::SheepFur);
    for case in 0..n {
        let time = d.u() as i32;
        let partial = d.f();
        let child = d.u() != 0; let sheared = d.u() != 0; let hurt = d.u() != 0;
        let color = d.u() as u8; let brightness = d.f(); let packed = d.u() as i32;
        let mut e = Entity::living(&mut world, "Sheep").unwrap();
        e.set_growing_age(if child { -24000 } else { 0 });
        e.kind = EntityKind::Sheep { sheared, fleece_color: color };
        e.living.sheep_graze_timer = time; e.entity_id = 337 * case as i32;
        e.set_position(0.25, 70.5, -2.75);
        e.prev_pos.x = 0.1; e.prev_pos.y = 70.4; e.prev_pos.z = -2.9;
        e.last_tick_pos = e.prev_pos;
        e.living.prev_render_yaw_offset = -33.0; e.living.render_yaw_offset = 17.0;
        e.prev_yaw = -75.0; e.yaw = 35.0; e.prev_pitch = -12.25; e.pitch = 37.5;
        e.living.prev_limb_amount = 0.2; e.living.limb_amount = 0.9; e.living.limb_swing = 3.75;
        e.ticks_existed = 31; e.living.prev_swing_progress = 0.15; e.living.swing_progress = 0.65;
        e.living.hurt_time = if hurt { 3 } else { 0 };
        let mut s = super::bridge::render_state(&e, partial, 1.0).unwrap();
        s.overlay_brightness = brightness;
        d.eq(s.sheep_head_position.to_bits(), "live sheep head position");
        d.eq(s.sheep_head_angle.to_bits(), "live sheep head angle");
        let pose = [s.limb_swing * if child { 3.0 } else { 1.0 }, s.limb_amount,
                    s.age, s.yaw - s.body_yaw, s.pitch, 0.0625];
        for m in [&mut body, &mut wool] {
            m.set_sheep_animations(s.sheep_head_position, s.sheep_head_angle);
            m.set_rotation_angles(pose);
            for value in [m["head"].rotation_point_y, m["head"].rotate_angle_x, m["head"].rotate_angle_y] {
                d.eq(value.to_bits(), "original sheep model head");
            }
        }
        let g = super::raw_living181::render(&s, [0.0, 64.0, 0.0], false);
        assert_eq!(d.u() as usize, g.draws.len(), "sheep {case} draw count");
        for (i, draw) in g.draws.iter().enumerate() { d.draw(draw, &format!("sheep {case} draw {i}")); }
        let lights = super::raw_living181::packed_brightnesses(&g.draws, packed);
        assert_eq!(d.u() as usize, lights.len());
        for light in lights { d.eq(light as u32, "sheep draw light"); }
    }
    assert_eq!(d.at, d.data.len());
    println!("SHEEP LIVE RENDER {}: {n} cases, {} comparisons, {} differences",
        if d.bad == 0 { "EXACT" } else { "DIFFER" }, d.checks, d.bad);
    assert_eq!(d.bad, 0);
}

/// Release 1.0 RenderLiving with baby animals (Mode_render_child4_10).
pub fn child_living_check(path: &str) -> bool {
    use super::living::{MobKind, RenderState};
    let mut d = Input { data: std::fs::read(path).unwrap(), at: 0, checks: 0, bad: 0 };
    assert_eq!(d.u(), 0x34444843);
    let n = d.u();
    let mut children = 0;
    for case in 0..n {
        let kind = d.u();
        let c = d.u() as i32;
        let child = d.u() != 0;
        children += child as usize;
        let partial = d.f();
        let brightness = d.f();
        let packed = d.u() as i32;
        let mut s = RenderState::new(match kind {
            6 => MobKind::Pig,
            7 => MobKind::Cow,
            8 => MobKind::Sheep,
            9 => MobKind::Chicken,
            11 => MobKind::Wolf,
            _ => panic!("child kind {kind}"),
        });
        s.is_child = child;
        s.entity_id = c * 337; s.partial = partial; s.brightness = 1.0; s.overlay_brightness = brightness; s.position = [-0.25, 10.5, 3.75];
        s.body_yaw = -33.0 + (17.0 - -33.0) * partial; s.yaw = -75.0 + (35.0 - -75.0) * partial; s.pitch = 25.0 + (-12.0 - 25.0) * partial;
        s.limb_amount = 0.2 + (0.9 - 0.2) * partial; s.limb_swing = 3.75 - 0.9 * (1.0 - partial); s.age = (c * 127) as f32 + partial; s.attack = 0.15 + (0.65 - 0.15) * partial;
        s.hurt_time = if c % 4 == 1 { 3 } else { 0 }; s.death_time = if c % 4 == 2 { c } else { 0 };
        s.saddled = c & 1 != 0; s.sheared = c & 1 != 0; s.fleece_color = c as usize;
        s.chicken_flap = 0.2 + (1.9 - 0.2) * partial; s.chicken_flap_amount = 0.15 + (0.8 - 0.15) * partial;
        s.wolf.angry = c % 8 & 2 != 0; s.wolf.sitting = c % 8 & 1 != 0; s.wolf_tamed = c % 8 & 4 != 0; s.wolf_health = c + 4; s.wolf.interested = [0.2, 0.8]; s.wolf.shake = [0.1, 0.6];
        let g = super::raw_living181::render(&s, [0.0; 3], c >= 8);
        assert_eq!(d.u() as usize, g.draws.len(), "child living {case} draw count");
        for (i, r) in g.draws.iter().enumerate() { d.draw(r, &format!("child living {case} draw {i}")); }
        let lights = super::raw_living181::packed_brightnesses(&g.draws, packed);
        assert_eq!(d.u() as usize, lights.len());
        for (i, v) in lights.into_iter().enumerate() { d.eq(v as u32, &format!("child living {case} light {i}")); }
    }
    assert_eq!(d.at, d.data.len());
    println!("RENDER LIVING CHILD10 {}: {n} cases ({children} babies), {} values, {} differences", if d.bad == 0 { "EXACT" } else { "DIFFER" }, d.checks, d.bad);
    d.bad == 0
}

#[test]
#[ignore = "requires original 1.1 renderer packets"]
fn item_passes_match_original_11() {
    let path=std::env::var("ITEMPASS11_FIXTURE").unwrap();
    let mut d=Input{data:std::fs::read(&path).unwrap(),at:0,checks:0,bad:0};
    assert_eq!(d.u(),u32::from_le_bytes(*b"R11P"));assert_eq!(d.u(),1);let n=d.u();
    for case in 0..n {
        let kind=d.u();let id=d.u() as i32;let damage=d.u() as i32;let count=d.u() as i32;let tint=d.u()!=0;let age=d.u() as i32;
        let hover=d.f();let partial=d.f();let view=[d.f(),d.f()];let position=[d.d(),d.d(),d.d()];let base=std::array::from_fn(|_|d.f());let z=d.f();let x=d.u() as i32;let y=d.u() as i32;
        let mut rng=worldgen::JavaRandom::new(777);
        let g=if kind==0 {super::objects::render_with_random(&ObjectState{kind:ObjectKind::Item{item:worldgen::entity::player::ItemStack::new(id,count,damage),age,hover},position,partial,brightness:1.0},[0.0;3],view,tint,Some(base),&mut rng)} else {super::gui_item::draw_item_values_into_gui(id,damage,x,y,tint,Some(base),z)};
        assert_eq!(d.u() as usize,g.draws.len(),"case {case} draws");for (i,draw) in g.draws.iter().enumerate(){d.draw(draw,&format!("case {case} draw {i}"));}
        let state=u64::from_le_bytes(d.bytes());assert_eq!(state,rng.state() as u64,"case {case} RNG state");let tail=u64::from_le_bytes(d.bytes());assert_eq!(tail,rng.next_long() as u64,"case {case} RNG tail");
    }
    assert_eq!(d.at,d.data.len());println!("{n} cases, {} comparisons, {} differences",d.checks,d.bad);assert_eq!(d.bad,0);
}

#[test]
#[ignore = "requires original 1.1 held primitive packet"]
fn held_item_passes_match_original_11() {
 let path=std::env::var("ITEMPASS11_HELD_FIXTURE").unwrap();let mut d=Input{data:std::fs::read(path).unwrap(),at:0,checks:0,bad:0};
 assert_eq!(d.u(),u32::from_le_bytes(*b"R11H"));assert_eq!(d.u(),1);let n=d.u();
 for case in 0..n {
  let item=worldgen::entity::player::ItemStack::new(d.u() as i32,1,d.u() as i32);let pass=d.u() as i32;
  d.eq(crate::render::first_person::Holder::MOB.icon(item,pass) as u32,"mob icon");d.eq(super::item_table::render_properties(item.id,item.damage,pass).1,"color");d.eq(super::item_table::multiple_passes(item.id) as u32,"multiple passes");
  let mut g=super::fixed::Fixed::new(1.0);g.material=std::array::from_fn(|_|d.f());crate::render::first_person::item_pass(&mut g,item,crate::render::first_person::Holder::MOB,pass,[0.0;2]);
  assert_eq!(d.u() as usize,g.draws.len(),"case {case}");for (i,draw) in g.draws.iter().enumerate(){d.draw(draw,&format!("case {case} draw {i}"));}
 }
 assert_eq!(d.at,d.data.len());println!("{n} held cases, {} comparisons, {} differences",d.checks,d.bad);assert_eq!(d.bad,0);
}
