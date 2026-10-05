//! Beta 1.8.1 `FoodStats`. Arithmetic and the one-exhaustion-step-per-tick bug
//! deliberately follow the original (regeneration does not add exhaustion).
use crate::nbt::{Compound, Tag};

#[derive(Clone, Debug)]
pub struct FoodStats {
    pub level: i32,
    pub saturation: f32,
    pub exhaustion: f32,
    pub tick_timer: i32,
    pub prev_level: i32,
}
impl Default for FoodStats {
    fn default() -> Self {
        Self { level: 20, saturation: 5.0, exhaustion: 0.0, tick_timer: 0, prev_level: 20 }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FoodAction { None, Heal, Starve }
impl FoodStats {
    pub fn add_food(&mut self, amount: i32, modifier: f32) {
        self.level = amount.wrapping_add(self.level).min(20);
        self.saturation = java_min(self.saturation + amount as f32 * modifier * 2.0, self.level as f32);
    }
    pub fn add_exhaustion(&mut self, amount: f32) {
        self.exhaustion = java_min(self.exhaustion + amount, 40.0);
    }
    pub fn needs_food(&self) -> bool { self.level < 20 }
    pub fn tick(&mut self, difficulty: i32, health: i32) -> FoodAction {
        self.prev_level = self.level;
        if self.exhaustion > 4.0 {
            self.exhaustion -= 4.0;
            if self.saturation > 0.0 {
                self.saturation = java_max(self.saturation - 1.0, 0.0);
            } else if difficulty > 0 {
                self.level = (self.level - 1).max(0);
            }
        }
        if self.level >= 18 && health > 0 && health < 20 {
            self.tick_timer = self.tick_timer.wrapping_add(1);
            if self.tick_timer >= 80 {
                self.tick_timer = 0;
                return FoodAction::Heal;
            }
        } else if self.level <= 0 {
            self.tick_timer = self.tick_timer.wrapping_add(1);
            if self.tick_timer >= 80 {
                self.tick_timer = 0;
                if health > 10 || difficulty >= 3 || health > 1 && difficulty >= 2 {
                    return FoodAction::Starve;
                }
            }
        } else {
            self.tick_timer = 0;
        }
        FoodAction::None
    }
    pub fn read_nbt(&mut self, n: &Compound) {
        if n.has("foodLevel") {
            self.level = n.int("foodLevel");
            self.tick_timer = n.int("foodTickTimer");
            self.saturation = match n.get("foodSaturationLevel") { Some(Tag::Float(v)) => *v, _ => 0.0 };
            self.exhaustion = match n.get("foodExhaustionLevel") { Some(Tag::Float(v)) => *v, _ => 0.0 };
        }
    }
    pub fn write_nbt(&self, n: &mut Compound) {
        n.put("foodLevel", Tag::Int(self.level));
        n.put("foodTickTimer", Tag::Int(self.tick_timer));
        n.put("foodSaturationLevel", Tag::Float(self.saturation));
        n.put("foodExhaustionLevel", Tag::Float(self.exhaustion));
    }
}

/// The four flags are intentionally neither persisted nor inferred from one another.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerCapabilities {
    pub disable_damage: bool,
    pub flying: bool,
    pub allow_flying: bool,
    pub creative: bool,
}

/// Beta 1.8.1 stores integer points in the current level, not a float fraction.
#[derive(Clone, Copy, Debug, Default)]
pub struct Experience {
    pub points: i32,
    pub level: i32,
    pub total: i32,
}
impl Experience {
    pub fn bar_cap(&self) -> i32 { self.level.wrapping_add(1).wrapping_mul(10) }
    pub fn add(&mut self, amount: i32) {
        self.points = self.points.wrapping_add(amount);
        self.total = self.total.wrapping_add(amount);
        while self.points >= self.bar_cap() {
            self.points = self.points.wrapping_sub(self.bar_cap());
            self.level = self.level.wrapping_add(1);
        }
    }
    pub fn read_nbt(&mut self, n: &Compound) {
        self.points = n.int("Xp");
        self.level = n.int("XpLevel");
        self.total = n.int("XpTotal");
    }
    pub fn write_nbt(&self, n: &mut Compound) {
        n.put("Xp", Tag::Int(self.points));
        n.put("XpLevel", Tag::Int(self.level));
        n.put("XpTotal", Tag::Int(self.total));
    }
}

/// Immutable parameters passed to the Beta 1.8.1 new-world flow.
#[derive(Clone, Copy, Debug)]
pub struct WorldSettings { pub seed: i64, pub game_mode: i32, pub map_features: bool }
impl WorldSettings { pub fn new(seed: i64, game_mode: i32, map_features: bool) -> Self { Self { seed, game_mode, map_features } } }

fn java_min(a: f32,b: f32) -> f32 {
    if a.is_nan() { a } else if b.is_nan() { b } else if a == 0.0 && b == 0.0 { f32::from_bits(a.to_bits() | b.to_bits()) } else if a <= b { a } else { b }
}
fn java_max(a: f32,b: f32) -> f32 {
    if a.is_nan() { a } else if b.is_nan() { b } else if a == 0.0 && b == 0.0 { f32::from_bits(a.to_bits() & b.to_bits()) } else if a >= b { a } else { b }
}
impl PlayerCapabilities {
    pub fn set_creative(&mut self, enabled: bool) {
        self.allow_flying=enabled;self.creative=enabled;self.disable_damage=enabled;
        if !enabled { self.flying=false; }
    }
}
