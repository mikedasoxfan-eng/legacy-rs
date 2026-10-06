//! Minecraft 1.0 additions to RenderBlocks. Keep Java's narrowing and draw order.
use super::RenderBlocks;

/// The eight slices of RenderBlocks.func_41088_a, also used by inventory rendering.
pub(crate) fn dragon_egg_boxes() -> [[f32; 6]; 8] {
    let mut depth = 0;
    [
        (2, 1),
        (3, 1),
        (4, 1),
        (5, 2),
        (6, 3),
        (7, 5),
        (6, 2),
        (3, 1),
    ]
    .map(|(radius, height)| {
        let r = radius as f32 / 16.0;
        let top = 1.0 - depth as f32 / 16.0;
        let bottom = 1.0 - (depth + height) as f32 / 16.0;
        depth += height;
        [0.5 - r, bottom, 0.5 - r, 0.5 + r, top, 0.5 + r]
    })
}

impl RenderBlocks<'_, '_> {
    pub fn clear_override_block_texture(&mut self) {
        self.override_texture = -1;
    }

    fn release_bounds(&mut self, b: [f32; 6]) {
        self.b = super::Bounds {
            min_x: b[0] as f64,
            min_y: b[1] as f64,
            min_z: b[2] as f64,
            max_x: b[3] as f64,
            max_y: b[4] as f64,
            max_z: b[5] as f64,
        };
    }

    pub(super) fn render_block_end_portal_frame(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let meta = self.w.meta(x, y, z);
        // Direction 2 deliberately leaves the previous rotation in place.
        match meta & 3 {
            0 => self.uv_rotate_top = 3,
            3 => self.uv_rotate_top = 1,
            1 => self.uv_rotate_top = 2,
            _ => {}
        }
        self.release_bounds([0.0, 0.0, 0.0, 1.0, 13.0 / 16.0, 1.0]);
        self.render_standard_block(id, x, y, z);
        if meta & 4 != 0 {
            self.override_texture = 174;
            self.release_bounds([0.25, 13.0 / 16.0, 0.25, 0.75, 1.0, 0.75]);
            self.render_standard_block(id, x, y, z);
            self.clear_override_block_texture();
        }
        self.release_bounds([0.0, 0.0, 0.0, 1.0, 13.0 / 16.0, 1.0]);
        self.uv_rotate_top = 0;
        true
    }

    pub(super) fn render_block_dragon_egg(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        for b in dragon_egg_boxes() {
            self.release_bounds(b);
            self.render_standard_block(id, x, y, z);
        }
        self.release_bounds([0.0, 0.0, 0.0, 1.0, 1.0, 1.0]);
        true
    }

    pub(super) fn render_block_brewing_stand(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        self.release_bounds([
            7.0 / 16.0,
            0.0,
            7.0 / 16.0,
            9.0 / 16.0,
            14.0 / 16.0,
            9.0 / 16.0,
        ]);
        self.render_standard_block(id, x, y, z);
        self.override_texture = 156;
        for b in [
            [
                9.0 / 16.0,
                0.0,
                5.0 / 16.0,
                15.0 / 16.0,
                2.0 / 16.0,
                11.0 / 16.0,
            ],
            [2.0 / 16.0, 0.0, 1.0 / 16.0, 0.5, 2.0 / 16.0, 7.0 / 16.0],
            [2.0 / 16.0, 0.0, 9.0 / 16.0, 0.5, 2.0 / 16.0, 15.0 / 16.0],
        ] {
            self.release_bounds(b);
            self.render_standard_block(id, x, y, z);
        }
        self.clear_override_block_texture();
        self.t.set_brightness(self.mixed(id, x, y, z));
        let (r, g, b) = self.tinted_rgb(self.color_multiplier(id, x, y, z));
        self.t.set_color_opaque_f(1.0 * r, 1.0 * g, 1.0 * b);
        let tex = self.texture_meta(id, 0, 0);
        let tu = ((tex & 15) << 4) as f32;
        let tv = (tex & 240) as f32;
        let v0 = (tv / 256.0) as f64;
        let v1 = ((tv + 15.99) / 256.0) as f64;
        let meta = self.w.meta(x, y, z);
        for arm in 0..3 {
            let angle = arm as f64 * std::f64::consts::PI * 2.0 / 3.0 + std::f64::consts::PI * 0.5;
            let (u0, u1) = if meta & (1 << arm) != 0 {
                (((tu + 7.99) / 256.0) as f64, ((tu + 0.0) / 256.0) as f64)
            } else {
                (((tu + 8.0) / 256.0) as f64, ((tu + 15.99) / 256.0) as f64)
            };
            let xa = x as f64 + 0.5;
            let xb = x as f64 + 0.5 + angle.sin() * 8.0 / 16.0;
            let za = z as f64 + 0.5;
            let zb = z as f64 + 0.5 + angle.cos() * 8.0 / 16.0;
            for (xx, yy, zz, u, v) in [
                (xa, (y + 1) as f64, za, u0, v0),
                (xa, y as f64, za, u0, v1),
                (xb, y as f64, zb, u1, v1),
                (xb, (y + 1) as f64, zb, u1, v0),
                (xb, (y + 1) as f64, zb, u1, v0),
                (xb, y as f64, zb, u1, v1),
                (xa, y as f64, za, u0, v1),
                (xa, (y + 1) as f64, za, u0, v0),
            ] {
                self.t.vertex_uv(xx, yy, zz, u, v);
            }
        }
        self.release_bounds([0.0, 0.0, 0.0, 1.0, 2.0 / 16.0, 1.0]);
        true
    }

    pub(super) fn render_block_cauldron(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        self.render_standard_block(id, x, y, z);
        self.t.set_brightness(self.mixed(id, x, y, z));
        let (r, g, b) = self.tinted_rgb(self.color_multiplier(id, x, y, z));
        self.t.set_color_opaque_f(1.0 * r, 1.0 * g, 1.0 * b);
        let inset = 2.0f32 / 16.0;
        self.render_south_face((x as f32 - 1.0 + inset) as f64, y as f64, z as f64, 154);
        self.render_north_face((x as f32 + 1.0 - inset) as f64, y as f64, z as f64, 154);
        self.render_west_face(x as f64, y as f64, (z as f32 - 1.0 + inset) as f64, 154);
        self.render_east_face(x as f64, y as f64, (z as f32 + 1.0 - inset) as f64, 154);
        self.render_top_face(x as f64, (y as f32 - 1.0 + 0.25) as f64, z as f64, 139);
        self.render_bottom_face(
            x as f64,
            (y as f32 + 1.0 - 12.0 / 16.0) as f64,
            z as f64,
            139,
        );
        let meta = self.w.meta(x, y, z);
        if meta > 0 {
            let meta = meta.min(3);
            self.render_top_face(
                x as f64,
                (y as f32 - 1.0 + (6.0 + meta as f32 * 3.0) / 16.0) as f64,
                z as f64,
                205,
            );
        }
        true
    }

    pub(super) fn render_block_lily_pad(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        let tex = self.face_tex(super::rprops(id).tex_index as i32);
        let tu = ((tex & 15) << 4) as f32;
        let tv = (tex & 240) as f32;
        let u0 = (tu / 256.0) as f64;
        let u1 = ((tu + 15.99) / 256.0) as f64;
        let v0 = (tv / 256.0) as f64;
        let v1 = ((tv + 15.99) / 256.0) as f64;
        let hash = x.wrapping_mul(3129871) as i64 ^ (z as i64).wrapping_mul(116129781) ^ y as i64;
        let hash = hash
            .wrapping_mul(hash)
            .wrapping_mul(42317861)
            .wrapping_add(hash.wrapping_mul(11));
        let rotation = (hash >> 16 & 3) as i32;
        let xx = x as f32 + 0.5;
        let zz = z as f32 + 0.5;
        let a = (rotation & 1) as f32 * 0.5 * (1 - rotation / 2 % 2 * 2) as f32;
        let b = ((rotation + 1) & 1) as f32 * 0.5 * (1 - (rotation + 1) / 2 % 2 * 2) as f32;
        let yy = (y as f32 + 0.015625) as f64;
        let vertices = [
            ((xx + a - b) as f64, (zz + a + b) as f64, u0, v0),
            ((xx + a + b) as f64, (zz - a + b) as f64, u1, v0),
            ((xx - a + b) as f64, (zz - a - b) as f64, u1, v1),
            ((xx - a - b) as f64, (zz + a - b) as f64, u0, v1),
        ];
        self.t.set_brightness(self.mixed(id, x, y, z));
        // This renderer intentionally does not apply the anaglyph conversion.
        for (color, reverse) in [(2129968u32, false), ((2129968 & 16711422) >> 1, true)] {
            self.t.set_color_opaque_i(color);
            for i in 0..4 {
                let (xx, zz, u, v) = vertices[if reverse { 3 - i } else { i }];
                self.t.vertex_uv(xx, yy, zz, u, v);
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::{access::Access, blocks::Colorizers, Tessellator};
    use std::cell::RefCell;
    use worldgen::world::chunk::{idx, Chunk};

    fn mesh(id: u8, meta: i32, ao: bool) -> Tessellator {
        let mut world = worldgen::World::new(0, 300);
        let mut chunk = Chunk::new(0, 0, Box::new([0; 32768]));
        chunk.blocks[idx(8, 64, 8)] = id;
        chunk.data.set(8, 64, 8, meta);
        // Different own and neighbouring light makes inset sampling observable.
        chunk.sky_light.set(8, 64, 8, 9);
        chunk.block_light.set(8, 64, 8, 5);
        world.chunks.insert((0, 0), Box::new(chunk));
        let biomes = RefCell::new(worldgen::BiomeSource::new(0));
        let access = Access::new(&world, 0, 0, 0, &biomes);
        let colors = Colorizers {
            grass: vec![0xffffff; 65536],
            foliage: vec![0xffffff; 65536],
        };
        let mut tess = Tessellator::new([0.0; 3]);
        tess.raw_positions = Some(Vec::new());
        let mut renderer = RenderBlocks::new(&access, &mut tess, &colors);
        renderer.ao = ao;
        assert!(renderer.render_block(id, 8, 64, 8));
        assert_eq!(renderer.override_texture, -1);
        assert_eq!(renderer.uv_rotate_top, 0);
        tess
    }

    #[test]
    fn new_render_types_and_metadata_emit_the_source_geometry() {
        assert_eq!(mesh(111, 0, true).verts.len(), 8);
        assert_eq!(mesh(118, 0, true).verts.len(), 48);
        for water in 1..=15 {
            let tess = mesh(118, water, true);
            assert_eq!(tess.verts.len(), 52);
            assert_eq!(
                tess.raw_positions.as_ref().unwrap()[48][1],
                (64.0f32 + (6.0 + water.min(3) as f32 * 3.0) / 16.0) as f64
            );
        }
        for bottles in 0..8 {
            assert_eq!(mesh(117, bottles, true).verts.len(), 120);
        }
        for direction in 0..4 {
            assert_eq!(mesh(120, direction, true).verts.len(), 24);
            assert_eq!(mesh(120, direction | 4, true).verts.len(), 48);
        }
        assert_eq!(mesh(122, 0, true).verts.len(), 192);
    }

    #[test]
    fn lily_pad_has_integer_colors_and_opposite_winding() {
        let tess = mesh(111, 0, false);
        assert_eq!(tess.verts[0].color, [32, 128, 48, 255]);
        assert_eq!(tess.verts[4].color, [16, 64, 24, 255]);
        for i in 0..4 {
            assert_eq!(
                tess.raw_positions.as_ref().unwrap()[i],
                tess.raw_positions.as_ref().unwrap()[7 - i]
            );
            assert_eq!(tess.verts[i].uv, tess.verts[7 - i].uv);
        }
        assert!(tess.verts.iter().all(|v| v.pos[1] == 64.015625));
    }

    #[test]
    fn emitting_blocks_bypass_ao_and_egg_inventory_is_one_batch() {
        for id in [117, 120, 122] {
            let a = mesh(id, 4, false);
            let b = mesh(id, 4, true);
            assert_eq!(
                bytemuck::cast_slice::<_, u8>(&a.verts),
                bytemuck::cast_slice::<_, u8>(&b.verts)
            );
        }
        let mut gl = crate::render::entity::fixed::Fixed::new(1.0);
        assert!(crate::render::entity::inventory::in_3d(122));
        crate::render::entity::inventory::inventory(&mut gl, 122, 0, 1.0, false);
        assert_eq!(gl.draws.len(), 1);
        assert_eq!(gl.draws[0].vertices.len(), 192);
    }

    #[test]
    fn fluids_are_inset_and_an_isolated_wire_is_trimmed() {
        let fluid = mesh(9, 0, false);
        let positions = fluid.raw_positions.as_ref().unwrap();
        let epsilon = 0.001f32 as f64;
        // Four air/fluid corner samples: 11 source weights and 3 air weights.
        let height = (1.0f32 - ((1.0f32 / 9.0 * 10.0 + 1.0 / 9.0) + 3.0) / 14.0) as f64;
        assert_eq!(positions[0][1], 64.0 + (height - epsilon));
        assert_eq!(positions[4][1], 64.0 + epsilon);
        assert_eq!(positions[8][2], 8.0 + epsilon);
        assert_eq!(positions[12][2], 9.0 - epsilon);
        assert_eq!(positions[16][0], 8.0 + epsilon);
        assert_eq!(positions[20][0], 9.0 - epsilon);
        let wire = mesh(55, 0, false);
        for p in wire.raw_positions.as_ref().unwrap() {
            assert!((8.3125..=8.6875).contains(&p[0]));
            assert!((8.3125..=8.6875).contains(&p[2]));
        }
    }

    #[test]
    fn inset_ao_face_uses_own_lightmap_brightness() {
        let table = mesh(116, 0, true);
        assert!(table.verts[4..8]
            .iter()
            .all(|v| v.light == (9 << 20 | 5 << 4)));
        assert!(table.verts[..4].iter().all(|v| v.light == 0));
    }
}
