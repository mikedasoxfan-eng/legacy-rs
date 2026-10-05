//! `MetadataChunkBlock`: one queued light recalculation over a box.

use super::chunk::LightKind;
use super::World;
use crate::block::{self, light_opacity};

#[derive(Clone, Copy, Debug)]
pub struct LightUpdate {
    pub kind: LightKind,
    pub x0: i32,
    pub y0: i32,
    pub z0: i32,
    pub x1: i32,
    pub y1: i32,
    pub z1: i32,
}

impl LightUpdate {
    pub fn new(kind: LightKind, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) -> Self {
        LightUpdate { kind, x0, y0, z0, x1, y1, z1 }
    }

    /// `func_866_a`: absorb a new box if it is inside this one, or grows it by at most 2 cells.
    pub fn try_merge(&mut self, x0: i32, y0: i32, z0: i32, x1: i32, y1: i32, z1: i32) -> bool {
        if x0 >= self.x0 && y0 >= self.y0 && z0 >= self.z0 && x1 <= self.x1 && y1 <= self.y1 && z1 <= self.z1 {
            return true;
        }
        let g = 1;
        if x0 >= self.x0 - g
            && y0 >= self.y0 - g
            && z0 >= self.z0 - g
            && x1 <= self.x1 + g
            && y1 <= self.y1 + g
            && z1 <= self.z1 + g
        {
            let old_vol = (self.x1 - self.x0) * (self.y1 - self.y0) * (self.z1 - self.z0);
            let nx0 = x0.min(self.x0);
            let ny0 = y0.min(self.y0);
            let nz0 = z0.min(self.z0);
            let nx1 = x1.max(self.x1);
            let ny1 = y1.max(self.y1);
            let nz1 = z1.max(self.z1);
            let new_vol = (nx1 - nx0) * (ny1 - ny0) * (nz1 - nz0);
            if new_vol - old_vol <= 2 {
                *self = LightUpdate::new(self.kind, nx0, ny0, nz0, nx1, ny1, nz1);
                return true;
            }
        }
        false
    }

    /// `func_4127_a`
    pub fn apply(&mut self, w: &mut World) {
        let sx = self.x1 - self.x0 + 1;
        let sy = self.y1 - self.y0 + 1;
        let sz = self.z1 - self.z0 + 1;
        if sx * sy * sz > 32768 {
            return; // "Light too large, skipping!"
        }
        let kind = self.kind;
        for x in self.x0..=self.x1 {
            for z in self.z0..=self.z1 {
                // The original caches this per chunk, but its cache flag is never set.
                if !w.check_chunks_exist(x - 1, -1, z - 1, x + 1, 1, z + 1) {
                    continue;
                }
                if self.y0 < 0 {
                    self.y0 = 0;
                }
                if self.y1 >= 128 {
                    self.y1 = 127;
                }
                for y in self.y0..=self.y1 {
                    let saved = w.get_saved_light_value(kind, x, y, z);
                    let id = w.get_block_id(x, y, z);
                    let mut opacity = light_opacity(id);
                    if opacity == 0 {
                        opacity = 1;
                    }
                    let emitted = match kind {
                        LightKind::Sky => {
                            if w.can_existing_block_see_the_sky(x, y, z) {
                                15
                            } else {
                                0
                            }
                        }
                        LightKind::Block => block::light_value(id),
                    };
                    let level = if opacity >= 15 && emitted == 0 {
                        0
                    } else {
                        let a = w.get_saved_light_value(kind, x - 1, y, z);
                        let b = w.get_saved_light_value(kind, x + 1, y, z);
                        let c = w.get_saved_light_value(kind, x, y - 1, z);
                        let d = w.get_saved_light_value(kind, x, y + 1, z);
                        let e = w.get_saved_light_value(kind, x, y, z - 1);
                        let f = w.get_saved_light_value(kind, x, y, z + 1);
                        let mut m = a;
                        if b > a {
                            m = b;
                        }
                        if c > m {
                            m = c;
                        }
                        if d > m {
                            m = d;
                        }
                        if e > m {
                            m = e;
                        }
                        if f > m {
                            m = f;
                        }
                        m -= opacity;
                        if m < 0 {
                            m = 0;
                        }
                        if emitted > m {
                            m = emitted;
                        }
                        m
                    };
                    if saved != level {
                        w.set_light_value(kind, x, y, z, level);
                        let mut n = level - 1;
                        if n < 0 {
                            n = 0;
                        }
                        w.neighbor_light_propagation_changed(kind, x - 1, y, z, n);
                        w.neighbor_light_propagation_changed(kind, x, y - 1, z, n);
                        w.neighbor_light_propagation_changed(kind, x, y, z - 1, n);
                        if x + 1 >= self.x1 {
                            w.neighbor_light_propagation_changed(kind, x + 1, y, z, n);
                        }
                        if y + 1 >= self.y1 {
                            w.neighbor_light_propagation_changed(kind, x, y + 1, z, n);
                        }
                        if z + 1 >= self.z1 {
                            w.neighbor_light_propagation_changed(kind, x, y, z + 1, n);
                        }
                    }
                }
            }
        }
    }
}
