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
    let existing = w.get_block_id(x, y, z);
    if w.block_collision_box(id, x, y, z)
        .is_some_and(|b| !w.check_if_aabb_is_clear(b))
    {
        return false;
    }
    if !matches!(existing, 0 | 8..=11 | 51 | 78 | VINE) {
        return false;
    }
    can_place_at(w, id, x, y, z, side)
}
pub fn can_place_at(w: &mut World, id: u8, x: i32, y: i32, z: i32, side: i32) -> bool {
    let target=w.get_block_id(x,y,z);
    let base=target==0 || mat(material_of(target)).ground_cover;
    match id {
        55 => w.is_block_normal_cube(x,y-1,z),
        96 => w.trapdoor_can_place_on_side181(x,y,z,side),
        VINE => crate::world::plants181::vine_can_place(w,x,y,z,side),
        FENCE_GATE => mat(w.get_block_material(x,y-1,z)).is_solid
            && (w.get_block_id(x,y,z)==0 || mat(w.get_block_material(x,y,z)).ground_cover),
        PUMPKIN_STEM | MELON_STEM => base && w.get_block_id(x,y-1,z)==TILLED_FIELD,
        34 | 36 => false,
        50 | 75 | 76 => {
            w.is_block_normal_cube(x - 1, y, z)
                || w.is_block_normal_cube(x + 1, y, z)
                || w.is_block_normal_cube(x, y, z - 1)
                || w.is_block_normal_cube(x, y, z + 1)
                || w.is_block_normal_cube(x, y - 1, z)
                || (id == 50 && w.get_block_id(x, y - 1, z) == 85)
        }
        69 | 77 => {
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
                && base
                && {let above=w.get_block_id(x,y+1,z);above==0 || mat(material_of(above)).ground_cover}
        }
        70 | 72 => w.is_block_normal_cube(x,y-1,z) || w.get_block_id(x,y-1,z)==85,
        86 | 91 => base && w.is_block_normal_cube(x,y-1,z),
        93 | 94 => w.is_block_normal_cube(x, y - 1, z) && base,
        27 | 28 | 66 => w.is_block_normal_cube(x, y - 1, z),
        92 => base && mat(w.get_block_material(x, y - 1, z)).is_solid,
        6 | 31 | 37 | 38 => base && matches!(w.get_block_id(x, y - 1, z), 2 | 3 | 60),
        32 => base && w.get_block_id(x, y - 1, z) == 12,
        39 | 40 => base && opaque_cube_lookup(w.get_block_id(x,y-1,z)),
        81 => base && crate::world::behavior::cactus_can_stay(w, x, y, z),
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
        _ => base,
    }
}
impl Entity {
    /// EntityPlayer.func_35190_e: Beta 1.8.1's singleplayer base implementation.
    pub fn can_edit_block181(&self, _x: i32, _y: i32, _z: i32) -> bool { true }
    /// ItemBlock.onItemUse; the player must be visible in World for placement collisions.
    pub fn place_block_item(
        &mut self, w: &mut World, stack: &mut ItemStack,
        x: i32, y: i32, z: i32, side: i32,
    ) -> bool {
        let item=crate::entity::item_block::BlockItem::registered(stack.id);
        item.on_item_use(&mut LiveBlockItemContext {player:self,world:w,side:0},Some(stack),[x,y,z],side)
            .expect("registered ItemBlock callback")
    }
    pub fn block_placed(&self, w: &mut World, id: u8, x: i32, y: i32, z: i32, side: i32) {
        let quarter = floor((self.yaw * 4.0 / 360.0) as f64 + 0.5) & 3;
        if id==54 {w.chest_placed181(x,y,z,self.yaw);return;}
        let meta = match id {
            29 | 33 => {
                let eye=self.pos.y+1.82-self.y_offset as f64;
                let vertical=if (self.pos.x as f32-x as f32).abs()<2.0 && (self.pos.z as f32-z as f32).abs()<2.0 {
                    if eye-y as f64>2.0 {Some(1)} else if y as f64-eye>0.0 {Some(0)} else {None}
                } else {None};
                Some(vertical.unwrap_or([2,5,3,4][quarter as usize]))
            }
            53 | 67 | STAIRS_BRICK | STAIRS_STONE_BRICK => Some([2, 1, 3, 0][quarter as usize]),
            61 | 62 | 23 => Some([2, 5, 3, 4][quarter as usize]),
            86 | 91 => Some(floor((self.yaw * 4.0 / 360.0) as f64 + 2.5) & 3),
            96 => Some(match side {
                3 => 1,
                4 => 2,
                5 => 3,
                _ => 0,
            }),
            FENCE_GATE => Some(quarter),
            VINE => match side {2=>Some(1),3=>Some(4),4=>Some(8),5=>Some(2),_=>None},
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
        if !self.can_edit_block181(x,y,z) || !self.can_edit_block181(x,y+1,z) || !can_place_at(w, id, x, y, z, side) {
            return false;
        }
        let dir = floor(((self.yaw + 180.0) * 4.0 / 360.0) as f64 - 0.5) & 3;
        place_door181(w, x, y, z, dir, id);
        stack.count = stack.count.wrapping_sub(1);
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
        let y = y.wrapping_add(1);
        let dir = floor((self.yaw * 4.0 / 360.0) as f64 + 0.5) & 3;
        let (dx, dz) = [(0, 1), (-1, 0), (0, -1), (1, 0)][dir as usize];
        if !self.can_edit_block181(x,y,z) || !self.can_edit_block181(x+dx,y,z+dz)
            || w.get_block_id(x, y, z) != 0
            || w.get_block_id(x + dx, y, z + dz) != 0
            || !w.is_block_normal_cube(x, y - 1, z)
            || !w.is_block_normal_cube(x + dx, y - 1, z + dz)
        {
            return false;
        }
        w.set_block_and_metadata_with_notify(x, y, z, 26, dir);
        w.set_block_and_metadata_with_notify(x + dx, y, z + dz, 26, dir + 8);
        stack.count = stack.count.wrapping_sub(1);
        true
    }
}

/// ItemDoor.func_35434_a, also used by vanilla structure generation.
pub fn place_door181(w: &mut World, x: i32, y: i32, z: i32, mut dir: i32, id: u8) {
        let (dx, dz) = match dir {0=>(0,1),1=>(-1,0),2=>(0,-1),3=>(1,0),_=>(0,0)};
        let left = w.is_block_normal_cube(x - dx, y, z - dz) as i32
            + w.is_block_normal_cube(x - dx, y + 1, z - dz) as i32;
        let right = w.is_block_normal_cube(x + dx, y, z + dz) as i32
            + w.is_block_normal_cube(x + dx, y + 1, z + dz) as i32;
        let left_door =
            w.get_block_id(x - dx, y, z - dz) == id || w.get_block_id(x - dx, y + 1, z - dz) == id;
        let right_door =
            w.get_block_id(x + dx, y, z + dz) == id || w.get_block_id(x + dx, y + 1, z + dz) == id;
        if (left_door && !right_door) || right > left {
            dir = (dir.wrapping_sub(1) & 3) + 4;
        }
        w.editing_blocks = true;
        w.set_block_and_metadata_with_notify(x, y, z, id, dir);
        w.set_block_and_metadata_with_notify(x, y + 1, z, id, dir.wrapping_add(8));
        w.editing_blocks = false;
        w.notify_blocks_of_neighbor_change(x, y, z, id);
        w.notify_blocks_of_neighbor_change(x, y + 1, z, id);
}

/// ItemBlock's ordered class dispatch borrows the pre-existing world callbacks.
struct LiveBlockItemContext<'a> {
    player: &'a mut Entity,
    world: &'a mut World,
    side: i32,
}
impl crate::entity::item_block::BlockItemContext for LiveBlockItemContext<'_> {
    type Block = u8;
    fn get_block_id(&mut self, p: [i32; 3]) -> Result<i32, crate::entity::item::ItemError> {
        Ok(self.world.get_block_id(p[0], p[1], p[2]) as i32)
    }
    fn can_edit(
        &mut self,
        p: [i32; 3],
        _stack: &mut ItemStack,
    ) -> Result<bool, crate::entity::item::ItemError> {
        Ok(self.player.can_edit_block181(p[0], p[1], p[2]))
    }
    fn solid(&mut self, id: i32) -> Result<bool, crate::entity::item::ItemError> {
        Ok(mat(material_of(id as u8)).is_solid)
    }
    fn can_place(
        &mut self,
        id: i32,
        p: [i32; 3],
        side: i32,
    ) -> Result<bool, crate::entity::item::ItemError> {
        self.world
            .entities
            .insert(self.player.entity_id, self.player.clone());
        let result = can_place(self.world, id as u8, p[0], p[1], p[2], side);
        self.world.entities.remove(&self.player.entity_id);
        Ok(result)
    }
    fn block(&mut self, id: i32) -> Result<u8, crate::entity::item::ItemError> {
        Ok(id as u8)
    }
    fn placed_metadata(
        &mut self,
        id: i32,
        damage: i32,
    ) -> Result<i32, crate::entity::item::ItemError> {
        Ok(super::registry::placed_block_metadata(id, damage))
    }
    fn set_block(
        &mut self,
        id: i32,
        p: [i32; 3],
        meta: i32,
    ) -> Result<bool, crate::entity::item::ItemError> {
        Ok(self
            .world
            .set_block_and_metadata_with_notify(p[0], p[1], p[2], id as u8, meta))
    }
    fn on_placed(
        &mut self,
        _id: u8,
        _p: [i32; 3],
        side: i32,
    ) -> Result<(), crate::entity::item::ItemError> {
        self.side = side;
        Ok(())
    }
    fn on_placed_by(&mut self, id: u8, p: [i32; 3]) -> Result<(), crate::entity::item::ItemError> {
        // Existing helper combines the two vanilla block callbacks.
        // Remembering the side is handled below by the context's on_placed.
        self.player
            .block_placed(self.world, id, p[0], p[1], p[2], self.side);
        Ok(())
    }
    fn sound(&mut self, id: u8, p: [f64; 3]) -> Result<(), crate::entity::item::ItemError> {
        let (name, volume, pitch) = crate::world::events::step_sound(id);
        self.world.play_sound(
            name,
            Vec3D::new(p[0], p[1], p[2]),
            (volume + 1.0) / 2.0,
            pitch * 0.8,
        );
        Ok(())
    }
}
