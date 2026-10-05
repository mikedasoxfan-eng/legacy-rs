use super::*;
impl Entity {
    pub fn activate_block(&mut self, w: &mut World, x: i32, y: i32, z: i32) -> bool {
        let id = w.get_block_id(x, y, z);
        let meta = w.get_block_metadata(x, y, z);
        match id {
            25 => { w.activate_note(x,y,z); true },
            36 => {
                if w.get_block_tile_entity(x,y,z).is_none() { w.set_block_with_notify(x,y,z,0); true } else { false }
            },
            84 => {
                if meta == 0 {
                    false
                } else {
                    w.eject_record(x, y, z);
                    true
                }
            }
            71 => true,
            64 => {
                if meta & 8 != 0 {
                    if w.get_block_id(x, y - 1, z) == id {
                        self.activate_block(w, x, y - 1, z);
                    }
                } else {
                    if w.get_block_id(x, y + 1, z) == id {
                        w.set_block_metadata_with_notify(x, y + 1, z, (meta ^ 4) + 8);
                    }
                    w.set_block_metadata_with_notify(x, y, z, meta ^ 4);
                    w.auxiliary_effect(1003, (x, y, z), 0);
                }
                true
            }
            96 => {
                w.set_block_metadata_with_notify(x, y, z, meta ^ 4);
                w.auxiliary_effect(1003, (x, y, z), 0);
                true
            }
            69 => {
                w.set_block_metadata_with_notify(x, y, z, (meta & 7) + (8 - (meta & 8)));
                w.play_sound(
                    "random.click",
                    Vec3D::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5),
                    0.3,
                    if meta & 8 == 0 { 0.6 } else { 0.5 },
                );
                notify_power(w, id, x, y, z, meta & 7);
                true
            }
            77 => {
                if meta & 8 != 0 {
                    return true;
                }
                w.set_block_metadata_with_notify(x, y, z, (meta & 7) + 8);
                w.play_sound(
                    "random.click",
                    Vec3D::new(x as f64 + 0.5, y as f64 + 0.5, z as f64 + 0.5),
                    0.3,
                    0.6,
                );
                notify_power(w, id, x, y, z, meta & 7);
                w.schedule_block_update(x, y, z, id, 20);
                true
            }
            58 | 61 | 62 | 23 => {
                if id != 58 {
                    w.get_block_tile_entity(x, y, z);
                }
                self.player_state_mut()
                    .unwrap()
                    .events
                    .push(PlayerEvent::OpenContainer {
                        block: id,
                        position: (x, y, z),
                    });
                true
            }
            54 => {
                w.get_block_tile_entity(x, y, z);
                let mut blocked = w.is_block_normal_cube(x, y + 1, z);
                for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    if w.get_block_id(x + dx, y, z + dz) == 54
                        && w.is_block_normal_cube(x + dx, y + 1, z + dz)
                    {
                        blocked = true;
                    }
                }
                if !blocked {
                    self.player_state_mut()
                        .unwrap()
                        .events
                        .push(PlayerEvent::OpenContainer {
                            block: id,
                            position: (x, y, z),
                        });
                }
                true
            }
            26 => {
                let (mut bx, mut bz) = (x, z);
                let mut m = meta;
                if m & 8 == 0 {
                    let (dx, dz) = [(0, 1), (-1, 0), (0, -1), (1, 0)][(meta & 3) as usize];
                    bx += dx;
                    bz += dz;
                    if w.get_block_id(bx, y, bz) != 26 {
                        return true;
                    }
                    m = w.get_block_metadata(bx, y, bz);
                }
                if w.dimension.is_hell_world() {
                    w.set_block_with_notify(bx, y, bz, 0);
                    let (dx, dz) = [(0, 1), (-1, 0), (0, -1), (1, 0)][(m & 3) as usize];
                    bx += dx;
                    bz += dz;
                    if w.get_block_id(bx, y, bz) == 26 {
                        w.set_block_with_notify(bx, y, bz, 0);
                    }
                    w.new_explosion(
                        None,
                        Vec3D::new(
                            (bx as f32 + 0.5) as f64,
                            (y as f32 + 0.5) as f64,
                            (bz as f32 + 0.5) as f64,
                        ),
                        5.0,
                        true,
                    );
                } else {
                    if m & 4 != 0 {
                        if w.entities.values().any(|e| {
                            e.player_state()
                                .is_some_and(|s| s.sleeping && s.bed == Some((bx, y, bz)))
                        }) {
                            self.player_state_mut().unwrap().events.push(PlayerEvent::Chat("tile.bed.occupied"));
                            return true;
                        }
                        w.set_block_metadata_with_notify(bx, y, bz, m & !4);
                    }
                    match self.sleep_in_bed(w, bx, y, bz) {
                        SleepStatus::Ok => {
                            let m = w.get_block_metadata(bx, y, bz);
                            w.set_block_metadata_with_notify(bx, y, bz, m | 4);
                        }
                        SleepStatus::NotPossibleNow => {
                            self.player_state_mut().unwrap().events.push(PlayerEvent::Chat("tile.bed.noSleep"));
                        }
                        _ => {}
                    }
                }
                true
            }
            92 => {
                if self.living.health < 20 {
                    self.heal(3);
                    if meta + 1 >= 6 {
                        w.set_block_with_notify(x, y, z, 0);
                    } else {
                        w.set_block_metadata_with_notify(x, y, z, meta + 1);
                    }
                }
                true
            }
            93 | 94 => {
                w.set_block_metadata_with_notify(
                    x,
                    y,
                    z,
                    (meta & 3) + (((meta >> 2) + 1) % 4 << 2),
                );
                true
            }
            _ => false,
        }
    }
}
pub fn notify_power(w: &mut World, id: u8, x: i32, y: i32, z: i32, orientation: i32) {
    w.notify_blocks_of_neighbor_change(x, y, z, id);
    let (dx, dy, dz) = match orientation {
        1 => (-1, 0, 0),
        2 => (1, 0, 0),
        3 => (0, 0, -1),
        4 => (0, 0, 1),
        _ => (0, -1, 0),
    };
    w.notify_blocks_of_neighbor_change(x + dx, y + dy, z + dz, id);
}
