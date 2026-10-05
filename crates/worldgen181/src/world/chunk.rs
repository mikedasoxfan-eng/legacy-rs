//! `Chunk` storage and the chunk-local halves of its methods. Anything that calls
//! back into the world (lighting schedules, block hooks) lives on `World`.

use super::tile::TileEntity;
use crate::block::{self, light_opacity};
use crate::overworld::CHUNK_VOLUME;
use std::collections::HashMap;

/// `NibbleArray` with the chunk layout (`x << 11 | z << 7 | y`).
#[derive(Clone)]
pub struct Nibbles(pub Box<[u8; CHUNK_VOLUME / 2]>);

impl Default for Nibbles {
    fn default() -> Self {
        Nibbles(
            vec![0u8; CHUNK_VOLUME / 2]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
        )
    }
}

impl Nibbles {
    #[inline]
    pub fn get(&self, x: i32, y: i32, z: i32) -> i32 {
        let i = (x << 11 | z << 7 | y) as usize;
        let b = self.0[i >> 1];
        (if i & 1 == 0 { b & 15 } else { b >> 4 & 15 }) as i32
    }

    #[inline]
    pub fn set(&mut self, x: i32, y: i32, z: i32, v: i32) {
        let i = (x << 11 | z << 7 | y) as usize;
        let b = &mut self.0[i >> 1];
        if i & 1 == 0 {
            *b = (*b & 0xf0) | (v as u8 & 15);
        } else {
            *b = (*b & 0x0f) | ((v as u8 & 15) << 4);
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LightKind {
    Sky,
    Block,
}

impl LightKind {
    /// `EnumSkyBlock.field_1722_c`
    pub fn default_value(self) -> i32 {
        match self {
            LightKind::Sky => 15,
            LightKind::Block => 0,
        }
    }
}

pub struct Chunk {
    pub x: i32,
    pub z: i32,
    pub blocks: Box<[u8; CHUNK_VOLUME]>,
    pub precipitation_height: [i32;256],
    pub skylight_occlusion: [bool;256],
    pub skylight_updated: bool,
    pub data: Nibbles,
    pub sky_light: Nibbles,
    pub block_light: Nibbles,
    /// Index `z << 4 | x`.
    pub height_map: [u8; 256],
    pub lowest_block_height: i32,
    pub populated: bool,
    pub loaded: bool,
    /// Keyed by chunk-local (x, y, z).
    pub tile_entities: HashMap<(i32, i32, i32), TileEntity>,
    /// Insertion order before Java's chunk tile-map bucket iteration.
    pub tile_entity_ids: HashMap<(i32, i32, i32), u64>,
    pub tile_entity_order: Vec<(i32, i32, i32)>,
    /// `World::mod_count` at this chunk's last block/metadata/light change (for renderers).
    pub version: u64,
    /// `isModified`: needs writing to disk.
    pub modified: bool,
    pub never_save: bool,
    /// `lastSaveTime`
    pub last_save_time: i64,
    /// `hasEntities`
    pub has_entities: bool,
    /// Unsupported entities from a save; written back unchanged.
    pub raw_entities: Vec<crate::nbt::Tag>,
    pub entity_lists: [Vec<i32>; 8],
    /// Tile entities of kinds not modelled yet (furnaces, signs...); written back unchanged.
    pub raw_tile_entities: Vec<crate::nbt::Compound>,
}

#[inline]
pub fn idx(x: i32, y: i32, z: i32) -> usize {
    (x << 11 | z << 7 | y) as usize
}

impl Chunk {
    pub fn new(x: i32, z: i32, blocks: Box<[u8; CHUNK_VOLUME]>) -> Self {
        Chunk {
            x,
            z,
            blocks,
            precipitation_height: [-999;256],
            skylight_occlusion: [false;256],
            skylight_updated: false,            data: Nibbles::default(),
            sky_light: Nibbles::default(),
            block_light: Nibbles::default(),
            height_map: [0; 256],
            lowest_block_height: 0,
            populated: false,
            loaded: false,
            tile_entities: HashMap::new(),
            tile_entity_order: Vec::new(),
            tile_entity_ids: HashMap::new(),
            version: 0,
            modified: false,
            never_save: false,
            last_save_time: 0,
            has_entities: false,
            entity_lists: Default::default(),
            raw_entities: Vec::new(),
            raw_tile_entities: Vec::new(),
        }
    }

    #[inline]
    pub fn block(&self, x: i32, y: i32, z: i32) -> u8 {
        self.blocks[idx(x, y, z)]
    }

    #[inline]
    pub fn height(&self, x: i32, z: i32) -> i32 {
        self.height_map[(z << 4 | x) as usize] as i32
    }

    /// First half of `func_1024_c`: height map and the straight-down skylight pass.
    /// The neighbour-edge lighting schedules (`func_996_c`) are done by the world.
    pub fn init_height_and_sky(&mut self) {
        self.init_height_and_sky_for_dimension(super::Dimension::Overworld);
    }

    /// `func_1024_c` with the provider's `hasNoSky` guard.
    pub fn init_height_and_sky_for_dimension(&mut self, dimension: super::Dimension) {
        let mut lowest = 127;
        for x in 0..16 {
            for z in 0..16 {
                let base = (x << 11 | z << 7) as usize;
                let mut h = 127i32;
                while h > 0 && light_opacity(self.blocks[base + h as usize - 1]) == 0 {
                    h -= 1;
                }
                self.height_map[(z << 4 | x) as usize] = h as u8;
                if h < lowest {
                    lowest = h;
                }
                if !dimension.has_no_sky() {
                    let mut light = 15;
                    let mut y = 127i32;
                    loop {
                        light -= light_opacity(self.blocks[base + y as usize]);
                        if light > 0 {
                            self.sky_light.set(x, y, z, light);
                        }
                        y -= 1;
                        if !(y > 0 && light > 0) {
                            break;
                        }
                    }
                }
            }
        }
        self.lowest_block_height = lowest;
        self.skylight_occlusion.fill(true);
        self.modified=true;
    }

    #[inline]
    pub fn can_block_see_the_sky(&self, x: i32, y: i32, z: i32) -> bool {
        y >= self.height(x, z)
    }

    pub fn saved_light(&self, kind: LightKind, x: i32, y: i32, z: i32) -> i32 {
        match kind {
            LightKind::Sky => self.sky_light.get(x, y, z),
            LightKind::Block => self.block_light.get(x, y, z),
        }
    }

    pub fn set_light(&mut self, kind: LightKind, x: i32, y: i32, z: i32, v: i32) {
        match kind {
            LightKind::Sky => self.sky_light.set(x, y, z, v),
            LightKind::Block => self.block_light.set(x, y, z, v),
        }
    }

    /// `getBlockLightValue(x, y, z, skylightSubtracted)`
    pub fn block_light_value(&self, x: i32, y: i32, z: i32, subtracted: i32) -> i32 {
        let mut v = self.sky_light.get(x, y, z) - subtracted;
        let b = self.block_light.get(x, y, z);
        if b > v {
            v = b;
        }
        v
    }

    pub fn is_air(&self, x: i32, y: i32, z: i32) -> bool {
        self.block(x, y, z) == block::AIR
    }
}

/// NibbleArray's new height-bit constructor. Chunks use the same layout with seven bits.
#[derive(Clone)]
pub struct NibbleArray181 {pub data:Vec<u8>,pub height_bits:u32}
impl NibbleArray181 {
    pub fn new(size:usize,height_bits:u32)->Self{Self{data:vec![0;size>>1],height_bits}}
    pub fn from_data(data:Vec<u8>,height_bits:u32)->Self{Self{data,height_bits}}
    pub fn get(&self,x:i32,y:i32,z:i32)->i32 {
        let i=(x.wrapping_shl(self.height_bits+4)|z.wrapping_shl(self.height_bits)|y) as usize;
        (self.data[i>>1]>>(if i&1==0{0}else{4})&15) as i32
    }
    pub fn set(&mut self,x:i32,y:i32,z:i32,v:i32) {
        let i=(x.wrapping_shl(self.height_bits+4)|z.wrapping_shl(self.height_bits)|y) as usize;
        let shift=if i&1==0{0}else{4};self.data[i>>1]=(self.data[i>>1]&!(15<<shift))|((v as u8&15)<<shift);
    }
}
