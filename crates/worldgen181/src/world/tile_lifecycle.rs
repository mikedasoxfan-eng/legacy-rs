//! World/Chunk tile ownership. IDs represent Java object identity, independently of
//! the chunk's position map. Replaced objects may still be in the update list.
use super::{tile::TileEntity, World};
use crate::block::props;

pub type TileId = u64;
pub struct TileObject {
    pub cached_metadata: i32,
    pub cached_block: Option<u8>,
    pub position: (i32, i32, i32),
    pub invalid: bool,
    // Attached objects live in Chunk.tile_entities for the inventory APIs. An
    // overwritten, deferred or unloaded object retains its own payload here.
    pub detached: Option<TileEntity>,
}

impl World {
    /// Chunk.onChunkUnload queues tiles without invalidating them. Their world
    /// reference and payload survive until World.updateEntities removes the list.
    pub(crate) fn unload_chunk_tiles181(&mut self,cx:i32,cz:i32) {
        if let Some(c)=self.chunks.get(&(cx,cz)) {
            for id in c.tile_entity_ids.values() {
                self.unloaded_tile_entities181.push(*id);
            }
        }
    }
    /// Preserve tile payloads only when the provider actually discards the chunk.
    /// A queued but retained chunk and its tiles still share the same payload.
    pub fn detach_unloaded_chunk_tiles181(&mut self,cx:i32,cz:i32) {
        if let Some(c)=self.chunks.get(&(cx,cz)) {
            for (key,id) in &c.tile_entity_ids {
                self.tile_objects.get_mut(id).unwrap().detached=c.tile_entities.get(key).cloned();
            }
        }
    }
    pub fn tile_metadata181(&mut self,id:TileId)->i32 {
        if self.tile_objects[&id].cached_metadata == -1 {
            let (x,y,z)=self.tile_objects[&id].position;
            let meta=self.get_block_metadata(x,y,z);self.tile_objects.get_mut(&id).unwrap().cached_metadata=meta;
        }
        self.tile_objects[&id].cached_metadata
    }
    pub fn tile_block181(&mut self,id:TileId)->Option<u8> {
        if self.tile_objects[&id].cached_block.is_none() {
            let (x,y,z)=self.tile_objects[&id].position;let block=self.get_block_id(x,y,z);
            self.tile_objects.get_mut(&id).unwrap().cached_block=if block==0 {None}else{Some(block)};
        }
        self.tile_objects[&id].cached_block
    }
    pub fn reset_tile_cache181(&mut self,id:TileId) {
        let o=self.tile_objects.get_mut(&id).unwrap();o.cached_metadata=-1;o.cached_block=None;
        if let Some(a)=self.chest_animations.get_mut(&id) {a.adjacency_checked=false;}
    }
    /// TileEntityChest.invalidate resets and refreshes adjacency before the
    /// base invalid flag changes; other tile classes only set that flag.
    pub fn invalidate_tile181(&mut self, id: TileId) {
        if self.chest_animations.contains_key(&id) {
            self.reset_tile_cache181(id);
            self.check_chest_adjacency181(id);
        }
        self.tile_objects.get_mut(&id).unwrap().invalid = true;
    }
    pub fn tile_inventory_changed181(&mut self,id:TileId) {
        let (x,y,z)=self.tile_objects[&id].position;
        let meta=self.get_block_metadata(x,y,z);self.tile_objects.get_mut(&id).unwrap().cached_metadata=meta;
        self.touch(x>>4,z>>4);
    }
    pub(crate) fn new_tile_object(&mut self, position: (i32, i32, i32), te: TileEntity) -> TileId {
        let chest=matches!(&te,TileEntity::Chest(_));
        let id = self.next_tile_id;
        self.next_tile_id += 1;
        if chest {self.chest_animations.insert(id,Default::default());}
        self.tile_objects.insert(
            id,
            TileObject {
                cached_metadata: -1,
                cached_block: None,
                position,
                invalid: false,
                detached: Some(te),
            },
        );
        id
    }
    pub fn tile_object(&self, id: TileId) -> Option<&TileEntity> {
        let o = self.tile_objects.get(&id)?;
        if let Some(te) = &o.detached {
            return Some(te);
        }
        let (x, y, z) = o.position;
        self.chunks
            .get(&(x >> 4, z >> 4))?
            .tile_entities
            .get(&(x & 15, y, z & 15))
    }
    pub(crate) fn update_tile_object(&mut self, id: TileId, te: TileEntity) {
        let o = self.tile_objects.get_mut(&id).unwrap();
        if o.detached.is_some() {
            o.detached = Some(te);
        } else {
            let (x, y, z) = o.position;
            self.chunks
                .get_mut(&(x >> 4, z >> 4))
                .unwrap()
                .tile_entities
                .insert((x & 15, y, z & 15), te);
        }
    }
    pub(crate) fn detach_chunk_tile(&mut self, x: i32, y: i32, z: i32) -> Option<TileId> {
        let c = self.chunks.get_mut(&(x >> 4, z >> 4))?;
        let key = (x & 15, y, z & 15);
        let id = c.tile_entity_ids.remove(&key)?;
        self.tile_objects.get_mut(&id).unwrap().detached = c.tile_entities.remove(&key);
        c.tile_entity_order.retain(|p| *p != key);
        Some(id)
    }
    pub(crate) fn insert_chunk_tile(&mut self, id: TileId) {
        let (x, y, z) = self.tile_objects[&id].position;
        if !self.access_chunk(x >> 4, z >> 4) {
            return;
        }
        let block = self.chunk(x >> 4, z >> 4).block(x & 15, y, z & 15);
        if block == 0 || !props(block).container {
            return;
        }
        let key = (x & 15, y, z & 15);
        if self.chunk(x >> 4, z >> 4).tile_entity_ids.get(&key) == Some(&id) {
            self.tile_objects.get_mut(&id).unwrap().invalid = false;
            return;
        }
        let order = self.chunk(x >> 4, z >> 4).tile_entity_order.clone();
        let replaced = self.detach_chunk_tile(x, y, z).is_some();
        let o = self.tile_objects.get_mut(&id).unwrap();
        o.invalid = false; // TileEntity.validate
        let te = o.detached.take().unwrap();
        let c = self.chunk_mut(x >> 4, z >> 4);
        c.tile_entity_ids.insert(key, id);
        if replaced {
            c.tile_entity_order = order;
        } else {
            c.tile_entity_order.push(key);
        }
        c.tile_entities.insert(key, te);
    }
    pub(crate) fn set_tile_object(&mut self, id: TileId) {
        if self.tile_objects[&id].invalid {
            return;
        }
        if self.scanning_tiles {
            self.pending_tile_entities.push(id);
        } else {
            self.loaded_tile_entities.push(id);
            self.insert_chunk_tile(id);
        }
    }
    /// Chunk.getChunkBlockTileEntity: lazy creation happens before invalid removal.
    pub fn get_block_tile_entity(&mut self, x: i32, y: i32, z: i32) -> Option<&mut TileEntity> {
        if !self.access_chunk(x >> 4, z >> 4) {
            return None;
        }
        let key = (x & 15, y, z & 15);
        if !self.chunk(x >> 4, z >> 4).tile_entities.contains_key(&key) {
            let block = self.chunk(x >> 4, z >> 4).block(x & 15, y, z & 15);
            if !props(block).container || block==36 {
                return None;
            }
            self.set_block_tile_entity(x,y,z,TileEntity::for_block(block));
        }
        if let Some(&id) = self.chunk(x >> 4, z >> 4).tile_entity_ids.get(&key) {
            if self.tile_objects[&id].invalid {
                self.detach_chunk_tile(x, y, z);
                return None;
            }
        }
        self.chunk_mut(x >> 4, z >> 4).tile_entities.get_mut(&key)
    }
    pub fn set_block_tile_entity(&mut self, x: i32, y: i32, z: i32, te: TileEntity) {
        let id = self.new_tile_object((x, y, z), te);
        self.set_tile_object(id);
    }
    pub(crate) fn remove_chunk_tile(&mut self, x: i32, y: i32, z: i32) {
        if self.access_chunk(x >> 4, z >> 4) && self.chunk(x >> 4, z >> 4).loaded {
            if let Some(id) = self.detach_chunk_tile(x, y, z) {
                self.invalidate_tile181(id);
            }
            self.chunk_mut(x >> 4, z >> 4)
                .raw_tile_entities
                .retain(|n| n.int("x") != x || n.int("y") != y || n.int("z") != z);
            self.dispenser_random.remove(&(x, y, z));
            self.spawner_animations.remove(&(x, y, z));
        }
    }
    pub fn remove_block_tile_entity(&mut self, x: i32, y: i32, z: i32) {
        if !self.access_chunk(x >> 4, z >> 4) {
            return;
        }
        self.get_block_tile_entity(x, y, z);
        let id = self
            .chunk(x >> 4, z >> 4)
            .tile_entity_ids
            .get(&(x & 15, y, z & 15))
            .copied();
        if let Some(id) = id {
            if self.scanning_tiles {
                self.invalidate_tile181(id);
                return;
            }
            if let Some(i) = self.loaded_tile_entities.iter().position(|p| *p == id) {
                self.loaded_tile_entities.remove(i);
            }
        }
        self.remove_chunk_tile(x, y, z);
    }
    /// World.updateEntities tile pass. The callback permits oracle probes to run
    /// real block hooks from inside updateEntity without adding a synthetic tile.
    pub fn scan_tile_entities(&mut self, mut update: impl FnMut(&mut World, TileId)) {
        self.scanning_tiles = true;
        let mut i = 0;
        while i < self.loaded_tile_entities.len() {
            let id = self.loaded_tile_entities[i];
            if !self.tile_objects[&id].invalid {
                update(self, id);
            }
            if self.tile_objects[&id].invalid {
                self.loaded_tile_entities.remove(i);
                let (x, y, z) = self.tile_objects[&id].position;
                if self.chunk_exists(x>>4,z>>4) {self.remove_chunk_tile(x, y, z);}
            } else {
                i += 1;
            }
        }
        self.finish_tile_scan();
    }
    pub(crate) fn finish_tile_scan(&mut self) {
        self.scanning_tiles = false;
        if !self.unloaded_tile_entities181.is_empty() {
            self.loaded_tile_entities.retain(|id|!self.unloaded_tile_entities181.contains(id));
            self.unloaded_tile_entities181.clear();
        }
        let pending = std::mem::take(&mut self.pending_tile_entities);
        for id in pending {
            if !self.tile_objects[&id].invalid {
                if !self.loaded_tile_entities.contains(&id) {
                    self.loaded_tile_entities.push(id);
                }
                if self.chunk_exists(self.tile_objects[&id].position.0>>4,self.tile_objects[&id].position.2>>4) {
                    self.insert_chunk_tile(id);
                }
                let (x, _, z) = self.tile_objects[&id].position;
                self.touch(x >> 4, z >> 4);
            }
        }
        // Only collect objects with no remaining Java owner.
        let mut owners: std::collections::HashSet<_> =
            self.loaded_tile_entities.iter().copied().collect();
        for c in self.chunks.values() {
            owners.extend(c.tile_entity_ids.values().copied());
        }
        self.tile_objects.retain(|id, _| owners.contains(id));
        self.chest_animations.retain(|id,_|owners.contains(id));
    }
}
