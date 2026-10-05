use super::{
    living::{wrap, Target},
    Entity, EntityKind, Vec3D,
};
use crate::{
    block::*,
    math::{self, floor_double as floor},
    World,
};
impl Entity {
    pub fn target_position(&self, w: &World, target: Target) -> Option<(Vec3D, f32, bool)> {
        match target {
            Target::Player(i) => {
                if let Some(e) = w
                    .target_entity_id(target)
                    .and_then(|id| w.entities.get(&id))
                {
                    Some((e.pos, e.eye_height(), !e.dead && e.living.health > 0))
                } else {
                    w.players
                        .get(i)
                        .map(|&(x, y, z)| (Vec3D::new(x, y, z), 0.12, true))
                }
            }
            Target::Entity(id) => w.entities.get(&id).map(|e| {
                (
                    e.pos,
                    e.eye_height(),
                    !e.dead && (!e.kind.is_living() || e.living.health > 0),
                )
            }),
        }
    }
    pub fn can_see_target(&self, w: &mut World, target: Target) -> bool {
        if let Some((p, eye, _)) = self.target_position(w, target) {
            w.ray_trace_blocks(
                self.pos.add(0.0, self.eye_height() as f64, 0.0),
                p.add(0.0, eye as f64, 0.0),
            )
            .is_none()
        } else {
            false
        }
    }
    pub fn face_target(&mut self, w: &World, target: Target, yaw_speed: f32, pitch_speed: f32) {
        if let Some((p, eye, _)) = self.target_position(w, target) {
            let x = p.x - self.pos.x;
            let z = p.z - self.pos.z;
            let y = self.pos.y + self.eye_height() as f64 - (p.y + eye as f64);
            let horizontal = (x * x + z * z).sqrt() as f32 as f64;
            let yaw = (crate::jmath::java_atan2(z, x) * 180.0 / math::PI_F32 as f64) as f32 - 90.0;
            let pitch =
                (-(crate::jmath::java_atan2(y, horizontal) * 180.0 / math::PI_F32 as f64)) as f32;
            self.pitch = -update_rotation(self.pitch, pitch, pitch_speed);
            self.yaw = update_rotation(self.yaw, yaw, yaw_speed);
        }
    }
    pub fn despawn(&mut self, w: &World) {
        if matches!(
            self.kind,
            EntityKind::Pig { .. }
                | EntityKind::Sheep { .. }
                | EntityKind::Cow
                | EntityKind::Chicken { .. }
                | EntityKind::Wolf(..)
        ) {
            return;
        }
        if let Some(i) = w.closest_player(self.pos, -1.0) {
            let (x, y, z) = w.players[i];
            let d = Vec3D::new(x, y, z).square_distance(self.pos);
            if d > 16384.0 {
                self.dead = true;
            }
            if self.living.entity_age > 600 && self.rand.next_int(800) == 0 {
                if d < 1024.0 {
                    self.living.entity_age = 0;
                } else {
                    self.dead = true;
                }
            }
        }
    }
    pub(super) fn living_action_state(&mut self, w: &mut World) {
        self.living.entity_age += 1;
        self.despawn(w);
        self.living.move_strafing = 0.0;
        self.living.move_forward = 0.0;
        if self.rand.next_float() < 0.02 {
            if let Some(i) = w.closest_player(self.pos, 8.0) {
                self.living.current_target = Some(Target::Player(i));
                self.living.chase_ticks = 10 + self.rand.next_int(20);
            } else {
                self.living.random_yaw_velocity = (self.rand.next_float() - 0.5) * 20.0;
            }
        }
        if let Some(target) = self.living.current_target {
            self.face_target(
                w,
                target,
                10.0,
                if matches!(&self.kind,EntityKind::Wolf(s) if s.flags&1!=0) {
                    20.0
                } else {
                    40.0
                },
            );
            let chase = self.living.chase_ticks;
            self.living.chase_ticks -= 1;
            if chase <= 0
                || self
                    .target_position(w, target)
                    .map_or(true, |(p, _, alive)| {
                        !alive || p.square_distance(self.pos) > 64.0
                    })
            {
                self.living.current_target = None;
            }
        } else {
            if self.rand.next_float() < 0.05 {
                self.living.random_yaw_velocity = (self.rand.next_float() - 0.5) * 20.0;
            }
            self.yaw += self.living.random_yaw_velocity;
            self.pitch = self.living.default_pitch;
        }
        let water = self.in_water;
        let lava = self.handle_lava_movement(w);
        if water || lava {
            self.living.is_jumping = self.rand.next_float() < 0.8;
        }
    }
    pub fn block_path_weight(&self, w: &mut World, x: i32, y: i32, z: i32) -> f32 {
        if matches!(self.kind, EntityKind::Silverfish { .. }) && w.get_block_id(x, y - 1, z) == 1 {
            return 10.0;
        }
        if self.kind.is_mob() {
            return if matches!(self.kind, EntityKind::Giant) {
                w.get_light_brightness(x, y, z) - 0.5
            } else {
                0.5 - w.get_light_brightness(x, y, z)
            };
        }
        if w.get_block_id(x, y - 1, z) == GRASS {
            10.0
        } else {
            w.get_light_brightness(x, y, z) - 0.5
        }
    }
    pub(super) fn wander(&mut self, w: &mut World) {
        let mut best = None;
        let mut weight = -99999.0f32;
        for _ in 0..10 {
            let x = floor(self.pos.x + self.rand.next_int(13) as f64 - 6.0);
            let y = floor(self.pos.y + self.rand.next_int(7) as f64 - 3.0);
            let z = floor(self.pos.z + self.rand.next_int(13) as f64 - 6.0);
            let score = self.block_path_weight(w, x, y, z);
            if score > weight {
                weight = score;
                best = Some((x, y, z));
            }
        }
        if let Some((x, y, z)) = best {
            self.living.path = w.entity_path_to_xyz(self, x, y, z, 10.0);
        }
    }
    pub(super) fn creature_action_state(&mut self, w: &mut World) {
        if self.living.flee_ticks > 0 {
            self.living.flee_ticks -= 1;
        }
        self.living.has_attacked =
            matches!(&self.kind,EntityKind::Wolf(s) if s.flags&1!=0||s.shake_active);
        if self.living.target.is_none() {
            self.living.target = self.find_attack_target(w);
            if let Some((mut p, _, _)) = self.living.target.and_then(|t| self.target_position(w, t))
            {
                if let Some(e) = self
                    .living
                    .target
                    .and_then(|t| w.target_entity_id(t))
                    .and_then(|id| w.entities.get(&id))
                {
                    p.y = e.bounding_box.min_y;
                }
                self.living.path = w.entity_path(self, p, 16.0, 16.0);
            }
        } else if let Some(target) = self.living.target {
            if let Some((p, _, alive)) = self.target_position(w, target) {
                if !alive {
                    self.living.target = None;
                } else {
                    let distance = p.distance(self.pos) as f32;
                    let visible = self.can_see_target(w, target);
                    self.attack_target(w, target, distance, visible);
                }
            } else {
                self.living.target = None;
            }
        }
        if self.living.has_attacked
            || self.living.target.is_none()
            || (self.living.path.is_some() && self.rand.next_int(20) != 0)
        {
            if !self.living.has_attacked
                && ((self.living.path.is_none() && self.rand.next_int(80) == 0)
                    || self.living.flee_ticks > 0
                    || self.rand.next_int(80) == 0)
            {
                self.wander(w);
            }
        } else if let Some((mut p, _, _)) =
            self.living.target.and_then(|t| self.target_position(w, t))
        {
            if let Some(e) = self
                .living
                .target
                .and_then(|t| w.target_entity_id(t))
                .and_then(|id| w.entities.get(&id))
            {
                p.y = e.bounding_box.min_y;
            }
            self.living.path = w.entity_path(self, p, 16.0, 16.0);
        }
        let y = floor(self.bounding_box.min_y + 0.5);
        let water = self.in_water;
        let lava = self.handle_lava_movement(w);
        self.pitch = 0.0;
        if self.living.path.is_some() && self.rand.next_int(100) != 0 {
            let size = (self.width * 2.0) as f64;
            let mut point = self
                .living
                .path
                .as_ref()
                .and_then(|p| p.position(self.width));
            while let Some(p) = point {
                if p.square_distance(Vec3D::new(self.pos.x, p.y, self.pos.z)) >= size * size {
                    break;
                }
                if let Some(path) = self.living.path.as_mut() {
                    path.index += 1;
                    if path.index >= path.points.len() {
                        self.living.path = None;
                    }
                }
                point = self
                    .living
                    .path
                    .as_ref()
                    .and_then(|p| p.position(self.width));
            }
            self.living.is_jumping = false;
            if let Some(p) = point {
                let x = p.x - self.pos.x;
                let z = p.z - self.pos.z;
                let dy = p.y - y as f64;
                let yaw =
                    (crate::jmath::java_atan2(z, x) * 180.0 / math::PI_F32 as f64) as f32 - 90.0;
                self.living.move_forward = self.living.move_speed;
                let diff = wrap(yaw - self.yaw).clamp(-30.0, 30.0);
                self.yaw += diff;
                if self.living.has_attacked {
                    if let Some((p, _, _)) =
                        self.living.target.and_then(|t| self.target_position(w, t))
                    {
                        let x = p.x - self.pos.x;
                        let z = p.z - self.pos.z;
                        let old = self.yaw;
                        self.yaw = (crate::jmath::java_atan2(z, x) * 180.0 / math::PI_F32 as f64)
                            as f32
                            - 90.0;
                        let diff = (old - self.yaw + 90.0) * math::PI_F32 / 180.0;
                        self.living.move_strafing =
                            -math::sin(diff) * self.living.move_forward * 1.0;
                        self.living.move_forward = math::cos(diff) * self.living.move_forward * 1.0;
                    }
                }
                if dy > 0.0 {
                    self.living.is_jumping = true;
                }
            }
            if let Some(t) = self.living.target {
                self.face_target(w, t, 30.0, 30.0);
            }
            if self.collided_horizontally && self.living.path.is_none() {
                self.living.is_jumping = true;
            }
            if self.rand.next_float() < 0.8 && (water || lava) {
                self.living.is_jumping = true;
            }
        } else {
            self.living_action_state(w);
            self.living.path = None;
        }
    }
}
pub fn update_rotation(from: f32, to: f32, speed: f32) -> f32 {
    from + wrap(to - from).clamp(-speed, speed)
}
