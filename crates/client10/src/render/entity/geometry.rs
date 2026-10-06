//! `ModelRenderer`, `TexturedQuad`, and `PositionTextureVertex` (b1.7.3).

use glam::{DVec3, Mat4, Vec3};
use worldgen::math::PI_F32;

/// Release 1.0 TextureOffset's immutable integer pair.
#[derive(Clone, Copy, Debug)]
pub struct TextureOffset { u: i32, v: i32 }
impl TextureOffset {
    pub fn new(u: i32, v: i32) -> Self { Self { u, v } }
    pub fn coordinates(self) -> [i32; 2] { [self.u, self.v] }
}

/// Positions are Java doubles narrowed from floats by `PositionTextureVertex`.
#[derive(Clone, Copy, Debug)]
pub struct PositionTextureVertex {
    pub pos: [f64; 3],
    pub uv: [f32; 2],
}

#[derive(Clone, Debug)]
pub struct TexturedQuad {
    pub vertices: [PositionTextureVertex; 4],
    pub invert_normal: bool,
}

impl TexturedQuad {
    fn new(
        mut vertices: [PositionTextureVertex; 4],
        u0: i32,
        v0: i32,
        u1: i32,
        v1: i32,
        texture: [f32; 2],
    ) -> Self {
        // Release 1.0 retains the arithmetic with zero texel offsets.
        let u0 = u0 as f32 / texture[0] + 0.0 / texture[0];
        let u1 = u1 as f32 / texture[0] - 0.0 / texture[0];
        let v0 = v0 as f32 / texture[1] + 0.0 / texture[1];
        let v1 = v1 as f32 / texture[1] - 0.0 / texture[1];
        for (v, uv) in vertices
            .iter_mut()
            .zip([[u1, v0], [u0, v0], [u0, v1], [u1, v1]])
        {
            v.uv = uv;
        }
        Self {
            vertices,
            invert_normal: false,
        }
    }

    /// `TexturedQuad.draw`: Vec3D.subtract subtracts *this* from its argument.
    /// Vec3D.normalize uses MathHelper.sqrt_double (a float result widened back).
    pub fn normal(&self) -> Vec3 {
        let p = self.vertices.map(|v| DVec3::from_array(v.pos));
        let a = p[0] - p[1];
        let b = p[2] - p[1];
        let n = b.cross(a);
        let len = ((n.x * n.x + n.y * n.y + n.z * n.z).sqrt() as f32) as f64;
        let n = if len < 1.0e-4 { DVec3::ZERO } else { n / len };
        let n = n.as_vec3();
        if self.invert_normal {
            -n
        } else {
            n
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModelRenderer {
    pub texture_size: [f32; 2],
    pub raw_bounds: [f32; 6],
    pub corners: [PositionTextureVertex; 8],
    pub faces: Vec<TexturedQuad>,
    texture_offset: [i32; 2],
    pub rotation_point_x: f32,
    pub rotation_point_y: f32,
    pub rotation_point_z: f32,
    pub rotate_angle_x: f32,
    pub rotate_angle_y: f32,
    pub rotate_angle_z: f32,
    pub mirror: bool,
    pub show_model: bool,
    /// `field_1402_i`.
    pub hidden: bool,
    /// Attachment support; the original Beta models themselves have no children.
    pub children: Vec<ModelRenderer>,
}

impl ModelRenderer {
    pub fn new(u: i32, v: i32) -> Self {
        Self {
            texture_size: [64.0, 32.0],
            raw_bounds: [0.0; 6],
            corners: [PositionTextureVertex {
                pos: [0.0; 3],
                uv: [0.0; 2],
            }; 8],
            faces: Vec::new(),
            texture_offset: [u, v],
            rotation_point_x: 0.0,
            rotation_point_y: 0.0,
            rotation_point_z: 0.0,
            rotate_angle_x: 0.0,
            rotate_angle_y: 0.0,
            rotate_angle_z: 0.0,
            mirror: false,
            show_model: true,
            hidden: false,
            children: Vec::new(),
        }
    }

    /// `ModelRenderer.addBox`: replaces the box, including its original face order.
    #[allow(clippy::too_many_arguments)]
    pub fn add_box(
        &mut self,
        mut x: f32,
        mut y: f32,
        mut z: f32,
        w: i32,
        h: i32,
        d: i32,
        inflate: f32,
    ) {
        self.raw_bounds = [x, y, z, x + w as f32, y + h as f32, z + d as f32];
        let mut x1 = x + w as f32;
        let mut y1 = y + h as f32;
        let mut z1 = z + d as f32;
        x -= inflate;
        y -= inflate;
        z -= inflate;
        x1 += inflate;
        y1 += inflate;
        z1 += inflate;
        if self.mirror {
            std::mem::swap(&mut x, &mut x1);
        }
        let pts = [
            [x, y, z],
            [x1, y, z],
            [x1, y1, z],
            [x, y1, z],
            [x, y, z1],
            [x1, y, z1],
            [x1, y1, z1],
            [x, y1, z1],
        ];
        self.corners = std::array::from_fn(|i| PositionTextureVertex {
            // Vec3D's constructor canonicalizes negative zero.
            pos: pts[i].map(|p| if p == 0.0 { 0.0 } else { p as f64 }),
            uv: [[0.0, 0.0], [0.0, 8.0], [8.0, 8.0], [8.0, 0.0]][i % 4],
        });
        let [u, v] = self.texture_offset;
        let specs = [
            ([5, 1, 2, 6], u + d + w, v + d, u + d + w + d, v + d + h),
            ([0, 4, 7, 3], u, v + d, u + d, v + d + h),
            ([5, 4, 0, 1], u + d, v, u + d + w, v + d),
            ([2, 3, 7, 6], u + d + w, v, u + d + w + w, v + d),
            ([1, 0, 3, 2], u + d, v + d, u + d + w, v + d + h),
            (
                [4, 5, 6, 7],
                u + d + w + d,
                v + d,
                u + d + w + d + w,
                v + d + h,
            ),
        ];
        self.faces = specs
            .into_iter()
            .map(|(idx, u0, v0, u1, v1)| {
                let mut q = TexturedQuad::new(
                    idx.map(|i| self.corners[i]),
                    u0,
                    v0,
                    u1,
                    v1,
                    self.texture_size,
                );
                if self.mirror {
                    q.vertices.reverse();
                }
                q
            })
            .collect();
    }

    pub fn set_rotation_point(&mut self, x: f32, y: f32, z: f32) {
        self.rotation_point_x = x;
        self.rotation_point_y = y;
        self.rotation_point_z = z;
    }

    /// `postRender`/`render`: translate, then rotate Z, Y, X. `renderWithRotation`
    /// instead rotates Y, X, Z (used by the wolf). GL takes degrees, even though
    /// the model stores radians, so preserve the round trip through degrees.
    pub fn transform(&self, mut m: Mat4, scale: f32, with_rotation: bool) -> Mat4 {
        m *= Mat4::from_translation(Vec3::new(
            self.rotation_point_x * scale,
            self.rotation_point_y * scale,
            self.rotation_point_z * scale,
        ));
        let rad = |a: f32| (a * (180.0 / PI_F32)).to_radians();
        let x = Mat4::from_rotation_x(rad(self.rotate_angle_x));
        let y = Mat4::from_rotation_y(rad(self.rotate_angle_y));
        let z = Mat4::from_rotation_z(rad(self.rotate_angle_z));
        // Keep each GL multiplication separate; reassociation changes float rounding.
        if with_rotation {
            if self.rotate_angle_y != 0.0 {
                m *= y;
            }
            if self.rotate_angle_x != 0.0 {
                m *= x;
            }
            if self.rotate_angle_z != 0.0 {
                m *= z;
            }
        } else {
            if self.rotate_angle_z != 0.0 {
                m *= z;
            }
            if self.rotate_angle_y != 0.0 {
                m *= y;
            }
            if self.rotate_angle_x != 0.0 {
                m *= x;
            }
        }
        m
    }

    /// Applies the part transform to the current matrix for an attached item/part.
    pub fn post_render(&self, parent: Mat4, scale: f32) -> Mat4 {
        if self.hidden || !self.show_model {
            parent
        } else {
            self.transform(parent, scale, false)
        }
    }
}
