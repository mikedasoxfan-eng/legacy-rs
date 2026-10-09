use super::registry;
use crate::nbt::{Compound, Tag};

/// Java's ItemStack uses signed ints, including malformed NBT counts and damage.
#[derive(Clone, Copy, Debug)]
pub struct ItemStack {
    /// Java object identity survives value copies used to borrow the inventory.
    pub identity: u64,
    pub id: i32,
    pub count: i32,
    pub damage: i32,
    pub animations: i32,
}
impl PartialEq for ItemStack {
    fn eq(&self, other: &Self) -> bool {
        (self.id,self.count,self.damage,self.animations)==(other.id,other.count,other.damage,other.animations)
    }
}
impl Eq for ItemStack {}
impl ItemStack {
    pub fn new(id: i32, count: i32, damage: i32) -> Self {
        Self {
            identity: {
                static NEXT:std::sync::atomic::AtomicU64=std::sync::atomic::AtomicU64::new(1);
                NEXT.fetch_add(1,std::sync::atomic::Ordering::Relaxed)
            },
            id,
            count,
            damage,
            animations: 0,
        }
    }
    pub fn copy_stack(&self) -> Self {
        let copy = Self::new(self.id, self.count, self.damage);
        copy.copy_tag_from11(self);
        copy
    }
    pub fn same_object(self, other: Self) -> bool { self.identity == other.identity }
    pub fn max_stack(&self) -> i32 {
        self.max_stack_checked181().expect("ItemStack.getMaxStackSize")
    }
    pub fn max_damage(&self) -> i32 {
        self.max_damage_checked181().expect("ItemStack.getMaxDamage")
    }
    pub fn is_damaged(&self) -> bool {
        self.max_damage() > 0 && self.damage > 0
    }
    pub fn stackable(&self) -> bool {
        self.max_stack() > 1 && !self.is_damaged()
    }
    /// `damageItem`; true when the item broke (the caller adds the "broken" stat).
    pub fn damage_item(&mut self, amount: i32) -> bool {
        if self.max_damage() > 0 {
            self.damage = self.damage.wrapping_add(amount);
            if self.damage > self.max_damage() {
                self.count = self.count.wrapping_sub(1).max(0);
                self.damage = 0;
                return true;
            }
        }
        false
    }
    pub fn split(&mut self, count: i32) -> Self {
        self.count = self.count.wrapping_sub(count);
        let split=Self::new(self.id, count, self.damage);
        split.copy_tag_from11(self);
        split
    }
    pub fn to_nbt(&self) -> Compound {
        let mut n = Compound::new();
        self.write_nbt181(Some(&mut n)).expect("nonnull ItemStack NBT");
        n
    }
    /// ItemStack.func_35864_a: unregistered positive IDs read as null.
    pub fn try_from_nbt181(n:&Compound)->Option<Self>{
        Self::load_checked181(Some(n), |id| {
            if !(0..32000).contains(&id) {return Err(crate::entity::item_stack::StackError::ArrayIndexOutOfBounds);}
            Ok(super::registry::item(id).is_some())
        }).expect("ItemStack NBT load")
    }
    pub fn from_nbt(n: &Compound) -> Self {
        let mut stack=Self::new(0,0,0);
        stack.read_nbt_checked181(Some(n)).expect("ItemStack.readFromNBT");
        stack
    }
}
impl From<crate::world::tile::ItemStack> for ItemStack {
    fn from(s: crate::world::tile::ItemStack) -> Self {
        let mut stack=Self::new(s.id as i32, s.count as i32, s.damage as i32);stack.identity=s.identity;stack
    }
}
impl From<ItemStack> for crate::world::tile::ItemStack {
    fn from(s: ItemStack) -> Self {
        let mut stack=Self::new(s.id, s.count, s.damage);stack.identity=s.identity;stack
    }
}
#[derive(Clone, Debug)]
pub struct InventoryPlayer {
    pub main: [Option<ItemStack>; 36],
    pub armor: [Option<ItemStack>; 4],
    pub current_item: i32,
    pub cursor: Option<ItemStack>,
    pub changed: bool,
}
impl Default for InventoryPlayer {
    fn default() -> Self {
        Self {
            main: [None; 36],
            armor: [None; 4],
            current_item: 0,
            cursor: None,
            changed: false,
        }
    }
}
impl InventoryPlayer {
    /// InventoryPlayer.func_35157_d: search all main slots, including zero-count stacks.
    pub fn contains_item(&self, id: i32) -> bool {
        self.main.iter().any(|s| s.is_some_and(|s| s.id == id))
    }
    pub fn select_item(&mut self, id: i32, damage: i32, subtypes: bool, _creative: bool) {
        if let Some(slot) = self
            .main
            .iter()
            .position(|s| s.is_some_and(|s| s.id == id && (!subtypes || s.damage == damage)))
        {
            if slot < 9 {
                self.current_item = slot as i32;
            }
        }
    }
    pub fn copy_from_inventory(&mut self, other: &Self) {
        self.main = other.main.map(|s| s.map(|s| s.copy_stack()));
        self.armor = other.armor.map(|s| s.map(|s| s.copy_stack()));
    }
    pub fn add_with_creative(&mut self, item: &mut ItemStack, creative: bool) -> bool {
        let added = self.add(item);
        if !added && creative {
            item.count = 0;
            true
        } else {
            added
        }
    }
    pub fn set_current_item(&mut self, id: i32) {
        if let Some(slot) = self.main.iter().position(|s| s.is_some_and(|s| s.id == id)) {
            if slot < 9 { self.current_item = slot as i32; }
        }
    }
    pub fn held(&self) -> Option<ItemStack> {
        if (0..9).contains(&self.current_item) {
            self.main[self.current_item as usize]
        } else {
            None
        }
    }
    pub fn change_current_item(&mut self, delta: i32) {
        self.current_item = (self.current_item - delta.signum()).rem_euclid(9);
    }
    pub fn consume(&mut self, id: i32) -> bool {
        if let Some(slot) = self.main.iter_mut().find(|s| s.is_some_and(|s| s.id == id)) {
            let s = slot.as_mut().unwrap();
            s.count -= 1;
            if s.count <= 0 {
                *slot = None;
            }
            true
        } else {
            false
        }
    }
    pub fn decrement(&mut self, slot: usize, count: i32) -> Option<ItemStack> {
        let slot = if slot < 36 {
            &mut self.main[slot]
        } else {
            &mut self.armor[slot - 36]
        };
        if slot.as_ref()?.count <= count {
            slot.take()
        } else {
            let s = slot.as_mut().unwrap().split(count);
            if slot.as_ref().unwrap().count == 0 {
                *slot = None;
            }
            Some(s)
        }
    }
    fn store_partial(&mut self, item: &ItemStack) -> i32 {
        if item.max_stack() == 1 {
            if let Some(slot) = self.main.iter().position(Option::is_none) {
                self.main[slot] = Some(item.copy_stack());
                return 0;
            }
            return item.count;
        }
        let found = self.main.iter().position(|s| {
            s.is_some_and(|s| {
                s.id == item.id
                    && s.tags_equal(item)
                    && s.stackable()
                    && s.count < s.max_stack()
                    && s.count < 64
                    && (!registry::item(s.id).unwrap().subtypes || s.damage == item.damage)
            })
        });
        let Some(slot) = found.or_else(|| self.main.iter().position(Option::is_none)) else {
            return item.count;
        };
        if self.main[slot].is_none() {
            let s=ItemStack::new(item.id,0,item.damage);s.copy_tag_from11(item);self.main[slot]=Some(s);
        }
        let s = self.main[slot].as_mut().unwrap();
        let count = item.count.min(s.max_stack() - s.count).min(64 - s.count);
        if count == 0 {
            return item.count;
        }
        s.count += count;
        s.animations = 5;
        item.count - count
    }
    /// Preserves vanilla's return-value bug on a partial insertion that fills all slots.
    pub fn add(&mut self, item: &mut ItemStack) -> bool {
        if item.is_damaged() {
            if let Some(slot) = self.main.iter().position(Option::is_none) {
                let mut s = item.copy_stack181();
                s.animations = 5;
                self.main[slot] = Some(s);
                item.count = 0;
                true
            } else {
                false
            }
        } else {
            loop {
                let before = item.count;
                item.count = self.store_partial(item);
                if !(item.count > 0 && item.count < before) {
                    return item.count < before;
                }
            }
        }
    }
    pub fn tick_animations(&mut self) {
        for s in self.main.iter_mut().flatten() {
            if s.animations > 0 {
                s.animations -= 1;
            }
        }
    }
    pub fn armor_value(&self) -> i32 { self.armor.iter().flatten()
            .filter_map(|s| registry::item(s.id))
            .map(|i| i.armor).sum
            ()
    }
    /// `damageArmor`: returns the ids of pieces that broke, in slot order.
    pub fn damage_armor(&mut self, amount: i32) -> Vec<i32> {
        let amount = (amount / 4).max(1);
        let mut broken = Vec::new();
        for slot in &mut self.armor {
            if let Some(s) = slot {
                if registry::item(s.id).is_some_and(|i| i.armor > 0) {
                    if s.damage_item(amount) {
                        broken.push(s.id);
                    }
                    if s.count == 0 {
                        *slot = None;
                    }
                }
            }
        }
        broken
    }
    pub fn to_nbt(&self) -> Tag {
        let mut tags = Vec::new();
        for (slots, offset) in [(&self.main[..], 0), (&self.armor[..], 100)] {
            for (slot, s) in slots.iter().enumerate() {
                if let Some(s) = s {
                    let mut n = s.to_nbt();
                    n.put("Slot", Tag::Byte((slot + offset) as i8));
                    tags.push(Tag::Compound(n));
                }
            }
        }
        Tag::List(if tags.is_empty() { 1 } else { 10 }, tags)
    }
    pub fn read_nbt(&mut self, tags: &[Tag]) {
        self.main = [None; 36];
        self.armor = [None; 4];
        for t in tags {
            if let Tag::Compound(n) = t {
                let slot = n.byte("Slot") as u8 as usize;
                if let Some(s) = ItemStack::try_from_nbt181(n) {
                    if slot < 36 {
                        self.main[slot] = Some(s);
                    } else if (100..104).contains(&slot) {
                        self.armor[slot - 100] = Some(s);
                    }
                }
            }
        }
    }
}
