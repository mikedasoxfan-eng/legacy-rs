//! Block IDs and the static per-block/per-material properties (`Block.lightOpacity`,
//! `Material.isSolid()` and friends). The property table is generated from the
//! original classes by the oracle, so it cannot drift from vanilla.

mod ids;
pub mod visual181;
pub mod table;

pub use ids::*;
pub use table::*;

#[derive(Clone, Copy, Debug)]
pub struct MaterialProps {
    pub name: &'static str,
    /// `Material.isSolid()`
    pub is_solid: bool,
    /// `Material.getIsSolid()` (blocks movement / fluids)
    pub blocks_movement: bool,
    /// `Material.getIsLiquid()`
    pub is_liquid: bool,
    /// `Material.getBurning()`
    pub burns: bool,
    /// `Material.getIsGroundCover()`
    pub ground_cover: bool,
    /// `Material.getIsTranslucent()` (despite the name: solid and not see-through)
    pub translucent_solid: bool,
    /// `Material.getCanBlockGrass()`
    pub blocks_grass: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct BlockProps {
    pub exists: bool,
    pub material: u8,
    pub light_opacity: u8,
    pub light_value: u8,
    /// `Block.opaqueCubeLookup` (fixed at class init)
    pub opaque_cube: bool,
    /// `renderAsNormalBlock()`
    pub normal_render: bool,
    /// `Block.isBlockContainer`
    pub container: bool,
    /// `Block.tickOnLoad`: receives random ticks.
    pub ticks: bool,
    /// `tickRate()`
    pub tick_rate: i32,
    pub name: &'static str,
}

impl BlockProps {
    pub const NONE: BlockProps = BlockProps {
        exists: false,
        material: MAT_AIR,
        light_opacity: 0,
        light_value: 0,
        opaque_cube: false,
        normal_render: false,
        container: false,
        ticks: false,
        tick_rate: 10,
        name: "",
    };
}

#[inline]
pub fn props(id: u8) -> &'static BlockProps {
    &BLOCKS[id as usize]
}

/// `getBlockMaterial`-style lookup: id 0 is `Material.air`.
#[inline]
pub fn material_of(id: u8) -> u8 {
    if id == AIR {
        MAT_AIR
    } else {
        BLOCKS[id as usize].material
    }
}

#[inline]
pub fn mat(m: u8) -> &'static MaterialProps {
    &MATERIALS[m as usize]
}

#[inline]
pub fn light_opacity(id: u8) -> i32 {
    BLOCKS[id as usize].light_opacity as i32
}

#[inline]
pub fn light_value(id: u8) -> i32 {
    BLOCKS[id as usize].light_value as i32
}

#[inline]
pub fn opaque_cube_lookup(id: u8) -> bool {
    BLOCKS[id as usize].opaque_cube
}

/// Block.useNeighborBrightness, initialized from stairs, both slabs and farmland.
pub fn use_neighbor_brightness(id:u8)->bool {matches!(id,43|44|53|60|67|108|109|114)}
