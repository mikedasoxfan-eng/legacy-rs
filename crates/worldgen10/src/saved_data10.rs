//! Release 1.0 WorldSavedData's abstract NBT contract and immutable-name state.
use crate::nbt::Compound;

#[derive(Clone, Debug)]
pub struct SavedDataState10 {
    map_name: String,
    dirty: bool,
}
impl SavedDataState10 {
    pub fn new(map_name: String) -> Self { Self { map_name, dirty: false } }
    pub fn map_name(&self) -> &str { &self.map_name }
    pub fn mark_dirty(&mut self) { self.set_dirty(true); }
    pub fn set_dirty(&mut self, dirty: bool) { self.dirty = dirty; }
    pub fn is_dirty(&self) -> bool { self.dirty }
}
pub trait WorldSavedData {
    fn state(&self) -> &SavedDataState10;
    fn state_mut(&mut self) -> &mut SavedDataState10;
    fn read_from_nbt(&mut self, tag: &Compound);
    fn write_to_nbt(&self, tag: &mut Compound);
    fn map_name(&self) -> &str { self.state().map_name() }
    fn mark_dirty(&mut self) { self.state_mut().mark_dirty(); }
    fn set_dirty(&mut self, dirty: bool) { self.state_mut().set_dirty(dirty); }
    fn is_dirty(&self) -> bool { self.state().is_dirty() }
}

/// IEnchantmentModifier's callback contract. The blocks worker supplies the
/// concrete Enchantment type; a type parameter preserves its reference argument.
pub trait IEnchantmentModifier<Enchantment> {
    fn calculate_modifier(&mut self, enchantment: &Enchantment, level: i32);
}
