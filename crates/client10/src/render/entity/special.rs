//! RenderGhast, inherited RenderBiped for pig zombies, and RenderGiantZombie.
//! Emit original GL inputs so both the frame and oracle use this implementation.
use super::{
    fixed::Fixed,
    geometry::ModelRenderer,
    living::{MobKind, RenderState},
    models::Model,
};
use worldgen::math::PI_F32;
fn angles(g: &mut Fixed, p: &ModelRenderer) {
    if p.rotate_angle_z != 0.0 {
        g.rotate(p.rotate_angle_z * (180.0 / PI_F32), 0.0, 0.0, 1.0);
    }
    if p.rotate_angle_y != 0.0 {
        g.rotate(p.rotate_angle_y * (180.0 / PI_F32), 0.0, 1.0, 0.0);
    }
    if p.rotate_angle_x != 0.0 {
        g.rotate(p.rotate_angle_x * (180.0 / PI_F32), 1.0, 0.0, 0.0);
    }
}
pub(crate) fn part(g: &mut Fixed, p: &ModelRenderer, post: bool) {
    if p.hidden || !p.show_model {
        return;
    }
    let rotated = p.rotate_angle_x != 0.0 || p.rotate_angle_y != 0.0 || p.rotate_angle_z != 0.0;
    let moved = p.rotation_point_x != 0.0 || p.rotation_point_y != 0.0 || p.rotation_point_z != 0.0;
    if rotated && !post {
        g.push();
    }
    if rotated || moved {
        g.translate(
            p.rotation_point_x * 0.0625,
            p.rotation_point_y * 0.0625,
            p.rotation_point_z * 0.0625,
        );
    }
    if rotated {
        angles(g, p);
    }
    if !post {
        for face in &p.faces {
            g.begin();
            let n = face.normal();
            g.set_normal(n.x, n.y, n.z);
            for v in &face.vertices {
                g.vertex(
                    v.pos.map(|x| (x as f32 * 0.0625) as f64),
                    v.uv.map(|x| x as f64),
                );
            }
            g.end();
        }
        if rotated {
            g.pop();
        } else if moved {
            g.translate(
                -p.rotation_point_x * 0.0625,
                -p.rotation_point_y * 0.0625,
                -p.rotation_point_z * 0.0625,
            );
        }
    }
}
pub(crate) fn model(g: &mut Fixed, m: &Model) {
    for (p, with_rotation) in m.render_parts() {
        if with_rotation {
            if p.hidden || !p.show_model {
                continue;
            }
            g.push();
            g.translate(
                p.rotation_point_x * 0.0625,
                p.rotation_point_y * 0.0625,
                p.rotation_point_z * 0.0625,
            );
            for (angle, axis) in [
                (p.rotate_angle_y, [0.0, 1.0, 0.0]),
                (p.rotate_angle_x, [1.0, 0.0, 0.0]),
                (p.rotate_angle_z, [0.0, 0.0, 1.0]),
            ] {
                if angle != 0.0 {
                    g.rotate(angle * (180.0 / PI_F32), axis[0], axis[1], axis[2]);
                }
            }
            for face in &p.faces {
                g.begin();
                let n = face.normal();
                g.set_normal(n.x, n.y, n.z);
                for v in &face.vertices {
                    g.vertex(
                        v.pos.map(|x| (x as f32 * 0.0625) as f64),
                        v.uv.map(|x| x as f64),
                    );
                }
                g.end();
            }
            g.pop();
        } else {
            part(g, p, false);
        }
    }
}
pub fn render(s: &RenderState, camera: [f64; 3], normalize: bool) -> Fixed {
    let mut g = Fixed::new(s.brightness);
    if normalize {
        g.enable(4, true);
    }
    let mut m = Model::new(super::living::model_kind(s.kind));
    m.on_ground = s.attack;
    m.is_riding = s.riding;
    let pose = [
        s.limb_swing,
        s.limb_amount.min(1.0),
        s.age,
        s.yaw - s.body_yaw,
        s.pitch,
        0.0625,
    ];
    m.set_rotation_angles(pose);
    g.push();
    g.translate(
        (s.position[0] - camera[0]) as f32,
        (s.position[1] - camera[1]) as f32,
        (s.position[2] - camera[2]) as f32,
    );
    g.rotate(180.0 - s.body_yaw, 0.0, 1.0, 0.0);
    if s.death_time > 0 {
        let mut f = ((s.death_time as f32 + s.partial - 1.0) / 20.0 * 1.6) as f64;
        f = f.sqrt();
        g.rotate((f as f32).min(1.0) * 90.0, 0.0, 0.0, 1.0);
    }
    g.enable(2, true);
    g.scale(-1.0, -1.0, 1.0);
    if s.kind == MobKind::Ghast {
        let mut f = (s.ghast_attack / 20.0).max(0.0);
        f = 1.0 / (f * f * f * f * f * 2.0 + 1.0);
        let y = (8.0 + f) / 2.0;
        let xz = (8.0 + 1.0 / f) / 2.0;
        g.scale(xz, y, xz);
        g.material = [1.0; 4];
    } else if s.kind == MobKind::Giant {
        g.scale(6.0, 6.0, 6.0);
    }
    g.translate(0.0, -24.0 * 0.0625 - 0.0078125, 0.0);
    g.texture = match s.kind {
        MobKind::Ghast => {
            if s.ghast_attacking {
                6
            } else {
                5
            }
        }
        MobKind::PigZombie => 8,
        MobKind::Giant => 7,
        _ => unreachable!(),
    };
    model(&mut g, &m);
    if s.kind == MobKind::PigZombie {
        if let Some(item) = s.held_item {
            held(&mut g, &m, item.icon);
        }
    }
    if s.hurt_time > 0 || s.death_time > 0 {
        g.enable(8, false);
        g.enable(16, true);
        g.src = 770;
        g.dst = 771;
        g.depth = 514;
        g.material = [s.brightness, 0.0, 0.0, 0.4];
        model(&mut g, &m);
        g.depth = 515;
        g.enable(16, false);
        g.enable(8, true);
    }
    g.enable(2, false);
    g.pop();
    g
}
fn held(g: &mut Fixed, m: &Model, icon: i32) {
    g.push();
    part(g, &m["bipedRightArm"], true);
    g.translate(-0.0625, 0.4375, 0.0625);
    g.translate(0.0, 0.1875, 0.0);
    g.scale(0.625, -0.625, 0.625);
    g.rotate(-100.0, 1.0, 0.0, 0.0);
    g.rotate(45.0, 0.0, 1.0, 0.0);
    sprite(g, icon, false);
    g.pop();
}

/// ItemRenderer.renderItem flat item extrusion.
pub(crate) fn sprite(g: &mut Fixed, icon: i32, terrain: bool) {
    g.push();
    g.texture = if terrain { 1 } else { 2 };
    g.enable(2, true);
    g.translate(-0.0, -0.3, 0.0);
    g.scale(1.5, 1.5, 1.5);
    g.rotate(50.0, 0.0, 1.0, 0.0);
    g.rotate(335.0, 0.0, 0.0, 1.0);
    g.translate(-0.9375, -0.0625, 0.0);
    let u0 = (icon % 16 * 16) as f32 / 256.0;
    let u1 = ((icon % 16 * 16) as f32 + 15.99) / 256.0;
    let v0 = (icon / 16 * 16) as f32 / 256.0;
    let v1 = ((icon / 16 * 16) as f32 + 15.99) / 256.0;
    let t = 0.0625;
    g.begin();
    g.set_normal(0.0, 0.0, 1.0);
    g.quad(
        [
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ],
        [[u1, v1], [u0, v1], [u0, v0], [u1, v0]],
    );
    g.end();
    g.begin();
    g.set_normal(0.0, 0.0, -1.0);
    g.quad(
        [
            [0.0, 1.0, -t],
            [1.0, 1.0, -t],
            [1.0, 0.0, -t],
            [0.0, 0.0, -t],
        ],
        [[u1, v0], [u0, v0], [u0, v1], [u1, v1]],
    );
    g.end();
    for side in 0..4 {
        g.begin();
        let n = [
            [-1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0],
        ][side];
        g.set_normal(n[0], n[1], n[2]);
        for i in 0..16 {
            let f = i as f32 / 16.0;
            let u = u1 + (u0 - u1) * f - 0.001953125;
            let v = v1 + (v0 - v1) * f - 0.001953125;
            let a = f + if side == 1 || side == 2 { 0.0625 } else { 0.0 };
            match side {
                0 => g.quad(
                    [[a, 0.0, -t], [a, 0.0, 0.0], [a, 1.0, 0.0], [a, 1.0, -t]],
                    [[u, v1], [u, v1], [u, v0], [u, v0]],
                ),
                1 => g.quad(
                    [[a, 1.0, -t], [a, 1.0, 0.0], [a, 0.0, 0.0], [a, 0.0, -t]],
                    [[u, v0], [u, v0], [u, v1], [u, v1]],
                ),
                2 => g.quad(
                    [[0.0, a, 0.0], [1.0, a, 0.0], [1.0, a, -t], [0.0, a, -t]],
                    [[u1, v], [u0, v], [u0, v], [u1, v]],
                ),
                3 => g.quad(
                    [[1.0, a, 0.0], [0.0, a, 0.0], [0.0, a, -t], [1.0, a, -t]],
                    [[u0, v], [u1, v], [u1, v], [u0, v]],
                ),
                _ => unreachable!(),
            }
        }
        g.end();
    }
    g.enable(2, false);
    g.pop();
}
