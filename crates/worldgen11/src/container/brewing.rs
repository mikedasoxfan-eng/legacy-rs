//! TileEntityBrewingStand and the brewing container's simulation rules (1.0).
use crate::{
    entity::player::ItemStack,
    nbt::{Compound, Tag},
    potion_helper as ph,
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct BrewingStand {
    pub items: [Option<ItemStack>; 4],
    pub brew_time: i32,
    pub bottle_metadata: i32,
    pub ingredient_id: i32,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BrewingUpdate {
    pub dirty: bool,
    pub metadata: Option<i32>,
}
pub fn splash(damage: i32) -> bool {
    damage & 16384 != 0
}
fn changed(old: i32, new: i32) -> bool {
    if !splash(old) && splash(new) {
        return true;
    }
    let a = crate::item10::potion_effects_snapshot(old);
    let b = crate::item10::potion_effects_snapshot(new);
    // The cache returns the same List for the same metadata; all null lists
    // are also reference-equal. Otherwise List.equals compares PotionEffects.
    let same_reference = old == new || (a.is_none() && b.is_none());
    (old <= 0 || !same_reference) && (a.is_none() || (a != b && b.is_some())) && old != new
}
impl BrewingStand {
    pub fn get(&self, slot: i32) -> Option<ItemStack> {
        if (0..4).contains(&slot) {
            self.items[slot as usize]
        } else {
            None
        }
    }
    /// In 1.0 this removes the entire slot, ignoring the requested count.
    pub fn decrement(&mut self, slot: i32, _count: i32) -> Option<ItemStack> {
        if (0..4).contains(&slot) {
            self.items[slot as usize].take()
        } else {
            None
        }
    }
    pub fn set(&mut self, slot: i32, stack: Option<ItemStack>) {
        if (0..4).contains(&slot) {
            self.items[slot as usize] = stack;
        }
    }
    pub fn metadata(&self) -> i32 {
        self.items[..3]
            .iter()
            .enumerate()
            .fold(0, |n, (i, s)| n | if s.is_some() { 1 << i } else { 0 })
    }
    pub fn can_brew(&self) -> bool {
        let Some(s) = self.items[3].filter(|s| s.count > 0) else {
            return false;
        };
        let Some(recipe) = ph::ingredient(s.id) else {
            return false;
        };
        self.items[..3]
            .iter()
            .flatten()
            .any(|s| s.id == 373 && changed(s.damage, ph::brew(s.damage, recipe)))
    }
    pub fn brew(&mut self) {
        if !self.can_brew() {
            return;
        }
        let ingredient = self.items[3].unwrap();
        let recipe = ph::ingredient(ingredient.id).unwrap();
        for s in self.items[..3].iter_mut().flatten() {
            if s.id != 373 {
                continue;
            }
            let old = s.damage;
            let new = ph::brew(old, recipe);
            let a = crate::item10::potion_effects_snapshot(old);
            let b = crate::item10::potion_effects_snapshot(new);
            let same_reference = old == new || (a.is_none() && b.is_none());
            if (old > 0 && same_reference) || (a.is_some() && (a == b || b.is_none())) {
                if !splash(old) && splash(new) {
                    s.damage = new;
                }
            } else if old != new {
                s.damage = new;
            }
        }
        let container = super::recipes::CraftingManager::instance()
            .container_items
            .get(ingredient.id as usize)
            .copied()
            .unwrap_or(-1);
        if container >= 0 {
            self.items[3] = Some(ItemStack::new(container, 1, 0));
        } else {
            let s = self.items[3].as_mut().unwrap();
            s.count = s.count.wrapping_sub(1);
            if s.count <= 0 {
                self.items[3] = None;
            }
        }
    }
    pub fn tick(&mut self) -> BrewingUpdate {
        let mut update = BrewingUpdate::default();
        if self.brew_time > 0 {
            self.brew_time -= 1;
            if self.brew_time == 0 {
                self.brew();
                update.dirty = true;
            } else if !self.can_brew() || self.ingredient_id != self.items[3].unwrap().id {
                self.brew_time = 0;
                update.dirty = true;
            }
        } else if self.can_brew() {
            self.brew_time = 600;
            self.ingredient_id = self.items[3].unwrap().id;
        }
        let metadata = self.metadata();
        if metadata != self.bottle_metadata {
            self.bottle_metadata = metadata;
            update.metadata = Some(metadata);
        }
        update
    }
    pub fn read_nbt(
        &mut self,
        n: &Compound,
    ) -> Result<(), crate::world::tile_nbt181::TileError181> {
        super::tiles::read_items181(n, &mut self.items, true)?;
        self.brew_time = crate::world::tile_nbt181::short(n, "BrewTime")? as i32;
        Ok(())
    }
    pub fn to_nbt(&self, position: (i32, i32, i32)) -> Compound {
        let mut n = super::tiles::inventory_nbt(
            "Cauldron",
            position.0,
            position.1,
            position.2,
            &self.items,
        );
        n.put("BrewTime", Tag::Short(self.brew_time as i16));
        n
    }
}
pub fn potion_slot_accepts(stack: Option<ItemStack>) -> bool {
    stack.is_some_and(|s| matches!(s.id, 373 | 374))
}
pub fn ingredient_slot_accepts(stack: Option<ItemStack>) -> bool {
    stack.is_some_and(|s| ph::ingredient(s.id).is_some())
}
/// ContainerBrewingStand.transferStackInSlot destination ranges, end exclusive.
pub fn transfer_range(index: usize, _stack: ItemStack) -> Option<(usize, usize, bool)> {
    if index < 4 {
        Some((4, 40, true))
    } else if index < 31 {
        Some((31, 40, false))
    } else if index < 40 {
        Some((4, 31, false))
    } else {
        Some((4, 40, false))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn full_brew_and_abort() {
        let mut b = BrewingStand::default();
        b.items[0] = Some(ItemStack::new(373, 1, 0));
        b.items[3] = Some(ItemStack::new(372, 2, 0));
        assert_eq!(b.tick().metadata, Some(1));
        assert_eq!(b.brew_time, 600);
        for _ in 0..600 {
            b.tick();
        }
        assert_eq!(b.items[0].unwrap().damage, 16);
        assert_eq!(b.items[3].unwrap().count, 1);
        b.items[3] = Some(ItemStack::new(353, 1, 0));
        b.tick();
        b.items[3] = Some(ItemStack::new(370, 1, 0));
        assert!(b.tick().dirty);
        assert_eq!(b.brew_time, 0);
    }
    #[test]
    fn slot_and_nbt_quirks() {
        let mut b = BrewingStand::default();
        b.set(3, Some(ItemStack::new(353, 12, 0)));
        assert_eq!(b.decrement(3, 1).unwrap().count, 12);
        b.brew_time = 600;
        b.ingredient_id = 353;
        let mut loaded = BrewingStand::default();
        loaded.read_nbt(&b.to_nbt((1, 2, 3))).unwrap();
        assert_eq!(loaded.brew_time, 600);
        assert_eq!(loaded.ingredient_id, 0);
    }
}
