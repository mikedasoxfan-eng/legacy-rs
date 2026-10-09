use super::*;
impl Entity {
    pub fn activate_block(&mut self, w: &mut World, x: i32, y: i32, z: i32) -> bool {
        let id = w.get_block_id(x, y, z);
        let meta = w.get_block_metadata(x, y, z);
        match id {
            118=>crate::world::blocks10::cauldron_activate(self,w,x,y,z),
            122=>{crate::world::blocks10::dragon_egg_teleport(w,x,y,z);true},
            73 | 74 => { w.activate_redstone_ore181(id,x,y,z); false },
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
                    w.mark_blocks_dirty181([x,y-1,z,x,y,z]);
                    w.auxiliary_effect(1003, (x, y, z), 0);
                }
                true
            }
            107 => {
                let m=if meta&4!=0 {meta & !4} else {
                    let quarter=crate::math::floor_double((self.yaw*4.0/360.0) as f64+0.5)&3;
                    (if meta&3==(quarter+2)%4 {quarter} else {meta}) | 4
                };
                w.set_block_metadata_with_notify(x,y,z,m);
                w.auxiliary_effect(1003,(x,y,z),0);
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
            58 | 61 | 62 | 23 | 116 | 117 => {
                if matches!(id, 23 | 61 | 62 | 116 | 117) && w.multiplayer_world { return true; }
                // BlockEnchantmentTable opens its GUI without looking up (or lazily creating) a tile.
                if id != 58 && id != 116 && w.get_block_tile_entity(x,y,z).is_none() {
                    return true;
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
                if w.get_block_tile_entity(x,y,z).is_none() {return true;}
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
                let state=self.player_state().unwrap();
                let actor=crate::world::beds181::BedActor181{id:self.entity_id,sleeping:state.sleeping,bed:state.bed};
                let chat=w.activate_bed181(x,y,z,Some(actor),
                    |w,x,y,z|self.sleep_in_bed(w,x,y,z),
                    |w,position|{w.new_explosion(None,position,5.0,true);});
                if let Some(key)=chat {self.player_state_mut().unwrap().events.push(PlayerEvent::Chat(key));}
                true
            }
            92 => {
                if self.player_state().unwrap().food.needs_food() && !self.player_state().unwrap().capabilities.disable_damage {
                    self.player_state_mut().unwrap().food.add_food(2,0.1);
                    if meta + 1 >= 6 {
                        w.set_block_with_notify(x, y, z, 0);
                    } else {
                        w.set_block_metadata_with_notify(x, y, z, meta + 1);
                        w.mark_blocks_dirty181([x,y,z,x,y,z]);
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
