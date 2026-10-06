//! BlockBed callbacks; player sleep and explosion simulation remain delegated.
use crate::{
    entity::{player::SleepStatus, Vec3D},
    World,
};
pub const BED_OFFSETS181: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];
#[derive(Clone, Copy)]
pub struct BedActor181 {
    pub id: i32,
    pub sleeping: bool,
    pub bed: Option<(i32, i32, i32)>,
}
impl World {
    /// Returns the block's translated chat key, if any. Both delegated callbacks
    /// receive the same world and coordinates as the original BlockBed bytecode.
    pub fn activate_bed181(
        &mut self,
        mut x: i32,
        y: i32,
        mut z: i32,
        actor: Option<BedActor181>,
        mut sleep: impl FnMut(&mut World, i32, i32, i32) -> SleepStatus,
        mut explode: impl FnMut(&mut World, Vec3D),
    ) -> Option<&'static str> {
        if self.multiplayer_world {
            return None;
        }
        let mut metadata = self.get_block_metadata(x, y, z);
        if metadata & 8 == 0 {
            let (dx, dz) = BED_OFFSETS181[(metadata & 3) as usize];
            x = x.wrapping_add(dx);
            z = z.wrapping_add(dz);
            if self.get_block_id(x, y, z) != 26 {
                return None;
            }
            metadata = self.get_block_metadata(x, y, z);
        }
        if self.dimension.is_hell_world() {
            self.set_block_with_notify(x, y, z, 0);
            let (dx, dz) = BED_OFFSETS181[(metadata & 3) as usize];
            x = x.wrapping_add(dx);
            z = z.wrapping_add(dz);
            if self.get_block_id(x, y, z) == 26 {
                self.set_block_with_notify(x, y, z, 0);
            }
            explode(
                self,
                Vec3D::new(
                    (x as f32 + 0.5) as f64,
                    (y as f32 + 0.5) as f64,
                    (z as f32 + 0.5) as f64,
                ),
            );
            return None;
        }
        if metadata & 4 != 0 {
            // Vanilla scans the complete list, including malformed later players.
            if self
                .player_entities
                .iter()
                .flatten()
                .fold(false, |occupied, id| {
                    if let Some(actor) = actor.filter(|actor| actor.id == *id) {
                        return occupied
                            | (actor.sleeping
                                && actor.bed.expect("sleeping player has null bed coordinates")
                                    == (x, y, z));
                    }
                    occupied
                        | self
                            .entities
                            .get(id)
                            .and_then(|e| e.player_state())
                            .is_some_and(|p| {
                                p.sleeping
                                    && p.bed.expect("sleeping player has null bed coordinates")
                                        == (x, y, z)
                            })
                })
            {
                return Some("tile.bed.occupied");
            }
            self.set_bed_occupied181(x, y, z, false);
        }
        match sleep(self, x, y, z) {
            SleepStatus::Ok => self.set_bed_occupied181(x, y, z, true),
            SleepStatus::NotPossibleNow => return Some("tile.bed.noSleep"),
            _ => {}
        }
        None
    }
    pub fn set_bed_occupied181(&mut self, x: i32, y: i32, z: i32, occupied: bool) {
        let metadata = self.get_block_metadata(x, y, z);
        self.set_block_metadata_with_notify(
            x,
            y,
            z,
            if occupied {
                metadata | 4
            } else {
                metadata & !4
            },
        );
    }
}
