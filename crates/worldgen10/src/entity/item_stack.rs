//! ItemStack's class boundary over the existing player-owned stack and identity.
//! Virtual dependencies borrow that same stack; mutations are visible before
//! the next registry lookup or stat callback, including on exceptional exits.
use super::player::{registry, ItemStack};
use super::Entity;
use crate::nbt::{Compound, Tag};
use crate::World;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StackError {
    NullPointer,
    ArrayIndexOutOfBounds,
    ClassCast,
    Callback,
}
impl StackError {
    pub fn class(self) -> &'static str {
        match self {
            Self::NullPointer => "NullPointerException",
            Self::ArrayIndexOutOfBounds => "ArrayIndexOutOfBoundsException",
            Self::ClassCast => "ClassCastException",
            Self::Callback => "IllegalStateException",
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum StackOp {
    Icon,
    Use { position: [i32; 3], side: i32 },
    Strength,
    RightClick,
    Eaten,
    MaxStack,
    Stackable,
    Damageable,
    HasSubtypes,
    Damaged,
    MaxDamage,
    Damage { amount: i32 },
    Hit,
    Destroy { block: i32, position: [i32; 3] },
    DamageVsEntity,
    Harvest,
    Saddle,
    Name,
    DisplayString,
    Update { slot: i32, selected: bool },
    Craft,
    Duration,
    Action,
    Stop { remaining: i32 },
    Destroyed,
}
#[derive(Clone, Debug)]
pub enum StackValue {
    Unit,
    Bool(bool),
    Int(i32),
    Float(f32),
    Name(Option<String>),
    Stack(Option<ItemStack>),
    Action(Option<registry::ItemUseAction>),
}
impl StackValue {
    pub fn int(self) -> i32 {
        if let Self::Int(v) = self {
            v
        } else {
            panic!("integer Item callback")
        }
    }
    pub fn boolean(self) -> bool {
        if let Self::Bool(v) = self {
            v
        } else {
            panic!("boolean Item callback")
        }
    }
}
#[derive(Clone, Copy, Debug)]
pub enum StackStat {
    Use,
    Break,
    Craft,
}
/// The resource arguments (World/player/block/target) belong to the borrowed
/// context. Lookup is nullable; invoke only receives an actual registered item.
pub trait StackContext {
    type Item: Copy;
    fn lookup(&mut self, id: i32) -> Result<Option<Self::Item>, StackError>;
    fn invoke(
        &mut self,
        item: Self::Item,
        op: StackOp,
        stack: &mut ItemStack,
    ) -> Result<StackValue, StackError>;
    fn entity_is_player(&self) -> bool;
    fn stat(
        &mut self,
        kind: StackStat,
        id: i32,
        amount: i32,
        stack: &mut ItemStack,
    ) -> Result<(), StackError>;
}
/// ItemStack.useItem borrows the existing live item dispatcher and stat sink.
/// Inventory/controller policies still own publication and creative restoration.
struct LiveUseContext<'a> {
    player: &'a mut Entity,
    world: &'a mut World,
}
impl StackContext for LiveUseContext<'_> {
    type Item = i32;
    fn lookup(&mut self, id: i32) -> Result<Option<i32>, StackError> {
        if !(0..32000).contains(&id) {
            return Err(StackError::ArrayIndexOutOfBounds);
        }
        Ok(registry::item(id).map(|_| id))
    }
    fn invoke(
        &mut self,
        _item: i32,
        op: StackOp,
        stack: &mut ItemStack,
    ) -> Result<StackValue, StackError> {
        let StackOp::Use { position, side } = op else {
            return Err(StackError::Callback);
        };
        Ok(StackValue::Bool(self.player.item_use_on_block181(
            self.world,
            stack,
            position[0],
            position[1],
            position[2],
            side,
        )))
    }
    fn entity_is_player(&self) -> bool {
        true
    }
    fn stat(
        &mut self,
        kind: StackStat,
        id: i32,
        amount: i32,
        _stack: &mut ItemStack,
    ) -> Result<(), StackError> {
        if !(0..32000).contains(&id) {
            return Err(StackError::ArrayIndexOutOfBounds);
        }
        let stat = match kind {
            StackStat::Use => super::player::stat::used(id),
            StackStat::Break => super::player::stat::broken(id),
            StackStat::Craft => super::player::stat::crafted(id),
        };
        self.player.add_stat(stat, amount);
        Ok(())
    }
}
impl ItemStack {
    pub fn use_item181(
        &mut self,
        player: &mut Entity,
        world: &mut World,
        position: [i32; 3],
        side: i32,
    ) -> Result<bool, StackError> {
        self.dispatch181(
            &mut LiveUseContext { player, world },
            StackOp::Use { position, side },
        )
        .map(StackValue::boolean)
    }
    pub fn max_stack_checked181(&self) -> Result<i32, StackError> {
        if !(0..32000).contains(&self.id) {
            return Err(StackError::ArrayIndexOutOfBounds);
        }
        Ok(registry::item(self.id)
            .ok_or(StackError::NullPointer)?
            .max_stack)
    }
    pub fn max_damage_checked181(&self) -> Result<i32, StackError> {
        if !(0..32000).contains(&self.id) {
            return Err(StackError::ArrayIndexOutOfBounds);
        }
        Ok(registry::item(self.id)
            .ok_or(StackError::NullPointer)?
            .durability)
    }
    pub fn from_item181(
        item: Option<&super::item::Item>,
        count: i32,
        damage: i32,
    ) -> Result<Self, StackError> {
        Ok(Self::new(
            item.ok_or(StackError::NullPointer)?.shifted_index,
            count,
            damage,
        ))
    }
    pub fn from_block181(
        block_id: Option<i32>,
        count: i32,
        damage: i32,
    ) -> Result<Self, StackError> {
        Ok(Self::new(
            block_id.ok_or(StackError::NullPointer)?,
            count,
            damage,
        ))
    }
    /// Java copy(), unlike borrowing a value, allocates a new identity and
    /// resets animations. Animation state is not part of any Java equality.
    pub fn copy_stack181(&self) -> Self {
        Self::new(self.id, self.count, self.damage)
    }
    pub fn copy_nullable181(stack: Option<&Self>) -> Option<Self> {
        stack.map(Self::copy_stack181)
    }
    pub fn stacks_equal181(a: Option<&Self>, b: Option<&Self>) -> bool {
        match (a, b) {
            (None, None) => true,
            (Some(a), Some(b)) => a.stack_equal181(b),
            _ => false,
        }
    }
    pub fn stack_equal181(&self, other: &Self) -> bool {
        self.id == other.id && self.count == other.count && self.damage == other.damage
    }
    pub fn item_equal181(&self, other: Option<&Self>) -> Result<bool, StackError> {
        let other = other.ok_or(StackError::NullPointer)?;
        Ok(self.id == other.id && self.damage == other.damage)
    }
    pub fn read_nbt_checked181(&mut self, n: Option<&Compound>) -> Result<(), StackError> {
        let n = n.ok_or(StackError::NullPointer)?;
        self.read_nbt_with181(|_, key, ty| {
            if n.get(key).is_some_and(|t| t.id() != ty) {
                return Err(StackError::ClassCast);
            }
            Ok(if ty == 1 {
                n.byte(key) as i32
            } else {
                n.short(key) as i32
            })
        })
    }
    /// NBTTagCompound getters are virtual. Each assignment happens only after
    /// its getter returns, and earlier assignments survive a later exception.
    pub fn read_nbt_with181(
        &mut self,
        mut read: impl FnMut(&mut Self, &str, u8) -> Result<i32, StackError>,
    ) -> Result<(), StackError> {
        self.id = read(self, "id", 2)? as i16 as i32;
        self.count = read(self, "Count", 1)? as i8 as i32;
        self.damage = read(self, "Damage", 2)? as i16 as i32;
        Ok(())
    }
    pub fn write_nbt_with181(
        &mut self,
        mut write: impl FnMut(&mut Self, &str, Tag) -> Result<(), StackError>,
    ) -> Result<(), StackError> {
        write(self, "id", Tag::Short(self.id as i16))?;
        write(self, "Count", Tag::Byte(self.count as i8))?;
        write(self, "Damage", Tag::Short(self.damage as i16))?;
        Ok(())
    }
    pub fn load_checked181(
        n: Option<&Compound>,
        mut lookup: impl FnMut(i32) -> Result<bool, StackError>,
    ) -> Result<Option<Self>, StackError> {
        let mut stack = Self::new(0, 0, 0);
        stack.read_nbt_checked181(n)?;
        Ok(lookup(stack.id)?.then_some(stack))
    }
    pub fn write_nbt181<'a>(
        &self,
        n: Option<&'a mut Compound>,
    ) -> Result<&'a mut Compound, StackError> {
        let n = n.ok_or(StackError::NullPointer)?;
        n.put("id", Tag::Short(self.id as i16));
        n.put("Count", Tag::Byte(self.count as i8));
        n.put("Damage", Tag::Short(self.damage as i16));
        Ok(n)
    }
    pub fn get_item181<C: StackContext>(
        &self,
        context: &mut C,
    ) -> Result<Option<C::Item>, StackError> {
        context.lookup(self.id)
    }
    fn item_callback181<C: StackContext>(
        &mut self,
        context: &mut C,
        op: StackOp,
    ) -> Result<StackValue, StackError> {
        let item = self.get_item181(context)?.ok_or(StackError::NullPointer)?;
        context.invoke(item, op, self)
    }
    pub fn dispatch181<C: StackContext>(
        &mut self,
        context: &mut C,
        op: StackOp,
    ) -> Result<StackValue, StackError> {
        use StackOp::*;
        match op {
            Stackable => {
                let limit = self.item_callback181(context, MaxStack)?.int();
                let result = limit > 1
                    && (!self.dispatch181(context, Damageable)?.boolean()
                        || !self.dispatch181(context, Damaged)?.boolean());
                Ok(StackValue::Bool(result))
            }
            Damageable => Ok(StackValue::Bool(
                self.item_callback181(context, MaxDamage)?.int() > 0,
            )),
            Damaged => Ok(StackValue::Bool(
                self.dispatch181(context, Damageable)?.boolean() && self.damage > 0,
            )),
            Damage { amount } => {
                if self.dispatch181(context, Damageable)?.boolean() {
                    self.damage = self.damage.wrapping_add(amount);
                    if self.damage > self.item_callback181(context, MaxDamage)?.int() {
                        if context.entity_is_player() {
                            context.stat(StackStat::Break, self.id, 1, self)?;
                        }
                        self.count = self.count.wrapping_sub(1).max(0);
                        self.damage = 0;
                    }
                }
                Ok(StackValue::Unit)
            }
            Use { .. } | Hit | Destroy { .. } => {
                let result = self.item_callback181(context, op)?.boolean();
                if result {
                    context.stat(StackStat::Use, self.id, 1, self)?;
                }
                Ok(if matches!(op, Use { .. }) {
                    StackValue::Bool(result)
                } else {
                    StackValue::Unit
                })
            }
            Update { .. } => {
                if self.animations > 0 {
                    self.animations -= 1;
                }
                self.item_callback181(context, op)
            }
            Craft => {
                context.stat(StackStat::Craft, self.id, self.count, self)?;
                self.item_callback181(context, op)
            }
            DisplayString => {
                // StringBuilder appends the count before evaluating virtual name.
                let count = self.count;
                let value = self.item_callback181(context, DisplayString)?;
                let StackValue::Name(name) = value else {
                    panic!("Item.getItemName callback")
                };
                Ok(StackValue::Name(Some(format!(
                    "{}x{}@{}",
                    count,
                    name.as_deref().unwrap_or("null"),
                    self.damage
                ))))
            }
            Destroyed => Ok(StackValue::Unit),
            _ => self.item_callback181(context, op),
        }
    }
}
