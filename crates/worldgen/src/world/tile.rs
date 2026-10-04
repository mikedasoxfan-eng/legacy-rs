//! Tile entities that world generation creates (dungeon chests and spawners).

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemStack {
    pub id: i16,
    pub count: u8,
    pub damage: i16,
}

impl ItemStack {
    pub fn new(id: i32, count: i32, damage: i32) -> Self {
        ItemStack { id: id as i16, count: count as u8, damage: damage as i16 }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TileEntity {
    Chest(Box<[Option<ItemStack>; 27]>),
    MobSpawner { mob: String, delay: i32 },
    Furnace(crate::container::tiles::Furnace),
    Dispenser(Box<[Option<crate::entity::player::ItemStack>; 9]>),
}

impl TileEntity {
    /// `BlockContainer.getBlockEntity()` for the given container block.
    pub fn for_block(id: u8) -> TileEntity {
        match id {
            crate::block::CHEST => TileEntity::Chest(Box::new([None; 27])),
            crate::block::MOB_SPAWNER => TileEntity::MobSpawner { mob: "Pig".into(), delay: 20 },
            61 | 62 => TileEntity::Furnace(Default::default()),
            23 => TileEntity::Dispenser(Box::new([None; 9])),
            _ => panic!("tile entity for block {id} not ported"),
        }
    }
}
