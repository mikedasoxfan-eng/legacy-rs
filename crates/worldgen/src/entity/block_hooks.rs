use super::{living::Target, AxisAlignedBB as BB, Entity};
use crate::{block::*, World};
pub fn provides_power(id: u8) -> bool {
    matches!(id, 28 | 55 | 69 | 70 | 72 | 75 | 76 | 77)
}
impl World {
    pub(crate) fn indirectly_provides_power(&mut self, x: i32, y: i32, z: i32, side: i32) -> bool {
        if self.is_block_normal_cube(x, y, z) {
            self.is_block_getting_powered(x, y, z)
        } else {
            self.block_power(x, y, z, side, false)
        }
    }
    pub(crate) fn scatter_chest_contents(&mut self, x: i32, y: i32, z: i32) {
        let slots = match self.get_block_tile_entity(x, y, z) {
            Some(crate::world::tile::TileEntity::Chest(slots)) => slots.clone(),
            _ => return,
        };
        for item in slots.iter().flatten() {
            let ox = self.chest_random.next_float() * 0.8 + 0.1;
            let oy = self.chest_random.next_float() * 0.8 + 0.1;
            let oz = self.chest_random.next_float() * 0.8 + 0.1;
            let mut count = item.count as i32;
            while count > 0 {
                let n = (self.chest_random.next_int(21) + 10).min(count);
                count -= n;
                let mut e = Entity::item(
                    self,
                    (x as f32 + ox) as f64,
                    (y as f32 + oy) as f64,
                    (z as f32 + oz) as f64,
                    crate::world::tile::ItemStack::new(item.id as i32, n, item.damage as i32),
                );
                e.motion.x = (self.chest_random.next_gaussian() as f32 * 0.05) as f64;
                e.motion.y = (self.chest_random.next_gaussian() as f32 * 0.05 + 0.2) as f64;
                e.motion.z = (self.chest_random.next_gaussian() as f32 * 0.05) as f64;
                self.entity_joined_world(e);
            }
        }
        self.remove_block_tile_entity(x, y, z);
    }
    pub fn is_block_indirectly_getting_powered(&mut self, x: i32, y: i32, z: i32) -> bool {
        for (side, (dx, dy, dz)) in [
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
            (-1, 0, 0),
            (1, 0, 0),
        ]
        .into_iter()
        .enumerate()
        {
            if self.is_block_normal_cube(x + dx, y + dy, z + dz) {
                if self.is_block_getting_powered(x + dx, y + dy, z + dz) {
                    return true;
                }
            } else if self.block_power(x + dx, y + dy, z + dz, side as i32, false) {
                return true;
            }
        }
        false
    }
    pub fn is_block_getting_powered(&mut self, x: i32, y: i32, z: i32) -> bool {
        for (side, (dx, dy, dz)) in [
            (0, -1, 0),
            (0, 1, 0),
            (0, 0, -1),
            (0, 0, 1),
            (-1, 0, 0),
            (1, 0, 0),
        ]
        .into_iter()
        .enumerate()
        {
            if self.block_power(x + dx, y + dy, z + dz, side as i32, true) {
                return true;
            }
        }
        false
    }
    fn wire_neighbor(&mut self, x: i32, y: i32, z: i32, side: i32) -> bool {
        let id = self.get_block_id(x, y, z);
        if id == 55 {
            return true;
        }
        if id == 0 {
            return false;
        }
        if provides_power(id) {
            return true;
        }
        if id != 93 && id != 94 {
            return false;
        }
        let meta = self.get_block_metadata(x, y, z);
        side == [2, 3, 0, 1][(meta & 3) as usize]
    }
    fn block_power(&mut self, x: i32, y: i32, z: i32, side: i32, strong: bool) -> bool {
        let id = self.get_block_id(x, y, z);
        if !provides_power(id) && id != 93 && id != 94 {
            return false;
        }
        let meta = self.get_block_metadata(x, y, z);
        match id {
            70 | 72 => meta > 0 && (!strong || side == 1),
            // BlockDetectorRail
            28 => meta & 8 != 0 && (!strong || side == 1),
            69 | 77 => {
                meta & 8 != 0
                    && (!strong
                        || matches!((meta & 7, side), (5, 1) | (4, 2) | (3, 3) | (2, 4) | (1, 5))
                        || (id == 69 && meta & 7 == 6 && side == 1))
            }
            75 => false,
            76 => {
                (!strong || side == 0)
                    && !matches!((meta, side), (5, 1) | (3, 3) | (4, 2) | (1, 5) | (2, 4))
            }
            93 => false,
            94 => matches!((meta & 3, side), (0, 3) | (1, 4) | (2, 2) | (3, 5)),
            55 => {
                if meta == 0 || !self.wires_provide_power {
                    return false;
                }
                if side == 1 {
                    return true;
                }
                let mut connections = [false; 4];
                for (k, (dx, dz, direction)) in [(-1, 0, 1), (1, 0, 3), (0, -1, 2), (0, 1, 0)]
                    .into_iter()
                    .enumerate()
                {
                    connections[k] = self.wire_neighbor(x + dx, y, z + dz, direction)
                        || (!self.is_block_normal_cube(x + dx, y, z + dz)
                            && self.wire_neighbor(x + dx, y - 1, z + dz, -1));
                }
                if !self.is_block_normal_cube(x, y + 1, z) {
                    for (k, (dx, dz)) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().enumerate()
                    {
                        if self.is_block_normal_cube(x + dx, y, z + dz)
                            && self.wire_neighbor(x + dx, y + 1, z + dz, -1)
                        {
                            connections[k] = true;
                        }
                    }
                }
                let [west, east, north, south] = connections;
                (!north && !east && !west && !south && (2..=5).contains(&side))
                    || (side == 2 && north && !west && !east)
                    || (side == 3 && south && !west && !east)
                    || (side == 4 && west && !north && !south)
                    || (side == 5 && east && !north && !south)
            }
            _ => false,
        }
    }
    pub(crate) fn farmland_tick(&mut self, x: i32, y: i32, z: i32, rs: crate::world::RandSel) {
        if self.rng(rs).next_int(5) == 0 {
            let mut water = false;
            'water: for bx in x - 4..=x + 4 {
                for by in y..=y + 1 {
                    for bz in z - 4..=z + 4 {
                        if self.get_block_material(bx, by, bz) == MAT_WATER {
                            water = true;
                            break 'water;
                        }
                    }
                }
            }
            if water || self.can_block_be_rained_on(x, y + 1, z) {
                self.set_block_metadata_with_notify(x, y, z, 7);
            } else {
                let meta = self.get_block_metadata(x, y, z);
                if meta > 0 {
                    self.set_block_metadata_with_notify(x, y, z, meta - 1);
                } else if self.get_block_id(x, y + 1, z) != 59 {
                    self.set_block_with_notify(x, y, z, DIRT);
                }
            }
        }
    }
    /// Apply combat damage with an entity or non-ticking player as its source.
    pub fn damage_entity(&mut self, id: i32, source: Option<Target>, damage: i32) -> bool {
        if let Some(mut e) = self.entities.remove(&id) {
            let attacker = source.map(|s| match s {
                Target::Entity(id) => id,
                Target::Player(i) => -1 - i as i32,
            });
            let result = e.attack(self, attacker, damage);
            self.entities.insert(id, e);
            result
        } else {
            false
        }
    }
    pub fn ignite_tnt(&mut self, x: i32, y: i32, z: i32) {
        let e = Entity::tnt(
            self,
            (x as f32 + 0.5) as f64,
            (y as f32 + 0.5) as f64,
            (z as f32 + 0.5) as f64,
        );
        self.entity_joined_world(e);
    }
    pub fn update_pressure_plate(&mut self, id: u8, x: i32, y: i32, z: i32) {
        let old = self.get_block_metadata(x, y, z) == 1;
        let b = BB::new(
            (x as f32 + 0.125) as f64,
            y as f64,
            (z as f32 + 0.125) as f64,
            ((x + 1) as f32 - 0.125) as f64,
            y as f64 + 0.25,
            ((z + 1) as f32 - 0.125) as f64,
        );
        let pressed = self
            .entities_within_aabb(None, b)
            .iter()
            .any(|eid| id == PRESSURE_PLATE_WOOD || self.entities[eid].kind.is_living());
        if pressed != old {
            self.set_block_metadata_with_notify(x, y, z, pressed as i32);
            self.notify_blocks_of_neighbor_change(x, y, z, id);
            self.notify_blocks_of_neighbor_change(x, y - 1, z, id);
        }
        if pressed {
            self.schedule_block_update(x, y, z, id, 20);
        }
    }
}
