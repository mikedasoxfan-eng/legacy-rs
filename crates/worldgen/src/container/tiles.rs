//! TileEntityFurnace updateEntity, including consumed lava buckets (no replacement).
use super::{recipes::CraftingManager, ItemStack};
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Furnace {
    pub items: [Option<ItemStack>; 3],
    pub burn: i32,
    pub current_fuel: i32,
    pub cook: i32,
}
impl Furnace {
    pub fn can_smelt(&self) -> bool {
        let Some(result) = self.items[0].and_then(|s| CraftingManager::instance().smelting_result(s.id)) else { return false };
        let Some(out) = self.items[2] else { return true };
        out.id == result.id && out.damage == result.damage && ((out.count < 64 && out.count < out.max_stack()) || out.count < result.max_stack())
    }
    pub fn smelt(&mut self) {
        if !self.can_smelt() { return }
        let mut input = self.items[0].unwrap();
        let result = CraftingManager::instance().smelting_result(input.id).unwrap();
        if let Some(out) = &mut self.items[2] { if out.id == result.id { out.count += 1; } } else { self.items[2] = Some(result); }
        input.count -= 1;
        self.items[0] = (input.count > 0).then_some(input);
    }
    /// Returns (block-state transition, inventory dirty).
    pub fn tick(&mut self, multiplayer: bool) -> (Option<bool>, bool) {
        let burning = self.burn > 0;
        let mut dirty = false;
        if self.burn > 0 { self.burn -= 1; }
        if !multiplayer {
            if self.burn == 0 && self.can_smelt() {
                self.current_fuel = CraftingManager::instance().fuel(self.items[1]);
                self.burn = self.current_fuel;
                if self.burn > 0 {
                    dirty = true;
                    if let Some(mut s) = self.items[1] { s.count -= 1; self.items[1] = (s.count != 0).then_some(s); }
                }
            }
            if self.burn > 0 && self.can_smelt() {
                self.cook += 1;
                if self.cook == 200 { self.cook = 0; self.smelt(); dirty = true; }
            } else { self.cook = 0; }
            if burning != (self.burn > 0) { return (Some(self.burn > 0), true) }
        }
        (None, dirty)
    }
    pub fn cook_scaled(&self, scale: i32) -> i32 { self.cook.wrapping_mul(scale) / 200 }
    pub fn burn_scaled(&mut self, scale: i32) -> i32 {
        if self.current_fuel == 0 { self.current_fuel = 200; }
        self.burn.wrapping_mul(scale) / self.current_fuel
    }
}
