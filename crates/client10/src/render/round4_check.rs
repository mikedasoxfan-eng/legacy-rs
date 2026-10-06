//! Release 1.0 rendering round 4 against `Mode_render_round4_10` (original bytecode): RenderItem's
//! GUI passes, ActiveRenderInfo, EntityRenderer fog and End lightmap, OpenGlHelper and
//! OpenGlCapsChecker, RenderHelper, Render shadows and the Tessellator.
use super::active_info::Info;
use super::camera::{self, FogState};
use super::entity::fixed::Fixed;
use super::opengl_helper::{self as gl, Call};
use worldgen::entity::Entity;

struct Reader {
    data: Vec<u8>,
    at: usize,
    checks: usize,
    bad: usize,
    label: String,
}
impl Reader {
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
    fn same(&mut self, j: u64, r: u64, what: &str) {
        self.checks += 1;
        if j != r {
            if self.bad < 30 {
                eprintln!("{} {what}: java {j:x}, rust {r:x}", self.label);
            }
            self.bad += 1;
        }
    }
    fn eq(&mut self, r: u32, what: &str) {
        let j = self.u();
        self.same(j as u64, r as u64, what);
    }
    fn ef(&mut self, r: f32, what: &str) {
        self.eq(r.to_bits(), what);
    }
    fn ops(&mut self, r: &[super::entity::fixed::Op], what: &str) {
        let n = self.u() as usize;
        self.same(n as u64, r.len() as u64, &format!("{what} count"));
        if n != r.len() {
            self.at += n * 20;
            return;
        }
        for op in r {
            self.eq(op.code, what);
            for a in op.args {
                self.ef(a, what);
            }
        }
    }
    /// Mode_render_effects10.draws: every draw's state and vertices, then the final GL state.
    fn draws(&mut self, g: &Fixed) {
        let n = self.u() as usize;
        self.same(n as u64, g.draws.len() as u64, "draw count");
        assert_eq!(n, g.draws.len(), "{}: draw count", self.label);
        for (i, v) in g.draws.iter().enumerate() {
            let what = format!("draw {i}");
            self.ops(&v.ops, &what);
            for actual in [v.cull as u32, v.flags, v.texture, v.src, v.dst, v.depth, v.mode] {
                self.eq(actual, &what);
            }
            self.ef(v.alpha_cutoff, &what);
            for value in v.material {
                self.ef(value, &format!("{what} color"));
            }
            let nv = self.u() as usize;
            self.same(nv as u64, v.vertices.len() as u64, &format!("{what} vertices"));
            assert_eq!(nv, v.vertices.len());
            for vertex in &v.vertices {
                self.eq(vertex.normal, &format!("{what} normal"));
                self.eq(vertex.color, &format!("{what} vertex color"));
                for f in vertex.pos.into_iter().chain(vertex.uv) {
                    self.ef(f, &format!("{what} vertex"));
                }
            }
        }
        self.ops(&g.ops, "final ops");
        for actual in [g.flags, g.src, g.dst, g.depth] {
            self.eq(actual, "final state");
        }
        self.ef(g.alpha_cutoff, "final alpha");
        for value in g.material {
            self.ef(value, "final color");
        }
    }
    /// GlLog10 commands.
    fn commands(&mut self, r: &[Call]) {
        let n = self.u() as usize;
        self.same(n as u64, r.len() as u64, "GL call count");
        for k in 0..n {
            let code = self.i();
            let args: [f32; 4] = std::array::from_fn(|_| self.f());
            if let Some((rc, ra)) = r.get(k) {
                self.same(code as u64, *rc as u64, &format!("GL call {k} code"));
                for (a, b) in args.iter().zip(ra) {
                    self.same(a.to_bits() as u64, b.to_bits() as u64, &format!("GL call {k} ({code}) argument"));
                }
            }
        }
    }
}

fn stack(id: i32, damage: i32) -> worldgen::world::tile::ItemStack {
    let s = worldgen::entity::player::ItemStack::new(id, 1, damage.max(0));
    if damage < 0 {
        s.add_enchantment(16, 1);
    }
    s.into()
}

fn gui(d: &mut Reader) {
    for case in 0..d.u() {
        let (id, damage, x, y) = (d.i(), d.i(), d.i(), d.i());
        let z = d.f();
        let tint = d.i() != 0;
        let base: [f32; 4] = std::array::from_fn(|_| d.f());
        let offsets: [f32; 2] = std::array::from_fn(|_| d.f());
        d.label = format!("gui {case} item {id}:{damage} z {z}");
        let g = super::entity::gui_item::render_item_geometry(&stack(id, damage), x, y, tint, Some(base), z, offsets);
        d.draws(&g);
    }
}

fn active_info(d: &mut Reader) {
    let mut info = Info::default();
    for case in 0..d.u() as i32 {
        d.label = format!("active info {case}");
        let mv: [f32; 16] = std::array::from_fn(|_| d.f());
        let pj: [f32; 16] = std::array::from_fn(|_| d.f());
        let vp: [i32; 4] = std::array::from_fn(|_| d.i());
        let (yaw, pitch, front) = (d.f(), d.f(), d.i() != 0);
        info.update(mv, pj, vp, yaw, pitch, front);
        for v in info.offset {
            d.ef(v, "near-plane offset");
        }
        for v in info.rotation {
            d.ef(v, "view vector");
        }
        let mut w = super::first_person_check::world();
        let mut e = Entity::player(&mut w, "Player");
        let pos = [d.d(), d.d(), d.d()];
        let prev = [d.d(), d.d(), d.d()];
        let t = d.f();
        let (id, meta, above) = (d.i(), d.i(), d.i());
        for y in 64..=67 {
            let block = if y == 66 { above } else { id };
            super::first_person_check::raw(&mut w, 0, y, -3, block as u8, meta);
        }
        e.set_position(pos[0], pos[1], pos[2]);
        e.prev_pos = worldgen::entity::Vec3D::new(prev[0], prev[1], prev[2]);
        let at = info.position(&e, t as f64);
        for (j, r) in [d.d(), d.d(), d.d()].into_iter().zip([at.x, at.y, at.z]) {
            d.same(j.to_bits(), r.to_bits(), "eye point");
        }
        let block = info.block(&e, t as f64, &mut w);
        d.eq(block as u32, "eye block");
    }
}

fn fog(d: &mut Reader) {
    for case in 0..d.u() {
        d.label = format!("fog {case}");
        let mut w = super::first_person_check::world();
        let mut e = Entity::player(&mut w, "Player");
        let pos = [d.d(), d.d(), d.d()];
        let prev = [d.d(), d.d(), d.d()];
        let last_y = d.d();
        let (yaw, prev_yaw, pitch, prev_pitch) = (d.f(), d.f(), d.f(), d.f());
        let t = d.f();
        let distance = d.i();
        let anaglyph = d.i() != 0;
        let nether = d.i() != 0;
        let has_no_sky = d.i() != 0;
        let mode = d.i();
        let far = d.f();
        let (old, new) = (d.f(), d.f());
        let blind = d.i();
        let water_breathing = d.i() != 0;
        let cloud = d.i() != 0;
        let offset: [f32; 3] = std::array::from_fn(|_| d.f());
        let (id, meta, above) = (d.i(), d.i(), d.i());
        let sky: [f32; 3] = std::array::from_fn(|_| d.f());
        let fog: [f32; 3] = std::array::from_fn(|_| d.f());
        let clouds: [f32; 3] = std::array::from_fn(|_| d.f());
        let (rain, thunder, angle) = (d.f(), d.f(), d.f());
        let light = d.i() as u32;
        super::first_person_check::raw(&mut w, 0, 65, -3, id as u8, meta);
        super::first_person_check::raw(&mut w, 0, 66, -3, above as u8, meta);
        e.set_position(pos[0], pos[1], pos[2]);
        e.prev_pos = worldgen::entity::Vec3D::new(prev[0], prev[1], prev[2]);
        e.last_tick_pos.y = last_y;
        let info = Info { offset, ..Default::default() };
        let state = FogState {
            medium: info.medium(&e, t as f64, &mut w),
            cloud: cloud.then_some(clouds),
            blindness: (blind >= 0).then_some(blind),
            water_breathing,
            has_no_sky,
            nether,
        };
        let look = super::sky::look(yaw, pitch, prev_yaw, prev_pitch, t);
        let base = camera::fog_base(sky, super::sky::sunrise_fog(fog, angle, distance, look), distance, rain, thunder);
        let eye_y = last_y + (pos[1] - last_y) * t as f64;
        let color = camera::fog_color_release(base, &state, camera::lerp(old, new, t), anaglyph, eye_y);
        let f = camera::fog_release(&state, far, mode < 0, camera::void_factor(light, eye_y));
        let mut calls: Vec<Call> = vec![(28, [color[0], color[1], color[2], 0.0])];
        calls.push((26, [2918.0, 0.0, 0.0, 0.0]));
        calls.push((22, [color[0], color[1], color[2], 1.0]));
        calls.push((27, [0.0, -1.0, 0.0, 0.0]));
        calls.push((18, [1.0, 1.0, 1.0, 1.0]));
        if f[2] == 2.0 {
            calls.push((24, [2917.0, 2048.0, 0.0, 0.0]));
            calls.push((25, [2914.0, f[0], 0.0, 0.0]));
        } else {
            calls.push((24, [2917.0, 9729.0, 0.0, 0.0]));
            // The Nether's later FOG_START 0 is a second call after the distance-based one.
            calls.push((25, [2915.0, if mode < 0 { 0.0 } else { f[1] * 0.25 }, 0.0, 0.0]));
            calls.push((25, [2916.0, f[1], 0.0, 0.0]));
            if state.blindness.is_none() && nether {
                calls.push((25, [2915.0, 0.0, 0.0, 0.0]));
            }
        }
        calls.push((6, [2903.0, 0.0, 0.0, 0.0]));
        calls.push((30, [1028.0, 4608.0, 0.0, 0.0]));
        d.commands(&calls);
    }
}

fn lightmap(d: &mut Reader) {
    for case in 0..d.u() {
        d.label = format!("lightmap {case}");
        let table: [f32; 16] = std::array::from_fn(|_| d.f());
        let (sky, flicker, flash, gamma, kind) = (d.f(), d.f(), d.i() != 0, d.f(), d.i());
        for v in super::lightmap::compute_dimension(&table, sky, flicker, flash, gamma, kind == 1) {
            d.eq(v, "texel");
        }
    }
}

fn gl_helper(d: &mut Reader) {
    for _ in 0..d.u() {
        let c = d.i();
        d.label = format!("OpenGlHelper {c}");
        let (multitexture, gl13, occlusion) = (c & 1 != 0, c & 2 != 0, c & 4 != 0);
        d.eq(gl::LIGHTMAP_DISABLED as u32, "lightmapDisabled");
        d.eq(gl::LIGHTMAP_ENABLED as u32, "lightmapEnabled");
        let arb = gl::use_multitexture_arb(multitexture, gl13);
        d.eq(arb as u32, "useMultitextureARB");
        d.eq(gl::check_arb_occlusion(occlusion) as u32, "checkARBOcclusion");
        let (s, t) = (d.f(), d.f());
        let calls = if arb {
            Vec::new()
        } else {
            vec![
                gl::set_active_texture(gl::LIGHTMAP_ENABLED),
                gl::set_lightmap_texture_coords(gl::LIGHTMAP_ENABLED, s, t),
                gl::set_client_active_texture(gl::LIGHTMAP_ENABLED),
                gl::set_client_active_texture(gl::LIGHTMAP_DISABLED),
                gl::set_active_texture(gl::LIGHTMAP_DISABLED),
            ]
        };
        d.commands(&calls);
    }
}

fn render_helper(d: &mut Reader) {
    for _ in 0..d.u() {
        let c = d.i();
        d.label = format!("RenderHelper {c}");
        let mut calls = Vec::new();
        match c {
            0 => super::render_helper::enable_standard_item_lighting(&mut calls),
            1 => super::render_helper::gui_item_lighting(&mut calls),
            _ => super::render_helper::disable_standard_item_lighting(&mut calls),
        }
        d.commands(&calls);
    }
}

fn shadow(d: &mut Reader) {
    let names = ["Pig", "Cow", "Sheep", "Chicken", "Wolf", "MushroomCow", "Zombie", "Villager"];
    let mut world = worldgen::World::new(0, 1);
    for case in 0..d.u() {
        let (k, child, partial) = (d.i() as usize, d.i() != 0, d.f());
        d.label = format!("shadow {case} {} child {child}", names[k]);
        let pos = [d.d(), d.d(), d.d()];
        let last = [d.d(), d.d(), d.d()];
        let camera = [d.d(), d.d(), d.d()];
        let surfaces = (0..d.u())
            .map(|_| super::entity::effects::Surface { xyz: [d.i(), d.i(), d.i()], block: d.i() as u8, brightness: d.f() })
            .collect::<Vec<_>>();
        let mut e = Entity::living(&mut world, names[k]).unwrap();
        e.set_position(pos[0], pos[1], pos[2]);
        e.last_tick_pos = worldgen::entity::Vec3D::new(last[0], last[1], last[2]);
        if child && e.kind.is_animal() {
            e.living.growing_age = -24000;
        }
        let g = super::entity::effects::shadow(&e, &surfaces, partial, camera);
        d.draws(&g);
    }
}

fn tessellator(d: &mut Reader) {
    for case in 0..d.u() {
        d.label = format!("tessellator {case}");
        let n = d.u() as usize;
        let bright = d.i() != 0;
        let translate = [d.d(), d.d(), d.d()];
        let mut t = super::Tessellator::new(translate);
        for _ in 0..n {
            let (color, light) = (d.u(), d.i());
            let (x, y, z, u, v) = (d.d(), d.d(), d.d(), d.d(), d.d());
            t.set_color_opaque_i(color);
            if bright {
                t.set_brightness(light);
            }
            t.vertex_uv(x, y, z, u, v);
        }
        d.eq(t.bytes_drawn() as u32, "draw() bytes");
        for v in t.verts.clone() {
            for f in v.pos.into_iter().chain(v.uv) {
                d.ef(f, "vertex word");
            }
            d.eq(u32::from_le_bytes(v.color), "vertex color");
            d.eq(if bright { v.light } else { 0 }, "vertex brightness");
        }
        let calls = t.draw_calls();
        d.commands(&calls);
    }
}

pub fn check(kind: &str, path: &str) -> bool {
    let mut d = Reader { data: std::fs::read(path).expect("round 4 dump"), at: 0, checks: 0, bad: 0, label: String::new() };
    assert_eq!(d.u(), 0x34524452);
    let _ = d.u();
    match kind {
        "gui" => gui(&mut d),
        "activeinfo" => active_info(&mut d),
        "fog" => fog(&mut d),
        "lightmap" => lightmap(&mut d),
        "glhelper" => gl_helper(&mut d),
        "renderhelper" => render_helper(&mut d),
        "shadow" => shadow(&mut d),
        "tessellator" => tessellator(&mut d),
        _ => panic!("round 4 kind {kind}"),
    }
    assert_eq!(d.at, d.data.len(), "unread round 4 bytes");
    println!("R10 RENDER4 {kind} {}: {} raw-bit comparisons, {} mismatches", if d.bad == 0 { "EXACT" } else { "DIFFER" }, d.checks, d.bad);
    d.bad == 0
}
