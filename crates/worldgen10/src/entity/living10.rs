//! Release 1.0 additions to EntityLiving, shared by every living subclass.
use super::{damage::DamageSource, player::ItemStack, Entity, EntityKind};
use crate::{entity::Vec3D, math, World};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnumCreatureAttribute {
    Undefined,
    Undead,
    Arthropod,
}
impl Entity {
    pub fn potion_attack_damage(&self, mut damage: i32) -> i32 {
        if let Some(p) = self.potion_effect(5) {
            damage = damage.wrapping_add(3i32.wrapping_shl(p.amplifier as u32));
        }
        if let Some(p) = self.potion_effect(18) {
            damage = damage.wrapping_sub(2i32.wrapping_shl(p.amplifier as u32));
        }
        damage
    }
    pub fn is_burning(&self, w: &World) -> bool {
        match self.kind {
            EntityKind::Blaze(..) => self.data_watcher.byte(16).unwrap() & 1 != 0,
            EntityKind::MagmaCube { .. } => false,
            _ => self.fire > 0 || (w.multiplayer_world && self.entity_flag(0)),
        }
    }
    pub fn max_health(&self) -> i32 {
        match &self.kind {
            EntityKind::Dragon(..) => 200,
            EntityKind::Giant => 100,
            EntityKind::Enderman(..) => 40,
            EntityKind::Blaze(..)
            | EntityKind::Villager { .. }
            | EntityKind::Player(..)
            | EntityKind::Zombie
            | EntityKind::Skeleton
            | EntityKind::Creeper { .. }
            | EntityKind::PigZombie { .. } => 20,
            EntityKind::Spider => 16,
            EntityKind::CaveSpider => 12,
            EntityKind::Sheep { .. } | EntityKind::Silverfish { .. } => 8,
            EntityKind::Wolf(s) => {
                if s.flags & 4 != 0 {
                    20
                } else {
                    8
                }
            }
            EntityKind::Chicken { .. } | EntityKind::Snowman => 4,
            EntityKind::Slime { size, .. } | EntityKind::MagmaCube { size, .. } => {
                size.wrapping_mul(*size)
            }
            _ => 10,
        }
    }
    /// The Java setter stores the original value, despite clamping its local afterward.
    pub fn set_entity_health(&mut self, health: i32) {
        self.living.health = health;
    }
    pub fn is_child(&self) -> bool {
        self.living.growing_age < 0
    }
    pub fn creature_attribute(&self) -> EnumCreatureAttribute {
        match self.kind {
            EntityKind::Zombie | EntityKind::Skeleton | EntityKind::PigZombie { .. } => {
                EnumCreatureAttribute::Undead
            }
            EntityKind::Spider | EntityKind::CaveSpider | EntityKind::Silverfish { .. } => {
                EnumCreatureAttribute::Arthropod
            }
            _ => EnumCreatureAttribute::Undefined,
        }
    }
    pub(super) fn living_sound_pitch(&mut self) -> f32 {
        (self.rand.next_float() - self.rand.next_float()) * 0.2
            + if self.is_child() { 1.5 } else { 1.0 }
    }
    pub(super) fn armor_damage(&mut self, source: DamageSource, mut damage: i32) -> i32 {
        if !source.bypasses_armor() {
            let armor = if let Some(p) = self.player_state() {
                p.inventory.armor_value()
            } else if let EntityKind::MagmaCube { size, .. } = self.kind {
                size * 3
            } else if matches!(self.kind, EntityKind::Zombie | EntityKind::PigZombie { .. }) {
                2
            } else {
                0
            };
            let total = damage
                .wrapping_mul(25 - armor)
                .wrapping_add(self.living.damage_remainder);
            if let Some(p) = self.player_state_mut() {
                p.inventory.damage_armor(damage);
            }
            damage = total / 25;
            self.living.damage_remainder = total % 25;
        }
        damage
    }
    pub(super) fn resistance_damage(&mut self, mut damage: i32) -> i32 {
        if let Some(effect) = self.potion_effect(super::potion::RESISTANCE) {
            let reduction = effect.amplifier.wrapping_add(1).wrapping_mul(5);
            let total = damage
                .wrapping_mul(25i32.wrapping_sub(reduction))
                .wrapping_add(self.living.damage_remainder);
            damage = total / 25;
            self.living.damage_remainder = total % 25;
        }
        damage
    }
    pub fn broken_item_particles(&mut self, w: &mut World, item: ItemStack) {
        let pitch = 0.8 + w.rand.next_float() * 0.4;
        w.play_sound_at_entity(self, "random.break", 0.8, pitch);
        for _ in 0..5 {
            let mut velocity = Vec3D::new(
                (self.rand.next_float() as f64 - 0.5) * 0.1,
                w.math_random.next_double() * 0.1 + 0.1,
                0.0,
            );
            velocity.rotate_x(-self.pitch * math::PI_F32 / 180.0);
            velocity.rotate_y(-self.yaw * math::PI_F32 / 180.0);
            let mut p = Vec3D::new(
                (self.rand.next_float() as f64 - 0.5) * 0.3,
                -self.rand.next_float() as f64 * 0.6 - 0.3,
                0.6,
            );
            p.rotate_x(-self.pitch * math::PI_F32 / 180.0);
            p.rotate_y(-self.yaw * math::PI_F32 / 180.0);
            p = p.add(
                self.pos.x,
                self.pos.y + self.eye_height() as f64,
                self.pos.z,
            );
            w.spawn_particle(
                &format!("iconcrack_{}", item.id),
                p,
                velocity.add(0.0, 0.05, 0.0),
            );
        }
    }
}
impl Entity {
    pub fn set_fire_seconds(&mut self, seconds: i32) {
        let ticks = seconds.wrapping_mul(20);
        if self.fire < ticks {
            self.fire = ticks;
        }
    }
    pub fn set_air(&mut self, air: i32) {
        self.air = air as i16 as i32;
        self.data_watcher
            .update(1, super::data_watcher::Value::Short(air as i16))
            .unwrap();
    }
    pub fn entity_identity_matches(&self, id: Option<i32>) -> bool {
        id == Some(self.entity_id)
            || matches!(self.kind,EntityKind::DragonPart {parent,..} if id==Some(parent))
    }
    pub fn set_growing_age(&mut self, age: i32) {
        self.living.growing_age = age;
        if self.data_watcher.int(12).is_some() {
            self.data_watcher
                .update(12, super::data_watcher::Value::Int(age))
                .unwrap();
        }
    }
}

#[cfg(test)]
mod release_tests {
    #[test]
    fn armor_keeps_full_protection_when_damaged() {
        let mut inventory = super::super::player::InventoryPlayer::default();
        for (slot, id) in [310, 311, 312, 313].into_iter().enumerate() {
            inventory.armor[slot] = Some(super::super::player::ItemStack::new(id, 1, 100));
        }
        assert_eq!(inventory.armor_value(), 20);
        inventory.damage_armor(8);
        assert!(inventory
            .armor
            .iter()
            .flatten()
            .all(|piece| piece.damage == 102));
    }
}
