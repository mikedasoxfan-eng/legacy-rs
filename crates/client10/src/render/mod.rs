//! Rendering: block meshing (a port of `RenderBlocks`), sky, and the GPU side.

pub mod access;
pub mod lightmap;
pub mod blocks;
pub mod camera;
pub mod pistons;
pub mod piston_check;
pub mod entity;
pub mod first_person;
pub mod first_person_check;
pub mod fx;
pub mod item_fx;
pub mod texture_check;
pub mod particles;
pub mod particle_check;
pub mod render_list;
pub mod effect_check8;
pub mod gpu;
pub mod map;
pub mod player_view;
pub mod sky;
pub mod sky_release;
pub mod sky_release_check;
pub mod potion_color;
pub mod table;
pub mod targeting;
pub mod video;
pub mod video_check;
pub mod weather;
pub mod weather_check;

/// Static per-block render data, generated from the original classes (Fancy graphics).
#[derive(Clone, Copy, Debug)]
pub struct RenderProps {
    pub exists: bool,
    pub render_type: i32,
    pub pass: u8,
    /// `isOpaqueCube()` with Fancy graphics (leaves are not opaque).
    pub opaque: bool,
    pub normal: bool,
    pub can_block_grass: bool,
    pub tex_index: u8,
    /// minX, minY, minZ, maxX, maxY, maxZ (Java doubles set from floats).
    pub bounds: [f32; 6],
    /// `getBlockTextureFromSideAndMetadata(side, meta)` as `[meta][side]`.
    pub textures: [[u8; 6]; 16],
}

impl RenderProps {
    pub const NONE: RenderProps = RenderProps {
        exists: false,
        render_type: -1,
        pass: 0,
        opaque: false,
        normal: false,
        can_block_grass: false,
        tex_index: 0,
        bounds: [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        textures: [[0; 6]; 16],
    };
}

#[inline]
pub fn rprops(id: u8) -> &'static RenderProps {
    &table::RENDER[id as usize]
}

/// One terrain vertex: position relative to the chunk origin, atlas UV, RGBA8 color, and the
/// b1.8.1 lightmap brightness (`Tessellator.setBrightness`: sky light << 20 | block light << 4).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub color: [u8; 4],
    pub light: u32,
}

/// `Tessellator`: accumulates quads with a current color and translation.
pub struct Tessellator {
    pub verts: Vec<Vertex>,
    pub raw_positions: Option<Vec<[f64;3]>>,
    color: [u8; 4],
    /// `setBrightness` value (raw[7]); written into every following vertex.
    pub brightness: u32,
    /// `xOffset`/`yOffset`/`zOffset`: starts at minus the chunk origin, like WorldRenderer's
    /// `setTranslationD`, and the cactus renderer nudges it with `setTranslationF`.
    offset: [f64; 3],
}

impl Tessellator {
    pub fn new(origin: [f64; 3]) -> Self {
        Tessellator { raw_positions:None, verts: Vec::new(), color: [255; 4], brightness: 0, offset: [-origin[0], -origin[1], -origin[2]] }
    }

    #[inline]
    fn channel(v: f32) -> u8 {
        // (int)(v * 255.0F), clamped to 0..255 by setColorRGBA.
        let i = (v * 255.0f32) as i32;
        i.clamp(0, 255) as u8
    }

    pub fn set_color_opaque_f(&mut self, r: f32, g: f32, b: f32) {
        self.color = [Self::channel(r), Self::channel(g), Self::channel(b), 255];
    }

    /// `setColorOpaque_I`: integer channels bypass float normalization.
    pub fn set_color_opaque_i(&mut self, color: u32) {
        self.color = [(color >> 16) as u8, (color >> 8) as u8, color as u8, 255];
    }

    /// `setBrightness` (func_35835_b).
    #[inline]
    pub fn set_brightness(&mut self, b: i32) {
        self.brightness = b as u32;
    }

    /// `setTranslationF` adds to the current translation (as used by the cactus renderer).
    pub fn add_translation(&mut self, x: f32, y: f32, z: f32) {
        self.offset[0] += x as f64;
        self.offset[1] += y as f64;
        self.offset[2] += z as f64;
    }

    #[inline]
    pub fn vertex_uv(&mut self, x: f64, y: f64, z: f64, u: f64, v: f64) {
        if let Some(raw)=&mut self.raw_positions { raw.push([x+self.offset[0],y+self.offset[1],z+self.offset[2]]); }
        self.verts.push(Vertex {
            pos: [(x + self.offset[0]) as f32, (y + self.offset[1]) as f32, (z + self.offset[2]) as f32],
            uv: [u as f32, v as f32],
            color: self.color,
            light: self.brightness,
        });
    }
}

/// `WorldProvider.generateLightBrightnessTable` (overworld).
pub fn brightness_table() -> [f32; 16] {
    let mut t = [0.0f32; 16];
    let min = 0.05f32;
    for (i, v) in t.iter_mut().enumerate() {
        let f = 1.0f32 - i as f32 / 15.0f32;
        *v = (1.0f32 - f) / (f * 3.0f32 + 1.0f32) * (1.0f32 - min) + min;
    }
    t
}
pub mod sign;
pub mod spawner;
pub mod painting;

pub mod fishing;

/// Original float anaglyph conversion (RenderBlocks and RenderGlobal).
pub fn stereo_color([r,g,b]: [f32; 3]) -> [f32; 3] {
    [(r * 30.0 + g * 59.0 + b * 11.0) / 100.0,
     (r * 30.0 + g * 70.0) / 100.0,
     (r * 30.0 + b * 70.0) / 100.0]
}

pub mod color_mask;
