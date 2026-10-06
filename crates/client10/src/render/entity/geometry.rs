//! Release 1.0 `ModelRenderer`, `ModelBox`, and textured model geometry.

use glam::{DVec3, Mat4, Vec3};
use std::cell::RefCell;
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

/// One immutable entry in ModelRenderer.cubeList, including unexpanded bounds.
#[derive(Clone, Debug)]
pub struct ModelBox {
    pub raw_bounds: [f32; 6],
    pub corners: [PositionTextureVertex; 8],
    pub faces: Vec<TexturedQuad>,
    pub name: Option<String>,
}
impl ModelBox {
    #[allow(clippy::too_many_arguments)]
    pub fn new(texture_size: [f32; 2], texture_offset: [i32; 2], mirror: bool,
        mut x: f32, mut y: f32, mut z: f32, w: i32, h: i32, d: i32, inflate: f32) -> Self {
        let mut result = Self { raw_bounds: [0.0; 6], corners: [PositionTextureVertex {pos: [0.0; 3], uv: [0.0; 2]}; 8], faces: Vec::new(), name: None };
        result.raw_bounds = [x, y, z, x + w as f32, y + h as f32, z + d as f32];
        let mut x1 = x + w as f32;
        let mut y1 = y + h as f32;
        let mut z1 = z + d as f32;
        x -= inflate;
        y -= inflate;
        z -= inflate;
        x1 += inflate;
        y1 += inflate;
        z1 += inflate;
        if mirror {
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
        result.corners = std::array::from_fn(|i| PositionTextureVertex {
            // Vec3D's constructor canonicalizes negative zero.
            pos: pts[i].map(|p| if p == 0.0 { 0.0 } else { p as f64 }),
            uv: [[0.0, 0.0], [0.0, 8.0], [8.0, 8.0], [8.0, 0.0]][i % 4],
        });
        let [u, v] = texture_offset;
        let specs = [
            ([5, 1, 2, 6], u + d + w, v + d, u + d + w + d, v + d + h),
            ([0, 4, 7, 3], u, v + d, u + d, v + d + h),
            ([5, 4, 0, 1], u + d, v, u + d + w, v + d),
            ([2, 3, 7, 6], u + d + w, v + d, u + d + w + w, v),
            ([1, 0, 3, 2], u + d, v + d, u + d + w, v + d + h),
            (
                [4, 5, 6, 7],
                u + d + w + d,
                v + d,
                u + d + w + d + w,
                v + d + h,
            ),
        ];
        result.faces = specs
            .into_iter()
            .map(|(idx, u0, v0, u1, v1)| {
                let mut q = TexturedQuad::new(
                    idx.map(|i| result.corners[i]),
                    u0,
                    v0,
                    u1,
                    v1,
                    texture_size,
                );
                if mirror {
                    q.vertices.reverse();
                }
                q
            })
            .collect();
        result
    }
}

/// A display list freezes both geometry and scale on its first compilation.
#[derive(Clone, Debug)]
pub struct CompiledModel {
    pub scale: f32,
    pub faces: Vec<TexturedQuad>,
}

#[derive(Clone, Debug)]
pub struct ModelRenderer {
    pub cube_list: Vec<ModelBox>,
    pub box_name: Option<String>,
    compiled: RefCell<Option<CompiledModel>>,
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
            cube_list: Vec::new(),
            box_name: None,
            compiled: RefCell::new(None),
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

    pub fn named(name: impl Into<String>, texture_size: [i32; 2]) -> Self {
        let mut part = Self::new(0, 0);
        part.box_name = Some(name.into());
        part.set_texture_size(texture_size[0], texture_size[1]);
        part
    }
    pub fn set_texture_offset(&mut self, u: i32, v: i32) -> &mut Self {
        self.texture_offset = [u, v];
        self
    }
    pub fn set_texture_size(&mut self, width: i32, height: i32) -> &mut Self {
        self.texture_size = [width as f32, height as f32];
        self
    }
    pub fn add_child(&mut self, child: ModelRenderer) { self.children.push(child); }
    /// Release 1.0 appends each cube, in order, without invalidating a compiled list.
    #[allow(clippy::too_many_arguments)]
    pub fn add_box(&mut self, x: f32, y: f32, z: f32, w: i32, h: i32, d: i32, inflate: f32) -> &mut Self {
        let cube = ModelBox::new(self.texture_size, self.texture_offset, self.mirror, x, y, z, w, h, d, inflate);
        // Retain the last box bounds for legacy diagnostics; rendering uses the complete list.
        self.raw_bounds = cube.raw_bounds;
        self.corners = cube.corners;
        self.faces.extend(cube.faces.iter().cloned());
        self.cube_list.push(cube);
        self
    }
    #[allow(clippy::too_many_arguments)]
    pub fn add_named_box(&mut self, name: &str, offset: TextureOffset, x: f32, y: f32, z: f32, w: i32, h: i32, d: i32) -> &mut Self {
        self.set_texture_offset(offset.u, offset.v).add_box(x, y, z, w, h, d, 0.0);
        self.cube_list.last_mut().unwrap().name = Some(format!("{}.{}", self.box_name.as_deref().unwrap_or("null"), name));
        self
    }
    pub fn compiled(&self, scale: f32) -> std::cell::Ref<'_, CompiledModel> {
        if self.compiled.borrow().is_none() {
            *self.compiled.borrow_mut() = Some(CompiledModel { scale, faces: self.cube_list.iter().flat_map(|b| b.faces.iter().cloned()).collect() });
        }
        std::cell::Ref::map(self.compiled.borrow(), |list| list.as_ref().unwrap())
    }

    pub fn set_rotation_point(&mut self, x: f32, y: f32, z: f32) {
        self.rotation_point_x = x;
        self.rotation_point_y = y;
        self.rotation_point_z = z;
    }

    /// Literal ModelRenderer GL inputs. `mode`: render=0, renderWithRotation=1,
    /// postRender=2. Only render visits children.
    pub fn render_fixed(&self, g: &mut super::fixed::Fixed, scale: f32, mode: u8) {
        self.render_fixed_children(g, scale, mode, |g| {for child in &self.children {child.render_fixed(g, scale, 0);}});
    }
    pub fn render_fixed_children(&self, g: &mut super::fixed::Fixed, scale: f32, mode: u8, mut children: impl FnMut(&mut super::fixed::Fixed)) {
        if self.hidden || !self.show_model { return; }
        let list = self.compiled(scale);
        let rotated = self.rotate_angle_x != 0.0 || self.rotate_angle_y != 0.0 || self.rotate_angle_z != 0.0;
        let moved = self.rotation_point_x != 0.0 || self.rotation_point_y != 0.0 || self.rotation_point_z != 0.0;
        if mode == 1 || (rotated && mode == 0) { g.push(); }
        if rotated || moved || mode == 1 {
            g.translate(self.rotation_point_x * scale, self.rotation_point_y * scale, self.rotation_point_z * scale);
        }
        let axes = if mode == 1 {
            [(self.rotate_angle_y, [0.0, 1.0, 0.0]), (self.rotate_angle_x, [1.0, 0.0, 0.0]), (self.rotate_angle_z, [0.0, 0.0, 1.0])]
        } else {
            [(self.rotate_angle_z, [0.0, 0.0, 1.0]), (self.rotate_angle_y, [0.0, 1.0, 0.0]), (self.rotate_angle_x, [1.0, 0.0, 0.0])]
        };
        for (a, axis) in axes { if a != 0.0 { g.rotate(a * (180.0 / PI_F32), axis[0], axis[1], axis[2]); } }
        if mode == 2 { return; }
        for face in &list.faces {
            g.begin();
            let n = face.normal();
            g.set_normal(n.x, n.y, n.z);
            for v in &face.vertices {
                g.vertex(v.pos.map(|x| (x as f32 * list.scale) as f64), v.uv.map(|x| x as f64));
            }
            g.end();
        }
        if mode == 0 { children(g); }
        if mode == 1 || rotated { g.pop(); }
        else if moved { g.translate(-self.rotation_point_x * scale, -self.rotation_point_y * scale, -self.rotation_point_z * scale); }
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
            self.compiled(scale);
            self.transform(parent, scale, false)
        }
    }
}
