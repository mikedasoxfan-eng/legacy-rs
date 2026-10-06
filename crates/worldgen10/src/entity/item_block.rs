//! Literal ItemBlock dispatch. The context borrows the existing player/world
//! APIs; block callbacks, collision and tile behavior are not reimplemented.
use super::{
    item::{ItemError, ItemHandle, ItemRegistry},
    player::ItemStack,
};

pub trait BlockItemProperties {
    fn texture(&mut self, id: i32, side: i32) -> Result<i32, ItemError>;
    fn name(&mut self, id: i32) -> Result<Option<String>, ItemError>;
}
pub trait BlockItemContext {
    type Block: Copy;
    fn get_block_id(&mut self, position: [i32; 3]) -> Result<i32, ItemError>;
    fn can_edit(&mut self, position: [i32; 3], stack: &mut ItemStack) -> Result<bool, ItemError>;
    fn solid(&mut self, block: i32) -> Result<bool, ItemError>;
    fn can_place(&mut self, block: i32, position: [i32; 3], side: i32) -> Result<bool, ItemError>;
    fn block(&mut self, id: i32) -> Result<Self::Block, ItemError>;
    fn placed_metadata(&mut self, block: i32, damage: i32) -> Result<i32, ItemError>;
    fn set_block(
        &mut self,
        block: i32,
        position: [i32; 3],
        metadata: i32,
    ) -> Result<bool, ItemError>;
    fn on_placed(
        &mut self,
        block: Self::Block,
        position: [i32; 3],
        side: i32,
    ) -> Result<(), ItemError>;
    fn on_placed_by(&mut self, block: Self::Block, position: [i32; 3]) -> Result<(), ItemError>;
    fn sound(&mut self, block: Self::Block, position: [f64; 3]) -> Result<(), ItemError>;
}
#[derive(Clone, Copy, Debug)]
pub struct BlockItem {
    block_id: i32,
}
impl BlockItem {
    pub fn registered(block_id: i32) -> Self {
        Self { block_id }
    }
    pub fn construct(
        registry: &mut ItemRegistry,
        index: i32,
        properties: &mut impl BlockItemProperties,
        diagnostic: impl FnMut(&str),
    ) -> Result<(Self, ItemHandle), ItemError> {
        let handle = registry.construct(index, diagnostic)?;
        let block_id = index.wrapping_add(256);
        // The registered object survives a failed subclass constructor.
        let icon = properties.texture(block_id, 2)?;
        registry.get_mut(handle).set_icon_index(icon);
        Ok((Self { block_id }, handle))
    }
    pub fn block_id(&self) -> i32 {
        self.block_id
    }
    pub fn name(
        &self,
        properties: &mut impl BlockItemProperties,
    ) -> Result<Option<String>, ItemError> {
        properties.name(self.block_id)
    }
    pub fn name_is(
        &self,
        properties: &mut impl BlockItemProperties,
        _stack: Option<&ItemStack>,
    ) -> Result<Option<String>, ItemError> {
        self.name(properties)
    }
    pub fn on_item_use(
        &self,
        context: &mut impl BlockItemContext,
        stack: Option<&mut ItemStack>,
        mut position: [i32; 3],
        mut side: i32,
    ) -> Result<bool, ItemError> {
        let existing = context.get_block_id(position)?;
        if existing == 78 {
            side = 0;
        } else if existing != 106 {
            match side {
                0 => position[1] = position[1].wrapping_sub(1),
                1 => position[1] = position[1].wrapping_add(1),
                2 => position[2] = position[2].wrapping_sub(1),
                3 => position[2] = position[2].wrapping_add(1),
                4 => position[0] = position[0].wrapping_sub(1),
                5 => position[0] = position[0].wrapping_add(1),
                _ => {}
            }
        }
        let stack = stack.ok_or(ItemError::NullPointer)?;
        if stack.count == 0 {
            return Ok(false);
        }
        if !context.can_edit(position, stack)? {
            return Ok(false);
        }
        if position[1] == 127 && context.solid(self.block_id)? {
            return Ok(false);
        }
        if !context.can_place(self.block_id, position, side)? {
            return Ok(false);
        }
        let sound_block = context.block(self.block_id)?;
        let metadata = context.placed_metadata(self.block_id, stack.damage)?;
        if context.set_block(self.block_id, position, metadata)? {
            if context.get_block_id(position)? == self.block_id {
                // Each callback loads the current registry slot. The sound uses
                // the object captured before metadata/setBlock callbacks.
                let block = context.block(self.block_id)?;
                context.on_placed(block, position, side)?;
                let block = context.block(self.block_id)?;
                context.on_placed_by(block, position)?;
            }
            context.sound(sound_block, position.map(|v| (v as f32 + 0.5f32) as f64))?;
            stack.count = stack.count.wrapping_sub(1);
        }
        Ok(true)
    }
}
