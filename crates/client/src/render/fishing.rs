//! RenderFish: billboard bobber and the original 17-vertex sagging line.
use super::entity::fixed::Fixed;
use worldgen::{
    entity::{Entity, Vec3D},
    math::{self, PI_F32 as PI},
};
pub fn render(
    fish: &Entity,
    angler: Option<&Entity>,
    relative: [f64; 3],
    partial: f32,
    brightness: f32,
    view: [f32; 2],
    third_person: bool,
) -> Fixed {
    let mut g = Fixed::new(brightness);
    g.push();
    g.translate(relative[0] as f32, relative[1] as f32, relative[2] as f32);
    g.enable(2, true);
    g.scale(0.5, 0.5, 0.5);
    g.texture = 17;
    g.rotate(180.0 - view[0], 0.0, 1.0, 0.0);
    g.rotate(-view[1], 1.0, 0.0, 0.0);
    g.begin();
    g.set_normal(0.0, 1.0, 0.0);
    g.quad(
        [
            [-0.5, -0.5, 0.0],
            [0.5, -0.5, 0.0],
            [0.5, 0.5, 0.0],
            [-0.5, 0.5, 0.0],
        ],
        [
            [8.0 / 128.0, 24.0 / 128.0],
            [16.0 / 128.0, 24.0 / 128.0],
            [16.0 / 128.0, 16.0 / 128.0],
            [8.0 / 128.0, 16.0 / 128.0],
        ],
    );
    g.end();
    g.enable(2, false);
    g.pop();
    if let Some(p) = angler {
        let yaw = p.prev_yaw + (p.yaw - p.prev_yaw) * partial;
        let pitch = p.prev_pitch + (p.pitch - p.prev_pitch) * partial;
        let mut swing = p.living.swing_progress - p.living.prev_swing_progress;
        if swing < 0.0 {
            swing += 1.0;
        }
        let progress = p.living.prev_swing_progress + swing * partial;
        let swing = math::sin(progress.sqrt() * PI);
        let mut hand = Vec3D::new(-0.5, 0.03, 0.8);
        hand.rotate_x(-pitch * PI / 180.0);
        hand.rotate_y(-yaw * PI / 180.0);
        hand.rotate_y(swing * 0.5);
        hand.rotate_x(-swing * 0.7);
        let lerp = |prev: f64, now: f64| prev + (now - prev) * partial as f64;
        let mut start = Vec3D::new(
            lerp(p.prev_pos.x, p.pos.x) + hand.x,
            lerp(p.prev_pos.y, p.pos.y) + hand.y,
            lerp(p.prev_pos.z, p.pos.z) + hand.z,
        );
        if third_person {
            let angle = (p.living.prev_render_yaw_offset
                + (p.living.render_yaw_offset - p.living.prev_render_yaw_offset) * partial)
                * PI
                / 180.0;
            let s = math::sin(angle) as f64;
            let c = math::cos(angle) as f64;
            start = Vec3D::new(
                lerp(p.prev_pos.x, p.pos.x) - c * 0.35 - s * 0.85,
                lerp(p.prev_pos.y, p.pos.y) - 0.45,
                lerp(p.prev_pos.z, p.pos.z) - s * 0.35 + c * 0.85,
            );
        }
        let end = Vec3D::new(
            lerp(fish.prev_pos.x, fish.pos.x),
            lerp(fish.prev_pos.y, fish.pos.y) + 0.25,
            lerp(fish.prev_pos.z, fish.pos.z),
        );
        let delta = [
            (start.x - end.x) as f32 as f64,
            (start.y - end.y) as f32 as f64,
            (start.z - end.z) as f32 as f64,
        ];
        g.enable(8, false);
        g.enable(1, false);
        g.begin();
        g.mode = 3;
        g.color([0.0, 0.0, 0.0, 1.0]);
        for i in 0..=16 {
            let t = i as f32 / 16.0;
            g.vertex(
                [
                    relative[0] + delta[0] * t as f64,
                    relative[1] + delta[1] * (t * t + t) as f64 * 0.5 + 0.25,
                    relative[2] + delta[2] * t as f64,
                ],
                [0.0; 2],
            );
        }
        g.untextured();
        g.end();
        g.enable(1, true);
        g.enable(8, true);
    }
    g
}
