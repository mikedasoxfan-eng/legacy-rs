//! BlockJukeBox: records (note blocks are in notes.rs).
use crate::{
    entity::{Entity, EntityKind},
    world::tile::{ItemStack, TileEntity},
    World,
};
impl World {
    pub fn insert_record(&mut self, x: i32, y: i32, z: i32, record: i32) -> bool {
        if self.get_block_id(x, y, z) != 84 || self.get_block_metadata(x, y, z) != 0 {
            return false;
        }
        self.set_record181(x,y,z,record);
        self.auxiliary_effect(1005, (x, y, z), record);
        true
    }
    /// BlockJukeBox.ejectRecord is the record-setting callback despite its MCP name.
    pub fn set_record181(&mut self,x:i32,y:i32,z:i32,record:i32) {
        if self.multiplayer_world {return;}
        if let Some(TileEntity::RecordPlayer { record: stored }) =
            self.get_block_tile_entity(x, y, z)
        {
            *stored = record;
            if let Some(&tile) =
                self.chunk(x >> 4, z >> 4)
                    .tile_entity_ids
                    .get(&(x & 15, y, z & 15))
            {
                self.tile_inventory_changed181(tile);
            }
            self.set_block_metadata_with_notify(x, y, z, 1);
        }
    }
    pub fn eject_record(&mut self, x: i32, y: i32, z: i32) {
        if self.multiplayer_world {return;}
        let Some(TileEntity::RecordPlayer { record }) = self.get_block_tile_entity(x, y, z) else {
            return;
        };
        let record = *record;
        if record == 0 {
            return;
        }
        self.auxiliary_effect(1005, (x, y, z), 0);
        self.play_record(None, (x, y, z));
        if let Some(TileEntity::RecordPlayer { record }) = self.get_block_tile_entity(x, y, z) {
            *record = 0;
        }
        if let Some(&tile) = self
            .chunk(x >> 4, z >> 4)
            .tile_entity_ids
            .get(&(x & 15, y, z & 15))
        {
            self.tile_inventory_changed181(tile);
        }
        self.set_block_metadata_with_notify(x, y, z, 0);
        let s = 0.7f32;
        let ox = (self.rand.next_float() * s) as f64 + (1.0 - s) as f64 * 0.5;
        let oy = (self.rand.next_float() * s) as f64 + (1.0 - s) as f64 * 0.2 + 0.6;
        let oz = (self.rand.next_float() * s) as f64 + (1.0 - s) as f64 * 0.5;
        let mut item = Entity::item(
            self,
            x as f64 + ox,
            y as f64 + oy,
            z as f64 + oz,
            ItemStack::new(record, 1, 0),
        );
        if let EntityKind::Item { pickup_delay, .. } = &mut item.kind {
            *pickup_delay = 10;
        }
        self.entity_joined_world(item);
    }
}
