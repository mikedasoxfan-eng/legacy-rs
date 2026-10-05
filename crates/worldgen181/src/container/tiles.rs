//! TileEntityFurnace updateEntity, including consumed lava buckets (no replacement).
use super::{ItemStack, recipes::CraftingManager};
use crate::nbt::{Compound, Tag};
use crate::world::tile_nbt181::{self as tn, TileError181};
use crate::{
    World,
    entity::{Entity, EntityKind},
    world::{RandSel, tile::TileEntity},
};
pub fn read_items(n: &Compound, items: &mut [Option<ItemStack>]) {
    read_items181(n, items, false).expect("TileEntityDispenser.readFromNBT");
}
/// readFromNBT allocates a fresh array only after getTagList succeeds.
pub fn read_items181(n: &Compound, items: &mut [Option<ItemStack>], signed_slot: bool) -> Result<(), TileError181> {
    let list = tn::list(n, "Items")?;
    items.fill(None);
    for tag in list {
        let n = tn::compound(tag)?;
        let slot = tn::byte(n, "Slot")?;
        let index = if signed_slot { slot as i32 } else { slot as u8 as i32 };
        if index >= 0 && (index as usize) < items.len() {
            // Assign only after the helper returns, including its null result.
            items[index as usize] = tn::item(n)?;
        }
    }
    Ok(())
}
pub fn inventory_nbt(id: &str, x: i32, y: i32, z: i32, items: &[Option<ItemStack>]) -> Compound {
    let mut n = Compound::new();
    n.put("id", Tag::String(id.into()));
    for (key, value) in [("x", x), ("y", y), ("z", z)] {
        n.put(key, Tag::Int(value));
    }
    let items: Vec<_> = items
        .iter()
        .enumerate()
        .filter_map(|(i, s)| {
            s.map(|s| {
                let mut n = s.to_nbt();
                n.put("Slot", Tag::Byte(i as i8));
                Tag::Compound(n)
            })
        })
        .collect();
    n.put(
        "Items",
        Tag::List(if items.is_empty() { 1 } else { 10 }, items),
    );
    n
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Furnace {
    pub items: [Option<ItemStack>; 3],
    pub burn: i32,
    pub current_fuel: i32,
    pub cook: i32,
}
impl World {
    pub fn set_dispenser_seed181(&mut self, id: u64, seed: i64) {
        self.tile_objects.get_mut(&id).unwrap().dispenser_random181 = Some(crate::JavaRandom::new(seed));
        let (x,y,z)=self.tile_objects[&id].position;
        if self.chunks.get(&(x>>4,z>>4)).and_then(|c|c.tile_entity_ids.get(&(x&15,y,z&15)))==Some(&id) {
            self.dispenser_random.remove(&(x,y,z));
        }
    }
    /// TileEntityDispenser owns its Random, even after replacement or unloading.
    pub fn random_dispenser_stack181(&mut self, id: u64) -> Option<ItemStack> {
        let TileEntity::Dispenser(items) = self.tile_object(id).unwrap().clone() else { panic!("not a dispenser"); };
        let rng = self.tile_objects.get_mut(&id).unwrap().dispenser_random181.as_mut().unwrap();
        let mut selected = None;
        let mut count = 1;
        for (slot, item) in items.iter().enumerate() {
            if item.is_some() {
                if rng.next_int(count) == 0 { selected = Some(slot); }
                count += 1;
            }
        }
        selected.and_then(|slot| self.decrement_machine181(id, slot, 1))
    }
    pub fn tick_furnace181(&mut self, id: u64) {
        let Some(TileEntity::Furnace(mut furnace)) = self.tile_object(id).cloned() else { return; };
        let (transition, dirty) = furnace.tick(self.multiplayer_world);
        self.update_tile_object(id, TileEntity::Furnace(furnace));
        if let Some(lit) = transition {
            let (x, y, z) = self.tile_objects[&id].position;
            self.swap_furnace_block(lit, x, y, z);
        }
        if dirty { self.tile_inventory_changed181(id); }
    }
    /// Furnace and dispenser inventory callbacks differ in their dirty hook.
    pub fn machine_stack181(&self, id: u64, slot: usize) -> Option<ItemStack> {
        match self.tile_object(id).unwrap() {
            TileEntity::Furnace(f) => f.items[slot],
            TileEntity::Dispenser(items) => items[slot],
            _ => panic!("not a machine inventory"),
        }
    }
    pub fn set_machine_stack181(&mut self, id: u64, slot: usize, stack: Option<ItemStack>) {
        let mut tile = self.tile_object(id).unwrap().clone();
        let stack = stack.map(|mut s| { if s.count > 64 { s.count = 64; } s });
        let dispenser = matches!(&tile, TileEntity::Dispenser(_));
        match &mut tile {
            TileEntity::Furnace(f) => f.items[slot] = stack,
            TileEntity::Dispenser(items) => items[slot] = stack,
            _ => panic!("not a machine inventory"),
        }
        self.update_tile_object(id, tile);
        if dispenser { self.tile_inventory_changed181(id); }
    }
    pub fn decrement_machine181(&mut self, id: u64, slot: usize, amount: i32) -> Option<ItemStack> {
        let mut tile = self.tile_object(id).unwrap().clone();
        let dispenser = matches!(&tile, TileEntity::Dispenser(_));
        let items: &mut [Option<ItemStack>] = match &mut tile {
            TileEntity::Furnace(f) => &mut f.items,
            TileEntity::Dispenser(items) => &mut **items,
            _ => panic!("not a machine inventory"),
        };
        let mut stack = items[slot]?;
        let result = if stack.count <= amount {
            items[slot] = None;
            stack
        } else {
            let result = stack.split(amount);
            items[slot] = (stack.count != 0).then_some(stack);
            result
        };
        self.update_tile_object(id, tile);
        if dispenser { self.tile_inventory_changed181(id); }
        Some(result)
    }
    pub fn machine_can_interact181(&mut self, id: u64, position: crate::entity::Vec3D) -> bool {
        let (x,y,z) = self.tile_objects[&id].position;
        self.get_block_tile_entity(x,y,z);
        if self.chunk(x >> 4,z >> 4).tile_entity_ids.get(&(x & 15,y,z & 15)) != Some(&id) { return false; }
        let dx = position.x - (x as f64 + 0.5);
        let dy = position.y - (y as f64 + 0.5);
        let dz = position.z - (z as f64 + 0.5);
        // Original dcmpl/ifle admits NaN (the decompiler's <= does not).
        !(dx * dx + dy * dy + dz * dz > 64.0)
    }
    pub fn mark_container_dirty(&mut self, x: i32, z: i32) {
        if let Some(c) = self.chunks.get_mut(&(x >> 4, z >> 4)) {
            c.modified = true;
        }
    }
    pub fn swap_furnace_block(&mut self, lit: bool, x: i32, y: i32, z: i32) {
        let metadata = self.get_block_metadata(x, y, z);
        self.get_block_tile_entity(x,y,z);
        let tile_id = self.chunk(x >> 4,z >> 4).tile_entity_ids.get(&(x & 15,y,z & 15)).copied();
        self.keep_furnace_inventory = true;
        self.set_block_with_notify(x, y, z, if lit { 62 } else { 61 });
        self.keep_furnace_inventory = false;
        self.set_block_metadata_with_notify(x, y, z, metadata);
        if let Some(id) = tile_id {
            self.tile_objects.get_mut(&id).unwrap().invalid = false;
            self.set_tile_object(id);
        }
    }
    pub fn dispense_item(&mut self, x: i32, y: i32, z: i32, random: RandSel) {
        let meta = self.get_block_metadata(x, y, z);
        let (dx, dz) = match meta {
            3 => (0, 1),
            2 => (0, -1),
            5 => (1, 0),
            _ => (-1, 0),
        };
        if self.get_block_tile_entity(x, y, z).is_none(){return;}
        let key = (x & 15, y, z & 15);
        let id = self.chunk(x >> 4, z >> 4).tile_entity_ids[&key];
        // Existing replay seed injection remains a one-call override; the object
        // retains the resulting state when the position map no longer owns it.
        if let Some(rng) = self.dispenser_random.remove(&(x, y, z)) {
            self.tile_objects.get_mut(&id).unwrap().dispenser_random181 = Some(rng);
        }
        let stack = self.random_dispenser_stack181(id);
        self.dispenser_random.insert((x, y, z), self.tile_objects[&id].dispenser_random181.clone().unwrap());
        let px = x as f64 + dx as f64 * 0.6 + 0.5;
        let py = y as f64 + 0.5;
        let pz = z as f64 + dz as f64 * 0.6 + 0.5;
        if let Some(s) = stack {
            let mut e = match s.id {
                262 => {
                    let mut e = Entity::arrow(self, px, py, pz);
                    if let EntityKind::Arrow(a) = &mut e.kind {
                        a.belongs_to_player = true;
                    }
                    e
                }
                344 | 332 => Entity::throwable_at(self, px, py, pz, s.id == 344),
                _ => Entity::item(self, px, py - 0.3, pz, s.into()),
            };
            let effect = if matches!(s.id, 262 | 344 | 332) {
                e.set_arrow_heading(dx as f64, 0.1f32 as f64, dz as f64, 1.1, 6.0);
                1002
            } else {
                let velocity = self.rng(random).next_double() * 0.1 + 0.2;
                e.motion.x = dx as f64 * velocity;
                e.motion.y = 0.2f32 as f64;
                e.motion.z = dz as f64 * velocity;
                e.motion.x += self.rng(random).next_gaussian() * 0.0075f32 as f64 * 6.0;
                e.motion.y += self.rng(random).next_gaussian() * 0.0075f32 as f64 * 6.0;
                e.motion.z += self.rng(random).next_gaussian() * 0.0075f32 as f64 * 6.0;
                1000
            };
            self.entity_joined_world(e);
            self.container_effects.push((effect, (x, y, z), 0));
            self.auxiliary_effect(effect, (x, y, z), 0);
            self.container_effects
                .push((2000, (x, y, z), dx + 1 + (dz + 1) * 3));
            self.auxiliary_effect(2000, (x, y, z), dx + 1 + (dz + 1) * 3);
            self.mark_container_dirty(x, z);
        } else {
            self.container_effects.push((1001, (x, y, z), 0));
            self.auxiliary_effect(1001, (x, y, z), 0);
        }
    }
    pub fn scatter_machine_contents(&mut self, block: u8, x: i32, y: i32, z: i32) {
        let items: Vec<_> = match self.get_block_tile_entity(x, y, z) {
            Some(TileEntity::Furnace(f)) => f.items.to_vec(),
            Some(TileEntity::Dispenser(items)) => items.to_vec(),
            _ => return,
        };
        let index = match block {
            61 => 0,
            62 => 1,
            _ => 2,
        };
        for (slot, item) in items.into_iter().enumerate() {
            let Some(item) = item else { continue; };
            let ox = self.container_drop_random[index].next_float() * 0.8 + 0.1;
            let oy = self.container_drop_random[index].next_float() * 0.8 + 0.1;
            let oz = self.container_drop_random[index].next_float() * 0.8 + 0.1;
            let mut count = item.count;
            while count > 0 {
                let n = (self.container_drop_random[index].next_int(21) + 10).min(count);
                count -= n;
                // ItemStack.stackSize is mutated in place even when the tile
                // is about to be invalidated and detached from the chunk.
                match self.get_block_tile_entity(x,y,z) {
                    Some(TileEntity::Furnace(f)) => f.items[slot].as_mut().unwrap().count = count,
                    Some(TileEntity::Dispenser(items)) => items[slot].as_mut().unwrap().count = count,
                    _ => (),
                }
                let mut e = Entity::item(
                    self,
                    (x as f32 + ox) as f64,
                    (y as f32 + oy) as f64,
                    (z as f32 + oz) as f64,
                    ItemStack::new(item.id, n, item.damage).into(),
                );
                e.motion.x =
                    (self.container_drop_random[index].next_gaussian() as f32 * 0.05) as f64;
                e.motion.y =
                    (self.container_drop_random[index].next_gaussian() as f32 * 0.05 + 0.2) as f64;
                e.motion.z =
                    (self.container_drop_random[index].next_gaussian() as f32 * 0.05) as f64;
                self.entity_joined_world(e);
            }
        }
    }
}
impl Furnace {
    pub fn from_nbt(n: &Compound) -> Self {
        let mut f = Self::default();
        f.read_from_nbt181(n).expect("TileEntityFurnace.readFromNBT");
        f
    }
    pub fn read_from_nbt181(&mut self, n: &Compound) -> Result<(), TileError181> {
        read_items181(n, &mut self.items, true)?;
        self.burn = tn::short(n, "BurnTime")? as i32;
        self.cook = tn::short(n, "CookTime")? as i32;
        self.current_fuel = CraftingManager::instance().fuel(self.items[1]);
        Ok(())
    }
    /// IInventory.openChest/closeChest are empty in the original furnace.
    pub fn open_chest181(&mut self) {}
    pub fn close_chest181(&mut self) {}
    pub fn to_nbt(&self, x: i32, y: i32, z: i32) -> Compound {
        let mut n = inventory_nbt("Furnace", x, y, z, &self.items);
        n.put("BurnTime", Tag::Short(self.burn as i16));
        n.put("CookTime", Tag::Short(self.cook as i16));
        n
    }
    pub fn can_smelt(&self) -> bool {
        let Some(result) =
            self.items[0].and_then(|s| CraftingManager::instance().smelting_result(s.id))
        else {
            return false;
        };
        let Some(out) = self.items[2] else {
            return true;
        };
        out.id == result.id
            && out.damage == result.damage
            && ((out.count < 64 && out.count < out.max_stack()) || out.count < result.max_stack())
    }
    pub fn smelt(&mut self) {
        if !self.can_smelt() {
            return;
        }
        let mut input = self.items[0].unwrap();
        let result = CraftingManager::instance()
            .smelting_result(input.id)
            .unwrap();
        if let Some(out) = &mut self.items[2] {
            if out.id == result.id {
            out.count = out.count.wrapping_add(1);
            }
        } else {
            self.items[2] = Some(result);
        }
        input.count = input.count.wrapping_sub(1);
        self.items[0] = (input.count > 0).then_some(input);
    }
    /// Returns (block-state transition, inventory dirty).
    pub fn tick(&mut self, multiplayer: bool) -> (Option<bool>, bool) {
        let burning = self.burn > 0;
        let mut dirty = false;
        if self.burn > 0 {
            self.burn -= 1;
        }
        if !multiplayer {
            if self.burn == 0 && self.can_smelt() {
                self.current_fuel = CraftingManager::instance().fuel(self.items[1]);
                self.burn = self.current_fuel;
                if self.burn > 0 {
                    dirty = true;
                    if let Some(mut s) = self.items[1] {
                        s.count = s.count.wrapping_sub(1);
                        self.items[1] = (s.count != 0).then_some(s);
                    }
                }
            }
            if self.burn > 0 && self.can_smelt() {
                self.cook = self.cook.wrapping_add(1);
                if self.cook == 200 {
                    self.cook = 0;
                    self.smelt();
                    dirty = true;
                }
            } else {
                self.cook = 0;
            }
            if burning != (self.burn > 0) {
                return (Some(self.burn > 0), true);
            }
        }
        (None, dirty)
    }
    pub fn cook_scaled(&self, scale: i32) -> i32 {
        self.cook.wrapping_mul(scale) / 200
    }
    pub fn burn_scaled(&mut self, scale: i32) -> i32 {
        if self.current_fuel == 0 {
            self.current_fuel = 200;
        }
        self.burn.wrapping_mul(scale).wrapping_div(self.current_fuel)
    }
}
