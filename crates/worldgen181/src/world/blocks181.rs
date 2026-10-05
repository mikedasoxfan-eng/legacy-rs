//! Neighbour-dependent collision and selection for the new thin blocks.
use super::World;
use crate::{block::*, entity::AxisAlignedBB as BB};

impl World {
    /// BlockFence.func_35303_f: gates join fences in the new renderer.
    pub fn fence_connects181(&mut self, x: i32, y: i32, z: i32) -> bool {
        matches!(self.get_block_id(x, y, z), 85 | 107)
    }
    fn pane_connections(&mut self, id: u8, x: i32, y: i32, z: i32) -> [bool; 4] {
        [(0, -1), (0, 1), (-1, 0), (1, 0)].map(|(dx, dz)| {
            let neighbor = self.get_block_id(x + dx, y, z + dz);
            opaque_cube_lookup(neighbor) || neighbor == id || neighbor == 20
        })
    }
    pub(crate) fn pane_boxes(&mut self, id: u8, x: i32, y: i32, z: i32) -> Vec<BB> {
        let [n, s, w, e] = self.pane_connections(id, x, y, z);
        let any = n || s || w || e;
        let mut boxes = Vec::new();
        if w && e || !any {
            boxes.push(BB::new(0.0, 0.0, 0.4375, 1.0, 1.0, 0.5625));
        } else if w {
            boxes.push(BB::new(0.0, 0.0, 0.4375, 0.5, 1.0, 0.5625));
        } else if e {
            boxes.push(BB::new(0.5, 0.0, 0.4375, 1.0, 1.0, 0.5625));
        }
        if n && s || !any {
            boxes.push(BB::new(0.4375, 0.0, 0.0, 0.5625, 1.0, 1.0));
        } else if n {
            boxes.push(BB::new(0.4375, 0.0, 0.0, 0.5625, 1.0, 0.5));
        } else if s {
            boxes.push(BB::new(0.4375, 0.0, 0.5, 0.5625, 1.0, 1.0));
        }
        boxes
    }
    pub(crate) fn thin_block_bounds(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let b = match id {
            IRON_BARS | GLASS_PANE => {
                let [n, s, w, e] = self.pane_connections(id, x, y, z);
                let any = n || s || w || e;
                BB::new(
                    if w || !any { 0.0 } else { 0.4375 },
                    0.0,
                    if n || !any { 0.0 } else { 0.4375 },
                    if e || !any { 1.0 } else { 0.5625 },
                    1.0,
                    if s || !any { 1.0 } else { 0.5625 },
                )
            }
            VINE => {
                let m = self.get_block_metadata(x, y, z);
                let mut b = BB::new(1.0, 1.0, 1.0, 0.0, 0.0, 0.0);
                if m & 2 != 0 {
                    b = BB::new(0.0, 0.0, 0.0, 0.0625, 1.0, 1.0);
                }
                if m & 8 != 0 {
                    b.min_x = b.min_x.min(0.9375);
                    b.max_x = 1.0;
                    b.min_y = 0.0;
                    b.max_y = 1.0;
                    b.min_z = 0.0;
                    b.max_z = 1.0;
                }
                if m & 4 != 0 {
                    b.max_z = b.max_z.max(0.0625);
                    b.min_z = 0.0;
                    b.min_x = 0.0;
                    b.max_x = 1.0;
                    b.min_y = 0.0;
                    b.max_y = 1.0;
                }
                if m & 1 != 0 {
                    b.min_z = b.min_z.min(0.9375);
                    b.max_z = 1.0;
                    b.min_x = 0.0;
                    b.max_x = 1.0;
                    b.min_y = 0.0;
                    b.max_y = 1.0;
                }
                if m == 0 && super::plants181::vine_support(self.get_block_id(x, y + 1, z)) {
                    b = BB::new(0.0, 0.9375, 0.0, 1.0, 1.0, 1.0);
                }
                b
            }
            _ => return false,
        };
        self.block_bounds[id as usize] = b;
        true
    }
}

impl crate::World {
    pub fn silverfish_destroyed181(&mut self,x:i32,y:i32,z:i32) {
        let mut e=crate::entity::Entity::living(self,"Silverfish").unwrap();
        e.set_location_and_angles(x as f64+0.5,y as f64,z as f64+0.5,0.0,0.0);
        let id=e.entity_id;self.entity_joined_world(e);
        if let Some(mut e)=self.entities.remove(&id) {e.explosion_particle(self);self.entities.insert(id,e);}
    }
}
