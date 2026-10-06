//! RenderBlocks.renderBlockOnInventory / renderBlockFallingSand.
use super::fixed::Fixed;
use crate::render::rprops;
#[derive(Clone, Copy)]
struct Bounds {
    min_x: f64,
    min_y: f64,
    min_z: f64,
    max_x: f64,
    max_y: f64,
    max_z: f64,
}

struct Faces<'a> {
    b: Bounds,
    gl: &'a mut Fixed,
    offset: [f64; 3],
}
impl Faces<'_> {
    fn emit(&mut self, _corner: usize, x: f64, y: f64, z: f64, u: f64, v: f64) {
        self.gl.vertex([x + self.offset[0], y + self.offset[1], z + self.offset[2]], [u, v]);
    }
    /// Texture rectangle shared by the horizontal faces (no UV rotation in natural terrain).
    fn uv_xz(&self, tex: i32) -> (f64, f64, f64, f64) {
        let tu = ((tex & 15) << 4) as f64;
        let tv = (tex & 240) as f64;
        let b = &self.b;
        let mut u0 = (tu + b.min_x * 16.0) / 256.0;
        let mut u1 = (tu + b.max_x * 16.0 - 0.01) / 256.0;
        let mut v0 = (tv + b.min_z * 16.0) / 256.0;
        let mut v1 = (tv + b.max_z * 16.0 - 0.01) / 256.0;
        if b.min_x < 0.0 || b.max_x > 1.0 {
            u0 = ((tu as f32 + 0.0) / 256.0f32) as f64;
            u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        }
        if b.min_z < 0.0 || b.max_z > 1.0 {
            v0 = ((tv as f32 + 0.0) / 256.0f32) as f64;
            v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
        }
        (u0, u1, v0, v1)
    }

    pub fn render_bottom_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let (u0, u1, v0, v1) = self.uv_xz(tex);
        let b = self.b;
        let (x0, x1, yy, z0, z1) = (x + b.min_x, x + b.max_x, y + b.min_y, z + b.min_z, z + b.max_z);
        self.emit(0, x0, yy, z1, u0, v1);
        self.emit(1, x0, yy, z0, u0, v0);
        self.emit(2, x1, yy, z0, u1, v0);
        self.emit(3, x1, yy, z1, u1, v1);
    }

    pub fn render_top_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let (u0, u1, v0, v1) = self.uv_xz(tex);
        let b = self.b;
        let (x0, x1, yy, z0, z1) = (x + b.min_x, x + b.max_x, y + b.max_y, z + b.min_z, z + b.max_z);
        self.emit(0, x1, yy, z1, u1, v1);
        self.emit(1, x1, yy, z0, u1, v0);
        self.emit(2, x0, yy, z0, u0, v0);
        self.emit(3, x0, yy, z1, u0, v1);
    }

    /// UVs for faces spanning (a, y): `a` is x for east/west, z for north/south.
    fn uv_side(&self, tex: i32, a_min: f64, a_max: f64) -> (f64, f64, f64, f64) {
        let tu = ((tex & 15) << 4) as f64;
        let tv = (tex & 240) as f64;
        let b = &self.b;
        let mut u0 = (tu + a_min * 16.0) / 256.0;
        let mut u1 = (tu + a_max * 16.0 - 0.01) / 256.0;
        let mut v0 = (tv + 16.0 - b.max_y * 16.0) / 256.0;
        let mut v1 = (tv + 16.0 - b.min_y * 16.0 - 0.01) / 256.0;
        if a_min < 0.0 || a_max > 1.0 {
            u0 = ((tu as f32 + 0.0) / 256.0f32) as f64;
            u1 = ((tu as f32 + 15.99f32) / 256.0f32) as f64;
        }
        if b.min_y < 0.0 || b.max_y > 1.0 {
            v0 = ((tv as f32 + 0.0) / 256.0f32) as f64;
            v1 = ((tv as f32 + 15.99f32) / 256.0f32) as f64;
        }
        (u0, u1, v0, v1)
    }

    pub fn render_east_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let b = self.b;
        let (u0, u1, v0, v1) = self.uv_side(tex, b.min_x, b.max_x);
        let (x0, x1, y0, y1, zz) = (x + b.min_x, x + b.max_x, y + b.min_y, y + b.max_y, z + b.min_z);
        self.emit(0, x0, y1, zz, u1, v0);
        self.emit(1, x1, y1, zz, u0, v0);
        self.emit(2, x1, y0, zz, u0, v1);
        self.emit(3, x0, y0, zz, u1, v1);
    }

    pub fn render_west_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let b = self.b;
        let (u0, u1, v0, v1) = self.uv_side(tex, b.min_x, b.max_x);
        let (x0, x1, y0, y1, zz) = (x + b.min_x, x + b.max_x, y + b.min_y, y + b.max_y, z + b.max_z);
        self.emit(0, x0, y1, zz, u0, v0);
        self.emit(1, x0, y0, zz, u0, v1);
        self.emit(2, x1, y0, zz, u1, v1);
        self.emit(3, x1, y1, zz, u1, v0);
    }

    pub fn render_north_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let b = self.b;
        let (u0, u1, v0, v1) = self.uv_side(tex, b.min_z, b.max_z);
        let (xx, y0, y1, z0, z1) = (x + b.min_x, y + b.min_y, y + b.max_y, z + b.min_z, z + b.max_z);
        self.emit(0, xx, y1, z1, u1, v0);
        self.emit(1, xx, y1, z0, u0, v0);
        self.emit(2, xx, y0, z0, u0, v1);
        self.emit(3, xx, y0, z1, u1, v1);
    }

    pub fn render_south_face(&mut self, x: f64, y: f64, z: f64, tex: i32) {
        let b = self.b;
        let (u0, u1, v0, v1) = self.uv_side(tex, b.min_z, b.max_z);
        let (xx, y0, y1, z0, z1) = (x + b.max_x, y + b.min_y, y + b.max_y, z + b.min_z, z + b.max_z);
        self.emit(0, xx, y0, z1, u0, v1);
        self.emit(1, xx, y0, z0, u1, v1);
        self.emit(2, xx, y1, z0, u1, v0);
        self.emit(3, xx, y1, z1, u0, v0);
    }
    fn face(&mut self, side: usize, tex: i32) {
        match side {
            0 => self.render_bottom_face(0.0, 0.0, 0.0, tex),
            1 => self.render_top_face(0.0, 0.0, 0.0, tex),
            2 => self.render_east_face(0.0, 0.0, 0.0, tex),
            3 => self.render_west_face(0.0, 0.0, 0.0, tex),
            4 => self.render_north_face(0.0, 0.0, 0.0, tex),
            5 => self.render_south_face(0.0, 0.0, 0.0, tex),
            _ => unreachable!(),
        }
    }
}
const NORMALS: [[f32; 3]; 6] =
    [[0.0, -1.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, -1.0], [0.0, 0.0, 1.0], [-1.0, 0.0, 0.0], [1.0, 0.0, 0.0]];
fn bounds(p: [f32; 6]) -> Bounds {
    Bounds {
        min_x: p[0] as f64,
        min_y: p[1] as f64,
        min_z: p[2] as f64,
        max_x: p[3] as f64,
        max_y: p[4] as f64,
        max_z: p[5] as f64,
    }
}
fn item_props(id:u8)->&'static crate::render::RenderProps {&crate::gui::screens::block_items::RENDER[id as usize]}
pub fn in_3d(id: i16) -> bool {
    (0..256).contains(&id) && matches!(item_props(id as u8).render_type, 0 | 13 | 10 | 11 | 16 | 21 | 22 | 27)
}
pub fn block_color(id: u8, meta: i32) -> u32 {
    if id == 18 {
        if meta & 1 == 1 {
            0x619961
        } else if meta & 2 == 2 {
            0x80a755
        } else {
            0x48b518
        }
    } else {
        0xffffff
    }
}
pub fn inventory(gl: &mut Fixed, id: u8, meta: i32, brightness: f32, tint: bool) {
    // RenderBlocks type 22: ChestItemRenderHelper, then GL_RESCALE_NORMAL is left enabled.
    if id==54 {crate::gui::screens::chest_item::append(gl);gl.enable(2,true);return;}
    if tint {
        let c = block_color(id, meta);
        gl.material = [
            (c >> 16 & 255) as f32 / 255.0 * brightness,
            (c >> 8 & 255) as f32 / 255.0 * brightness,
            (c & 255) as f32 / 255.0 * brightness,
            1.0,
        ];
    }
    let rt = item_props(id).render_type;
    if rt == 1 {
        let texture = item_props(id).textures[(meta & 15) as usize][0] as i32;
        let u0 = (((texture & 15) << 4) as f32 / 256.0) as f64;
        let u1 = ((((texture & 15) << 4) as f32 + 15.99) / 256.0) as f64;
        let v0 = ((texture & 240) as f32 / 256.0) as f64;
        let v1 = (((texture & 240) as f32 + 15.99) / 256.0) as f64;
        gl.begin();gl.set_normal(0.0, -1.0, 0.0);
        for (x0, z0, x1, z1) in [(-0.45, -0.45, 0.45, 0.45), (0.45, 0.45, -0.45, -0.45), (-0.45, 0.45, 0.45, -0.45), (0.45, -0.45, -0.45, 0.45)] {
            for (position, uv) in [([x0, 0.5, z0], [u0, v0]), ([x0, -0.5, z0], [u0, v1]), ([x1, -0.5, z1], [u1, v1]), ([x1, 0.5, z1], [u1, v0])] {gl.vertex(position, uv);}
        }
        gl.end();return;
    }
    if rt == 23 { return; } // Vanilla's inventory lily-pad branch emits no vertices.
    if rt == 27 {
        gl.translate(-0.5, -0.5, -0.5);
        gl.begin();
        for b in crate::render::blocks::release::dragon_egg_boxes() {
            for (side, n) in NORMALS.iter().enumerate() {
                gl.set_normal(n[0], n[1], n[2]);
                Faces { b: bounds(b), gl, offset: [0.0; 3] }.face(side, item_props(id).textures[0][side] as i32);
            }
        }
        gl.end();
        gl.translate(0.5, 0.5, 0.5);
        return;
    }
    let meta = if rt == 16 { 1 } else { meta };
    let mut boxes = vec![item_props(id).bounds];
    if rt == 10 {
        boxes = vec![[0.0, 0.0, 0.0, 1.0, 1.0, 0.5], [0.0, 0.0, 0.5, 1.0, 0.5, 1.0]];
    }
    if rt == 11 {
        boxes = vec![
            [0.375, 0.0, 0.0, 0.625, 1.0, 0.25],
            [0.375, 0.0, 0.75, 0.625, 1.0, 1.0],
            [0.4375, 0.8125, -0.125, 0.5625, 0.9375, 1.125],
            [0.4375, 0.3125, -0.125, 0.5625, 0.4375, 1.125],
        ];
    }
    if rt==21 {
        boxes=vec![[0.4375,0.3,0.0,0.5625,1.0,0.125],[0.4375,0.3,0.875,0.5625,1.0,1.0],[0.4375,0.5,0.0,0.5625,0.9375,1.0]];
    }
    if matches!(id, 70 | 72) {
        boxes = vec![[0.0, 0.375, 0.0, 1.0, 0.625, 1.0]];
    }
    if id == 77 {
        boxes = vec![[0.3125, 0.375, 0.375, 0.6875, 0.625, 0.625]];
    }
    if id == 92 {
        boxes = vec![[0.0625, 0.0, 0.0625, 0.9375, 0.5, 0.9375]];
    }
    if id == 96 {
        boxes = vec![[0.0, 0.40625, 0.0, 1.0, 0.59375, 1.0]];
    }
    if rt == 16 {
        boxes = vec![[0.0, 0.0, 0.0, 1.0, 1.0, 1.0]];
    }
    for b in boxes {
        gl.translate(-0.5, -0.5, -0.5);
        for (side, n) in NORMALS.iter().enumerate() {
            if id==2 && tint {
                let c=if side==1 {grass_color()}else {0xffffff};
                gl.material=[(c>>16&255) as f32/255.0*brightness,(c>>8&255) as f32/255.0*brightness,(c&255) as f32/255.0*brightness,1.0];
            }
            gl.begin();
            gl.set_normal(n[0], n[1], n[2]);
            let tex = item_props(id).textures[if rt == 0 || rt == 16 { (meta & 15) as usize } else { 0 }][side] as i32;
            let offset = if rt == 13 {
                match side {
                    2 => [0.0, 0.0, 0.0625],
                    3 => [0.0, 0.0, -0.0625],
                    4 => [0.0625, 0.0, 0.0],
                    5 => [-0.0625, 0.0, 0.0],
                    _ => [0.0; 3],
                }
            } else {
                [0.0; 3]
            };
            Faces { b: bounds(b), gl, offset }.face(side, tex);
            gl.end();
        }
        gl.translate(0.5, 0.5, 0.5);
    }
}
/// Brightness order: own, below, above, -z, +z, -x, +x.
pub fn falling(gl: &mut Fixed, id: u8, light: [f32; 7]) {
    gl.begin();
    for side in 0..6 {
        let f = [0.5, 1.0, 0.8, 0.8, 0.6, 0.6][side] * light[side + 1].max(light[0]);
        gl.color([f, f, f, 1.0]);
        Faces { b: bounds(rprops(id).bounds), gl, offset: [-0.5; 3] }.face(side, rprops(id).textures[0][side] as i32);
    }
    gl.end();
}

fn grass_color()->u32 {
    static COLOR:std::sync::OnceLock<u32>=std::sync::OnceLock::new();
    *COLOR.get_or_init(||{let mut jar=crate::assets::Jar::open().unwrap();let image=jar.image("misc/grasscolor.png").unwrap();let at=(127*256+127)*4;let c=&image.rgba[at..at+3];(c[0] as u32)<<16|(c[1] as u32)<<8|c[2] as u32})
}
