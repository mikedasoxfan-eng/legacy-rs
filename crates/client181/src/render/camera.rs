//! EntityRenderer camera operations, in the original Java evaluation order.
//! GL input bits are the portable oracle boundary; driver matrix multiplication
//! and wgpu's clip-depth conversion happen only after that boundary.
use super::entity::fixed::{Fixed, Op};
use glam::{Mat4, Vec3};
use worldgen::{
    entity::{Entity, MovingObjectPosition, Vec3D},
    math::{self, PI_F32},
    World,
};

#[derive(Clone, Copy, Debug, Default)]
pub struct Settings {
    pub bobbing: bool,
    pub third_person: bool,
    pub anaglyph: bool,
    pub eye: i32,
    pub ticks: i32,
    pub distance: i32,
    pub debug_camera: bool,
    pub debug_yaw: [f32; 2],
    pub debug_pitch: [f32; 2],
    pub roll: [f32; 2],
    pub fov: [f32; 2],
    /// b1.8.1 FOV option (`field_35379_L`, 0..1 adds up to 40 degrees).
    pub fov_option: f32,
    /// b1.8.1 sprint/bow zoom `[field_35813_N, field_35812_M]`; `None` is 1.0 (no zoom).
    pub fov_zoom: Option<[f32; 2]>,
    pub zoom: f64,
    pub zoom_yaw: f64,
    pub zoom_pitch: f64,
}
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}
/// `getFOVModifier(t, projection)` (b1.8.1): the option and zoom only apply to the world
/// projection, not to the hand.
pub fn fov(e: &Entity, water: bool, t: f32, s: &Settings, projection: bool) -> f32 {
    let mut f = 70.0f32;
    if projection {
        f += s.fov_option * 40.0f32;
        let z = s.fov_zoom.unwrap_or([1.0, 1.0]);
        f *= z[0] + (z[1] - z[0]) * t;
    }
    if e.living.health <= 0 {
        let death = e.living.death_time as f32 + t;
        f /= (1.0 - 500.0 / (death + 500.0)) * 2.0 + 1.0;
    }
    if water {
        f = f * 60.0f32 / 70.0f32;
    }
    f + s.fov[0] + (s.fov[1] - s.fov[0]) * t
}
pub fn hurt(g: &mut Fixed, e: &Entity, t: f32) {
    let l = &e.living;
    let mut h = l.hurt_time as f32 - t;
    if l.health <= 0 {
        let d = l.death_time as f32 + t;
        g.rotate(40.0 - 8000.0 / (d + 200.0), 0.0, 0.0, 1.0);
    }
    if h >= 0.0 {
        h /= l.max_hurt_time as f32;
        h = math::sin(h * h * h * h * PI_F32);
        g.rotate(-l.attacked_at_yaw, 0.0, 1.0, 0.0);
        g.rotate(-h * 14.0, 0.0, 0.0, 1.0);
        g.rotate(l.attacked_at_yaw, 0.0, 1.0, 0.0);
    }
}
pub fn bob(g: &mut Fixed, e: &Entity, t: f32) {
    if let Some(p) = e.player_state() {
        let delta = e.distance_walked - e.prev_distance_walked;
        let walk = -(e.distance_walked + delta * t);
        let yaw = lerp(p.prev_camera_yaw, p.camera_yaw, t);
        let pitch = lerp(p.prev_camera_pitch, p.camera_pitch, t);
        g.translate(
            math::sin(walk * PI_F32) * yaw * 0.5,
            -(math::cos(walk * PI_F32) * yaw).abs(),
            0.0,
        );
        g.rotate(math::sin(walk * PI_F32) * yaw * 3.0, 0.0, 0.0, 1.0);
        g.rotate(
            (math::cos(walk * PI_F32 - 0.2) * yaw).abs() * 5.0,
            1.0,
            0.0,
            0.0,
        );
        g.rotate(pitch, 1.0, 0.0, 0.0);
    }
}
/// The eight probes use the current rotation, not the interpolated rotation.
/// Preserve the extra Z offset in the endpoint's X coordinate.
pub fn third_distance(e: &Entity, t: f32, w: &mut World) -> f64 {
    let off = e.y_offset - 1.62;
    let p = e.view_position(t).add(0.0, -(off as f64), 0.0);
    let mut distance = 4.0;
    let dx = (-math::sin(e.yaw / 180.0 * PI_F32) * math::cos(e.pitch / 180.0 * PI_F32)) as f64
        * distance;
    let dz =
        (math::cos(e.yaw / 180.0 * PI_F32) * math::cos(e.pitch / 180.0 * PI_F32)) as f64 * distance;
    let dy = (-math::sin(e.pitch / 180.0 * PI_F32)) as f64 * distance;
    for i in 0..8 {
        let x = ((i & 1) * 2 - 1) as f32 * 0.1;
        let y = ((i >> 1 & 1) * 2 - 1) as f32 * 0.1;
        let z = ((i >> 2 & 1) * 2 - 1) as f32 * 0.1;
        let start = p.add(x as f64, y as f64, z as f64);
        let end = Vec3D::new(
            p.x - dx + x as f64 + z as f64,
            p.y - dy + y as f64,
            p.z - dz + z as f64,
        );
        if let Some(MovingObjectPosition::Block { hit, .. }) = w.ray_trace_blocks(start, end) {
            let d = hit.distance(p);
            if d < distance {
                distance = d;
            }
        }
    }
    distance
}
pub fn orient(g: &mut Fixed, e: &Entity, t: f32, s: Settings, distance: f64, bed: Option<i32>) {
    let mut offset = e.y_offset - 1.62;
    g.rotate(lerp(s.roll[0], s.roll[1], t), 0.0, 0.0, 1.0);
    let pitch = lerp(e.prev_pitch, e.pitch, t);
    let yaw = lerp(e.prev_yaw, e.yaw, t);
    if e.player_state().is_some_and(|p| p.sleeping) {
        offset = (offset as f64 + 1.0) as f32;
        g.translate(0.0, 0.3, 0.0);
        if !s.debug_camera {
            if let Some(b) = bed {
                g.rotate((b * 90) as f32, 0.0, 1.0, 0.0);
            }
            g.rotate(yaw + 180.0, 0.0, -1.0, 0.0);
            g.rotate(pitch, -1.0, 0.0, 0.0);
        }
    } else if s.third_person {
        if s.debug_camera {
            g.translate(0.0, 0.0, -distance as f32);
            g.rotate(lerp(s.debug_pitch[0], s.debug_pitch[1], t), 1.0, 0.0, 0.0);
            g.rotate(lerp(s.debug_yaw[0], s.debug_yaw[1], t), 0.0, 1.0, 0.0);
        } else {
            g.rotate(e.pitch - e.pitch, 1.0, 0.0, 0.0);
            g.rotate(e.yaw - e.yaw, 0.0, 1.0, 0.0);
            g.translate(0.0, 0.0, -distance as f32);
            g.rotate(e.yaw - e.yaw, 0.0, 1.0, 0.0);
            g.rotate(e.pitch - e.pitch, 1.0, 0.0, 0.0);
        }
    } else {
        g.translate(0.0, 0.0, -0.1);
    }
    if !s.debug_camera {
        g.rotate(pitch, 1.0, 0.0, 0.0);
        g.rotate(yaw + 180.0, 0.0, 1.0, 0.0);
    }
    g.translate(0.0, offset, 0.0);
}
pub fn transform(e: &Entity, t: f32, s: Settings, distance: f64, bed: Option<i32>) -> Vec<Op> {
    let mut g = Fixed::new(1.0);
    if s.anaglyph {
        g.translate((s.eye * 2 - 1) as f32 * 0.1, 0.0, 0.0);
    }
    hurt(&mut g, e, t);
    if s.bobbing {
        bob(&mut g, e, t);
    }
    if let Some(p) = e.player_state() {
        let portal = lerp(p.prev_time_in_portal, p.time_in_portal, t);
        if portal > 0.0 {
            let mut scale = 5.0 / (portal * portal + 5.0) - portal * 0.04;
            scale *= scale;
            g.rotate((s.ticks as f32 + t) * 20.0, 0.0, 1.0, 1.0);
            g.scale(1.0 / scale, 1.0, 1.0);
            g.rotate(-(s.ticks as f32 + t) * 20.0, 0.0, 1.0, 1.0);
        }
    }
    orient(&mut g, e, t, s, distance, bed);
    g.ops
}
/// Matrix in camera-relative world coordinates. GL translations in orientCamera
/// must remain in modelview: the world subtracts the player's position separately.
pub fn matrix(ops: &[Op]) -> Mat4 {
    super::entity::fixed::matrix(ops)
}
/// LWJGL Project.gluPerspective's float arithmetic, followed by GL-to-wgpu Z.
pub fn projection_gl(fov: f32, aspect: f32, far: f32) -> Mat4 {
    let radians = fov / 2.0 * PI_F32 / 180.0;
    let sine = (radians as f64).sin() as f32;
    let cot = (radians as f64).cos() as f32 / sine;
    let delta = far - 0.05;
    Mat4::from_cols_array(&[
        cot / aspect,
        0.0,
        0.0,
        0.0,
        0.0,
        cot,
        0.0,
        0.0,
        0.0,
        0.0,
        -(far + 0.05) / delta,
        -1.0,
        0.0,
        0.0,
        -2.0 * 0.05 * far / delta,
        0.0,
    ])
}
pub fn projection(fov: f32, aspect: f32, far: f32, s: Settings) -> Mat4 {
    let gl = projection_gl(fov, aspect, far);
    let mut p = Mat4::IDENTITY;
    if s.anaglyph {
        p *= Mat4::from_translation(Vec3::new(-(s.eye * 2 - 1) as f32 * 0.07, 0.0, 0.0));
    }
    if s.zoom != 1.0 && s.zoom != 0.0 {
        p *= Mat4::from_translation(Vec3::new(s.zoom_yaw as f32, -s.zoom_pitch as f32, 0.0));
        p *= Mat4::from_scale(Vec3::new(s.zoom as f32, s.zoom as f32, 1.0));
    }
    let convert = Mat4::from_cols_array(&[
        1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
    ]);
    convert * p * gl
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Medium {
    #[default]
    Air,
    Water,
    Lava,
}
pub fn medium(e: &Entity, w: &mut World) -> Medium {
    if e.is_inside_material(w, worldgen::block::MAT_WATER) {
        Medium::Water
    } else if e.is_inside_material(w, worldgen::block::MAT_LAVA) {
        Medium::Lava
    } else {
        Medium::Air
    }
}
/// `eye_y`: the view entity's interpolated posY; b1.8.1 darkens the fog below y 32.
pub fn fog_color(base: [f32; 3], medium: Medium, fade: f32, anaglyph: bool, eye_y: f64) -> [f32; 3] {
    let mut c = match medium {
        Medium::Air => base,
        Medium::Water => [0.02, 0.02, 0.2],
        Medium::Lava => [0.6, 0.1, 0.0],
    }
    .map(|c| c * fade);
    let mut depth = eye_y / 32.0;
    if depth < 1.0 {
        if depth < 0.0 {
            depth = 0.0;
        }
        depth *= depth;
        c = c.map(|v| (v as f64 * depth) as f32);
    }
    if anaglyph {
        [
            (c[0] * 30.0 + c[1] * 59.0 + c[2] * 11.0) / 100.0,
            (c[0] * 30.0 + c[1] * 70.0) / 100.0,
            (c[0] * 30.0 + c[2] * 70.0) / 100.0,
        ]
    } else {
        c
    }
}
pub fn fog_base(sky: [f32; 3], fog: [f32; 3], distance: i32, rain: f32, thunder: f32) -> [f32; 3] {
    let k = 1.0 - ((1.0f32 / (4 - distance) as f32) as f64).powf(0.25) as f32;
    let mut c = std::array::from_fn(|i| fog[i] + (sky[i] - fog[i]) * k);
    if rain > 0.0 {
        c[0] *= 1.0 - rain * 0.5;
        c[1] *= 1.0 - rain * 0.5;
        c[2] *= 1.0 - rain * 0.4;
    }
    if thunder > 0.0 {
        for v in &mut c {
            *v *= 1.0 - thunder * 0.5;
        }
    }
    c
}
/// Uniform convention: linear=1, exponential=2 (density in x).
/// RenderGlobal.func_27307_a always returns false in b1.7.3: no cloud-fog branch.
pub fn fog(m: Medium, far: f32, sky: bool, nether: bool) -> [f32; 3] {
    match m {
        Medium::Water => [0.1, 0.0, 2.0],
        Medium::Lava => [2.0, 0.0, 2.0],
        Medium::Air => [
            if sky || nether { 0.0 } else { far * 0.25 },
            if sky { far * 0.8 } else { far },
            1.0,
        ],
    }
}
