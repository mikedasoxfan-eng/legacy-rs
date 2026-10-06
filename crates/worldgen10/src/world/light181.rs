//! b1.8.1 synchronous light propagation, including the original queue ordering.
use super::{chunk::LightKind, World};
use crate::block::{light_opacity, light_value};
const NEIGHBORS: [(i32, i32, i32); 6] = [
    (-1, 0, 0),
    (1, 0, 0),
    (0, -1, 0),
    (0, 1, 0),
    (0, 0, -1),
    (0, 0, 1),
];
const DARK_NEIGHBORS: [(i32, i32, i32); 6] = [
    (0, 0, -1),
    (0, 0, 1),
    (0, -1, 0),
    (0, 1, 0),
    (-1, 0, 0),
    (1, 0, 0),
];
fn encode(x: i32, y: i32, z: i32, level: i32) -> i32 {
    x + 32 + ((y + 32) << 6) + ((z + 32) << 12) + (level << 18)
}
impl World {
    /// World.func_35461_e / Chunk.func_35840_c. Cached precipitation surface.
    pub fn precipitation_height181(&mut self, x: i32, z: i32) -> i32 {
        let c = self.chunk_mut(x >> 4, z >> 4);
        let column = ((x & 15) | ((z & 15) << 4)) as usize;
        if c.precipitation_height[column] == -999 {
            let mut height = -1;
            for y in (1..128).rev() {
                let m = crate::block::mat(crate::block::material_of(c.block(x & 15, y, z & 15)));
                if m.blocks_movement || m.is_liquid {
                    height = y + 1;
                    break;
                }
            }
            c.precipitation_height[column] = height;
        }
        c.precipitation_height[column]
    }
    pub fn fluid_packed_light181(&mut self, x: i32, y: i32, z: i32) -> i32 {
        let a = self.packed_light181(x, y, z, 0);
        let b = self.packed_light181(x, y + 1, z, 0);
        (a & 255).max(b & 255) | ((a >> 16 & 255).max(b >> 16 & 255) << 16)
    }
    pub fn fluid_side_solid181(&mut self, material: u8, x: i32, y: i32, z: i32, side: i32) -> bool {
        let m = self.get_block_material(x, y, z);
        m != material
            && (side == 1 || (m != crate::block::MAT_ICE && crate::block::mat(m).is_solid))
    }
    fn desired_light181(&mut self, kind: LightKind, x: i32, y: i32, z: i32) -> i32 {
        let id = self.get_block_id(x, y, z);
        let opacity = light_opacity(id).max(1);
        if kind == LightKind::Sky && self.can_block_see_the_sky(x, y, z) {
            return 15;
        }
        let mut light = if kind == LightKind::Block {
            light_value(id)
        } else {
            0
        };
        for (dx, dy, dz) in NEIGHBORS {
            light = light.max(self.get_saved_light_value(kind, x + dx, y + dy, z + dz) - opacity);
        }
        light
    }
    /// `World.func_35459_c`: radius-17 removal followed by light restoration.
    pub fn update_light181(&mut self, kind: LightKind, x: i32, y: i32, z: i32) {
        if !self.check_chunks_exist(x - 17, y - 17, z - 17, x + 17, y + 17, z + 17) {
            return;
        }
        let current = self.get_saved_light_value(kind, x, y, z);
        let target = self.desired_light181(kind, x, y, z);
        let mut queue = Vec::with_capacity(32768);
        if target > current {
            queue.push(133152);
        } else if target < current {
            queue.push(133152 + (current << 18));
            let mut read = 0;
            while read < queue.len() {
                let entry = queue[read];
                read += 1;
                let bx = (entry & 63) - 32 + x;
                let by = ((entry >> 6) & 63) - 32 + y;
                let bz = ((entry >> 12) & 63) - 32 + z;
                let level = (entry >> 18) & 15;
                if self.get_saved_light_value(kind, bx, by, bz) != level {
                    continue;
                }
                self.set_light_value(kind, bx, by, bz, 0);
                if level <= 0 || (bx - x).abs() + (by - y).abs() + (bz - z).abs() >= 17 {
                    continue;
                }
                for (dx, dy, dz) in DARK_NEIGHBORS {
                    let next = level - light_opacity(self.get_block_id(bx + dx, by + dy, bz + dz)).max(1);
                    if self.get_saved_light_value(kind, bx + dx, by + dy, bz + dz) == next {
                        assert!(queue.len() < 32768, "vanilla light queue overflow");
                        queue.push(encode(bx + dx - x, by + dy - y, bz + dz - z, next));
                    }
                }
            }
        }
        let mut read = 0;
        while read < queue.len() {
            let entry = queue[read];
            read += 1;
            let bx = (entry & 63) - 32 + x;
            let by = ((entry >> 6) & 63) - 32 + y;
            let bz = ((entry >> 12) & 63) - 32 + z;
            let old = self.get_saved_light_value(kind, bx, by, bz);
            let new = self.desired_light181(kind, bx, by, bz);
            if new != old {
                self.set_light_value(kind, bx, by, bz, new);
                if new > old
                    && (bx - x).abs() + (by - y).abs() + (bz - z).abs() < 17
                    && queue.len() < 32768 - 6
                {
                    for (dx, dy, dz) in NEIGHBORS {
                        if self.get_saved_light_value(kind, bx + dx, by + dy, bz + dz) < new {
                            queue.push(encode(bx + dx - x, by + dy - y, bz + dz - z, 0));
                        }
                    }
                }
            }
        }
    }
    /// `World.func_35463_p`.
    pub(crate) fn update_all_light181(&mut self, x: i32, y: i32, z: i32) {
        if !self.dimension.has_no_sky() {
            self.update_light181(LightKind::Sky, x, y, z);
        }
        self.update_light181(LightKind::Block, x, y, z);
    }
    /// Release 1.0 neighbour light uses Block.useNeighborBrightness.
    pub fn saved_light_with_neighbors181(
        &mut self,
        kind: LightKind,
        x: i32,
        y: i32,
        z: i32,
    ) -> i32 {
        if kind == LightKind::Sky && self.dimension.has_no_sky() { return 0; }
        let y = y.max(0);
        if y >= 128 && kind == LightKind::Sky {
            return 15;
        }
        if y >= 128 || !(-30000000..30000000).contains(&x) || !(-30000000..30000000).contains(&z) {
            return kind.default_value();
        }
        if !self.chunk_exists(x >> 4, z >> 4) {
            return 0;
        }
        if crate::block::use_neighbor_brightness(self.get_block_id(x, y, z)) {
            [(0, 1, 0), (1, 0, 0), (-1, 0, 0), (0, 0, 1), (0, 0, -1)]
                .into_iter()
                .map(|(dx, dy, dz)| self.get_saved_light_value(kind, x + dx, y + dy, z + dz))
                .max()
                .unwrap()
        } else {
            self.get_saved_light_value(kind, x, y, z)
        }
    }
    /// `World.func_35451_b`: the two lightmap coordinates packed in one integer.
    pub fn packed_light181(&mut self, x: i32, y: i32, z: i32, minimum: i32) -> i32 {
        let sky = self.saved_light_with_neighbors181(LightKind::Sky, x, y, z);
        let block = self
            .saved_light_with_neighbors181(LightKind::Block, x, y, z)
            .max(minimum);
        sky << 20 | block << 4
    }
    pub(crate) fn check_neighbor_height181(&mut self, x: i32, z: i32, height: i32) {
        let actual = self.get_height_value(x, z);
        if actual != height {
            self.light_column181(x, z, actual.min(height), actual.max(height) + 1);
        }
    }
    fn light_column181(&mut self, x: i32, z: i32, y0: i32, y1: i32) {
        if y1 > y0 && self.check_chunks_exist(x - 16, 48, z - 16, x + 16, 80, z + 16) {
            for y in y0..y1 {
                self.update_light181(LightKind::Sky, x, y, z);
            }
            if let Some(c) = self.chunks.get_mut(&(x >> 4, z >> 4)) {
                c.modified = true;
            }
        }
    }
    /// `Chunk.func_35841_j` / `func_35839_k`.
    pub fn recheck_skylight181(&mut self, cx: i32, cz: i32) {
        if self.dimension.has_no_sky() || !self.chunk(cx, cz).skylight_pending10 { return; }
        let (x0, z0) = (cx * 16, cz * 16);
        if !self.check_chunks_exist(x0 - 8, 48, z0 - 8, x0 + 24, 80, z0 + 24) {
            return;
        }
        for x in 0..16 {
            for z in 0..16 {
                let column = (x + z * 16) as usize;
                if !self.chunk(cx, cz).skylight_occlusion[column] {
                    continue;
                }
                self.chunk_mut(cx, cz).skylight_occlusion[column] = false;
                let h = self.chunk(cx, cz).height(x, z);
                let bx = x0 + x;
                let bz = z0 + z;
                let lowest = [
                    self.get_height_value(bx - 1, bz),
                    self.get_height_value(bx + 1, bz),
                    self.get_height_value(bx, bz - 1),
                    self.get_height_value(bx, bz + 1),
                ]
                .into_iter()
                .min()
                .unwrap();
                self.check_neighbor_height181(bx, bz, lowest);
                for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    self.check_neighbor_height181(bx + dx, bz + dz, h);
                }
                self.chunk_mut(cx, cz).skylight_pending10 = false;
            }
        }
    }
    pub(crate) fn relight_column181(&mut self, cx: i32, cz: i32, x: i32, y: i32, z: i32) {
        let old = self.chunk(cx, cz).height(x, z);
        let mut h = old.max(y);
        while h > 0 && light_opacity(self.chunk(cx, cz).block(x, h - 1, z)) == 0 {
            h -= 1;
        }
        if h == old {
            return;
        }
        // Vanilla passes chunk-local x/z here, before updating the height map.
        if !self.dimension.has_no_sky() {
            for yy in h.min(old)..=h.max(old) { self.update_light181(LightKind::Sky, x, yy, z); }
        }
        self.mark_blocks_dirty181([x, h.min(old), z, x, h.max(old), z]);
        let no_sky = self.dimension.has_no_sky();
        let c = self.chunk_mut(cx, cz);
        c.height_map[(z << 4 | x) as usize] = h as u8;
        c.lowest_block_height = if h < c.lowest_block_height {
            h
        } else {
            c.height_map
                .iter()
                .map(|&v| v as i32)
                .min()
                .unwrap()
                .min(127)
        };
        if !no_sky {
        if h < old {
            for yy in h..old {
                c.sky_light.set(x, yy, z, 15);
            }
        } else {
            for yy in old..h {
                c.sky_light.set(x, yy, z, 0);
            }
        }
        let mut light = 15;
        let mut yy = h;
        while yy > 0 && light > 0 {
            yy -= 1;
            light = (light - light_opacity(c.block(x, yy, z)).max(1)).max(0);
            c.sky_light.set(x, yy, z, light);
        }
        }
        // The byte height is sign-extended, a vanilla bug at height 128.
        let signed = (h as u8 as i8) as i32;
        let (low, high) = (old.min(signed), old.max(signed));
        let (bx, bz) = (cx * 16 + x, cz * 16 + z);
        if !no_sky {
        for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1), (0, 0)] {
            self.light_column181(bx + dx, bz + dz, low, high);
        }
        }
        self.touch(cx, cz);
    }
}
