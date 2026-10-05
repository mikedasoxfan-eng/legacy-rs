//! Literal RenderMinecart / RenderBoat and their ModelRenderer geometry.
use super::{fixed::Fixed, geometry::ModelRenderer};
use worldgen::{
    entity::{Entity, EntityKind, Vec3D},
    math::{self, PI_F32},
};

/// Only the rail cells needed by the three partial-tick projection queries cross threads.
#[derive(Clone, Debug, Default)]
pub struct RailSnapshot(pub Vec<((i32, i32, i32), u8, i32)>);
impl RailSnapshot {
    pub fn capture(e: &Entity, mut block: impl FnMut(i32, i32, i32) -> (u8, i32)) -> Self {
        let f = worldgen::math::floor_double;
        let mut cells = Vec::new();
        for x in f(e.last_tick_pos.x.min(e.pos.x)) - 1..=f(e.last_tick_pos.x.max(e.pos.x)) + 1 {
            for y in f(e.last_tick_pos.y.min(e.pos.y)) - 2..=f(e.last_tick_pos.y.max(e.pos.y)) + 1 {
                for z in
                    f(e.last_tick_pos.z.min(e.pos.z)) - 1..=f(e.last_tick_pos.z.max(e.pos.z)) + 1
                {
                    let (id, m) = block(x, y, z);
                    if worldgen::entity::vehicles::is_rail(id) {
                        cells.push(((x, y, z), id, m));
                    }
                }
            }
        }
        Self(cells)
    }
    pub fn sample(&self, pos: [f64; 3]) -> [Option<Vec3D>; 3] {
        let [x, y, z] = pos;
        let mut block = |x, y, z| {
            self.0
                .iter()
                .find(|c| c.0 == (x, y, z))
                .map_or((0, 0), |c| (c.1, c.2))
        };
        [
            Entity::rail_position_with(&mut block, x, y, z),
            Entity::rail_offset_with(&mut block, x, y, z, 0.3f32 as f64),
            Entity::rail_offset_with(&mut block, x, y, z, -0.3f32 as f64),
        ]
    }
}
#[derive(Clone, Copy, Debug)]
pub struct VehicleState {
    pub cart_type: Option<i32>,
    pub time_since_hit: i32,
    pub damage: i32,
    pub rock_direction: i32,
    pub yaw: f32,
    pub pitch: f32,
    pub position: [f64; 3],
    pub track: [Option<Vec3D>; 3],
}
pub fn state(e: &Entity, partial: f32) -> Option<VehicleState> {
    let (cart_type, time_since_hit, damage, rock_direction) = match &e.kind {
        EntityKind::Minecart(s) => (
            Some(s.cart_type),
            s.time_since_hit,
            s.damage,
            s.rock_direction,
        ),
        EntityKind::Boat(s) => (None, s.time_since_hit, s.damage, s.rock_direction),
        _ => return None,
    };
    Some(VehicleState {
        cart_type,
        time_since_hit,
        damage,
        rock_direction,
        yaw: e.prev_yaw + (e.yaw - e.prev_yaw) * partial,
        pitch: e.prev_pitch + (e.pitch - e.prev_pitch) * partial,
        position: super::bridge::position(e, partial),
        track: [None; 3],
    })
}
pub fn model(cart: bool) -> Vec<ModelRenderer> {
    let (width, height, depth) = if cart { (20, 8, 16) } else { (24, 6, 20) };
    let mut parts: Vec<_> = (0..if cart { 6 } else { 5 })
        .map(|i| {
            if i == 0 {
                ModelRenderer::new(0, if cart { 10 } else { 8 })
            } else if i == 5 {
                ModelRenderer::new(44, 10)
            } else {
                ModelRenderer::new(0, 0)
            }
        })
        .collect();
    if cart {
        parts[0].add_box(
            (-width / 2) as f32,
            (-depth / 2) as f32,
            -1.0,
            width,
            depth,
            2,
            0.0,
        );
    } else {
        parts[0].add_box(
            (-width / 2) as f32,
            (-depth / 2 + 2) as f32,
            -3.0,
            width,
            depth - 4,
            4,
            0.0,
        );
    }
    parts[0].set_rotation_point(0.0, 4.0, 0.0);
    parts[0].rotate_angle_x = PI_F32 * 0.5;
    for p in &mut parts[1..5] {
        p.add_box(
            (-width / 2 + 2) as f32,
            (-height - 1) as f32,
            -1.0,
            width - 4,
            height,
            2,
            0.0,
        );
    }
    parts[1].set_rotation_point((-width / 2 + 1) as f32, 4.0, 0.0);
    parts[1].rotate_angle_y = PI_F32 * 3.0 / 2.0;
    parts[2].set_rotation_point((width / 2 - 1) as f32, 4.0, 0.0);
    parts[2].rotate_angle_y = PI_F32 * 0.5;
    parts[3].set_rotation_point(0.0, 4.0, (-depth / 2 + 1) as f32);
    parts[3].rotate_angle_y = PI_F32;
    parts[4].set_rotation_point(0.0, 4.0, (depth / 2 - 1) as f32);
    if cart {
        parts[5].add_box(
            (-width / 2 + 1) as f32,
            (-depth / 2 + 1) as f32,
            -1.0,
            width - 2,
            depth - 2,
            1,
            0.0,
        );
        parts[5].set_rotation_point(0.0, 4.0 - -0.1, 0.0);
        parts[5].rotate_angle_x = PI_F32 * -0.5;
    }
    parts
}
pub fn render(
    g: &mut Fixed,
    s: VehicleState,
    mut relative: [f64; 3],
    partial: f32,
    brightness: f32,
) {
    g.push();
    let mut yaw = s.yaw;
    let mut pitch = s.pitch;
    if s.cart_type.is_some() {
        if let Some(center) = s.track[0] {
            let forward = s.track[1].unwrap_or(center);
            let back = s.track[2].unwrap_or(center);
            relative[0] += center.x - s.position[0];
            relative[1] += (forward.y + back.y) / 2.0 - s.position[1];
            relative[2] += center.z - s.position[2];
            let direction = back.add(-forward.x, -forward.y, -forward.z);
            if direction.length() != 0.0 {
                let n = direction.normalize();
                yaw = (worldgen::jmath::java_atan2(n.z, n.x) * 180.0 / std::f64::consts::PI) as f32;
                pitch = (worldgen::jmath::java_atan(n.y) * 73.0) as f32;
            }
        }
    }
    g.translate(relative[0] as f32, relative[1] as f32, relative[2] as f32);
    g.rotate(180.0 - yaw, 0.0, 1.0, 0.0);
    if s.cart_type.is_some() {
        g.rotate(-pitch, 0.0, 0.0, 1.0);
    }
    let hit = s.time_since_hit as f32 - partial;
    let damage = (s.damage as f32 - partial).max(0.0);
    if hit > 0.0 {
        g.rotate(
            math::sin(hit) * hit * damage / 10.0 * s.rock_direction as f32,
            1.0,
            0.0,
            0.0,
        );
    }
    if let Some(t) = s.cart_type.filter(|t| *t != 0) {
        g.texture = 1;
        g.scale(0.75, 0.75, 0.75);
        g.translate(0.0, 0.3125, 0.0);
        g.rotate(90.0, 0.0, 1.0, 0.0);
        if t == 1 || t == 2 {
            super::inventory::inventory(g, if t == 1 { 54 } else { 61 }, 0, brightness, true);
        }
        g.rotate(-90.0, 0.0, 1.0, 0.0);
        g.translate(0.0, -0.3125, 0.0);
        g.scale(1.0 / 0.75, 1.0 / 0.75, 1.0 / 0.75);
    } else if s.cart_type.is_none() {
        g.texture = 1;
        g.scale(0.75, 0.75, 0.75);
        g.scale(1.0 / 0.75, 1.0 / 0.75, 1.0 / 0.75);
    }
    g.texture = if s.cart_type.is_some() { 18 } else { 19 };
    g.scale(-1.0, -1.0, 1.0);
    for part in model(s.cart_type.is_some()) {
        super::special::part(g, &part, false);
    }
    g.pop();
}
