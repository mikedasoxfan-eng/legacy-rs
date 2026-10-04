// RenderBlocks.renderBlockLever; destruction keeps its override throughout.
impl<'a, 'b> RenderBlocks<'a, 'b> {
    fn damage_lever(&mut self, id: u8, x: i32, y: i32, z: i32) -> bool {
        use worldgen::entity::Vec3D;
        let meta = self.w.meta(x, y, z);
        let direction = meta & 7;
        let a = 0.25f32;
        let b = 3.0f32 / 16.0;
        let c = 3.0f32 / 16.0;
        let bounds = match direction {
            5 => Some([0.5 - b, 0.0, 0.5 - a, 0.5 + b, c, 0.5 + a]),
            6 => Some([0.5 - a, 0.0, 0.5 - b, 0.5 + a, c, 0.5 + b]),
            4 => Some([0.5 - b, 0.5 - a, 1.0 - c, 0.5 + b, 0.5 + a, 1.0]),
            3 => Some([0.5 - b, 0.5 - a, 0.0, 0.5 + b, 0.5 + a, c]),
            2 => Some([1.0 - c, 0.5 - a, 0.5 - b, 1.0, 0.5 + a, 0.5 + b]),
            1 => Some([0.0, 0.5 - a, 0.5 - b, c, 0.5 + a, 0.5 + b]),
            _ => None,
        };
        if let Some(b) = bounds {
            self.damage_bounds(b);
        }
        self.render_standard_block(id, x, y, z);
        let texture = self.override_texture.unwrap();
        let tx = (texture & 15) << 4;
        let ty = texture & 240;
        let mut u0 = tx as f32 / 256.0;
        let mut u1 = (tx as f32 + 15.99) / 256.0;
        let mut v0 = ty as f32 / 256.0;
        let mut v1 = (ty as f32 + 15.99) / 256.0;
        let a = 1.0f32 / 16.0;
        let b = 1.0f32 / 16.0;
        let h = 10.0f32 / 16.0;
        let mut vertices = [
            [-a, 0.0, -b],
            [a, 0.0, -b],
            [a, 0.0, b],
            [-a, 0.0, b],
            [-a, h, -b],
            [a, h, -b],
            [a, h, b],
            [-a, h, b],
        ]
        .map(|v| Vec3D::helper(v[0] as f64, v[1] as f64, v[2] as f64));
        for v in &mut vertices {
            if meta & 8 > 0 {
                v.z -= 1.0 / 16.0;
                v.rotate_x(worldgen::math::PI_F32 * 2.0 / 9.0);
            } else {
                v.z += 1.0 / 16.0;
                v.rotate_x(-(worldgen::math::PI_F32 * 2.0 / 9.0));
            }
            if direction == 6 {
                v.rotate_y(worldgen::math::PI_F32 * 0.5);
            }
            if direction < 5 {
                v.y -= 0.375;
                v.rotate_x(worldgen::math::PI_F32 * 0.5);
                match direction {
                    4 => v.rotate_y(0.0),
                    3 => v.rotate_y(worldgen::math::PI_F32),
                    2 => v.rotate_y(worldgen::math::PI_F32 * 0.5),
                    1 => v.rotate_y(worldgen::math::PI_F32 * -0.5),
                    _ => {}
                }
                v.x += x as f64 + 0.5;
                v.y += (y as f32 + 0.5) as f64;
                v.z += z as f64 + 0.5;
            } else {
                v.x += x as f64 + 0.5;
                v.y += (y as f32 + 2.0 / 16.0) as f64;
                v.z += z as f64 + 0.5;
            }
        }
        for (face, indices) in [
            [0, 1, 2, 3],
            [7, 6, 5, 4],
            [1, 0, 4, 5],
            [2, 1, 5, 6],
            [3, 2, 6, 7],
            [0, 3, 7, 4],
        ]
        .into_iter()
        .enumerate()
        {
            if face == 0 || face == 2 {
                u0 = (tx + 7) as f32 / 256.0;
                u1 = ((tx + 9) as f32 - 0.01) / 256.0;
                v0 = (ty + 6) as f32 / 256.0;
                v1 = ((ty + if face == 0 { 8 } else { 16 }) as f32 - 0.01) / 256.0;
            }
            for (i, uv) in indices
                .into_iter()
                .zip([[u0, v1], [u1, v1], [u1, v0], [u0, v0]])
            {
                let v = vertices[i];
                self.t.vertex_uv(v.x, v.y, v.z, uv[0] as f64, uv[1] as f64);
            }
        }
        true
    }
}
