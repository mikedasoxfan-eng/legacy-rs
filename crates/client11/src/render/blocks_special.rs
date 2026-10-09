//! The rest of `RenderBlocks`: render types 2 (torch), 3 (fire), 5 (redstone wire),
//! 6 (crops), 7 (door), 8 (ladder), 9 (rails), 10 (stairs), 11 (fence), 12 (lever), 14 (bed),
//! 15 (repeater), 16 (piston base) and 17 (piston head), plus the `Block` virtuals they
//! depend on: `setBlockBoundsBasedOnState`, world-dependent `getBlockTexture`, door textures,
//! `canBlockCatchFire` and `isPowerProviderOrWire`.
//!
//! Block bounds live in the Block object in the original and persist between calls; a
//! renderer that sets them without resetting leaves them for the next block of that kind.
//! `BOUNDS` keeps that state per block id for this thread.

use super::{rgb, Bounds, RenderBlocks};
use crate::render::rprops;
use std::cell::RefCell;
use worldgen::block::{self, mat, material_of};
use worldgen::math;

thread_local! {
    static BOUNDS: RefCell<Vec<Bounds>> = RefCell::new(
        (0..256)
            .map(|id| {
                let p = &rprops(id as u8).bounds;
                Bounds {
                    min_x: p[0] as f64,
                    min_y: p[1] as f64,
                    min_z: p[2] as f64,
                    max_x: p[3] as f64,
                    max_y: p[4] as f64,
                    max_z: p[5] as f64,
                }
            })
            .collect(),
    );
}

/// `ModelBed.field_22280_a` / `field_22279_b`
const BED_FACE: [i32; 4] = [3, 4, 2, 5];
const BED_OPPOSITE: [usize; 4] = [2, 3, 0, 1];
/// `BlockRedstoneRepeater.field_22024_a`
const REPEATER_TORCH: [f64; 4] = [-0.0625, 1.0 / 16.0, 0.1875, 0.3125];

/// `Block.chanceToEncourageFire > 0` (BlockFire.setBurnRate).
fn catches_fire(id: u8) -> bool {
    matches!(id, 5 | 85 | 53 | 17 | 18 | 47 | 46 | 31 | 35)
}

/// `canProvidePower()` (redstone wire's `wiresProvidePower` is true outside its own update).
fn provides_power(id: u8) -> bool {
    matches!(id, 77 | 28 | 69 | 70 | 72 | 75 | 76 | 55)
}

impl<'a, 'b> RenderBlocks<'a, 'b> {
    /// The persistent bounds of `id`, then `setBlockBoundsBasedOnState`.
    pub(super) fn load_bounds(&mut self, id: u8, x: i32, y: i32, z: i32) {
        self.b = BOUNDS.with(|b| b.borrow()[id as usize]);
        let meta = self.w.meta(x, y, z);
        match id {
            119 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0 / 16.0, 1.0),
            block::SNOW => {
                self.b.max_y = ((2 * (1 + (meta & 7))) as f32 / 16.0f32) as f64;
            }
            26 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 9.0 / 16.0, 1.0),
            77 => {
                let (a, bb, c) = (6.0f32 / 16.0, 10.0f32 / 16.0, 3.0f32 / 16.0);
                let d = if meta & 8 > 0 { 1.0f32 / 16.0 } else { 2.0f32 / 16.0 };
                match meta & 7 {
                    1 => self.set_block_bounds(0.0, a, 0.5 - c, d, bb, 0.5 + c),
                    2 => self.set_block_bounds(1.0 - d, a, 0.5 - c, 1.0, bb, 0.5 + c),
                    3 => self.set_block_bounds(0.5 - c, a, 0.0, 0.5 + c, bb, d),
                    4 => self.set_block_bounds(0.5 - c, a, 1.0 - d, 0.5 + c, bb, 1.0),
                    _ => {}
                }
            }
            92 => {
                let e = 1.0f32 / 16.0;
                let start = (1 + meta * 2) as f32 / 16.0f32;
                self.set_block_bounds(start, 0.0, e, 1.0 - e, 0.5, 1.0 - e);
            }
            64 | 71 => self.set_door_rotation(door_state(meta)),
            69 => {
                let v = 3.0f32 / 16.0;
                match meta & 7 {
                    1 => self.set_block_bounds(0.0, 0.2, 0.5 - v, v * 2.0, 0.8, 0.5 + v),
                    2 => self.set_block_bounds(1.0 - v * 2.0, 0.2, 0.5 - v, 1.0, 0.8, 0.5 + v),
                    3 => self.set_block_bounds(0.5 - v, 0.2, 0.0, 0.5 + v, 0.8, v * 2.0),
                    4 => self.set_block_bounds(0.5 - v, 0.2, 1.0 - v * 2.0, 0.5 + v, 0.8, 1.0),
                    _ => self.set_block_bounds(0.25, 0.0, 0.25, 0.75, 0.6, 0.75),
                }
            }
            33 | 29 => {
                if meta & 8 != 0 {
                    match meta & 7 {
                        0 => self.set_block_bounds(0.0, 0.25, 0.0, 1.0, 1.0, 1.0),
                        1 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 12.0 / 16.0, 1.0),
                        2 => self.set_block_bounds(0.0, 0.0, 0.25, 1.0, 1.0, 1.0),
                        3 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 12.0 / 16.0),
                        4 => self.set_block_bounds(0.25, 0.0, 0.0, 1.0, 1.0, 1.0),
                        5 => self.set_block_bounds(0.0, 0.0, 0.0, 12.0 / 16.0, 1.0, 1.0),
                        _ => {}
                    }
                } else {
                    self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
                }
            }
            34 => match meta & 7 {
                0 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 0.25, 1.0),
                1 => self.set_block_bounds(0.0, 12.0 / 16.0, 0.0, 1.0, 1.0, 1.0),
                2 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 0.25),
                3 => self.set_block_bounds(0.0, 0.0, 12.0 / 16.0, 1.0, 1.0, 1.0),
                4 => self.set_block_bounds(0.0, 0.0, 0.0, 0.25, 1.0, 1.0),
                5 => self.set_block_bounds(12.0 / 16.0, 0.0, 0.0, 1.0, 1.0, 1.0),
                _ => {}
            },
            90 => {
                if self.w.block(x - 1, y, z) != 90 && self.w.block(x + 1, y, z) != 90 {
                    let (a, c) = (2.0f32 / 16.0, 0.5f32);
                    self.set_block_bounds(0.5 - a, 0.0, 0.5 - c, 0.5 + a, 1.0, 0.5 + c);
                } else {
                    let (a, c) = (0.5f32, 2.0f32 / 16.0);
                    self.set_block_bounds(0.5 - a, 0.0, 0.5 - c, 0.5 + a, 1.0, 0.5 + c);
                }
            }
            70 | 72 => {
                let e = 1.0f32 / 16.0;
                let h = if meta == 1 { 0.03125 } else { 1.0 / 16.0 };
                self.set_block_bounds(e, 0.0, e, 1.0 - e, h, 1.0 - e);
            }
            66 | 27 | 28 => {
                let h = if (2..=5).contains(&meta) { 10.0 / 16.0 } else { 2.0 / 16.0 };
                self.set_block_bounds(0.0, 0.0, 0.0, 1.0, h, 1.0);
            }
            53 | 67 | 108 | 109 | 114 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0),
            96 => {
                let v = 3.0f32 / 16.0;
                self.set_block_bounds(0.0, 0.0, 0.0, 1.0, v, 1.0);
                if meta & 4 != 0 {
                    match meta & 3 {
                        0 => self.set_block_bounds(0.0, 0.0, 1.0 - v, 1.0, 1.0, 1.0),
                        1 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, v),
                        2 => self.set_block_bounds(1.0 - v, 0.0, 0.0, 1.0, 1.0, 1.0),
                        _ => self.set_block_bounds(0.0, 0.0, 0.0, v, 1.0, 1.0),
                    }
                }
            }
            _ => {}
        }
    }

    /// Keep this block's bounds for the next one (the Block object outlives the call).
    pub(super) fn store_bounds(&self, id: u8) {
        BOUNDS.with(|b| b.borrow_mut()[id as usize] = self.b);
    }

    /// `Block.setBlockBounds(float...)`
    fn set_block_bounds(&mut self, x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32) {
        self.b = Bounds {
            min_x: x0 as f64,
            min_y: y0 as f64,
            min_z: z0 as f64,
            max_x: x1 as f64,
            max_y: y1 as f64,
            max_z: z1 as f64,
        };
    }

    /// `BlockDoor.setDoorRotation`
    fn set_door_rotation(&mut self, state: i32) {
        let v = 3.0f32 / 16.0;
        self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 2.0, 1.0);
        match state {
            0 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, v),
            1 => self.set_block_bounds(1.0 - v, 0.0, 0.0, 1.0, 1.0, 1.0),
            2 => self.set_block_bounds(0.0, 0.0, 1.0 - v, 1.0, 1.0, 1.0),
            3 => self.set_block_bounds(0.0, 0.0, 0.0, v, 1.0, 1.0),
            _ => {}
        }
    }

    /// World-dependent `getBlockTexture` for blocks other than grass and chests.
    pub(super) fn special_texture(&self, id: u8, x: i32, y: i32, z: i32, side: usize) -> Option<i32> {
        let base = rprops(id).tex_index as i32;
        let side = side as i32;
        Some(match id {
            // BlockFurnace / BlockDispenser: the front faces the metadata side.
            61 | 62 | 23 => {
                if side == 1 || side == 0 {
                    base + 17
                } else if side != self.w.meta(x, y, z) {
                    base
                } else if id == 23 {
                    base + 1
                } else if id == 62 {
                    base + 16
                } else {
                    base - 1
                }
            }
            // BlockStairs: the model block's texture.
            53 => self.texture_meta(5, side as usize, self.w.meta(x, y, z)),
            67 => self.texture_meta(4, side as usize, self.w.meta(x, y, z)),
            95 => {
                if side == 1 || side == 0 {
                    base - 1
                } else {
                    let o = |dx: i32, dz: i32| {
                        let id = self.w.block(x + dx, y, z + dz);
                        rprops(id).exists && rprops(id).opaque
                    };
                    let (n, s, w, e) = (o(0, -1), o(0, 1), o(-1, 0), o(1, 0));
                    let mut front = 3;
                    if n && !s {
                        front = 3;
                    }
                    if s && !n {
                        front = 2;
                    }
                    if w && !e {
                        front = 5;
                    }
                    if e && !w {
                        front = 4;
                    }
                    if side == front {
                        base + 1
                    } else {
                        base
                    }
                }
            }
            _ => return None,
        })
    }

    /// `ChunkCache.isBlockNormalCube`: solid material and `renderAsNormalBlock`.
    fn is_block_normal_cube(&self, x: i32, y: i32, z: i32) -> bool {
        let id = self.w.block(x, y, z);
        rprops(id).exists && mat(material_of(id)).is_solid && rprops(id).normal
    }

    /// `BlockRedstoneWire.isPowerProviderOrWire`
    fn is_power_provider_or_wire(&self, x: i32, y: i32, z: i32, dir: i32) -> bool {
        let id = self.w.block(x, y, z);
        if id == 55 {
            true
        } else if id == 0 {
            false
        } else if provides_power(id) && dir != -1 {
            true
        } else if id != 93 && id != 94 {
            false
        } else {
            let orientation = self.w.meta(x, y, z) & 3;
            dir == orientation || dir == BED_OPPOSITE[orientation as usize] as i32
        }
    }

    #[inline]
    fn color(&mut self, r: f32, g: f32, b: f32) {
        self.t.set_color_opaque_f(r, g, b);
    }

    #[inline]
    fn v(&mut self, x: f64, y: f64, z: f64, u: f64, v: f64) {
        self.t.vertex_uv(x, y, z, u, v);
    }

    // ------------------------------------------------------------------ type 14: bed

    pub(super) fn render_block_bed(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let meta = self.w.meta(x, y, z);
        let dir = meta & 3;
        // BlockBed.isBlockFootOfBed is (meta & 8) != 0, despite the name.
        let foot = meta & 8 != 0;
        let (c_bottom, c_top, c_ns, c_ew) = (0.5f32, 1.0f32, 0.8f32, 0.6f32);
        let own = self.mixed(id, x, y, z);
        self.t.set_brightness(own);
        self.color(c_bottom, c_bottom, c_bottom);
        let tex = self.texture(id, x, y, z, 0);
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let u0 = (tu as f32 / 256.0f32) as f64;
        let u1 = ((tu + 16) as f64 - 0.01) / 256.0;
        let v0 = (tv as f32 / 256.0f32) as f64;
        let v1 = ((tv + 16) as f64 - 0.01) / 256.0;
        let b = self.b;
        let (x0, x1, yb, z0, z1) =
            (x as f64 + b.min_x, x as f64 + b.max_x, y as f64 + b.min_y + 0.1875, z as f64 + b.min_z, z as f64 + b.max_z);
        self.v(x0, yb, z1, u0, v1);
        self.v(x0, yb, z0, u0, v0);
        self.v(x1, yb, z0, u1, v0);
        self.v(x1, yb, z1, u1, v1);

        let above = self.mixed(id, x, y + 1, z);
        self.t.set_brightness(above);
        self.color(c_top, c_top, c_top);
        let tex = self.texture(id, x, y, z, 1);
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let a30 = (tu as f32 / 256.0f32) as f64;
        let a32 = ((tu + 16) as f64 - 0.01) / 256.0;
        let a34 = (tv as f32 / 256.0f32) as f64;
        let a36 = ((tv + 16) as f64 - 0.01) / 256.0;
        let (mut a38, mut a40, mut a42, mut a44) = (a30, a32, a34, a34);
        let (mut a46, mut a48, mut a50, mut a52) = (a30, a32, a36, a36);
        if dir == 0 {
            a40 = a30;
            a42 = a36;
            a46 = a32;
            a52 = a34;
        } else if dir == 2 {
            a38 = a32;
            a44 = a36;
            a48 = a30;
            a50 = a34;
        } else if dir == 3 {
            a38 = a32;
            a44 = a36;
            a48 = a30;
            a50 = a34;
            a40 = a30;
            a42 = a36;
            a46 = a32;
            a52 = a34;
        }
        let yt = y as f64 + b.max_y;
        self.v(x1, yt, z1, a46, a50);
        self.v(x1, yt, z0, a38, a42);
        self.v(x0, yt, z0, a40, a44);
        self.v(x0, yt, z1, a48, a52);

        let mut skip = BED_FACE[dir as usize];
        if foot {
            skip = BED_FACE[BED_OPPOSITE[dir as usize]];
        }
        let flip = match dir {
            0 => 5,
            1 => 3,
            3 => 2,
            _ => 4,
        };
        if skip != 2 && (self.render_all_faces || self.should_side_be_rendered(id, x, y, z - 1, 2)) {
            // b1.8.1 tests minZ/maxZ for the x faces too (vanilla bug, kept).
            let l = if self.b.min_z > 0.0 { own } else { self.mixed(id, x, y, z - 1) };
            self.t.set_brightness(l);
            self.color(c_ns, c_ns, c_ns);
            self.flip_texture = flip == 2;
            let t = self.texture(id, x, y, z, 2);
            self.render_east_face(x as f64, y as f64, z as f64, t);
        }
        if skip != 3 && (self.render_all_faces || self.should_side_be_rendered(id, x, y, z + 1, 3)) {
            // b1.8.1 tests minZ/maxZ for the x faces too (vanilla bug, kept).
            let l = if self.b.max_z < 1.0 { own } else { self.mixed(id, x, y, z + 1) };
            self.t.set_brightness(l);
            self.color(c_ns, c_ns, c_ns);
            self.flip_texture = flip == 3;
            let t = self.texture(id, x, y, z, 3);
            self.render_west_face(x as f64, y as f64, z as f64, t);
        }
        if skip != 4 && (self.render_all_faces || self.should_side_be_rendered(id, x - 1, y, z, 4)) {
            // b1.8.1 tests minZ/maxZ for the x faces too (vanilla bug, kept).
            let l = if self.b.min_z > 0.0 { own } else { self.mixed(id, x - 1, y, z) };
            self.t.set_brightness(l);
            self.color(c_ew, c_ew, c_ew);
            self.flip_texture = flip == 4;
            let t = self.texture(id, x, y, z, 4);
            self.render_north_face(x as f64, y as f64, z as f64, t);
        }
        if skip != 5 && (self.render_all_faces || self.should_side_be_rendered(id, x + 1, y, z, 5)) {
            // b1.8.1 tests minZ/maxZ for the x faces too (vanilla bug, kept).
            let l = if self.b.max_z < 1.0 { own } else { self.mixed(id, x + 1, y, z) };
            self.t.set_brightness(l);
            self.color(c_ew, c_ew, c_ew);
            self.flip_texture = flip == 5;
            let t = self.texture(id, x, y, z, 5);
            self.render_south_face(x as f64, y as f64, z as f64, t);
        }
        self.flip_texture = false;
        true
    }

    // ------------------------------------------------------------------ type 2: torch

    pub(super) fn render_block_torch(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let meta = self.w.meta(x, y, z);
        let l = self.mixed(id, x, y, z);
        self.t.set_brightness(l);
        self.color(1.0, 1.0, 1.0);
        let lean = 0.4f32 as f64;
        let off = 0.5 - lean;
        let up = 0.2f32 as f64;
        let (xf, yf, zf) = (x as f64, y as f64, z as f64);
        match meta {
            1 => self.render_torch_at_angle(id, xf - off, yf + up, zf, -lean, 0.0),
            2 => self.render_torch_at_angle(id, xf + off, yf + up, zf, lean, 0.0),
            3 => self.render_torch_at_angle(id, xf, yf + up, zf - off, 0.0, -lean),
            4 => self.render_torch_at_angle(id, xf, yf + up, zf + off, 0.0, lean),
            _ => self.render_torch_at_angle(id, xf, yf, zf, 0.0, 0.0),
        }
        true
    }

    /// `renderTorchAtAngle`
    fn render_torch_at_angle(&mut self, id: u8, mut x: f64, y: f64, mut z: f64, dx: f64, dz: f64) {
        let tex = self.face_tex(self.texture_meta(id, 0, 0));
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let u0 = tu as f32 / 256.0f32;
        let u1 = (tu as f32 + 15.99f32) / 256.0f32;
        let v0 = tv as f32 / 256.0f32;
        let v1 = (tv as f32 + 15.99f32) / 256.0f32;
        let cu0 = u0 as f64 + 1.75 / 64.0;
        let cv0 = v0 as f64 + 6.0 / 256.0;
        let cu1 = u0 as f64 + 9.0 / 256.0;
        let cv1 = v0 as f64 + 1.0 / 32.0;
        x += 0.5;
        z += 0.5;
        let x0 = x - 0.5;
        let x1 = x + 0.5;
        let z0 = z - 0.5;
        let z1 = z + 0.5;
        let w = 1.0 / 16.0;
        let h = 0.625;
        let (u0, u1, v0, v1) = (u0 as f64, u1 as f64, v0 as f64, v1 as f64);
        self.v(x + dx * (1.0 - h) - w, y + h, z + dz * (1.0 - h) - w, cu0, cv0);
        self.v(x + dx * (1.0 - h) - w, y + h, z + dz * (1.0 - h) + w, cu0, cv1);
        self.v(x + dx * (1.0 - h) + w, y + h, z + dz * (1.0 - h) + w, cu1, cv1);
        self.v(x + dx * (1.0 - h) + w, y + h, z + dz * (1.0 - h) - w, cu1, cv0);
        self.v(x - w, y + 1.0, z0, u0, v0);
        self.v(x - w + dx, y + 0.0, z0 + dz, u0, v1);
        self.v(x - w + dx, y + 0.0, z1 + dz, u1, v1);
        self.v(x - w, y + 1.0, z1, u1, v0);
        self.v(x + w, y + 1.0, z1, u0, v0);
        self.v(x + dx + w, y + 0.0, z1 + dz, u0, v1);
        self.v(x + dx + w, y + 0.0, z0 + dz, u1, v1);
        self.v(x + w, y + 1.0, z0, u1, v0);
        self.v(x0, y + 1.0, z + w, u0, v0);
        self.v(x0 + dx, y + 0.0, z + w + dz, u0, v1);
        self.v(x1 + dx, y + 0.0, z + w + dz, u1, v1);
        self.v(x1, y + 1.0, z + w, u1, v0);
        self.v(x1, y + 1.0, z - w, u0, v0);
        self.v(x1 + dx, y + 0.0, z - w + dz, u0, v1);
        self.v(x0 + dx, y + 0.0, z - w + dz, u1, v1);
        self.v(x0, y + 1.0, z - w, u1, v0);
    }

    // ------------------------------------------------------------------ type 15: repeater

    pub(super) fn render_block_repeater(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let meta = self.w.meta(x, y, z);
        let dir = meta & 3;
        let delay = ((meta & 12) >> 2) as usize;
        self.render_standard_block_pub(id, x, y, z);
        self.t.set_brightness(self.mixed(id, x, y, z));
        self.color(1.0, 1.0, 1.0);
        let down = -0.1875;
        let (mut ax, mut az, mut bx, mut bz) = (0.0, 0.0, 0.0, 0.0);
        match dir {
            0 => {
                bz = -0.3125;
                az = REPEATER_TORCH[delay];
            }
            1 => {
                bx = 0.3125;
                ax = -REPEATER_TORCH[delay];
            }
            2 => {
                bz = 0.3125;
                az = -REPEATER_TORCH[delay];
            }
            _ => {
                bx = -0.3125;
                ax = REPEATER_TORCH[delay];
            }
        }
        let (xf, yf, zf) = (x as f64, y as f64, z as f64);
        self.render_torch_at_angle(id, xf + ax, yf + down, zf + az, 0.0, 0.0);
        self.render_torch_at_angle(id, xf + bx, yf + down, zf + bz, 0.0, 0.0);
        let tex = self.texture_meta(id, 1, 0);
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let u0 = (tu as f32 / 256.0f32) as f64;
        let u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        let v0 = (tv as f32 / 256.0f32) as f64;
        let v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
        let h = 2.0f32 / 16.0;
        let (mut c32, mut c33, mut c34, mut c35) = ((x + 1) as f32, (x + 1) as f32, x as f32, x as f32);
        let (mut c36, mut c37, mut c38, mut c39) = (z as f32, (z + 1) as f32, (z + 1) as f32, z as f32);
        let c40 = y as f32 + h;
        if dir == 2 {
            c33 = x as f32;
            c32 = c33;
            c35 = (x + 1) as f32;
            c34 = c35;
            c39 = (z + 1) as f32;
            c36 = c39;
            c38 = z as f32;
            c37 = c38;
        } else if dir == 3 {
            c35 = x as f32;
            c32 = c35;
            c34 = (x + 1) as f32;
            c33 = c34;
            c37 = z as f32;
            c36 = c37;
            c39 = (z + 1) as f32;
            c38 = c39;
        } else if dir == 1 {
            c35 = (x + 1) as f32;
            c32 = c35;
            c34 = x as f32;
            c33 = c34;
            c37 = (z + 1) as f32;
            c36 = c37;
            c39 = z as f32;
            c38 = c39;
        }
        self.v(c35 as f64, c40 as f64, c39 as f64, u0, v0);
        self.v(c34 as f64, c40 as f64, c38 as f64, u0, v1);
        self.v(c33 as f64, c40 as f64, c37 as f64, u1, v1);
        self.v(c32 as f64, c40 as f64, c36 as f64, u1, v0);
        true
    }

    // ------------------------------------------------------------------ types 16, 17: pistons

    fn set_piston_rotation(&mut self, facing: i32) {
        match facing {
            0 => {
                self.uv_rotate_east = 3;
                self.uv_rotate_west = 3;
                self.uv_rotate_south = 3;
                self.uv_rotate_north = 3;
            }
            2 => {
                self.uv_rotate_south = 1;
                self.uv_rotate_north = 2;
            }
            3 => {
                self.uv_rotate_south = 2;
                self.uv_rotate_north = 1;
                self.uv_rotate_top = 3;
                self.uv_rotate_bottom = 3;
            }
            4 => {
                self.uv_rotate_east = 1;
                self.uv_rotate_west = 2;
                self.uv_rotate_top = 2;
                self.uv_rotate_bottom = 1;
            }
            5 => {
                self.uv_rotate_east = 2;
                self.uv_rotate_west = 1;
                self.uv_rotate_top = 1;
                self.uv_rotate_bottom = 2;
            }
            _ => {}
        }
    }

    fn reset_rotation(&mut self) {
        self.uv_rotate_east = 0;
        self.uv_rotate_west = 0;
        self.uv_rotate_south = 0;
        self.uv_rotate_north = 0;
        self.uv_rotate_top = 0;
        self.uv_rotate_bottom = 0;
    }

    /// `func_31074_b(block, x, y, z, false)`: the piston base.
    pub(crate) fn render_piston_base(&mut self, id: u8, x: i32, y: i32, z: i32, extended_override: bool) -> bool {
        let meta = self.w.meta(x, y, z);
        let extended = extended_override || meta & 8 != 0;
        let facing = meta & 7;
        self.set_piston_rotation(facing);
        if extended {
            match facing {
                0 => self.set_block_bounds(0.0, 0.25, 0.0, 1.0, 1.0, 1.0),
                1 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 12.0 / 16.0, 1.0),
                2 => self.set_block_bounds(0.0, 0.0, 0.25, 1.0, 1.0, 1.0),
                3 => self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 12.0 / 16.0),
                4 => self.set_block_bounds(0.25, 0.0, 0.0, 1.0, 1.0, 1.0),
                5 => self.set_block_bounds(0.0, 0.0, 0.0, 12.0 / 16.0, 1.0, 1.0),
                _ => {}
            }
            self.render_standard_block_pub(id, x, y, z);
            self.reset_rotation();
            self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        } else {
            self.render_standard_block_pub(id, x, y, z);
            self.reset_rotation();
        }
        true
    }

    /// `func_31076_a`: piston arm quad along y.
    #[allow(clippy::too_many_arguments)]
    fn piston_arm_y(&mut self, x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64, l: f32, len: f64) {
        let (u0, v0, u1, v1) = self.arm_uv(len);
        self.color(l, l, l);
        self.v(x0, y1, z0, u1, v0);
        self.v(x0, y0, z0, u0, v0);
        self.v(x1, y0, z1, u0, v1);
        self.v(x1, y1, z1, u1, v1);
    }

    /// `func_31081_b`: arm quad along z.
    #[allow(clippy::too_many_arguments)]
    fn piston_arm_z(&mut self, x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64, l: f32, len: f64) {
        let (u0, v0, u1, v1) = self.arm_uv(len);
        self.color(l, l, l);
        self.v(x0, y0, z1, u1, v0);
        self.v(x0, y0, z0, u0, v0);
        self.v(x1, y1, z0, u0, v1);
        self.v(x1, y1, z1, u1, v1);
    }

    /// `func_31077_c`: arm quad along x.
    #[allow(clippy::too_many_arguments)]
    fn piston_arm_x(&mut self, x0: f64, x1: f64, y0: f64, y1: f64, z0: f64, z1: f64, l: f32, len: f64) {
        let (u0, v0, u1, v1) = self.arm_uv(len);
        self.color(l, l, l);
        self.v(x1, y0, z0, u1, v0);
        self.v(x0, y0, z0, u0, v0);
        self.v(x0, y1, z1, u0, v1);
        self.v(x1, y1, z1, u1, v1);
    }

    fn arm_uv(&self, len: f64) -> (f64, f64, f64, f64) {
        let tex = self.face_tex(108);
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        (
            (tu as f32 / 256.0f32) as f64,
            (tv as f32 / 256.0f32) as f64,
            (tu as f64 + len - 0.01) / 256.0,
            ((tv as f32 + 4.0f32) as f64 - 0.01) / 256.0,
        )
    }

    /// `func_31080_c(block, x, y, z, full)`: the piston head and its arm.
    pub(crate) fn render_piston_head(&mut self, id: u8, x: i32, y: i32, z: i32, full: bool) -> bool {
        let meta = self.w.meta(x, y, z);
        let facing = meta & 7;
        let l = self.brightness(id, x, y, z);
        let arm = if full { 1.0f32 } else { 0.5f32 };
        let len = if full { 16.0 } else { 8.0 };
        let (xf, yf, zf) = (x as f32, y as f32, z as f32);
        let a = 6.0f32 / 16.0;
        let c = 10.0f32 / 16.0;
        let d = |v: f32| v as f64;
        match facing {
            0 => {
                self.set_piston_rotation(0);
                self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 0.25, 1.0);
                self.render_standard_block_pub(id, x, y, z);
                let (y0, y1) = (d(yf + 0.25), d(yf + 0.25 + arm));
                self.piston_arm_y(d(xf + a), d(xf + c), y0, y1, d(zf + c), d(zf + c), l * 0.8, len);
                self.piston_arm_y(d(xf + c), d(xf + a), y0, y1, d(zf + a), d(zf + a), l * 0.8, len);
                self.piston_arm_y(d(xf + a), d(xf + a), y0, y1, d(zf + a), d(zf + c), l * 0.6, len);
                self.piston_arm_y(d(xf + c), d(xf + c), y0, y1, d(zf + c), d(zf + a), l * 0.6, len);
            }
            1 => {
                self.set_block_bounds(0.0, 12.0 / 16.0, 0.0, 1.0, 1.0, 1.0);
                self.render_standard_block_pub(id, x, y, z);
                let (y0, y1) = (d(yf - 0.25 + 1.0 - arm), d(yf - 0.25 + 1.0));
                self.piston_arm_y(d(xf + a), d(xf + c), y0, y1, d(zf + c), d(zf + c), l * 0.8, len);
                self.piston_arm_y(d(xf + c), d(xf + a), y0, y1, d(zf + a), d(zf + a), l * 0.8, len);
                self.piston_arm_y(d(xf + a), d(xf + a), y0, y1, d(zf + a), d(zf + c), l * 0.6, len);
                self.piston_arm_y(d(xf + c), d(xf + c), y0, y1, d(zf + c), d(zf + a), l * 0.6, len);
            }
            2 => {
                self.set_piston_rotation(2);
                self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 0.25);
                self.render_standard_block_pub(id, x, y, z);
                let (z0, z1) = (d(zf + 0.25), d(zf + 0.25 + arm));
                self.piston_arm_z(d(xf + a), d(xf + a), d(yf + c), d(yf + a), z0, z1, l * 0.6, len);
                self.piston_arm_z(d(xf + c), d(xf + c), d(yf + a), d(yf + c), z0, z1, l * 0.6, len);
                self.piston_arm_z(d(xf + a), d(xf + c), d(yf + a), d(yf + a), z0, z1, l * 0.5, len);
                self.piston_arm_z(d(xf + c), d(xf + a), d(yf + c), d(yf + c), z0, z1, l, len);
            }
            3 => {
                self.set_piston_rotation(3);
                self.set_block_bounds(0.0, 0.0, 12.0 / 16.0, 1.0, 1.0, 1.0);
                self.render_standard_block_pub(id, x, y, z);
                let (z0, z1) = (d(zf - 0.25 + 1.0 - arm), d(zf - 0.25 + 1.0));
                self.piston_arm_z(d(xf + a), d(xf + a), d(yf + c), d(yf + a), z0, z1, l * 0.6, len);
                self.piston_arm_z(d(xf + c), d(xf + c), d(yf + a), d(yf + c), z0, z1, l * 0.6, len);
                self.piston_arm_z(d(xf + a), d(xf + c), d(yf + a), d(yf + a), z0, z1, l * 0.5, len);
                self.piston_arm_z(d(xf + c), d(xf + a), d(yf + c), d(yf + c), z0, z1, l, len);
            }
            4 => {
                self.set_piston_rotation(4);
                self.set_block_bounds(0.0, 0.0, 0.0, 0.25, 1.0, 1.0);
                self.render_standard_block_pub(id, x, y, z);
                let (x0, x1) = (d(xf + 0.25), d(xf + 0.25 + arm));
                self.piston_arm_x(x0, x1, d(yf + a), d(yf + a), d(zf + c), d(zf + a), l * 0.5, len);
                self.piston_arm_x(x0, x1, d(yf + c), d(yf + c), d(zf + a), d(zf + c), l, len);
                self.piston_arm_x(x0, x1, d(yf + a), d(yf + c), d(zf + a), d(zf + a), l * 0.6, len);
                self.piston_arm_x(x0, x1, d(yf + c), d(yf + a), d(zf + c), d(zf + c), l * 0.6, len);
            }
            5 => {
                self.set_piston_rotation(5);
                self.set_block_bounds(12.0 / 16.0, 0.0, 0.0, 1.0, 1.0, 1.0);
                self.render_standard_block_pub(id, x, y, z);
                let (x0, x1) = (d(xf - 0.25 + 1.0 - arm), d(xf - 0.25 + 1.0));
                self.piston_arm_x(x0, x1, d(yf + a), d(yf + a), d(zf + c), d(zf + a), l * 0.5, len);
                self.piston_arm_x(x0, x1, d(yf + c), d(yf + c), d(zf + a), d(zf + c), l, len);
                self.piston_arm_x(x0, x1, d(yf + a), d(yf + c), d(zf + a), d(zf + a), l * 0.6, len);
                self.piston_arm_x(x0, x1, d(yf + c), d(yf + a), d(zf + c), d(zf + c), l * 0.6, len);
            }
            _ => {}
        }
        self.reset_rotation();
        self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        true
    }

    // ------------------------------------------------------------------ type 12: lever

    pub(super) fn render_block_lever(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let meta = self.w.meta(x, y, z);
        let facing = meta & 7;
        let on = meta & 8 > 0;
        let had_override = self.override_texture >= 0;
        if !had_override {
            // Block.cobblestone.blockIndexInTexture
            self.override_texture = rprops(4).tex_index as i32;
        }
        let (r, w, h) = (0.25f32, 3.0f32 / 16.0, 3.0f32 / 16.0);
        match facing {
            5 => self.set_block_bounds(0.5 - w, 0.0, 0.5 - r, 0.5 + w, h, 0.5 + r),
            6 => self.set_block_bounds(0.5 - r, 0.0, 0.5 - w, 0.5 + r, h, 0.5 + w),
            4 => self.set_block_bounds(0.5 - w, 0.5 - r, 1.0 - h, 0.5 + w, 0.5 + r, 1.0),
            3 => self.set_block_bounds(0.5 - w, 0.5 - r, 0.0, 0.5 + w, 0.5 + r, h),
            2 => self.set_block_bounds(1.0 - h, 0.5 - r, 0.5 - w, 1.0, 0.5 + r, 0.5 + w),
            1 => self.set_block_bounds(0.0, 0.5 - r, 0.5 - w, h, 0.5 + r, 0.5 + w),
            _ => {}
        }
        self.render_standard_block_pub(id, x, y, z);
        if !had_override {
            self.override_texture = -1;
        }
        let own = self.mixed(id, x, y, z);
        self.t.set_brightness(own);
        let l = 1.0f32;
        self.color(l, l, l);
        let tex = self.face_tex(self.texture_meta(id, 0, 0));
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let mut u0 = tu as f32 / 256.0f32;
        let mut u1 = (tu as f32 + 15.99f32) / 256.0f32;
        let mut v0 = tv as f32 / 256.0f32;
        let mut v1 = (tv as f32 + 15.99f32) / 256.0f32;
        let (hw, hd, hh) = (1.0f32 / 16.0, 1.0f32 / 16.0, 10.0f32 / 16.0);
        let mut p: [[f64; 3]; 8] = [
            [-hw as f64, 0.0, -hd as f64],
            [hw as f64, 0.0, -hd as f64],
            [hw as f64, 0.0, hd as f64],
            [-hw as f64, 0.0, hd as f64],
            [-hw as f64, hh as f64, -hd as f64],
            [hw as f64, hh as f64, -hd as f64],
            [hw as f64, hh as f64, hd as f64],
            [-hw as f64, hh as f64, hd as f64],
        ];
        let rot_x = |v: &mut [f64; 3], a: f32| {
            let (c, s) = (math::cos(a), math::sin(a));
            let y = v[1] * c as f64 + v[2] * s as f64;
            let z = v[2] * c as f64 - v[1] * s as f64;
            v[1] = y;
            v[2] = z;
        };
        let rot_y = |v: &mut [f64; 3], a: f32| {
            let (c, s) = (math::cos(a), math::sin(a));
            let x = v[0] * c as f64 + v[2] * s as f64;
            let z = v[2] * c as f64 - v[0] * s as f64;
            v[0] = x;
            v[2] = z;
        };
        let pi = math::PI_F32;
        for v in p.iter_mut() {
            if on {
                v[2] -= 1.0 / 16.0;
                rot_x(v, pi * 2.0 / 9.0);
            } else {
                v[2] += 1.0 / 16.0;
                rot_x(v, -(pi * 2.0 / 9.0));
            }
            if facing == 6 {
                rot_y(v, pi * 0.5);
            }
            if facing < 5 {
                v[1] -= 0.375;
                rot_x(v, pi * 0.5);
                if facing == 4 {
                    rot_y(v, 0.0);
                }
                if facing == 3 {
                    rot_y(v, pi);
                }
                if facing == 2 {
                    rot_y(v, pi * 0.5);
                }
                if facing == 1 {
                    rot_y(v, pi * -0.5);
                }
                v[0] += x as f64 + 0.5;
                v[1] += (y as f32 + 0.5f32) as f64;
                v[2] += z as f64 + 0.5;
            } else {
                v[0] += x as f64 + 0.5;
                v[1] += (y as f32 + 2.0f32 / 16.0) as f64;
                v[2] += z as f64 + 0.5;
            }
        }
        for face in 0..6 {
            if face == 0 {
                u0 = (tu + 7) as f32 / 256.0f32;
                u1 = ((tu + 9) as f32 - 0.01f32) / 256.0f32;
                v0 = (tv + 6) as f32 / 256.0f32;
                v1 = ((tv + 8) as f32 - 0.01f32) / 256.0f32;
            } else if face == 2 {
                u0 = (tu + 7) as f32 / 256.0f32;
                u1 = ((tu + 9) as f32 - 0.01f32) / 256.0f32;
                v0 = (tv + 6) as f32 / 256.0f32;
                v1 = ((tv + 16) as f32 - 0.01f32) / 256.0f32;
            }
            let [a, b, c, d] = match face {
                0 => [0, 1, 2, 3],
                1 => [7, 6, 5, 4],
                2 => [1, 0, 4, 5],
                3 => [2, 1, 5, 6],
                4 => [3, 2, 6, 7],
                _ => [0, 3, 7, 4],
            };
            let (u0, u1, v0, v1) = (u0 as f64, u1 as f64, v0 as f64, v1 as f64);
            self.v(p[a][0], p[a][1], p[a][2], u0, v1);
            self.v(p[b][0], p[b][1], p[b][2], u1, v1);
            self.v(p[c][0], p[c][1], p[c][2], u1, v0);
            self.v(p[d][0], p[d][1], p[d][2], u0, v0);
        }
        true
    }

    // ------------------------------------------------------------------ type 3: fire

    pub(super) fn render_block_fire(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let tex = self.face_tex(self.texture_meta(id, 0, 0));
        self.color(1.0, 1.0, 1.0);
        let own = self.mixed(id, x, y, z);
        self.t.set_brightness(own);
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let mut u0 = (tu as f32 / 256.0f32) as f64;
        let mut u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        let mut v0 = (tv as f32 / 256.0f32) as f64;
        let mut v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
        let mut hgt = 1.4f32;
        let (xf, zf) = (x as f32, z as f32);
        let catches = |s: &Self, x: i32, y: i32, z: i32| catches_fire(s.w.block(x, y, z));
        if !self.is_block_normal_cube(x, y - 1, z) && !catches(self, x, y - 1, z) {
            let side = 0.2f32;
            let lift = 1.0f32 / 16.0;
            if (x + y + z) & 1 == 1 {
                u0 = (tu as f32 / 256.0f32) as f64;
                u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
                v0 = ((tv + 16) as f32 / 256.0f32) as f64;
                v1 = ((tv as f32 + 15.99f32 + 16.0f32) / 256.0f32) as f64;
            }
            if (x / 2 + y / 2 + z / 2) & 1 == 1 {
                std::mem::swap(&mut u0, &mut u1);
            }
            let top = (y as f32 + hgt + lift) as f64;
            let bot = ((y + 0) as f32 + lift) as f64;
            let (x0, x1, z0, z1) = (x as f64, (x + 1) as f64, z as f64, (z + 1) as f64);
            if catches(self, x - 1, y, z) {
                let xs = (xf + side) as f64;
                self.v(xs, top, z1, u1, v0);
                self.v(x0, bot, z1, u1, v1);
                self.v(x0, bot, z0, u0, v1);
                self.v(xs, top, z0, u0, v0);
                self.v(xs, top, z0, u0, v0);
                self.v(x0, bot, z0, u0, v1);
                self.v(x0, bot, z1, u1, v1);
                self.v(xs, top, z1, u1, v0);
            }
            if catches(self, x + 1, y, z) {
                let xs = ((x + 1) as f32 - side) as f64;
                self.v(xs, top, z0, u0, v0);
                self.v(x1, bot, z0, u0, v1);
                self.v(x1, bot, z1, u1, v1);
                self.v(xs, top, z1, u1, v0);
                self.v(xs, top, z1, u1, v0);
                self.v(x1, bot, z1, u1, v1);
                self.v(x1, bot, z0, u0, v1);
                self.v(xs, top, z0, u0, v0);
            }
            if catches(self, x, y, z - 1) {
                let zs = (zf + side) as f64;
                self.v(x0, top, zs, u1, v0);
                self.v(x0, bot, z0, u1, v1);
                self.v(x1, bot, z0, u0, v1);
                self.v(x1, top, zs, u0, v0);
                self.v(x1, top, zs, u0, v0);
                self.v(x1, bot, z0, u0, v1);
                self.v(x0, bot, z0, u1, v1);
                self.v(x0, top, zs, u1, v0);
            }
            if catches(self, x, y, z + 1) {
                let zs = ((z + 1) as f32 - side) as f64;
                self.v(x1, top, zs, u0, v0);
                self.v(x1, bot, z1, u0, v1);
                self.v(x0, bot, z1, u1, v1);
                self.v(x0, top, zs, u1, v0);
                self.v(x0, top, zs, u1, v0);
                self.v(x0, bot, z1, u1, v1);
                self.v(x1, bot, z1, u0, v1);
                self.v(x1, top, zs, u0, v0);
            }
            if catches(self, x, y + 1, z) {
                let a21 = x as f64 + 0.5 + 0.5;
                let a23 = x as f64 + 0.5 - 0.5;
                let a25 = z as f64 + 0.5 + 0.5;
                let a27 = z as f64 + 0.5 - 0.5;
                let a29 = x as f64 + 0.5 - 0.5;
                let a31 = x as f64 + 0.5 + 0.5;
                let a33 = z as f64 + 0.5 - 0.5;
                let a35 = z as f64 + 0.5 + 0.5;
                u0 = (tu as f32 / 256.0f32) as f64;
                u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
                v0 = (tv as f32 / 256.0f32) as f64;
                v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
                let y1 = y + 1;
                hgt = -0.2;
                let yt = (y1 as f32 + hgt) as f64;
                let yb = y1 as f64;
                if (x + y1 + z) & 1 == 0 {
                    self.v(a29, yt, z as f64, u1, v0);
                    self.v(a21, yb, z as f64, u1, v1);
                    self.v(a21, yb, (z + 1) as f64, u0, v1);
                    self.v(a29, yt, (z + 1) as f64, u0, v0);
                    u0 = (tu as f32 / 256.0f32) as f64;
                    u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
                    v0 = ((tv + 16) as f32 / 256.0f32) as f64;
                    v1 = ((tv as f32 + 15.99f32 + 16.0f32) / 256.0f32) as f64;
                    self.v(a31, yt, (z + 1) as f64, u1, v0);
                    self.v(a23, yb, (z + 1) as f64, u1, v1);
                    self.v(a23, yb, z as f64, u0, v1);
                    self.v(a31, yt, z as f64, u0, v0);
                } else {
                    self.v(x as f64, yt, a35, u1, v0);
                    self.v(x as f64, yb, a27, u1, v1);
                    self.v((x + 1) as f64, yb, a27, u0, v1);
                    self.v((x + 1) as f64, yt, a35, u0, v0);
                    u0 = (tu as f32 / 256.0f32) as f64;
                    u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
                    v0 = ((tv + 16) as f32 / 256.0f32) as f64;
                    v1 = ((tv as f32 + 15.99f32 + 16.0f32) / 256.0f32) as f64;
                    self.v((x + 1) as f64, yt, a33, u1, v0);
                    self.v((x + 1) as f64, yb, a25, u1, v1);
                    self.v(x as f64, yb, a25, u0, v1);
                    self.v(x as f64, yt, a33, u0, v0);
                }
            }
        } else {
            let top = (y as f32 + hgt) as f64;
            let bot = y as f64;
            let a19 = x as f64 + 0.5 + 0.2;
            let a21 = x as f64 + 0.5 - 0.2;
            let a23 = z as f64 + 0.5 + 0.2;
            let a25 = z as f64 + 0.5 - 0.2;
            let a27 = x as f64 + 0.5 - 0.3;
            let a29 = x as f64 + 0.5 + 0.3;
            let a31 = z as f64 + 0.5 - 0.3;
            let a33 = z as f64 + 0.5 + 0.3;
            let (x0, x1, z0, z1) = (x as f64, (x + 1) as f64, z as f64, (z + 1) as f64);
            self.v(a27, top, z1, u1, v0);
            self.v(a19, bot, z1, u1, v1);
            self.v(a19, bot, z0, u0, v1);
            self.v(a27, top, z0, u0, v0);
            self.v(a29, top, z0, u1, v0);
            self.v(a21, bot, z0, u1, v1);
            self.v(a21, bot, z1, u0, v1);
            self.v(a29, top, z1, u0, v0);
            u0 = (tu as f32 / 256.0f32) as f64;
            u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
            v0 = ((tv + 16) as f32 / 256.0f32) as f64;
            v1 = ((tv as f32 + 15.99f32 + 16.0f32) / 256.0f32) as f64;
            self.v(x1, top, a33, u1, v0);
            self.v(x1, bot, a25, u1, v1);
            self.v(x0, bot, a25, u0, v1);
            self.v(x0, top, a33, u0, v0);
            self.v(x0, top, a31, u1, v0);
            self.v(x0, bot, a23, u1, v1);
            self.v(x1, bot, a23, u0, v1);
            self.v(x1, top, a31, u0, v0);
            let b19 = x as f64 + 0.5 - 0.5;
            let b21 = x as f64 + 0.5 + 0.5;
            let b23 = z as f64 + 0.5 - 0.5;
            let b25 = z as f64 + 0.5 + 0.5;
            let b27 = x as f64 + 0.5 - 0.4;
            let b29 = x as f64 + 0.5 + 0.4;
            let b31 = z as f64 + 0.5 - 0.4;
            let b33 = z as f64 + 0.5 + 0.4;
            self.v(b27, top, z0, u0, v0);
            self.v(b19, bot, z0, u0, v1);
            self.v(b19, bot, z1, u1, v1);
            self.v(b27, top, z1, u1, v0);
            self.v(b29, top, z1, u0, v0);
            self.v(b21, bot, z1, u0, v1);
            self.v(b21, bot, z0, u1, v1);
            self.v(b29, top, z0, u1, v0);
            u0 = (tu as f32 / 256.0f32) as f64;
            u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
            v0 = (tv as f32 / 256.0f32) as f64;
            v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
            self.v(x0, top, b33, u0, v0);
            self.v(x0, bot, b25, u0, v1);
            self.v(x1, bot, b25, u1, v1);
            self.v(x1, top, b33, u1, v0);
            self.v(x1, top, b31, u0, v0);
            self.v(x1, bot, b23, u0, v1);
            self.v(x0, bot, b23, u1, v1);
            self.v(x0, top, b31, u1, v0);
        }
        true
    }

    // ------------------------------------------------------------------ type 5: redstone wire

    pub(super) fn render_block_redstone_wire(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let meta = self.w.meta(x, y, z);
        let tex = self.face_tex(self.texture_meta(id, 1, meta));
        let own = self.mixed(id, x, y, z);
        self.t.set_brightness(own);
        let l = 1.0f32;
        let p = meta as f32 / 15.0f32;
        let mut r = p * 0.6f32 + 0.4f32;
        if meta == 0 {
            r = 0.3;
        }
        let mut g = p * p * 0.7f32 - 0.5f32;
        let mut b = p * p * 0.6f32 - 0.7f32;
        if g < 0.0 {
            g = 0.0;
        }
        if b < 0.0 {
            b = 0.0;
        }
        self.color(l * r, l * g, l * b);
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let mut u0 = (tu as f32 / 256.0f32) as f64;
        let mut u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        let mut v0 = (tv as f32 / 256.0f32) as f64;
        let mut v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
        let mut west = self.is_power_provider_or_wire(x - 1, y, z, 1)
            || !self.is_block_normal_cube(x - 1, y, z) && self.is_power_provider_or_wire(x - 1, y - 1, z, -1);
        let mut east = self.is_power_provider_or_wire(x + 1, y, z, 3)
            || !self.is_block_normal_cube(x + 1, y, z) && self.is_power_provider_or_wire(x + 1, y - 1, z, -1);
        let mut north = self.is_power_provider_or_wire(x, y, z - 1, 2)
            || !self.is_block_normal_cube(x, y, z - 1) && self.is_power_provider_or_wire(x, y - 1, z - 1, -1);
        let mut south = self.is_power_provider_or_wire(x, y, z + 1, 0)
            || !self.is_block_normal_cube(x, y, z + 1) && self.is_power_provider_or_wire(x, y - 1, z + 1, -1);
        if !self.is_block_normal_cube(x, y + 1, z) {
            if self.is_block_normal_cube(x - 1, y, z) && self.is_power_provider_or_wire(x - 1, y + 1, z, -1) {
                west = true;
            }
            if self.is_block_normal_cube(x + 1, y, z) && self.is_power_provider_or_wire(x + 1, y + 1, z, -1) {
                east = true;
            }
            if self.is_block_normal_cube(x, y, z - 1) && self.is_power_provider_or_wire(x, y + 1, z - 1, -1) {
                north = true;
            }
            if self.is_block_normal_cube(x, y, z + 1) && self.is_power_provider_or_wire(x, y + 1, z + 1, -1) {
                south = true;
            }
        }
        let mut x0 = x as f32;
        let mut x1 = (x + 1) as f32;
        let mut z0 = z as f32;
        let mut z1 = (z + 1) as f32;
        let mut shape = 0;
        if (west || east) && !north && !south {
            shape = 1;
        }
        if (north || south) && !east && !west {
            shape = 2;
        }
        if shape != 0 {
            u0 = ((tu + 16) as f32 / 256.0f32) as f64;
            u1 = (((tu + 16) as f32 + 15.99f32) / 256.0f32) as f64;
            v0 = (tv as f32 / 256.0f32) as f64;
            v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
        }
        let yy = (y as f32 + 0.015625f32) as f64;
        let s = 1.0 / 16.0;
        if shape == 0 {
            if !west {
                x0 += 5.0 / 16.0;
            }
            if !west {
                u0 += 1.25 / 64.0;
            }
            if !east {
                x1 -= 5.0 / 16.0;
            }
            if !east {
                u1 -= 1.25 / 64.0;
            }
            if !north {
                z0 += 5.0 / 16.0;
            }
            if !north {
                v0 += 1.25 / 64.0;
            }
            if !south {
                z1 -= 5.0 / 16.0;
            }
            if !south {
                v1 -= 1.25 / 64.0;
            }
            let (x0, x1, z0, z1) = (x0 as f64, x1 as f64, z0 as f64, z1 as f64);
            self.v(x1, yy, z1, u1, v1);
            self.v(x1, yy, z0, u1, v0);
            self.v(x0, yy, z0, u0, v0);
            self.v(x0, yy, z1, u0, v1);
            self.color(l, l, l);
            self.v(x1, yy, z1, u1, v1 + s);
            self.v(x1, yy, z0, u1, v0 + s);
            self.v(x0, yy, z0, u0, v0 + s);
            self.v(x0, yy, z1, u0, v1 + s);
        } else if shape == 1 {
            let (x0, x1, z0, z1) = (x0 as f64, x1 as f64, z0 as f64, z1 as f64);
            self.v(x1, yy, z1, u1, v1);
            self.v(x1, yy, z0, u1, v0);
            self.v(x0, yy, z0, u0, v0);
            self.v(x0, yy, z1, u0, v1);
            self.color(l, l, l);
            self.v(x1, yy, z1, u1, v1 + s);
            self.v(x1, yy, z0, u1, v0 + s);
            self.v(x0, yy, z0, u0, v0 + s);
            self.v(x0, yy, z1, u0, v1 + s);
        } else {
            let (x0, x1, z0, z1) = (x0 as f64, x1 as f64, z0 as f64, z1 as f64);
            self.v(x1, yy, z1, u1, v1);
            self.v(x1, yy, z0, u0, v1);
            self.v(x0, yy, z0, u0, v0);
            self.v(x0, yy, z1, u1, v0);
            self.color(l, l, l);
            self.v(x1, yy, z1, u1, v1 + s);
            self.v(x1, yy, z0, u0, v1 + s);
            self.v(x0, yy, z0, u0, v0 + s);
            self.v(x0, yy, z1, u1, v0 + s);
        }
        if !self.is_block_normal_cube(x, y + 1, z) {
            u0 = ((tu + 16) as f32 / 256.0f32) as f64;
            u1 = (((tu + 16) as f32 + 15.99f32) / 256.0f32) as f64;
            v0 = (tv as f32 / 256.0f32) as f64;
            v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
            let top = ((y + 1) as f32 + 7.0f32 / 320.0f32) as f64;
            let bot = y as f64;
            let (xa, xb, za, zb) = (x as f64, (x + 1) as f64, z as f64, (z + 1) as f64);
            if self.is_block_normal_cube(x - 1, y, z) && self.w.block(x - 1, y + 1, z) == 55 {
                let xs = (x as f32 + 0.015625f32) as f64;
                self.color(l * r, l * g, l * b);
                self.v(xs, top, zb, u1, v0);
                self.v(xs, bot, zb, u0, v0);
                self.v(xs, bot, za, u0, v1);
                self.v(xs, top, za, u1, v1);
                self.color(l, l, l);
                self.v(xs, top, zb, u1, v0 + s);
                self.v(xs, bot, zb, u0, v0 + s);
                self.v(xs, bot, za, u0, v1 + s);
                self.v(xs, top, za, u1, v1 + s);
            }
            if self.is_block_normal_cube(x + 1, y, z) && self.w.block(x + 1, y + 1, z) == 55 {
                let xs = ((x + 1) as f32 - 0.015625f32) as f64;
                self.color(l * r, l * g, l * b);
                self.v(xs, bot, zb, u0, v1);
                self.v(xs, top, zb, u1, v1);
                self.v(xs, top, za, u1, v0);
                self.v(xs, bot, za, u0, v0);
                self.color(l, l, l);
                self.v(xs, bot, zb, u0, v1 + s);
                self.v(xs, top, zb, u1, v1 + s);
                self.v(xs, top, za, u1, v0 + s);
                self.v(xs, bot, za, u0, v0 + s);
            }
            if self.is_block_normal_cube(x, y, z - 1) && self.w.block(x, y + 1, z - 1) == 55 {
                let zs = (z as f32 + 0.015625f32) as f64;
                self.color(l * r, l * g, l * b);
                self.v(xb, bot, zs, u0, v1);
                self.v(xb, top, zs, u1, v1);
                self.v(xa, top, zs, u1, v0);
                self.v(xa, bot, zs, u0, v0);
                self.color(l, l, l);
                self.v(xb, bot, zs, u0, v1 + s);
                self.v(xb, top, zs, u1, v1 + s);
                self.v(xa, top, zs, u1, v0 + s);
                self.v(xa, bot, zs, u0, v0 + s);
            }
            if self.is_block_normal_cube(x, y, z + 1) && self.w.block(x, y + 1, z + 1) == 55 {
                let zs = ((z + 1) as f32 - 0.015625f32) as f64;
                self.color(l * r, l * g, l * b);
                self.v(xb, top, zs, u1, v0);
                self.v(xb, bot, zs, u0, v0);
                self.v(xa, bot, zs, u0, v1);
                self.v(xa, top, zs, u1, v1);
                self.color(l, l, l);
                self.v(xb, top, zs, u1, v0 + s);
                self.v(xb, bot, zs, u0, v0 + s);
                self.v(xa, bot, zs, u0, v1 + s);
                self.v(xa, top, zs, u1, v1 + s);
            }
        }
        true
    }

    // ------------------------------------------------------------------ type 9: rails

    pub(super) fn render_block_rail(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let mut meta = self.w.meta(x, y, z);
        let tex = self.face_tex(self.texture_meta(id, 0, meta));
        // BlockRail.getIsPowered: golden and detector rails.
        if id == 27 || id == 28 {
            meta &= 7;
        }
        let own = self.mixed(id, x, y, z);
        self.t.set_brightness(own);
        self.color(1.0, 1.0, 1.0);
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let u0 = (tu as f32 / 256.0f32) as f64;
        let u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        let v0 = (tv as f32 / 256.0f32) as f64;
        let v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
        let lift = 1.0f32 / 16.0;
        let (mut c20, mut c21, mut c22, mut c23) = ((x + 1) as f32, (x + 1) as f32, x as f32, x as f32);
        let (mut c24, mut c25, mut c26, mut c27) = (z as f32, (z + 1) as f32, (z + 1) as f32, z as f32);
        let (mut c28, mut c29, mut c30, mut c31) =
            (y as f32 + lift, y as f32 + lift, y as f32 + lift, y as f32 + lift);
        if meta != 1 && meta != 2 && meta != 3 && meta != 7 {
            if meta == 8 {
                c21 = x as f32;
                c20 = c21;
                c23 = (x + 1) as f32;
                c22 = c23;
                c27 = (z + 1) as f32;
                c24 = c27;
                c26 = z as f32;
                c25 = c26;
            } else if meta == 9 {
                c23 = x as f32;
                c20 = c23;
                c22 = (x + 1) as f32;
                c21 = c22;
                c25 = z as f32;
                c24 = c25;
                c27 = (z + 1) as f32;
                c26 = c27;
            }
        } else {
            c23 = (x + 1) as f32;
            c20 = c23;
            c22 = x as f32;
            c21 = c22;
            c25 = (z + 1) as f32;
            c24 = c25;
            c27 = z as f32;
            c26 = c27;
        }
        if meta != 2 && meta != 4 {
            if meta == 3 || meta == 5 {
                c29 += 1.0;
                c30 += 1.0;
            }
        } else {
            c28 += 1.0;
            c31 += 1.0;
        }
        let d = |v: f32| v as f64;
        self.v(d(c20), d(c28), d(c24), u1, v0);
        self.v(d(c21), d(c29), d(c25), u1, v1);
        self.v(d(c22), d(c30), d(c26), u0, v1);
        self.v(d(c23), d(c31), d(c27), u0, v0);
        self.v(d(c23), d(c31), d(c27), u0, v0);
        self.v(d(c22), d(c30), d(c26), u0, v1);
        self.v(d(c21), d(c29), d(c25), u1, v1);
        self.v(d(c20), d(c28), d(c24), u1, v0);
        true
    }

    // ------------------------------------------------------------------ type 8: ladder

    pub(super) fn render_block_ladder(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let tex = self.face_tex(self.texture_meta(id, 0, 0));
        let own = self.mixed(id, x, y, z);
        self.t.set_brightness(own);
        self.color(1.0, 1.0, 1.0);
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let u0 = (tu as f32 / 256.0f32) as f64;
        let u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        let v0 = (tv as f32 / 256.0f32) as f64;
        let v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
        let meta = self.w.meta(x, y, z);
        // b1.8.1 does the ladder offsets in double (0.05F widened).
        let (e, o) = (0.0f64, 0.05f32 as f64);
        let d = |v: f64| v;
        let (xf, yf, zf) = (x as f64, y as f64, z as f64);
        let (x1f, y1f, z1f) = ((x + 1) as f64, (y + 1) as f64, (z + 1) as f64);
        if meta == 5 {
            self.v(d(xf + o), d(y1f + e), d(z1f + e), u0, v0);
            self.v(d(xf + o), d(yf - e), d(z1f + e), u0, v1);
            self.v(d(xf + o), d(yf - e), d(zf - e), u1, v1);
            self.v(d(xf + o), d(y1f + e), d(zf - e), u1, v0);
        }
        if meta == 4 {
            self.v(d(x1f - o), d(yf - e), d(z1f + e), u1, v1);
            self.v(d(x1f - o), d(y1f + e), d(z1f + e), u1, v0);
            self.v(d(x1f - o), d(y1f + e), d(zf - e), u0, v0);
            self.v(d(x1f - o), d(yf - e), d(zf - e), u0, v1);
        }
        if meta == 3 {
            self.v(d(x1f + e), d(yf - e), d(zf + o), u1, v1);
            self.v(d(x1f + e), d(y1f + e), d(zf + o), u1, v0);
            self.v(d(xf - e), d(y1f + e), d(zf + o), u0, v0);
            self.v(d(xf - e), d(yf - e), d(zf + o), u0, v1);
        }
        if meta == 2 {
            self.v(d(x1f + e), d(y1f + e), d(z1f - o), u0, v0);
            self.v(d(x1f + e), d(yf - e), d(z1f - o), u0, v1);
            self.v(d(xf - e), d(yf - e), d(z1f - o), u1, v1);
            self.v(d(xf - e), d(y1f + e), d(z1f - o), u1, v0);
        }
        true
    }

    // ------------------------------------------------------------------ type 6: crops

    pub(super) fn render_block_crops(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let own = self.mixed(id, x, y, z);
        self.t.set_brightness(own);
        self.color(1.0, 1.0, 1.0);
        let meta = self.w.meta(x, y, z);
        self.render_crops_squares(id, meta, x as f64, (y as f32 - 1.0f32 / 16.0) as f64, z as f64);
        true
    }

    /// `func_1245_b`
    fn render_crops_squares(&mut self, id: u8, meta: i32, x: f64, y: f64, z: f64) {
        let tex = self.face_tex(self.texture_meta(id, 0, meta));
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let u0 = (tu as f32 / 256.0f32) as f64;
        let u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        let v0 = (tv as f32 / 256.0f32) as f64;
        let v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
        let mut a = x + 0.5 - 0.25;
        let mut b = x + 0.5 + 0.25;
        let mut c = z + 0.5 - 0.5;
        let mut d = z + 0.5 + 0.5;
        self.v(a, y + 1.0, c, u0, v0);
        self.v(a, y + 0.0, c, u0, v1);
        self.v(a, y + 0.0, d, u1, v1);
        self.v(a, y + 1.0, d, u1, v0);
        self.v(a, y + 1.0, d, u0, v0);
        self.v(a, y + 0.0, d, u0, v1);
        self.v(a, y + 0.0, c, u1, v1);
        self.v(a, y + 1.0, c, u1, v0);
        self.v(b, y + 1.0, d, u0, v0);
        self.v(b, y + 0.0, d, u0, v1);
        self.v(b, y + 0.0, c, u1, v1);
        self.v(b, y + 1.0, c, u1, v0);
        self.v(b, y + 1.0, c, u0, v0);
        self.v(b, y + 0.0, c, u0, v1);
        self.v(b, y + 0.0, d, u1, v1);
        self.v(b, y + 1.0, d, u1, v0);
        a = x + 0.5 - 0.5;
        b = x + 0.5 + 0.5;
        c = z + 0.5 - 0.25;
        d = z + 0.5 + 0.25;
        self.v(a, y + 1.0, c, u0, v0);
        self.v(a, y + 0.0, c, u0, v1);
        self.v(b, y + 0.0, c, u1, v1);
        self.v(b, y + 1.0, c, u1, v0);
        self.v(b, y + 1.0, c, u0, v0);
        self.v(b, y + 0.0, c, u0, v1);
        self.v(a, y + 0.0, c, u1, v1);
        self.v(a, y + 1.0, c, u1, v0);
        self.v(b, y + 1.0, d, u0, v0);
        self.v(b, y + 0.0, d, u0, v1);
        self.v(a, y + 0.0, d, u1, v1);
        self.v(a, y + 1.0, d, u1, v0);
        self.v(a, y + 1.0, d, u0, v0);
        self.v(a, y + 0.0, d, u0, v1);
        self.v(b, y + 0.0, d, u1, v1);
        self.v(b, y + 1.0, d, u1, v0);
    }

    // ------------------------------------------------------------------ type 11: fence

    pub(super) fn render_block_fence(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let (p0, p1) = (6.0f32 / 16.0, 10.0f32 / 16.0);
        self.set_block_bounds(p0, 0.0, p0, p1, 1.0, p1);
        self.render_standard_block_pub(id, x, y, z);
        let mut along_x = false;
        let mut along_z = false;
        // BlockFence.isFenceAt: also join opaque normal blocks except pumpkins.
        let joins = |b: u8| b == id || b == 107 ||
            (b != 86 && b != 91 && rprops(b).exists && rprops(b).normal && (mat(material_of(b)).translucent_solid || matches!(b, 110 | 112 | 121)));
        if joins(self.w.block(x - 1, y, z)) || joins(self.w.block(x + 1, y, z)) {
            along_x = true;
        }
        if joins(self.w.block(x, y, z - 1)) || joins(self.w.block(x, y, z + 1)) {
            along_z = true;
        }
        let w = joins(self.w.block(x - 1, y, z));
        let e = joins(self.w.block(x + 1, y, z));
        let n = joins(self.w.block(x, y, z - 1));
        let s = joins(self.w.block(x, y, z + 1));
        if !along_x && !along_z {
            along_x = true;
        }
        let (a, b) = (7.0f32 / 16.0, 9.0f32 / 16.0);
        let mut y0 = 12.0f32 / 16.0;
        let mut y1 = 15.0f32 / 16.0;
        let x0 = if w { 0.0 } else { a };
        let x1 = if e { 1.0 } else { b };
        let z0 = if n { 0.0 } else { a };
        let z1 = if s { 1.0 } else { b };
        for _ in 0..2 {
            if along_x {
                self.set_block_bounds(x0, y0, a, x1, y1, b);
                self.render_standard_block_pub(id, x, y, z);
            }
            if along_z {
                self.set_block_bounds(a, y0, z0, b, y1, z1);
                self.render_standard_block_pub(id, x, y, z);
            }
            y0 = 6.0 / 16.0;
            y1 = 9.0 / 16.0;
        }
        self.set_block_bounds(if w { 0.0 } else { p0 }, 0.0, if n { 0.0 } else { p0 }, if e { 1.0 } else { p1 }, 1.0, if s { 1.0 } else { p1 });
        true
    }

    // ------------------------------------------------------------------ b1.8.1 render types

    /// `BlockPane.func_35298_d`: panes join panes, glass and opaque cubes.
    fn pane_connects(&self, id: u8, other: u8) -> bool {
        block::opaque_cube_lookup(other) || other == id || other == 20
    }

    /// Render type 18, `func_35922_a`: glass panes and iron bars.
    #[allow(unused_parens, clippy::all)]
    pub(super) fn render_block_pane(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let var5 = 128;
        let own = self.mixed(id, x, y, z);
        self.t.set_brightness(own);
        let var7 = 1.0f32;
        let (r, g, b) = self.tinted_rgb(self.color_multiplier(id, x, y, z));
        self.color(var7 * r, var7 * g, var7 * b);
        let (var65, var67) = if self.override_texture >= 0 {
            (self.override_texture, self.override_texture)
        } else {
            let m = self.w.meta(x, y, z);
            // BlockPane.func_35299_s: the pane edge texture (iron bars 85, glass pane 148).
            (self.texture_meta(id, 0, m), if id == 101 { 85 } else { 148 })
        };
        let var68 = (var65 & 15) << 4;
        let var15 = var65 & 240;
        let var16 = (var68 as f32 / 256.0f32) as f64;
        let var18 = ((var68 as f32 + 7.99f32) / 256.0f32) as f64;
        let var20 = ((var68 as f32 + 15.99f32) / 256.0f32) as f64;
        let var22 = (var15 as f32 / 256.0f32) as f64;
        let var24 = ((var15 as f32 + 15.99f32) / 256.0f32) as f64;
        let var26 = (var67 & 15) << 4;
        let var27 = var67 & 240;
        let var28 = ((var26 + 7) as f32 / 256.0f32) as f64;
        let var30 = ((var26 as f32 + 8.99f32) / 256.0f32) as f64;
        let var32 = (var27 as f32 / 256.0f32) as f64;
        let var34 = ((var27 + 8) as f32 / 256.0f32) as f64;
        let var36 = ((var27 as f32 + 15.99f32) / 256.0f32) as f64;
        let var38 = x as f64;
        let var40 = x as f64 + 0.5;
        let var42 = (x + 1) as f64;
        let var44 = z as f64;
        let var46 = z as f64 + 0.5;
        let var48 = (z + 1) as f64;
        let var50 = x as f64 + 0.5 - 1.0 / 16.0;
        let var52 = x as f64 + 0.5 + 1.0 / 16.0;
        let var54 = z as f64 + 0.5 - 1.0 / 16.0;
        let var56 = z as f64 + 0.5 + 1.0 / 16.0;
        let _ = (var44, var48, var50, var52);
        let var58 = self.pane_connects(id, self.w.block(x, y, z - 1));
        let var59 = self.pane_connects(id, self.w.block(x, y, z + 1));
        let var60 = self.pane_connects(id, self.w.block(x - 1, y, z));
        let var61 = self.pane_connects(id, self.w.block(x + 1, y, z));
        // BlockPane.shouldSideBeRendered: never against the same pane.
        let var62 = self.w.block(x, y + 1, z) != id && self.should_side_be_rendered(id, x, y + 1, z, 1);
        let var63 = self.w.block(x, y - 1, z) != id && self.should_side_be_rendered(id, x, y - 1, z, 0);
        if (!var60 || !var61) && (var60 || var61 || var58 || var59) {
            if var60 && !var61 {
                self.t.vertex_uv(var38, ((y + 1) as f64), var46, var16, var22);
                self.t.vertex_uv(var38, ((y + 0) as f64), var46, var16, var24);
                self.t.vertex_uv(var40, ((y + 0) as f64), var46, var18, var24);
                self.t.vertex_uv(var40, ((y + 1) as f64), var46, var18, var22);
                self.t.vertex_uv(var40, ((y + 1) as f64), var46, var16, var22);
                self.t.vertex_uv(var40, ((y + 0) as f64), var46, var16, var24);
                self.t.vertex_uv(var38, ((y + 0) as f64), var46, var18, var24);
                self.t.vertex_uv(var38, ((y + 1) as f64), var46, var18, var22);
                if !var59 && !var58 {
                    self.t.vertex_uv(var40, ((y + 1) as f64), var56, var28, var32);
                    self.t.vertex_uv(var40, ((y + 0) as f64), var56, var28, var36);
                    self.t.vertex_uv(var40, ((y + 0) as f64), var54, var30, var36);
                    self.t.vertex_uv(var40, ((y + 1) as f64), var54, var30, var32);
                    self.t.vertex_uv(var40, ((y + 1) as f64), var54, var28, var32);
                    self.t.vertex_uv(var40, ((y + 0) as f64), var54, var28, var36);
                    self.t.vertex_uv(var40, ((y + 0) as f64), var56, var30, var36);
                    self.t.vertex_uv(var40, ((y + 1) as f64), var56, var30, var32);
                }
                if var62 || y < var5 - 1 && self.w.is_air(x - 1, y + 1, z) {
                    self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var56, var30, var34);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var56, var30, var36);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var54, var28, var36);
                    self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var54, var28, var34);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var56, var30, var34);
                    self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var56, var30, var36);
                    self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var54, var28, var36);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var54, var28, var34);
                }
                if var63 || y > 1 && self.w.is_air(x - 1, y - 1, z) {
                    self.t.vertex_uv(var38, (y as f64) - 0.01, var56, var30, var34);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var56, var30, var36);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var54, var28, var36);
                    self.t.vertex_uv(var38, (y as f64) - 0.01, var54, var28, var34);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var56, var30, var34);
                    self.t.vertex_uv(var38, (y as f64) - 0.01, var56, var30, var36);
                    self.t.vertex_uv(var38, (y as f64) - 0.01, var54, var28, var36);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var54, var28, var34);
                }
            } else if !var60 && var61 {
                self.t.vertex_uv(var40, ((y + 1) as f64), var46, var18, var22);
                self.t.vertex_uv(var40, ((y + 0) as f64), var46, var18, var24);
                self.t.vertex_uv(var42, ((y + 0) as f64), var46, var20, var24);
                self.t.vertex_uv(var42, ((y + 1) as f64), var46, var20, var22);
                self.t.vertex_uv(var42, ((y + 1) as f64), var46, var18, var22);
                self.t.vertex_uv(var42, ((y + 0) as f64), var46, var18, var24);
                self.t.vertex_uv(var40, ((y + 0) as f64), var46, var20, var24);
                self.t.vertex_uv(var40, ((y + 1) as f64), var46, var20, var22);
                if !var59 && !var58 {
                    self.t.vertex_uv(var40, ((y + 1) as f64), var54, var28, var32);
                    self.t.vertex_uv(var40, ((y + 0) as f64), var54, var28, var36);
                    self.t.vertex_uv(var40, ((y + 0) as f64), var56, var30, var36);
                    self.t.vertex_uv(var40, ((y + 1) as f64), var56, var30, var32);
                    self.t.vertex_uv(var40, ((y + 1) as f64), var56, var28, var32);
                    self.t.vertex_uv(var40, ((y + 0) as f64), var56, var28, var36);
                    self.t.vertex_uv(var40, ((y + 0) as f64), var54, var30, var36);
                    self.t.vertex_uv(var40, ((y + 1) as f64), var54, var30, var32);
                }
                if var62 || y < var5 - 1 && self.w.is_air(x + 1, y + 1, z) {
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var56, var30, var32);
                    self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var56, var30, var34);
                    self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var54, var28, var34);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var54, var28, var32);
                    self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var56, var30, var32);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var56, var30, var34);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var54, var28, var34);
                    self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var54, var28, var32);
                }
                if var63 || y > 1 && self.w.is_air(x + 1, y - 1, z) {
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var56, var30, var32);
                    self.t.vertex_uv(var42, (y as f64) - 0.01, var56, var30, var34);
                    self.t.vertex_uv(var42, (y as f64) - 0.01, var54, var28, var34);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var54, var28, var32);
                    self.t.vertex_uv(var42, (y as f64) - 0.01, var56, var30, var32);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var56, var30, var34);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var54, var28, var34);
                    self.t.vertex_uv(var42, (y as f64) - 0.01, var54, var28, var32);
                }
            }
        } else {
            self.t.vertex_uv(var38, ((y + 1) as f64), var46, var16, var22);
            self.t.vertex_uv(var38, ((y + 0) as f64), var46, var16, var24);
            self.t.vertex_uv(var42, ((y + 0) as f64), var46, var20, var24);
            self.t.vertex_uv(var42, ((y + 1) as f64), var46, var20, var22);
            self.t.vertex_uv(var42, ((y + 1) as f64), var46, var16, var22);
            self.t.vertex_uv(var42, ((y + 0) as f64), var46, var16, var24);
            self.t.vertex_uv(var38, ((y + 0) as f64), var46, var20, var24);
            self.t.vertex_uv(var38, ((y + 1) as f64), var46, var20, var22);
            if var62 {
                self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var56, var30, var36);
                self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var56, var30, var32);
                self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var54, var28, var32);
                self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var54, var28, var36);
                self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var56, var30, var36);
                self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var56, var30, var32);
                self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var54, var28, var32);
                self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var54, var28, var36);
            } else {
                if y < var5 - 1 && self.w.is_air(x - 1, y + 1, z) {
                    self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var56, var30, var34);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var56, var30, var36);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var54, var28, var36);
                    self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var54, var28, var34);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var56, var30, var34);
                    self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var56, var30, var36);
                    self.t.vertex_uv(var38, ((y + 1) as f64) + 0.01, var54, var28, var36);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var54, var28, var34);
                }
                if y < var5 - 1 && self.w.is_air(x + 1, y + 1, z) {
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var56, var30, var32);
                    self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var56, var30, var34);
                    self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var54, var28, var34);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var54, var28, var32);
                    self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var56, var30, var32);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var56, var30, var34);
                    self.t.vertex_uv(var40, ((y + 1) as f64) + 0.01, var54, var28, var34);
                    self.t.vertex_uv(var42, ((y + 1) as f64) + 0.01, var54, var28, var32);
                }
            }
            if var63 {
                self.t.vertex_uv(var38, (y as f64) - 0.01, var56, var30, var36);
                self.t.vertex_uv(var42, (y as f64) - 0.01, var56, var30, var32);
                self.t.vertex_uv(var42, (y as f64) - 0.01, var54, var28, var32);
                self.t.vertex_uv(var38, (y as f64) - 0.01, var54, var28, var36);
                self.t.vertex_uv(var42, (y as f64) - 0.01, var56, var30, var36);
                self.t.vertex_uv(var38, (y as f64) - 0.01, var56, var30, var32);
                self.t.vertex_uv(var38, (y as f64) - 0.01, var54, var28, var32);
                self.t.vertex_uv(var42, (y as f64) - 0.01, var54, var28, var36);
            } else {
                if y > 1 && self.w.is_air(x - 1, y - 1, z) {
                    self.t.vertex_uv(var38, (y as f64) - 0.01, var56, var30, var34);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var56, var30, var36);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var54, var28, var36);
                    self.t.vertex_uv(var38, (y as f64) - 0.01, var54, var28, var34);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var56, var30, var34);
                    self.t.vertex_uv(var38, (y as f64) - 0.01, var56, var30, var36);
                    self.t.vertex_uv(var38, (y as f64) - 0.01, var54, var28, var36);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var54, var28, var34);
                }
                if y > 1 && self.w.is_air(x + 1, y - 1, z) {
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var56, var30, var32);
                    self.t.vertex_uv(var42, (y as f64) - 0.01, var56, var30, var34);
                    self.t.vertex_uv(var42, (y as f64) - 0.01, var54, var28, var34);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var54, var28, var32);
                    self.t.vertex_uv(var42, (y as f64) - 0.01, var56, var30, var32);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var56, var30, var34);
                    self.t.vertex_uv(var40, (y as f64) - 0.01, var54, var28, var34);
                    self.t.vertex_uv(var42, (y as f64) - 0.01, var54, var28, var32);
                }
            }
        }
        if (!var58 || !var59) && (var60 || var61 || var58 || var59) {
            if var58 && !var59 {
                self.t.vertex_uv(var40, ((y + 1) as f64), var44, var16, var22);
                self.t.vertex_uv(var40, ((y + 0) as f64), var44, var16, var24);
                self.t.vertex_uv(var40, ((y + 0) as f64), var46, var18, var24);
                self.t.vertex_uv(var40, ((y + 1) as f64), var46, var18, var22);
                self.t.vertex_uv(var40, ((y + 1) as f64), var46, var16, var22);
                self.t.vertex_uv(var40, ((y + 0) as f64), var46, var16, var24);
                self.t.vertex_uv(var40, ((y + 0) as f64), var44, var18, var24);
                self.t.vertex_uv(var40, ((y + 1) as f64), var44, var18, var22);
                if !var61 && !var60 {
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var28, var32);
                    self.t.vertex_uv(var50, ((y + 0) as f64), var46, var28, var36);
                    self.t.vertex_uv(var52, ((y + 0) as f64), var46, var30, var36);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var30, var32);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var28, var32);
                    self.t.vertex_uv(var52, ((y + 0) as f64), var46, var28, var36);
                    self.t.vertex_uv(var50, ((y + 0) as f64), var46, var30, var36);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var30, var32);
                }
                if var62 || y < var5 - 1 && self.w.is_air(x, y + 1, z - 1) {
                    self.t.vertex_uv(var50, ((y + 1) as f64), var44, var30, var32);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var30, var34);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var28, var34);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var44, var28, var32);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var30, var32);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var44, var30, var34);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var44, var28, var34);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var28, var32);
                }
                if var63 || y > 1 && self.w.is_air(x, y - 1, z - 1) {
                    self.t.vertex_uv(var50, (y as f64), var44, var30, var32);
                    self.t.vertex_uv(var50, (y as f64), var46, var30, var34);
                    self.t.vertex_uv(var52, (y as f64), var46, var28, var34);
                    self.t.vertex_uv(var52, (y as f64), var44, var28, var32);
                    self.t.vertex_uv(var50, (y as f64), var46, var30, var32);
                    self.t.vertex_uv(var50, (y as f64), var44, var30, var34);
                    self.t.vertex_uv(var52, (y as f64), var44, var28, var34);
                    self.t.vertex_uv(var52, (y as f64), var46, var28, var32);
                }
            } else if !var58 && var59 {
                self.t.vertex_uv(var40, ((y + 1) as f64), var46, var18, var22);
                self.t.vertex_uv(var40, ((y + 0) as f64), var46, var18, var24);
                self.t.vertex_uv(var40, ((y + 0) as f64), var48, var20, var24);
                self.t.vertex_uv(var40, ((y + 1) as f64), var48, var20, var22);
                self.t.vertex_uv(var40, ((y + 1) as f64), var48, var18, var22);
                self.t.vertex_uv(var40, ((y + 0) as f64), var48, var18, var24);
                self.t.vertex_uv(var40, ((y + 0) as f64), var46, var20, var24);
                self.t.vertex_uv(var40, ((y + 1) as f64), var46, var20, var22);
                if !var61 && !var60 {
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var28, var32);
                    self.t.vertex_uv(var52, ((y + 0) as f64), var46, var28, var36);
                    self.t.vertex_uv(var50, ((y + 0) as f64), var46, var30, var36);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var30, var32);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var28, var32);
                    self.t.vertex_uv(var50, ((y + 0) as f64), var46, var28, var36);
                    self.t.vertex_uv(var52, ((y + 0) as f64), var46, var30, var36);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var30, var32);
                }
                if var62 || y < var5 - 1 && self.w.is_air(x, y + 1, z + 1) {
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var28, var34);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var48, var28, var36);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var48, var30, var36);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var30, var34);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var48, var28, var34);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var28, var36);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var30, var36);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var48, var30, var34);
                }
                if var63 || y > 1 && self.w.is_air(x, y - 1, z + 1) {
                    self.t.vertex_uv(var50, (y as f64), var46, var28, var34);
                    self.t.vertex_uv(var50, (y as f64), var48, var28, var36);
                    self.t.vertex_uv(var52, (y as f64), var48, var30, var36);
                    self.t.vertex_uv(var52, (y as f64), var46, var30, var34);
                    self.t.vertex_uv(var50, (y as f64), var48, var28, var34);
                    self.t.vertex_uv(var50, (y as f64), var46, var28, var36);
                    self.t.vertex_uv(var52, (y as f64), var46, var30, var36);
                    self.t.vertex_uv(var52, (y as f64), var48, var30, var34);
                }
            }
        } else {
            self.t.vertex_uv(var40, ((y + 1) as f64), var48, var16, var22);
            self.t.vertex_uv(var40, ((y + 0) as f64), var48, var16, var24);
            self.t.vertex_uv(var40, ((y + 0) as f64), var44, var20, var24);
            self.t.vertex_uv(var40, ((y + 1) as f64), var44, var20, var22);
            self.t.vertex_uv(var40, ((y + 1) as f64), var44, var16, var22);
            self.t.vertex_uv(var40, ((y + 0) as f64), var44, var16, var24);
            self.t.vertex_uv(var40, ((y + 0) as f64), var48, var20, var24);
            self.t.vertex_uv(var40, ((y + 1) as f64), var48, var20, var22);
            if var62 {
                self.t.vertex_uv(var52, ((y + 1) as f64), var48, var30, var36);
                self.t.vertex_uv(var52, ((y + 1) as f64), var44, var30, var32);
                self.t.vertex_uv(var50, ((y + 1) as f64), var44, var28, var32);
                self.t.vertex_uv(var50, ((y + 1) as f64), var48, var28, var36);
                self.t.vertex_uv(var52, ((y + 1) as f64), var44, var30, var36);
                self.t.vertex_uv(var52, ((y + 1) as f64), var48, var30, var32);
                self.t.vertex_uv(var50, ((y + 1) as f64), var48, var28, var32);
                self.t.vertex_uv(var50, ((y + 1) as f64), var44, var28, var36);
            } else {
                if y < var5 - 1 && self.w.is_air(x, y + 1, z - 1) {
                    self.t.vertex_uv(var50, ((y + 1) as f64), var44, var30, var32);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var30, var34);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var28, var34);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var44, var28, var32);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var30, var32);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var44, var30, var34);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var44, var28, var34);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var28, var32);
                }
                if y < var5 - 1 && self.w.is_air(x, y + 1, z + 1) {
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var28, var34);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var48, var28, var36);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var48, var30, var36);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var30, var34);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var48, var28, var34);
                    self.t.vertex_uv(var50, ((y + 1) as f64), var46, var28, var36);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var46, var30, var36);
                    self.t.vertex_uv(var52, ((y + 1) as f64), var48, var30, var34);
                }
            }
            if var63 {
                self.t.vertex_uv(var52, (y as f64), var48, var30, var36);
                self.t.vertex_uv(var52, (y as f64), var44, var30, var32);
                self.t.vertex_uv(var50, (y as f64), var44, var28, var32);
                self.t.vertex_uv(var50, (y as f64), var48, var28, var36);
                self.t.vertex_uv(var52, (y as f64), var44, var30, var36);
                self.t.vertex_uv(var52, (y as f64), var48, var30, var32);
                self.t.vertex_uv(var50, (y as f64), var48, var28, var32);
                self.t.vertex_uv(var50, (y as f64), var44, var28, var36);
            } else {
                if y > 1 && self.w.is_air(x, y - 1, z - 1) {
                    self.t.vertex_uv(var50, (y as f64), var44, var30, var32);
                    self.t.vertex_uv(var50, (y as f64), var46, var30, var34);
                    self.t.vertex_uv(var52, (y as f64), var46, var28, var34);
                    self.t.vertex_uv(var52, (y as f64), var44, var28, var32);
                    self.t.vertex_uv(var50, (y as f64), var46, var30, var32);
                    self.t.vertex_uv(var50, (y as f64), var44, var30, var34);
                    self.t.vertex_uv(var52, (y as f64), var44, var28, var34);
                    self.t.vertex_uv(var52, (y as f64), var46, var28, var32);
                }
                if y > 1 && self.w.is_air(x, y - 1, z + 1) {
                    self.t.vertex_uv(var50, (y as f64), var46, var28, var34);
                    self.t.vertex_uv(var50, (y as f64), var48, var28, var36);
                    self.t.vertex_uv(var52, (y as f64), var48, var30, var36);
                    self.t.vertex_uv(var52, (y as f64), var46, var30, var34);
                    self.t.vertex_uv(var50, (y as f64), var48, var28, var34);
                    self.t.vertex_uv(var50, (y as f64), var46, var28, var36);
                    self.t.vertex_uv(var52, (y as f64), var46, var30, var36);
                    self.t.vertex_uv(var52, (y as f64), var48, var30, var34);
                }
            }
        }
        true
    }

    /// Render type 20, `func_35929_i`: vines on each attached side and under a ceiling.
    #[allow(unused_parens, clippy::all)]
    pub(super) fn render_block_vine(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let var6 = self.face_tex(self.texture_meta(id, 0, 0));
        let var7 = 1.0f32;
        let own = self.mixed(id, x, y, z);
        self.t.set_brightness(own);
        let (var9, var10, var11) = rgb(self.color_multiplier(id, x, y, z));
        self.color(var7 * var9, var7 * var10, var7 * var11);
        let var8 = (var6 & 15) << 4;
        let var21 = var6 & 240;
        let var22 = (var8 as f32 / 256.0f32) as f64;
        let var12 = ((var8 as f32 + 15.99f32) / 256.0f32) as f64;
        let var14 = (var21 as f32 / 256.0f32) as f64;
        let var16 = ((var21 as f32 + 15.99f32) / 256.0f32) as f64;
        let var18 = 0.05f32 as f64;
        let var20 = self.w.meta(x, y, z);
        if var20 & 2 != 0 {
            self.t.vertex_uv((x as f64) + var18, ((y + 1) as f64), ((z + 1) as f64), var22, var14);
            self.t.vertex_uv((x as f64) + var18, ((y + 0) as f64), ((z + 1) as f64), var22, var16);
            self.t.vertex_uv((x as f64) + var18, ((y + 0) as f64), ((z + 0) as f64), var12, var16);
            self.t.vertex_uv((x as f64) + var18, ((y + 1) as f64), ((z + 0) as f64), var12, var14);
            self.t.vertex_uv((x as f64) + var18, ((y + 1) as f64), ((z + 0) as f64), var12, var14);
            self.t.vertex_uv((x as f64) + var18, ((y + 0) as f64), ((z + 0) as f64), var12, var16);
            self.t.vertex_uv((x as f64) + var18, ((y + 0) as f64), ((z + 1) as f64), var22, var16);
            self.t.vertex_uv((x as f64) + var18, ((y + 1) as f64), ((z + 1) as f64), var22, var14);
        }
        if var20 & 8 != 0 {
            self.t.vertex_uv(((x + 1) as f64) - var18, ((y + 0) as f64), ((z + 1) as f64), var12, var16);
            self.t.vertex_uv(((x + 1) as f64) - var18, ((y + 1) as f64), ((z + 1) as f64), var12, var14);
            self.t.vertex_uv(((x + 1) as f64) - var18, ((y + 1) as f64), ((z + 0) as f64), var22, var14);
            self.t.vertex_uv(((x + 1) as f64) - var18, ((y + 0) as f64), ((z + 0) as f64), var22, var16);
            self.t.vertex_uv(((x + 1) as f64) - var18, ((y + 0) as f64), ((z + 0) as f64), var22, var16);
            self.t.vertex_uv(((x + 1) as f64) - var18, ((y + 1) as f64), ((z + 0) as f64), var22, var14);
            self.t.vertex_uv(((x + 1) as f64) - var18, ((y + 1) as f64), ((z + 1) as f64), var12, var14);
            self.t.vertex_uv(((x + 1) as f64) - var18, ((y + 0) as f64), ((z + 1) as f64), var12, var16);
        }
        if var20 & 4 != 0 {
            self.t.vertex_uv(((x + 1) as f64), ((y + 0) as f64), (z as f64) + var18, var12, var16);
            self.t.vertex_uv(((x + 1) as f64), ((y + 1) as f64), (z as f64) + var18, var12, var14);
            self.t.vertex_uv(((x + 0) as f64), ((y + 1) as f64), (z as f64) + var18, var22, var14);
            self.t.vertex_uv(((x + 0) as f64), ((y + 0) as f64), (z as f64) + var18, var22, var16);
            self.t.vertex_uv(((x + 0) as f64), ((y + 0) as f64), (z as f64) + var18, var22, var16);
            self.t.vertex_uv(((x + 0) as f64), ((y + 1) as f64), (z as f64) + var18, var22, var14);
            self.t.vertex_uv(((x + 1) as f64), ((y + 1) as f64), (z as f64) + var18, var12, var14);
            self.t.vertex_uv(((x + 1) as f64), ((y + 0) as f64), (z as f64) + var18, var12, var16);
        }
        if var20 & 1 != 0 {
            self.t.vertex_uv(((x + 1) as f64), ((y + 1) as f64), ((z + 1) as f64) - var18, var22, var14);
            self.t.vertex_uv(((x + 1) as f64), ((y + 0) as f64), ((z + 1) as f64) - var18, var22, var16);
            self.t.vertex_uv(((x + 0) as f64), ((y + 0) as f64), ((z + 1) as f64) - var18, var12, var16);
            self.t.vertex_uv(((x + 0) as f64), ((y + 1) as f64), ((z + 1) as f64) - var18, var12, var14);
            self.t.vertex_uv(((x + 0) as f64), ((y + 1) as f64), ((z + 1) as f64) - var18, var12, var14);
            self.t.vertex_uv(((x + 0) as f64), ((y + 0) as f64), ((z + 1) as f64) - var18, var12, var16);
            self.t.vertex_uv(((x + 1) as f64), ((y + 0) as f64), ((z + 1) as f64) - var18, var22, var16);
            self.t.vertex_uv(((x + 1) as f64), ((y + 1) as f64), ((z + 1) as f64) - var18, var22, var14);
        }
        if self.w.is_block_normal_cube(x, y + 1, z) {
            self.t.vertex_uv(((x + 1) as f64), ((y + 1) as f64) - var18, ((z + 0) as f64), var22, var14);
            self.t.vertex_uv(((x + 1) as f64), ((y + 1) as f64) - var18, ((z + 1) as f64), var22, var16);
            self.t.vertex_uv(((x + 0) as f64), ((y + 1) as f64) - var18, ((z + 1) as f64), var12, var16);
            self.t.vertex_uv(((x + 0) as f64), ((y + 1) as f64) - var18, ((z + 0) as f64), var12, var14);
        }
        true
    }

    /// Render type 19, `renderBlockCrops` in b1.8.1: pumpkin and melon stems, bent towards a
    /// grown fruit.
    pub(super) fn render_block_stem(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let own = self.mixed(id, x, y, z);
        self.t.set_brightness(own);
        let var7 = 1.0f32;
        let (r, g, b) = self.tinted_rgb(self.color_multiplier(id, x, y, z));
        self.color(var7 * r, var7 * g, var7 * b);
        let meta = self.w.meta(x, y, z);
        // BlockStem.setBlockBoundsBasedOnState: maxY = (meta * 2 + 2) / 16.
        let max_y = ((meta * 2 + 2) as f32 / 16.0f32) as f64;
        let fruit = if id == 104 { 86 } else { 103 };
        let dir = if meta < 7 {
            -1
        } else if self.w.block(x - 1, y, z) == fruit {
            0
        } else if self.w.block(x + 1, y, z) == fruit {
            1
        } else if self.w.block(x, y, z - 1) == fruit {
            2
        } else if self.w.block(x, y, z + 1) == fruit {
            3
        } else {
            -1
        };
        let (xf, yf, zf) = (x as f64, y as f64, z as f64);
        if dir < 0 {
            self.render_stem_straight(id, meta, max_y, xf, yf, zf);
        } else {
            self.render_stem_straight(id, meta, 0.5, xf, yf, zf);
            self.render_stem_bent(id, meta, dir, max_y, xf, yf, zf);
        }
        true
    }

    /// `func_35928_a`: the straight stem, `height` tall.
    fn render_stem_straight(&mut self, id: u8, meta: i32, height: f64, x: f64, y: f64, z: f64) {
        let tex = self.face_tex(self.texture_meta(id, 0, meta));
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let u0 = (tu as f32 / 256.0f32) as f64;
        let u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        let v0 = (tv as f32 / 256.0f32) as f64;
        let v1 = (tv as f64 + 15.99f32 as f64 * height) / 256.0;
        let x0 = x + 0.5 - 0.45f32 as f64;
        let x1 = x + 0.5 + 0.45f32 as f64;
        let z0 = z + 0.5 - 0.45f32 as f64;
        let z1 = z + 0.5 + 0.45f32 as f64;
        let t = &mut *self.t;
        t.vertex_uv(x0, y + height, z0, u0, v0);
        t.vertex_uv(x0, y + 0.0, z0, u0, v1);
        t.vertex_uv(x1, y + 0.0, z1, u1, v1);
        t.vertex_uv(x1, y + height, z1, u1, v0);
        t.vertex_uv(x1, y + height, z1, u0, v0);
        t.vertex_uv(x1, y + 0.0, z1, u0, v1);
        t.vertex_uv(x0, y + 0.0, z0, u1, v1);
        t.vertex_uv(x0, y + height, z0, u1, v0);
        t.vertex_uv(x0, y + height, z1, u0, v0);
        t.vertex_uv(x0, y + 0.0, z1, u0, v1);
        t.vertex_uv(x1, y + 0.0, z0, u1, v1);
        t.vertex_uv(x1, y + height, z0, u1, v0);
        t.vertex_uv(x1, y + height, z0, u0, v0);
        t.vertex_uv(x1, y + 0.0, z0, u0, v1);
        t.vertex_uv(x0, y + 0.0, z1, u1, v1);
        t.vertex_uv(x0, y + height, z1, u1, v0);
    }

    /// `func_35926_a`: the bent stem towards the fruit (texture one row below).
    #[allow(clippy::too_many_arguments)]
    fn render_stem_bent(&mut self, id: u8, meta: i32, dir: i32, height: f64, x: f64, y: f64, z: f64) {
        let tex = if self.override_texture >= 0 { self.override_texture } else { self.texture_meta(id, 0, meta) + 16 };
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let mut u0 = (tu as f32 / 256.0f32) as f64;
        let mut u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        let v0 = (tv as f32 / 256.0f32) as f64;
        let v1 = (tv as f64 + 15.99f32 as f64 * height) / 256.0;
        let x0 = x + 0.5 - 0.5;
        let x1 = x + 0.5 + 0.5;
        let z0 = z + 0.5 - 0.5;
        let z1 = z + 0.5 + 0.5;
        let xm = x + 0.5;
        let zm = z + 0.5;
        if (dir + 1) / 2 % 2 == 1 {
            std::mem::swap(&mut u0, &mut u1);
        }
        let t = &mut *self.t;
        if dir < 2 {
            t.vertex_uv(x0, y + height, zm, u0, v0);
            t.vertex_uv(x0, y + 0.0, zm, u0, v1);
            t.vertex_uv(x1, y + 0.0, zm, u1, v1);
            t.vertex_uv(x1, y + height, zm, u1, v0);
            t.vertex_uv(x1, y + height, zm, u1, v0);
            t.vertex_uv(x1, y + 0.0, zm, u1, v1);
            t.vertex_uv(x0, y + 0.0, zm, u0, v1);
            t.vertex_uv(x0, y + height, zm, u0, v0);
        } else {
            t.vertex_uv(xm, y + height, z1, u0, v0);
            t.vertex_uv(xm, y + 0.0, z1, u0, v1);
            t.vertex_uv(xm, y + 0.0, z0, u1, v1);
            t.vertex_uv(xm, y + height, z0, u1, v0);
            t.vertex_uv(xm, y + height, z0, u1, v0);
            t.vertex_uv(xm, y + 0.0, z0, u1, v1);
            t.vertex_uv(xm, y + 0.0, z1, u0, v1);
            t.vertex_uv(xm, y + height, z1, u0, v0);
        }
    }

    /// Render type 21, `func_35923_a`: fence gates (posts, and the closed or open bars).
    #[allow(unused_assignments, unused_parens, unused_mut, clippy::all)]
    pub(super) fn render_block_fence_gate(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let var6 = self.w.meta(x, y, z);
        let var7 = var6 & 4 != 0;
        let var8 = var6 & 3;
        let mut var15: f32;
        let mut var16: f32;
        let mut var17: f32;
        let mut var18: f32;
        if var8 != 3 && var8 != 1 {
            var15 = 0.0f32;
            var16 = 2.0f32 / 16.0;
            var17 = 7.0f32 / 16.0;
            var18 = 9.0f32 / 16.0;
            self.set_block_bounds(var15, 5.0f32 / 16.0, var17, var16, 1.0f32, var18);
            self.render_standard_block_pub(id, x, y, z);
            var15 = 14.0f32 / 16.0;
            var16 = 1.0f32;
            self.set_block_bounds(var15, 5.0f32 / 16.0, var17, var16, 1.0f32, var18);
            self.render_standard_block_pub(id, x, y, z);
        } else {
            var15 = 7.0f32 / 16.0;
            var16 = 9.0f32 / 16.0;
            var17 = 0.0f32;
            var18 = 2.0f32 / 16.0;
            self.set_block_bounds(var15, 5.0f32 / 16.0, var17, var16, 1.0f32, var18);
            self.render_standard_block_pub(id, x, y, z);
            var17 = 14.0f32 / 16.0;
            var18 = 1.0f32;
            self.set_block_bounds(var15, 5.0f32 / 16.0, var17, var16, 1.0f32, var18);
            self.render_standard_block_pub(id, x, y, z);
        }
        if !var7 {
            if var8 != 3 && var8 != 1 {
                var15 = 6.0f32 / 16.0;
                var16 = 0.5f32;
                var17 = 7.0f32 / 16.0;
                var18 = 9.0f32 / 16.0;
                self.set_block_bounds(var15, 6.0f32 / 16.0, var17, var16, 15.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
                var15 = 0.5f32;
                var16 = 10.0f32 / 16.0;
                self.set_block_bounds(var15, 6.0f32 / 16.0, var17, var16, 15.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
                var15 = 10.0f32 / 16.0;
                var16 = 14.0f32 / 16.0;
                self.set_block_bounds(var15, 6.0f32 / 16.0, var17, var16, 9.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
                self.set_block_bounds(var15, 12.0f32 / 16.0, var17, var16, 15.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
                var15 = 2.0f32 / 16.0;
                var16 = 6.0f32 / 16.0;
                self.set_block_bounds(var15, 6.0f32 / 16.0, var17, var16, 9.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
                self.set_block_bounds(var15, 12.0f32 / 16.0, var17, var16, 15.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
            } else {
                var15 = 7.0f32 / 16.0;
                var16 = 9.0f32 / 16.0;
                var17 = 6.0f32 / 16.0;
                var18 = 0.5f32;
                self.set_block_bounds(var15, 6.0f32 / 16.0, var17, var16, 15.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
                var17 = 0.5f32;
                var18 = 10.0f32 / 16.0;
                self.set_block_bounds(var15, 6.0f32 / 16.0, var17, var16, 15.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
                var17 = 10.0f32 / 16.0;
                var18 = 14.0f32 / 16.0;
                self.set_block_bounds(var15, 6.0f32 / 16.0, var17, var16, 9.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
                self.set_block_bounds(var15, 12.0f32 / 16.0, var17, var16, 15.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
                var17 = 2.0f32 / 16.0;
                var18 = 6.0f32 / 16.0;
                self.set_block_bounds(var15, 6.0f32 / 16.0, var17, var16, 9.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
                self.set_block_bounds(var15, 12.0f32 / 16.0, var17, var16, 15.0f32 / 16.0, var18);
                self.render_standard_block_pub(id, x, y, z);
            }
        } else if var8 == 3 {
            self.set_block_bounds(13.0f32 / 16.0, 6.0f32 / 16.0, 0.0f32, 15.0f32 / 16.0, 15.0f32 / 16.0, 2.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(13.0f32 / 16.0, 6.0f32 / 16.0, 14.0f32 / 16.0, 15.0f32 / 16.0, 15.0f32 / 16.0, 1.0f32);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(9.0f32 / 16.0, 6.0f32 / 16.0, 0.0f32, 13.0f32 / 16.0, 9.0f32 / 16.0, 2.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(9.0f32 / 16.0, 6.0f32 / 16.0, 14.0f32 / 16.0, 13.0f32 / 16.0, 9.0f32 / 16.0, 1.0f32);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(9.0f32 / 16.0, 12.0f32 / 16.0, 0.0f32, 13.0f32 / 16.0, 15.0f32 / 16.0, 2.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(9.0f32 / 16.0, 12.0f32 / 16.0, 14.0f32 / 16.0, 13.0f32 / 16.0, 15.0f32 / 16.0, 1.0f32);
            self.render_standard_block_pub(id, x, y, z);
        } else if var8 == 1 {
            self.set_block_bounds(1.0f32 / 16.0, 6.0f32 / 16.0, 0.0f32, 3.0f32 / 16.0, 15.0f32 / 16.0, 2.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(1.0f32 / 16.0, 6.0f32 / 16.0, 14.0f32 / 16.0, 3.0f32 / 16.0, 15.0f32 / 16.0, 1.0f32);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(3.0f32 / 16.0, 6.0f32 / 16.0, 0.0f32, 7.0f32 / 16.0, 9.0f32 / 16.0, 2.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(3.0f32 / 16.0, 6.0f32 / 16.0, 14.0f32 / 16.0, 7.0f32 / 16.0, 9.0f32 / 16.0, 1.0f32);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(3.0f32 / 16.0, 12.0f32 / 16.0, 0.0f32, 7.0f32 / 16.0, 15.0f32 / 16.0, 2.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(3.0f32 / 16.0, 12.0f32 / 16.0, 14.0f32 / 16.0, 7.0f32 / 16.0, 15.0f32 / 16.0, 1.0f32);
            self.render_standard_block_pub(id, x, y, z);
        } else if var8 == 0 {
            self.set_block_bounds(0.0f32, 6.0f32 / 16.0, 13.0f32 / 16.0, 2.0f32 / 16.0, 15.0f32 / 16.0, 15.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(14.0f32 / 16.0, 6.0f32 / 16.0, 13.0f32 / 16.0, 1.0f32, 15.0f32 / 16.0, 15.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(0.0f32, 6.0f32 / 16.0, 9.0f32 / 16.0, 2.0f32 / 16.0, 9.0f32 / 16.0, 13.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(14.0f32 / 16.0, 6.0f32 / 16.0, 9.0f32 / 16.0, 1.0f32, 9.0f32 / 16.0, 13.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(0.0f32, 12.0f32 / 16.0, 9.0f32 / 16.0, 2.0f32 / 16.0, 15.0f32 / 16.0, 13.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(14.0f32 / 16.0, 12.0f32 / 16.0, 9.0f32 / 16.0, 1.0f32, 15.0f32 / 16.0, 13.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
        } else if var8 == 2 {
            self.set_block_bounds(0.0f32, 6.0f32 / 16.0, 1.0f32 / 16.0, 2.0f32 / 16.0, 15.0f32 / 16.0, 3.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(14.0f32 / 16.0, 6.0f32 / 16.0, 1.0f32 / 16.0, 1.0f32, 15.0f32 / 16.0, 3.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(0.0f32, 6.0f32 / 16.0, 3.0f32 / 16.0, 2.0f32 / 16.0, 9.0f32 / 16.0, 7.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(14.0f32 / 16.0, 6.0f32 / 16.0, 3.0f32 / 16.0, 1.0f32, 9.0f32 / 16.0, 7.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(0.0f32, 12.0f32 / 16.0, 3.0f32 / 16.0, 2.0f32 / 16.0, 15.0f32 / 16.0, 7.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
            self.set_block_bounds(14.0f32 / 16.0, 12.0f32 / 16.0, 3.0f32 / 16.0, 1.0f32, 15.0f32 / 16.0, 7.0f32 / 16.0);
            self.render_standard_block_pub(id, x, y, z);
        }
        self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        true
    }

    // ------------------------------------------------------------------ type 10: stairs

    pub(super) fn render_block_stairs(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let meta = self.w.meta(x, y, z);
        let mut rendered = false;
        let parts: Option<[[f32; 6]; 2]> = match meta {
            0 => Some([[0.0, 0.0, 0.0, 0.5, 0.5, 1.0], [0.5, 0.0, 0.0, 1.0, 1.0, 1.0]]),
            1 => Some([[0.0, 0.0, 0.0, 0.5, 1.0, 1.0], [0.5, 0.0, 0.0, 1.0, 0.5, 1.0]]),
            2 => Some([[0.0, 0.0, 0.0, 1.0, 0.5, 0.5], [0.0, 0.0, 0.5, 1.0, 1.0, 1.0]]),
            3 => Some([[0.0, 0.0, 0.0, 1.0, 1.0, 0.5], [0.0, 0.0, 0.5, 1.0, 0.5, 1.0]]),
            _ => None,
        };
        if let Some(parts) = parts {
            for p in parts {
                self.set_block_bounds(p[0], p[1], p[2], p[3], p[4], p[5]);
                self.render_standard_block_pub(id, x, y, z);
            }
            rendered = true;
        }
        self.set_block_bounds(0.0, 0.0, 0.0, 1.0, 1.0, 1.0);
        rendered
    }

    // ------------------------------------------------------------------ type 7: door

    pub(super) fn render_block_door(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let (cb, ct, cns, cew) = (0.5f32, 1.0f32, 0.8f32, 0.6f32);
        let own = self.mixed(id, x, y, z);
        let meta = self.w.meta(x, y, z);
        let base = rprops(id).tex_index as i32;
        let b = self.b;
        let l = if b.min_y > 0.0 { own } else { self.mixed(id, x, y - 1, z) };
        self.t.set_brightness(l);
        self.color(cb, cb, cb);
        let t = door_texture(base, 0, meta);
        self.render_bottom_face(x as f64, y as f64, z as f64, t);
        let l = if b.max_y < 1.0 { own } else { self.mixed(id, x, y + 1, z) };
        self.t.set_brightness(l);
        self.color(ct, ct, ct);
        let t = door_texture(base, 1, meta);
        self.render_top_face(x as f64, y as f64, z as f64, t);
        for (side, (dx, dz), shade, inside) in [
            (2usize, (0, -1), cns, b.min_z > 0.0),
            (3, (0, 1), cns, b.max_z < 1.0),
            (4, (-1, 0), cew, b.min_x > 0.0),
            (5, (1, 0), cew, b.max_x < 1.0),
        ] {
            let l = if inside { own } else { self.mixed(id, x + dx, y, z + dz) };
            self.t.set_brightness(l);
            self.color(shade, shade, shade);
            let mut t = door_texture(base, side as i32, meta);
            if t < 0 {
                self.flip_texture = true;
                t = -t;
            }
            match side {
                2 => self.render_east_face(x as f64, y as f64, z as f64, t),
                3 => self.render_west_face(x as f64, y as f64, z as f64, t),
                4 => self.render_north_face(x as f64, y as f64, z as f64, t),
                _ => self.render_south_face(x as f64, y as f64, z as f64, t),
            }
            self.flip_texture = false;
        }
        true
    }
}

/// `BlockDoor.getState`
fn door_state(meta: i32) -> i32 {
    if meta & 4 == 0 {
        (meta - 1) & 3
    } else {
        meta & 3
    }
}

/// `BlockDoor.getBlockTextureFromSideAndMetadata` (negative: mirrored).
fn door_texture(base: i32, side: i32, meta: i32) -> i32 {
    if side != 0 && side != 1 {
        let state = door_state(meta);
        if (state == 0 || state == 2) ^ (side <= 3) {
            base
        } else {
            let mut k = state / 2 + ((side & 1) ^ state);
            k += (meta & 4) / 4;
            let mut t = base - (meta & 8) * 2;
            if k & 1 != 0 {
                t = -t;
            }
            t
        }
    } else {
        base
    }
}
