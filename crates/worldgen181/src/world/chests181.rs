//! Beta 1.8.1 chest facing and per-object lid animation state.
use super::{tile_lifecycle::TileId, World};
use crate::{block::opaque_cube_lookup as opaque, entity::Vec3D};
#[derive(Clone, Debug, Default)]
pub struct ChestAnimation {
    pub adjacency_checked: bool,
    /// North, east, west, south; retain tile identity like Java references.
    pub adjacent: [Option<TileId>; 4],
    pub lid: f32,
    pub previous_lid: f32,
    pub users: i32,
    pub ticks: i32,
}
impl World {
    pub fn chest_stack181(&self,id:TileId,slot:usize)->Option<super::tile::ItemStack> {
        let super::tile::TileEntity::Chest(items)=self.tile_object(id).unwrap() else{panic!("not a chest")};items[slot]
    }
    pub fn set_chest_stack181(&mut self,id:TileId,slot:usize,stack:Option<super::tile::ItemStack>) {
        let mut tile=self.tile_object(id).unwrap().clone();
        let super::tile::TileEntity::Chest(items)=&mut tile else{panic!("not a chest")};
        items[slot]=stack.map(|mut s|{if s.count>64{s.count=64;}s});self.update_tile_object(id,tile);self.tile_inventory_changed181(id);
    }
    pub fn decrement_chest181(&mut self,id:TileId,slot:usize,amount:i32)->Option<super::tile::ItemStack> {
        let mut tile=self.tile_object(id).unwrap().clone();
        let super::tile::TileEntity::Chest(items)=&mut tile else{panic!("not a chest")};
        let mut stack=items[slot]?;
        let result=if stack.count<=amount {items[slot]=None;stack}else{stack.count-=amount;items[slot]=if stack.count==0{None}else{Some(stack)};super::tile::ItemStack::new(stack.id as i32,amount,stack.damage as i32)};
        self.update_tile_object(id,tile);self.tile_inventory_changed181(id);Some(result)
    }
    pub fn orient_chest181(&mut self, x: i32, y: i32, z: i32) {
        let n = self.get_block_id(x, y, z - 1);
        let s = self.get_block_id(x, y, z + 1);
        let w = self.get_block_id(x - 1, y, z);
        let e = self.get_block_id(x + 1, y, z);
        let mut facing;
        if n != 54 && s != 54 {
            if w != 54 && e != 54 {
                facing = 3;
                if opaque(s) && !opaque(n) {
                    facing = 2;
                }
                if opaque(w) && !opaque(e) {
                    facing = 5;
                }
                if opaque(e) && !opaque(w) {
                    facing = 4;
                }
            } else {
                let nx = if w == 54 { x - 1 } else { x + 1 };
                let nn = self.get_block_id(nx, y, z - 1);
                let ss = self.get_block_id(nx, y, z + 1);
                facing = if self.get_block_metadata(nx, y, z) == 2 {
                    2
                } else {
                    3
                };
                if (opaque(n) || opaque(nn)) && !opaque(s) && !opaque(ss) {
                    facing = 3;
                }
                if (opaque(s) || opaque(ss)) && !opaque(n) && !opaque(nn) {
                    facing = 2;
                }
            }
        } else {
            let nz = if n == 54 { z - 1 } else { z + 1 };
            let ww = self.get_block_id(x - 1, y, nz);
            let ee = self.get_block_id(x + 1, y, nz);
            facing = if self.get_block_metadata(x, y, nz) == 4 {
                4
            } else {
                5
            };
            if (opaque(w) || opaque(ww)) && !opaque(e) && !opaque(ee) {
                facing = 5;
            }
            if (opaque(e) || opaque(ee)) && !opaque(w) && !opaque(ww) {
                facing = 4;
            }
        }
        self.set_block_metadata_with_notify(x, y, z, facing);
    }
    pub(crate) fn chest_added181(&mut self, x: i32, y: i32, z: i32) {
        self.set_block_tile_entity(x, y, z, super::tile::TileEntity::for_block(54));
        self.orient_chest181(x, y, z);
        for (dx, dz) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
            if self.get_block_id(x + dx, y, z + dz) == 54 {
                self.orient_chest181(x + dx, y, z + dz);
            }
        }
    }
    pub fn chest_placed181(&mut self, x: i32, y: i32, z: i32, yaw: f32) {
        let n = self.get_block_id(x, y, z - 1) == 54;
        let s = self.get_block_id(x, y, z + 1) == 54;
        let w = self.get_block_id(x - 1, y, z) == 54;
        let e = self.get_block_id(x + 1, y, z) == 54;
        let f = [2, 5, 3, 4]
            [(crate::math::floor_double((yaw * 4.0 / 360.0) as f64 + 0.5) & 3) as usize];
        if !(n || s || w || e) {
            self.set_block_metadata_with_notify(x, y, z, f);
        } else {
            if (n || s) && matches!(f, 4 | 5) {
                self.set_block_metadata_with_notify(x, y, if n { z - 1 } else { z + 1 }, f);
                self.set_block_metadata_with_notify(x, y, z, f);
            }
            if (w || e) && matches!(f, 2 | 3) {
                self.set_block_metadata_with_notify(if w { x - 1 } else { x + 1 }, y, z, f);
                self.set_block_metadata_with_notify(x, y, z, f);
            }
        }
    }
    pub fn chest_id181(&mut self, x: i32, y: i32, z: i32) -> Option<TileId> {
        if self.get_block_id(x, y, z) != 54 {
            return None;
        }
        self.get_block_tile_entity(x, y, z);
        self.chunk(x >> 4, z >> 4)
            .tile_entity_ids
            .get(&(x & 15, y, z & 15))
            .copied()
    }
    pub(crate) fn chest_neighbor181(&mut self, x: i32, y: i32, z: i32) {
        if let Some(id) = self.chest_id181(x, y, z) {
            self.reset_tile_cache181(id);
        }
    }
    pub fn check_chest_adjacency181(&mut self, id: TileId) {
        if self
            .chest_animations
            .entry(id)
            .or_default()
            .adjacency_checked
        {
            return;
        }
        self.chest_animations
            .get_mut(&id)
            .unwrap()
            .adjacency_checked = true;
        let (x, y, z) = self.tile_objects[&id].position;
        let adjacent = [
            self.chest_id181(x, y, z - 1),
            self.chest_id181(x + 1, y, z),
            self.chest_id181(x - 1, y, z),
            self.chest_id181(x, y, z + 1),
        ];
        self.chest_animations.get_mut(&id).unwrap().adjacent = adjacent;
        for k in [0, 3, 1, 2] {
            if let Some(other) = adjacent[k] {
                self.reset_tile_cache181(other);
            }
        }
    }
    pub fn chest_use181(&mut self, x: i32, y: i32, z: i32, opening: bool) {
        if let Some(id) = self.chest_id181(x, y, z) {
            let a = self.chest_animations.entry(id).or_default();
            a.users = a.users.wrapping_add(if opening { 1 } else { -1 });
            let users = a.users;
            self.play_note_at(x, y, z, 1, users);
        }
    }
    pub fn tick_chest181(&mut self, id: TileId) {
        self.check_chest_adjacency181(id);
        let a = self.chest_animations.get_mut(&id).unwrap();
        a.ticks = a.ticks.wrapping_add(1);
        let (x, y, z) = self.tile_objects[&id].position;
        if a.ticks % 20 * 4 == 0 {
            let users = a.users;
            self.play_note_at(x, y, z, 1, users);
        }
        let a = self.chest_animations.get_mut(&id).unwrap();
        a.previous_lid = a.lid;
        let leader = a.adjacent[0].is_none() && a.adjacent[2].is_none();
        let mut sound = if a.users > 0 && a.lid == 0.0 && leader {
            Some("random.door_open")
        } else {
            None
        };
        if (a.users == 0 && a.lid > 0.0) || (a.users > 0 && a.lid < 1.0) {
            a.lid += if a.users > 0 { 0.1f32 } else { -0.1f32 };
            if a.lid > 1.0 {
                a.lid = 1.0;
            }
            if a.lid < 0.0 {
                a.lid = 0.0;
                if leader {
                    sound = Some("random.door_close");
                }
            }
        }
        let pos = Vec3D::new(
            x as f64 + 0.5 + if a.adjacent[1].is_some() { 0.5 } else { 0.0 },
            y as f64 + 0.5,
            z as f64 + 0.5 + if a.adjacent[3].is_some() { 0.5 } else { 0.0 },
        );
        if let Some(name) = sound {
            let pitch = self.rand.next_float() * 0.1f32 + 0.9f32;
            self.play_sound(name, pos, 1.0, pitch);
        }
    }
}
