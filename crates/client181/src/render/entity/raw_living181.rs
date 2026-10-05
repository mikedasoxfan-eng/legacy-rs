//! Original draw inputs for Enderman, Silverfish and the two spider renderers.
use super::{
    fixed::Fixed,
    living::{MobKind, RenderState},
    models::Model,
    objects::{self, ObjectKind, ObjectState},
    special,
};
use worldgen::{math::PI_F32, JavaRandom};
pub fn embedded_arrows(g: &mut Fixed, m: &Model, id: i32, count: i32, partial: f32) {
    let mut r = JavaRandom::new(id as i64);
    for _ in 0..count {
        g.push();
        let p = m.random_part(&mut r);
        special::part(g, p, true);
        let mut v: [f32; 3] = std::array::from_fn(|_| r.next_float());
        let xyz: [f32; 3] = std::array::from_fn(|i| {
            (p.raw_bounds[i] + (p.raw_bounds[i + 3] - p.raw_bounds[i]) * v[i]) / 16.0
        });
        g.translate(xyz[0], xyz[1], xyz[2]);
        for v in &mut v {
            *v = *v * 2.0 - 1.0;
            *v *= -1.0;
        }
        let horizontal = (v[0] * v[0] + v[2] * v[2]).sqrt();
        let yaw =
            (worldgen::jmath::java_atan2(v[0] as f64, v[2] as f64) * 180.0 / PI_F32 as f64) as f32;
        let pitch = (worldgen::jmath::java_atan2(v[1] as f64, horizontal as f64) * 180.0
            / PI_F32 as f64) as f32;
        let state = ObjectState {
            kind: ObjectKind::Arrow {
                yaw,
                pitch,
                shake: 0,
                visible: true,
            },
            position: [0.0; 3],
            partial,
            brightness: g.material[0],
        };
        let arrow = objects::render(&state, [0.0; 3], [0.0; 2]);
        for mut draw in arrow.draws {
            let mut ops = g.ops.clone();
            ops.extend(draw.ops);
            draw.ops = ops;
            draw.material = g.material;
            draw.flags = (draw.flags & (64 | 128 | 512)) | (g.flags & !(64 | 128 | 512)) | 2;
            draw.src = g.src;
            draw.dst = g.dst;
            draw.depth = g.depth;
            g.draws.push(draw);
        }
        g.texture = 3;
        g.normal = [0.0, 0.0, 0.05625];
        g.enable(2, false);
        g.pop();
    }
}
pub fn render(s: &RenderState, camera: [f64; 3], normalize: bool) -> Fixed {
    if !matches!(
        s.kind,
        MobKind::Enderman | MobKind::Silverfish | MobKind::Spider | MobKind::CaveSpider
    ) {
        return general(s, camera, normalize);
    }
    let mut g = Fixed::new(s.brightness);
    if normalize {
        g.enable(4, true);
    }
    let mut m = Model::new(super::living::model_kind(s.kind));
    m.on_ground = s.attack;
    m.is_riding = s.riding;
    m.carrying = s.carrying;
    m.screaming = s.screaming;
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
        (s.position[0] - camera[0] + s.scream_jitter[0]) as f32,
        (s.position[1] - camera[1]) as f32,
        (s.position[2] - camera[2] + s.scream_jitter[1]) as f32,
    );
    g.rotate(180.0 - s.body_yaw, 0.0, 1.0, 0.0);
    if s.death_time > 0 {
        let f = (((s.death_time as f32 + s.partial - 1.0) / 20.0 * 1.6) as f64).sqrt() as f32;
        g.rotate(
            f.min(1.0)
                * if matches!(s.kind, MobKind::Enderman) {
                    90.0
                } else {
                    180.0
                },
            0.0,
            0.0,
            1.0,
        );
    }
    g.enable(2, true);
    g.scale(-1.0, -1.0, 1.0);
    if matches!(s.kind, MobKind::Spider | MobKind::CaveSpider) {
        let scale = if s.kind == MobKind::CaveSpider {
            0.7
        } else {
            1.0
        };
        g.scale(scale, scale, scale);
    }
    g.translate(0.0, -24.0 * 0.0625 - 0.0078125, 0.0);
    g.texture = match s.kind {
        MobKind::Enderman => 40,
        MobKind::Silverfish => 42,
        MobKind::Spider => 43,
        MobKind::CaveSpider => 45,
        _ => unreachable!(),
    };
    special::model(&mut g, &m);
    let eyes = s.kind != MobKind::Silverfish;
    if eyes {
        g.texture = if s.kind == MobKind::Enderman { 41 } else { 44 };
        g.enable(16, true);
        g.src = 1;
        g.dst = 1;
        g.material = [1.0; 4];
        special::model(&mut g, &m);
        g.enable(16, false);
    }
    embedded_arrows(&mut g, &m, s.entity_id, s.arrow_count, s.partial);
    if s.kind == MobKind::Enderman && s.carried_block > 0 {
        g.enable(2, true);
        g.push();
        g.translate(0.0, 11.0 / 16.0, -12.0 / 16.0);
        g.rotate(20.0, 1.0, 0.0, 0.0);
        g.rotate(45.0, 0.0, 1.0, 0.0);
        g.scale(0.5, -0.5, 0.5);
        g.material = [1.0; 4];
        g.texture = 1;
        super::inventory::inventory(&mut g, s.carried_block, s.carried_data, 1.0, false);
        g.pop();
        g.enable(2, false);
    }
    if s.hurt_time > 0 || s.death_time > 0 {
        g.enable(8, false);
        g.enable(16, true);
        g.src = 770;
        g.dst = 771;
        g.depth = 514;
        g.material = [s.overlay_brightness, 0.0, 0.0, 0.4];
        special::model(&mut g, &m);
        if eyes {
            g.texture = if s.kind == MobKind::Enderman { 41 } else { 44 };
            g.src = 1;
            g.dst = 1;
            g.material = [s.overlay_brightness, 0.0, 0.0, 0.4];
            special::model(&mut g, &m);
        }
        g.depth = 515;
        g.enable(16, false);
        g.enable(8, true);
    }
    g.enable(2, false);
    g.pop();
    g
}

/// RenderLiving and the animal/biped callbacks, using the same models as the frame.
fn general(s: &RenderState, camera: [f64; 3], normalize: bool) -> Fixed {
    let mut g = Fixed::new(s.brightness);
    if normalize {
        g.enable(4, true);
    }
    let mut m = Model::new(super::living::model_kind(s.kind));
    m.on_ground = s.attack;
    m.is_riding = s.riding;
    let age = match s.kind {
        MobKind::Chicken => (worldgen::math::sin(s.chicken_flap) + 1.0) * s.chicken_flap_amount,
        MobKind::Squid => s.tentacle_angle,
        MobKind::Wolf => {
            if s.wolf.angry {
                PI_F32 * 0.49
            } else if s.wolf_tamed {
                (0.55 - (20 - s.wolf_health) as f32 * 0.02) * PI_F32
            } else {
                PI_F32 * 0.2
            }
        }
        _ => s.age,
    };
    let pose = [
        s.limb_swing,
        s.limb_amount.min(1.0),
        age,
        s.yaw - s.body_yaw,
        s.pitch,
        0.0625,
    ];
    m.set_living_animations(pose[0], pose[1], s.partial, &s.wolf);
    m.set_rotation_angles(pose);
    g.push();
    g.translate(
        (s.position[0] - camera[0]) as f32,
        (s.position[1] - camera[1]) as f32,
        (s.position[2] - camera[2]) as f32,
    );
    if s.kind == MobKind::Squid {
        g.translate(0.0, 0.5, 0.0);
        g.rotate(180.0 - s.body_yaw, 0.0, 1.0, 0.0);
        g.rotate(s.squid_pitch, 1.0, 0.0, 0.0);
        g.rotate(s.squid_yaw, 0.0, 1.0, 0.0);
        g.translate(0.0, -1.2, 0.0);
    } else {
        g.rotate(180.0 - s.body_yaw, 0.0, 1.0, 0.0);
        if s.death_time > 0 {
            let f = (((s.death_time as f32 + s.partial - 1.0) / 20.0 * 1.6) as f64).sqrt() as f32;
            g.rotate(f.min(1.0) * 90.0, 0.0, 0.0, 1.0);
        }
    }
    g.enable(2, true);
    g.scale(-1.0, -1.0, 1.0);
    if s.kind == MobKind::Creeper {
        let mut f = s.creeper_flash;
        let pulse = 1.0 + worldgen::math::sin(f * 100.0) * f * 0.01;
        f = f.clamp(0.0, 1.0);
        f *= f;
        f *= f;
        let x = (1.0 + f * 0.4) * pulse;
        g.scale(x, (1.0 + f * 0.1) / pulse, x);
    }
    g.translate(0.0, -24.0 * 0.0625 - 0.0078125, 0.0);
    g.texture = match s.kind {
        MobKind::Zombie => 7,
        MobKind::Skeleton => 46,
        MobKind::Pig => 47,
        MobKind::Cow => 49,
        MobKind::Sheep => 50,
        MobKind::Chicken => 52,
        MobKind::Squid => 53,
        MobKind::Wolf => {
            if s.wolf_tamed {
                55
            } else if s.wolf.angry {
                56
            } else {
                54
            }
        }
        MobKind::Creeper => 57,
        _ => unreachable!(),
    };
    special::model(&mut g, &m);
    if s.kind == MobKind::Pig {
        g.texture = 48;
    }
    let pass = match s.kind {
        MobKind::Pig if s.saddled => Some((
            48,
            Model::with_options(super::models::ModelKind::Pig, 0.5, 0.0, 6),
        )),
        MobKind::Sheep if !s.sheared => Some((51, Model::new(super::models::ModelKind::SheepFur))),
        MobKind::Creeper if s.powered => Some((
            58,
            Model::with_options(super::models::ModelKind::Creeper, 2.0, 0.0, 6),
        )),
        _ => None,
    };
    let mut pass = pass.map(|(texture, mut model)| {
        model.on_ground = s.attack;
        model.is_riding = s.riding;
        model.set_rotation_angles(pose);
        (texture, model)
    });
    if let Some((texture, model)) = &mut pass {
        g.texture = *texture;
        if s.kind == MobKind::Sheep {
            let c = super::living::FLEECE_COLORS[s.fleece_color];
            g.material = [c[0], c[1], c[2], 1.0];
        }
        if s.kind == MobKind::Creeper {
            g.enable(16, true);
            g.enable(1, false);
            g.src = 1;
            g.dst = 1;
            g.material = [0.5, 0.5, 0.5, 1.0];
        }
        special::model(&mut g, model);
        if s.kind == MobKind::Creeper {
            g.enable(1, true);
            g.enable(16, false);
        }
    }
    embedded_arrows(&mut g, &m, s.entity_id, s.arrow_count, s.partial);
    if let Some(item) = s.held_item {
        g.push();
        special::part(&mut g, &m["bipedRightArm"], true);
        g.translate(-0.0625, 0.4375, 0.0625);
        if item.icon == 21 {
            g.translate(0.0, 0.125, 0.3125);
            g.rotate(-20.0, 0.0, 1.0, 0.0);
        } else {
            g.translate(0.0, 0.1875, 0.0);
        }
        g.scale(0.625, -0.625, 0.625);
        g.rotate(-100.0, 1.0, 0.0, 0.0);
        g.rotate(45.0, 0.0, 1.0, 0.0);
        special::sprite(&mut g, item.icon, item.terrain);
        g.pop();
    }
    let flash = if s.kind == MobKind::Creeper && (s.creeper_flash * 10.0) as i32 % 2 != 0 {
        (s.creeper_flash * 0.2 * 255.0) as i32
    } else {
        0
    };
    if s.hurt_time > 0 || s.death_time > 0 || flash > 0 {
        g.enable(8, false);
        g.enable(16, true);
        g.src = 770;
        g.dst = 771;
        g.depth = 514;
        if s.hurt_time > 0 || s.death_time > 0 {
            g.material = [s.overlay_brightness, 0.0, 0.0, 0.4];
            special::model(&mut g, &m);
            if s.kind != MobKind::Creeper {
                if let Some((texture, model)) = &pass {
                    g.texture = *texture;
                    special::model(&mut g, model);
                }
            }
        }
        if flash > 0 {
            g.material = [1.0, 1.0, 1.0, flash.clamp(0, 255) as f32 / 255.0];
            special::model(&mut g, &m);
        }
        g.depth = 515;
        g.enable(16, false);
        g.enable(8, true);
    }
    g.enable(2, false);
    g.pop();
    g
}

/// Lightmap coordinate transitions made by the original eye/carried-block callbacks.
pub fn packed_brightnesses(draws: &[super::fixed::Draw], base: i32) -> Vec<i32> {
    let mut value = base;
    draws
        .iter()
        .map(|d| {
            if matches!(d.texture, 41 | 44) {
                value = 61680;
            } else if d.texture == 1 {
                value = base;
            }
            value
        })
        .collect()
}
