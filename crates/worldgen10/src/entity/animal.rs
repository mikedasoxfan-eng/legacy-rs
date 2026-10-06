//! EntityAnimal: watched age, wheat attraction, love timer and exact mating sequence.
use super::{living::Target, Entity, EntityKind, Vec3D};
use crate::{math, World};
impl EntityKind {
    pub fn is_animal(&self) -> bool {
        matches!(
            self,
            Self::Pig { .. }
                | Self::Sheep { .. }
                | Self::Cow
                | Self::Mooshroom
                | Self::Chicken { .. }
                | Self::Wolf(..)
        )
    }
}
impl Entity {
    pub(super) fn animal_update(&mut self, w: &mut World) {
        if !self.kind.is_animal() {
            return;
        }
        let age = self.living.growing_age;
        if age < 0 {
            self.set_growing_age(age + 1);
        } else if age > 0 {
            self.set_growing_age(age - 1);
        }
        if self.living.in_love > 0 {
            self.living.in_love -= 1;
            if self.living.in_love % 10 == 0 {
                self.animal_hearts(w, 1);
            }
        } else {
            self.living.breeding_ticks = 0;
        }
    }
    fn animal_hearts(&mut self, w: &mut World, count: i32) {
        for _ in 0..count {
            let v = Vec3D::new(
                self.rand.next_gaussian() * 0.02,
                self.rand.next_gaussian() * 0.02,
                self.rand.next_gaussian() * 0.02,
            );
            let p = Vec3D::new(
                self.pos.x + (self.rand.next_float() * self.width * 2.0) as f64 - self.width as f64,
                self.pos.y + 0.5 + (self.rand.next_float() * self.height) as f64,
                self.pos.z + (self.rand.next_float() * self.width * 2.0) as f64 - self.width as f64,
            );
            w.spawn_particle("heart", p, v);
        }
    }
    pub(super) fn feed_animal(&mut self, w: &mut World, player: &mut Entity) -> bool {
        if !self.kind.is_animal()
            || self.living.growing_age != 0
            || !player.held_item().is_some_and(|s| s.id == 296)
        {
            return false;
        }
        let inv = &mut player.player_state_mut().unwrap().inventory;
        let slot = inv.current_item as usize;
        if let Some(item) = &mut inv.main[slot] {
            item.count -= 1;
            if item.count <= 0 {
                inv.main[slot] = None;
            }
        }
        self.living.in_love = 600;
        self.living.target = None;
        self.animal_hearts(w, 7);
        true
    }
    pub(super) fn animal_find_target(&self, w: &World) -> Option<Target> {
        if self.living.flee_ticks > 0 {
            return None;
        }
        // Java uses addCoord, expanding toward positive axes only.
        for id in w.entities_within_aabb(None, self.bounding_box.add_coord(8.0, 8.0, 8.0)) {
            let e = &w.entities[&id];
            if self.living.in_love > 0 {
                if id != self.entity_id && self.animal_class_accepts(&e.kind) && e.living.in_love > 0
                {
                    return Some(Target::Entity(id));
                }
            } else if self.living.growing_age == 0 {
                if e.player_state().is_some() && e.held_item().is_some_and(|s| s.id == 296) {
                    return Some(Target::Entity(id));
                }
            } else if self.living.growing_age > 0
                && id != self.entity_id
                && self.animal_class_accepts(&e.kind)
                && e.living.growing_age < 0
            {
                return Some(Target::Entity(id));
            }
        }
        None
    }
    fn animal_class_accepts(&self, other: &EntityKind) -> bool {
        // Class.isAssignableFrom: MushroomCow extends Cow. A Cow search includes
        // mushroom cows, while a MushroomCow search excludes ordinary cows.
        self.kind.name() == other.name()
            || matches!((&self.kind, other), (EntityKind::Cow, EntityKind::Mooshroom))
    }
    pub(super) fn animal_attack_target(&mut self, w: &mut World, t: Target, distance: f32) {
        let Some(id) = w.target_entity_id(t) else {
            return;
        };
        let Some(mut other) = w.entities.remove(&id) else {
            return;
        };
        if other.player_state().is_some() {
            if distance < 3.0 {
                self.yaw =
                    (crate::jmath::java_atan2(other.pos.z - self.pos.z, other.pos.x - self.pos.x)
                        * 180.0
                        / math::PI_F32 as f64) as f32
                        - 90.0;
                self.living.has_attacked = true;
            }
            if !other.held_item().is_some_and(|s| s.id == 296) {
                self.living.target = None;
            }
        } else if other.kind.is_animal() {
            if self.living.growing_age > 0 && other.living.growing_age < 0 {
                if (distance as f64) < 2.5 {
                    self.living.has_attacked = true;
                }
            } else if self.living.in_love > 0 && other.living.in_love > 0 {
                if other.living.target.is_none() {
                    other.living.target = Some(Target::Entity(self.entity_id));
                }
                if other.living.target == Some(Target::Entity(self.entity_id))
                    && (distance as f64) < 3.5
                {
                    other.living.in_love += 1;
                    self.living.in_love += 1;
                    self.living.breeding_ticks += 1;
                    if self.living.breeding_ticks % 4 == 0 {
                        let p = Vec3D::new(
                            self.pos.x + (self.rand.next_float() * self.width * 2.0) as f64
                                - self.width as f64,
                            self.pos.y + 0.5 + (self.rand.next_float() * self.height) as f64,
                            self.pos.z + (self.rand.next_float() * self.width * 2.0) as f64
                                - self.width as f64,
                        );
                        w.spawn_particle("heart", p, Vec3D::default());
                    }
                    if self.living.breeding_ticks == 60 {
                        if let Some(mut child) = Self::living(w, self.kind.name()) {
                            if let EntityKind::Sheep { fleece_color, .. } = &mut child.kind {
                                let parent = if self.rand.next_bool() {
                                    &self.kind
                                } else {
                                    &other.kind
                                };
                                if let EntityKind::Sheep {
                                    fleece_color: color,
                                    ..
                                } = parent
                                {
                                    *fleece_color = *color;
                                }
                            }
                            self.set_growing_age(6000);
                            other.set_growing_age(6000);
                            self.living.in_love = 0;
                            self.living.breeding_ticks = 0;
                            self.living.target = None;
                            other.living.target = None;
                            other.living.breeding_ticks = 0;
                            other.living.in_love = 0;
                            child.set_growing_age(-24000);
                            child.set_location_and_angles(
                                self.pos.x, self.pos.y, self.pos.z, self.yaw, self.pitch,
                            );
                            self.animal_hearts(w, 7);
                            w.entity_joined_world(child);
                        }
                    }
                } else {
                    self.living.breeding_ticks = 0;
                }
            } else {
                self.living.breeding_ticks = 0;
            }
        }
        w.entities.insert(id, other);
    }
}
