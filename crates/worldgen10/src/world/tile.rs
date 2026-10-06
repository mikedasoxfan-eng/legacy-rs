//! World-generation and live block-container tile entities.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ItemStack {
    pub id: i32,
    pub count: i32,
    pub damage: i16,
}

impl ItemStack {
    pub fn new(id: i32, count: i32, damage: i32) -> Self {
        ItemStack {
            id,
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
    pub fn mapping181(&self) -> &'static str {
        match self {
            Self::Furnace(_) => "Furnace", Self::Chest(_) => "Chest", Self::RecordPlayer { .. } => "RecordPlayer",
            Self::Dispenser(_) => "Trap", Self::Sign(_) => "Sign", Self::MobSpawner { .. } => "MobSpawner",
            Self::Note(_) => "Music", Self::Piston(_) => "Piston",
        }
    }
    /// TileEntity.createAndLoadEntity: lookup precedes coordinate/subclass reads.
    pub fn create_and_load181(n: &crate::nbt::Compound) -> Result<Option<(super::tile_nbt181::TileBase181, Self)>, super::tile_nbt181::TileError181> {
        use super::tile_nbt181 as tn;
        let name=tn::string(n,"id")?;
        let mut tile=match name {
            "Furnace" => Self::for_block(61), "Chest" => Self::for_block(54), "RecordPlayer" => Self::for_block(84),
            "Trap" => Self::for_block(23), "Sign" => Self::for_block(63), "MobSpawner" => Self::for_block(52),
            "Music" => Self::for_block(25), "Piston" => Self::Piston(super::pistons::Piston::new(0,0,0,false,false)),
            _ => return Ok(None),
        };
        let mut base=tn::TileBase181::default();base.read_from_nbt(n)?;
        match &mut tile {
            Self::Furnace(_) | Self::Dispenser(_) => tile.read_machine_nbt181(&mut base.position,n)?,
            Self::Chest(slots) => {
                let mut items=[None;27];crate::container::tiles::read_items181(n,&mut items,false)?;
                *slots=items.into_iter().map(|s|s.map(Into::into)).collect::<Vec<_>>().into_boxed_slice();
            },
            Self::RecordPlayer { record } => *record=tn::int(n,"Record")?,
            Self::MobSpawner { mob,delay } => {*mob=tn::string(n,"EntityId")?.into();*delay=tn::short(n,"Delay")? as i32;},
            Self::Note(note) => {tn::byte(n,"note")?;*note=super::notes::Note::from_nbt(n);},
            Self::Sign(sign) => {for k in 1..=4 {tn::string(n,&format!("Text{k}"))?;}*sign=crate::sign::Sign::from_nbt(n);},
            Self::Piston(p) => {
                tn::int(n,"blockId")?;tn::int(n,"blockData")?;tn::int(n,"facing")?;tn::float(n,"progress")?;tn::byte(n,"extending")?;
                *p=super::pistons::Piston::from_nbt(n);
            },
        }
        Ok(Some((base,tile)))
    }
    pub fn write_nbt181(&self, position: (i32,i32,i32)) -> crate::nbt::Compound {
        use crate::nbt::Tag;
        let (x,y,z)=position;
        match self {
            Self::Furnace(f) => f.to_nbt(x,y,z),
            Self::Dispenser(items) => crate::container::tiles::inventory_nbt("Trap",x,y,z,&**items),
            Self::Chest(slots) => {
                let items:Vec<_>=slots.iter().map(|s|s.map(Into::into)).collect();
                crate::container::tiles::inventory_nbt("Chest",x,y,z,&items)
            },
            Self::RecordPlayer { record } => {let mut n=super::notes::tile_nbt("RecordPlayer",x,y,z);if *record>0{n.put("Record",Tag::Int(*record));}n},
            Self::Note(note) => note.to_nbt(x,y,z),
            Self::Piston(p) => p.to_nbt(x,y,z),
            Self::Sign(sign) => sign.to_nbt(x,y,z),
            Self::MobSpawner { mob,delay } => {let mut n=super::notes::tile_nbt("MobSpawner",x,y,z);n.put("EntityId",Tag::String(mob.clone()));n.put("Delay",Tag::Short(*delay as i16));n},
        }
    }
    /// The base coordinates are assigned before subclass inventory loading.
    pub fn read_machine_nbt181(&mut self, position: &mut (i32, i32, i32), n: &crate::nbt::Compound) -> Result<(), super::tile_nbt181::TileError181> {
        super::tile_nbt181::read_position(position, n)?;
        match self {
            TileEntity::Furnace(f) => f.read_from_nbt181(n),
            TileEntity::Dispenser(items) => crate::container::tiles::read_items181(n, &mut **items, false),
            _ => panic!("not a machine inventory"),
        }
    }
    pub fn read_piston_nbt181(&mut self, position: &mut (i32,i32,i32), n: &crate::nbt::Compound) -> Result<(), super::tile_nbt181::TileError181> {
        super::tile_nbt181::read_position(position,n)?;
        let Self::Piston(p)=self else{panic!("not a piston");};p.read_from_nbt181(n)
    }
    pub fn open_machine181(&mut self) {
        if let TileEntity::Furnace(f) = self { f.open_chest181(); }
    }
    pub fn close_machine181(&mut self) {
        if let TileEntity::Furnace(f) = self { f.close_chest181(); }
    }
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
