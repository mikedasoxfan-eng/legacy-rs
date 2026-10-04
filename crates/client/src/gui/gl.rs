//! Just enough fixed-function OpenGL 1.x state for the GUI code to be ported line by line:
//! matrix stacks, current color, enable flags, blend/alpha functions, texture binding and a
//! `Tessellator` that turns quads into draw commands for `gui::render`.
//!
//! Vertices are transformed by the modelview matrix here (in f32, like the GL pipeline);
//! the projection is applied on the GPU.

use glam::{Mat4, Vec3, Vec4};

/// A texture known to the GUI renderer (index into its texture table).
pub type TexId = u32;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Factor {
    Zero,
    One,
    SrcAlpha,
    OneMinusSrcAlpha,
    DstColor,
    OneMinusDstColor,
    SrcColor,
    OneMinusSrcColor,
}

/// `GL_FLAT` shading takes each quad's color from its last vertex (the provoking vertex of
/// an independent quad).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shade {
    Flat,
    Smooth,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct GuiVertex {
    /// Eye-space position (after the modelview matrix).
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
    /// x: 1 if texturing is enabled; y: alpha-test reference (GREATER), or -1 if disabled.
    pub flags: [f32; 2],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DrawState {
    pub texture: Option<TexId>,
    /// `glDepthFunc`: true = GL_GEQUAL (the achievements background), false = GL_LEQUAL.
    pub depth_gequal: bool,
    pub blend: Option<(Factor, Factor)>,
    pub depth_test: bool,
    pub depth_write: bool,
}

#[derive(Clone, Debug)]
pub struct Draw {
    pub first: u32,
    pub count: u32,
    pub state: DrawState,
    /// Projection in effect for this draw.
    pub proj: Mat4,
}

/// `Tessellator` vertex as captured between `startDrawing` and `draw`.
#[derive(Clone, Copy)]
struct TessVertex {
    pos: Vec3,
    uv: Option<[f64; 2]>,
    color: Option<u32>,
}

pub struct Gl {
    pub proj: Mat4,
    modelview: Vec<Mat4>,
    /// `glColor4f`
    pub color: [f32; 4],
    pub texture_2d: bool,
    pub blend: bool,
    pub blend_func: (Factor, Factor),
    pub alpha_test: bool,
    pub alpha_ref: f32,
    pub depth_test: bool,
    pub depth_mask: bool,
    /// `glDepthFunc(GL_GEQUAL)` instead of the usual GL_LEQUAL.
    pub depth_gequal: bool,
    pub shade: Shade,
    pub bound: Option<TexId>,
    /// Current texture coordinate (`glTexCoord`), left behind by textured draws.
    current_uv: [f32; 2],
    pub verts: Vec<GuiVertex>,
    pub draws: Vec<Draw>,
    // Tessellator
    drawing: bool,
    tess: Vec<TessVertex>,
    tess_color: Option<u32>,
    tess_uv: Option<[f64; 2]>,
    color_disabled: bool,
    /// `Tessellator.setTranslation`
    pub offset: [f64; 3],
}

impl Default for Gl {
    fn default() -> Self {
        Self::new()
    }
}

impl Gl {
    /// The state `Minecraft.startGame` leaves behind.
    pub fn new() -> Gl {
        Gl {
            proj: Mat4::IDENTITY,
            modelview: vec![Mat4::IDENTITY],
            color: [1.0; 4],
            texture_2d: true,
            blend: false,
            blend_func: (Factor::One, Factor::Zero),
            alpha_test: true,
            alpha_ref: 0.1,
            depth_test: true,
            depth_mask: true,
            depth_gequal: false,
            shade: Shade::Smooth,
            bound: None,
            current_uv: [0.0; 2],
            verts: Vec::new(),
            draws: Vec::new(),
            drawing: false,
            tess: Vec::new(),
            tess_color: None,
            tess_uv: None,
            color_disabled: false,
            offset: [0.0; 3],
        }
    }

    /// Drop recorded geometry (start of a frame); state persists like a GL context.
    pub fn begin_frame(&mut self) {
        self.verts.clear();
        self.draws.clear();
    }

    // ------------------------------------------------------------------ matrices

    fn top(&mut self) -> &mut Mat4 {
        self.modelview.last_mut().unwrap()
    }

    pub fn load_identity(&mut self) {
        *self.top() = Mat4::IDENTITY;
    }

    pub fn push_matrix(&mut self) {
        let m = *self.modelview.last().unwrap();
        self.modelview.push(m);
    }

    pub fn pop_matrix(&mut self) {
        if self.modelview.len() > 1 {
            self.modelview.pop();
        }
    }

    pub fn translatef(&mut self, x: f32, y: f32, z: f32) {
        let m = self.top();
        *m *= Mat4::from_translation(Vec3::new(x, y, z));
    }

    pub fn scalef(&mut self, x: f32, y: f32, z: f32) {
        let m = self.top();
        *m *= Mat4::from_scale(Vec3::new(x, y, z));
    }

    /// `glRotatef(angle, x, y, z)` (axis normalized, angle in degrees).
    pub fn rotatef(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        // The rotation matrix as the GL spec defines it, built in double precision (as
        // drivers do) and rounded to float.
        let (x, y, z) = (x as f64, y as f64, z as f64);
        let len = (x * x + y * y + z * z).sqrt();
        let (x, y, z) = (x / len, y / len, z / len);
        let a = (angle as f64).to_radians();
        let (s, c) = a.sin_cos();
        let t = 1.0 - c;
        let r = glam::DMat4::from_cols(
            glam::DVec4::new(x * x * t + c, y * x * t + z * s, x * z * t - y * s, 0.0),
            glam::DVec4::new(x * y * t - z * s, y * y * t + c, y * z * t + x * s, 0.0),
            glam::DVec4::new(x * z * t + y * s, y * z * t - x * s, z * z * t + c, 0.0),
            glam::DVec4::new(0.0, 0.0, 0.0, 1.0),
        )
        .as_mat4();
        let m = self.top();
        *m *= r;
    }

    /// `glOrtho` into the projection matrix (replacing it, after `glLoadIdentity`).
    pub fn ortho(&mut self, l: f64, r: f64, b: f64, t: f64, n: f64, f: f64) {
        let (l, r, b, t, n, f) = (l as f32, r as f32, b as f32, t as f32, n as f32, f as f32);
        // OpenGL clip z in [-1, 1].
        self.proj = Mat4::from_cols(
            Vec4::new(2.0 / (r - l), 0.0, 0.0, 0.0),
            Vec4::new(0.0, 2.0 / (t - b), 0.0, 0.0),
            Vec4::new(0.0, 0.0, -2.0 / (f - n), 0.0),
            Vec4::new(-(r + l) / (r - l), -(t + b) / (t - b), -(f + n) / (f - n), 1.0),
        );
    }

    // ------------------------------------------------------------------ state

    pub fn color4f(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.color = [r, g, b, a];
    }

    pub fn color3f(&mut self, r: f32, g: f32, b: f32) {
        self.color = [r, g, b, 1.0];
    }

    pub fn bind_texture(&mut self, t: TexId) {
        self.bound = Some(t);
    }

    pub fn blend_func(&mut self, s: Factor, d: Factor) {
        self.blend_func = (s, d);
    }

    // ------------------------------------------------------------------ Tessellator

    pub fn start_drawing_quads(&mut self) {
        assert!(!self.drawing, "Already tesselating!");
        self.drawing = true;
        self.tess.clear();
        self.tess_color = None;
        self.tess_uv = None;
        self.color_disabled = false;
    }

    pub fn disable_color(&mut self) {
        self.color_disabled = true;
    }

    pub fn set_color_rgba(&mut self, r: i32, g: i32, b: i32, a: i32) {
        if self.color_disabled {
            return;
        }
        let c = |v: i32| v.clamp(0, 255) as u32;
        self.tess_color = Some(c(r) | c(g) << 8 | c(b) << 16 | c(a) << 24);
    }

    pub fn set_color_opaque(&mut self, r: i32, g: i32, b: i32) {
        self.set_color_rgba(r, g, b, 255);
    }

    pub fn set_color_opaque_i(&mut self, c: i32) {
        self.set_color_opaque(c >> 16 & 255, c >> 8 & 255, c & 255);
    }

    pub fn set_color_rgba_i(&mut self, c: i32, a: i32) {
        self.set_color_rgba(c >> 16 & 255, c >> 8 & 255, c & 255, a);
    }

    pub fn set_color_rgba_f(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.set_color_rgba((r * 255.0) as i32, (g * 255.0) as i32, (b * 255.0) as i32, (a * 255.0) as i32);
    }

    pub fn set_color_opaque_f(&mut self, r: f32, g: f32, b: f32) {
        self.set_color_opaque((r * 255.0) as i32, (g * 255.0) as i32, (b * 255.0) as i32);
    }

    /// Outside `startDrawing` (as GuiMainMenu does) the color is set and then forgotten.
    pub fn add_vertex_with_uv(&mut self, x: f64, y: f64, z: f64, u: f64, v: f64) {
        self.tess_uv = Some([u, v]);
        self.add_vertex(x, y, z);
    }

    pub fn add_vertex(&mut self, x: f64, y: f64, z: f64) {
        // Tessellator stores (float)(x + xOffset).
        let p = Vec3::new((x + self.offset[0]) as f32, (y + self.offset[1]) as f32, (z + self.offset[2]) as f32);
        self.tess.push(TessVertex { pos: p, uv: self.tess_uv, color: self.tess_color });
    }

    /// `Tessellator.draw` with the current GL state.
    pub fn draw(&mut self) {
        assert!(self.drawing, "Not tesselating!");
        self.drawing = false;
        let quads = std::mem::take(&mut self.tess);
        self.submit_quads(&quads);
        self.tess = quads;
        self.tess.clear();
    }

    /// A pre-lit item mesh at the current GUI modelview and projection.
    pub fn triangles(&mut self, vertices: &[crate::render::entity::living::EntityVertex]) {
        if vertices.is_empty() {
            return;
        }
        let first = self.verts.len() as u32;
        let mv = *self.modelview.last().unwrap();
        let flags = [if self.texture_2d { 1.0 } else { 0.0 }, if self.alpha_test { self.alpha_ref } else { -1.0 }];
        self.verts.extend(vertices.iter().map(|v| GuiVertex {
            pos: mv.transform_point3(Vec3::from_array(v.pos)).to_array(),
            uv: v.uv,
            color: v.color,
            flags,
        }));
        let state = DrawState {
            texture: if self.texture_2d { self.bound } else { None },
            blend: if self.blend { Some(self.blend_func) } else { None },
            depth_test: self.depth_test,
            depth_write: self.depth_mask,
            depth_gequal: self.depth_gequal,
        };
        self.draws.push(Draw { first, count: vertices.len() as u32, state, proj: self.proj });
        if let Some(v) = vertices.last() {
            self.color = v.color;
            self.current_uv = v.uv;
        }
    }

    fn submit_quads(&mut self, q: &[TessVertex]) {
        if q.len() < 4 {
            return;
        }
        let mv = *self.modelview.last().unwrap();
        let state = DrawState {
            texture: if self.texture_2d { self.bound } else { None },
            blend: if self.blend { Some(self.blend_func) } else { None },
            depth_test: self.depth_test,
            depth_write: self.depth_mask,
            depth_gequal: self.depth_gequal,
        };
        let flags = [if self.texture_2d { 1.0 } else { 0.0 }, if self.alpha_test { self.alpha_ref } else { -1.0 }];
        let first = self.verts.len() as u32;
        let to_f = |c: u32| {
            [
                (c & 255) as f32 / 255.0,
                (c >> 8 & 255) as f32 / 255.0,
                (c >> 16 & 255) as f32 / 255.0,
                (c >> 24 & 255) as f32 / 255.0,
            ]
        };
        for quad in q.chunks_exact(4) {
            let mut colors = [[0.0f32; 4]; 4];
            let mut uvs = [[0.0f32; 2]; 4];
            for (i, v) in quad.iter().enumerate() {
                colors[i] = match v.color {
                    Some(c) => to_f(c),
                    None => self.color,
                };
                uvs[i] = match v.uv {
                    Some([u, w]) => [u as f32, w as f32],
                    None => self.current_uv,
                };
            }
            if self.shade == Shade::Flat {
                colors = [colors[3]; 4];
            }
            let mut out = [GuiVertex { pos: [0.0; 3], uv: [0.0; 2], color: [0.0; 4], flags }; 4];
            for i in 0..4 {
                let e = mv * quad[i].pos.extend(1.0);
                out[i] = GuiVertex { pos: [e.x, e.y, e.z], uv: uvs[i], color: colors[i], flags };
            }
            for &i in &[0, 1, 2, 0, 2, 3] {
                self.verts.push(out[i]);
            }
        }
        // Drivers leave the last array values as the current color / texcoord.
        if let Some(last) = q.last() {
            if let Some(c) = last.color {
                self.color = to_f(c);
            }
            if let Some([u, v]) = last.uv {
                self.current_uv = [u as f32, v as f32];
            }
        }
        let count = self.verts.len() as u32 - first;
        let proj = self.proj;
        if let Some(d) = self.draws.last_mut() {
            if d.state == state && d.proj == proj && d.first + d.count == first {
                d.count += count;
                return;
            }
        }
        self.draws.push(Draw { first, count, state, proj });
    }

    /// Draw a quad list built elsewhere (font glyphs) at the current modelview.
    pub fn quad_uv(&mut self, x0: f32, y0: f32, x1: f32, y1: f32, u0: f32, v0: f32, u1: f32, v1: f32) {
        self.start_drawing_quads();
        self.add_vertex_with_uv(x0 as f64, y1 as f64, 0.0, u0 as f64, v1 as f64);
        self.add_vertex_with_uv(x1 as f64, y1 as f64, 0.0, u1 as f64, v1 as f64);
        self.add_vertex_with_uv(x1 as f64, y0 as f64, 0.0, u1 as f64, v0 as f64);
        self.add_vertex_with_uv(x0 as f64, y0 as f64, 0.0, u0 as f64, v0 as f64);
        self.draw();
    }
}
