//! Release 1.0 Blaze and snow golem overrides.
use super::{
    damage::{DamageKind, DamageSource},
    data_watcher::Value,
    living::Target,
    Entity, EntityKind, Vec3D,
};
use crate::{jmath, math, World};
#[derive(Clone, Debug)]
pub struct BlazeState {
    pub hover_offset: f32,
    pub height_timer: i32,
    pub attack_step: i32,
}
impl Default for BlazeState {
    fn default() -> Self {
        Self {
            hover_offset: 0.5,
            height_timer: 0,
            attack_step: 0,
        }
    }
}
impl Entity {
    pub fn villager_texture(&self) -> Option<&'static str> {
        let EntityKind::Villager { profession } = self.kind else {
            return None;
        };
        Some(match profession {
            0 => "/mob/villager/farmer.png",
            1 => "/mob/villager/librarian.png",
            2 => "/mob/villager/priest.png",
            3 => "/mob/villager/smith.png",
            4 => "/mob/villager/butcher.png",
            _ => "/mob/villager/villager.png",
        })
    }

    pub(super) fn blaze_before_living(&mut self, w: &mut World) {
        let EntityKind::Blaze(mut s) = self.kind.clone() else {
            return;
        };
        if !w.multiplayer_world {
            if self.is_wet(w) {
                self.attack_from(w, DamageSource::new(DamageKind::Drown), 1);
            }
            s.height_timer -= 1;
            if s.height_timer <= 0 {
                s.height_timer = 100;
                s.hover_offset = 0.5 + self.rand.next_gaussian() as f32 * 3.0;
            }
            if let Some(t) = self.living.target {
                if let Some((p, eye, _)) = self.target_position(w, t) {
                    if p.y + eye as f64
                        > self.pos.y + self.eye_height() as f64 + s.hover_offset as f64
                    {
                        self.motion.y += (0.3f32 as f64 - self.motion.y) * 0.3f32 as f64;
                    }
                }
            }
        }
        if self.rand.next_int(24) == 0 {
            let volume = 1.0 + self.rand.next_float();
            let pitch = self.rand.next_float() * 0.7 + 0.3;
            w.play_sound("fire.fire", self.pos.add(0.5, 0.5, 0.5), volume, pitch);
        }
        if !self.on_ground && self.motion.y < 0.0 {
            self.motion.y *= 0.6;
        }
        for _ in 0..2 {
            let p = Vec3D::new(
                self.pos.x + (self.rand.next_double() - 0.5) * self.width as f64,
                self.pos.y + self.rand.next_double() * self.height as f64,
                self.pos.z + (self.rand.next_double() - 0.5) * self.width as f64,
            );
            w.spawn_particle("largesmoke", p, Vec3D::default());
        }
        self.kind = EntityKind::Blaze(s);
    }
    pub(super) fn snowman_after_living(&mut self, w: &mut World) {
        if !matches!(self.kind, EntityKind::Snowman) {
            return;
        }
        if self.living.target.is_none() && self.living.path.is_none() && w.rand.next_int(100) == 0 {
            let b = super::AxisAlignedBB::new(
                self.pos.x,
                self.pos.y,
                self.pos.z,
                self.pos.x + 1.0,
                self.pos.y + 1.0,
                self.pos.z + 1.0,
            )
            .expand(16.0, 4.0, 16.0);
            let ids: Vec<_> = w
                .entities_within_aabb(None, b)
                .into_iter()
                .filter(|id| w.entities[id].kind.is_mob())
                .collect();
            if !ids.is_empty() {
                self.living.target = Some(Target::Entity(
                    ids[w.rand.next_int(ids.len() as i32) as usize],
                ));
            }
        }
        let x = math::floor_double(self.pos.x);
        let z = math::floor_double(self.pos.z);
        if w.terrain.biome_source.biome_at(x, z).temperature() > 1.0 {
            self.attack_from(w, DamageSource::new(DamageKind::OnFire), 1);
        }
        for i in 0..4 {
            let x = math::floor_double(self.pos.x + ((i % 2) * 2 - 1) as f64 * 0.25);
            let y = math::floor_double(self.pos.y);
            let z = math::floor_double(self.pos.z + ((i / 2 % 2) * 2 - 1) as f64 * 0.25);
            let below = w.get_block_id(x, y - 1, z);
            if w.get_block_id(x, y, z) == 0
                && w.terrain.biome_source.biome_at(x, z).temperature() < 0.8
                && (below == 18 || crate::block::opaque_cube_lookup(below))
            {
                w.set_block_with_notify(x, y, z, 78);
            }
        }
    }
    pub(super) fn new_mob_attack(&mut self, w: &mut World, t: Target, distance: f32) {
        let Some((p, eye, _)) = self.target_position(w, t) else {
            return;
        };
        let dx = p.x - self.pos.x;
        let dz = p.z - self.pos.z;
        if matches!(self.kind, EntityKind::Snowman) {
            if distance >= 10.0 {
                return;
            }
            if self.living.attack_time == 0 {
                let ball = Self::throwable(w, self, false);
                let dy = p.y + eye as f64 - 1.1f32 as f64 - ball.pos.y;
                let arc = ((dx * dx + dz * dz).sqrt() as f32 * 0.2) as f64;
                let pitch = 1.0 / (self.rand.next_float() * 0.4 + 0.8);
                w.play_sound_at_entity(self, "random.bow", 1.0, pitch);
                let id = ball.entity_id;
                w.entity_joined_world(ball);
                if let Some(mut ball) = w.entities.remove(&id) {
                    ball.set_arrow_heading(dx, dy + arc, dz, 1.6, 12.0);
                    w.entities.insert(id, ball);
                }
                self.living.attack_time = 10;
            }
        } else {
            if self.living.attack_time <= 0 && distance < 2.0 {
                if let Some(id) = w.target_entity_id(t) {
                    if let Some(mut e) = w.entities.remove(&id) {
                        if e.bounding_box.max_y > self.bounding_box.min_y
                            && e.bounding_box.min_y < self.bounding_box.max_y
                        {
                            self.living.attack_time = 20;
                            w.entities.insert(self.entity_id, self.clone());
                            e.attack_from(
                                w,
                                DamageSource::entity(DamageKind::Mob, Some(self.entity_id)),
                                self.potion_attack_damage(6),
                            );
                            w.entities.remove(&self.entity_id);
                        }
                        w.entities.insert(id, e);
                    }
                }
                return;
            }
            if distance >= 30.0 {
                return;
            }
            if self.living.attack_time == 0 {
                let EntityKind::Blaze(mut s) = self.kind.clone() else {
                    return;
                };
                s.attack_step += 1;
                if s.attack_step == 1 {
                    self.living.attack_time = 60;
                    self.data_watcher.update(16, Value::Byte(1)).unwrap();
                } else if s.attack_step <= 4 {
                    self.living.attack_time = 6;
                } else {
                    self.living.attack_time = 100;
                    s.attack_step = 0;
                    self.data_watcher.update(16, Value::Byte(0)).unwrap();
                }
                if s.attack_step > 1 {
                    w.auxiliary_effect(
                        1009,
                        (self.pos.x as i32, self.pos.y as i32, self.pos.z as i32),
                        0,
                    );
                    let target_center = w
                        .target_entity_id(t)
                        .and_then(|id| w.entities.get(&id))
                        .map_or(p.y + 0.9f32 as f64, |e| {
                            e.bounding_box.min_y + (e.height / 2.0) as f64
                        });
                    let dy = target_center - (self.pos.y + (self.height / 2.0) as f64);
                    let spread = (distance.sqrt() * 0.5) as f64;
                    let aim = Vec3D::new(
                        dx + self.rand.next_gaussian() * spread,
                        dy,
                        dz + self.rand.next_gaussian() * spread,
                    );
                    let mut ball = Self::small_fireball_from_owner(w, self, aim);
                    ball.pos.y = self.pos.y + (self.height / 2.0) as f64 + 0.5;
                    w.entity_joined_world(ball);
                }
                self.kind = EntityKind::Blaze(s);
            }
        }
        self.yaw = (jmath::java_atan2(dz, dx) * 180.0 / math::PI_F32 as f64) as f32 - 90.0;
        self.living.has_attacked = true;
    }
}
