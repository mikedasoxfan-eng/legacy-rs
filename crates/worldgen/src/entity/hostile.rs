use super::{living::Target, Entity, EntityKind};
use crate::{
    math::{floor_double as floor, PI_F32 as PI},
    World,
};
impl Entity {
    pub fn brightness(&self, w: &mut World) -> f32 {
        let b = self.bounding_box;
        if w.check_chunks_exist(
            floor(b.min_x),
            floor(b.min_y),
            floor(b.min_z),
            floor(b.max_x),
            floor(b.max_y),
            floor(b.max_z),
        ) {
            w.get_light_brightness(
                floor(self.pos.x),
                floor(self.pos.y - self.y_offset as f64 + (b.max_y - b.min_y) * 0.66),
                floor(self.pos.z),
            )
        } else {
            0.0
        }
    }
    pub(super) fn update_hostile_living(&mut self, w: &mut World) {
        if let EntityKind::Creeper {
            ignition,
            prev_ignition,
            ..
        } = &mut self.kind
        {
            *prev_ignition = *ignition;
        }
        if let EntityKind::PigZombie { sound_delay, .. } = &mut self.kind {
            self.living.move_speed = if self.living.target.is_some() {
                0.95
            } else {
                0.5
            };
            if *sound_delay > 0 {
                *sound_delay -= 1;
                if *sound_delay == 0 {
                    let pitch =
                        ((self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0) * 1.8;
                    w.play_sound(
                        "mob.zombiepig.zpigangry",
                        self.pos.add(0.0, -(self.y_offset as f64), 0.0),
                        2.0,
                        pitch,
                    );
                }
            }
        }
        self.update_living(w);
        self.wolf_update(w);
        if self.kind.is_mob() && w.difficulty == 0 {
            self.dead = true;
        }
        if let EntityKind::Creeper {
            ignition, state, ..
        } = &mut self.kind
        {
            if self.living.target.is_none() && *ignition > 0 {
                *state = -1;
                *ignition -= 1;
                if *ignition < 0 {
                    *ignition = 0;
                }
            }
        }
    }
    pub(super) fn mob_before_living(&mut self, w: &mut World) {
        if matches!(
            self.kind,
            EntityKind::Zombie | EntityKind::Skeleton | EntityKind::PigZombie { .. }
        ) && w.skylight_subtracted < 4
        {
            let light = self.brightness(w);
            if light > 0.5
                && w.can_block_see_the_sky(floor(self.pos.x), floor(self.pos.y), floor(self.pos.z))
                && self.rand.next_float() * 30.0 < (light - 0.4) * 2.0
            {
                self.fire = 300;
            }
        }
        if self.kind.is_mob() && self.brightness(w) > 0.5 {
            self.living.entity_age += 2;
        }
    }
    pub(super) fn find_attack_target(&mut self, w: &mut World) -> Option<Target> {
        if let EntityKind::Wolf(s) = &self.kind {
            return if s.flags & 2 != 0 {
                w.closest_player(self.pos, 16.0).map(Target::Player)
            } else {
                None
            };
        }
        if !self.kind.is_mob() {
            return None;
        }
        if matches!(self.kind, EntityKind::PigZombie { anger: 0, .. }) {
            return None;
        }
        let spider = matches!(self.kind, EntityKind::Spider);
        if spider && self.brightness(w) >= 0.5 {
            return None;
        }
        let t = w.closest_player(self.pos, 16.0).map(Target::Player);
        if spider {
            t
        } else {
            t.filter(|t| self.can_see_target(w, *t))
        }
    }
    pub fn attack_target(&mut self, w: &mut World, target: Target, distance: f32, visible: bool) {
        if let EntityKind::Creeper {
            ignition, state, ..
        } = &mut self.kind
        {
            if visible && ((*state <= 0 && distance < 3.0) || (*state > 0 && distance < 7.0)) {
                if *ignition == 0 {
                    w.play_sound(
                        "random.fuse",
                        self.pos.add(0.0, -(self.y_offset as f64), 0.0),
                        1.0,
                        0.5,
                    );
                }
                *state = 1;
                *ignition += 1;
                let explode = *ignition >= 30;
                self.living.has_attacked = true;
                if explode {
                    let powered = matches!(self.kind, EntityKind::Creeper { powered: true, .. });
                    w.entities.insert(self.entity_id, self.clone());
                    w.create_explosion(
                        Some(self.entity_id),
                        self.pos,
                        if powered { 6.0 } else { 3.0 },
                    );
                    w.entities.remove(&self.entity_id);
                    self.dead = true;
                }
            } else if visible || *ignition > 0 {
                *state = -1;
                *ignition = (*ignition - 1).max(0);
            }
            return;
        }
        if !visible {
            return;
        }
        if matches!(self.kind, EntityKind::Skeleton) {
            if distance < 10.0 {
                if let Some((p, eye, _)) = self.target_position(w, target) {
                    let dx = p.x - self.pos.x;
                    let dz = p.z - self.pos.z;
                    if self.living.attack_time == 0 {
                        let mut arrow = Self::arrow_from_owner(w, self);
                        arrow.pos.y += 1.4f32 as f64;
                        let dy = p.y + eye as f64 - 0.2f32 as f64 - arrow.pos.y;
                        let arc = ((dx * dx + dz * dz).sqrt() as f32 * 0.2) as f64;
                        let pitch = 1.0 / (self.rand.next_float() * 0.4 + 0.8);
                        w.play_sound_at_entity(self, "random.bow", 1.0, pitch);
                        let id = arrow.entity_id;
                        w.entity_joined_world(arrow);
                        if let Some(mut arrow) = w.entities.remove(&id) {
                            arrow.set_arrow_heading(dx, dy + arc, dz, 0.6, 12.0);
                            w.entities.insert(id, arrow);
                        }
                        self.living.attack_time = 30;
                    }
                    self.yaw = (crate::jmath::java_atan2(dz, dx) * 180.0 / PI as f64) as f32 - 90.0;
                    self.living.has_attacked = true;
                }
            }
            return;
        }
        if matches!(self.kind, EntityKind::Wolf(..)) {
            self.wolf_attack_target(w, target, distance);
            return;
        }
        if matches!(self.kind, EntityKind::Spider) {
            if self.brightness(w) > 0.5 && self.rand.next_int(100) == 0 {
                self.living.target = None;
                return;
            }
            if distance > 2.0 && distance < 6.0 && self.rand.next_int(10) == 0 {
                if self.on_ground {
                    if let Some((p, _, _)) = self.target_position(w, target) {
                        let dx = p.x - self.pos.x;
                        let dz = p.z - self.pos.z;
                        let n = (dx * dx + dz * dz).sqrt() as f32 as f64;
                        self.motion.x =
                            dx / n * 0.5 * 0.8f32 as f64 + self.motion.x * 0.2f32 as f64;
                        self.motion.z =
                            dz / n * 0.5 * 0.8f32 as f64 + self.motion.z * 0.2f32 as f64;
                        self.motion.y = 0.4f32 as f64;
                    }
                }
                return;
            }
        }
        if self.kind.is_mob() && self.living.attack_time <= 0 && distance < 2.0 {
            if let Some((p, _, _)) = self.target_position(w, target) {
                let (min, max) = match target {
                    Target::Player(_) => w
                        .target_entity_id(target)
                        .and_then(|id| w.entities.get(&id))
                        .map_or((p.y, p.y + 1.8f32 as f64), |e| {
                            (e.bounding_box.min_y, e.bounding_box.max_y)
                        }),
                    Target::Entity(id) => w
                        .entities
                        .get(&id)
                        .map(|e| (e.bounding_box.min_y, e.bounding_box.max_y))
                        .unwrap_or((0.0, 0.0)),
                };
                if max > self.bounding_box.min_y && min < self.bounding_box.max_y {
                    self.living.attack_time = 20;
                    let damage = match self.kind {
                        EntityKind::Zombie | EntityKind::PigZombie { .. } => 5,
                        EntityKind::Giant => 50,
                        _ => 2,
                    };
                    if let Some(id) = w.target_entity_id(target) {
                        if let Some(mut e) = w.entities.remove(&id) {
                            w.entities.insert(self.entity_id, self.clone());
                            e.attack(w, Some(self.entity_id), damage);
                            w.entities.remove(&self.entity_id);
                            w.entities.insert(id, e);
                        }
                    }
                }
            }
        }
    }
    pub fn become_angry_at(&mut self, target: Target) {
        self.living.target = Some(target);
        if let EntityKind::PigZombie { anger, sound_delay } = &mut self.kind {
            *anger = 400 + self.rand.next_int(400);
            *sound_delay = self.rand.next_int(40);
        }
    }
}
