//! Item use and the food state introduced by Beta 1.8.1.
use super::*;
use crate::nbt::{Compound, Tag};

#[derive(Clone, Copy, Debug)]
pub struct ItemUse {
    pub slot: usize,
    pub stack: ItemStack,
    pub remaining: i32,
}
#[derive(Clone, Copy, Debug)]
pub struct FoodEffect {
    pub id: i32,
    pub duration: i32,
    pub amplifier: i32,
}
#[derive(Clone, Debug)]
pub struct FoodStats {
    pub level: i32,
    pub saturation: f32,
    pub exhaustion: f32,
    pub timer: i32,
    pub previous_level: i32,
}
impl Default for FoodStats {
    fn default() -> Self {
        Self {
            level: 20,
            saturation: 5.0,
            exhaustion: 0.0,
            timer: 0,
            previous_level: 20,
        }
    }
}
fn java_min(a: f32, b: f32) -> f32 {
    if a.is_nan() {
        a
    } else if a == 0.0 && b == 0.0 && b.is_sign_negative() {
        b
    } else if a <= b {
        a
    } else {
        b
    }
}
impl FoodStats {
    pub fn add(&mut self, amount: i32, saturation: f32) {
        self.level = amount.wrapping_add(self.level).min(20);
        self.saturation = java_min(
            self.saturation + amount as f32 * saturation * 2.0,
            self.level as f32,
        );
    }
    pub fn exhaust(&mut self, amount: f32) {
        self.exhaustion = java_min(self.exhaustion + amount, 40.0);
    }
    pub fn read(&mut self, n: &Compound) {
        if n.has("foodLevel") {
            self.level = n.int("foodLevel");
            self.timer = n.int("foodTickTimer");
            self.saturation = match n.get("foodSaturationLevel") {
                Some(Tag::Float(v)) => *v,
                _ => 0.0,
            };
            self.exhaustion = match n.get("foodExhaustionLevel") {
                Some(Tag::Float(v)) => *v,
                _ => 0.0,
            };
        }
    }
    pub fn write(&self, n: &mut Compound) {
        n.put("foodLevel", Tag::Int(self.level));
        n.put("foodTickTimer", Tag::Int(self.timer));
        n.put("foodSaturationLevel", Tag::Float(self.saturation));
        n.put("foodExhaustionLevel", Tag::Float(self.exhaustion));
    }
}
/// (food points, saturation multiplier, always edible, effect id/duration/chance).
pub fn food(id: i32) -> Option<(i32, f32, bool, Option<(i32, i32, f32)>)> {
    Some(match id {
        260 => (4, 0.3, false, None),
        282 => (8, 0.6, false, None),
        297 => (5, 0.6, false, None),
        319 | 363 => (3, 0.3, false, None),
        320 | 364 => (8, 0.8, false, None),
        322 => (10, 1.2, true, Some((10, 600, 1.0))),
        349 => (2, 0.3, false, None),
        350 => (5, 0.6, false, None),
        357 => (1, 0.1, false, None),
        360 => (2, 0.3, false, None),
        365 => (2, 0.3, false, Some((17, 600, 0.3))),
        366 => (6, 0.6, false, None),
        367 => (4, 0.1, false, Some((17, 600, 0.8))),
        _ => return None,
    })
}
impl Entity {
    pub fn add_food_exhaustion181(&mut self, amount: f32) {
        if let Some(s) = self.player_state_mut() {
            if !s.invulnerable {
                s.food.exhaust(amount);
            }
        }
    }
    pub fn is_using_item(&self) -> bool {
        self.player_state().is_some_and(|s| s.item_use.is_some())
    }
    pub fn is_blocking(&self) -> bool {
        self.player_state()
            .and_then(|s| s.item_use)
            .is_some_and(|u| registry::use_action(u.stack.id) == registry::ItemUseAction::Block)
    }
    pub fn start_item_use(&mut self, stack: ItemStack) {
        let s = self.player_state_mut().unwrap();
        if s.item_use
            .is_none_or(|u| u.stack.identity != stack.identity)
        {
            s.item_use = Some(ItemUse {
                slot: s.inventory.current_item as usize,
                stack,
                remaining: registry::use_duration(stack.id),
            });
        }
    }
    pub fn clear_item_use(&mut self) {
        self.player_state_mut().unwrap().item_use = None;
    }
    pub fn stop_item_use(&mut self, w: &mut World) {
        let Some(u) = self.player_state().and_then(|s| s.item_use) else {
            return;
        };
        if u.stack.id == 261
            && self
                .player_state()
                .unwrap()
                .inventory
                .main
                .iter()
                .any(|s| s.is_some_and(|s| s.id == 262))
        {
            if let Some(charge) = registry::bow_charge(u.remaining) {
                let mut arrow = Entity::arrow_from_owner_speed(w, self, charge * 2.0);
                if let EntityKind::Arrow(s) = &mut arrow.kind {
                    s.critical = charge == 1.0;
                }
                let pitch = 1.0 / (w.item_random.next_float() * 0.4 + 1.2) + charge * 0.5;
                w.play_sound_at_entity(self, "random.bow", 1.0, pitch);
                self.player_state_mut().unwrap().inventory.consume(262);
                w.entity_joined_world(arrow);
            }
        }
        self.clear_item_use();
    }
    pub fn tick_item_use(&mut self, w: &mut World) {
        let Some(u) = self.player_state().and_then(|s| s.item_use) else {
            return;
        };
        if self
            .held_item()
            .is_none_or(|v| v.identity != u.stack.identity)
        {
            self.clear_item_use();
            return;
        }
        self.player_state_mut()
            .unwrap()
            .item_use
            .as_mut()
            .unwrap()
            .stack = self.held_item().unwrap();
        if u.remaining <= 25 && u.remaining % 4 == 0 {
            self.item_use_effects(w, u.stack, 5);
        }
        self.player_state_mut()
            .unwrap()
            .item_use
            .as_mut()
            .unwrap()
            .remaining -= 1;
        if u.remaining == 1 {
            self.finish_item_use(w);
        }
    }
    pub fn item_use_effects(&mut self, w: &mut World, stack: ItemStack, count: i32) {
        if registry::use_action(stack.id) != registry::ItemUseAction::Eat {
            return;
        }
        for _ in 0..count {
            let mut velocity = Vec3D::helper(
                (self.rand.next_float() as f64 - 0.5) * 0.1,
                w.math_random.next_double() * 0.1 + 0.1,
                0.0,
            );
            velocity.rotate_x(-self.pitch * math::PI_F32 / 180.0);
            velocity.rotate_y(-self.yaw * math::PI_F32 / 180.0);
            let mut position = Vec3D::helper(
                (self.rand.next_float() as f64 - 0.5) * 0.3,
                -self.rand.next_float() as f64 * 0.6 - 0.3,
                0.6,
            );
            position.rotate_x(-self.pitch * math::PI_F32 / 180.0);
            position.rotate_y(-self.yaw * math::PI_F32 / 180.0);
            position = position.add(
                self.pos.x,
                self.pos.y + self.eye_height() as f64,
                self.pos.z,
            );
            w.spawn_particle(
                &format!("iconcrack_{}", stack.id),
                position,
                velocity.add(0.0, 0.05, 0.0),
            );
        }
        let volume = 0.5 + 0.5 * self.rand.next_int(2) as f32;
        let pitch = (self.rand.next_float() - self.rand.next_float()) * 0.2 + 1.0;
        w.play_sound_at_entity(self, "mob.eat", volume, pitch);
    }
    pub fn finish_item_use(&mut self, w: &mut World) {
        let Some(u) = self.player_state().and_then(|s| s.item_use) else {
            return;
        };
        self.item_use_effects(w, u.stack, 16);
        if let Some((amount, saturation, _, effect)) = food(u.stack.id) {
            self.player_state_mut()
                .unwrap()
                .food
                .add(amount, saturation);
            if let Some((id, duration, chance)) = effect {
                if w.rand.next_float() < chance {
                    let effects = &mut self.player_state_mut().unwrap().food_effects;
                    if let Some(e) = effects.iter_mut().find(|e| e.id == id) {
                        e.duration = duration;
                        e.amplifier = 0;
                    } else {
                        effects.push(FoodEffect {
                            id,
                            duration,
                            amplifier: 0,
                        });
                    }
                }
            }
            let mut stack = u.stack;
            stack.count -= 1;
            if stack.id == 282 {
                stack = ItemStack::new(281, 1, 0);
            }
            let s = self.player_state_mut().unwrap();
            s.inventory.main[s.inventory.current_item as usize] =
                if stack.count == 0 { None } else { Some(stack) };
        }
        self.clear_item_use();
    }
    pub fn tick_food(&mut self, w: &mut World) {
        let health = self.living.health;
        let s = &mut self.player_state_mut().unwrap().food;
        s.previous_level = s.level;
        if s.exhaustion > 4.0 {
            s.exhaustion -= 4.0;
            if s.saturation > 0.0 {
                s.saturation = (s.saturation - 1.0).max(0.0);
            } else if w.difficulty > 0 {
                s.level = (s.level - 1).max(0);
            }
        }
        if s.level >= 18 && health > 0 && health < 20 {
            s.timer += 1;
            if s.timer >= 80 {
                s.timer = 0;
                self.heal(1);
            }
        } else if s.level <= 0 {
            s.timer += 1;
            if s.timer >= 80 {
                s.timer = 0;
                if health > 10 || w.difficulty >= 3 || health > 1 && w.difficulty >= 2 {
                    self.attack_source181(w, None, 1, true, 0.0);
                }
            }
        } else {
            s.timer = 0;
        }
    }
    pub fn tick_food_effects(&mut self, w: &mut World) {
        let effects = self.player_state().unwrap().food_effects.clone();
        for e in effects {
            if e.duration > 0 {
                if e.id == 17 && !self.player_state().unwrap().invulnerable {
                    self.player_state_mut()
                        .unwrap()
                        .food
                        .exhaust(0.025 * (e.amplifier + 1) as f32);
                }
                if e.id == 10
                    && (25i32.wrapping_shr(e.amplifier as u32) <= 0
                        || e.duration % 25i32.wrapping_shr(e.amplifier as u32) == 0)
                    && self.living.health < 20
                {
                    self.heal(1);
                }
            }
        }
        let s = self.player_state_mut().unwrap();
        for e in &mut s.food_effects {
            if e.duration > 0 {
                e.duration -= 1;
            }
        }
        s.food_effects.retain(|e| e.duration > 0);
        let _ = w;
    }
}
