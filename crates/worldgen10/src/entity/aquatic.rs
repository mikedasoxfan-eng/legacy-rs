use super::{living::Target, Entity, EntityKind};
use crate::{
    block::*,
    math::{self, PI_F32 as PI},
    World,
};
#[derive(Clone, Debug, Default)]
pub struct SquidState {
    pub pitch: f32,
    pub prev_pitch: f32,
    pub yaw: f32,
    pub prev_yaw: f32,
    pub rotation: f32,
    pub prev_rotation: f32,
    pub tentacle_angle: f32,
    pub prev_tentacle_angle: f32,
    pub speed: f32,
    pub rotation_speed: f32,
    pub yaw_speed: f32,
    pub motion_x: f32,
    pub motion_y: f32,
    pub motion_z: f32,
}
impl Entity {
    pub fn set_slime_size(&mut self, new_size: i32) {
        self.data_watcher.update(16, super::data_watcher::Value::Byte(new_size as i8)).unwrap();
        if let EntityKind::Slime { size, .. } = &mut self.kind {
            *size = new_size as i8 as i32;
        }
        self.width = 0.6 * new_size as f32;
        self.height = 0.6 * new_size as f32;
        self.living.health = new_size * new_size;
        self.set_position(self.pos.x, self.pos.y, self.pos.z);
    }
    pub(super) fn squid_action_state(&mut self, w: &mut World) {
        if let EntityKind::Squid(s) = &mut self.kind {
            if self.rand.next_int(50) == 0
                || !self.in_water
                || (s.motion_x == 0.0 && s.motion_y == 0.0 && s.motion_z == 0.0)
            {
                let angle = self.rand.next_float() * PI * 2.0;
                s.motion_x = math::cos(angle) * 0.2;
                s.motion_y = -0.1 + self.rand.next_float() * 0.2;
                s.motion_z = math::sin(angle) * 0.2;
            }
        }
        self.despawn(w);
    }
    pub(super) fn slime_action_state(&mut self, w: &mut World) {
        self.despawn(w);
        let player = w.closest_player(self.pos, 16.0);
        if let Some(p) = player {
            self.face_target(w, Target::Player(p), 10.0, 20.0);
        }
        if let EntityKind::Slime {
            size,
            squish,
            jump_delay,
            ..
        } = &mut self.kind
        {
            let jump = if self.on_ground {
                let old = *jump_delay;
                *jump_delay -= 1;
                old <= 0
            } else {
                false
            };
            if jump {
                *jump_delay = self.rand.next_int(20) + 10;
                if player.is_some() {
                    *jump_delay /= 3;
                }
                self.living.is_jumping = true;
                if *size > 1 {
                    let pitch =
                        ((self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0) * 0.8;
                    w.play_sound(
                        "mob.slime",
                        self.pos.add(0.0, -(self.y_offset as f64), 0.0),
                        0.6,
                        pitch,
                    );
                }
                *squish = 1.0;
                self.living.move_strafing = 1.0 - self.rand.next_float() * 2.0;
                self.living.move_forward = *size as f32;
            } else {
                self.living.is_jumping = false;
                if self.on_ground {
                    self.living.move_strafing = 0.0;
                    self.living.move_forward = 0.0;
                }
            }
        }
    }
    pub fn is_in_water_query(&mut self, w: &mut World) -> bool {
        if matches!(self.kind, EntityKind::Squid(..)) {
            w.handle_material_acceleration(
                self.bounding_box.expand(0.0, -0.6f32 as f64, 0.0),
                MAT_WATER,
                &mut self.motion,
            )
        } else {
            self.in_water
        }
    }
    pub(super) fn squid_living_update(&mut self, w: &mut World) {
        let water = self.is_in_water_query(w);
        if let EntityKind::Squid(s) = &mut self.kind {
            s.prev_pitch = s.pitch;
            s.prev_yaw = s.yaw;
            s.prev_rotation = s.rotation;
            s.prev_tentacle_angle = s.tentacle_angle;
            s.rotation += s.rotation_speed;
            if s.rotation > PI * 2.0 {
                s.rotation -= PI * 2.0;
                if self.rand.next_int(10) == 0 {
                    s.rotation_speed = 1.0 / (self.rand.next_float() + 1.0) * 0.2;
                }
            }
            if water {
                if s.rotation < PI {
                    let phase = s.rotation / PI;
                    s.tentacle_angle = math::sin(phase * phase * PI) * PI * 0.25;
                    if phase as f64 > 0.75 {
                        s.speed = 1.0;
                        s.yaw_speed = 1.0;
                    } else {
                        s.yaw_speed *= 0.8;
                    }
                } else {
                    s.tentacle_angle = 0.0;
                    s.speed *= 0.9;
                    s.yaw_speed *= 0.99;
                }
                self.motion.x = (s.motion_x * s.speed) as f64;
                self.motion.y = (s.motion_y * s.speed) as f64;
                self.motion.z = (s.motion_z * s.speed) as f64;
                let horizontal =
                    (self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt() as f32;
                self.living.render_yaw_offset +=
                    (-(crate::jmath::java_atan2(self.motion.x, self.motion.z) as f32) * 180.0 / PI
                        - self.living.render_yaw_offset)
                        * 0.1;
                self.yaw = self.living.render_yaw_offset;
                s.yaw += PI * s.yaw_speed * 1.5;
                s.pitch += (-(crate::jmath::java_atan2(horizontal as f64, self.motion.y) as f32)
                    * 180.0
                    / PI
                    - s.pitch)
                    * 0.1;
            } else {
                s.tentacle_angle = math::sin(s.rotation).abs() * PI * 0.25;
                self.motion.x = 0.0;
                self.motion.y -= 0.08;
                self.motion.y *= 0.98f32 as f64;
                self.motion.z = 0.0;
                s.pitch = (s.pitch as f64 + (-90.0 - s.pitch) as f64 * 0.02) as f32;
            }
        }
    }
    pub(super) fn slime_after_update(&mut self, w: &mut World, previous_ground: bool) {
        if let EntityKind::Slime { size, squish, .. } = &mut self.kind {
            if self.on_ground && !previous_ground {
                for _ in 0..*size * 8 {
                    let angle = self.rand.next_float() * PI * 2.0;
                    let radius = self.rand.next_float() * 0.5 + 0.5;
                    let dx = math::sin(angle) * *size as f32 * 0.5 * radius;
                    let dz = math::cos(angle) * *size as f32 * 0.5 * radius;
                    w.spawn_particle(
                        "slime",
                        super::Vec3D::new(
                            self.pos.x + dx as f64,
                            self.bounding_box.min_y,
                            self.pos.z + dz as f64,
                        ),
                        super::Vec3D::default(),
                    );
                }
                if *size > 2 {
                    let pitch =
                        ((self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0) / 0.8;
                    w.play_sound(
                        "mob.slime",
                        self.pos.add(0.0, -(self.y_offset as f64), 0.0),
                        0.6,
                        pitch,
                    );
                }
                *squish = -0.5;
            }
            *squish *= 0.6;
        }
    }
}
