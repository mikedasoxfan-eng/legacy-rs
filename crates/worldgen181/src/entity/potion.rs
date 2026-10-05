//! The small, pre-brewing Beta 1.8.1 potion registry and PotionEffect.
pub const SPEED: i32 = 1;
pub const SLOWNESS: i32 = 2;
pub const HEAL: i32 = 6;
pub const HARM: i32 = 7;
pub const REGENERATION: i32 = 10;
pub const WATER_BREATHING: i32 = 13;
pub const HUNGER: i32 = 17;
pub const POISON: i32 = 19;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PotionEffect { pub id: i32, pub duration: i32, pub amplifier: i32 }
impl PotionEffect {
    pub fn new(id: i32, duration: i32, amplifier: i32) -> Self { Self { id, duration, amplifier } }
    pub fn combine(&mut self, other: Self) {
        if other.amplifier >= self.amplifier {
            self.amplifier = other.amplifier;
            self.duration = other.duration;
        }
    }
    pub fn ready(self) -> bool { is_ready(self.id, self.duration, self.amplifier) }
}
pub fn is_ready(id: i32, duration: i32, amplifier: i32) -> bool {
    match id {
        HEAL | HARM => duration >= 1,
        REGENERATION | POISON => {
            let interval = 25i32 >> (amplifier & 31);
            interval <= 0 || duration % interval == 0
        }
        HUNGER => true,
        _ => false,
    }
}
impl super::Entity {
    pub fn add_potion_effect(&mut self, effect: PotionEffect) {
        if let Some(old) = self.living.effects.iter_mut().find(|e| e.id == effect.id) {
            old.combine(effect);
        } else { self.living.effects.push(effect);if self.living.effects.len()>self.living.effect_capacity*3/4 {self.living.effect_capacity*=2;} }
    }
    pub fn potion_effect(&self, id: i32) -> Option<PotionEffect> {
        self.living.effects.iter().find(|e| e.id == id).copied()
    }
    pub fn remove_potion_effect(&mut self, id: i32) { self.living.effects.retain(|e| e.id != id); }
    pub fn potion_speed_multiplier(&self) -> f32 {
        let mut multiplier = 1.0f32;
        if let Some(e) = self.potion_effect(SPEED) { multiplier *= 1.0 + 0.2 * e.amplifier.wrapping_add(1) as f32; }
        if let Some(e) = self.potion_effect(SLOWNESS) { multiplier *= 1.0 - 0.15 * e.amplifier.wrapping_add(1) as f32; }
        if self.living.flee_ticks > 0 { multiplier *= 2.0; }
        multiplier
    }
    pub fn tick_potion_effects(&mut self, w: &mut crate::World) {
        // Java 8 HashMap<Integer,...>: ascending buckets, insertion order within a bucket.
        let capacity = self.living.effect_capacity;
        let mut effects = std::mem::take(&mut self.living.effects);
        effects.sort_by_key(|e| ((e.id as u32 ^ (e.id as u32 >> 16)) as usize) & (capacity - 1));
        for mut effect in effects {
            if effect.duration > 0 {
                if effect.ready() { self.apply_potion(w, effect); }
                effect.duration -= 1;
            }
            if effect.duration > 0 { self.living.effects.push(effect); }
        }
    }
    pub fn apply_potion(&mut self, w: &mut crate::World, effect: PotionEffect) {
        use super::damage::{DamageKind, DamageSource};
        match effect.id {
            REGENERATION if self.living.health < 20 => self.heal(1),
            POISON if self.living.health > 1 => { self.attack_from(w, DamageSource::new(DamageKind::Magic), 1); }
            HEAL => self.heal(4i32.wrapping_shl((effect.amplifier & 31) as u32)),
            HARM => { self.attack_from(w, DamageSource::new(DamageKind::Magic), 4i32.wrapping_shl((effect.amplifier & 31) as u32)); }
            HUNGER => { if let Some(p) = self.player_state_mut() { p.food_exhaustion = (p.food_exhaustion + 0.025 * effect.amplifier.wrapping_add(1) as f32).min(40.0); } }
            _ => {}
        }
    }
}
