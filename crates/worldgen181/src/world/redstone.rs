//! Redstone: `BlockRedstoneWire` (current propagation, with the original's HashSet-ordered
//! neighbour notifications) and `BlockRedstoneTorch` (inversion and burnout).

use super::{RandSel, World};
use crate::world::behavior::drop_block_as_item;

/// `RedstoneUpdateInfo`: one torch flip, kept in the class-wide `torchUpdates` list.
#[derive(Clone, Copy, Debug)]
pub struct TorchUpdate {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub time: i64,
}

/// `ChunkPosition.hashCode`
fn chunk_position_hash(x: i32, y: i32, z: i32) -> i32 {
    x.wrapping_mul(8976890).wrapping_add(y.wrapping_mul(981131)).wrapping_add(z)
}

const WIRE: u8 = 55;

impl World {
    // ------------------------------------------------------------------ wire

    /// `updateAndPropagateCurrentStrength`
    fn wire_update(&mut self, x: i32, y: i32, z: i32) {
        self.wire_calculate(x, y, z, x, y, z);
        let positions: Vec<(i32, i32, i32)> = self.wire_updates.iter().collect();
        self.wire_updates.clear();
        for (px, py, pz) in positions {
            self.notify_blocks_of_neighbor_change(px, py, pz, WIRE);
        }
    }

    /// `getMaxCurrentStrength`
    fn wire_max_strength(&mut self, x: i32, y: i32, z: i32, current: i32) -> i32 {
        if self.get_block_id(x, y, z) != WIRE {
            current
        } else {
            let m = self.get_block_metadata(x, y, z);
            if m > current {
                m
            } else {
                current
            }
        }
    }

    /// `func_21030_a`: recompute this wire's strength from its sources and neighbours,
    /// then recurse into neighbours that disagree.
    fn wire_calculate(&mut self, x: i32, y: i32, z: i32, sx: i32, sy: i32, sz: i32) {
        let old = self.get_block_metadata(x, y, z);
        let mut strength = 0;
        self.wires_provide_power = false;
        let powered = self.is_block_indirectly_getting_powered(x, y, z);
        self.wires_provide_power = true;
        if powered {
            strength = 15;
        } else {
            for k in 0..4 {
                let (nx, nz) = neighbour(x, z, k);
                if nx != sx || y != sy || nz != sz {
                    strength = self.wire_max_strength(nx, y, nz, strength);
                }
                if self.is_block_normal_cube(nx, y, nz) && !self.is_block_normal_cube(x, y + 1, z) {
                    if nx != sx || y + 1 != sy || nz != sz {
                        strength = self.wire_max_strength(nx, y + 1, nz, strength);
                    }
                } else if !self.is_block_normal_cube(nx, y, nz) && (nx != sx || y - 1 != sy || nz != sz) {
                    strength = self.wire_max_strength(nx, y - 1, nz, strength);
                }
            }
            if strength > 0 {
                strength -= 1;
            } else {
                strength = 0;
            }
        }
        if old != strength {
            self.editing_blocks = true;
            self.set_block_metadata_with_notify(x, y, z, strength);
            self.editing_blocks = false;
            let mut current = strength;
            for k in 0..4 {
                let (nx, nz) = neighbour(x, z, k);
                let mut ny = y - 1;
                if self.is_block_normal_cube(nx, y, nz) {
                    ny += 2;
                }
                let n = self.wire_max_strength(nx, y, nz, -1);
                current = self.get_block_metadata(x, y, z);
                if current > 0 {
                    current -= 1;
                }
                if n >= 0 && n != current {
                    self.wire_calculate(nx, y, nz, x, y, z);
                }
                let n = self.wire_max_strength(nx, ny, nz, -1);
                current = self.get_block_metadata(x, y, z);
                if current > 0 {
                    current -= 1;
                }
                if n >= 0 && n != current {
                    self.wire_calculate(nx, ny, nz, x, y, z);
                }
            }
            if old == 0 || current == 0 {
                for (px, py, pz) in [
                    (x, y, z),
                    (x - 1, y, z),
                    (x + 1, y, z),
                    (x, y - 1, z),
                    (x, y + 1, z),
                    (x, y, z - 1),
                    (x, y, z + 1),
                ] {
                    self.wire_updates.insert(chunk_position_hash(px, py, pz), (px, py, pz));
                }
            }
        }
    }

    /// `notifyWireNeighborsOfNeighborChange`
    fn notify_wire_neighbours(&mut self, x: i32, y: i32, z: i32) {
        if self.get_block_id(x, y, z) == WIRE {
            for (nx, ny, nz) in [(x, y, z), (x - 1, y, z), (x + 1, y, z), (x, y, z - 1), (x, y, z + 1), (x, y - 1, z), (x, y + 1, z)]
            {
                self.notify_blocks_of_neighbor_change(nx, ny, nz, WIRE);
            }
        }
    }

    /// The diagonal part shared by `onBlockAdded` and `onBlockRemoval`.
    fn notify_wire_around(&mut self, x: i32, y: i32, z: i32) {
        self.notify_wire_neighbours(x - 1, y, z);
        self.notify_wire_neighbours(x + 1, y, z);
        self.notify_wire_neighbours(x, y, z - 1);
        self.notify_wire_neighbours(x, y, z + 1);
        for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if self.is_block_normal_cube(x + dx, y, z + dz) {
                self.notify_wire_neighbours(x + dx, y + 1, z + dz);
            } else {
                self.notify_wire_neighbours(x + dx, y - 1, z + dz);
            }
        }
    }

    /// `BlockRedstoneWire.onBlockAdded`
    pub(crate) fn wire_added(&mut self, x: i32, y: i32, z: i32) {
        if self.multiplayer_world {return;}
        self.wire_update(x, y, z);
        self.notify_blocks_of_neighbor_change(x, y + 1, z, WIRE);
        self.notify_blocks_of_neighbor_change(x, y - 1, z, WIRE);
        self.notify_wire_around(x, y, z);
    }

    /// `BlockRedstoneWire.onBlockRemoval`
    pub(crate) fn wire_removed(&mut self, x: i32, y: i32, z: i32) {
        if self.multiplayer_world {return;}
        self.notify_blocks_of_neighbor_change(x, y + 1, z, WIRE);
        self.notify_blocks_of_neighbor_change(x, y - 1, z, WIRE);
        self.wire_update(x, y, z);
        self.notify_wire_around(x, y, z);
    }

    /// `BlockRedstoneWire.onNeighborBlockChange`
    pub(crate) fn wire_neighbor_change(&mut self, x: i32, y: i32, z: i32) {
        if self.multiplayer_world {return;}
        let meta = self.get_block_metadata(x, y, z);
        // canPlaceBlockAt: a normal cube below.
        if !self.is_block_normal_cube(x, y - 1, z) {
            drop_block_as_item(self, WIRE, x, y, z, meta);
            self.set_block_with_notify(x, y, z, 0);
        } else {
            self.wire_update(x, y, z);
        }
    }

    // ------------------------------------------------------------------ redstone torch

    /// `BlockTorch.canPlaceBlockAt` support test for one side (`func_31032_h` below).
    fn torch_support_below(&mut self, x: i32, y: i32, z: i32) -> bool {
        self.is_block_normal_cube(x, y, z) || self.get_block_id(x, y, z) == 85
    }

    fn torch_can_stay(&mut self, x: i32, y: i32, z: i32) -> bool {
        self.is_block_normal_cube(x - 1, y, z)
            || self.is_block_normal_cube(x + 1, y, z)
            || self.is_block_normal_cube(x, y, z - 1)
            || self.is_block_normal_cube(x, y, z + 1)
            || self.torch_support_below(x, y - 1, z)
    }

    /// `BlockTorch.dropTorchIfCantStay`
    fn drop_torch_if_cant_stay(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        if !self.torch_can_stay(x, y, z) {
            let m = self.get_block_metadata(x, y, z);
            drop_block_as_item(self, id, x, y, z, m);
            self.set_block_with_notify(x, y, z, 0);
            false
        } else {
            true
        }
    }

    /// `BlockTorch.onBlockAdded`
    pub(crate) fn torch_added(&mut self, id: u8, x: i32, y: i32, z: i32) {
        if self.is_block_normal_cube(x - 1, y, z) {
            self.set_block_metadata_with_notify(x, y, z, 1);
        } else if self.is_block_normal_cube(x + 1, y, z) {
            self.set_block_metadata_with_notify(x, y, z, 2);
        } else if self.is_block_normal_cube(x, y, z - 1) {
            self.set_block_metadata_with_notify(x, y, z, 3);
        } else if self.is_block_normal_cube(x, y, z + 1) {
            self.set_block_metadata_with_notify(x, y, z, 4);
        } else if self.torch_support_below(x, y - 1, z) {
            self.set_block_metadata_with_notify(x, y, z, 5);
        }
        self.drop_torch_if_cant_stay(id, x, y, z);
    }

    /// `BlockTorch.onNeighborBlockChange`
    pub(crate) fn torch_neighbor_change(&mut self, id: u8, x: i32, y: i32, z: i32) {
        if self.drop_torch_if_cant_stay(id, x, y, z) {
            let m = self.get_block_metadata(x, y, z);
            let mut drop = false;
            if !self.is_block_normal_cube(x - 1, y, z) && m == 1 {
                drop = true;
            }
            if !self.is_block_normal_cube(x + 1, y, z) && m == 2 {
                drop = true;
            }
            if !self.is_block_normal_cube(x, y, z - 1) && m == 3 {
                drop = true;
            }
            if !self.is_block_normal_cube(x, y, z + 1) && m == 4 {
                drop = true;
            }
            if !self.torch_support_below(x, y - 1, z) && m == 5 {
                drop = true;
            }
            if drop {
                let m = self.get_block_metadata(x, y, z);
                drop_block_as_item(self, id, x, y, z, m);
                self.set_block_with_notify(x, y, z, 0);
            }
        }
    }

    fn notify_six(&mut self, id: u8, x: i32, y: i32, z: i32) {
        for (nx, ny, nz) in [(x, y - 1, z), (x, y + 1, z), (x - 1, y, z), (x + 1, y, z), (x, y, z - 1), (x, y, z + 1)] {
            self.notify_blocks_of_neighbor_change(nx, ny, nz, id);
        }
    }

    /// `BlockRedstoneTorch.onBlockAdded`
    pub(crate) fn redstone_torch_added(&mut self, id: u8, x: i32, y: i32, z: i32) {
        if self.get_block_metadata(x, y, z) == 0 {
            self.torch_added(id, x, y, z);
        }
        if id == 76 {
            self.notify_six(id, x, y, z);
        }
    }

    /// `BlockRedstoneTorch.onBlockRemoval`
    pub(crate) fn redstone_torch_removed(&mut self, id: u8, x: i32, y: i32, z: i32) {
        if id == 76 {
            self.notify_six(id, x, y, z);
        }
    }

    /// `BlockRedstoneTorch.onNeighborBlockChange`
    pub(crate) fn redstone_torch_neighbor_change(&mut self, id: u8, x: i32, y: i32, z: i32) {
        self.torch_neighbor_change(id, x, y, z);
        self.schedule_block_update(x, y, z, id, 2);
    }

    /// `checkForBurnout`
    fn torch_burnout(&mut self, x: i32, y: i32, z: i32, record: bool) -> bool {
        if record {
            self.torch_updates.push(TorchUpdate { x, y, z, time: self.world_time });
        }
        let mut n = 0;
        for u in &self.torch_updates {
            if u.x == x && u.y == y && u.z == z {
                n += 1;
                if n >= 8 {
                    return true;
                }
            }
        }
        false
    }

    /// `func_30002_h`: the block the torch hangs on is powered.
    fn torch_powered(&mut self, x: i32, y: i32, z: i32) -> bool {
        let m = self.get_block_metadata(x, y, z);
        m == 5 && self.indirectly_provides_power(x, y - 1, z, 0)
            || m == 3 && self.indirectly_provides_power(x, y, z - 1, 2)
            || m == 4 && self.indirectly_provides_power(x, y, z + 1, 3)
            || m == 1 && self.indirectly_provides_power(x - 1, y, z, 4)
            || m == 2 && self.indirectly_provides_power(x + 1, y, z, 5)
    }

    /// `BlockRedstoneTorch.updateTick`
    pub(crate) fn redstone_torch_tick(&mut self, id: u8, x: i32, y: i32, z: i32, rs: RandSel) {
        let powered = self.torch_powered(x, y, z);
        while !self.torch_updates.is_empty() && self.world_time - self.torch_updates[0].time > 100 {
            self.torch_updates.remove(0);
        }
        if id == 76 {
            if powered {
                let m = self.get_block_metadata(x, y, z);
                self.set_block_and_metadata_with_notify(x, y, z, 75, m);
                if self.torch_burnout(x, y, z, true) {
                    // playSoundEffect("random.fizz", 0.5, 2.6 + (r - r) * 0.8): World.rand
                    let a = self.rand.next_float();
                    let b = self.rand.next_float();
                    let _pitch = 2.6f32 + (a - b) * 0.8f32;
                    // Five "smoke" particles from the tick's Random.
                    for _ in 0..5 {
                        let r = self.rng(rs);
                        let _ = (r.next_double(), r.next_double(), r.next_double());
                    }
                }
            }
        } else if !powered && !self.torch_burnout(x, y, z, false) {
            let m = self.get_block_metadata(x, y, z);
            self.set_block_and_metadata_with_notify(x, y, z, 76, m);
        }
    }
}

/// Neighbour `k` of the wire's four (x-1, x+1, z-1, z+1).
fn neighbour(x: i32, z: i32, k: i32) -> (i32, i32) {
    match k {
        0 => (x - 1, z),
        1 => (x + 1, z),
        2 => (x, z - 1),
        _ => (x, z + 1),
    }
}
