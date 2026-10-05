//! World-generation and live block-container tile entities.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemStack {
    pub id: i16,
    pub count: i32,
    pub damage: i16,
}

impl ItemStack {
    pub fn new(id: i32, count: i32, damage: i32) -> Self {
        ItemStack {
            id: id as i16,
            count,
            damage: damage as i16,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum TileEntity {
    RecordPlayer { record: i32 },
    Note(super::notes::Note),
    Piston(super::pistons::Piston),
    Sign(crate::sign::Sign),
    /// Vanilla allocates 36 initially, but readFromNBT replaces it with 27.
    Chest(Box<[Option<ItemStack>]>),
    MobSpawner { mob: String, delay: i32 },
    Furnace(crate::container::tiles::Furnace),
    Dispenser(Box<[Option<crate::entity::player::ItemStack>; 9]>),
}

impl TileEntity {
    /// `BlockContainer.getBlockEntity()` for the given container block.
    pub fn for_block(id: u8) -> TileEntity {
        match id {
            84 => TileEntity::RecordPlayer { record: 0 },
            25 => TileEntity::Note(Default::default()),
            63 | 68 => TileEntity::Sign(Default::default()),
            crate::block::CHEST => TileEntity::Chest(Box::new([None; 36])),
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
