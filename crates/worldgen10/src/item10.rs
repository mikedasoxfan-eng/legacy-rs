//! Release 1.0 item subclass logic. Projectile construction is supplied by the
//! entity owner; it occurs after the item's count, sound and RNG changes.
use crate::{
    entity::{player::ItemStack, potion::PotionEffect, Entity, MovingObjectPosition, Vec3D},
    potion_helper as ph, World,
};
use std::{collections::HashMap, sync::Arc};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
}
impl Rarity {
    pub fn color(self) -> i32 {
        match self {
            Self::Common => 15,
            Self::Uncommon => 14,
            Self::Rare => 11,
            Self::Epic => 13,
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Common => "Common",
            Self::Uncommon => "Uncommon",
            Self::Rare => "Rare",
            Self::Epic => "Epic",
        }
    }
}
pub fn rarity(stack: ItemStack) -> Rarity {
    if stack.id == 322 {
        Rarity::Epic
    } else if (2256..=2266).contains(&stack.id) || stack.is_enchanted() {
        Rarity::Rare
    } else {
        Rarity::Common
    }
}
pub fn glint(stack: ItemStack) -> bool {
    if stack.id == 322 {
        true
    } else if stack.id == 373 {
        ph::effects(stack.damage, false).is_some_and(|e| !e.is_empty())
    } else {
        stack.is_enchanted()
    }
}
#[derive(Default)]
pub struct PotionItem {
    cache: HashMap<i32, Option<Arc<[PotionEffect]>>>,
}
impl PotionItem {
    pub fn effects(&mut self, damage: i32) -> Option<Arc<[PotionEffect]>> {
        if let Some(Some(effects)) = self.cache.get(&damage) {
            return Some(effects.clone());
        }
        let effects = ph::effects(damage, false).map(Arc::from);
        self.cache.insert(damage, effects.clone());
        effects
    }
    pub fn icon(damage: i32) -> i32 {
        if damage & 16384 != 0 {
            154
        } else {
            140
        }
    }
    pub fn has_instant(&mut self, damage: i32) -> bool {
        self.effects(damage)
            .is_some_and(|e| e.iter().any(|e| matches!(e.id, 6 | 7)))
    }
    pub fn drink(
        &mut self,
        mut stack: ItemStack,
        multiplayer: bool,
        mut apply: impl FnMut(PotionEffect),
        mut add_bottle: impl FnMut(ItemStack),
    ) -> ItemStack {
        stack.count = stack.count.wrapping_sub(1);
        if !multiplayer {
            if let Some(effects) = self.effects(stack.damage) {
                for &effect in effects.iter() {
                    apply(effect);
                }
            }
        }
        if stack.count <= 0 {
            ItemStack::new(374, 1, 0)
        } else {
            add_bottle(ItemStack::new(374, 1, 0));
            stack
        }
    }
    pub fn display_name(
        &mut self,
        damage: i32,
        base_name: &str,
        translate: impl Fn(&str) -> String,
    ) -> String {
        if damage == 0 {
            return translate("item.emptyPotion.name").trim().into();
        }
        let base = if damage & 16384 != 0 {
            format!(
                "{} {}",
                translate("potion.prefix.grenade").trim(),
                base_name
            )
        } else {
            base_name.into()
        };
        if let Some(effects) = self.effects(damage).filter(|e| !e.is_empty()) {
            format!(
                "{} {}",
                base,
                translate(&format!("{}.postfix", potion_name(effects[0].id))).trim()
            )
        } else {
            format!("{} {}", translate(&ph::prefix(damage)).trim(), base)
        }
    }
    pub fn tooltip(&mut self, damage: i32, translate: impl Fn(&str) -> String) -> Vec<String> {
        if damage == 0 {
            return Vec::new();
        }
        let Some(effects) = self.effects(damage).filter(|e| !e.is_empty()) else {
            return vec![format!("§7{}", translate("potion.empty").trim())];
        };
        effects
            .iter()
            .map(|e| {
                let mut name = translate(potion_name(e.id)).trim().to_string();
                if e.amplifier > 0 {
                    name.push_str(&format!(
                        " {}",
                        translate(&format!("potion.potency.{}", e.amplifier)).trim()
                    ));
                }
                if e.duration > 20 {
                    let seconds = e.duration / 20;
                    name.push_str(&format!(" ({}:{:02})", seconds / 60, seconds % 60));
                }
                format!(
                    "§{}{}",
                    if matches!(e.id, 2 | 4 | 7 | 9 | 15 | 17 | 18 | 19) {
                        'c'
                    } else {
                        '7'
                    },
                    name
                )
            })
            .collect()
    }
}
pub fn potion_name(id: i32) -> &'static str {
    match id {
        1 => "potion.moveSpeed",
        2 => "potion.moveSlowdown",
        5 => "potion.damageBoost",
        6 => "potion.heal",
        7 => "potion.harm",
        10 => "potion.regeneration",
        12 => "potion.fireResistance",
        18 => "potion.weakness",
        19 => "potion.poison",
        _ => panic!("potion without brewing requirement"),
    }
}
impl Entity {
    /// Item.func_40402_a uses !liquids for ignoreBlockWithoutBoundingBox.
    pub fn item_ray10(&self, w: &mut World, liquids: bool) -> Option<MovingObjectPosition> {
        let (start, end) = self.bucket_ray181();
        w.ray_trace_blocks_options(start, end, liquids, !liquids)
    }
    pub fn glass_bottle_right_click10(&mut self, w: &mut World, mut stack: ItemStack) -> ItemStack {
        if let Some(MovingObjectPosition::Block { x, y, z, .. }) = self.item_ray10(w, true) {
            if self.can_edit_block181(x, y, z)
                && w.get_block_material(x, y, z) == crate::block::MAT_WATER
            {
                stack.count -= 1;
                if stack.count <= 0 {
                    return ItemStack::new(373, 1, 0);
                }
                let mut potion = ItemStack::new(373, 1, 0);
                if !self.player_state_mut().unwrap().inventory.add(&mut potion) {
                    self.drop_player_item(w, ItemStack::new(373, 1, 0), false);
                }
            }
        }
        stack
    }
    pub fn lily_pad_right_click10(&mut self, w: &mut World, mut stack: ItemStack) -> ItemStack {
        if let Some(MovingObjectPosition::Block { x, y, z, .. }) = self.item_ray10(w, true) {
            if self.can_edit_block181(x, y, z)
                && w.get_block_material(x, y, z) == crate::block::MAT_WATER
                && w.get_block_metadata(x, y, z) == 0
                && w.get_block_id(x, y + 1, z) == 0
            {
                w.set_block_with_notify(x, y + 1, z, 111);
                if !self.player_state().unwrap().capabilities.creative {
                    stack.count -= 1;
                }
            }
        }
        stack
    }
    pub fn drink_potion10(&mut self, w: &mut World, mut stack: ItemStack) -> ItemStack {
        stack.count -= 1;
        if !w.multiplayer_world {
            if let Some(effects) = ph::effects(stack.damage, false) {
                for e in effects {
                    self.add_potion_effect(e);
                }
            }
        }
        if stack.count <= 0 {
            ItemStack::new(374, 1, 0)
        } else {
            self.player_state_mut()
                .unwrap()
                .inventory
                .add(&mut ItemStack::new(374, 1, 0));
            stack
        }
    }
    pub fn drink_milk10(&mut self, w: &mut World, mut stack: ItemStack) -> ItemStack {
        stack.count -= 1;
        if !w.multiplayer_world {
            self.living.effects.clear();
        }
        if stack.count <= 0 {
            ItemStack::new(325, 1, 0)
        } else {
            stack
        }
    }
}
/// Shared callback boundary for ItemPotion/ItemEnderPearl right clicks, before
/// the entity's constructor. The caller owns the original Item.itemRand.
pub fn throw_item(
    stack: &mut ItemStack,
    rng: &mut crate::JavaRandom,
    multiplayer: bool,
    mut sound: impl FnMut(f32),
    mut spawn: impl FnMut(i32, i32),
) {
    stack.count = stack.count.wrapping_sub(1);
    let pitch = 0.4f32 / (rng.next_float() * 0.4f32 + 0.8f32);
    sound(pitch);
    if !multiplayer {
        spawn(stack.id, stack.damage);
    }
}
pub fn item_tool_efficiency(id: i32) -> f32 {
    match id {
        269..=271 => 2.0,
        273..=275 => 4.0,
        256..=258 => 6.0,
        277..=279 => 8.0,
        284..=286 => 12.0,
        _ => 1.0,
    }
}
/// ItemEnderPearl's creative guard occurs before the shared throw callback.
pub fn throw_ender_pearl(
    stack: &mut ItemStack,
    rng: &mut crate::JavaRandom,
    creative: bool,
    multiplayer: bool,
    sound: impl FnMut(f32),
    spawn: impl FnMut(i32, i32),
) {
    if !creative {
        throw_item(stack, rng, multiplayer, sound, spawn);
    }
}
impl Entity {
    pub fn insert_ender_eye10(
        &self,
        w: &mut World,
        stack: &mut ItemStack,
        x: i32,
        y: i32,
        z: i32,
    ) -> bool {
        let id = w.get_block_id(x, y, z);
        let meta = w.get_block_metadata(x, y, z);
        if !self.can_edit_block181(x, y, z) || id != 120 || meta & 4 != 0 {
            return false;
        }
        if w.multiplayer_world {
            return true;
        }
        w.set_block_metadata_with_notify(x, y, z, meta + 4);
        stack.count -= 1;
        for _ in 0..16 {
            let px = (x as f32 + (5.0 + w.item_random.next_float() * 6.0) / 16.0) as f64;
            let py = (y as f32 + 13.0 / 16.0) as f64;
            let pz = (z as f32 + (5.0 + w.item_random.next_float() * 6.0) / 16.0) as f64;
            w.spawn_particle("smoke", Vec3D::new(px, py, pz), Vec3D::default());
        }
        let d = (meta & 3) as usize;
        let side = (d + 1) % 4;
        let dx = [0, -1, 0, 1];
        let dz = [1, 0, -1, 0];
        let (mut first, mut last, mut found, mut valid) = (0, 0, false, true);
        for offset in -2..=2 {
            let (px, pz) = (x + dx[side] * offset, z + dz[side] * offset);
            if w.get_block_id(px, y, pz) == 120 {
                if w.get_block_metadata(px, y, pz) & 4 == 0 {
                    valid = false;
                    break;
                }
                if !found {
                    first = offset;
                    last = offset;
                    found = true;
                } else {
                    last = offset;
                }
            }
        }
        if valid && last == first + 2 {
            for offset in first..=last {
                let (px, pz) = (
                    x + dx[side] * offset + dx[d] * 4,
                    z + dz[side] * offset + dz[d] * 4,
                );
                let id = w.get_block_id(px, y, pz);
                let metadata = w.get_block_metadata(px, y, pz);
                if id != 120 || metadata & 4 == 0 {
                    valid = false;
                    break;
                }
            }
            for offset in (first - 1..=last + 1).step_by(4) {
                for forward in 1..=3 {
                    let (px, pz) = (
                        x + dx[side] * offset + dx[d] * forward,
                        z + dz[side] * offset + dz[d] * forward,
                    );
                    let id = w.get_block_id(px, y, pz);
                    let metadata = w.get_block_metadata(px, y, pz);
                    if id != 120 || metadata & 4 == 0 {
                        valid = false;
                        break;
                    }
                }
            }
            if valid {
                for offset in first..=last {
                    for forward in 1..=3 {
                        w.set_block_with_notify(
                            x + dx[side] * offset + dx[d] * forward,
                            y,
                            z + dz[side] * offset + dz[d] * forward,
                            119,
                        );
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn potion_cache_names_and_drink() {
        let mut p = PotionItem::default();
        let a = p.effects(8194).unwrap();
        assert!(Arc::ptr_eq(&a, &p.effects(8194).unwrap()));
        let mut applied = Vec::new();
        let result = p.drink(
            ItemStack::new(373, 1, 8194),
            false,
            |e| applied.push(e),
            |_| panic!(),
        );
        assert_eq!(result.id, 374);
        assert_eq!(applied, vec![PotionEffect::new(1, 3600, 0)]);
        assert_eq!(
            p.tooltip(8194, |s| s.into()),
            vec!["§7potion.moveSpeed (3:00)"]
        );
    }
}

pub const RECORD_NAMES: [&str; 11] = [
    "13", "cat", "blocks", "chirp", "far", "mall", "mellohi", "stal", "strad", "ward", "11",
];
pub fn record_tooltip(id: i32) -> Option<String> {
    RECORD_NAMES
        .get(id.wrapping_sub(2256) as usize)
        .map(|name| format!("C418 - {name}"))
}
pub fn item_properties(id: i32, damage: i32) -> Option<(i32, u32, i32)> {
    if (110..=122).contains(&id) {
        return Some((
            crate::world::blocks10::texture(id as u8, 2, damage).unwrap(),
            if id == 111 { 2129968 } else { 0xffffff },
            0,
        ));
    }
    if let Some((m, slot)) = crate::enchantment::armor(id) {
        return Some((
            ((id - 298) / 4) + slot as i32 * 16,
            0xffffff,
            m.durability(slot),
        ));
    }
    if id == 261 {
        return Some((21, 0xffffff, 384));
    }
    let icon = match id {
        372 => 125,
        373 => PotionItem::icon(damage),
        374 => 140,
        375 => 139,
        376 => 138,
        377 => 157,
        378 => 173,
        379 => 172,
        380 => 156,
        381 => 155,
        382 => 137,
        2258..=2266 => 240 + (id - 2256),
        351 => 78 + damage.clamp(0, 15) % 8 * 16 + damage.clamp(0, 15) / 8,
        _ => return None,
    };
    Some((
        icon,
        if id == 373 {
            ph::color(ph::effects(damage, false).as_deref()) as u32
        } else {
            0xffffff
        },
        0,
    ))
}

impl Entity {
    /// ItemEnderEye's Stronghold lookup and new entity are delegated to their owners.
    /// Coordinates truncate toward zero, unlike the block-placement floor helpers.
    pub fn ender_eye_right_click10(
        &self,
        world: &mut World,
        mut stack: ItemStack,
        mut find: impl FnMut(&mut World, &str, [i32; 3]) -> Option<[i32; 3]>,
        spawn: impl FnOnce(&mut World, Vec3D, [i32; 3]),
    ) -> ItemStack {
        if let Some(MovingObjectPosition::Block { x, y, z, .. }) = self.item_ray10(world, false) {
            if world.get_block_id(x, y, z) == 120 {
                return stack;
            }
        }
        if !world.multiplayer_world {
            if let Some(target) = find(
                world,
                "Stronghold",
                [self.pos.x as i32, self.pos.y as i32, self.pos.z as i32],
            ) {
                spawn(
                    world,
                    Vec3D::new(
                        self.pos.x,
                        self.pos.y + 1.62 - self.y_offset as f64,
                        self.pos.z,
                    ),
                    target,
                );
                let pitch = 0.4f32 / (world.item_random.next_float() * 0.4f32 + 0.8f32);
                world.play_sound("random.bow", self.pos, 0.5, pitch);
                world.auxiliary_effect(
                    1002,
                    (self.pos.x as i32, self.pos.y as i32, self.pos.z as i32),
                    0,
                );
                if !self.player_state().unwrap().capabilities.creative {
                    stack.count = stack.count.wrapping_sub(1);
                }
            }
        }
        stack
    }
}
