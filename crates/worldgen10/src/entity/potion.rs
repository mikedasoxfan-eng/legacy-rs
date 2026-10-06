//! Release 1.0 potion registry, effects, and living-entity potion behavior.
pub const SPEED: i32 = 1;
pub const JUMP: i32 = 8;
pub const RESISTANCE: i32 = 11;
pub const FIRE_RESISTANCE: i32 = 12;
pub const SLOWNESS: i32 = 2;
pub const HEAL: i32 = 6;
pub const HARM: i32 = 7;
pub const REGENERATION: i32 = 10;
pub const WATER_BREATHING: i32 = 13;
pub const HUNGER: i32 = 17;
pub const POISON: i32 = 19;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PotionEffect {
    pub id: i32,
    pub duration: i32,
    pub amplifier: i32,
}
impl PotionEffect {
    pub fn new(id: i32, duration: i32, amplifier: i32) -> Self {
        Self {
            id,
            duration,
            amplifier,
        }
    }
    pub fn combine(&mut self, other: Self) {
        if self.id!=other.id {eprintln!("This method should only be called for matching effects!");}
        if other.amplifier > self.amplifier {
            self.amplifier = other.amplifier;
            self.duration = other.duration;
        } else if other.amplifier == self.amplifier && other.duration > self.duration {
            self.duration = other.duration;
        }
    }
    pub fn ready(self) -> bool {
        is_ready(self.id, self.duration, self.amplifier)
    }
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
        if self.creature_attribute() == super::living10::EnumCreatureAttribute::Undead
            && matches!(effect.id, REGENERATION | POISON)
        {
            return;
        }
        if matches!(
            self.kind,
            super::EntityKind::Spider | super::EntityKind::CaveSpider
        ) && effect.id == POISON
        {
            return;
        }
        self.living.effects_changed = true;
        if let Some(old) = self.living.effects.iter_mut().find(|e| e.id == effect.id) {
            old.combine(effect);
        } else {
            self.living.effects.push(effect);
            if self.living.effects.len() > self.living.effect_capacity * 3 / 4 {
                self.living.effect_capacity *= 2;
            }
        }
    }
    pub fn potion_effect(&self, id: i32) -> Option<PotionEffect> {
        self.living.effects.iter().find(|e| e.id == id).copied()
    }
    pub fn remove_potion_effect(&mut self, id: i32) {
        let count = self.living.effects.len();
        self.living.effects.retain(|e| e.id != id);
        if self.living.effects.len() != count {
            self.living.effects_changed = true;
        }
    }
    pub fn potion_speed_multiplier(&self) -> f32 {
        let mut multiplier = 1.0f32;
        if let Some(e) = self.potion_effect(SPEED) {
            multiplier *= 1.0 + 0.2 * e.amplifier.wrapping_add(1) as f32;
        }
        if let Some(e) = self.potion_effect(SLOWNESS) {
            multiplier *= 1.0 - 0.15 * e.amplifier.wrapping_add(1) as f32;
        }
        if self.living.flee_ticks > 0 {
            multiplier *= 2.0;
        }
        multiplier
    }
    pub fn tick_potion_effects(&mut self, w: &mut crate::World) {
        // Java 8 HashMap<Integer,...>: ascending buckets, insertion order within a bucket.
        let capacity = self.living.effect_capacity;
        let mut effects = std::mem::take(&mut self.living.effects);
        effects.sort_by_key(|e| ((e.id as u32 ^ (e.id as u32 >> 16)) as usize) & (capacity - 1));
        for mut effect in effects {
            if effect.duration > 0 {
                if effect.ready() {
                    self.apply_potion(w, effect);
                }
                effect.duration -= 1;
            }
            if effect.duration > 0 || w.multiplayer_world {
                self.living.effects.push(effect);
            } else {
                self.living.effects_changed = true;
            }
        }
        if self.living.effects_changed {
            if !w.multiplayer_world {
                let color = if self.living.effects.is_empty() {
                    0
                } else {
                    effect_color(&self.living.effects)
                };
                self.data_watcher
                    .update(8, super::data_watcher::Value::Int(color))
                    .unwrap();
            }
            self.living.effects_changed = false;
        }
        if self.rand.next_bool() {
            let color = self.data_watcher.int(8).unwrap_or(0);
            if color > 0 {
                let r = (color >> 16 & 255) as f64 / 255.0;
                let g = (color >> 8 & 255) as f64 / 255.0;
                let b = (color & 255) as f64 / 255.0;
                let p = super::Vec3D::new(
                    self.pos.x + (self.rand.next_double() - 0.5) * self.width as f64,
                    self.pos.y + self.rand.next_double() * self.height as f64
                        - self.y_offset as f64,
                    self.pos.z + (self.rand.next_double() - 0.5) * self.width as f64,
                );
                w.spawn_particle("mobSpell", p, super::Vec3D::new(r, g, b));
            }
        }
    }
    pub fn clear_potion_effects(&mut self, w: &crate::World) {
        if !w.multiplayer_world && !self.living.effects.is_empty() {
            self.living.effects.clear();
            self.living.effects_changed = true;
        }
    }
    pub fn apply_potion(&mut self, w: &mut crate::World, effect: PotionEffect) {
        use super::damage::{DamageKind, DamageSource};
        match effect.id {
            REGENERATION if self.living.health < self.max_health() => self.heal(1),
            POISON if self.living.health > 1 => {
                self.attack_from(w, DamageSource::new(DamageKind::Magic), 1);
            }
            HEAL | HARM
                if (effect.id == HEAL)
                    != (self.creature_attribute()
                        == super::living10::EnumCreatureAttribute::Undead) =>
            { self.heal(6i32.wrapping_shl((effect.amplifier & 31) as u32))
            }
            HEAL |
            HARM => {
                self.attack_from(
                    w,
                    DamageSource::new(DamageKind::Magic),
                    6i32.wrapping_shl((effect.amplifier & 31) as u32),
                );
            }
            HUNGER => {
                // EntityPlayer.addExhaustion (skipped while damage is disabled).
                self.add_exhaustion(0.025 * effect.amplifier.wrapping_add(1) as f32);
            }
            _ => {}
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Potion {
    pub id: i32,
    pub name: &'static str,
    pub harmful: bool,
    pub color: i32,
    pub icon: i32,
    pub effectiveness: f64,
    pub usable: bool,
}
pub const POTIONS: [Potion; 19] = [
    Potion {
        id: 1,
        name: "potion.moveSpeed",
        harmful: false,
        color: 8171462,
        icon: 0,
        effectiveness: 1.0,
        usable: false,
    },
    Potion {
        id: 2,
        name: "potion.moveSlowdown",
        harmful: true,
        color: 5926017,
        icon: 1,
        effectiveness: 0.5,
        usable: false,
    },
    Potion {
        id: 3,
        name: "potion.digSpeed",
        harmful: false,
        color: 14270531,
        icon: 2,
        effectiveness: 1.5,
        usable: false,
    },
    Potion {
        id: 4,
        name: "potion.digSlowDown",
        harmful: true,
        color: 4866583,
        icon: 3,
        effectiveness: 0.5,
        usable: false,
    },
    Potion {
        id: 5,
        name: "potion.damageBoost",
        harmful: false,
        color: 9643043,
        icon: 4,
        effectiveness: 1.0,
        usable: false,
    },
    Potion {
        id: 6,
        name: "potion.heal",
        harmful: false,
        color: 16262179,
        icon: -1,
        effectiveness: 1.0,
        usable: false,
    },
    Potion {
        id: 7,
        name: "potion.harm",
        harmful: true,
        color: 4393481,
        icon: -1,
        effectiveness: 0.5,
        usable: false,
    },
    Potion {
        id: 8,
        name: "potion.jump",
        harmful: false,
        color: 7889559,
        icon: 10,
        effectiveness: 1.0,
        usable: false,
    },
    Potion {
        id: 9,
        name: "potion.confusion",
        harmful: true,
        color: 5578058,
        icon: 11,
        effectiveness: 0.25,
        usable: false,
    },
    Potion {
        id: 10,
        name: "potion.regeneration",
        harmful: false,
        color: 13458603,
        icon: 7,
        effectiveness: 0.25,
        usable: false,
    },
    Potion {
        id: 11,
        name: "potion.resistance",
        harmful: false,
        color: 10044730,
        icon: 14,
        effectiveness: 1.0,
        usable: false,
    },
    Potion {
        id: 12,
        name: "potion.fireResistance",
        harmful: false,
        color: 14981690,
        icon: 15,
        effectiveness: 1.0,
        usable: false,
    },
    Potion {
        id: 13,
        name: "potion.waterBreathing",
        harmful: false,
        color: 3035801,
        icon: 16,
        effectiveness: 1.0,
        usable: false,
    },
    Potion {
        id: 14,
        name: "potion.invisibility",
        harmful: false,
        color: 8356754,
        icon: 8,
        effectiveness: 1.0,
        usable: true,
    },
    Potion {
        id: 15,
        name: "potion.blindness",
        harmful: true,
        color: 2039587,
        icon: 13,
        effectiveness: 0.25,
        usable: false,
    },
    Potion {
        id: 16,
        name: "potion.nightVision",
        harmful: false,
        color: 2039713,
        icon: 12,
        effectiveness: 1.0,
        usable: true,
    },
    Potion {
        id: 17,
        name: "potion.hunger",
        harmful: true,
        color: 5797459,
        icon: 9,
        effectiveness: 0.5,
        usable: false,
    },
    Potion {
        id: 18,
        name: "potion.weakness",
        harmful: true,
        color: 4738376,
        icon: 5,
        effectiveness: 0.5,
        usable: false,
    },
    Potion {
        id: 19,
        name: "potion.poison",
        harmful: true,
        color: 5149489,
        icon: 6,
        effectiveness: 0.25,
        usable: false,
    },
];
pub fn by_id(id: i32) -> Option<&'static Potion> {
    POTIONS.iter().find(|p| p.id == id)
}
pub fn duration_string(effect: PotionEffect) -> String {
    let seconds = effect.duration / 20;
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    if seconds < 10 {
        format!("{minutes}:0{seconds}")
    } else {
        format!("{minutes}:{seconds}")
    }
}
impl PotionEffect {
    pub fn hash_code(self) -> i32 {
        self.id
    }
    pub fn description(self) -> String {
        let p = by_id(self.id).expect("Potion.potionTypes[id]");
        let text = if self.amplifier > 0 {
            format!(
                "{} x {}, Duration: {}",
                p.name,
                self.amplifier.wrapping_add(1),
                self.duration
            )
        } else {
            format!("{}, Duration: {}", p.name, self.duration)
        };
        if p.usable {
            format!("({text})")
        } else {
            text
        }
    }
}
/// The living-entity call to PotionHelper.func_40354_a, retaining float accumulation.
pub fn effect_color(effects: &[PotionEffect]) -> i32 {
    if effects.is_empty() {
        return 3694022;
    }
    let (mut r, mut g, mut b, mut count) = (0.0f32, 0.0f32, 0.0f32, 0.0f32);
    for effect in effects {
        let color = by_id(effect.id).expect("Potion.potionTypes[id]").color;
        for _ in 0..=effect.amplifier {
            r += (color >> 16 & 255) as f32 / 255.0;
            g += (color >> 8 & 255) as f32 / 255.0;
            b += (color & 255) as f32 / 255.0;
            count += 1.0;
        }
    }
    ((r / count * 255.0) as i32).wrapping_shl(16)
        | ((g / count * 255.0) as i32).wrapping_shl(8)
        | (b / count * 255.0) as i32
}
impl super::Entity {
    pub fn apply_instant_potion(
        &mut self,
        w: &mut crate::World,
        id: i32,
        owner: Option<i32>,
        amplifier: i32,
        strength: f64,
    ) {
        use super::damage::{DamageKind, DamageSource};
        let undead = self.creature_attribute() == super::living10::EnumCreatureAttribute::Undead;
        let amount = (strength * 6i32.wrapping_shl((amplifier & 31) as u32) as f64 + 0.5) as i32;
        if (id == HEAL && !undead) || (id == HARM && undead) {
            self.heal(amount);
        } else if (id == HARM && !undead) || (id == HEAL && undead) {
            let source = owner.map_or(DamageSource::new(DamageKind::Magic), |owner| {
                DamageSource::indirect(DamageKind::IndirectMagic, self.entity_id, Some(owner))
            });
            self.attack_from(w, source, amount);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn release_metadata_selects_durations_and_instant_effects() {
        assert_eq!(
            effects_for_metadata(8194),
            vec![PotionEffect::new(SPEED, 3600, 0)]
        );
        assert_eq!(
            effects_for_metadata(8258),
            vec![PotionEffect::new(SPEED, 9600, 0)]
        );
        assert_eq!(
            effects_for_metadata(8226),
            vec![PotionEffect::new(SPEED, 1800, 1)]
        );
        assert_eq!(
            effects_for_metadata(16386),
            vec![PotionEffect::new(SPEED, 2701, 0)]
        );
        assert_eq!(
            effects_for_metadata(16389),
            vec![PotionEffect::new(HEAL, 1, 0)]
        );
        assert!(effects_for_metadata(0).is_empty());
    }
    #[test]
    fn equal_amplifier_does_not_shorten_existing_effect() {
        let mut effect = PotionEffect::new(SPEED, 100, 1);
        effect.combine(PotionEffect::new(SPEED, 20, 1));
        assert_eq!(effect.duration, 100);
        effect.combine(PotionEffect::new(SPEED, 20, 2));
        assert_eq!((effect.duration, effect.amplifier), (20, 2));
    }
    #[test]
    fn undead_inverts_healing_and_rejects_regeneration() {
        let mut w = super::super::verify181::world();
        let mut e = super::super::Entity::living(&mut w, "Zombie").unwrap();
        e.add_potion_effect(PotionEffect::new(REGENERATION, 100, 0));
        e.add_potion_effect(PotionEffect::new(POISON, 100, 0));
        assert!(e.living.effects.is_empty());
        e.living.health = 10;
        e.apply_potion(&mut w, PotionEffect::new(HARM, 1, 0));
        assert_eq!(e.living.health, 16);
        e.apply_potion(&mut w, PotionEffect::new(HEAL, 1, 0));
        assert_eq!(e.living.health, 10);
    }
}

/// PotionHelper's expression evaluator, needed by the EntityPotion effect supplier.
fn requirement(expression: &str, start: i32, end: i32, metadata: i32) -> i32 {
    let text = expression.as_bytes();
    if start >= text.len() as i32 || end < 0 || start >= end {
        return 0;
    }
    for operator in [b'|', b'&'] {
        if let Some(relative) = text[start as usize..].iter().position(|&c| c == operator) {
            let split = start + relative as i32;
            if split < end {
                let left = requirement(expression, start, split - 1, metadata);
                if operator == b'|' {
                    if left > 0 {
                        return left;
                    }
                    let right = requirement(expression, split + 1, end, metadata);
                    return if right > 0 { right } else { 0 };
                }
                if left <= 0 {
                    return 0;
                }
                let right = requirement(expression, split + 1, end, metadata);
                return if right <= 0 {
                    0
                } else {
                    if left > right {
                        left
                    } else {
                        right
                    }
                };
            }
        }
    }
    let (mut multiply, mut has_multiplier, mut has_number, mut invert, mut negative) =
        (false, false, false, false, false);
    let (mut comparison, mut number, mut multiplier, mut result) = (-1, 0i32, 0i32, 0i32);
    let evaluate = |invert: bool,
                    has_multiplier: bool,
                    negative: bool,
                    comparison: i32,
                    number: i32,
                    multiplier: i32| {
        let mut value = 0i32;
        if invert {
            value = i32::from(metadata & 1i32.wrapping_shl((number & 31) as u32) == 0);
        } else if comparison != -1 {
            let mut bits = metadata;
            let mut count = 0;
            while bits > 0 {
                bits &= bits - 1;
                count += 1;
            }
            if (comparison == 0 && count == number)
                || (comparison == 1 && count > number)
                || (comparison == 2 && count < number)
            {
                value = 1;
            }
        } else {
            value = i32::from(metadata & 1i32.wrapping_shl((number & 31) as u32) != 0);
        }
        if has_multiplier {
            value = value.wrapping_mul(multiplier);
        }
        if negative {
            value = value.wrapping_mul(-1);
        }
        value
    };
    for &character in &text[start as usize..end as usize] {
        if character.is_ascii_digit() {
            if multiply {
                multiplier = (character - b'0') as i32;
                has_multiplier = true;
            } else {
                number = number
                    .wrapping_mul(10)
                    .wrapping_add((character - b'0') as i32);
                has_number = true;
            }
        } else if character == b'*' {
            multiply = true;
        } else if matches!(character, b'!' | b'-' | b'+' | b'=' | b'<' | b'>') {
            if has_number {
                result = result.wrapping_add(evaluate(
                    invert,
                    has_multiplier,
                    negative,
                    comparison,
                    number,
                    multiplier,
                ));
                invert = false;
                if character != b'-' {
                    negative = false;
                }
                multiply = false;
                has_multiplier = false;
                has_number = false;
                number = 0;
                multiplier = 0;
                comparison = -1;
            }
            match character {
                b'!' => invert = true,
                b'-' => negative = true,
                b'=' => comparison = 0,
                b'<' => comparison = 2,
                b'>' => comparison = 1,
                _ => {}
            }
        }
    }
    if has_number {
        result = result.wrapping_add(evaluate(
            invert,
            has_multiplier,
            negative,
            comparison,
            number,
            multiplier,
        ));
    }
    result
}
pub fn effects_for_metadata(metadata: i32) -> Vec<PotionEffect> {
    let mut effects = Vec::new();
    for potion in &POTIONS {
        if potion.usable {
            continue;
        }
        let expression = match potion.id {
            10 => "0 & !1 & !2 & !3 & 0+6",
            1 => "!0 & 1 & !2 & !3 & 1+6",
            12 => "0 & 1 & !2 & !3 & 0+6",
            6 => "0 & !1 & 2 & !3",
            19 => "!0 & !1 & 2 & !3 & 2+6",
            18 => "!0 & !1 & !2 & 3 & 3+6",
            7 => "!0 & !1 & 2 & 3",
            2 => "!0 & 1 & !2 & 3 & 3+6",
            5 => "0 & !1 & !2 & 3 & 3+6",
            _ => continue,
        };
        let mut score = requirement(expression, 0, expression.len() as i32, metadata);
        if score <= 0 {
            continue;
        }
        let amplifier = if matches!(potion.id, 1 | 3 | 5 | 10 | 7 | 6 | 11 | 19) {
            requirement("5", 0, 1, metadata).max(0)
        } else {
            0
        };
        if matches!(potion.id, HEAL | HARM) {
            score = 1;
        } else {
            score = 1200i32.wrapping_mul(
                score
                    .wrapping_mul(3)
                    .wrapping_add(score.wrapping_sub(1).wrapping_mul(2)),
            );
            score >>= amplifier & 31;
            score = (score as f64 * potion.effectiveness + 0.5).floor() as i64 as i32;
            if metadata & 16384 != 0 {
                score = (score as f64 * 0.75 + 1.0).floor() as i64 as i32;
            }
        }
        effects.push(PotionEffect::new(potion.id, score, amplifier));
    }
    effects
}
