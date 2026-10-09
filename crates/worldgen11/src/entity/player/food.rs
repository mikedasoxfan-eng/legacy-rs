//! Release 1.0 `FoodStats`. Arithmetic and the one-exhaustion-step-per-tick bug
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
    pub fn add(&mut self, mut amount: i32) {
        let remaining = i32::MAX.wrapping_sub(self.total);
        if amount > remaining {
            amount = remaining;
        }
        self.points += amount as f32 / self.bar_cap() as f32;
        self.total = self.total.wrapping_add(amount);
        while self.points >= 1.0 {
            self.points = (self.points - 1.0) * self.bar_cap() as f32;
            self.level = self.level.wrapping_add(1);
            self.points /= self.bar_cap() as f32;
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
    pub hardcore: bool, pub world_type: crate::storage::WorldType, }
impl WorldSettings { pub fn new(seed: i64, game_mode: i32, map_features: bool) -> Self { Self::with_hardcore(seed, game_mode, map_features, false)
    }
    pub fn with_hardcore(seed: i64, game_mode: i32, map_features: bool, hardcore: bool) -> Self {
        Self { seed, game_mode, map_features,
            hardcore, world_type: crate::storage::WorldType::Default, } }
    pub fn with_world_type(mut self, world_type: crate::storage::WorldType) -> Self {
        self.world_type = world_type;
        self
    }
}

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
    fn experience_matches_original_11_bytecode_vectors() {
        // GameplaySlice11 invokes EntityPlayer.increaseXP from the original jar.
        for line in include_str!("../../fixtures/xp11.txt").lines() {
            let v: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
            let mut xp = Experience { level: v[0], total: v[1], points: f32::from_bits(v[3] as u32) };
            xp.add(v[4]);
            assert_eq!((xp.level, xp.total, xp.points.to_bits()), (v[5], v[6], v[8] as u32), "{line}");
            assert_eq!(v[2].wrapping_add(v[4]), v[7], "original score increment: {line}");
        }
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

/// Java FoodStats object identity. Replacement players retain the same object;
/// HUD snapshots explicitly copy its value before crossing the worker boundary.
#[derive(Clone, Debug, Default)]
pub struct SharedFoodStats(std::sync::Arc<std::sync::Mutex<FoodStats>>);
impl SharedFoodStats {
    pub fn share(&self) -> Self { Self(self.0.clone()) }
    pub fn same_object(&self, other: &Self) -> bool { std::sync::Arc::ptr_eq(&self.0, &other.0) }
    pub fn read(&self) -> std::sync::MutexGuard<'_, FoodStats> { self.0.lock().expect("FoodStats lock") }
    pub fn write(&self) -> std::sync::MutexGuard<'_, FoodStats> { self.read() }
    pub fn snapshot(&self) -> FoodStats { self.read().clone() }
    pub fn add_food(&self, amount: i32, modifier: f32) { self.write().add_food(amount, modifier); }
    pub fn add_exhaustion(&self, amount: f32) { self.write().add_exhaustion(amount); }
    pub fn needs_food(&self) -> bool { self.read().needs_food() }
    pub fn tick(&self, difficulty: i32, health: i32) -> FoodAction { self.write().tick(difficulty, health) }
    pub fn read_nbt(&self, n: &Compound) { self.write().read_nbt(n); }
    pub fn write_nbt(&self, n: &mut Compound) { self.read().write_nbt(n); }
}
