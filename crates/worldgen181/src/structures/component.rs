//! StructureComponent: coordinates, metadata rotation and clipped block operations.
use super::{BlockSelector, BoundingBox as B, StructurePieceTreasure};
use crate::world::tile::{ItemStack, TileEntity};
use crate::{block, weighted, JavaRandom, World};

/// Nested chunk loading reseeds the same provider Random used by population.
/// Preserve that alias while components use a separately borrowed RNG.
pub(crate) fn random<'a>(w: &mut World, r: &'a mut JavaRandom) -> &'a mut JavaRandom {
    if let Some(epoch) = w.structure_rng_epoch {
        if epoch != w.generator_epoch {
            *r = w.prand().clone();
            w.structure_rng_epoch = Some(w.generator_epoch);
        }
    }
    r
}

#[derive(Clone, Copy, Debug)]
pub struct Component {
    pub bounds: B,
    pub orientation: i32,
    pub depth: i32,
}
impl Component {
    pub fn position(self, x: i32, y: i32, z: i32) -> (i32, i32, i32) {
        let b = self.bounds;
        let wx = match self.orientation {
            0 | 2 => b.min_x.wrapping_add(x),
            1 => b.max_x.wrapping_sub(z),
            3 => b.min_x.wrapping_add(z),
            _ => x,
        };
        let wy = if self.orientation == -1 {
            y
        } else {
            b.min_y.wrapping_add(y)
        };
        let wz = match self.orientation {
            0 => b.min_z.wrapping_add(z),
            1 | 3 => b.min_z.wrapping_add(x),
            2 => b.max_z.wrapping_sub(z),
            _ => z,
        };
        (wx, wy, wz)
    }
    pub fn rotate(self, id: i32, m: i32) -> i32 {
        let d = self.orientation;
        if id == 66 {
            if d == 1 || d == 3 {
                return if m == 1 { 0 } else { 1 };
            }
        } else if id == 64 || id == 71 {
            match d {
                0 => match m {
                    0 => return 2,
                    2 => return 0,
                    _ => {}
                },
                1 => return m.wrapping_add(1) & 3,
                3 => return m.wrapping_add(3) & 3,
                _ => {}
            }
        } else if id == 67 || id == 53 {
            match (d, m) {
                (0, 2) => return 3,
                (0, 3) => return 2,
                (1, 0) | (3, 0) => return 2,
                (1, 1) | (3, 1) => return 3,
                (1, 2) | (3, 3) => return 0,
                (1, 3) | (3, 2) => return 1,
                _ => {}
            }
        } else if id == 65 {
            match (d, m) {
                (0, 2) => return 3,
                (0, 3) => return 2,
                (1, 2) | (3, 3) => return 4,
                (1, 3) | (3, 2) => return 5,
                (1, 4) | (3, 4) => return 2,
                (1, 5) | (3, 5) => return 3,
                _ => {}
            }
        } else if id == 77 {
            match (d, m) {
                (0, 3) => return 4,
                (0, 4) => return 3,
                (1, 3) | (3, 4) => return 1,
                (1, 4) | (3, 3) => return 2,
                (1, 2) | (3, 2) => return 3,
                (1, 1) | (3, 1) => return 4,
                _ => {}
            }
        }
        m
    }
    pub fn put(self, w: &mut World, clip: B, id: i32, meta: i32, x: i32, y: i32, z: i32) {
        let (x, y, z) = self.position(x, y, z);
        if clip.contains(x, y, z) {
            w.set_block_and_metadata(x, y, z, id as u8, meta);
        }
    }
    pub fn get(self, w: &mut World, clip: B, x: i32, y: i32, z: i32) -> i32 {
        let (x, y, z) = self.position(x, y, z);
        if clip.contains(x, y, z) {
            w.get_block_id(x, y, z) as i32
        } else {
            0
        }
    }
    pub fn liquids(self, w: &mut World, clip: B) -> bool {
        let b = self.bounds;
        let x0 = b.min_x.wrapping_sub(1).max(clip.min_x);
        let y0 = b.min_y.wrapping_sub(1).max(clip.min_y);
        let z0 = b.min_z.wrapping_sub(1).max(clip.min_z);
        let x1 = b.max_x.wrapping_add(1).min(clip.max_x);
        let y1 = b.max_y.wrapping_add(1).min(clip.max_y);
        let z1 = b.max_z.wrapping_add(1).min(clip.max_z);
        let liquid = |w: &mut World, x, y, z| block::mat(w.get_block_material(x, y, z)).is_liquid;
        for x in x0..=x1 {
            for z in z0..=z1 {
                if liquid(w, x, y0, z) || liquid(w, x, y1, z) {
                    return true;
                }
            }
        }
        for x in x0..=x1 {
            for y in y0..=y1 {
                if liquid(w, x, y, z0) || liquid(w, x, y, z1) {
                    return true;
                }
            }
        }
        for z in z0..=z1 {
            for y in y0..=y1 {
                if liquid(w, x0, y, z) || liquid(w, x1, y, z) {
                    return true;
                }
            }
        }
        false
    }
    pub fn fill(self, w: &mut World, clip: B, a: [i32; 6], wall: i32, inside: i32, existing: bool) {
        self.fill_with(w, clip, a, existing, |_, edge| {
            (if edge { wall } else { inside }, 0)
        });
    }
    fn fill_with(
        self,
        w: &mut World,
        clip: B,
        a: [i32; 6],
        existing: bool,
        mut choose: impl FnMut([i32; 3], bool) -> (i32, i32),
    ) {
        let [x0, y0, z0, x1, y1, z1] = a;
        for y in y0..=y1 {
            for x in x0..=x1 {
                for z in z0..=z1 {
                    if !existing || self.get(w, clip, x, y, z) != 0 {
                        let (id, m) = choose(
                            [x, y, z],
                            y == y0 || y == y1 || x == x0 || x == x1 || z == z0 || z == z1,
                        );
                        self.put(w, clip, id, m, x, y, z);
                    }
                }
            }
        }
    }
    pub fn fill_stones(
        self,
        w: &mut World,
        clip: B,
        a: [i32; 6],
        existing: bool,
        r: &mut JavaRandom,
        selector: &mut BlockSelector,
    ) {
        let [x0, y0, z0, x1, y1, z1] = a;
        for y in y0..=y1 {
            for x in x0..=x1 {
                for z in z0..=z1 {
                    if !existing || self.get(w, clip, x, y, z) != 0 {
                        selector.stronghold_stone(
                            random(w, r),
                            x,
                            y,
                            z,
                            y == y0 || y == y1 || x == x0 || x == x1 || z == z0 || z == z1,
                        );
                        self.put(w, clip, selector.block, selector.metadata, x, y, z);
                    }
                }
            }
        }
    }
    pub fn random_fill(
        self,
        w: &mut World,
        clip: B,
        r: &mut JavaRandom,
        probability: f32,
        a: [i32; 6],
        wall: i32,
        inside: i32,
        existing: bool,
    ) {
        let [x0, y0, z0, x1, y1, z1] = a;
        for y in y0..=y1 {
            for x in x0..=x1 {
                for z in z0..=z1 {
                    if random(w, r).next_float() <= probability
                        && (!existing || self.get(w, clip, x, y, z) != 0)
                    {
                        self.put(
                            w,
                            clip,
                            if y == y0 || y == y1 || x == x0 || x == x1 || z == z0 || z == z1 {
                                wall
                            } else {
                                inside
                            },
                            0,
                            x,
                            y,
                            z,
                        );
                    }
                }
            }
        }
    }
    pub fn random_put(
        self,
        w: &mut World,
        clip: B,
        r: &mut JavaRandom,
        probability: f32,
        id: i32,
        meta: i32,
        x: i32,
        y: i32,
        z: i32,
    ) {
        if random(w, r).next_float() < probability {
            self.put(w, clip, id, meta, x, y, z);
        }
    }
    pub fn dome(self, w: &mut World, clip: B, a: [i32; 6], id: i32, existing: bool) {
        let [x0, y0, z0, x1, y1, z1] = a;
        let dx = (x1 - x0 + 1) as f32;
        let dy = (y1 - y0 + 1) as f32;
        let dz = (z1 - z0 + 1) as f32;
        let cx = x0 as f32 + dx / 2.0;
        let cz = z0 as f32 + dz / 2.0;
        for y in y0..=y1 {
            let fy = (y - y0) as f32 / dy;
            for x in x0..=x1 {
                let fx = (x as f32 - cx) / (dx * 0.5);
                for z in z0..=z1 {
                    let fz = (z as f32 - cz) / (dz * 0.5);
                    if (!existing || self.get(w, clip, x, y, z) != 0)
                        && fx * fx + fy * fy + fz * fz <= 1.05
                    {
                        self.put(w, clip, id, 0, x, y, z);
                    }
                }
            }
        }
    }
    pub fn clear_up(self, w: &mut World, clip: B, x: i32, y: i32, z: i32) {
        let (x, mut y, z) = self.position(x, y, z);
        if clip.contains(x, y, z) {
            while !w.is_air_block(x, y, z) && y < 127 {
                w.set_block_and_metadata(x, y, z, 0, 0);
                y += 1;
            }
        }
    }
    pub fn fill_down(self, w: &mut World, clip: B, id: i32, meta: i32, x: i32, y: i32, z: i32) {
        let (x, mut y, z) = self.position(x, y, z);
        if clip.contains(x, y, z) {
            while (w.is_air_block(x, y, z) || block::mat(w.get_block_material(x, y, z)).is_liquid)
                && y > 1
            {
                w.set_block_and_metadata(x, y, z, id as u8, meta);
                y -= 1;
            }
        }
    }
    pub fn chest(
        self,
        w: &mut World,
        clip: B,
        r: &mut JavaRandom,
        x: i32,
        y: i32,
        z: i32,
        loot: &[StructurePieceTreasure],
        count: i32,
    ) {
        let (x, y, z) = self.position(x, y, z);
        if clip.contains(x, y, z) && w.get_block_id(x, y, z) != 54 {
            w.set_block_with_notify(x, y, z, 54);
            random(w, r);
            if let Some(TileEntity::Chest(slots)) = w.get_block_tile_entity(x, y, z) {
                Self::loot(r, loot, slots, count);
            }
        }
    }
    pub fn loot(
        r: &mut JavaRandom,
        loot: &[StructurePieceTreasure],
        slots: &mut [Option<ItemStack>; 27],
        count: i32,
    ) {
        for _ in 0..count {
            let t = weighted::choose(r, loot).expect("empty treasure list");
            let n = t.min + r.next_int(t.max - t.min + 1);
            let limit = crate::entity::player::ItemStack::new(t.item, 1, 0).max_stack();
            if limit >= n {
                slots[r.next_int(27) as usize] = Some(ItemStack::new(t.item, n, t.metadata));
            } else {
                for _ in 0..n {
                    slots[r.next_int(27) as usize] = Some(ItemStack::new(t.item, 1, t.metadata));
                }
            }
        }
    }
    pub fn door(self, w: &mut World, clip: B, x: i32, y: i32, z: i32, mut direction: i32) {
        let (x, y, z) = self.position(x, y, z);
        if !clip.contains(x, y, z) {
            return;
        }
        let (dx, dz) = match direction {
            0 => (0, 1),
            1 => (-1, 0),
            2 => (0, -1),
            3 => (1, 0),
            _ => (0, 0),
        };
        let left = w.is_block_normal_cube(x - dx, y, z - dz) as i32
            + w.is_block_normal_cube(x - dx, y + 1, z - dz) as i32;
        let right = w.is_block_normal_cube(x + dx, y, z + dz) as i32
            + w.is_block_normal_cube(x + dx, y + 1, z + dz) as i32;
        let left_door =
            w.get_block_id(x - dx, y, z - dz) == 64 || w.get_block_id(x - dx, y + 1, z - dz) == 64;
        let right_door =
            w.get_block_id(x + dx, y, z + dz) == 64 || w.get_block_id(x + dx, y + 1, z + dz) == 64;
        if (left_door && !right_door) || right > left {
            direction = ((direction - 1) & 3) + 4;
        }
        w.editing_blocks = true;
        w.set_block_and_metadata_with_notify(x, y, z, 64, direction);
        w.set_block_and_metadata_with_notify(x, y + 1, z, 64, direction + 8);
        w.editing_blocks = false;
        w.notify_blocks_of_neighbor_change(x, y, z, 64);
        w.notify_blocks_of_neighbor_change(x, y + 1, z, 64);
    }
}
