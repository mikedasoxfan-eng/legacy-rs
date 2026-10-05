//! Beta 1.8.1 Item base class. Subclass callbacks and resource translation stay
//! with their existing owners; the base class preserves nullable arguments.
use super::{player::ItemStack, Entity};
use crate::World;

pub type ItemHandle = usize;
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemError {
    ArrayIndexOutOfBounds,
    NullPointer,
    ContainerStackSize,
    Callback,
}
impl ItemError {
    pub fn class(self) -> &'static str {
        match self {
            Self::ArrayIndexOutOfBounds => "ArrayIndexOutOfBoundsException",
            Self::NullPointer => "NullPointerException",
            Self::ContainerStackSize => "IllegalArgumentException",
            Self::Callback => "IllegalStateException",
        }
    }
}

#[derive(Clone, Debug)]
pub struct Item {
    pub shifted_index: i32,
    max_stack_size: i32,
    max_damage: i32,
    icon_index: i32,
    full_3d: bool,
    has_subtypes: bool,
    container: Option<ItemHandle>,
    name: Option<String>,
}
impl Item {
    pub fn defaults(shifted_index: i32) -> Self {
        Self {
            shifted_index,
            max_stack_size: 64,
            max_damage: 0,
            icon_index: 0,
            full_3d: false,
            has_subtypes: false,
            container: None,
            name: None,
        }
    }
    pub fn set_icon_index(&mut self, value: i32) -> &mut Self {
        self.icon_index = value;
        self
    }
    pub fn set_icon_coord(&mut self, x: i32, y: i32) -> &mut Self {
        self.set_icon_index(x.wrapping_add(y.wrapping_mul(16)))
    }
    pub fn set_max_stack_size(&mut self, value: i32) -> &mut Self {
        self.max_stack_size = value;
        self
    }
    pub fn set_max_damage(&mut self, value: i32) -> &mut Self {
        self.max_damage = value;
        self
    }
    pub fn set_has_subtypes(&mut self, value: bool) -> &mut Self {
        self.has_subtypes = value;
        self
    }
    pub fn set_full_3d(&mut self) -> &mut Self {
        self.full_3d = true;
        self
    }
    pub fn set_item_name(&mut self, value: Option<&str>) -> &mut Self {
        self.name = Some(format!("item.{}", value.unwrap_or("null")));
        self
    }
    pub fn set_container_item(
        &mut self,
        value: Option<ItemHandle>,
    ) -> Result<&mut Self, ItemError> {
        if self.max_stack_size > 1 {
            return Err(ItemError::ContainerStackSize);
        }
        self.container = value;
        Ok(self)
    }
    pub fn max_stack_size(&self) -> i32 {
        self.max_stack_size
    }
    pub fn max_damage(&self) -> i32 {
        self.max_damage
    }
    pub fn has_subtypes(&self) -> bool {
        self.has_subtypes
    }
    pub fn is_damageable(&self) -> bool {
        self.max_damage > 0 && !self.has_subtypes
    }
    pub fn is_full_3d(&self) -> bool {
        self.full_3d
    }
    pub fn should_rotate(&self) -> bool {
        false
    }
    pub fn container_item(&self) -> Option<ItemHandle> {
        self.container
    }
    pub fn has_container_item(&self) -> bool {
        self.container.is_some()
    }
    pub fn item_name(&self) -> Option<&str> {
        self.name.as_deref()
    }
    pub fn item_name_is(&self, _stack: Option<&ItemStack>) -> Option<&str> {
        self.item_name()
    }
    pub fn icon_from_damage(&self, _damage: i32) -> i32 {
        self.icon_index
    }
    /// The final getIconIndex calls the virtual getIconFromDamage once.
    pub fn icon_index_with(
        stack: Option<&ItemStack>,
        icon: impl FnOnce(i32) -> Result<i32, ItemError>,
    ) -> Result<i32, ItemError> {
        icon(stack.ok_or(ItemError::NullPointer)?.damage)
    }
    pub fn icon_index(&self, stack: Option<&ItemStack>) -> Result<i32, ItemError> {
        Self::icon_index_with(stack, |damage| Ok(self.icon_from_damage(damage)))
    }
    /// getStatName calls virtual getItemName, then the existing translator.
    pub fn stat_name_with(
        name: impl FnOnce() -> Option<String>,
        translate: impl FnOnce(&str) -> String,
    ) -> String {
        translate(&format!("{}.name", name().as_deref().unwrap_or("null")))
    }
    pub fn stat_name(&self, translate: impl FnOnce(&str) -> String) -> String {
        Self::stat_name_with(|| self.name.clone(), translate)
    }
    pub fn placed_metadata(&self, _damage: i32) -> i32 {
        0
    }
    pub fn color_from_damage(&self, _damage: i32) -> i32 {
        0xffffff
    }
    pub fn on_item_use(
        &self,
        _stack: Option<&mut ItemStack>,
        _player: Option<&mut Entity>,
        _world: Option<&mut World>,
        _position: [i32; 3],
        _side: i32,
    ) -> bool {
        false
    }
    pub fn strength(&self, _stack: Option<&ItemStack>, _block: Option<u8>) -> f32 {
        1.0
    }
    pub fn on_item_right_click<'a>(
        &self,
        stack: Option<&'a mut ItemStack>,
        _world: Option<&mut World>,
        _player: Option<&mut Entity>,
    ) -> Option<&'a mut ItemStack> {
        stack
    }
    pub fn on_eaten<'a>(
        &self,
        stack: Option<&'a mut ItemStack>,
        _world: Option<&mut World>,
        _player: Option<&mut Entity>,
    ) -> Option<&'a mut ItemStack> {
        stack
    }
    pub fn hit_entity(
        &self,
        _stack: Option<&mut ItemStack>,
        _target: Option<&mut Entity>,
        _attacker: Option<&mut Entity>,
    ) -> bool {
        false
    }
    pub fn on_block_destroyed(
        &self,
        _stack: Option<&mut ItemStack>,
        _block: i32,
        _position: [i32; 3],
        _entity: Option<&mut Entity>,
    ) -> bool {
        false
    }
    pub fn damage_vs_entity(&self, _entity: Option<&Entity>) -> i32 {
        1
    }
    pub fn can_harvest(&self, _block: Option<u8>) -> bool {
        false
    }
    pub fn saddle_entity(&self, _stack: Option<&mut ItemStack>, _entity: Option<&mut Entity>) {}
    pub fn on_update(
        &self,
        _stack: Option<&mut ItemStack>,
        _world: Option<&mut World>,
        _entity: Option<&mut Entity>,
        _slot: i32,
        _selected: bool,
    ) {
    }
    pub fn on_created(
        &self,
        _stack: Option<&mut ItemStack>,
        _world: Option<&mut World>,
        _player: Option<&mut Entity>,
    ) {
    }
    pub fn use_action(&self, _stack: Option<&ItemStack>) -> super::player::registry::ItemUseAction {
        super::player::registry::ItemUseAction::None
    }
    pub fn use_duration(&self, _stack: Option<&ItemStack>) -> i32 {
        0
    }
    pub fn on_stopped_using(
        &self,
        _stack: Option<&mut ItemStack>,
        _world: Option<&mut World>,
        _player: Option<&mut Entity>,
        _remaining: i32,
    ) {
    }
}

/// Array slots contain handles, so replacing a slot never changes retained
/// object/container aliases. Registration precedes subclass constructor work.
pub struct ItemRegistry {
    slots: Vec<Option<ItemHandle>>,
    objects: Vec<Item>,
}
impl Default for ItemRegistry {
    fn default() -> Self {
        Self {
            slots: vec![None; 32000],
            objects: Vec::new(),
        }
    }
}
impl ItemRegistry {
    pub fn construct(
        &mut self,
        index: i32,
        mut diagnostic: impl FnMut(&str),
    ) -> Result<ItemHandle, ItemError> {
        let shifted = index.wrapping_add(256);
        let slot = self
            .slots
            .get_mut(shifted as usize)
            .ok_or(ItemError::ArrayIndexOutOfBounds)?;
        if slot.is_some() {
            diagnostic(&format!("CONFLICT @ {index}"));
        }
        let handle = self.objects.len();
        self.objects.push(Item::defaults(shifted));
        *slot = Some(handle);
        Ok(handle)
    }
    pub fn lookup(&self, id: i32) -> Result<Option<ItemHandle>, ItemError> {
        self.slots
            .get(id as usize)
            .copied()
            .ok_or(ItemError::ArrayIndexOutOfBounds)
    }
    pub fn get(&self, handle: ItemHandle) -> &Item {
        &self.objects[handle]
    }
    pub fn get_mut(&mut self, handle: ItemHandle) -> &mut Item {
        &mut self.objects[handle]
    }
}
