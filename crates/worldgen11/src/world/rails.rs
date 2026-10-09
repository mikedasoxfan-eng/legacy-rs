//! `BlockRail` (rails 66, powered rails 27, detector rails 28) and `RailLogic`: how a rail
//! picks its shape from its neighbours and reshapes them, and powered-rail propagation.

use super::World;
use crate::world::behavior::drop_block_as_item;

pub const RAIL: u8 = 66;
pub const POWERED_RAIL: u8 = 27;
pub const DETECTOR_RAIL: u8 = 28;

/// `BlockRail.isRailBlock`
pub fn is_rail(id: u8) -> bool {
    matches!(id, RAIL | POWERED_RAIL | DETECTOR_RAIL)
}

/// `BlockRail.isPowered` (the field: powered and detector rails, whose bit 8 isn't shape).
fn has_power_bit(id: u8) -> bool {
    id == POWERED_RAIL || id == DETECTOR_RAIL
}

/// `RailLogic`
struct RailLogic {
    x: i32,
    y: i32,
    z: i32,
    powered: bool,
    tracks: Vec<(i32, i32, i32)>,
}

impl RailLogic {
    fn new(w: &mut World, x: i32, y: i32, z: i32) -> RailLogic {
        let id = w.get_block_id(x, y, z);
        let mut meta = w.get_block_metadata(x, y, z);
        let powered = has_power_bit(id);
        if powered {
            meta &= -9;
        }
        let mut r = RailLogic { x, y, z, powered, tracks: Vec::new() };
        r.set_connections(meta);
        r
    }

    fn set_connections(&mut self, meta: i32) {
        let (x, y, z) = (self.x, self.y, self.z);
        self.tracks.clear();
        let pair = match meta {
            0 => [(x, y, z - 1), (x, y, z + 1)],
            1 => [(x - 1, y, z), (x + 1, y, z)],
            2 => [(x - 1, y, z), (x + 1, y + 1, z)],
            3 => [(x - 1, y + 1, z), (x + 1, y, z)],
            4 => [(x, y + 1, z - 1), (x, y, z + 1)],
            5 => [(x, y, z - 1), (x, y + 1, z + 1)],
            6 => [(x + 1, y, z), (x, y, z + 1)],
            7 => [(x - 1, y, z), (x, y, z + 1)],
            8 => [(x - 1, y, z), (x, y, z - 1)],
            9 => [(x + 1, y, z), (x, y, z - 1)],
            _ => return,
        };
        self.tracks.extend(pair);
    }

    /// `func_785_b`: keep only connections that connect back.
    fn refresh(&mut self, w: &mut World) {
        let mut i = 0;
        while i < self.tracks.len() {
            match logic_at(w, self.tracks[i]) {
                Some(o) if o.is_connected_to(self) => {
                    self.tracks[i] = (o.x, o.y, o.z);
                    i += 1;
                }
                _ => {
                    self.tracks.remove(i);
                }
            }
        }
    }

    fn is_connected_to(&self, o: &RailLogic) -> bool {
        self.tracks.iter().any(|&(x, _, z)| x == o.x && z == o.z)
    }

    fn is_in_track(&self, x: i32, _y: i32, z: i32) -> bool {
        self.tracks.iter().any(|&(tx, _, tz)| tx == x && tz == z)
    }

    /// `isMinecartTrack`: a rail at, above or below.
    fn is_track(w: &mut World, x: i32, y: i32, z: i32) -> bool {
        is_rail(w.get_block_id(x, y, z)) || is_rail(w.get_block_id(x, y + 1, z)) || is_rail(w.get_block_id(x, y - 1, z))
    }

    /// `getAdjacentTracks`
    fn adjacent_tracks(&self, w: &mut World) -> i32 {
        let (x, y, z) = (self.x, self.y, self.z);
        [(x, z - 1), (x, z + 1), (x - 1, z), (x + 1, z)]
            .into_iter()
            .filter(|&(nx, nz)| Self::is_track(w, nx, y, nz))
            .count() as i32
    }

    /// `handleKeyPress` (sic): can `o` join this rail's connections?
    fn can_connect(&self, o: &RailLogic) -> bool {
        if self.is_connected_to(o) {
            true
        } else if self.tracks.len() == 2 {
            false
        } else {
            // Empty, or one connection: the original's slope test answers true either way.
            true
        }
    }

    /// `func_788_d`: add `o` as a connection and reshape.
    fn connect_to(&mut self, w: &mut World, o: &RailLogic) {
        self.tracks.push((o.x, o.y, o.z));
        let (x, y, z) = (self.x, self.y, self.z);
        let n = self.is_in_track(x, y, z - 1);
        let s = self.is_in_track(x, y, z + 1);
        let west = self.is_in_track(x - 1, y, z);
        let east = self.is_in_track(x + 1, y, z);
        let mut shape = -1;
        if n || s {
            shape = 0;
        }
        if west || east {
            shape = 1;
        }
        if !self.powered {
            if s && east && !n && !west {
                shape = 6;
            }
            if s && west && !n && !east {
                shape = 7;
            }
            if n && west && !s && !east {
                shape = 8;
            }
            if n && east && !s && !west {
                shape = 9;
            }
        }
        shape = slope(w, x, y, z, shape);
        if shape < 0 {
            shape = 0;
        }
        let mut meta = shape;
        if self.powered {
            meta = w.get_block_metadata(x, y, z) & 8 | shape;
        }
        w.set_block_metadata_with_notify(x, y, z, meta);
    }

    /// `func_786_c`: whether the rail at (x, y, z) would accept this one.
    fn neighbour_accepts(&self, w: &mut World, x: i32, y: i32, z: i32) -> bool {
        match logic_at(w, (x, y, z)) {
            None => false,
            Some(mut o) => {
                o.refresh(w);
                o.can_connect(self)
            }
        }
    }

    /// `func_792_a(powered, force)`: choose this rail's shape and reshape its neighbours.
    fn place(mut self, w: &mut World, power: bool, force: bool) {
        let (x, y, z) = (self.x, self.y, self.z);
        let n = self.neighbour_accepts(w, x, y, z - 1);
        let s = self.neighbour_accepts(w, x, y, z + 1);
        let west = self.neighbour_accepts(w, x - 1, y, z);
        let east = self.neighbour_accepts(w, x + 1, y, z);
        let mut shape = -1;
        if (n || s) && !west && !east {
            shape = 0;
        }
        if (west || east) && !n && !s {
            shape = 1;
        }
        if !self.powered {
            if s && east && !n && !west {
                shape = 6;
            }
            if s && west && !n && !east {
                shape = 7;
            }
            if n && west && !s && !east {
                shape = 8;
            }
            if n && east && !s && !west {
                shape = 9;
            }
        }
        if shape == -1 {
            if n || s {
                shape = 0;
            }
            if west || east {
                shape = 1;
            }
            if !self.powered {
                if power {
                    if s && east {
                        shape = 6;
                    }
                    if west && s {
                        shape = 7;
                    }
                    if east && n {
                        shape = 9;
                    }
                    if n && west {
                        shape = 8;
                    }
                } else {
                    if n && west {
                        shape = 8;
                    }
                    if east && n {
                        shape = 9;
                    }
                    if west && s {
                        shape = 7;
                    }
                    if s && east {
                        shape = 6;
                    }
                }
            }
        }
        shape = slope(w, x, y, z, shape);
        if shape < 0 {
            shape = 0;
        }
        self.set_connections(shape);
        let mut meta = shape;
        if self.powered {
            meta = w.get_block_metadata(x, y, z) & 8 | shape;
        }
        if force || w.get_block_metadata(x, y, z) != meta {
            w.set_block_metadata_with_notify(x, y, z, meta);
            for i in 0..self.tracks.len() {
                if let Some(mut o) = logic_at(w, self.tracks[i]) {
                    o.refresh(w);
                    if o.can_connect(&self) {
                        o.connect_to(w, &self);
                    }
                }
            }
        }
    }
}

/// Straight shapes become slopes toward a rail one block up.
fn slope(w: &mut World, x: i32, y: i32, z: i32, mut shape: i32) -> i32 {
    if shape == 0 {
        if is_rail(w.get_block_id(x, y + 1, z - 1)) {
            shape = 4;
        }
        if is_rail(w.get_block_id(x, y + 1, z + 1)) {
            shape = 5;
        }
    }
    if shape == 1 {
        if is_rail(w.get_block_id(x + 1, y + 1, z)) {
            shape = 2;
        }
        if is_rail(w.get_block_id(x - 1, y + 1, z)) {
            shape = 3;
        }
    }
    shape
}

/// `getMinecartTrackLogic`: the rail at, above or below a position.
fn logic_at(w: &mut World, (x, y, z): (i32, i32, i32)) -> Option<RailLogic> {
    if is_rail(w.get_block_id(x, y, z)) {
        Some(RailLogic::new(w, x, y, z))
    } else if is_rail(w.get_block_id(x, y + 1, z)) {
        Some(RailLogic::new(w, x, y + 1, z))
    } else if is_rail(w.get_block_id(x, y - 1, z)) {
        Some(RailLogic::new(w, x, y - 1, z))
    } else {
        None
    }
}

impl World {
    /// `BlockRail.func_4031_h`
    fn rail_update_shape(&mut self, x: i32, y: i32, z: i32, force: bool) {
        let power = self.is_block_indirectly_getting_powered(x, y, z);
        RailLogic::new(self, x, y, z).place(self, power, force);
    }

    /// `BlockRail.onBlockAdded`
    pub(crate) fn rail_added(&mut self, x: i32, y: i32, z: i32) {
        if self.multiplayer_world {return;}
        self.rail_update_shape(x, y, z, true);
        if self.get_block_id(x,y,z)==POWERED_RAIL {self.rail_neighbor_change(POWERED_RAIL,x,y,z,POWERED_RAIL);}
    }

    /// `BlockRail.onNeighborBlockChange`
    pub(crate) fn rail_neighbor_change(&mut self, id: u8, x: i32, y: i32, z: i32, changed: u8) {
        let meta = self.get_block_metadata(x, y, z);
        let shape = if has_power_bit(id) { meta & 7 } else { meta };
        let mut drop = !self.is_block_normal_cube(x, y - 1, z);
        if shape == 2 && !self.is_block_normal_cube(x + 1, y, z) {
            drop = true;
        }
        if shape == 3 && !self.is_block_normal_cube(x - 1, y, z) {
            drop = true;
        }
        if shape == 4 && !self.is_block_normal_cube(x, y, z - 1) {
            drop = true;
        }
        if shape == 5 && !self.is_block_normal_cube(x, y, z + 1) {
            drop = true;
        }
        if drop {
            let m = self.get_block_metadata(x, y, z);
            drop_block_as_item(self, id, x, y, z, m);
            self.set_block_with_notify(x, y, z, 0);
        } else if id == POWERED_RAIL {
            let mut powered = self.is_block_indirectly_getting_powered(x, y, z)
                || self.is_block_indirectly_getting_powered(x, y + 1, z);
            powered = powered
                || self.powered_rail_search(x, y, z, meta, true, 0)
                || self.powered_rail_search(x, y, z, meta, false, 0);
            let mut changed_state = false;
            if powered && meta & 8 == 0 {
                self.set_block_metadata_with_notify(x, y, z, shape | 8);
                changed_state = true;
            } else if !powered && meta & 8 != 0 {
                self.set_block_metadata_with_notify(x, y, z, shape);
                changed_state = true;
            }
            if changed_state {
                self.notify_blocks_of_neighbor_change(x, y - 1, z, id);
                if (2..=5).contains(&shape) {
                    self.notify_blocks_of_neighbor_change(x, y + 1, z, id);
                }
            }
        } else if changed > 0
            && crate::entity::block_hooks::provides_power(changed)
            && !has_power_bit(id)
            && RailLogic::new(self, x, y, z).adjacent_tracks(self) == 3
        {
            self.rail_update_shape(x, y, z, false);
        }
    }

    /// `func_27044_a`: a powered rail further along the line (up to 8) is powered by a source.
    fn powered_rail_search(&mut self, mut x: i32, mut y: i32, mut z: i32, meta: i32, forward: bool, depth: i32) -> bool {
        if depth >= 8 {
            return false;
        }
        let mut axis = meta & 7;
        let mut below = true;
        match axis {
            0 => {
                if forward {
                    z += 1;
                } else {
                    z -= 1;
                }
            }
            1 => {
                if forward {
                    x -= 1;
                } else {
                    x += 1;
                }
            }
            2 => {
                if forward {
                    x -= 1;
                } else {
                    x += 1;
                    y += 1;
                    below = false;
                }
                axis = 1;
            }
            3 => {
                if forward {
                    x -= 1;
                    y += 1;
                    below = false;
                } else {
                    x += 1;
                }
                axis = 1;
            }
            4 => {
                if forward {
                    z += 1;
                } else {
                    z -= 1;
                    y += 1;
                    below = false;
                }
                axis = 0;
            }
            5 => {
                if forward {
                    z += 1;
                    y += 1;
                    below = false;
                } else {
                    z -= 1;
                }
                axis = 0;
            }
            _ => {}
        }
        self.powered_rail_at(x, y, z, forward, depth, axis)
            || below && self.powered_rail_at(x, y - 1, z, forward, depth, axis)
    }

    /// `func_27043_a`
    fn powered_rail_at(&mut self, x: i32, y: i32, z: i32, forward: bool, depth: i32, axis: i32) -> bool {
        if self.get_block_id(x, y, z) == POWERED_RAIL {
            let meta = self.get_block_metadata(x, y, z);
            let shape = meta & 7;
            if axis == 1 && (shape == 0 || shape == 4 || shape == 5) {
                return false;
            }
            if axis == 0 && (shape == 1 || shape == 2 || shape == 3) {
                return false;
            }
            if meta & 8 != 0 {
                if !self.is_block_indirectly_getting_powered(x, y, z)
                    && !self.is_block_indirectly_getting_powered(x, y + 1, z)
                {
                    return self.powered_rail_search(x, y, z, meta, forward, depth + 1);
                }
                return true;
            }
        }
        false
    }

    pub(crate) fn detector_rail_tick(&mut self, x: i32, y: i32, z: i32) {
        if self.get_block_metadata(x,y,z)&8!=0 { self.update_detector_rail(x,y,z); }
    }
    /// BlockDetectorRail.setStateIfMinecartInteractsWithRail.
    pub(crate) fn update_detector_rail(&mut self,x:i32,y:i32,z:i32) {
        let meta = self.get_block_metadata(x, y, z);
        let b=detector_box11(x,y,z);
        let occupied=self.entities_within_aabb(None,b).into_iter().any(|id|
            matches!(self.entities[&id].kind,crate::entity::EntityKind::Minecart(..)));
        if occupied != (meta&8!=0) {
            self.set_block_metadata_with_notify(x, y, z, if occupied {meta|8} else {meta&7});
            self.notify_blocks_of_neighbor_change(x, y, z, DETECTOR_RAIL);
            self.notify_blocks_of_neighbor_change(x, y - 1, z, DETECTOR_RAIL);
        }
        if occupied { self.schedule_block_update(x,y,z,DETECTOR_RAIL,20); }
    }
}

pub(crate) fn detector_box11(x:i32,y:i32,z:i32)->crate::entity::AxisAlignedBB {
    crate::entity::AxisAlignedBB::new((x as f32+0.125) as f64,y as f64,(z as f32+0.125) as f64,
        (x.wrapping_add(1) as f32-0.125) as f64,(y.wrapping_add(1) as f32-0.125) as f64,(z.wrapping_add(1) as f32-0.125) as f64)
}
