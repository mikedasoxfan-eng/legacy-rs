//! `RenderBlocks`: 0 (standard cube, with the smooth-lighting path), 1 (crossed plants),
//! 4 (fluids) and 13 (cactus) here, every other render type in `blocks_special.rs`.
//! Field and branch structure follow the Java so the lighting and UVs come out the same;
//! obfuscated names are noted. Checked vertex for vertex by `tools/verify.sh` (terrain and
//! the block showcase).

#[path = "blocks_special.rs"]
mod special;

use super::access::Access;
use super::{rprops, Tessellator};
use worldgen::block::{self, mat, MAT_BUILTSNOW, MAT_ICE, MAT_SNOW};
use worldgen::math;

/// Biome color lookup tables (`grasscolor.png`, `foliagecolor.png`), 256x256 RGB.
#[derive(Clone)]
pub struct Colorizers {
    pub grass: Vec<u32>,
    pub foliage: Vec<u32>,
}

impl Colorizers {
    fn lookup(buf: &[u32], temp: f64, humid: f64) -> u32 {
        let humid = humid * temp;
        let i = ((1.0 - temp) * 255.0) as i32;
        let j = ((1.0 - humid) * 255.0) as i32;
        buf[(j << 8 | i) as usize]
    }
    pub fn grass(&self, t: f64, h: f64) -> u32 {
        Self::lookup(&self.grass, t, h)
    }
    pub fn foliage(&self, t: f64, h: f64) -> u32 {
        Self::lookup(&self.foliage, t, h)
    }
}

/// The mutable block bounds RenderBlocks reads (`Block.minX` ... as doubles).
#[derive(Clone, Copy)]
pub(crate) struct Bounds {
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
}

pub struct RenderBlocks<'a, 'b> {
    pub w: &'b Access<'a>,
    pub t: &'b mut Tessellator,
    col: &'b Colorizers,
    b: Bounds,
    pub ao: bool,
    enable_ao: bool,
    // per-vertex colors for the AO face functions
    r_tl: f32,
    r_bl: f32,
    r_br: f32,
    r_tr: f32,
    g_tl: f32,
    g_bl: f32,
    g_br: f32,
    g_tr: f32,
    b_tl: f32,
    b_bl: f32,
    b_br: f32,
    b_tr: f32,
    /// `overrideBlockTexture`
    pub override_texture: i32,
    /// `flipTexture` (doors)
    flip_texture: bool,
    /// `renderAllFaces` (`func_31075_a`, pistons being pushed)
    pub render_all_faces: bool,
    /// UV rotations `field_31083_k` (top), `field_31082_l` (bottom), `field_31087_g` (east),
    /// `field_31086_h` (west), `field_31085_i` (south), `field_31084_j` (north).
    uv_rotate_top: i32,
    uv_rotate_bottom: i32,
    uv_rotate_east: i32,
    uv_rotate_west: i32,
    uv_rotate_south: i32,
    uv_rotate_north: i32,
}

#[inline]
fn rgb(c: u32) -> (f32, f32, f32) {
    ((c >> 16 & 255) as f32 / 255.0, (c >> 8 & 255) as f32 / 255.0, (c & 255) as f32 / 255.0)
}

impl<'a, 'b> RenderBlocks<'a, 'b> {
    pub fn new(w: &'b Access<'a>, t: &'b mut Tessellator, col: &'b Colorizers) -> Self {
        RenderBlocks {
            w,
            t,
            col,
            b: Bounds {
                min_x: 0.0,
                min_y: 0.0,
                min_z: 0.0,
                max_x: 1.0,
                max_y: 1.0,
                max_z: 1.0,
            },
            ao: true,
            enable_ao: false,
            r_tl: 0.0,
            r_bl: 0.0,
            r_br: 0.0,
            r_tr: 0.0,
            g_tl: 0.0,
            g_bl: 0.0,
            g_br: 0.0,
            g_tr: 0.0,
            b_tl: 0.0,
            b_bl: 0.0,
            b_br: 0.0,
            b_tr: 0.0,
            override_texture: -1,
            flip_texture: false,
            render_all_faces: false,
            uv_rotate_top: 0,
            uv_rotate_bottom: 0,
            uv_rotate_east: 0,
            uv_rotate_west: 0,
            uv_rotate_south: 0,
            uv_rotate_north: 0,
        }
    }

    // ---------------------------------------------------------------- Block virtuals

    fn set_bounds(&mut self, id: u8, x: i32, y: i32, z: i32) {
        if self.override_texture >= 0 {
            use worldgen::entity::block_table as table;
            let m = self.w.meta(x, y, z) as usize;
            let retain = (matches!(id, 29 | 33 | 34) && m & 7 > 5)
                || (id == 77 && !(1..=4).contains(&(m & 7)));
            let mut b = if table::HAS_STATE_BOUNDS[id as usize] && !retain {
                table::STATE_BOUNDS[id as usize][m]
            } else {
                self.w.world.block_bounds[id as usize]
            };
            if id == 90 {
                b = if self.w.block(x - 1, y, z) != id && self.w.block(x + 1, y, z) != id {
                    worldgen::entity::AxisAlignedBB::new(0.375, 0.0, 0.0, 0.625, 1.0, 1.0)
                } else {
                    worldgen::entity::AxisAlignedBB::new(0.0, 0.0, 0.375, 1.0, 1.0, 0.625)
                };
            }
            self.b = Bounds {
                min_x: b.min_x,
                min_y: b.min_y,
                min_z: b.min_z,
                max_x: b.max_x,
                max_y: b.max_y,
                max_z: b.max_z,
            };
            return;
        }
        self.load_bounds(id, x, y, z);
    }

    /// `getBlockBrightness`; fluids override it with the brighter of their own block and
    /// the one above, ignoring their own light value (so lava does not light itself).
    #[inline]
    fn brightness(&self, id: u8, x: i32, y: i32, z: i32) -> f32 {
        if matches!(id, block::WATER_MOVING | block::WATER_STILL | block::LAVA_MOVING | block::LAVA_STILL) {
            let own = self.w.light_brightness(x, y, z);
            let above = self.w.light_brightness(x, y + 1, z);
            return if own > above { own } else { above };
        }
        self.w.brightness(x, y, z, block::light_value(id))
    }

    /// `getBlockTexture(IBlockAccess, x, y, z, side)`
    fn texture(&self, id: u8, x: i32, y: i32, z: i32, side: usize) -> i32 {
        match id {
            block::GRASS => match side {
                1 => 0,
                0 => 2,
                _ => {
                    let m = self.w.material(x, y + 1, z);
                    if m != MAT_SNOW && m != MAT_BUILTSNOW {
                        3
                    } else {
                        68
                    }
                }
            },
            block::CHEST => self.chest_texture(x, y, z, side as i32),
            _ => self
                .special_texture(id, x, y, z, side)
                .unwrap_or_else(|| self.texture_meta(id, side, self.w.meta(x, y, z))),
        }
    }

    #[inline]
    fn texture_meta(&self, id: u8, side: usize, meta: i32) -> i32 {
        rprops(id).textures[(meta & 15) as usize][side] as i32
            + if id == block::LEAVES && !self.w.fancy { 1 } else { 0 }
    }

    /// `colorMultiplier`
    fn color_multiplier(&self, id: u8, x: i32, y: i32, z: i32) -> u32 {
        match id {
            block::GRASS => {
                let (t, h) = self.w.climate(x, z);
                self.col.grass(t, h)
            }
            block::LEAVES => {
                let m = self.w.meta(x, y, z);
                if m & 1 == 1 {
                    6396257
                } else if m & 2 == 2 {
                    8431445
                } else {
                    let (t, h) = self.w.climate(x, z);
                    self.col.foliage(t, h)
                }
            }
            block::TALL_GRASS => {
                if self.w.meta(x, y, z) == 0 {
                    0xffffff
                } else {
                    let mut v = x.wrapping_mul(3129871).wrapping_add(z.wrapping_mul(6129781)).wrapping_add(y) as i64;
                    v = v.wrapping_mul(v).wrapping_mul(42317861).wrapping_add(v.wrapping_mul(11));
                    let x2 = (x as i64 + (v >> 14 & 31)) as i32;
                    let z2 = (z as i64 + (v >> 24 & 31)) as i32;
                    let (t, h) = self.w.climate(x2, z2);
                    self.col.grass(t, h)
                }
            }
            _ => 0xffffff,
        }
    }

    /// `shouldSideBeRendered`, called for block `id` with the neighbour position.
    fn should_side_be_rendered(&self, id: u8, x: i32, y: i32, z: i32, side: i32) -> bool {
        if matches!(id, 93 | 94) && self.override_texture >= 0 {
            return side != 0 && side != 1;
        }
        if id == 90 && self.override_texture >= 0 {
            if self.w.block(x, y, z) == id {
                return false;
            }
            let a = (self.w.block(x - 1, y, z) == id && self.w.block(x - 2, y, z) != id)
                || (self.w.block(x + 1, y, z) == id && self.w.block(x + 2, y, z) != id);
            let b = (self.w.block(x, y, z - 1) == id && self.w.block(x, y, z - 2) != id)
                || (self.w.block(x, y, z + 1) == id && self.w.block(x, y, z + 2) != id);
            return (a && matches!(side, 4 | 5)) || (b && matches!(side, 2 | 3));
        }
        match id {
            block::WATER_MOVING | block::WATER_STILL | block::LAVA_MOVING | block::LAVA_STILL => {
                let m = self.w.material(x, y, z);
                if m == block::material_of(id) || m == MAT_ICE {
                    return false;
                }
                if side == 1 {
                    return true;
                }
            }
            block::ICE | 20 => {
                if self.w.block(x, y, z) == id {
                    return false;
                }
            }
            block::SNOW => {
                if side == 1 {
                    return true;
                }
            }
            // BlockPortal: faces only toward the frame's open sides.
            90 => {
                let w = self.w;
                if w.block(x, y, z) == 90 {
                    return false;
                }
                let along_x = w.block(x - 1, y, z) == 90 && w.block(x - 2, y, z) != 90
                    || w.block(x + 1, y, z) == 90 && w.block(x + 2, y, z) != 90;
                let along_z = w.block(x, y, z - 1) == 90 && w.block(x, y, z - 2) != 90
                    || w.block(x, y, z + 1) == 90 && w.block(x, y, z + 2) != 90;
                return along_x && (side == 4 || side == 5) || along_z && (side == 2 || side == 3);
            }
            // BlockRedstoneRepeater
            93 | 94 => return side != 0 && side != 1,
            // BlockStep (slabs and double slabs): the top is always drawn.
            43 | 44 => {
                if side == 1 {
                    return true;
                }
                if !self.default_side(x, y, z, side) {
                    return false;
                }
                return side == 0 || self.w.block(x, y, z) != id;
            }
            _ => {}
        }
        self.default_side(x, y, z, side)
    }

    /// `Block.shouldSideBeRendered`
    fn default_side(&self, x: i32, y: i32, z: i32, side: i32) -> bool {
        let b = &self.b;
        if side == 0 && b.min_y > 0.0
            || side == 1 && b.max_y < 1.0
            || side == 2 && b.min_z > 0.0
            || side == 3 && b.max_z < 1.0
            || side == 4 && b.min_x > 0.0
            || side == 5 && b.max_x < 1.0
        {
            return true;
        }
        !self.w.is_opaque_cube(x, y, z)
    }

    // ---------------------------------------------------------------- dispatch

    /// `renderBlockByRenderType`
    pub fn render_block(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        self.set_bounds(id, x, y, z);
        let r = match rprops(id).render_type {
            0 => self.render_standard_block(id, x, y, z),
            1 => self.render_block_reed(id, x, y, z),
            2 => self.render_block_torch(id, x, y, z),
            3 => self.render_block_fire(id, x, y, z),
            4 => self.render_block_fluids(id, x, y, z),
            5 => self.render_block_redstone_wire(id, x, y, z),
            6 => self.render_block_crops(id, x, y, z),
            7 => self.render_block_door(id, x, y, z),
            8 => self.render_block_ladder(id, x, y, z),
            9 => self.render_block_rail(id, x, y, z),
            10 => self.render_block_stairs(id, x, y, z),
            11 => self.render_block_fence(id, x, y, z),
            12 => self.render_block_lever(id, x, y, z),
            13 => self.render_block_cactus(id, x, y, z),
            14 => self.render_block_bed(id, x, y, z),
            15 => self.render_block_repeater(id, x, y, z),
            16 => self.render_piston_base(id, x, y, z, false),
            17 => self.render_piston_head(id, x, y, z, true),
            _ => false,
        };
        self.store_bounds(id);
        r
    }

    /// `renderStandardBlock`, for the special renderers.
    fn render_standard_block_pub(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        self.render_standard_block(id, x, y, z)
    }

    /// `renderStandardBlock` (smooth lighting is on by default in b1.7.3).
    fn render_standard_block(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let (r, g, b) = rgb(self.color_multiplier(id, x, y, z));
        if self.ao {
            self.render_standard_block_ao(id, x, y, z, r, g, b)
        } else {
            self.render_standard_block_color(id, x, y, z, r, g, b)
        }
    }

    /// `renderStandardBlockWithColorMultiplier`. Preserve Java's multiplication order.
    #[allow(clippy::too_many_arguments)]
    fn render_standard_block_color(&mut self, id: u8, x: i32, y: i32, z: i32, r: f32, g: f32, b: f32) -> bool {
        self.enable_ao = false;
        let own = self.brightness(id, x, y, z);
        let neighbours = [(x, y - 1, z), (x, y + 1, z), (x, y, z - 1), (x, y, z + 1), (x - 1, y, z), (x + 1, y, z)];
        let mut rendered = false;
        for (side, &(nx, ny, nz)) in neighbours.iter().enumerate() {
            if !self.should_side_be_rendered(id, nx, ny, nz, side as i32) {
                continue;
            }
            let mut light = self.brightness(id, nx, ny, nz);
            let bounds = self.b;
            if match side {
                1 => {
                    bounds.max_y != 1.0
                        && !matches!(block::material_of(id), block::MAT_WATER | block::MAT_LAVA)
                }
                2 => bounds.min_z > 0.0,
                3 => bounds.max_z < 1.0,
                4 => bounds.min_x > 0.0,
                5 => bounds.max_x < 1.0,
                _ => false,
            } {
                light = own;
            }
            let shade = [0.5f32, 1.0, 0.8, 0.8, 0.6, 0.6][side];
            let tint = id != block::GRASS || side == 1;
            let (cr, cg, cb) = if tint { (shade * r, shade * g, shade * b) } else { (shade, shade, shade) };
            self.t.set_color_opaque_f(cr * light, cg * light, cb * light);
            let tex = self.texture(id, x, y, z, side);
            self.standard_face(side, x, y, z, tex);
            if side >= 2 && self.w.fancy && tex == 3 && self.override_texture < 0 {
                self.t.set_color_opaque_f(cr * light * r, cg * light * g, cb * light * b);
                self.standard_face(side, x, y, z, 38);
            }
            rendered = true;
        }
        rendered
    }

    fn standard_face(&mut self, side: usize, x: i32, y: i32, z: i32, tex: i32) {
        let (x, y, z) = (x as f64, y as f64, z as f64);
        match side {
            0 => self.render_bottom_face(x, y, z, tex),
            1 => self.render_top_face(x, y, z, tex),
            2 => self.render_east_face(x, y, z, tex),
            3 => self.render_west_face(x, y, z, tex),
            4 => self.render_north_face(x, y, z, tex),
            _ => self.render_south_face(x, y, z, tex),
        }
    }

    // ---------------------------------------------------------------- smooth lighting

    #[allow(clippy::too_many_arguments)]
    fn set_corner_colors(&mut self, base: (f32, f32, f32), tl: f32, bl: f32, br: f32, tr: f32) {
        self.r_tl = base.0 * tl;
        self.g_tl = base.1 * tl;
        self.b_tl = base.2 * tl;
        self.r_bl = base.0 * bl;
        self.g_bl = base.1 * bl;
        self.b_bl = base.2 * bl;
        self.r_br = base.0 * br;
        self.g_br = base.1 * br;
        self.b_br = base.2 * br;
        self.r_tr = base.0 * tr;
        self.g_tr = base.1 * tr;
        self.b_tr = base.2 * tr;
    }

    fn tint_corners(&mut self, r: f32, g: f32, b: f32) {
        self.r_tl *= r;
        self.r_bl *= r;
        self.r_br *= r;
        self.r_tr *= r;
        self.g_tl *= g;
        self.g_bl *= g;
        self.g_br *= g;
        self.g_tr *= g;
        self.b_tl *= b;
        self.b_bl *= b;
        self.b_br *= b;
        self.b_tr *= b;
    }

    /// `renderStandardBlockWithAmbientOcclusion`
    #[allow(clippy::too_many_arguments)]
    fn render_standard_block_ao(&mut self, id: u8, x: i32, y: i32, z: i32, r: f32, g: f32, b: f32) -> bool {
        self.enable_ao = true;
        let mut rendered = false;
        let (mut tint_bottom, tint_top, mut tint_e, mut tint_w, mut tint_n, mut tint_s) =
            (true, true, true, true, true, true);
        let br = |s: &Self, x: i32, y: i32, z: i32| s.brightness(id, x, y, z);
        let ao_x_neg = br(self, x - 1, y, z);
        let ao_y_neg = br(self, x, y - 1, z);
        let ao_z_neg = br(self, x, y, z - 1);
        let ao_x_pos = br(self, x + 1, y, z);
        let ao_y_pos = br(self, x, y + 1, z);
        let ao_z_pos = br(self, x, y, z + 1);
        let cbg = |s: &Self, x: i32, y: i32, z: i32| rprops(s.w.block(x, y, z)).can_block_grass;
        let g_xp_yp = cbg(self, x + 1, y + 1, z); // field_22338_U
        let g_xp_yn = cbg(self, x + 1, y - 1, z); // field_22359_ac
        let g_xp_zp = cbg(self, x + 1, y, z + 1); // field_22334_Y
        let g_xp_zn = cbg(self, x + 1, y, z - 1); // field_22363_aa
        let g_xn_yp = cbg(self, x - 1, y + 1, z); // field_22337_V
        let g_xn_yn = cbg(self, x - 1, y - 1, z); // field_22357_ad
        let g_xn_zn = cbg(self, x - 1, y, z - 1); // field_22335_X
        let g_xn_zp = cbg(self, x - 1, y, z + 1); // field_22333_Z
        let g_yp_zp = cbg(self, x, y + 1, z + 1); // field_22336_W
        let g_yp_zn = cbg(self, x, y + 1, z - 1); // field_22339_T
        let g_yn_zp = cbg(self, x, y - 1, z + 1); // field_22355_ae
        let g_yn_zn = cbg(self, x, y - 1, z - 1); // field_22361_ab
        if rprops(id).tex_index == 3 || self.override_texture >= 0 {
            // Grass: only the top face is biome-tinted; nothing is while cracking.
            tint_bottom = false;
            tint_e = false;
            tint_w = false;
            tint_n = false;
            tint_s = false;
        }
        let base = |t: bool, shade: f32| {
            if t {
                (r * shade, g * shade, b * shade)
            } else {
                (shade, shade, shade)
            }
        };

        // bottom (y-1)
        if self.should_side_be_rendered(id, x, y - 1, z, 0) {
            let yy = y - 1;
            let n_xn = br(self, x - 1, yy, z); // field_22376_n
            let n_zn = br(self, x, yy, z - 1); // field_22374_p
            let n_zp = br(self, x, yy, z + 1); // field_22373_q
            let n_xp = br(self, x + 1, yy, z); // field_22371_s
            let c_xn_zn = if !g_yn_zn && !g_xn_yn { n_xn } else { br(self, x - 1, yy, z - 1) };
            let c_xn_zp = if !g_yn_zp && !g_xn_yn { n_xn } else { br(self, x - 1, yy, z + 1) };
            let c_xp_zn = if !g_yn_zn && !g_xp_yn { n_xp } else { br(self, x + 1, yy, z - 1) };
            let c_xp_zp = if !g_yn_zp && !g_xp_yn { n_xp } else { br(self, x + 1, yy, z + 1) };
            let v9 = (c_xn_zp + n_xn + n_zp + ao_y_neg) / 4.0;
            let v12 = (n_zp + ao_y_neg + c_xp_zp + n_xp) / 4.0;
            let v11 = (ao_y_neg + n_zn + n_xp + c_xp_zn) / 4.0;
            let v10 = (n_xn + c_xn_zn + ao_y_neg + n_zn) / 4.0;
            self.set_corner_colors(base(tint_bottom, 0.5), v9, v10, v11, v12);
            let tex = self.texture(id, x, y, z, 0);
            self.render_bottom_face(x as f64, y as f64, z as f64, tex);
            rendered = true;
        }

        // top (y+1)
        if self.should_side_be_rendered(id, x, y + 1, z, 1) {
            let yy = y + 1;
            let n_xn = br(self, x - 1, yy, z); // field_22368_v
            let n_xp = br(self, x + 1, yy, z); // field_22364_z
            let n_zn = br(self, x, yy, z - 1); // field_22366_x
            let n_zp = br(self, x, yy, z + 1); // field_22362_A
            let c_xn_zn = if !g_yp_zn && !g_xn_yp { n_xn } else { br(self, x - 1, yy, z - 1) };
            let c_xp_zn = if !g_yp_zn && !g_xp_yp { n_xp } else { br(self, x + 1, yy, z - 1) };
            let c_xn_zp = if !g_yp_zp && !g_xn_yp { n_xn } else { br(self, x - 1, yy, z + 1) };
            let c_xp_zp = if !g_yp_zp && !g_xp_yp { n_xp } else { br(self, x + 1, yy, z + 1) };
            let v12 = (c_xn_zp + n_xn + n_zp + ao_y_pos) / 4.0;
            let v9 = (n_zp + ao_y_pos + c_xp_zp + n_xp) / 4.0;
            let v10 = (ao_y_pos + n_zn + n_xp + c_xp_zn) / 4.0;
            let v11 = (n_xn + c_xn_zn + ao_y_pos + n_zn) / 4.0;
            self.set_corner_colors(base(tint_top, 1.0), v9, v10, v11, v12);
            let tex = self.texture(id, x, y, z, 1);
            self.render_top_face(x as f64, y as f64, z as f64, tex);
            rendered = true;
        }

        // east (z-1)
        if self.should_side_be_rendered(id, x, y, z - 1, 2) {
            let zz = z - 1;
            let n_xn = br(self, x - 1, y, zz); // field_22358_C
            let n_yn = br(self, x, y - 1, zz); // field_22374_p
            let n_yp = br(self, x, y + 1, zz); // field_22366_x
            let n_xp = br(self, x + 1, y, zz); // field_22356_D
            let c_xn_yn = if !g_xn_zn && !g_yn_zn { n_xn } else { br(self, x - 1, y - 1, zz) };
            let c_xn_yp = if !g_xn_zn && !g_yp_zn { n_xn } else { br(self, x - 1, y + 1, zz) };
            let c_xp_yn = if !g_xp_zn && !g_yn_zn { n_xp } else { br(self, x + 1, y - 1, zz) };
            let c_xp_yp = if !g_xp_zn && !g_yp_zn { n_xp } else { br(self, x + 1, y + 1, zz) };
            let v9 = (n_xn + c_xn_yp + ao_z_neg + n_yp) / 4.0;
            let v10 = (ao_z_neg + n_yp + n_xp + c_xp_yp) / 4.0;
            let v11 = (n_yn + ao_z_neg + c_xp_yn + n_xp) / 4.0;
            let v12 = (c_xn_yn + n_xn + n_yn + ao_z_neg) / 4.0;
            self.set_corner_colors(base(tint_e, 0.8), v9, v10, v11, v12);
            let tex = self.texture(id, x, y, z, 2);
            self.render_east_face(x as f64, y as f64, z as f64, tex);
            if self.w.fancy && tex == 3 && self.override_texture < 0 {
                self.tint_corners(r, g, b);
                self.render_east_face(x as f64, y as f64, z as f64, 38);
            }
            rendered = true;
        }

        // west (z+1)
        if self.should_side_be_rendered(id, x, y, z + 1, 3) {
            let zz = z + 1;
            let n_xn = br(self, x - 1, y, zz); // field_22354_E
            let n_xp = br(self, x + 1, y, zz); // field_22353_F
            let n_yn = br(self, x, y - 1, zz); // field_22373_q
            let n_yp = br(self, x, y + 1, zz); // field_22362_A
            let c_xn_yn = if !g_xn_zp && !g_yn_zp { n_xn } else { br(self, x - 1, y - 1, zz) };
            let c_xn_yp = if !g_xn_zp && !g_yp_zp { n_xn } else { br(self, x - 1, y + 1, zz) };
            let c_xp_yn = if !g_xp_zp && !g_yn_zp { n_xp } else { br(self, x + 1, y - 1, zz) };
            let c_xp_yp = if !g_xp_zp && !g_yp_zp { n_xp } else { br(self, x + 1, y + 1, zz) };
            let v9 = (n_xn + c_xn_yp + ao_z_pos + n_yp) / 4.0;
            let v12 = (ao_z_pos + n_yp + n_xp + c_xp_yp) / 4.0;
            let v11 = (n_yn + ao_z_pos + c_xp_yn + n_xp) / 4.0;
            let v10 = (c_xn_yn + n_xn + n_yn + ao_z_pos) / 4.0;
            self.set_corner_colors(base(tint_w, 0.8), v9, v10, v11, v12);
            let tex = self.texture(id, x, y, z, 3);
            self.render_west_face(x as f64, y as f64, z as f64, tex);
            if self.w.fancy && tex == 3 && self.override_texture < 0 {
                self.tint_corners(r, g, b);
                self.render_west_face(x as f64, y as f64, z as f64, 38);
            }
            rendered = true;
        }

        // north (x-1)
        if self.should_side_be_rendered(id, x - 1, y, z, 4) {
            let xx = x - 1;
            let n_yn = br(self, xx, y - 1, z); // field_22376_n
            let n_zn = br(self, xx, y, z - 1); // field_22358_C
            let n_zp = br(self, xx, y, z + 1); // field_22354_E
            let n_yp = br(self, xx, y + 1, z); // field_22368_v
            let c_yn_zn = if !g_xn_zn && !g_xn_yn { n_zn } else { br(self, xx, y - 1, z - 1) };
            let c_yn_zp = if !g_xn_zp && !g_xn_yn { n_zp } else { br(self, xx, y - 1, z + 1) };
            let c_yp_zn = if !g_xn_zn && !g_xn_yp { n_zn } else { br(self, xx, y + 1, z - 1) };
            let c_yp_zp = if !g_xn_zp && !g_xn_yp { n_zp } else { br(self, xx, y + 1, z + 1) };
            let v12 = (n_yn + c_yn_zp + ao_x_neg + n_zp) / 4.0;
            let v9 = (ao_x_neg + n_zp + n_yp + c_yp_zp) / 4.0;
            let v10 = (n_zn + ao_x_neg + c_yp_zn + n_yp) / 4.0;
            let v11 = (c_yn_zn + n_yn + n_zn + ao_x_neg) / 4.0;
            self.set_corner_colors(base(tint_n, 0.6), v9, v10, v11, v12);
            let tex = self.texture(id, x, y, z, 4);
            self.render_north_face(x as f64, y as f64, z as f64, tex);
            if self.w.fancy && tex == 3 && self.override_texture < 0 {
                self.tint_corners(r, g, b);
                self.render_north_face(x as f64, y as f64, z as f64, 38);
            }
            rendered = true;
        }

        // south (x+1)
        if self.should_side_be_rendered(id, x + 1, y, z, 5) {
            let xx = x + 1;
            let n_yn = br(self, xx, y - 1, z); // field_22371_s
            let n_zn = br(self, xx, y, z - 1); // field_22356_D
            let n_zp = br(self, xx, y, z + 1); // field_22353_F
            let n_yp = br(self, xx, y + 1, z); // field_22364_z
            let c_yn_zn = if !g_xp_yn && !g_xp_zn { n_zn } else { br(self, xx, y - 1, z - 1) };
            let c_yn_zp = if !g_xp_yn && !g_xp_zp { n_zp } else { br(self, xx, y - 1, z + 1) };
            let c_yp_zn = if !g_xp_yp && !g_xp_zn { n_zn } else { br(self, xx, y + 1, z - 1) };
            let c_yp_zp = if !g_xp_yp && !g_xp_zp { n_zp } else { br(self, xx, y + 1, z + 1) };
            let v9 = (n_yn + c_yn_zp + ao_x_pos + n_zp) / 4.0;
            let v12 = (ao_x_pos + n_zp + n_yp + c_yp_zp) / 4.0;
            let v11 = (n_zn + ao_x_pos + c_yp_zn + n_yp) / 4.0;
            let v10 = (c_yn_zn + n_yn + n_zn + ao_x_pos) / 4.0;
            self.set_corner_colors(base(tint_s, 0.6), v9, v10, v11, v12);
            let tex = self.texture(id, x, y, z, 5);
            self.render_south_face(x as f64, y as f64, z as f64, tex);
            if self.w.fancy && tex == 3 && self.override_texture < 0 {
                self.tint_corners(r, g, b);
                self.render_south_face(x as f64, y as f64, z as f64, 38);
            }
            rendered = true;
        }

        self.enable_ao = false;
        rendered
    }

    // ---------------------------------------------------------------- faces

    #[inline]
    fn corner(&mut self, which: u8) {
        let (r, g, b) = match which {
            0 => (self.r_tl, self.g_tl, self.b_tl),
            1 => (self.r_bl, self.g_bl, self.b_bl),
            2 => (self.r_br, self.g_br, self.b_br),
            _ => (self.r_tr, self.g_tr, self.b_tr),
        };
        self.t.set_color_opaque_f(r, g, b);
    }

    #[inline]
    fn emit(&mut self, which: u8, x: f64, y: f64, z: f64, u: f64, v: f64) {
        if self.enable_ao {
            self.corner(which);
        }
        self.t.vertex_uv(x, y, z, u, v);
    }

    /// `overrideBlockTexture` (>= 0 replaces every face texture).
    #[inline]
    fn face_tex(&self, tex: i32) -> i32 {
        if self.override_texture >= 0 {
            self.override_texture
        } else {
            tex
        }
    }

    #[inline]
    fn full_u(tu: i32) -> (f64, f64) {
        (((tu as f32 + 0.0f32) / 256.0f32) as f64, ((tu as f32 + 15.99f32) / 256.0f32) as f64)
    }

    /// `renderBottomFace` (UV rotation `field_31082_l`).
    pub fn render_bottom_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let tex = self.face_tex(tex);
        let b = self.b;
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let (tuf, tvf) = (tu as f64, tv as f64);
        let mut v12 = (tuf + b.min_x * 16.0) / 256.0;
        let mut v14 = (tuf + b.max_x * 16.0 - 0.01) / 256.0;
        let mut v16 = (tvf + b.min_z * 16.0) / 256.0;
        let mut v18 = (tvf + b.max_z * 16.0 - 0.01) / 256.0;
        if b.min_x < 0.0 || b.max_x > 1.0 {
            (v12, v14) = Self::full_u(tu);
        }
        if b.min_z < 0.0 || b.max_z > 1.0 {
            (v16, v18) = Self::full_u(tv);
        }
        let mut v20 = v14;
        let mut v22 = v12;
        let mut v24 = v16;
        let mut v26 = v18;
        match self.uv_rotate_bottom {
            2 => {
                v12 = (tuf + b.min_z * 16.0) / 256.0;
                v16 = ((tv + 16) as f64 - b.max_x * 16.0) / 256.0;
                v14 = (tuf + b.max_z * 16.0) / 256.0;
                v18 = ((tv + 16) as f64 - b.min_x * 16.0) / 256.0;
                v24 = v16;
                v26 = v18;
                v20 = v12;
                v22 = v14;
                v16 = v18;
                v18 = v24;
            }
            1 => {
                v12 = ((tu + 16) as f64 - b.max_z * 16.0) / 256.0;
                v16 = (tvf + b.min_x * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.min_z * 16.0) / 256.0;
                v18 = (tvf + b.max_x * 16.0) / 256.0;
                v20 = v14;
                v22 = v12;
                v12 = v14;
                v14 = v22;
                v24 = v18;
                v26 = v16;
            }
            3 => {
                v12 = ((tu + 16) as f64 - b.min_x * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.max_x * 16.0 - 0.01) / 256.0;
                v16 = ((tv + 16) as f64 - b.min_z * 16.0) / 256.0;
                v18 = ((tv + 16) as f64 - b.max_z * 16.0 - 0.01) / 256.0;
                v20 = v14;
                v22 = v12;
                v24 = v16;
                v26 = v18;
            }
            _ => {}
        }
        let (x0, x1, yy, z0, z1) = (x + b.min_x, x + b.max_x, y + b.min_y, z + b.min_z, z + b.max_z);
        self.emit(0, x0, yy, z1, v22, v26);
        self.emit(1, x0, yy, z0, v12, v16);
        self.emit(2, x1, yy, z0, v20, v24);
        self.emit(3, x1, yy, z1, v14, v18);
    }

    /// `renderTopFace` (UV rotation `field_31083_k`).
    pub fn render_top_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let tex = self.face_tex(tex);
        let b = self.b;
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let (tuf, tvf) = (tu as f64, tv as f64);
        let mut v12 = (tuf + b.min_x * 16.0) / 256.0;
        let mut v14 = (tuf + b.max_x * 16.0 - 0.01) / 256.0;
        let mut v16 = (tvf + b.min_z * 16.0) / 256.0;
        let mut v18 = (tvf + b.max_z * 16.0 - 0.01) / 256.0;
        if b.min_x < 0.0 || b.max_x > 1.0 {
            (v12, v14) = Self::full_u(tu);
        }
        if b.min_z < 0.0 || b.max_z > 1.0 {
            (v16, v18) = Self::full_u(tv);
        }
        let mut v20 = v14;
        let mut v22 = v12;
        let mut v24 = v16;
        let mut v26 = v18;
        match self.uv_rotate_top {
            1 => {
                v12 = (tuf + b.min_z * 16.0) / 256.0;
                v16 = ((tv + 16) as f64 - b.max_x * 16.0) / 256.0;
                v14 = (tuf + b.max_z * 16.0) / 256.0;
                v18 = ((tv + 16) as f64 - b.min_x * 16.0) / 256.0;
                v24 = v16;
                v26 = v18;
                v20 = v12;
                v22 = v14;
                v16 = v18;
                v18 = v24;
            }
            2 => {
                v12 = ((tu + 16) as f64 - b.max_z * 16.0) / 256.0;
                v16 = (tvf + b.min_x * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.min_z * 16.0) / 256.0;
                v18 = (tvf + b.max_x * 16.0) / 256.0;
                v20 = v14;
                v22 = v12;
                v12 = v14;
                v14 = v22;
                v24 = v18;
                v26 = v16;
            }
            3 => {
                v12 = ((tu + 16) as f64 - b.min_x * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.max_x * 16.0 - 0.01) / 256.0;
                v16 = ((tv + 16) as f64 - b.min_z * 16.0) / 256.0;
                v18 = ((tv + 16) as f64 - b.max_z * 16.0 - 0.01) / 256.0;
                v20 = v14;
                v22 = v12;
                v24 = v16;
                v26 = v18;
            }
            _ => {}
        }
        let (x0, x1, yy, z0, z1) = (x + b.min_x, x + b.max_x, y + b.max_y, z + b.min_z, z + b.max_z);
        self.emit(0, x1, yy, z1, v14, v18);
        self.emit(1, x1, yy, z0, v20, v24);
        self.emit(2, x0, yy, z0, v12, v16);
        self.emit(3, x0, yy, z1, v22, v26);
    }

    /// Side-face UVs before rotation: `a` is x (east/west) or z (north/south).
    #[inline]
    fn side_uv(&self, tex: i32, a_min: f64, a_max: f64) -> (f64, f64, f64, f64) {
        let b = self.b;
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let mut v12 = (tu as f64 + a_min * 16.0) / 256.0;
        let mut v14 = (tu as f64 + a_max * 16.0 - 0.01) / 256.0;
        let mut v16 = ((tv + 16) as f64 - b.max_y * 16.0) / 256.0;
        let mut v18 = ((tv + 16) as f64 - b.min_y * 16.0 - 0.01) / 256.0;
        if self.flip_texture {
            std::mem::swap(&mut v12, &mut v14);
        }
        if a_min < 0.0 || a_max > 1.0 {
            (v12, v14) = Self::full_u(tu);
        }
        if b.min_y < 0.0 || b.max_y > 1.0 {
            (v16, v18) = Self::full_u(tv);
        }
        (v12, v14, v16, v18)
    }

    /// `renderEastFace` (the -Z side; UV rotation `field_31087_g`).
    pub fn render_east_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let tex = self.face_tex(tex);
        let b = self.b;
        let (tu, tv) = ((tex & 15) << 4, tex & 240);
        let (mut v12, mut v14, mut v16, mut v18) = self.side_uv(tex, b.min_x, b.max_x);
        let (mut v20, mut v22, mut v24, mut v26) = (v14, v12, v16, v18);
        match self.uv_rotate_east {
            2 => {
                v12 = (tu as f64 + b.min_y * 16.0) / 256.0;
                v16 = ((tv + 16) as f64 - b.min_x * 16.0) / 256.0;
                v14 = (tu as f64 + b.max_y * 16.0) / 256.0;
                v18 = ((tv + 16) as f64 - b.max_x * 16.0) / 256.0;
                v24 = v16;
                v26 = v18;
                v20 = v12;
                v22 = v14;
                v16 = v18;
                v18 = v24;
            }
            1 => {
                v12 = ((tu + 16) as f64 - b.max_y * 16.0) / 256.0;
                v16 = (tv as f64 + b.max_x * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.min_y * 16.0) / 256.0;
                v18 = (tv as f64 + b.min_x * 16.0) / 256.0;
                v20 = v14;
                v22 = v12;
                v12 = v14;
                v14 = v22;
                v24 = v18;
                v26 = v16;
            }
            3 => {
                v12 = ((tu + 16) as f64 - b.min_x * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.max_x * 16.0 - 0.01) / 256.0;
                v16 = (tv as f64 + b.max_y * 16.0) / 256.0;
                v18 = (tv as f64 + b.min_y * 16.0 - 0.01) / 256.0;
                v20 = v14;
                v22 = v12;
                v24 = v16;
                v26 = v18;
            }
            _ => {}
        }
        let (x0, x1, y0, y1, zz) = (x + b.min_x, x + b.max_x, y + b.min_y, y + b.max_y, z + b.min_z);
        self.emit(0, x0, y1, zz, v20, v24);
        self.emit(1, x1, y1, zz, v12, v16);
        self.emit(2, x1, y0, zz, v22, v26);
        self.emit(3, x0, y0, zz, v14, v18);
    }

    /// `renderWestFace` (the +Z side; UV rotation `field_31086_h`).
    pub fn render_west_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let tex = self.face_tex(tex);
        let b = self.b;
        let (tu, tv) = ((tex & 15) << 4, tex & 240);
        let (mut v12, mut v14, mut v16, mut v18) = self.side_uv(tex, b.min_x, b.max_x);
        let (mut v20, mut v22, mut v24, mut v26) = (v14, v12, v16, v18);
        match self.uv_rotate_west {
            1 => {
                v12 = (tu as f64 + b.min_y * 16.0) / 256.0;
                v18 = ((tv + 16) as f64 - b.min_x * 16.0) / 256.0;
                v14 = (tu as f64 + b.max_y * 16.0) / 256.0;
                v16 = ((tv + 16) as f64 - b.max_x * 16.0) / 256.0;
                v24 = v16;
                v26 = v18;
                v20 = v12;
                v22 = v14;
                v16 = v18;
                v18 = v24;
            }
            2 => {
                v12 = ((tu + 16) as f64 - b.max_y * 16.0) / 256.0;
                v16 = (tv as f64 + b.min_x * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.min_y * 16.0) / 256.0;
                v18 = (tv as f64 + b.max_x * 16.0) / 256.0;
                v20 = v14;
                v22 = v12;
                v12 = v14;
                v14 = v22;
                v24 = v18;
                v26 = v16;
            }
            3 => {
                v12 = ((tu + 16) as f64 - b.min_x * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.max_x * 16.0 - 0.01) / 256.0;
                v16 = (tv as f64 + b.max_y * 16.0) / 256.0;
                v18 = (tv as f64 + b.min_y * 16.0 - 0.01) / 256.0;
                v20 = v14;
                v22 = v12;
                v24 = v16;
                v26 = v18;
            }
            _ => {}
        }
        let (x0, x1, y0, y1, zz) = (x + b.min_x, x + b.max_x, y + b.min_y, y + b.max_y, z + b.max_z);
        self.emit(0, x0, y1, zz, v12, v16);
        self.emit(1, x0, y0, zz, v22, v26);
        self.emit(2, x1, y0, zz, v14, v18);
        self.emit(3, x1, y1, zz, v20, v24);
    }

    /// `renderNorthFace` (the -X side; UV rotation `field_31084_j`).
    pub fn render_north_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let tex = self.face_tex(tex);
        let b = self.b;
        let (tu, tv) = ((tex & 15) << 4, tex & 240);
        let (mut v12, mut v14, mut v16, mut v18) = self.side_uv(tex, b.min_z, b.max_z);
        let (mut v20, mut v22, mut v24, mut v26) = (v14, v12, v16, v18);
        match self.uv_rotate_north {
            1 => {
                v12 = (tu as f64 + b.min_y * 16.0) / 256.0;
                v16 = ((tv + 16) as f64 - b.max_z * 16.0) / 256.0;
                v14 = (tu as f64 + b.max_y * 16.0) / 256.0;
                v18 = ((tv + 16) as f64 - b.min_z * 16.0) / 256.0;
                v24 = v16;
                v26 = v18;
                v20 = v12;
                v22 = v14;
                v16 = v18;
                v18 = v24;
            }
            2 => {
                v12 = ((tu + 16) as f64 - b.max_y * 16.0) / 256.0;
                v16 = (tv as f64 + b.min_z * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.min_y * 16.0) / 256.0;
                v18 = (tv as f64 + b.max_z * 16.0) / 256.0;
                v20 = v14;
                v22 = v12;
                v12 = v14;
                v14 = v22;
                v24 = v18;
                v26 = v16;
            }
            3 => {
                v12 = ((tu + 16) as f64 - b.min_z * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.max_z * 16.0 - 0.01) / 256.0;
                v16 = (tv as f64 + b.max_y * 16.0) / 256.0;
                v18 = (tv as f64 + b.min_y * 16.0 - 0.01) / 256.0;
                v20 = v14;
                v22 = v12;
                v24 = v16;
                v26 = v18;
            }
            _ => {}
        }
        let (xx, y0, y1, z0, z1) = (x + b.min_x, y + b.min_y, y + b.max_y, z + b.min_z, z + b.max_z);
        self.emit(0, xx, y1, z1, v20, v24);
        self.emit(1, xx, y1, z0, v12, v16);
        self.emit(2, xx, y0, z0, v22, v26);
        self.emit(3, xx, y0, z1, v14, v18);
    }

    /// `renderSouthFace` (the +X side; UV rotation `field_31085_i`).
    pub fn render_south_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let tex = self.face_tex(tex);
        let b = self.b;
        let (tu, tv) = ((tex & 15) << 4, tex & 240);
        let (mut v12, mut v14, mut v16, mut v18) = self.side_uv(tex, b.min_z, b.max_z);
        let (mut v20, mut v22, mut v24, mut v26) = (v14, v12, v16, v18);
        match self.uv_rotate_south {
            2 => {
                v12 = (tu as f64 + b.min_y * 16.0) / 256.0;
                v16 = ((tv + 16) as f64 - b.min_z * 16.0) / 256.0;
                v14 = (tu as f64 + b.max_y * 16.0) / 256.0;
                v18 = ((tv + 16) as f64 - b.max_z * 16.0) / 256.0;
                v24 = v16;
                v26 = v18;
                v20 = v12;
                v22 = v14;
                v16 = v18;
                v18 = v24;
            }
            1 => {
                v12 = ((tu + 16) as f64 - b.max_y * 16.0) / 256.0;
                v16 = (tv as f64 + b.max_z * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.min_y * 16.0) / 256.0;
                v18 = (tv as f64 + b.min_z * 16.0) / 256.0;
                v20 = v14;
                v22 = v12;
                v12 = v14;
                v14 = v22;
                v24 = v18;
                v26 = v16;
            }
            3 => {
                v12 = ((tu + 16) as f64 - b.min_z * 16.0) / 256.0;
                v14 = ((tu + 16) as f64 - b.max_z * 16.0 - 0.01) / 256.0;
                v16 = (tv as f64 + b.max_y * 16.0) / 256.0;
                v18 = (tv as f64 + b.min_y * 16.0 - 0.01) / 256.0;
                v20 = v14;
                v22 = v12;
                v24 = v16;
                v26 = v18;
            }
            _ => {}
        }
        let (xx, y0, y1, z0, z1) = (x + b.max_x, y + b.min_y, y + b.max_y, z + b.min_z, z + b.max_z);
        self.emit(0, xx, y0, z1, v22, v26);
        self.emit(1, xx, y0, z0, v14, v18);
        self.emit(2, xx, y1, z0, v20, v24);
        self.emit(3, xx, y1, z1, v12, v16);
    }

    // ---------------------------------------------------------------- crossed plants

    /// `renderBlockReed` (flowers, tall grass, dead bushes, mushrooms, reeds, saplings).
    fn render_block_reed(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let br = self.brightness(id, x, y, z);
        let (r, g, b) = rgb(self.color_multiplier(id, x, y, z));
        self.t.set_color_opaque_f(br * r, br * g, br * b);
        let (mut px, mut py, mut pz) = (x as f64, y as f64, z as f64);
        if id == block::TALL_GRASS {
            let mut v = (x.wrapping_mul(3129871) as i64) ^ (z as i64).wrapping_mul(116129781) ^ (y as i64);
            v = v.wrapping_mul(v).wrapping_mul(42317861).wrapping_add(v.wrapping_mul(11));
            px += (((v >> 16 & 15) as f32 / 15.0f32) as f64 - 0.5) * 0.5;
            py += (((v >> 20 & 15) as f32 / 15.0f32) as f64 - 1.0) * 0.2;
            pz += (((v >> 24 & 15) as f32 / 15.0f32) as f64 - 0.5) * 0.5;
        }
        let meta = self.w.meta(x, y, z);
        self.render_crossed_squares(id, meta, px, py, pz);
        true
    }

    /// `renderCrossedSquares`
    fn render_crossed_squares(&mut self, id: u8, meta: i32, x: f64, y: f64, z: f64) {
        let tex = self.face_tex(self.texture_meta(id, 0, meta));
        let tu = (tex & 15) << 4;
        let tv = tex & 240;
        let u0 = (tu as f32 / 256.0f32) as f64;
        let u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        let v0 = (tv as f32 / 256.0f32) as f64;
        let v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
        let x0 = x + 0.5 - 0.45f32 as f64;
        let x1 = x + 0.5 + 0.45f32 as f64;
        let z0 = z + 0.5 - 0.45f32 as f64;
        let z1 = z + 0.5 + 0.45f32 as f64;
        let t = &mut *self.t;
        t.vertex_uv(x0, y + 1.0, z0, u0, v0);
        t.vertex_uv(x0, y + 0.0, z0, u0, v1);
        t.vertex_uv(x1, y + 0.0, z1, u1, v1);
        t.vertex_uv(x1, y + 1.0, z1, u1, v0);
        t.vertex_uv(x1, y + 1.0, z1, u0, v0);
        t.vertex_uv(x1, y + 0.0, z1, u0, v1);
        t.vertex_uv(x0, y + 0.0, z0, u1, v1);
        t.vertex_uv(x0, y + 1.0, z0, u1, v0);
        t.vertex_uv(x0, y + 1.0, z1, u0, v0);
        t.vertex_uv(x0, y + 0.0, z1, u0, v1);
        t.vertex_uv(x1, y + 0.0, z0, u1, v1);
        t.vertex_uv(x1, y + 1.0, z0, u1, v0);
        t.vertex_uv(x1, y + 1.0, z0, u0, v0);
        t.vertex_uv(x1, y + 0.0, z0, u0, v1);
        t.vertex_uv(x0, y + 0.0, z1, u1, v1);
        t.vertex_uv(x0, y + 1.0, z1, u1, v0);
    }

    // ---------------------------------------------------------------- cactus

    /// `renderBlockCactus` → `func_1230_b`
    fn render_block_cactus(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let (r, g, b) = rgb(self.color_multiplier(id, x, y, z));
        let mut rendered = false;
        let (s0, s1, s2, s3) = (0.5f32, 1.0f32, 0.8f32, 0.6f32);
        let inset = 1.0f32 / 16.0f32;
        let own = self.brightness(id, x, y, z);
        let bnd = self.b;
        if self.should_side_be_rendered(id, x, y - 1, z, 0) {
            let l = self.brightness(id, x, y - 1, z);
            self.t.set_color_opaque_f(s0 * r * l, s0 * g * l, s0 * b * l);
            let tex = self.texture(id, x, y, z, 0);
            self.render_bottom_face(x as f64, y as f64, z as f64, tex);
            rendered = true;
        }
        if self.should_side_be_rendered(id, x, y + 1, z, 1) {
            let mut l = self.brightness(id, x, y + 1, z);
            if bnd.max_y != 1.0 {
                l = own;
            }
            self.t.set_color_opaque_f(s1 * r * l, s1 * g * l, s1 * b * l);
            let tex = self.texture(id, x, y, z, 1);
            self.render_top_face(x as f64, y as f64, z as f64, tex);
            rendered = true;
        }
        let sides: [(i32, i32, i32, usize, f32, f32, f32, f32); 4] = [
            (0, 0, -1, 2, s2, 0.0, 0.0, inset),
            (0, 0, 1, 3, s2, 0.0, 0.0, -inset),
            (-1, 0, 0, 4, s3, inset, 0.0, 0.0),
            (1, 0, 0, 5, s3, -inset, 0.0, 0.0),
        ];
        for (dx, _dy, dz, side, shade, tx, ty, tz) in sides {
            if self.should_side_be_rendered(id, x + dx, y, z + dz, side as i32) {
                let mut l = self.brightness(id, x + dx, y, z + dz);
                let partial = match side {
                    2 => bnd.min_z > 0.0,
                    3 => bnd.max_z < 1.0,
                    4 => bnd.min_x > 0.0,
                    _ => bnd.max_x < 1.0,
                };
                if partial {
                    l = own;
                }
                self.t.set_color_opaque_f(shade * r * l, shade * g * l, shade * b * l);
                self.t.add_translation(tx, ty, tz);
                let tex = self.texture(id, x, y, z, side);
                match side {
                    2 => self.render_east_face(x as f64, y as f64, z as f64, tex),
                    3 => self.render_west_face(x as f64, y as f64, z as f64, tex),
                    4 => self.render_north_face(x as f64, y as f64, z as f64, tex),
                    _ => self.render_south_face(x as f64, y as f64, z as f64, tex),
                }
                self.t.add_translation(-tx, -ty, -tz);
                rendered = true;
            }
        }
        rendered
    }

    // ---------------------------------------------------------------- fluids

    fn effective_flow_decay(&self, fm: u8, x: i32, y: i32, z: i32) -> i32 {
        if self.w.material(x, y, z) != fm {
            -1
        } else {
            let m = self.w.meta(x, y, z);
            if m >= 8 {
                0
            } else {
                m
            }
        }
    }

    /// `BlockFluid.getIsBlockSolid`
    fn fluid_is_block_solid(&self, fm: u8, x: i32, y: i32, z: i32, side: i32) -> bool {
        let m = self.w.material(x, y, z);
        if m == fm || m == MAT_ICE {
            false
        } else if side == 1 {
            true
        } else {
            mat(m).is_solid
        }
    }

    /// `BlockFluid.getFlowVector`
    fn flow_vector(&self, fm: u8, x: i32, y: i32, z: i32) -> (f64, f64, f64) {
        let (mut vx, mut vy, mut vz) = (0.0f64, 0.0f64, 0.0f64);
        let own = self.effective_flow_decay(fm, x, y, z);
        for dir in 0..4 {
            let (nx, nz) = match dir {
                0 => (x - 1, z),
                1 => (x, z - 1),
                2 => (x + 1, z),
                _ => (x, z + 1),
            };
            let mut d = self.effective_flow_decay(fm, nx, y, nz);
            if d < 0 {
                if !mat(self.w.material(nx, y, nz)).blocks_movement {
                    d = self.effective_flow_decay(fm, nx, y - 1, nz);
                    if d >= 0 {
                        let k = d - (own - 8);
                        vx += ((nx - x) * k) as f64;
                        vy += 0.0;
                        vz += ((nz - z) * k) as f64;
                    }
                }
            } else {
                let k = d - own;
                vx += ((nx - x) * k) as f64;
                vz += ((nz - z) * k) as f64;
            }
        }
        if self.w.meta(x, y, z) >= 8 {
            let falls = self.fluid_is_block_solid(fm, x, y, z - 1, 2)
                || self.fluid_is_block_solid(fm, x, y, z + 1, 3)
                || self.fluid_is_block_solid(fm, x - 1, y, z, 4)
                || self.fluid_is_block_solid(fm, x + 1, y, z, 5)
                || self.fluid_is_block_solid(fm, x, y + 1, z - 1, 2)
                || self.fluid_is_block_solid(fm, x, y + 1, z + 1, 3)
                || self.fluid_is_block_solid(fm, x - 1, y + 1, z, 4)
                || self.fluid_is_block_solid(fm, x + 1, y + 1, z, 5);
            if falls {
                let n = normalize((vx, vy, vz));
                vx = n.0;
                vy = n.1 - 6.0;
                vz = n.2;
            }
        }
        normalize((vx, vy, vz))
    }

    /// `func_1224_a`: corner height of a fluid surface.
    fn fluid_height(&self, fm: u8, x: i32, y: i32, z: i32) -> f32 {
        let mut count = 0;
        let mut sum = 0.0f32;
        for i in 0..4 {
            let bx = x - (i & 1);
            let bz = z - (i >> 1 & 1);
            if self.w.material(bx, y + 1, bz) == fm {
                return 1.0;
            }
            let m = self.w.material(bx, y, bz);
            if m != fm {
                if !mat(m).is_solid {
                    sum += 1.0;
                    count += 1;
                }
            } else {
                let meta = self.w.meta(bx, y, bz);
                if meta >= 8 || meta == 0 {
                    sum += percent_air(meta) * 10.0;
                    count += 10;
                }
                sum += percent_air(meta);
                count += 1;
            }
        }
        1.0 - sum / count as f32
    }

    /// `renderBlockFluids`
    fn render_block_fluids(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        // Fluid top and sides ignore overrideBlockTexture in vanilla. Only
        // renderBottomFace applies the destruction texture.
        let damage_texture = std::mem::replace(&mut self.override_texture, -1);
        let (r, g, b) = rgb(self.color_multiplier(id, x, y, z));
        let top = self.should_side_be_rendered(id, x, y + 1, z, 1);
        let bottom = self.should_side_be_rendered(id, x, y - 1, z, 0);
        let sides = [
            self.should_side_be_rendered(id, x, y, z - 1, 2),
            self.should_side_be_rendered(id, x, y, z + 1, 3),
            self.should_side_be_rendered(id, x - 1, y, z, 4),
            self.should_side_be_rendered(id, x + 1, y, z, 5),
        ];
        if !top && !bottom && !sides.iter().any(|&s| s) {
            self.override_texture = damage_texture;
            return false;
        }
        let mut rendered = false;
        let (s_bottom, s_top, s_z, s_x) = (0.5f32, 1.0f32, 0.8f32, 0.6f32);
        let fm = block::material_of(id);
        let meta = self.w.meta(x, y, z);
        let h00 = self.fluid_height(fm, x, y, z);
        let h01 = self.fluid_height(fm, x, y, z + 1);
        let h11 = self.fluid_height(fm, x + 1, y, z + 1);
        let h10 = self.fluid_height(fm, x + 1, y, z);

        if top {
            rendered = true;
            let mut tex = self.texture_meta(id, 1, meta);
            let mut angle = flow_angle(self.flow_vector(fm, x, y, z)) as f32;
            if angle > -999.0 {
                tex = self.texture_meta(id, 2, meta);
            }
            let tu = (tex & 15) << 4;
            let tv = tex & 240;
            let mut cu = (tu as f64 + 8.0) / 256.0;
            let mut cv = (tv as f64 + 8.0) / 256.0;
            if angle < -999.0 {
                angle = 0.0;
            } else {
                cu = ((tu + 16) as f32 / 256.0f32) as f64;
                cv = ((tv + 16) as f32 / 256.0f32) as f64;
            }
            let s = (math::sin(angle) * 8.0f32 / 256.0f32) as f64;
            let c = (math::cos(angle) * 8.0f32 / 256.0f32) as f64;
            let l = self.brightness(id, x, y, z);
            self.t.set_color_opaque_f(s_top * l * r, s_top * l * g, s_top * l * b);
            let (fx, fy, fz) = (x as f64, y as f32, z as f64);
            self.t.vertex_uv(fx, (fy + h00) as f64, fz, cu - c - s, cv - c + s);
            self.t.vertex_uv(fx, (fy + h01) as f64, fz + 1.0, cu - c + s, cv + c + s);
            self.t.vertex_uv(fx + 1.0, (fy + h11) as f64, fz + 1.0, cu + c + s, cv + c - s);
            self.t.vertex_uv(fx + 1.0, (fy + h10) as f64, fz, cu + c - s, cv - c - s);
        }
        if bottom {
            let l = self.brightness(id, x, y - 1, z);
            self.t.set_color_opaque_f(s_bottom * l, s_bottom * l, s_bottom * l);
            let tex = if damage_texture >= 0 { damage_texture } else { self.texture_meta(id, 0, meta) };
            self.override_texture = damage_texture;
            self.render_bottom_face(x as f64, y as f64, z as f64, tex);
            self.override_texture = -1;
            rendered = true;
        }
        for (i, &draw) in sides.iter().enumerate() {
            let (nx, nz) = match i {
                0 => (x, z - 1),
                1 => (x, z + 1),
                2 => (x - 1, z),
                _ => (x + 1, z),
            };
            let tex = self.texture_meta(id, i + 2, meta);
            let tu = (tex & 15) << 4;
            let tv = tex & 240;
            if !draw {
                continue;
            }
            let (ha, hb, xa, xb, za, zb) = match i {
                0 => (h00, h10, x as f32, (x + 1) as f32, z as f32, z as f32),
                1 => (h11, h01, (x + 1) as f32, x as f32, (z + 1) as f32, (z + 1) as f32),
                2 => (h01, h00, x as f32, x as f32, (z + 1) as f32, z as f32),
                _ => (h10, h11, (x + 1) as f32, (x + 1) as f32, z as f32, (z + 1) as f32),
            };
            rendered = true;
            let u0 = ((tu + 0) as f32 / 256.0f32) as f64;
            let u1 = ((tu + 16) as f64 - 0.01) / 256.0;
            let va = ((tv as f32 + (1.0 - ha) * 16.0f32) / 256.0f32) as f64;
            let vb = ((tv as f32 + (1.0 - hb) * 16.0f32) / 256.0f32) as f64;
            let v1 = ((tv + 16) as f64 - 0.01) / 256.0;
            let mut l = self.brightness(id, nx, y, nz);
            l *= if i < 2 { s_z } else { s_x };
            self.t.set_color_opaque_f(s_top * l * r, s_top * l * g, s_top * l * b);
            let fy = y as f32;
            self.t.vertex_uv(xa as f64, (fy + ha) as f64, za as f64, u0, va);
            self.t.vertex_uv(xb as f64, (fy + hb) as f64, zb as f64, u1, vb);
            self.t.vertex_uv(xb as f64, y as f64, zb as f64, u1, v1);
            self.t.vertex_uv(xa as f64, y as f64, za as f64, u0, v1);
        }
        self.override_texture = damage_texture;
        rendered
    }

    // ---------------------------------------------------------------- chest

    /// `BlockChest.getBlockTexture` (single and double chest faces).
    fn chest_texture(&self, x: i32, y: i32, z: i32, side: i32) -> i32 {
        let base = 26;
        let me = block::CHEST;
        if side == 1 || side == 0 {
            return base - 1;
        }
        let w = self.w;
        let zn = w.block(x, y, z - 1);
        let zp = w.block(x, y, z + 1);
        let xn = w.block(x - 1, y, z);
        let xp = w.block(x + 1, y, z);
        let op = |id: u8| rprops(id).exists && block::opaque_cube_lookup(id);
        if zn != me && zp != me {
            if xn != me && xp != me {
                let mut front = 3;
                if op(zn) && !op(zp) {
                    front = 3;
                }
                if op(zp) && !op(zn) {
                    front = 2;
                }
                if op(xn) && !op(xp) {
                    front = 5;
                }
                if op(xp) && !op(xn) {
                    front = 4;
                }
                return if side == front { base + 1 } else { base };
            } else if side != 4 && side != 5 {
                let mut off = 0;
                if xn == me {
                    off = -1;
                }
                let ox = if xn == me { x - 1 } else { x + 1 };
                let a = w.block(ox, y, z - 1);
                let c = w.block(ox, y, z + 1);
                if side == 3 {
                    off = -1 - off;
                }
                let mut front = 3;
                if (op(zn) || op(a)) && !op(zp) && !op(c) {
                    front = 3;
                }
                if (op(zp) || op(c)) && !op(zn) && !op(a) {
                    front = 2;
                }
                return (if side == front { base + 16 } else { base + 32 }) + off;
            } else {
                return base;
            }
        } else if side != 2 && side != 3 {
            let mut off = 0;
            if zn == me {
                off = -1;
            }
            let oz = if zn == me { z - 1 } else { z + 1 };
            let a = w.block(x - 1, y, oz);
            let c = w.block(x + 1, y, oz);
            if side == 4 {
                off = -1 - off;
            }
            let mut front = 5;
            if (op(xn) || op(a)) && !op(xp) && !op(c) {
                front = 5;
            }
            if (op(xp) || op(c)) && !op(xn) && !op(a) {
                front = 4;
            }
            return (if side == front { base + 16 } else { base + 32 }) + off;
        }
        base
    }
}

/// `BlockFluid.getPercentAir`
fn percent_air(meta: i32) -> f32 {
    let m = if meta >= 8 { 0 } else { meta };
    (m + 1) as f32 / 9.0f32
}

/// `Vec3D.normalize` (note the float square root).
fn normalize(v: (f64, f64, f64)) -> (f64, f64, f64) {
    let len = ((v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt() as f32) as f64;
    if len < 1.0e-4 {
        (0.0, 0.0, 0.0)
    } else {
        (v.0 / len, v.1 / len, v.2 / len)
    }
}

/// `BlockFluid.func_293_a`: flow direction angle, or -1000 when still.
fn flow_angle(v: (f64, f64, f64)) -> f64 {
    if v.0 == 0.0 && v.2 == 0.0 {
        -1000.0
    } else {
        v.2.atan2(v.0) - std::f64::consts::PI * 0.5
    }
}
