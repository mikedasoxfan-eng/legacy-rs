use super::registry;
use crate::nbt::{Compound, Tag};

/// Java's ItemStack uses signed ints, including malformed NBT counts and damage.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemStack { pub id: i32, pub count: i32, pub damage: i32, pub animations: i32 }
impl ItemStack {
    pub fn new(id: i32, count: i32, damage: i32) -> Self { Self { id, count, damage, animations: 0 } }
    pub fn max_stack(&self) -> i32 { registry::item(self.id).map_or(64, |i| i.max_stack) }
    pub fn max_damage(&self) -> i32 { registry::item(self.id).map_or(0, |i| i.durability) }
    pub fn is_damaged(&self) -> bool { self.max_damage()>0 && self.damage>0 }
    pub fn stackable(&self) -> bool { self.max_stack()>1 && !self.is_damaged() }
    pub fn damage_item(&mut self, amount: i32) {
        if self.max_damage()>0 {
            self.damage = self.damage.wrapping_add(amount);
            if self.damage>self.max_damage() { self.count=(self.count-1).max(0); self.damage=0; }
        }
    }
    pub fn split(&mut self, count: i32) -> Self { self.count-=count; Self::new(self.id,count,self.damage) }
    pub fn to_nbt(&self) -> Compound {
        let mut n=Compound::new(); n.put("id",Tag::Short(self.id as i16)); n.put("Count",Tag::Byte(self.count as i8)); n.put("Damage",Tag::Short(self.damage as i16)); n
    }
    pub fn from_nbt(n: &Compound) -> Self { Self::new(n.short("id") as i32,n.byte("Count") as i32,n.short("Damage") as i32) }
}
impl From<crate::world::tile::ItemStack> for ItemStack {
    fn from(s: crate::world::tile::ItemStack) -> Self { Self::new(s.id as i32,s.count as i32,s.damage as i32) }
}
impl From<ItemStack> for crate::world::tile::ItemStack {
    fn from(s: ItemStack) -> Self { Self::new(s.id,s.count,s.damage) }
}
#[derive(Clone, Debug)]
pub struct InventoryPlayer {
    pub main: [Option<ItemStack>;36], pub armor: [Option<ItemStack>;4], pub current_item: i32,
    pub cursor: Option<ItemStack>, pub changed: bool,
}
impl Default for InventoryPlayer {
    fn default()->Self { Self { main:[None;36],armor:[None;4],current_item:0,cursor:None,changed:false } }
}
impl InventoryPlayer {
    pub fn held(&self)->Option<ItemStack> { if (0..9).contains(&self.current_item) { self.main[self.current_item as usize] } else { None } }
    pub fn change_current_item(&mut self, delta:i32) { self.current_item=(self.current_item-delta.signum()).rem_euclid(9); }
    pub fn consume(&mut self,id:i32)->bool {
        if let Some(slot)=self.main.iter_mut().find(|s|s.is_some_and(|s|s.id==id)) {
            let s=slot.as_mut().unwrap(); s.count-=1; if s.count<=0 { *slot=None; } true
        } else { false }
    }
    pub fn decrement(&mut self, slot:usize, count:i32)->Option<ItemStack> {
        let slot=if slot<36 { &mut self.main[slot] } else { &mut self.armor[slot-36] };
        if slot.as_ref()?.count<=count { slot.take() } else { let s=slot.as_mut().unwrap().split(count); if slot.as_ref().unwrap().count==0 { *slot=None; } Some(s) }
    }
    fn store_partial(&mut self,item:&ItemStack)->i32 {
        let found=self.main.iter().position(|s|s.is_some_and(|s|s.id==item.id && s.stackable() && s.count<s.max_stack() && s.count<64 && (!registry::item(s.id).unwrap().subtypes || s.damage==item.damage)));
        let Some(slot)=found.or_else(||self.main.iter().position(Option::is_none)) else {return item.count;};
        let s=self.main[slot].get_or_insert(ItemStack::new(item.id,0,item.damage));
        let count=item.count.min(s.max_stack()-s.count).min(64-s.count);
        if count==0 { return item.count; }
        s.count+=count; s.animations=5; item.count-count
    }
    /// Preserves vanilla's return-value bug on a partial insertion that fills all slots.
    pub fn add(&mut self,item:&mut ItemStack)->bool {
        if item.is_damaged() {
            if let Some(slot)=self.main.iter().position(Option::is_none) { let mut s=ItemStack::new(item.id,item.count,item.damage); s.animations=5; self.main[slot]=Some(s); item.count=0; true } else {false}
        } else {
            loop { let before=item.count; item.count=self.store_partial(item); if !(item.count>0 && item.count<before) { return item.count<before; } }
        }
    }
    pub fn tick_animations(&mut self) { for s in self.main.iter_mut().flatten() { if s.animations>0 {s.animations-=1;} } }
    pub fn armor_value(&self)->i32 {
        let (mut protection,mut remaining,mut total)=(0,0,0);
        for s in self.armor.iter().flatten() { if let Some(i)=registry::item(s.id) { if i.armor>0 {protection+=i.armor; remaining+=i.durability-s.damage; total+=i.durability;} } }
        if total==0 {0} else {(protection-1)*remaining/total+1}
    }
    pub fn damage_armor(&mut self,amount:i32) { for slot in &mut self.armor { if let Some(s)=slot { if registry::item(s.id).is_some_and(|i|i.armor>0) { s.damage_item(amount); if s.count==0 {*slot=None;} } } } }
    pub fn to_nbt(&self)->Tag {
        let mut tags=Vec::new();
        for (slots,offset) in [(&self.main[..],0),(&self.armor[..],100)] { for (slot,s) in slots.iter().enumerate() { if let Some(s)=s { let mut n=s.to_nbt(); n.put("Slot",Tag::Byte((slot+offset) as i8)); tags.push(Tag::Compound(n)); } } }
        Tag::List(10,tags)
    }
    pub fn read_nbt(&mut self,tags:&[Tag]) {
        self.main=[None;36]; self.armor=[None;4];
        for t in tags { if let Tag::Compound(n)=t {let slot=n.byte("Slot") as u8 as usize; let s=ItemStack::from_nbt(n); if registry::item(s.id).is_some() { if slot<36 {self.main[slot]=Some(s);} else if (100..104).contains(&slot) {self.armor[slot-100]=Some(s);} } } }
    }
}
