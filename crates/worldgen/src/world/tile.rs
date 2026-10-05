//! World-generation and live block-container tile entities.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemStack {
    pub id: i16,
    pub count: u8,
    pub damage: i16,
}

impl ItemStack {
    pub fn new(id: i32, count: i32, damage: i32) -> Self {
        ItemStack {
            id: id as i16,
            count: count as u8,
            damage: damage as i16,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileEntity {
    Note(super::notes::Note),
    Piston(super::pistons::Piston),
    Sign(crate::sign::Sign),
    Chest(Box<[Option<ItemStack>; 27]>),
    MobSpawner { mob: String, delay: i32 },
    Furnace(crate::container::tiles::Furnace),
    Dispenser(Box<[Option<crate::entity::player::ItemStack>; 9]>),
}

impl TileEntity {
    /// `BlockContainer.getBlockEntity()` for the given container block.
    pub fn for_block(id: u8) -> TileEntity {
        match id {
            25 => TileEntity::Note(Default::default()),
            63 | 68 => TileEntity::Sign(Default::default()),
            crate::block::CHEST => TileEntity::Chest(Box::new([None; 27])),
            crate::block::MOB_SPAWNER => TileEntity::MobSpawner {
                mob: "Pig".into(),
                delay: 20,
            },
            61 | 62 => TileEntity::Furnace(Default::default()),
            23 => TileEntity::Dispenser(Box::new([None; 9])),
            _ => panic!("tile entity for block {id} not ported"),
        }
    }
}
