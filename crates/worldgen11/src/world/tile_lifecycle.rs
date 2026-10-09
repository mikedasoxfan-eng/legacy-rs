//! World/Chunk tile ownership. IDs represent Java object identity, independently of
//! the chunk's position map. Replaced objects may still be in the update list.
use super::{tile::TileEntity, World};
use crate::block::props;

#[cfg(test)]
mod release11_coordinate_tests {
    #[test]
    fn chunk_tile_coordinates_match_original_11() {
        for line in include_str!("../fixtures/chunktiles11.txt").lines() {
            let v:Vec<i32>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();
            let mut w=crate::entity::verify181::world();
            w.scanning_tiles=v[3]!=0;w.multiplayer_world=v[4]!=0;
            let changed=if v[6]!=0 {w.set_block_and_metadata(v[0],70,v[1],v[2] as u8,v[5])}else{w.set_block(v[0],70,v[1],v[2] as u8)};
            let mut observed=vec![changed as i32];
            for ids in [&w.loaded_tile_entities,&w.pending_tile_entities] {
                observed.push(ids.len() as i32);
                for id in ids {let (x,y,z)=w.tile_objects[id].position;observed.extend([x,y,z]);}
            }
            assert_eq!(observed,&v[7..],"{line}");
        }
    }
}

pub type TileId = u64;
pub struct TileObject {
    pub base181: super::tile_nbt181::TileBase181,
    pub world_attached181: bool,
    pub dispenser_random181: Option<crate::JavaRandom>,
    // World.field_34900_Q membership: removeAll after the current tile pass.
    pub pending_unload: bool,
    // Attached objects live in Chunk.tile_entities for the inventory APIs. An
    // overwritten, deferred or unloaded object retains its own payload here.
    pub detached: Option<TileEntity>,
}
impl std::ops::Deref for TileObject {
    type Target = super::tile_nbt181::TileBase181;
    fn deref(&self)->&Self::Target { &self.base181 }
}
impl std::ops::DerefMut for TileObject {
    fn deref_mut(&mut self)->&mut Self::Target { &mut self.base181 }
}

impl World {
    pub(crate) fn tile_id10(&mut self, x: i32, y: i32, z: i32) -> Option<TileId> {
        self.chunk(x >> 4, z >> 4).tile_entity_ids.get(&(x & 15, y, z & 15)).copied()
            .or_else(|| self.pending_tile_entities.iter().copied().find(|id| {
                let o = &self.tile_objects[id]; !o.invalid && o.position == (x, y, z)
            }))
    }
    pub fn container_added181(&mut self, block: u8, x: i32, y: i32, z: i32) {
        if block != 36 { self.set_block_tile_entity(x, y, z, TileEntity::for_block(block)); }
    }
    /// BlockContainer.playBlock forwards to the tile's event override.
    pub fn container_play181(&mut self, x: i32, y: i32, z: i32, event: i32, value: i32) {
        let chest = matches!(self.get_block_tile_entity(x,y,z),Some(TileEntity::Chest(_)));
        if chest && event == 1 {
            let id = self.tile_id10(x, y, z).unwrap();
            self.chest_animations.get_mut(&id).unwrap().users = value;
        }
    }
    /// The two Chunk block setters refresh container caches after onBlockAdded.
    /// 1.1 fixes both block setters' fallback to use world coordinates.
    pub(crate) fn refresh_changed_block_tile181(&mut self, cx: i32, cz: i32, x: i32, y: i32, z: i32, block: u8, old: u8) {
        // 1.0 Chunk.setBlockID*: these use Chunk.getChunkBlockTileEntity, which
        // (unlike 1.0 World.getBlockTileEntity) never searches addedTileEntityList.
        if props(block).container || (block == 0 && props(old).container) {
            match self.chunk_tile_id10(cx * 16 + x, y, cz * 16 + z) {
                None => if props(block).container && block != 36 {
                    self.set_block_tile_entity(cx*16+x, y, cz*16+z, TileEntity::for_block(block));
                },
                Some(id) => self.reset_tile_cache181(id),
            }
        }
    }
    /// 1.0 `Chunk.getChunkBlockTileEntity`: lazy creation through World.setBlockTileEntity,
    /// then removal of an invalid entry. No World.addedTileEntityList fallback.
    pub(crate) fn chunk_tile_id10(&mut self, x: i32, y: i32, z: i32) -> Option<TileId> {
        if !self.access_chunk(x >> 4, z >> 4) {
            return None;
        }
        let key = (x & 15, y, z & 15);
        if !self.chunk(x >> 4, z >> 4).tile_entities.contains_key(&key) {
            let block = self.chunk(x >> 4, z >> 4).block(x & 15, y, z & 15);
            if props(block).container && block != 36 {
                self.set_block_tile_entity(x,y,z,TileEntity::for_block(block));
            }
        }
        let id = self.chunk(x >> 4, z >> 4).tile_entity_ids.get(&key).copied()?;
        if self.tile_objects[&id].invalid {
            self.detach_chunk_tile(x, y, z);
            return None;
        }
        Some(id)
    }
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
        let mut base=self.tile_objects[&id].base181.clone();
        let attached=self.tile_objects[&id].world_attached181;
        let result=base.metadata(attached.then_some(&mut *self)).expect("TileEntity.getBlockMetadata");
        self.tile_objects.get_mut(&id).unwrap().base181=base;
        result
    }
    pub fn tile_block181(&mut self,id:TileId)->Option<u8> {
        let mut base=self.tile_objects[&id].base181.clone();
        let attached=self.tile_objects[&id].world_attached181;
        let result=base.block(attached.then_some(&mut *self)).expect("TileEntity.getBlockType");
        self.tile_objects.get_mut(&id).unwrap().base181=base;
        result
    }
    pub fn reset_tile_cache181(&mut self,id:TileId) {
        self.tile_objects.get_mut(&id).unwrap().reset_cache();
        if let Some(a)=self.chest_animations.get_mut(&id) {a.adjacency_checked=false;}
    }
    /// TileEntityChest.invalidate resets and refreshes adjacency before the
    /// base invalid flag changes; other tile classes only set that flag.
    pub fn invalidate_tile181(&mut self, id: TileId) {
        if self.chest_animations.contains_key(&id) {
            self.reset_tile_cache181(id);
            self.check_chest_adjacency181(id);
        }
        self.tile_objects.get_mut(&id).unwrap().invalidate();
    }
    pub fn tile_inventory_changed181(&mut self,id:TileId) {
        let mut base=self.tile_objects[&id].base181.clone();
        let attached=self.tile_objects[&id].world_attached181;
        base.inventory_changed(attached.then_some(&mut *self));
        self.tile_objects.get_mut(&id).unwrap().base181=base;
    }
    pub(crate) fn new_tile_object(&mut self, position: (i32, i32, i32), te: TileEntity) -> TileId {
        let chest=matches!(&te,TileEntity::Chest(_));
        if matches!(&te,TileEntity::Dispenser(_)) {self.dispenser_random.remove(&position);}
        let dispenser_random181 = matches!(&te, TileEntity::Dispenser(_)).then(|| crate::JavaRandom::new(crate::entity::EntitySeedSource::Time.next_seed()));
        let id = self.next_tile_id;
        self.next_tile_id += 1;
        if chest {self.chest_animations.insert(id,Default::default());}
        self.tile_objects.insert(
            id,
            TileObject {
                base181: super::tile_nbt181::TileBase181 {position, ..Default::default()},
                world_attached181: true,
                dispenser_random181,
                pending_unload: false,
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
        if self.chunk_tile_id10(x, y, z).is_some() {
            return self.chunk_mut(x >> 4, z >> 4).tile_entities.get_mut(&(x & 15, y, z & 15));
        }
        // 1.0 World.getBlockTileEntity: first valid addedTileEntityList entry at the position.
        let pending = self.pending_tile_entities.iter().copied().find(|id| {
            let o = &self.tile_objects[id];
            !o.invalid && o.position == (x, y, z)
        });
        pending.and_then(|id| self.tile_objects.get_mut(&id).unwrap().detached.as_mut())
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
        let id = self.chunk(x >> 4, z >> 4).tile_entity_ids.get(&(x & 15, y, z & 15)).copied()
            .or_else(|| self.pending_tile_entities.iter().copied().find(|id| {
                let o = &self.tile_objects[id]; !o.invalid && o.position == (x, y, z)
            }));
        if let Some(id) = id {
            // 1.0 World.removeBlockTileEntity: List.remove drops the first occurrence only.
            if let Some(i) = self.pending_tile_entities.iter().position(|p| *p == id) {
                self.pending_tile_entities.remove(i);
            }
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
            let (x, y, z) = self.tile_objects[&id].position;
            if !self.tile_objects[&id].invalid && self.tile_objects[&id].world_attached181 && self.block_exists(x, y, z) {
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
    /// World.func_35455_a queues removal after the current tile pass.
    pub fn defer_tile_entity_removal10(&mut self,id:TileId) {self.tile_entity_removal10.push(id);}
    pub(crate) fn finish_tile_scan(&mut self) {
        self.scanning_tiles = false;
        if !self.unloaded_tile_entities181.is_empty() {
            self.loaded_tile_entities.retain(|id|!self.unloaded_tile_entities181.contains(id));
            self.unloaded_tile_entities181.clear();
        }
        if !self.tile_entity_removal10.is_empty() {
            self.loaded_tile_entities.retain(|id|!self.tile_entity_removal10.contains(id));
            self.tile_entity_removal10.clear();
        }
        crate::profiler::end_start_section("pendingTileEntities");
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
