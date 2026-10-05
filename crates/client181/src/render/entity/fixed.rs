//! Fixed-function renderer draw inputs. The oracle compares these before GPU transforms.
use super::living::{self, Batch, EntityVertex, Pass};
use glam::{Mat4, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct Op {
    pub code: u32,
    pub args: [f32; 4],
}
#[derive(Clone, Copy, Debug)]
pub struct RawVertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub color: u32,
    pub normal: u32,
}
#[derive(Clone, Debug)]
pub struct Draw {
    pub ops: Vec<Op>,
    pub texture: u32,
    pub material: [f32; 4],
    pub normal: [f32; 3],
    pub flags: u32,
    pub src: u32,
    pub dst: u32,
    pub depth: u32,
    pub vertices: Vec<RawVertex>,
    pub mode: u32,
    pub light_rotation: f32,
    pub light_transform: Option<Mat4>,
}
/// Flag bits, in order: lighting, rescale, normalize, texturing, blending,
/// depth-write, vertex-color, vertex-normal, depth-test, vertex-UV.
pub struct Fixed {
    pub draws: Vec<Draw>,
    pub ops: Vec<Op>,
    stack: Vec<Vec<Op>>,
    pub texture: u32,
    pub material: [f32; 4],
    pub normal: [f32; 3],
    pub flags: u32,
    pub src: u32,
    pub dst: u32,
    pub depth: u32,
    pub vertices: Vec<RawVertex>,
    pub mode: u32,
    pub vertex_color: Option<u32>,
    pub vertex_normal: Option<u32>,
    pub textured: bool,
    /// Degrees the GUI rotated the lights about X before enabling them (0 in the world).
    pub light_rotation: f32,
    pub light_transform: Option<Mat4>,
}
impl Fixed {
    pub fn new(brightness: f32) -> Self {
        Self {
            draws: vec![],
            ops: vec![],
            stack: vec![],
            texture: 0,
            material: [brightness, brightness, brightness, 1.0],
            normal: [0.0, 0.0, 1.0],
            flags: 1 | 8 | 32 | 256,
            src: 1,
            dst: 0,
            depth: 515,
            vertices: vec![],
            mode: 7,
            vertex_color: None,
            vertex_normal: None,
            textured: false,
            light_rotation: 0.0,
            light_transform: None,
        }
    }
    pub fn push(&mut self) {
        self.stack.push(self.ops.clone());
    }
    pub fn pop(&mut self) {
        self.ops = self.stack.pop().expect("matrix stack");
    }
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.ops.push(Op { code: 0, args: [x, y, z, 0.0] });
    }
    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.ops.push(Op { code: 1, args: [x, y, z, 0.0] });
    }
    pub fn rotate(&mut self, a: f32, x: f32, y: f32, z: f32) {
        self.ops.push(Op { code: 2, args: [a, x, y, z] });
    }
    pub fn enable(&mut self, bit: u32, on: bool) {
        if on {
            self.flags |= bit;
        } else {
            self.flags &= !bit;
        }
    }
    pub fn begin(&mut self) {
        self.vertices.clear();
        self.vertex_color = None;
        self.vertex_normal = None;
        self.textured = false;
    }
    pub fn set_normal(&mut self, x: f32, y: f32, z: f32) {
        self.vertex_normal = Some(living::pack_normal(Vec3::new(x, y, z)));
    }
    pub fn color(&mut self, c: [f32; 4]) {
        self.vertex_color = Some(u32::from_le_bytes(c.map(|x| ((x * 255.0) as i32).clamp(0, 255) as u8)));
    }
    pub fn vertex(&mut self, p: [f64; 3], uv: [f64; 2]) {
        self.textured = true;
        self.vertices.push(RawVertex {
            pos: p.map(|x| x as f32),
            uv: uv.map(|x| x as f32),
            color: self.vertex_color.unwrap_or(0),
            normal: self.vertex_normal.unwrap_or(0),
        });
    }
    pub fn quad(&mut self, p: [[f32; 3]; 4], uv: [[f32; 2]; 4]) {
        for i in 0..4 {
            self.vertex(p[i].map(|x| x as f64), uv[i].map(|x| x as f64));
        }
    }
    pub fn untextured(&mut self) {
        self.textured = false;
        for v in &mut self.vertices {
            v.uv = [0.0; 2];
        }
    }
    pub fn end(&mut self) {
        if self.vertices.is_empty() {
            return;
        }
        let flags = self.flags
            | if self.textured { 512 } else { 0 }
            | if self.vertex_color.is_some() { 64 } else { 0 }
            | if self.vertex_normal.is_some() { 128 } else { 0 };
        self.draws.push(Draw {
            ops: self.ops.clone(),
            texture: self.texture,
            material: self.material,
            normal: self.normal,
            flags,
            src: self.src,
            dst: self.dst,
            depth: self.depth,
            vertices: std::mem::take(&mut self.vertices),
            mode: self.mode,
            light_rotation: self.light_rotation,
            light_transform: self.light_transform,
        });
    }
}
impl Draw {
    pub fn matrix(&self) -> Mat4 {
        matrix(&self.ops)
    }
    pub fn batch(&self) -> Batch {
        let m = self.matrix();
        let inv = m.inverse();
        let nm = inv.transpose();
        let rescale = 1.0 / inv.row(2).truncate().length();
        let texture = if self.flags & 8 == 0 {
            None
        } else {
            Some(match self.texture {
                1 => "terrain.png",
                2 => "gui/items.png",
                3 => "item/arrows.png",
                4 => "misc/shadow.png",
                5 => "mob/ghast.png",
                6 => "mob/ghast_fire.png",
                7 => "mob/zombie.png",
                8 => "mob/pigzombie.png",
                9 => "mob/char.png",
                10 => "misc/water.png",
                11 => "misc/mapbg.png",
                12 => "__map",
                13 => "misc/mapicons.png",
                14 => "font/default.png",
                15 => "item/sign.png",
                16 => "art/kz.png",
                17 => "particles.png",
                18 => "item/cart.png",
                19 => "item/boat.png",
                30 => "item/xporb.png",
                20..=29 => super::player::ARMOR[(self.texture - 20) as usize],
                _ => "__white",
            })
        };
        let pass = if self.mode == 3 {
            Pass::Line
        } else if self.texture == 12 {
            Pass::UntestedAlpha
        } else if self.depth == 514 {
            Pass::Overlay
        } else if self.flags & 16 == 0 {
            Pass::Opaque
        } else if self.dst == 768 {
            Pass::Damage
        } else if self.dst == 772 {
            Pass::TntFlash
        } else if self.flags & 32 == 0 {
            Pass::Shadow
        } else {
            Pass::Alpha
        };
        let verts: Vec<_> = self
            .vertices
            .iter()
            .map(|v| {
                let mut color = if self.flags & 64 != 0 {
                    v.color.to_le_bytes().map(|x| x as f32 / 255.0)
                } else {
                    self.material
                };
                if self.flags & 1 != 0 {
                    let n = if self.flags & 128 != 0 {
                        living::normal_from_packed(v.normal)
                    } else {
                        Vec3::from_array(self.normal)
                    };
                    let mut n = nm.transform_vector3(n);
                    if self.flags & 4 != 0 {
                        n = n.normalize_or_zero();
                    } else if self.flags & 2 != 0 {
                        n *= rescale;
                    }
                    if let Some(light) = self.light_transform {
                        n = light.inverse().transform_vector3(n);
                    }
                    color = living::lit_color_rotated(n, color, self.light_rotation);
                }
                EntityVertex {
                    pos: m.transform_point3(Vec3::from_array(v.pos)).to_array(),
                    uv: v.uv,
                    color,
                }
            })
            .collect();
        let mut vertices = Vec::with_capacity(verts.len() / 4 * 6);
        for q in verts.chunks_exact(4) {
            for i in [0, 1, 2, 0, 2, 3] {
                vertices.push(q[i]);
            }
        }
        if self.mode==3 {vertices=verts;}
        Batch {
            texture,
            pass,
            vertices,
        }
    }
}

pub fn matrix(ops: &[Op]) -> Mat4 {
    let mut m = Mat4::IDENTITY;
    for o in ops {
        let [a, b, c, d] = o.args;
        m *= match o.code {
            0 => Mat4::from_translation(Vec3::new(a, b, c)),
            1 => Mat4::from_scale(Vec3::new(a, b, c)),
            2 => {
                // GL's axis-angle matrix, with the same conversion as gui::Gl.
                let r = (a as f64) * std::f64::consts::PI / 180.0;
                let (s, c) = r.sin_cos();
                let t = 1.0 - c;
                let len = ((b as f64) * (b as f64)
                    + (o.args[2] as f64) * (o.args[2] as f64)
                    + (d as f64) * (d as f64))
                    .sqrt();
                let (x, y, z) = (b as f64 / len, o.args[2] as f64 / len, d as f64 / len);
                Mat4::from_cols_array(&[
                    (x * x * t + c) as f32,
                    (y * x * t + z * s) as f32,
                    (z * x * t - y * s) as f32,
                    0.0,
                    (x * y * t - z * s) as f32,
                    (y * y * t + c) as f32,
                    (z * y * t + x * s) as f32,
                    0.0,
                    (x * z * t + y * s) as f32,
                    (y * z * t - x * s) as f32,
                    (z * z * t + c) as f32,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0,
                ])
            }
            _ => unreachable!(),
        };
    }
    m
}
