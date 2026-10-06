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
        Self { level: 20, saturation: 5.0, exhaustion: 0.0, tick_timer: 0, prev_level: 20, }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FoodAction { None, Heal, Starve, }
impl FoodStats {
    pub fn add_food(&mut self, amount: i32, modifier: f32) {
        self.level = amount.wrapping_add(self.level).min(20);
        self.saturation = java_min(self.saturation + amount as f32 * modifier * 2.0, self.level as f32,);
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
            self.saturation = match n.get("foodSaturationLevel") { Some(Tag::Float(v)) => *v, _ => 0.0, };
            self.exhaustion = match n.get("foodExhaustionLevel") { Some(Tag::Float(v)) => *v, _ => 0.0, };
        }
    }
    pub fn write_nbt(&self, n: &mut Compound) {
        n.put("foodLevel", Tag::Int(self.level));
        n.put("foodTickTimer", Tag::Int(self.tick_timer));
        n.put("foodSaturationLevel", Tag::Float(self.saturation));
        n.put("foodExhaustionLevel", Tag::Float(self.exhaustion));
    }
}

/// EntityPlayer abilities, including the original serialization behavior.
#[derive(Clone, Copy, Debug, Default)]
pub struct PlayerCapabilities {
    pub disable_damage: bool,
    pub flying: bool,
    pub allow_flying: bool,
    pub creative: bool,
}

/// EntityPlayer.currentXP is a float fraction in Release 1.0.
#[derive(Clone, Copy, Debug, Default)]
pub struct Experience {
    pub points: f32,
    pub level: i32,
    pub total: i32,
}
impl Experience {
    pub fn bar_cap(&self) -> i32 {
        7i32.wrapping_add( self.level.wrapping_mul(7) >> 1) }
    pub fn add(&mut self, amount: i32) {
        self.points += amount as f32 / self.bar_cap() as f32;
        self.total = self.total.wrapping_add(amount);
        while self.points >= 1.0 {
            self.points -= 1.0;
            self.level = self.level.wrapping_add(1);
        }
    }
    pub fn spend_levels(&mut self, amount: i32) {
        self.level = self.level.wrapping_sub(amount).max(0);
    }
    pub fn read_nbt(&mut self, n: &Compound) {
        self.points = match n.get("XpP") {
            Some(Tag::Float(v)) => *v,
            _ => 0.0,
        };
        self.level = n.int("XpLevel");
        self.total = n.int("XpTotal");
    }
    pub fn write_nbt(&self, n: &mut Compound) {
        n.put("XpP", Tag::Float(self.points));
        n.put("XpLevel", Tag::Int(self.level));
        n.put("XpTotal", Tag::Int(self.total));
    }
}
#[derive(Clone, Copy, Debug)]
pub struct WorldSettings { pub seed: i64, pub game_mode: i32, pub map_features: bool,
    pub hardcore: bool, }
impl WorldSettings { pub fn new(seed: i64, game_mode: i32, map_features: bool) -> Self { Self::with_hardcore(seed, game_mode, map_features, false)
    }
    pub fn with_hardcore(seed: i64, game_mode: i32, map_features: bool, hardcore: bool) -> Self {
        Self { seed, game_mode, map_features,
            hardcore, } } }

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

impl PlayerCapabilities {
    pub fn write_nbt(&self, n: &mut Compound) {
        let mut a = Compound::new();
        a.put("invulnerable", Tag::Byte(self.disable_damage as i8));
        // Preserve the original 1.0 save bug: flying writes disableDamage.
        a.put("flying", Tag::Byte(self.disable_damage as i8));
        a.put("mayfly", Tag::Byte(self.allow_flying as i8));
        a.put("instabuild", Tag::Byte(self.creative as i8));
        n.put("abilities", Tag::Compound(a));
    }
    pub fn read_nbt(&mut self, n: &Compound) {
        if let Some(Tag::Compound(a)) = n.get("abilities") {
            self.disable_damage = a.boolean("invulnerable");
            self.flying = a.boolean("flying");
            self.allow_flying = a.boolean("mayfly");
            self.creative = a.boolean("instabuild");
        }
    }
}

#[cfg(test)]
mod release_tests {
    use super::*;
    #[test]
    fn experience_uses_initial_cap_for_all_awarded_points() {
        let mut xp = Experience::default();
        xp.add(14);
        assert_eq!((xp.points, xp.level, xp.total), (0.0, 2, 14));
        assert_eq!(xp.bar_cap(), 14);
        xp.add(7);
        assert_eq!(xp.points, 0.5);
        xp.spend_levels(4);
        assert_eq!((xp.points, xp.level, xp.total), (0.5, 0, 21));
    }
    #[test]
    fn abilities_preserve_release_flying_serialization_bug() {
        let mut abilities = PlayerCapabilities {
            disable_damage: true,
            flying: false,
            allow_flying: true,
            creative: true,
        };
        let mut n = Compound::new();
        abilities.write_nbt(&mut n);
        abilities.read_nbt(&n);
        assert!(abilities.flying);
    }
}
