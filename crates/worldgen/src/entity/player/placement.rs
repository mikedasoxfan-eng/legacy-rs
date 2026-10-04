use super::*;
use crate::block::*;
pub fn offset(x: i32, y: i32, z: i32, side: i32) -> (i32, i32, i32) {
    let (dx, dy, dz) = [
        (0, -1, 0),
        (0, 1, 0),
        (0, 0, -1),
        (0, 0, 1),
        (-1, 0, 0),
        (1, 0, 0),
    ]
    .get(side as usize)
    .copied()
    .unwrap_or((0, 0, 0));
    (x + dx, y + dy, z + dz)
}
pub fn can_place(w: &mut World, id: u8, x: i32, y: i32, z: i32, side: i32) -> bool {
    if y < 0 || y >= 128 {
        return false;
    }
    let existing = w.get_block_id(x, y, z);
    if w.block_collision_box(id, x, y, z)
        .is_some_and(|b| !w.check_if_aabb_is_clear(b))
    {
        return false;
    }
    if !matches!(existing, 0 | 8..=11 | 51 | 78) {
        return false;
    }
    can_place_at(w, id, x, y, z, side)
}
pub fn can_place_at(w: &mut World, id: u8, x: i32, y: i32, z: i32, side: i32) -> bool {
    match id {
        34 | 36 => false,
        50 | 75 | 76 => {
            w.is_block_normal_cube(x - 1, y, z)
                || w.is_block_normal_cube(x + 1, y, z)
                || w.is_block_normal_cube(x, y, z - 1)
                || w.is_block_normal_cube(x, y, z + 1)
                || w.is_block_normal_cube(x, y - 1, z)
                || (id == 50 && w.get_block_id(x, y - 1, z) == 85)
        }
        69 | 77 | 96 => {
            if side == 1 && id == 69 {
                return w.is_block_normal_cube(x, y - 1, z);
            }
            let (bx, by, bz) = offset(
                x,
                y,
                z,
                match side {
                    2 => 3,
                    3 => 2,
                    4 => 5,
                    5 => 4,
                    _ => return false,
                },
            );
            w.is_block_normal_cube(bx, by, bz)
        }
        65 => {
            w.is_block_normal_cube(x - 1, y, z)
                || w.is_block_normal_cube(x + 1, y, z)
                || w.is_block_normal_cube(x, y, z - 1)
                || w.is_block_normal_cube(x, y, z + 1)
        }
        64 | 71 => {
            y < 127
                && w.is_block_normal_cube(x, y - 1, z)
                && matches!(w.get_block_id(x, y, z), 0 | 78)
                && matches!(w.get_block_id(x, y + 1, z), 0 | 78)
        }
        86 | 91 | 70 | 72 | 93 | 94 => w.is_block_normal_cube(x, y - 1, z),
        92 => mat(w.get_block_material(x, y - 1, z)).is_solid,
        6 | 31 | 37 | 38 => matches!(w.get_block_id(x, y - 1, z), 2 | 3 | 60),
        32 => w.get_block_id(x, y - 1, z) == 12,
        39 | 40 => opaque_cube_lookup(w.get_block_id(x, y - 1, z)),
        81 => crate::world::behavior::cactus_can_stay(w, x, y, z),
        83 => crate::world::behavior::reed_can_place(w, x, y, z),
        78 => crate::world::behavior::snow_can_place(w, x, y, z),
        54 => {
            let mut neighbors = 0;
            for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                if w.get_block_id(x + dx, y, z + dz) == 54 {
                    neighbors += 1;
                    for (sx, sz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        if w.get_block_id(x + dx + sx, y, z + dz + sz) == 54 {
                            return false;
                        }
                    }
                }
            }
            neighbors <= 1
        }
        _ => true,
    }
}
impl Entity {
    /// ItemBlock.onItemUse; the player must be visible in World for placement collisions.
    pub fn place_block_item(
        &mut self,
        w: &mut World,
        stack: &mut ItemStack,
        mut x: i32,
        mut y: i32,
        mut z: i32,
        mut side: i32,
    ) -> bool {
        if w.get_block_id(x, y, z) == 78 {
            side = 0;
        } else {
            (x, y, z) = offset(x, y, z, side);
        }
        if stack.count == 0 {
            return false;
        }
        let id = stack.id as u8;
        if y == 127 && mat(material_of(id)).is_solid {
            return false;
        }
        w.entities.insert(self.entity_id, self.clone());
        let place = can_place(w, id, x, y, z, side);
        w.entities.remove(&self.entity_id);
        if !place {
            return false;
        }
        let meta = match id {
            29 | 33 => 7,
            6 | 17 | 35 | 44 => stack.damage,
            18 => stack.damage | 4,
            _ => 0,
        };
        if w.set_block_and_metadata_with_notify(x, y, z, id, meta) {
            self.block_placed(w, id, x, y, z, side);
            stack.count -= 1;
        }
        true
    }
    pub fn block_placed(&self, w: &mut World, id: u8, x: i32, y: i32, z: i32, side: i32) {
        let quarter = floor((self.yaw * 4.0 / 360.0) as f64 + 0.5) & 3;
        let meta = match id {
            29 | 33 => {
                let eye=self.pos.y+1.82-self.y_offset as f64;
                let vertical=if (self.pos.x as f32-x as f32).abs()<2.0 && (self.pos.z as f32-z as f32).abs()<2.0 {
                    if eye-y as f64>2.0 {Some(1)} else if y as f64-eye>0.0 {Some(0)} else {None}
                } else {None};
                Some(vertical.unwrap_or([2,5,3,4][quarter as usize]))
            }
            53 | 67 => Some([2, 1, 3, 0][quarter as usize]),
            61 | 62 | 23 => Some([2, 5, 3, 4][quarter as usize]),
            86 | 91 => Some(floor((self.yaw * 4.0 / 360.0) as f64 + 2.5) & 3),
            96 => Some(match side {
                3 => 1,
                4 => 2,
                5 => 3,
                _ => 0,
            }),
            93 | 94 => Some((quarter + 2) % 4),
            65 => {
                let mut meta = w.get_block_metadata(x, y, z);
                for (s, dx, dz) in [(2, 0, 1), (3, 0, -1), (4, 1, 0), (5, -1, 0)] {
                    if (meta == 0 || side == s) && w.is_block_normal_cube(x + dx, y, z + dz) {
                        meta = s;
                    }
                }
                Some(meta)
            }
            50 | 75 | 76 | 69 | 77 => {
                let mut meta = w.get_block_metadata(x, y, z) & 7;
                if side == 1
                    && id != 77
                    && (w.is_block_normal_cube(x, y - 1, z)
                        || (id == 50 && w.get_block_id(x, y - 1, z) == 85))
                {
                    meta = if id == 69 { 5 + w.rand.next_int(2) } else { 5 };
                }
                for (s, dx, dz, m) in [(2, 0, 1, 4), (3, 0, -1, 3), (4, 1, 0, 2), (5, -1, 0, 1)] {
                    if side == s && w.is_block_normal_cube(x + dx, y, z + dz) {
                        meta = m;
                    }
                }
                Some(meta)
            }
            _ => None,
        };
        if let Some(meta) = meta {
            w.set_block_metadata_with_notify(x, y, z, meta);
            if matches!(id,29|33) { w.piston_update(id,x,y,z); }
            if matches!(id, 93 | 94) && super::block_hooks::repeater_input(w, x, y, z, meta) {
                w.schedule_block_update(x, y, z, id, 1);
            }
        }
    }
    pub fn place_door(
        &self,
        w: &mut World,
        stack: &mut ItemStack,
        x: i32,
        y: i32,
        z: i32,
        side: i32,
    ) -> bool {
        if side != 1 {
            return false;
        }
        let y = y + 1;
        let id = if stack.id == 324 { 64 } else { 71 };
        if !can_place_at(w, id, x, y, z, side) {
            return false;
        }
        let mut dir = floor(((self.yaw + 180.0) * 4.0 / 360.0) as f64 - 0.5) & 3;
        let (dx, dz) = [(0, 1), (-1, 0), (0, -1), (1, 0)][dir as usize];
        let left = w.is_block_normal_cube(x - dx, y, z - dz) as i32
            + w.is_block_normal_cube(x - dx, y + 1, z - dz) as i32;
        let right = w.is_block_normal_cube(x + dx, y, z + dz) as i32
            + w.is_block_normal_cube(x + dx, y + 1, z + dz) as i32;
        let left_door =
            w.get_block_id(x - dx, y, z - dz) == id || w.get_block_id(x - dx, y + 1, z - dz) == id;
        let right_door =
            w.get_block_id(x + dx, y, z + dz) == id || w.get_block_id(x + dx, y + 1, z + dz) == id;
        if (left_door && !right_door) || right > left {
            dir = ((dir - 1) & 3) + 4;
        }
        w.editing_blocks = true;
        w.set_block_and_metadata_with_notify(x, y, z, id, dir);
        w.set_block_and_metadata_with_notify(x, y + 1, z, id, dir + 8);
        w.editing_blocks = false;
        w.notify_blocks_of_neighbor_change(x, y, z, id);
        w.notify_blocks_of_neighbor_change(x, y + 1, z, id);
        stack.count -= 1;
        true
    }
    pub fn place_bed(
        &self,
        w: &mut World,
        stack: &mut ItemStack,
        x: i32,
        y: i32,
        z: i32,
        side: i32,
    ) -> bool {
        if side != 1 {
            return false;
        }
        let y = y + 1;
        let dir = floor((self.yaw * 4.0 / 360.0) as f64 + 0.5) & 3;
        let (dx, dz) = [(0, 1), (-1, 0), (0, -1), (1, 0)][dir as usize];
        if w.get_block_id(x, y, z) != 0
            || w.get_block_id(x + dx, y, z + dz) != 0
            || !w.is_block_normal_cube(x, y - 1, z)
            || !w.is_block_normal_cube(x + dx, y - 1, z + dz)
        {
            return false;
        }
        w.set_block_and_metadata_with_notify(x, y, z, 26, dir);
        w.set_block_and_metadata_with_notify(x + dx, y, z + dz, 26, dir + 8);
        stack.count -= 1;
        true
    }
}
