//! Beta 1.8.1 ChunkCache. The view retains its original chunk rectangle and
//! reads the live chunks, as Java's array of Chunk references does.
use super::{chunk::LightKind, World};
use crate::{biome::BiomeSource, block};

pub struct ChunkCache181<'a> {
    world: &'a mut World,
    min_x: i32,
    min_z: i32,
    width: i32,
    depth: i32,
}

impl<'a> ChunkCache181<'a> {
    pub fn new(world: &'a mut World, x0: i32, z0: i32, x1: i32, z1: i32) -> Self {
        let (min_x, min_z, max_x, max_z) = (x0 >> 4, z0 >> 4, x1 >> 4, z1 >> 4);
        assert!(max_x >= min_x && max_z >= min_z, "negative ChunkCache array size");
        for x in min_x..=max_x {
            for z in min_z..=max_z {
                world.access_chunk_pub(x, z);
            }
        }
        Self { world, min_x, min_z, width: max_x - min_x + 1, depth: max_z - min_z + 1 }
    }
    fn contains(&self, x: i32, z: i32) -> bool {
        let (x, z) = ((x >> 4) - self.min_x, (z >> 4) - self.min_z);
        x >= 0 && x < self.width && z >= 0 && z < self.depth
    }
    fn indexed(&self, x: i32, z: i32) -> &super::chunk::Chunk {
        assert!(self.contains(x, z), "ChunkCache array index out of bounds");
        self.world.chunk(x >> 4, z >> 4)
    }
    pub fn block_id(&self, x: i32, y: i32, z: i32) -> u8 {
        if !(0..128).contains(&y) || !self.contains(x, z) { return 0; }
        self.world.chunks.get(&(x >> 4, z >> 4)).map_or(0, |c| c.block(x & 15, y, z & 15))
    }
    pub fn metadata(&self, x: i32, y: i32, z: i32) -> i32 {
        if !(0..128).contains(&y) { return 0; }
        self.indexed(x, z).data.get(x & 15, y, z & 15)
    }
    pub fn tile_entity(&mut self, x: i32, y: i32, z: i32) -> Option<&mut super::tile::TileEntity> {
        assert!(self.contains(x, z), "ChunkCache array index out of bounds");
        self.world.get_block_tile_entity(x, y, z)
    }
    pub fn material(&self, x: i32, y: i32, z: i32) -> u8 { block::material_of(self.block_id(x, y, z)) }
    pub fn opaque_cube(&self, x: i32, y: i32, z: i32) -> bool { self.world.is_opaque_cube_instance(self.block_id(x, y, z)) }
    pub fn normal_cube(&self, x: i32, y: i32, z: i32) -> bool {
        let p = block::props(self.block_id(x, y, z));
        p.exists && block::mat(p.material).blocks_movement && p.normal_render
    }
    pub fn air_block(&self, x: i32, y: i32, z: i32) -> bool { !block::props(self.block_id(x, y, z)).exists }
    pub fn biome_source(&self) -> &BiomeSource { &self.world.terrain.biome_source }
    pub fn height(&self) -> i32 { 128 }
    // ChunkCache's positive z limit is inclusive; World's is exclusive.
    fn in_bounds(x: i32, z: i32) -> bool { x >= -30000000 && z >= -30000000 && x < 30000000 && z <= 30000000 }
    pub fn light_value(&self, x: i32, y: i32, z: i32) -> i32 { self.light_value_ext(x, y, z, true) }
    pub fn light_value_ext(&self, x: i32, y: i32, z: i32, neighbors: bool) -> i32 {
        if !Self::in_bounds(x, z) { return 15; }
        if neighbors && matches!(self.block_id(x, y, z), 44 | 60 | 53 | 67) {
            return [(0,1,0),(1,0,0),(-1,0,0),(0,0,1),(0,0,-1)].into_iter()
                .map(|(dx,dy,dz)| self.light_value_ext(x+dx,y+dy,z+dz,false)).max().unwrap();
        }
        if y < 0 { return 0; }
        if y >= 128 { return (15 - self.world.skylight_subtracted).max(0); }
        self.indexed(x, z).block_light_value(x & 15, y, z & 15, self.world.skylight_subtracted)
    }
    pub fn saved_light_raw(&self, kind: LightKind, x: i32, y: i32, z: i32) -> i32 {
        if !Self::in_bounds(x, z) { return kind.default_value(); }
        self.indexed(x, z).saved_light(kind, x & 15, y.clamp(0,127), z & 15)
    }
    pub fn saved_light(&self, kind: LightKind, x: i32, y: i32, z: i32) -> i32 {
        let y = y.clamp(0,127);
        if !Self::in_bounds(x, z) { return kind.default_value(); }
        if matches!(self.block_id(x, y, z), 44 | 60 | 53 | 67) {
            [(0,1,0),(1,0,0),(-1,0,0),(0,0,1),(0,0,-1)].into_iter()
                .map(|(dx,dy,dz)| self.saved_light_raw(kind,x+dx,y+dy,z+dz)).max().unwrap()
        } else { self.saved_light_raw(kind,x,y,z) }
    }
    pub fn packed_light(&self, x: i32, y: i32, z: i32, minimum: i32) -> i32 {
        self.saved_light(LightKind::Sky,x,y,z).wrapping_shl(20)
            | self.saved_light(LightKind::Block,x,y,z).max(minimum).wrapping_shl(4)
    }
    pub fn brightness(&self, x: i32, y: i32, z: i32, minimum: i32) -> f32 {
        self.world.light_brightness_table[self.light_value(x,y,z).max(minimum) as usize]
    }
    pub fn light_brightness(&self, x: i32, y: i32, z: i32) -> f32 { self.brightness(x,y,z,0) }
    pub fn block_texture(&self,id:u8,x:i32,y:i32,z:i32,side:usize)->Result<i32,()> {
        block::visual181::texture_at_with_bounds(id,side,self.metadata(x,y,z) as usize,!self.world.leaves_opaque,
            x,y,z,self.world.block_bounds[id as usize],|x,y,z|self.block_id(x,y,z))
    }
    pub fn should_side_render(&self,id:u8,x:i32,y:i32,z:i32,side:i32)->bool {
        block::visual181::should_side_render(id,side,self.world.block_bounds[id as usize],!self.world.leaves_opaque,
            x,y,z,|x,y,z|self.block_id(x,y,z))
    }
}
