//! BlockNote, TileEntityNote and BlockJukeBox presentation callbacks.
use crate::{
    entity::{Entity, EntityKind, Vec3D},
    nbt::{Compound, Tag},
    world::tile::ItemStack,
    World,
};
impl World {
    fn music_tile(&mut self, x: i32, y: i32, z: i32, id: u8) -> &mut Compound {
        crate::entity::player::block_hooks::ensure_container(self, id, x, y, z);
        self.chunk_mut(x >> 4, z >> 4)
            .raw_tile_entities
            .iter_mut()
            .find(|n| n.int("x") == x && n.int("y") == y && n.int("z") == z)
            .unwrap()
    }
    pub fn play_note_at(&mut self, x: i32, y: i32, z: i32, instrument: i32, note: i32) {
        if self.get_block_id(x, y, z) != 25 {
            return;
        }
        let pitch = 2.0f64.powf((note - 12) as f64 / 12.0) as f32;
        let name = match instrument {
            1 => "note.bd",
            2 => "note.snare",
            3 => "note.hat",
            4 => "note.bassattack",
            _ => "note.harp",
        };
        self.play_sound(
            name,
            Vec3D::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5),
            3.0,
            pitch,
        );
        self.spawn_particle(
            "note",
            Vec3D::new(x as f64 + 0.5, y as f64 + 1.2, z as f64 + 0.5),
            Vec3D::new(note as f64 / 24.0, 0.0, 0.0),
        );
    }
    pub fn trigger_note(&mut self, x: i32, y: i32, z: i32, change_pitch: bool) {
        let n = self.music_tile(x, y, z, 25);
        let mut note = n.byte("note").clamp(0, 24);
        if change_pitch {
            note = (note + 1) % 25;
            n.put("note", Tag::Byte(note));
            self.chunk_mut(x >> 4, z >> 4).modified = true;
        }
        if self.get_block_material(x, y + 1, z) != crate::block::MAT_AIR {
            return;
        }
        let instrument = match self.get_block_material(x, y - 1, z) {
            crate::block::MAT_ROCK => 1,
            crate::block::MAT_SAND => 2,
            crate::block::MAT_GLASS => 3,
            crate::block::MAT_WOOD => 4,
            _ => 0,
        };
        self.play_note_at(x, y, z, instrument, note as i32);
    }
    pub fn insert_record(&mut self, x: i32, y: i32, z: i32, record: i32) -> bool {
        if self.get_block_id(x, y, z) != 84 || self.get_block_metadata(x, y, z) != 0 {
            return false;
        }
        self.music_tile(x, y, z, 84).put("Record", Tag::Int(record));
        self.set_block_metadata_with_notify(x, y, z, 1);
        self.auxiliary_effect(1005, (x, y, z), record);
        true
    }
    pub fn eject_record(&mut self, x: i32, y: i32, z: i32) {
        let record = self.music_tile(x, y, z, 84).int("Record");
        if record == 0 {
            return;
        }
        self.auxiliary_effect(1005, (x, y, z), 0);
        self.play_record(None, (x, y, z));
        self.music_tile(x, y, z, 84).put("Record", Tag::Int(0));
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
