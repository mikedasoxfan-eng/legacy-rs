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
        if let EntityKind::Slime { size, .. } | EntityKind::MagmaCube { size, .. } = &mut self.kind {
            *size = new_size as i8 as i32;
        }
        self.width = 0.6 * new_size as f32;
        self.height = 0.6 * new_size as f32;
        self.living.health = self.max_health();
        self.set_position(self.pos.x, self.pos.y, self.pos.z);
    }
    pub(super) fn squid_action_state(&mut self, w: &mut World) {
        self.living.entity_age += 1;
        if let EntityKind::Squid(s) = &mut self.kind {
            if self.living.entity_age > 100 {
                s.motion_x = 0.0;
                s.motion_y = 0.0;
                s.motion_z = 0.0;
            } else
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
        let player = w.closest_vulnerable_player(self.pos, 16.0);
        let magma = matches!(self.kind, EntityKind::MagmaCube { .. });
        if let Some(p) = player {
            self.face_target(w, Target::Player(p), 10.0, 20.0);
        }
        if let EntityKind::Slime {
            size,
            squish,
            jump_delay,
            ..
        }
        | EntityKind::MagmaCube {
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
                *jump_delay = ( self.rand.next_int(20) + 10) * if magma { 4 } else { 1 };
                if player.is_some() {
                    *jump_delay /= 3;
                }
                self.living.is_jumping = true;
                if *size > 1 {
                    let pitch =
                        ((self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0) * 0.8;
                    w.play_sound(
                        if magma {
                            if *size > 1 {
                                "mob.magmacube.big"
                            } else {
                                "mob.magmacube.small"
                            }
                        } else {
                        "mob.slime"
                        },
                        self.pos.add(0.0, -(self.y_offset as f64), 0.0),
                        0.4 * *size as f32,
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
                if !w.multiplayer_world {
                    self.motion.x = (s.motion_x * s.speed) as f64;
                    self.motion.y = (s.motion_y * s.speed) as f64;
                    self.motion.z = (s.motion_z * s.speed) as f64;
                }
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
                if !w.multiplayer_world {
                    self.motion.x = 0.0;
                    self.motion.y -= 0.08;
                    self.motion.y *= 0.98f32 as f64;
                    self.motion.z = 0.0;
                }
                s.pitch = (s.pitch as f64 + (-90.0 - s.pitch) as f64 * 0.02) as f32;
            }
        }
    }
    pub(super) fn slime_after_update(&mut self, w: &mut World, previous_ground: bool) {
        let magma = matches!(self.kind, EntityKind::MagmaCube { .. });
        if let EntityKind::Slime { size, squish, .. } | EntityKind::MagmaCube { size, squish, .. } = &mut self.kind {
            if self.on_ground && !previous_ground {
                for _ in 0..*size * 8 {
                    let angle = self.rand.next_float() * PI * 2.0;
                    let radius = self.rand.next_float() * 0.5 + 0.5;
                    let dx = math::sin(angle) * *size as f32 * 0.5 * radius;
                    let dz = math::cos(angle) * *size as f32 * 0.5 * radius;
                    w.spawn_particle(
                        if magma { "flame" } else {
                        "slime" },
                        super::Vec3D::new(
                            self.pos.x + dx as f64,
                            self.bounding_box.min_y,
                            self.pos.z + dz as f64,
                        ),
                        super::Vec3D::default(),
                    );
                }
                if magma || *size > 2 {
                    let pitch =
                        ((self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0) / 0.8;
                    w.play_sound(
                        if magma {
                            if *size > 1 {
                                "mob.magmacube.big"
                            } else {
                                "mob.magmacube.small"
                            }
                        } else {
                        "mob.slime"
                        },
                        self.pos.add(0.0, -(self.y_offset as f64), 0.0),
                        0.4 * *size as f32,
                        pitch,
                    );
                }
                *squish = -0.5;
            }
            *squish *= if magma { 0.9 } else { 0.6 };
        }
    }
}

#[cfg(test)]
mod release11_guard_tests {
 use super::*;
 #[test]
 fn squid_motion_guards_match_original_11_bytecode(){
  let mut w=crate::entity::verify181::world();
  for line in include_str!("../fixtures/squid_guard11.txt").lines(){
   let v:Vec<i64>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();w.multiplayer_world=v[0]!=0;
   w.editing_blocks=true;for x in -1..=1 {for z in -1..=1 {for y in 63..=65 {w.set_block_and_metadata(x,y,z,if v[2]!=0 {9}else{0},0);}}}w.editing_blocks=false;
   let mut e=Entity::living(&mut w,"Squid").unwrap();e.set_position(0.0,64.0,0.0);e.network.multiplayer_entity=v[1]!=0;e.motion=super::super::Vec3D::new(0.125,0.25,-0.375);e.living.render_yaw_offset=20.0;e.yaw=30.0;e.rand.set_seed(v[4]);
   let a:Vec<f32>=(1..=14).map(|n|n as f32*0.1).collect();
   e.kind=EntityKind::Squid(SquidState{pitch:a[0],prev_pitch:a[1],yaw:a[2],prev_yaw:a[3],rotation:f32::from_bits(v[3] as u32),prev_rotation:a[5],tentacle_angle:a[6],prev_tentacle_angle:a[7],speed:a[8],rotation_speed:a[9],yaw_speed:a[10],motion_x:a[11],motion_y:a[12],motion_z:a[13]});
   e.squid_living_update(&mut w);
   let EntityKind::Squid(q)=&e.kind else{unreachable!()};
   let actual:Vec<i64>=[q.pitch,q.prev_pitch,q.yaw,q.prev_yaw,q.rotation,q.prev_rotation,q.tentacle_angle,q.prev_tentacle_angle,q.speed,q.rotation_speed,q.yaw_speed,q.motion_x,q.motion_y,q.motion_z].iter().map(|f|f.to_bits() as i32 as i64).chain([e.motion.x.to_bits() as i64,e.motion.y.to_bits() as i64,e.motion.z.to_bits() as i64,e.living.render_yaw_offset.to_bits() as i32 as i64,e.yaw.to_bits() as i32 as i64,e.rand.next_long()]).collect();
   assert_eq!(actual,v[5..],"{line}");
  }
 }
 #[test]
 fn living_action_guards_match_original_11_bytecode(){
  let mut w=crate::entity::verify181::world();
  for line in include_str!("../fixtures/living_guard11.txt").lines(){let v:Vec<i64>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();w.multiplayer_world=v[0]!=0;
   let mut e=Entity::living(&mut w,"Cow").unwrap();e.network.multiplayer_entity=v[1]!=0;assert_eq!(e.runs_living_action11(&w) as i64,v[2],"{line}");
   let mut p=Entity::player(&mut w,"local");p.network.multiplayer_entity=v[1]!=0;assert_eq!(p.runs_living_action11(&w) as i64,v[3],"{line}");
   let mut squid=Entity::living(&mut w,"Squid").unwrap();squid.set_position(0.0,64.0,0.0);squid.network.multiplayer_entity=v[1]!=0;squid.living.entity_age=0;squid.living_update(&mut w);assert_eq!(squid.living.entity_age,v[2] as i32,"Actual callback must use the world guard: {line}");
  }
 }
}
