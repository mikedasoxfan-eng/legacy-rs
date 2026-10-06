//! Raw-bit first-person oracle comparison; no window or graphics driver required.
use super::{
    camera::{self, Settings},
    entity::fixed::{Draw, Fixed, Op, RawVertex},
    first_person::{self, Equip},
    targeting,
};
use worldgen::{
    entity::player::ItemStack,
    entity::{Entity, Vec3D},
    world::chunk::{idx, Chunk},
    World,
};
struct Input {
    data: Vec<u8>,
    at: usize,
    checks: usize,
    bad: usize,
    label: String,
}
impl Input {
    fn u(&mut self) -> u32 {
        let b = self.data[self.at..self.at + 4].try_into().unwrap();
        self.at += 4;
        u32::from_le_bytes(b)
    }
    fn i(&mut self) -> i32 {
        self.u() as i32
    }
    fn f(&mut self) -> f32 {
        f32::from_bits(self.u())
    }
    fn d(&mut self) -> f64 {
        let b = self.data[self.at..self.at + 8].try_into().unwrap();
        self.at += 8;
        f64::from_bits(u64::from_le_bytes(b))
    }
    fn compare(&mut self, j: u32, r: u32, name: &str) {
        self.checks += 1;
        if j != r {
            if self.bad < 40 || std::env::var("FP_SHOW").is_ok_and(|f| self.label.contains(&f)) {
                eprintln!("{} {name}: java {j:08x}, rust {r:08x}", self.label);
            }
            self.bad += 1;
        }
    }
    fn eq(&mut self, r: u32, name: &str) {
        let j = self.u();
        self.compare(j, r, name);
    }
    fn ops(&mut self) -> Vec<Op> {
        let n = self.u();
        (0..n)
            .map(|_| Op {
                code: self.u(),
                args: std::array::from_fn(|_| self.f()),
            })
            .collect()
    }
    fn compare_ops(&mut self, j: &[Op], r: &[Op], name: &str) {
        self.compare(j.len() as u32, r.len() as u32, &format!("{name} count"));
        for (i, (j, r)) in j.iter().zip(r).enumerate() {
            self.compare(j.code, r.code, &format!("{name}[{i}] code"));
            for k in 0..4 {
                self.compare(
                    j.args[k].to_bits(),
                    r.args[k].to_bits(),
                    &format!("{name}[{i}][{k}]"),
                );
            }
        }
    }
    fn draws(&mut self, r: &Fixed) {
        let n = self.u();
        let mut js = vec![];
        for _ in 0..n {
            let texture = self.u();
            let material = std::array::from_fn(|_| self.f());
            let normal = std::array::from_fn(|_| self.f());
            let flags = self.u();
            let src = self.u();
            let dst = self.u();
            let depth = self.u();
            let ops = self.ops();
            let nv = self.u();
            let vertices = (0..nv)
                .map(|_| RawVertex {
                    pos: std::array::from_fn(|_| self.f()),
                    uv: std::array::from_fn(|_| self.f()),
                    color: self.u(),
                    normal: self.u(),
                })
                .collect();
            js.push(Draw {
                cull: 0,
                alpha_cutoff: 0.1,
                ops,
                texture,
                material,
                normal,
                flags,
                src,
                dst,
                depth,
                vertices,
                mode: 7,
                light_rotation: 0.0,
                light_transform: None,
            });
        }
        let nm = self.u();
        assert_eq!(nm, n);
        for j in &mut js {
            j.mode = self.u();
        }
        for i in 0..n as usize {
            let jm: [f32; 16] = std::array::from_fn(|_| self.f());
            if let Some(r) = r.draws.get(i) {
                for (j, r) in jm.into_iter().zip(r.matrix().to_cols_array()) {
                    self.compare(j.to_bits(), r.to_bits(), "draw modelview matrix");
                }
            }
        }
        self.compare(n, r.draws.len() as u32, "draw count");
        for (i, (j, r)) in js.iter().zip(&r.draws).enumerate() {
            for (name, j, r) in [
                ("texture", j.texture, r.texture),
                ("flags", j.flags, r.flags),
                ("src", j.src, r.src),
                ("dst", j.dst, r.dst),
                ("depth", j.depth, r.depth),
                ("mode", j.mode, r.mode),
            ] {
                self.compare(j, r, &format!("draw {i} {name}"));
            }
            for (k, (j, r)) in j
                .material
                .iter()
                .chain(&j.normal)
                .zip(r.material.iter().chain(&r.normal))
                .enumerate()
            {
                self.compare(
                    j.to_bits(),
                    r.to_bits(),
                    &format!("draw {i} color/normal {k}"),
                );
            }
            self.compare_ops(&j.ops, &r.ops, &format!("draw {i} ops"));
            self.compare(
                j.vertices.len() as u32,
                r.vertices.len() as u32,
                &format!("draw {i} vertices"),
            );
            for (k, (j, r)) in j.vertices.iter().zip(&r.vertices).enumerate() {
                for (v, (j, r)) in j
                    .pos
                    .iter()
                    .chain(&j.uv)
                    .zip(r.pos.iter().chain(&r.uv))
                    .enumerate()
                {
                    self.compare(
                        j.to_bits(),
                        r.to_bits(),
                        &format!("draw {i} vertex {k}[{v}]"),
                    );
                }
                self.compare(j.color, r.color, "vertex color");
                self.compare(j.normal, r.normal, "vertex normal");
            }
        }
    }
    fn entity(&mut self, w: &mut World) -> Entity {
        let mut e = Entity::player(w, "Player");
        e.living.health = self.i();
        e.living.hurt_time = self.i();
        e.living.max_hurt_time = self.i();
        e.living.death_time = self.i();
        e.ticks_existed = self.i();
        e.fire = self.i();
        let sleep = self.i() != 0;
        e.sneaking = self.i() != 0;
        e.yaw = self.f();
        e.prev_yaw = self.f();
        e.pitch = self.f();
        e.prev_pitch = self.f();
        e.y_offset = self.f();
        e.width = self.f();
        e.height = self.f();
        e.distance_walked = self.f();
        e.prev_distance_walked = self.f();
        e.living.swing_progress = self.f();
        e.living.prev_swing_progress = self.f();
        e.living.render_yaw_offset = self.f();
        e.living.prev_render_yaw_offset = self.f();
        e.living.limb_amount = self.f();
        e.living.prev_limb_amount = self.f();
        e.living.limb_swing = self.f();
        let p = e.player_state_mut().unwrap();
        p.sleeping = sleep;
        p.camera_yaw = self.f();
        p.prev_camera_yaw = self.f();
        p.camera_pitch = self.f();
        p.prev_camera_pitch = self.f();
        p.time_in_portal = self.f();
        p.prev_time_in_portal = self.f();
        p.bed_offset = Vec3D::new(self.f() as f64, self.f() as f64, self.f() as f64);
        e.living.attacked_at_yaw = self.f();
        let (x, y, z) = (self.d(), self.d(), self.d());
        e.set_position(x, y, z);
        e.prev_pos = Vec3D::new(self.d(), self.d(), self.d());
        e.last_tick_pos = Vec3D::new(self.d(), self.d(), self.d());
        e
    }
    fn settings(&mut self) -> (Settings, bool) {
        let flags = self.u();
        let eye = self.i();
        let ticks = self.i();
        let distance = self.i();
        let roll = [self.f(), self.f()];
        let fov = [self.f(), self.f()];
        let debug_yaw = [self.f(), self.f()];
        let debug_pitch = [self.f(), self.f()];
        let fov_option = self.f();
        let fov_zoom = Some([self.f(), self.f()]);
        let zoom = self.d();
        let zoom_yaw = self.d();
        let zoom_pitch = self.d();
        (
            Settings {
                bobbing: flags & 1 != 0,
                third_person: flags & 2 != 0,
                anaglyph: flags & 4 != 0,
                debug_camera: flags & 8 != 0,
                eye,
                ticks,
                distance,
                roll,
                fov,
                fov_option,
                fov_zoom,
                debug_yaw,
                debug_pitch,
                zoom,
                zoom_yaw,
                zoom_pitch,
            },
            flags & 16 != 0,
        )
    }
    fn blocks(&mut self, w: &mut World) {
        let n = self.u();
        for _ in 0..n {
            let (x, y, z, id, m) = (self.i(), self.i(), self.i(), self.i(), self.i());
            raw(w, x, y, z, id as u8, m);
        }
    }
}
fn world() -> World {
    let mut w = World::new(0, 0);
    w.skylight_subtracted = 0;
    for x in -3..=3 {
        for z in -3..=3 {
            let mut c = Chunk::new(x, z, Box::new([0; 32768]));
            for bx in 0..16 {
                for bz in 0..16 {
                    c.blocks[idx(bx, 63, bz)] = 1;
                }
            }
            w.chunks.insert((x, z), Box::new(c));
        }
    }
    w
}
fn raw(w: &mut World, x: i32, y: i32, z: i32, id: u8, m: i32) {
    let c = w
        .chunks
        .entry((x >> 4, z >> 4))
        .or_insert_with(|| Box::new(Chunk::new(x >> 4, z >> 4, Box::new([0; 32768]))));
    c.blocks[idx(x & 15, y, z & 15)] = id;
    c.data.set(x & 15, y, z & 15, m);
}
pub fn check(path: &str) -> bool {
    let mut d = Input {
        data: std::fs::read(path).expect("oracle file"),
        at: 0,
        checks: 0,
        bad: 0,
        label: String::new(),
    };
    assert_eq!(d.u(), u32::from_le_bytes(*b"F173"));
    assert_eq!(d.u(), 1);
    let n = d.u();
    let assets = crate::assets::load().expect("jar textures");
    let col = super::blocks::Colorizers {
        grass: assets.grass_color,
        foliage: assets.foliage_color,
    };
    let mut bounds = worldgen::entity::block_table::INITIAL_BOUNDS;
    let mut equip = Equip::default();
    let mut persistent = super::entity::player::Renderer::default();
    for case in 0..n {
        let kind = d.u();
        d.label = format!("case {case} kind {kind}");
        let before = d.bad;
        let mut w = world();
        match kind {
            0 => {
                let e = d.entity(&mut w);
                let t = d.f();
                let (s, _) = d.settings();
                d.blocks(&mut w);
                let medium = camera::medium(&e, &mut w);
                let fov = camera::fov(&e, medium == camera::Medium::Water, t, &s, true);
                d.eq(fov.to_bits(), "fov");
                let j = d.ops();
                let mut p = Fixed::new(1.0);
                if s.anaglyph {
                    p.translate(-(s.eye * 2 - 1) as f32 * 0.07, 0.0, 0.0);
                }
                if s.zoom != 1.0 {
                    p.translate(s.zoom_yaw as f32, -s.zoom_pitch as f32, 0.0);
                    p.scale(s.zoom as f32, s.zoom as f32, 1.0);
                }
                d.compare_ops(&j, &p.ops, "projection ops");
                for v in camera::projection_gl(fov, 853.0 / 480.0, (256 >> s.distance) as f32 * 2.0)
                    .to_cols_array()
                {
                    d.eq(v.to_bits(), "projection matrix");
                }
                let j = d.ops();
                let distance = if s.third_person && !s.debug_camera {
                    camera::third_distance(&e, t, &mut w)
                } else {
                    4.0
                };
                let bed = if w.get_block_id(0, 65, -3) == 26 {
                    Some(w.get_block_metadata(0, 65, -3) & 3)
                } else {
                    None
                };
                let r = camera::transform(&e, t, s, distance, bed);
                d.compare_ops(&j, &r, "modelview ops");
                for v in camera::matrix(&r).to_cols_array() {
                    d.eq(v.to_bits(), "modelview matrix");
                }
                let projection = camera::matrix(&p.ops)
                    * camera::projection_gl(fov, 853.0 / 480.0, (256 >> s.distance) as f32 * 2.0);
                for v in projection.to_cols_array() {
                    d.eq(v.to_bits(), "combined projection matrix");
                }
            }
            1 | 10 => {
                let mut e = d.entity(&mut w);
                if kind==10 {e.player_state_mut().unwrap().fish_entity=Some(-1);}
                let t = d.f();
                let (mut s, hide) = d.settings();
                s.anaglyph = false;
                s.third_person = false;
                s.debug_camera = false;
                let id = d.i();
                let damage = d.i();
                let previous = d.f();
                let progress = d.f();
                let brightness = d.f();
                let block_brightness = d.f();
                d.label.push_str(&format!(" item {id}"));
                let equip = Equip {
                    item: if id == 0 {
                        None
                    } else {
                        Some(ItemStack::new(id, 1, damage))
                    },
                    previous,
                    progress,
                    slot: 0,
                };
                let c = match damage {
                    1 => 1usize,
                    3 => 2,
                    15 => 3,
                    _ => 0,
                };
                let map = super::map::Data {
                    colors: (0..16384).map(|i| ((i + c) % 56) as u8).collect(),
                    icons: (0..c)
                        .map(|i| {
                            [
                                (i % 16) as i8,
                                (i as i32 * 43 - 127) as i8,
                                (127 - i as i32 * 37) as i8,
                                (i * 5) as i8,
                            ]
                        })
                        .collect(),
                    label: format!("map_{damage}"),
                };
                let r = first_person::render_with_map(
                    &e,
                    t,
                    &equip,
                    brightness,
                    block_brightness,
                    s,
                    hide,
                    Some(&map),
                );
                d.draws(&r);
                if id == 358 {
                    for p in super::map::pixels(&map, false) {
                        d.eq(p, "map ARGB");
                    }
                    let image = super::map::image(&map, false);
                    for rgba in image.rgba.chunks_exact(4) {
                        let p = u32::from_le_bytes(rgba.try_into().unwrap());
                        d.eq(p, "map upload RGBA");
                    }
                }
            }
            2 => {
                let e = d.entity(&mut w);
                let _t = d.f();
                let (s, _) = d.settings();
                d.blocks(&mut w);
                let brightness = d.f();
                let water = d.i() != 0;
                let opaque = first_person::opaque_textures(&e, &mut w);
                let r = first_person::overlays(&e, water, &opaque, brightness, s);
                d.draws(&r);
            }
            3 | 11 => {
                let mut e = d.entity(&mut w);
                if kind==11 {e.player_state_mut().unwrap().fish_entity=Some(-1);}
                if d.i() != 0 {
                    e.riding = Some(-1);
                }
                let reuse = d.i() != 0;
                let t = d.f();
                let id = d.i();
                let damage = d.i();
                let p = e.player_state_mut().unwrap();
                p.inventory.main[0] = if id == 0 {
                    None
                } else {
                    Some(ItemStack::new(id, 1, damage))
                };
                for a in 0..4 {
                    let id = d.i();
                    p.inventory.armor[a] = if id == 0 {
                        None
                    } else {
                        Some(ItemStack::new(id, 1, 0))
                    };
                }
                let bed = d.i();
                let brightness = d.f();
                let camera = std::array::from_fn(|_| d.d());
                let r = if reuse {
                    persistent.render(&e, t, camera, brightness, bed)
                } else {
                    super::entity::player::Renderer::default()
                        .render(&e, t, camera, brightness, bed)
                };
                d.draws(&r);
            }
            4 => {
                let id = d.i();
                let meta = d.i();
                let (x, y, z) = (d.i(), d.i(), d.i());
                bounds[id as usize] =
                    worldgen::entity::AxisAlignedBB::new(d.d(), d.d(), d.d(), d.d(), d.d(), d.d());
                let camera = std::array::from_fn(|_| d.d());
                raw(&mut w, x, y, z, id as u8, meta);
                w.block_bounds = bounds;
                d.label.push_str(&format!(" block {id}:{meta}"));
                let r = targeting::selection_bounds(&mut w, x, y, z)
                    .map_or_else(|| Fixed::new(1.0), |b| targeting::selection(b, camera));
                bounds = w.block_bounds;
                d.draws(&r);
            }
            5 => {
                let id = d.i();
                let meta = d.i();
                let occlude = d.i() != 0;
                let progress = d.f();
                w.block_bounds[id as usize] =
                    worldgen::entity::AxisAlignedBB::new(d.d(), d.d(), d.d(), d.d(), d.d(), d.d());
                let camera = std::array::from_fn(|_| d.d());
                raw(&mut w, 0, 65, -3, id as u8, meta);
                if occlude {
                    raw(&mut w, 1, 65, -3, 1, 0);
                }
                d.label.push_str(&format!(" block {id}:{meta}"));
                let r = targeting::damage(&w, 0, 65, -3, progress, camera, &col, true);
                d.draws(&r);
            }
            6 => {
                let e = d.entity(&mut w);
                let t = d.f();
                let distance = d.i();
                let anaglyph = d.i() != 0;
                let nether = d.i() != 0;
                let mode = d.i();
                let far = d.f();
                let old = d.f();
                let new = d.f();
                let sky = std::array::from_fn(|_| d.f());
                let fog = std::array::from_fn(|_| d.f());
                let rain = d.f();
                let thunder = d.f();
                let medium = match d.i() {
                    1 => camera::Medium::Water,
                    2 => camera::Medium::Lava,
                    _ => camera::Medium::Air,
                };
                let angle = d.f();
                let light = d.i() as u32;
                let look = super::sky::look(e.yaw, e.pitch, e.prev_yaw, e.prev_pitch, t);
                let fog = super::sky::sunrise_fog(fog, angle, distance, look);
                let base = camera::fog_base(sky, fog, distance, rain, thunder);
                let eye_y = e.last_tick_pos.y + (e.pos.y - e.last_tick_pos.y) * t as f64;
                for v in camera::fog_color(base, medium, camera::lerp(old, new, t), anaglyph, eye_y) {
                    d.eq(v.to_bits(), "fog color");
                }
                let fog = camera::fog(medium, far, mode < 0, nether, camera::void_factor(light, eye_y));
                d.eq(if fog[2] == 2.0 { 2048 } else { 9729 }, "fog mode");
                d.eq(fog[0].to_bits(), "fog start/density");
                if fog[2] != 2.0 {
                    d.eq(fog[1].to_bits(), "fog end");
                }
            }
            7 => {
                let id = d.i();
                let damage = d.i();
                let slot = d.i();
                let identity = d.i() != 0;
                let mutated = d.i();
                if mutated >= 0 {
                    if let Some(i) = equip.item.as_mut() {
                        i.damage = mutated;
                    }
                }
                equip.update_identity(
                    if id == 0 {
                        None
                    } else {
                        Some(ItemStack::new(id, 1, damage))
                    },
                    slot,
                    identity,
                );
                d.eq(equip.previous.to_bits(), "equip previous");
                d.eq(equip.progress.to_bits(), "equip progress");
                d.eq(equip.slot as u32, "equip slot");
                d.eq(equip.item.map_or(0, |i| i.id) as u32, "equip item");
                d.eq(equip.item.map_or(0, |i| i.damage) as u32, "equip damage");
            }
            8 => {
                let id = d.i();
                let mut e = d.entity(&mut w);
                e.entity_id = id;
                let t = d.f();
                d.blocks(&mut w);
                let n = d.u();
                for _ in 0..n {
                    let id = d.i();
                    let mut pig = Entity::living(&mut w, "Pig").unwrap();
                    pig.entity_id = id;
                    pig.set_position(d.d(), d.d(), d.d());
                    w.entity_joined_world(pig);
                }
                let snapshot = super::player_view::Snapshot::capture(&w, &e);
                let mut mirror = world();
                snapshot.apply(&mut mirror);
                let hit = e.mouse_over(&mut mirror, t);
                let kind = d.i();
                let rk = match hit {
                    None => -1,
                    Some(worldgen::entity::MovingObjectPosition::Block { .. }) => 0,
                    Some(worldgen::entity::MovingObjectPosition::Entity { .. }) => 1,
                };
                d.compare(kind as u32, rk as u32, "pick type");
                if kind == 1 {
                    let expected = d.i();
                    if let Some(worldgen::entity::MovingObjectPosition::Entity { id, .. }) = hit {
                        d.compare(expected as u32, id as u32, "picked entity");
                    }
                } else if kind == 0 {
                    let j = [d.i(), d.i(), d.i(), d.i()];
                    let v = [d.d(), d.d(), d.d()];
                    if let Some(worldgen::entity::MovingObjectPosition::Block {
                        x,
                        y,
                        z,
                        side,
                        hit,
                    }) = hit
                    {
                        for (j, r) in j.into_iter().zip([x, y, z, side]) {
                            d.compare(j as u32, r as u32, "picked block");
                        }
                        for (j, r) in v.into_iter().zip([hit.x, hit.y, hit.z]) {
                            d.checks += 1;
                            if j.to_bits() != r.to_bits() {
                                d.bad += 1;
                                eprintln!(
                                    "{} hit coordinate: {:016x} != {:016x}",
                                    d.label,
                                    j.to_bits(),
                                    r.to_bits()
                                );
                            }
                        }
                    }
                }
            }
            9 => {
                let c = d.i();
                let anaglyph = d.i() != 0;
                let n = d.i();
                let map = super::map::Data {
                    colors: (0..16384).map(|i| ((i + n) % 56) as u8).collect(),
                    icons: (0..n)
                        .map(|i| {
                            [
                                (i % 16) as i8,
                                (i * 43 - 127) as i8,
                                (127 - i * 37) as i8,
                                (i * 5) as i8,
                            ]
                        })
                        .collect(),
                    label: format!("map_{c}"),
                };
                let mut g = Fixed::new(1.0);
                super::map::render(&mut g, &map);
                d.draws(&g);
                for p in super::map::pixels(&map, anaglyph) {
                    d.eq(p, "map ARGB");
                }
                for rgba in super::map::image(&map, anaglyph).rgba.chunks_exact(4) {
                    d.eq(
                        u32::from_le_bytes(rgba.try_into().unwrap()),
                        "map upload RGBA",
                    );
                }
            }
            _ => panic!("case kind {kind}"),
        }
        if d.bad > before {
            eprintln!("{}: {} differences", d.label, d.bad - before);
        }
    }
    assert_eq!(d.at, d.data.len(), "unread oracle bytes");
    println!(
        "first person: {n} cases, {} raw-bit comparisons, {} differences",
        d.checks, d.bad
    );
    d.bad == 0
}
