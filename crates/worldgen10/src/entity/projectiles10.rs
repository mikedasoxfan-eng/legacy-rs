//! Release 1.0 projectiles layered on the shared EntityThrowable/EntityFireball loops.
use super::{
    damage::{DamageKind, DamageSource},
    projectile::{ArrowState, FireballState},
    Entity, EntityKind, MovingObjectPosition as Hit, Vec3D,
};
use crate::{jmath, math, World};
#[derive(Clone, Debug, Default)]
pub struct EyeState {
    pub target: Vec3D,
    pub lifetime: i32,
    pub drops: bool,
    pub shake: i32,
}
impl Entity {
    pub fn pearl_at(w: &mut World, pos: Vec3D) -> Self {
        let mut e = Self::pearl_base(w);
        e.set_position(pos.x, pos.y, pos.z);
        e.y_offset = 0.0;
        e
    }
    pub fn potion_base(w: &mut World) -> Self {
        let mut e = Self::base(w, EntityKind::Potion(Default::default(), 0));
        e.width = 0.25;
        e.height = 0.25;
        e
    }
    pub fn potion_at(w: &mut World, pos: Vec3D, metadata: i32) -> Self {
        let mut e = Self::potion_base(w);
        e.kind = EntityKind::Potion(Default::default(), metadata);
        e.set_position(pos.x, pos.y, pos.z);
        e.y_offset = 0.0;
        e
    }

    pub fn pearl_base(w: &mut World) -> Self {
        let mut e = Self::base(w, EntityKind::EnderPearl(Default::default()));
        e.width = 0.25;
        e.height = 0.25;
        e
    }
    pub fn pearl(w: &mut World, owner: &Entity) -> Self {
        let mut e = Self::throwable(w, owner, false);
        let EntityKind::Snowball(s) = e.kind else {
            unreachable!()
        };
        e.kind = EntityKind::EnderPearl(s);
        e
    }
    pub fn potion_projectile(w: &mut World, owner: &Entity, metadata: i32) -> Self {
        let mut e = Self::base(
            w,
            EntityKind::Potion(
                ArrowState {
                    owner: Some(owner.entity_id),
                    ..Default::default()
                },
                metadata,
            ),
        );
        e.width = 0.25;
        e.height = 0.25;
        e.set_location_and_angles(
            owner.pos.x,
            owner.pos.y + owner.eye_height() as f64,
            owner.pos.z,
            owner.yaw,
            owner.pitch,
        );
        e.pos.x -= (math::cos(e.yaw / 180.0 * math::PI_F32) * 0.16) as f64;
        e.pos.y -= 0.1f32 as f64;
        e.pos.z -= (math::sin(e.yaw / 180.0 * math::PI_F32) * 0.16) as f64;
        e.set_position(e.pos.x, e.pos.y, e.pos.z);
        e.y_offset = 0.0;
        e.set_arrow_heading(
            (-math::sin(e.yaw / 180.0 * math::PI_F32)
                * math::cos(e.pitch / 180.0 * math::PI_F32)
                * 0.4) as f64,
            (-math::sin((e.pitch - 20.0) / 180.0 * math::PI_F32) * 0.4) as f64,
            (math::cos(e.yaw / 180.0 * math::PI_F32)
                * math::cos(e.pitch / 180.0 * math::PI_F32)
                * 0.4) as f64,
            0.5,
            1.0,
        );
        e
    }
    pub(super) fn store_throwable_state(&mut self, s: ArrowState) {
        self.kind = match self.kind {
            EntityKind::Egg(..) => EntityKind::Egg(s),
            EntityKind::EnderPearl(..) => EntityKind::EnderPearl(s),
            EntityKind::Potion(_, metadata) => EntityKind::Potion(s, metadata),
            _ => EntityKind::Snowball(s),
        };
    }
    pub(super) fn splash_potion(
        &mut self,
        w: &mut World,
        hit: &Hit,
        owner: Option<i32>,
        metadata: i32,
    ) {
        if w.multiplayer_world {
            return;
        }
        let effects = super::potion::effects_for_metadata(metadata);
        if !effects.is_empty() {
            for id in w.entities_within_aabb(None, self.bounding_box.expand(4.0, 2.0, 4.0)) {
                let Some(mut e) = w.entities.remove(&id) else {
                    continue;
                };
                let distance = self.pos.square_distance(e.pos);
                if e.kind.is_living() && distance < 16.0 {
                    let strength = if matches!(hit,Hit::Entity {id:target,..} if *target==id) {
                        1.0
                    } else {
                        1.0 - distance.sqrt() / 4.0
                    };
                    for effect in &effects {
                        if matches!(effect.id, super::potion::HEAL | super::potion::HARM) {
                            e.apply_instant_potion(w, effect.id, owner, effect.amplifier, strength);
                        } else {
                            let duration = (strength * effect.duration as f64 + 0.5) as i32;
                            if duration > 20 {
                                e.add_potion_effect(super::potion::PotionEffect::new(
                                    effect.id,
                                    duration,
                                    effect.amplifier,
                                ));
                            }
                        }
                    }
                }
                w.entities.insert(id, e);
            }
        }
        w.auxiliary_effect(
            2002,
            (
                super::player::java_round_d(self.pos.x),
                super::player::java_round_d(self.pos.y),
                super::player::java_round_d(self.pos.z),
            ),
            metadata,
        );
        self.dead = true;
    }
    pub fn ender_eye_base(w: &mut World) -> Self {
        let mut e = Self::base(w, EntityKind::EnderEye(Default::default()));
        e.width = 0.25;
        e.height = 0.25;
        e
    }
    pub fn ender_eye(w: &mut World, pos: Vec3D) -> Self {
        let mut e = Self::ender_eye_base(w);
        e.set_position(pos.x, pos.y, pos.z);
        e
    }
    pub fn set_eye_target(&mut self, x: f64, y: i32, z: f64) {
        let dx = x - self.pos.x;
        let dz = z - self.pos.z;
        let distance = (dx * dx + dz * dz).sqrt() as f32;
        if let EntityKind::EnderEye(s) = &mut self.kind {
            s.target = if distance > 12.0 {
                Vec3D::new(
                    self.pos.x + dx / distance as f64 * 12.0,
                    self.pos.y + 8.0,
                    self.pos.z + dz / distance as f64 * 12.0,
                )
            } else {
                Vec3D::new(x, y as f64, z)
            };
            s.lifetime = 0;
            s.drops = self.rand.next_int(5) > 0;
        }
    }
    pub(super) fn update_ender_eye(&mut self, w: &mut World) {
        self.last_tick_pos = self.pos;
        self.on_entity_update(w);
        let EntityKind::EnderEye(mut s) = self.kind.clone() else {
            return;
        };
        self.pos = self.pos.add(self.motion.x, self.motion.y, self.motion.z);
        let horizontal =
            (self.motion.x * self.motion.x + self.motion.z * self.motion.z).sqrt() as f32;
        self.projectile_angles();
        while self.pitch - self.prev_pitch < -180.0 {
            self.prev_pitch -= 360.0;
        }
        while self.pitch - self.prev_pitch >= 180.0 {
            self.prev_pitch += 360.0;
        }
        while self.yaw - self.prev_yaw < -180.0 {
            self.prev_yaw -= 360.0;
        }
        while self.yaw - self.prev_yaw >= 180.0 {
            self.prev_yaw += 360.0;
        }
        self.pitch = self.prev_pitch + (self.pitch - self.prev_pitch) * 0.2;
        self.yaw = self.prev_yaw + (self.yaw - self.prev_yaw) * 0.2;
        if !w.multiplayer_world {
            let dx = s.target.x - self.pos.x;
            let dz = s.target.z - self.pos.z;
            let distance = (dx * dx + dz * dz).sqrt() as f32;
            let angle = jmath::java_atan2(dz, dx) as f32;
            let mut speed = horizontal as f64 + (distance - horizontal) as f64 * 0.0025;
            if distance < 1.0 {
                speed *= 0.8;
                self.motion.y *= 0.8;
            }
            self.motion.x = jmath::java_cos(angle as f64) * speed;
            self.motion.z = jmath::java_sin(angle as f64) * speed;
            self.motion.y += (if self.pos.y < s.target.y {
                1.0 - self.motion.y
            } else {
                -1.0 - self.motion.y
            }) * 0.015f32 as f64;
        }
        if self.in_water {
            for _ in 0..4 {
                w.spawn_particle(
                    "bubble",
                    self.pos.add(
                        -self.motion.x * 0.25,
                        -self.motion.y * 0.25,
                        -self.motion.z * 0.25,
                    ),
                    self.motion,
                );
            }
        } else {
            let p = Vec3D::new(
                self.pos.x - self.motion.x * 0.25 + self.rand.next_double() * 0.6 - 0.3,
                self.pos.y - self.motion.y * 0.25 - 0.5,
                self.pos.z - self.motion.z * 0.25 + self.rand.next_double() * 0.6 - 0.3,
            );
            w.spawn_particle("portal", p, self.motion);
        }
        if !w.multiplayer_world {
            self.set_position(self.pos.x, self.pos.y, self.pos.z);
            s.lifetime += 1;
            if s.lifetime > 80 {
                self.dead = true;
                if s.drops {
                    let item = Self::item(
                        w,
                        self.pos.x,
                        self.pos.y,
                        self.pos.z,
                        crate::world::tile::ItemStack::new(381, 1, 0),
                    );
                    w.entity_joined_world(item);
                } else {
                    w.auxiliary_effect(
                        2003,
                        (
                            super::player::java_round_d(self.pos.x),
                            super::player::java_round_d(self.pos.y),
                            super::player::java_round_d(self.pos.z),
                        ),
                        0,
                    );
                }
            }
        }
        self.kind = EntityKind::EnderEye(s);
    }
    pub fn fireball_base(w: &mut World, small: bool) -> Self {
        let mut e = Self::base(
            w,
            if small {
                EntityKind::SmallFireball(Default::default())
            } else {
                EntityKind::Fireball(Default::default())
            },
        );
        e.width = if small { 5.0 / 16.0 } else { 1.0 };
        e.height = e.width;
        e
    }
    pub fn small_fireball_from_owner(w: &mut World, owner: &Entity, aim: Vec3D) -> Self {
        let mut e = Self::fireball_from_owner(w, owner, aim);
        let EntityKind::Fireball(s) = e.kind else {
            unreachable!()
        };
        e.kind = EntityKind::SmallFireball(s);
        e.width = 5.0 / 16.0;
        e.height = e.width;
        e
    }
    pub(super) fn update_small_fireball(&mut self, w: &mut World) {
        self.update_fireball(w);
    }
    pub(super) fn small_fireball_impact(&mut self, w: &mut World, hit: Hit, s: &FireballState) {
        if w.multiplayer_world {
            return;
        }
        match hit {
            Hit::Entity { id, .. } => {
                if let Some(mut target) = w.entities.remove(&id) {
                    if !target.immune_to_fire
                        && target.attack_from(
                            w,
                            DamageSource::indirect(
                                DamageKind::Fireball,
                                self.entity_id,
                                s.arrow.owner,
                            ),
                            5,
                        )
                    {
                        target.set_fire_seconds(5);
                    }
                    w.entities.insert(id, target);
                }
            }
            Hit::Block { x, y, z, side, .. } => {
                let (dx, dy, dz) = [
                    (0, -1, 0),
                    (0, 1, 0),
                    (0, 0, -1),
                    (0, 0, 1),
                    (-1, 0, 0),
                    (1, 0, 0),
                ][side as usize];
                if w.get_block_id(x + dx, y + dy, z + dz) == 0 {
                    w.set_block_with_notify(x + dx, y + dy, z + dz, 51);
                }
            }
        }
        self.dead = true;
    }
}
