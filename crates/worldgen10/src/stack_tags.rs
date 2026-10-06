//! NBT attached to the existing Java ItemStack identity. Borrowed value copies
//! retain identity; Java copy/split explicitly clone the compound to a new one.
//! This follows the existing identity arena used by the simulation's inventories.
use crate::{
    enchantment::EnchantmentData,
    entity::player::ItemStack,
    nbt::{Compound, Tag},
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, OnceLock},
};
/// A Java NBTTagCompound reference. Snapshot reads are available through tag();
/// callers retaining the reference can mutate it through this handle.
pub type StackTag = Arc<Mutex<Compound>>;
fn tags() -> &'static Mutex<HashMap<u64, StackTag>> {
    static TAGS: OnceLock<Mutex<HashMap<u64, StackTag>>> = OnceLock::new();
    TAGS.get_or_init(|| Mutex::new(HashMap::new()))
}
impl ItemStack {
    pub fn tag(&self) -> Option<Compound> {
        self.tag_handle().map(|tag| tag.lock().unwrap_or_else(|e| e.into_inner()).clone())
    }
    pub fn tag_handle(&self) -> Option<StackTag> {
        tags().lock().unwrap_or_else(|e| e.into_inner()).get(&self.identity).cloned()
    }
    pub fn set_tag(&self, tag: Option<Compound>) {
        self.set_tag_handle(tag.map(|tag| Arc::new(Mutex::new(tag))));
    }
    pub fn set_tag_handle(&self, tag: Option<StackTag>) {
        let mut tags = tags().lock().unwrap_or_else(|e| e.into_inner());
        if let Some(tag) = tag {
            tags.insert(self.identity, tag);
        } else {
            tags.remove(&self.identity);
        }
    }
    /// The public Java tag setter rejects stackable items. NBT loading and
    /// copying assign the field directly and use set_tag above.
    pub fn set_tag_checked(&self, tag: Option<Compound>) {
        self.set_tag_handle_checked(tag.map(|tag| Arc::new(Mutex::new(tag))));
    }
    pub fn set_tag_handle_checked(&self, tag: Option<StackTag>) {
        assert_eq!(
            self.max_stack(),
            1,
            "IllegalArgumentException: Cannot add tag data to a stackable item"
        );
        self.set_tag_handle(tag);
    }
    pub fn tags_equal(&self, other: &Self) -> bool {
        if let (Some(a), Some(b)) = (self.tag_handle(), other.tag_handle()) {
            if Arc::ptr_eq(&a, &b) { return true; }
        }
        match (self.tag(), other.tag()) {
            (None, None) => true,
            (Some(a), Some(b)) => compound_equal(&a, &b),
            _ => false,
        }
    }
    pub fn has_tag(&self) -> bool {
        tags().lock().unwrap_or_else(|e| e.into_inner()).contains_key(&self.identity)
    }
    pub fn is_enchanted(&self) -> bool {
        self.tag().is_some_and(|n| n.has("ench"))
    }
    pub fn enchantments(&self) -> Vec<EnchantmentData> {
        self.enchantment_tags().iter().map(enchantment_data).collect()
    }
    pub(crate) fn enchantment_tags(&self) -> Vec<Tag> {
        self.tag().map_or_else(Vec::new, |n| match n.get("ench") {
            None => Vec::new(),
            Some(Tag::List(_, list)) => list.clone(),
            Some(_) => panic!("ClassCastException"),
        })
    }
    pub fn add_enchantment(&self, id: i32, level: i32) {
        if !self.has_tag() {
            self.set_tag_checked(Some(Compound::new()));
        }
        let handle = self.tag_handle().unwrap();
        let mut n = handle.lock().unwrap_or_else(|e| e.into_inner());
        if !n.has("ench") {
            n.put("ench", Tag::List(10, Vec::new()));
        }
        let Tag::List(_, list) = &mut n.0.iter_mut().find(|(key, _)| key == "ench").unwrap().1
        else {
            panic!("ClassCastException");
        };
        let mut e = Compound::new();
        e.put("id", Tag::Short(id as i16));
        e.put("lvl", Tag::Short(level as i8 as i16));
        list.push(Tag::Compound(e));
    }
    pub fn is_enchantable(&self) -> bool {
        self.max_stack() == 1 && self.max_damage() > 0
            && !crate::entity::player::registry::item(self.id).unwrap().subtypes && !self.is_enchanted()
    }
}
pub(crate) fn enchantment_data(t: &Tag) -> EnchantmentData {
    let Tag::Compound(c) = t else { panic!("ClassCastException"); };
    let short = |key| match c.get(key) {
        None => 0,
        Some(Tag::Short(value)) => *value as i32,
        Some(_) => panic!("ClassCastException"),
    };
    EnchantmentData { id: short("id"), level: short("lvl") }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn copied_and_split_tags_are_independent() {
        let mut s = ItemStack::new(276, 2, 0);
        s.add_enchantment(16, 257);
        let copy = s.copy_stack181();
        let split = s.split(1);
        s.add_enchantment(20, 2);
        assert_eq!(
            copy.enchantments(),
            vec![EnchantmentData { id: 16, level: 1 }]
        );
        assert_eq!(split.enchantments(), copy.enchantments());
        let loaded = ItemStack::from_nbt(&s.to_nbt());
        assert_eq!(loaded.enchantments(), s.enchantments());
    }
}

fn compound_equal(a: &Compound, b: &Compound) -> bool {
    a.0.len() == b.0.len()
        && a.0
            .iter()
            .all(|(key, value)| b.get(key).is_some_and(|other| tag_equal(value, other)))
}
fn tag_equal(a: &Tag, b: &Tag) -> bool {
    match (a, b) {
        (Tag::Compound(a), Tag::Compound(b)) => compound_equal(a, b),
        (Tag::List(ta, a), Tag::List(tb, b)) => {
            ta == tb && a.len() == b.len() && a.iter().zip(b).all(|(a, b)| tag_equal(a, b))
        }
        _ => a == b,
    }
}

impl ItemStack {
    /// ItemStack.func_40712_q retains subclass tooltip lines before enchantments.
    pub fn tooltip10(
        &self,
        display: impl FnOnce(&Self) -> String,
        additional: impl FnOnce(&Self, &mut Vec<String>),
        translate: impl Fn(&str) -> String,
    ) -> Vec<String> {
        let mut lines = vec![display(self)];
        additional(self, &mut lines);
        if self.has_tag() {
            for data in self.enchantment_tags().iter().map(enchantment_data) {
                if let Some(e) = crate::enchantment::get(data.id) {
                    lines.push(e.translated_name(data.level, &translate));
                }
            }
        }
        lines
    }
    pub fn has_effect10(&self) -> bool {
        crate::item10::glint(*self)
    }
    pub fn rarity10(&self) -> crate::item10::Rarity {
        crate::item10::rarity(*self)
    }
}
