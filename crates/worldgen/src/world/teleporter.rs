//! `Teleporter`: after a dimension change, put the entity at the nearest portal within 128
//! blocks, or build a portal (and a platform if needed) within 16 blocks first.

use super::World;
use crate::block::{OBSIDIAN, PORTAL};
use crate::math::floor_double;
use crate::JavaRandom;

impl World {
    /// `Teleporter.func_4107_a` for the entity `id`. `rand` is the teleporter's own
    /// `new Random()` (unseeded in vanilla).
    pub fn teleport_to_portal(&mut self, id: i32, rand: &mut JavaRandom) {
        let Some(e) = self.entities.get(&id) else { return };
        let pos = (e.pos.x, e.pos.y, e.pos.z);
        if let Some((x, y, z)) = self.teleport_position(pos, rand) {
            if let Some(e) = self.entities.get_mut(&id) {
                // setLocationAndAngles(x, y, z, rotationYaw, 0); motion = 0
                let yaw = e.yaw;
                e.set_location_and_angles(x, y, z, yaw, 0.0);
                e.motion.x = 0.0;
                e.motion.y = 0.0;
                e.motion.z = 0.0;
            }
        }
    }

    /// `func_4107_a` for an entity at `pos` (`posX/posY/posZ`): where it is placed, or
    /// `None` if it stays.
    pub fn teleport_position(&mut self, pos: (f64, f64, f64), rand: &mut JavaRandom) -> Option<(f64, f64, f64)> {
        if let Some(p) = self.find_portal(pos) {
            return Some(p);
        }
        self.create_portal(pos, rand);
        self.find_portal(pos)
    }

    /// `func_4106_b`: the nearest portal block column within 128 blocks.
    fn find_portal(&mut self, (px, py, pz): (f64, f64, f64)) -> Option<(f64, f64, f64)> {
        let r = 128;
        let mut best = -1.0f64;
        let (mut bx, mut by, mut bz) = (0, 0, 0);
        let x0 = floor_double(px);
        let z0 = floor_double(pz);
        for x in x0 - r..=x0 + r {
            let dx = x as f64 + 0.5 - px;
            for z in z0 - r..=z0 + r {
                let dz = z as f64 + 0.5 - pz;
                let mut y = 127;
                while y >= 0 {
                    if self.get_block_id(x, y, z) == PORTAL {
                        while self.get_block_id(x, y - 1, z) == PORTAL {
                            y -= 1;
                        }
                        let dy = y as f64 + 0.5 - py;
                        let d = dx * dx + dy * dy + dz * dz;
                        if best < 0.0 || d < best {
                            best = d;
                            bx = x;
                            by = y;
                            bz = z;
                        }
                    }
                    y -= 1;
                }
            }
        }
        if best < 0.0 {
            return None;
        }
        let mut x = bx as f64 + 0.5;
        let y = by as f64 + 0.5;
        let mut z = bz as f64 + 0.5;
        if self.get_block_id(bx - 1, by, bz) == PORTAL {
            x -= 0.5;
        }
        if self.get_block_id(bx + 1, by, bz) == PORTAL {
            x += 0.5;
        }
        if self.get_block_id(bx, by, bz - 1) == PORTAL {
            z -= 0.5;
        }
        if self.get_block_id(bx, by, bz + 1) == PORTAL {
            z += 0.5;
        }
        Some((x, y, z))
    }

    /// `func_4108_c`
    fn create_portal(&mut self, (px, py, pz): (f64, f64, f64), rand: &mut JavaRandom) {
        let r = 16;
        let mut best = -1.0f64;
        let x0 = floor_double(px);
        let y0 = floor_double(py);
        let z0 = floor_double(pz);
        let (mut bx, mut by, mut bz) = (x0, y0, z0);
        let mut orient = 0;
        let start = rand.next_int(4);

        for x in x0 - r..=x0 + r {
            let dx = x as f64 + 0.5 - px;
            for z in z0 - r..=z0 + r {
                let dz = z as f64 + 0.5 - pz;
                let mut y = 127;
                'column: while y >= 0 {
                    if self.is_air_block(x, y, z) {
                        while y > 0 && self.is_air_block(x, y - 1, z) {
                            y -= 1;
                        }
                        for o in start..start + 4 {
                            let mut ax = o % 2;
                            let mut az = 1 - ax;
                            if o % 4 >= 2 {
                                ax = -ax;
                                az = -az;
                            }
                            for w in 0..3 {
                                for l in 0..4 {
                                    for h in -1..4 {
                                        let bx2 = x + (l - 1) * ax + w * az;
                                        let by2 = y + h;
                                        let bz2 = z + (l - 1) * az - w * ax;
                                        if h < 0 && !self.material_is_solid(bx2, by2, bz2)
                                            || h >= 0 && !self.is_air_block(bx2, by2, bz2)
                                        {
                                            y -= 1;
                                            continue 'column;
                                        }
                                    }
                                }
                            }
                            let dy = y as f64 + 0.5 - py;
                            let d = dx * dx + dy * dy + dz * dz;
                            if best < 0.0 || d < best {
                                best = d;
                                bx = x;
                                by = y;
                                bz = z;
                                orient = o % 4;
                            }
                        }
                    }
                    y -= 1;
                }
            }
        }

        if best < 0.0 {
            for x in x0 - r..=x0 + r {
                let dx = x as f64 + 0.5 - px;
                for z in z0 - r..=z0 + r {
                    let dz = z as f64 + 0.5 - pz;
                    let mut y = 127;
                    'column2: while y >= 0 {
                        if self.is_air_block(x, y, z) {
                            // No `y > 0` guard here, as in vanilla.
                            while self.is_air_block(x, y - 1, z) {
                                y -= 1;
                            }
                            for o in start..start + 2 {
                                let ax = o % 2;
                                let az = 1 - ax;
                                for l in 0..4 {
                                    for h in -1..4 {
                                        let bx2 = x + (l - 1) * ax;
                                        let by2 = y + h;
                                        let bz2 = z + (l - 1) * az;
                                        if h < 0 && !self.material_is_solid(bx2, by2, bz2)
                                            || h >= 0 && !self.is_air_block(bx2, by2, bz2)
                                        {
                                            y -= 1;
                                            continue 'column2;
                                        }
                                    }
                                }
                                let dy = y as f64 + 0.5 - py;
                                let d = dx * dx + dy * dy + dz * dz;
                                if best < 0.0 || d < best {
                                    best = d;
                                    bx = x;
                                    by = y;
                                    bz = z;
                                    orient = o % 2;
                                }
                            }
                        }
                        y -= 1;
                    }
                }
            }
        }

        let cx = bx;
        let mut cy = by;
        let cz = bz;
        let mut ax = orient % 2;
        let mut az = 1 - ax;
        if orient % 4 >= 2 {
            ax = -ax;
            az = -az;
        }
        if best < 0.0 {
            // No room anywhere: a floating platform at y 70..118.
            by = by.clamp(70, 118);
            cy = by;
            for w in -1..=1 {
                for l in 1..3 {
                    for h in -1..3 {
                        let x = cx + (l - 1) * ax + w * az;
                        let y = cy + h;
                        let z = cz + (l - 1) * az - w * ax;
                        self.set_block_with_notify(x, y, z, if h < 0 { OBSIDIAN } else { 0 });
                    }
                }
            }
        }
        for _ in 0..4 {
            self.editing_blocks = true;
            for l in 0..4 {
                for h in -1..4 {
                    let x = cx + (l - 1) * ax;
                    let y = cy + h;
                    let z = cz + (l - 1) * az;
                    let frame = l == 0 || l == 3 || h == -1 || h == 3;
                    self.set_block_with_notify(x, y, z, if frame { OBSIDIAN } else { PORTAL });
                }
            }
            self.editing_blocks = false;
            for l in 0..4 {
                for h in -1..4 {
                    let x = cx + (l - 1) * ax;
                    let y = cy + h;
                    let z = cz + (l - 1) * az;
                    let id = self.get_block_id(x, y, z);
                    self.notify_blocks_of_neighbor_change(x, y, z, id);
                }
            }
        }
    }
}
