//! BlockNote and TileEntityNote. Block events are immediate in singleplayer;
//! their audio/particle output is retained for the client.
use super::{tile::TileEntity, World};
use crate::{
    block::*,
    nbt::{Compound, Tag},
};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Note {
    pub note: i8,
    pub powered: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockEvent {
    Note {
        position: (i32, i32, i32),
        instrument: i32,
        note: i32,
        pitch: f32,
    },
    PistonSound {
        position: (i32, i32, i32),
        extending: bool,
        pitch: f32,
    },
}
pub fn tile_nbt(id: &str, x: i32, y: i32, z: i32) -> Compound {
    let mut n = Compound::new();
    n.put("id", Tag::String(id.into()));
    for (key, value) in [("x", x), ("y", y), ("z", z)] {
        n.put(key, Tag::Int(value));
    }
    n
}
impl Note {
    pub fn from_nbt(n: &Compound) -> Self {
        Self {
            note: n.byte("note").clamp(0, 24),
            powered: false,
        }
    }
    pub fn to_nbt(&self, x: i32, y: i32, z: i32) -> Compound {
        let mut n = tile_nbt("Music", x, y, z);
        n.put("note", Tag::Byte(self.note));
        n
    }
}
impl World {
    pub fn play_note_at(&mut self, x: i32, y: i32, z: i32, instrument: i32, note: i32) {
        match self.get_block_id(x, y, z) {
            29 | 33 => {
                let id = self.get_block_id(x, y, z);
                self.piston_play(id, x, y, z, instrument, note);
            }
            25 => {
                let pitch = 2.0f64.powf((note - 12) as f64 / 12.0) as f32;
                self.block_events.push(BlockEvent::Note {
                    position: (x, y, z),
                    instrument,
                    note,
                    pitch,
                });
            }
            _ => {}
        }
    }
    pub fn trigger_note(&mut self, x: i32, y: i32, z: i32) {
        let Some(TileEntity::Note(n)) = self.get_block_tile_entity(x, y, z) else {
            return;
        };
        let note = n.note as i32;
        if self.get_block_material(x, y + 1, z) == MAT_AIR {
            let instrument = match self.get_block_material(x, y - 1, z) {
                MAT_ROCK => 1,
                MAT_SAND => 2,
                MAT_GLASS => 3,
                MAT_WOOD => 4,
                _ => 0,
            };
            self.play_note_at(x, y, z, instrument, note);
        }
    }
    pub(crate) fn note_neighbor(&mut self, x: i32, y: i32, z: i32, changed: u8) {
        if changed > 0 && crate::entity::block_hooks::provides_power(changed) {
            let powered = self.is_block_getting_powered(x, y, z);
            let previous = match self.get_block_tile_entity(x, y, z) {
                Some(TileEntity::Note(n)) => n.powered,
                _ => return,
            };
            if powered != previous {
                if powered {
                    self.trigger_note(x, y, z);
                }
                if let Some(TileEntity::Note(n)) = self.get_block_tile_entity(x, y, z) {
                    n.powered = powered;
                }
            }
        }
    }
    pub fn activate_note(&mut self, x: i32, y: i32, z: i32) {
        if let Some(TileEntity::Note(n)) = self.get_block_tile_entity(x, y, z) {
            n.note = (n.note + 1) % 25;
        }
        self.mark_container_dirty(x, z);
        self.trigger_note(x, y, z);
    }
}
