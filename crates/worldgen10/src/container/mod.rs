//! Literal Beta 1.8.1 Container/Slot rules. The world thread owns this object;
//! the client only receives snapshots and sends window-tagged commands.
pub mod recipes;
pub mod creative;
pub mod session;
pub mod tiles;
pub mod verify;
pub mod verify_interfaces;
use crate::entity::player::registry;
pub use crate::entity::player::{InventoryPlayer, ItemStack};
use recipes::CraftingManager;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ContainerKind {
    Player,
    Creative,
    Workbench,
    Chest { rows: usize },
    Furnace,
    Dispenser,
    BrewingStand,
    Enchantment,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SlotKind {
    Normal,
    Crafting,
    Armor(i32),
    FurnaceOutput,
    Potion,
    BrewingIngredient,
    Enchantment,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inventory {
    Main,
    Armor,
    Matrix,
    Result,
    Tile,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Slot {
    pub inventory: Inventory,
    pub index: usize,
    pub x: i32,
    pub y: i32,
    pub kind: SlotKind,
}
impl Slot {
    pub fn stack_limit(&self) -> i32 {
        if matches!(self.kind, SlotKind::Armor(_) | SlotKind::Potion | SlotKind::Enchantment) {
            1
        } else {
            64
        }
    }
    pub fn is_item_valid(&self, s: ItemStack) -> bool {
        match self.kind {
            SlotKind::Crafting | SlotKind::FurnaceOutput => false,
            SlotKind::Armor(t) => {
                CraftingManager::instance().armor_types.get(s.id as usize) == Some(&t)
                    || (s.id == 86 && t == 0)
            }
            SlotKind::Potion => brewing::potion_slot_accepts(Some(s)),
            SlotKind::BrewingIngredient => brewing::ingredient_slot_accepts(Some(s)),
            SlotKind::Enchantment | SlotKind::Normal => true,
        }
    }
    // SlotArmor does not override getBackgroundIconIndex in this version.
    pub fn background_icon(&self) -> i32 {
        -1
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ContainerEvent {
    Drop(ItemStack),
    Crafted { item: i32, count: i32 },
    Achievement(&'static str),
    MapCreated { id: i32 },
    Closed { window: i32 },
}
#[derive(Clone, Debug)]
pub struct Container {
    pub window: i32,
    pub kind: ContainerKind,
    pub slots: Vec<Slot>,
    pub matrix: Vec<Option<ItemStack>>,
    pub result: Option<ItemStack>,
    pub tile: Vec<Option<ItemStack>>,
    pub positions: Vec<(i32, i32, i32)>,
    pub vehicle: Option<i32>,
    pub progress: [i32; 3], // cook, burn, current fuel
    pub events: Vec<ContainerEvent>,
    pub transaction: i16,
    pub enchanting: Option<enchanting::Enchanting>,
    pub bookshelves: i32,
    pub multiplayer: bool,
    /// World.getUniqueDataId("map"); session copies this between windows.
    pub next_map_id: i32,
}
fn copy(s: ItemStack) -> ItemStack {
    s.copy_stack181()
}
fn same(a: ItemStack, b: ItemStack) -> bool {
    a.id == b.id && (!registry::item(a.id).is_some_and(|i| i.subtypes) || a.damage == b.damage)
}
impl Container {
    pub fn new(kind: ContainerKind, window: i32) -> Self {
        let mut c = Self {
            window,
            kind,
            slots: Vec::new(),
            matrix: Vec::new(),
            result: None,
            tile: Vec::new(),
            positions: Vec::new(),
            vehicle: None,
            progress: [0; 3],
            events: Vec::new(),
            transaction: 0,
            enchanting: None,bookshelves:0,multiplayer:false,
            next_map_id: -1,
        };
        let mut player_y = 84;
        match kind {
            ContainerKind::Creative => {
                c.tile=creative::contents(0.0);c.grid(Inventory::Tile,8,9,8,18);
                for x in 0..9 {c.add(Inventory::Main,x,8+x as i32*18,184,SlotKind::Normal);}return c;
            }
            ContainerKind::Player => {
                c.matrix = vec![None; 4];
                c.add(Inventory::Result, 0, 144, 36, SlotKind::Crafting);
                c.grid(Inventory::Matrix, 2, 2, 88, 26);
                for t in 0..4 {
                    c.add(
                        Inventory::Armor,
                        3 - t,
                        8,
                        8 + t as i32 * 18,
                        SlotKind::Armor(t as i32),
                    );
                }
            }
            ContainerKind::Workbench => {
                c.matrix = vec![None; 9];
                c.add(Inventory::Result, 0, 124, 35, SlotKind::Crafting);
                c.grid(Inventory::Matrix, 3, 3, 30, 17);
            }
            ContainerKind::Chest { rows } => {
                c.tile = vec![None; rows * 9];
                c.grid(Inventory::Tile, 9, rows, 8, 18);
                player_y = 103 + (rows as i32 - 4) * 18;
            }
            ContainerKind::Furnace => {
                c.tile = vec![None; 3];
                c.add(Inventory::Tile, 0, 56, 17, SlotKind::Normal);
                c.add(Inventory::Tile, 1, 56, 53, SlotKind::Normal);
                c.add(Inventory::Tile, 2, 116, 35, SlotKind::FurnaceOutput);
            }
            ContainerKind::BrewingStand => {
                c.tile=vec![None;4];
                for (index,x,y) in [(0,56,46),(1,79,53),(2,102,46)] {c.add(Inventory::Tile,index,x,y,SlotKind::Potion);}
                c.add(Inventory::Tile,3,79,17,SlotKind::BrewingIngredient);
            }
            ContainerKind::Enchantment => {
                c.tile=vec![None];
                c.enchanting=Some(enchanting::Enchanting::new(crate::entity::EntitySeedSource::Time.next_seed()));
                c.add(Inventory::Tile,0,25,47,SlotKind::Enchantment);
            }
            ContainerKind::Dispenser => {
                c.tile = vec![None; 9];
                c.grid(Inventory::Tile, 3, 3, 62, 17);
            }
        }
        for row in 0..3 {
            for x in 0..9 {
                c.add(
                    Inventory::Main,
                    x + row * 9 + 9,
                    8 + x as i32 * 18,
                    player_y + row as i32 * 18,
                    SlotKind::Normal,
                );
            }
        }
        for x in 0..9 {
            c.add(
                Inventory::Main,
                x,
                8 + x as i32 * 18,
                player_y + 58,
                SlotKind::Normal,
            );
        }
        c
    }
    fn add(&mut self, inventory: Inventory, index: usize, x: i32, y: i32, kind: SlotKind) {
        self.slots.push(Slot {
            inventory,
            index,
            x,
            y,
            kind,
        });
    }
    fn grid(&mut self, inventory: Inventory, width: usize, height: usize, x: i32, y: i32) {
        for row in 0..height {
            for col in 0..width {
                self.add(
                    inventory,
                    col + row * width,
                    x + col as i32 * 18,
                    y + row as i32 * 18,
                    SlotKind::Normal,
                );
            }
        }
    }
    pub fn size(&self) -> (i32, i32) {
        (
            176,
            match self.kind {
                ContainerKind::Chest { rows } => 114 + rows as i32 * 18,
                ContainerKind::Creative => 208,
                _ => 166,
            },
        )
    }
    pub fn stack(&self, p: &InventoryPlayer, slot: usize) -> Option<ItemStack> {
        let s = self.slots[slot];
        match s.inventory {
            Inventory::Main => p.main[s.index],
            Inventory::Armor => p.armor[s.index],
            Inventory::Matrix => self.matrix[s.index],
            Inventory::Result => self.result,
            Inventory::Tile => self.tile[s.index],
        }
    }
    fn store(&mut self, p: &mut InventoryPlayer, slot: usize, stack: Option<ItemStack>) {
        let s = self.slots[slot];
        match s.inventory {
            Inventory::Main => p.main[s.index] = stack,
            Inventory::Armor => p.armor[s.index] = stack,
            Inventory::Matrix => self.matrix[s.index] = stack,
            Inventory::Result => self.result = stack,
            Inventory::Tile => {
                self.tile[s.index] = stack;
                if let Some(e)=&mut self.enchanting {e.item=stack;}
            },
        }
    }
    pub fn put_stack(
        &mut self,
        p: &mut InventoryPlayer,
        slot: usize,
        mut stack: Option<ItemStack>,
    ) {
        if self.slots[slot].inventory == Inventory::Tile {
            if let Some(s) = &mut stack {
                s.count = s.count.min(if self.kind==ContainerKind::Enchantment {1}else{64});
            }
        }
        self.store(p, slot, stack);
        if self.kind == ContainerKind::Enchantment && slot == 0 {
            self.enchantment_inventory_changed10();
        }
        if self.slots[slot].inventory == Inventory::Matrix {
            self.matrix_changed();
        }
        self.changed(p, slot);
    }
    fn changed(&mut self, p: &mut InventoryPlayer, slot: usize) {
        if self.kind == ContainerKind::Enchantment && slot == 0 {
            self.enchantment_inventory_changed10();
        }
        if matches!(
            self.slots[slot].inventory,
            Inventory::Main | Inventory::Armor
        ) {
            p.changed = true;
        }
    }
    fn enchantment_inventory_changed10(&mut self) {
        let e = self.enchanting.as_mut().unwrap();
        e.changed(self.multiplayer, self.bookshelves);
        self.progress = e.costs;
    }
    pub fn matrix_changed(&mut self) {
        if !self.matrix.is_empty() {
            self.result = CraftingManager::instance().find_matching_recipe(
                &self.matrix,
                if self.kind == ContainerKind::Player {
                    2
                } else {
                    3
                },
            );
        }
    }
    fn decrement(&mut self, p: &mut InventoryPlayer, slot: usize, count: i32) -> Option<ItemStack> {
        let mut s = self.stack(p, slot)?;
        if s.count <= count || self.slots[slot].inventory == Inventory::Result || (self.kind==ContainerKind::BrewingStand && self.slots[slot].inventory==Inventory::Tile) {
            self.store(p, slot, None);
        } else {
            let part = s.split(count);
            self.store(p, slot, Some(s));
            s = part;
        }
        if self.slots[slot].inventory == Inventory::Matrix {
            self.matrix_changed();
        }
        if self.kind == ContainerKind::Enchantment && slot == 0 {
            self.enchantment_inventory_changed10();
        }
        Some(s)
    }
    fn pickup(&mut self, p: &mut InventoryPlayer, slot: usize, mut stack: ItemStack) -> ItemStack {
        let kind = self.slots[slot].kind;
        if kind==SlotKind::Potion && stack.id==373 && stack.damage>0 {
            self.events.push(ContainerEvent::Achievement("potion"));
        }
        if matches!(kind, SlotKind::Crafting | SlotKind::FurnaceOutput) {
            self.events.push(ContainerEvent::Crafted {
                item: stack.id,
                count: stack.count,
            });
            if stack.id == 358 {
                self.next_map_id = (self.next_map_id as i16).wrapping_add(1) as i32;
                stack.damage = self.next_map_id;
                self.events
                    .push(ContainerEvent::MapCreated { id: stack.damage });
            }
            let achievement = if kind == SlotKind::Crafting {
                match stack.id {
                    58 => Some("buildWorkBench"),
                    270 => Some("buildPickaxe"),
                    61 => Some("buildFurnace"),
                    290 => Some("buildHoe"),
                    297 => Some("makeBread"),
                    354 => Some("bakeCake"),
                    274 => Some("buildBetterPickaxe"),
                    268 => Some("buildSword"),
                    _ => None,
                }
            } else {
                match stack.id {
                    265 => Some("acquireIron"),
                    350 => Some("cookFish"),
                    _ => None,
                }
            };
            if let Some(a) = achievement {
                self.events.push(ContainerEvent::Achievement(a));
            }
        }
        if kind == SlotKind::Crafting {
            for i in 0..self.matrix.len() {
                if let Some(mut ingredient) = self.matrix[i] {
                    ingredient.count -= 1;
                    self.matrix[i] = (ingredient.count > 0).then_some(ingredient);
                    self.matrix_changed();
                    let id = CraftingManager::instance()
                        .container_items
                        .get(ingredient.id as usize)
                        .copied()
                        .unwrap_or(-1);
                    if id >= 0 {
                        self.matrix[i] = Some(ItemStack::new(id, 1, 0));
                        self.matrix_changed();
                    }
                }
            }
        } else {
            self.changed(p, slot);
        }
        stack
    }
    /// Beta merge ignores slot validity and slot limit, including the empty-slot pass.
    fn merge(
        &mut self,
        p: &mut InventoryPlayer,
        stack: &mut ItemStack,
        start: usize,
        end: usize,
        reverse: bool,
    ) {
        let indices: Vec<_> = if reverse {
            (start..end).rev().collect()
        } else {
            (start..end).collect()
        };
        if stack.stackable() {
            for &i in &indices {
                if stack.count <= 0 {
                    break;
                }
                if let Some(mut s) = self.stack(p, i) {
                    if same(*stack, s) {
                        let total = s.count.wrapping_add(stack.count);
                        if total <= stack.max_stack() {
                            stack.count = 0;
                            s.count = total;
                            self.store(p, i, Some(s));
                            self.changed(p, i);
                        } else if s.count < stack.max_stack() {
                            stack.count -= stack.max_stack() - s.count;
                            s.count = stack.max_stack();
                            self.store(p, i, Some(s));
                            self.changed(p, i);
                        }
                    }
                }
            }
        }
        if stack.count > 0 {
            for i in indices {
                if self.stack(p, i).is_none() {
                    self.put_stack(p, i, Some(copy(*stack)));
                    self.changed(p, i);
                    stack.count = 0;
                    break;
                }
            }
        }
    }
    fn transfer(&mut self, p: &mut InventoryPlayer, slot: usize) -> Option<ItemStack> {
        let mut s = self.stack(p, slot)?;
        if self.kind==ContainerKind::Creative {return Some(s);}
        let old = copy(s);
        if self.kind==ContainerKind::Enchantment && slot!=0 {
            return None;
        }
        let (start, end, reverse) = match self.kind {
            ContainerKind::BrewingStand => brewing::transfer_range(slot,s)?,
            ContainerKind::Enchantment => (1,37,true),
            ContainerKind::Dispenser => {
                if slot < 9 { (9, 45, true) } else { (0, 9, false) }
            }
            ContainerKind::Chest { rows } => {
                if slot < rows * 9 {
                    (rows * 9, self.slots.len(), true)
                } else {
                    (0, rows * 9, false)
                }
            }
            _ => {
                let first = match self.kind {
                    ContainerKind::Player => 9,
                    ContainerKind::Workbench => 10,
                    _ => 3,
                };
                if self.slots[slot].kind == SlotKind::Crafting
                    || self.slots[slot].kind == SlotKind::FurnaceOutput
                {
                    (first, first + 36, true)
                } else if slot >= first && slot < first + 27 {
                    (first + 27, first + 36, false)
                } else if slot >= first + 27 {
                    (first, first + 27, false)
                } else {
                    (first, first + 36, false)
                }
            }
        };
        self.merge(p, &mut s, start, end, reverse);
        if s.count == old.count { return None; }
        if s.count == 0 {
            self.put_stack(p, slot, None);
        } else {
            self.store(p, slot, Some(s));
            self.changed(p, slot);
        }
        if matches!(self.kind, ContainerKind::Chest { .. }) {
            return Some(old);
        }
        if s.count == old.count {
            return None;
        }
        self.pickup(p, slot, s);
        Some(old)
    }
    pub fn slot_click(
        &mut self,
        p: &mut InventoryPlayer,
        slot: i32,
        button: i32,
        shift: bool,
    ) -> Option<ItemStack> {
        if button != 0 && button != 1 {
            return None;
        }
        if self.kind==ContainerKind::Creative && slot!=-999 && slot<72 {return self.creative_click(p,slot,button,shift);}
        if slot == -999 {
            if let Some(mut s) = p.cursor.take() {
                let dropped = if button == 0 { s } else { s.split(1) };
                self.events.push(ContainerEvent::Drop(dropped));
                if button == 1 && s.count != 0 {
                    p.cursor = Some(s);
                }
            }
            return None;
        }
        if slot < 0 || slot as usize >= self.slots.len() {
            return None;
        }
        let slot = slot as usize;
        if shift && self.kind==ContainerKind::Creative {return self.transfer(p,slot);}
        if shift {
            let transferred = self.transfer(p, slot)?;
            let result = copy(transferred);
            // Beta 1.8.1 recurses while the replenished source retains the same item ID.
            if self
                .stack(p, slot)
                .is_some_and(|s| s.id == transferred.id)
            {
                self.slot_click(p, slot as i32, button, true);
            }
            return Some(result);
        }
        self.changed(p, slot);
        let before = self.stack(p, slot);
        let result = before.map(copy);
        match (before, p.cursor) {
            (None, Some(mut held)) if self.slots[slot].is_item_valid(held) => {
                let n =
                    (if button == 0 { held.count } else { 1 }).min(self.slots[slot].stack_limit());
                let part = held.split(n);
                self.put_stack(p, slot, Some(part));
                p.cursor = (held.count != 0).then_some(held);
            }
            (Some(s), None) => {
                let n = if button == 0 {
                    s.count
                } else {
                    (s.count + 1) / 2
                };
                p.cursor = self.decrement(p, slot, n);
                if let Some(cursor) = p.cursor {
                    p.cursor = Some(self.pickup(p, slot, cursor));
                }
            }
            (Some(mut s), Some(mut held)) if self.slots[slot].is_item_valid(held) => {
                if !same(s, held) {
                    if held.count <= self.slots[slot].stack_limit() {
                        self.put_stack(p, slot, Some(held));
                        p.cursor = Some(s);
                    }
                } else {
                    let n = (if button == 0 { held.count } else { 1 })
                        .min(self.slots[slot].stack_limit() - s.count)
                        .min(held.max_stack() - s.count);
                    held.split(n);
                    s.count += n;
                    p.cursor = (held.count != 0).then_some(held);
                    self.store(p, slot, Some(s)); // Java mutates in place; no matrix notification.
                }
            }
            (Some(mut s), Some(mut held)) if same(s, held) && held.max_stack() > 1 => {
                let n = s.count;
                if n > 0 && n + held.count <= held.max_stack() {
                    held.count += n;
                    s.split(n);
                    p.cursor = Some(held);
                    self.put_stack(p, slot, None);
                    p.cursor = Some(self.pickup(p, slot, held));
                }
            }
            _ => {}
        }
        result
    }
    pub fn close(&mut self, p: &mut InventoryPlayer, multiplayer: bool) {
        if let Some(s) = p.cursor.take() {
            self.events.push(ContainerEvent::Drop(s));
        }
        if self.kind == ContainerKind::Player
            || (self.kind == ContainerKind::Workbench && !multiplayer)
        {
            if self.kind==ContainerKind::Enchantment&&!multiplayer {
            if let Some(item)=self.tile[0] {self.events.push(ContainerEvent::Drop(item));}
        }
        for i in 0..self.matrix.len() {
                if let Some(s) = self.matrix[i] {
                    self.events.push(ContainerEvent::Drop(s));
                }
                if self.kind == ContainerKind::Player {
                    self.matrix[i] = None;
                    self.matrix_changed();
                }
            }
        }
    }
    pub fn next_transaction(&mut self) -> i16 {
        self.transaction = self.transaction.wrapping_add(1);
        self.transaction
    }
    pub fn snapshot(&self, p: &InventoryPlayer) -> ContainerSnapshot {
        ContainerSnapshot {
            inventory_name: if self.vehicle.is_some() {Some("Minecart")} else {None},
            window: self.window,
            kind: self.kind,
            slots: self.slots.clone(),
            contents: (0..self.slots.len()).map(|i| self.stack(p, i)).collect(),
            cursor: p.cursor,
            armor: p.armor,
            held: p.held(),
            player_inventory: p.clone(),
            creative: self.kind == ContainerKind::Creative,
            preview: PlayerPreview::default(),
            progress: self.progress,
            enchantment_name_seed: self.enchanting.as_ref().map_or(0, |e| e.name_seed),
            alive: true,
        }
    }
}
#[derive(Clone, Debug)]
pub struct ContainerSnapshot {
    pub enchantment_name_seed: i64,
    pub player_inventory: InventoryPlayer,
    pub creative: bool,
    pub inventory_name: Option<&'static str>,
    pub window: i32,
    pub kind: ContainerKind,
    pub slots: Vec<Slot>,
    pub contents: Vec<Option<ItemStack>>,
    pub cursor: Option<ItemStack>,
    pub armor: [Option<ItemStack>; 4],
    pub held: Option<ItemStack>,
    pub preview: PlayerPreview,
    pub progress: [i32; 3],
    pub alive: bool,
}
#[derive(Clone, Copy, Debug)]
pub struct PlayerPreview {
    pub brightness: f32,
    pub death_time: i32,
    pub age: f32,
    pub limb_swing: f32,
    pub limb_amount: f32,
    pub attack: f32,
    pub riding: bool,
    pub sneak: bool,
    pub hurt_time: i32,
}
impl Default for PlayerPreview {
    fn default() -> Self {
        Self {
            brightness: 1.0,
            death_time: 0,
            age: 1.0,
            limb_swing: 0.0,
            limb_amount: 0.0,
            attack: 0.0,
            riding: false,
            sneak: false,
            hurt_time: 0,
        }
    }
}
impl PlayerPreview {
    pub fn from_entity(e: &crate::entity::Entity) -> Self {
        Self {
            brightness: 1.0,
            death_time: e.living.death_time,
            age: e.ticks_existed as f32 + 1.0,
            limb_swing: e.living.limb_swing,
            limb_amount: e.living.prev_limb_amount
                + (e.living.limb_amount - e.living.prev_limb_amount),
            attack: e.swing_progress(1.0),
            riding: e.riding.is_some(),
            sneak: e.sneaking,
            hurt_time: e.living.hurt_time,
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum ContainerCommand {
    Enchant { window: i32, offer: usize },
    OpenVehicle { entity: i32 },
    OpenInventory,
    OpenBlock {
        block: u8,
        position: (i32, i32, i32),
    },
    Click {
        window: i32,
        slot: i32,
        button: i32,
        shift: bool,
    },
    ScrollCreative { window:i32, position:f32 },
    Close {
        window: i32,
    },
    /// GuiContainer.onGuiClosed calls onCraftGuiClosed before the controller
    /// calls it again. Vanilla keeps both callbacks, including their effects.
    CloseFromGui { window: i32 },
}
pub mod brewing;
pub mod enchanting;
