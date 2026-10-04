//! MapItemRenderer's palette, dynamic texture, decorations, and map caption.
use super::{entity::fixed::Fixed, gpu::Image};
#[derive(Clone)]
pub struct Data {
    pub colors: Vec<u8>,
    pub icons: Vec<[i8; 4]>,
    pub label: String,
}
impl Default for Data {
    fn default() -> Self {
        Self {
            colors: vec![0; 16384],
            icons: vec![],
            label: String::new(),
        }
    }
}
pub const PALETTE: [u32; 14] = [
    0, 8368696, 16247203, 10987431, 16711680, 10526975, 10987431, 31744, 16777215, 10791096,
    12020271, 7368816, 4210943, 6837042,
];
fn stereo([r, g, b]: [u32; 3]) -> [u32; 3] {
    [
        (r * 30 + g * 59 + b * 11) / 100,
        (r * 30 + g * 70) / 100,
        (r * 30 + b * 70) / 100,
    ]
}
pub fn pixels(data: &Data, anaglyph: bool) -> Vec<u32> {
    data.colors
        .iter()
        .enumerate()
        .map(|(i, &v)| {
            if v / 4 == 0 {
                return ((((i + i / 128) & 1) * 8 + 16) as u32) << 24;
            }
            let color = PALETTE[(v / 4) as usize];
            let shade = match v & 3 {
                0 => 180,
                2 => 255,
                _ => 220,
            };
            let mut c = [
                (color >> 16 & 255) * shade / 255,
                (color >> 8 & 255) * shade / 255,
                (color & 255) * shade / 255,
            ];
            if anaglyph {
                c = stereo(c);
            }
            0xff000000 | c[0] << 16 | c[1] << 8 | c[2]
        })
        .collect()
}
pub fn image(data: &Data, anaglyph: bool) -> Image {
    // RenderEngine.func_28150_a performs another anaglyph conversion after
    // MapItemRenderer's conversion. Preserve the double conversion.
    let rgba = pixels(data, anaglyph)
        .into_iter()
        .flat_map(|p| {
            let mut c = [p >> 16 & 255, p >> 8 & 255, p & 255];
            if anaglyph {
                c = stereo(c);
            }
            [c[0] as u8, c[1] as u8, c[2] as u8, (p >> 24) as u8]
        })
        .collect();
    Image {
        width: 128,
        height: 128,
        rgba,
    }
}
pub fn render(g: &mut Fixed, data: &Data) {
    g.texture = 12;
    g.enable(16, true);
    g.begin();
    g.quad(
        [
            [0.0, 128.0, -0.01],
            [128.0, 128.0, -0.01],
            [128.0, 0.0, -0.01],
            [0.0, 0.0, -0.01],
        ],
        [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]],
    );
    g.end();
    g.enable(16, false);
    g.texture = 13;
    for &[kind, x, y, rotation] in &data.icons {
        g.push();
        g.translate(x as f32 / 2.0 + 64.0, y as f32 / 2.0 + 64.0, -0.02);
        g.rotate((rotation as i32 * 360) as f32 / 16.0, 0.0, 0.0, 1.0);
        g.scale(4.0, 4.0, 3.0);
        g.translate(-2.0 / 16.0, 2.0 / 16.0, 0.0);
        let u = (kind as i32 % 4) as f32 / 4.0;
        let v = (kind as i32 / 4) as f32 / 4.0;
        let u1 = (kind as i32 % 4 + 1) as f32 / 4.0;
        let v1 = (kind as i32 / 4 + 1) as f32 / 4.0;
        g.begin();
        g.quad(
            [
                [-1.0, 1.0, 0.0],
                [1.0, 1.0, 0.0],
                [1.0, -1.0, 0.0],
                [-1.0, -1.0, 0.0],
            ],
            [[u, v], [u1, v], [u1, v1], [u, v1]],
        );
        g.end();
        g.pop();
    }
    g.push();
    g.translate(0.0, 0.0, -0.04);
    g.scale(1.0, 1.0, 1.0);
    caption(g, &data.label);
    g.pop();
}
fn caption(g: &mut Fixed, label: &str) {
    static WIDTHS: std::sync::OnceLock<[i32; 256]> = std::sync::OnceLock::new();
    let widths = WIDTHS.get_or_init(|| {
        let assets = crate::assets::load().expect("map font");
        let img = &assets.mobs["font/default.png"];
        std::array::from_fn(|c| {
            let mut x = 7i32;
            while x >= 0
                && (0..8).all(|y| {
                    img.rgba[((c / 16 * 8 + y) * 128 + c % 16 * 8 + x as usize) * 4 + 2] == 0
                })
            {
                x -= 1;
            }
            if c == 32 {
                x = 2;
            }
            x + 2
        })
    });
    g.texture = 14;
    g.material = [0.0, 0.0, 0.0, 1.0];
    g.push();
    g.translate(0.0, 0.0, 0.0);
    for c in label.bytes().filter(|c| (32..127).contains(c)) {
        let (u, v) = ((c % 16 * 8) as f32, (c / 16 * 8) as f32);
        let s = 7.99;
        g.begin();
        g.quad(
            [[0.0, s, 0.0], [s, s, 0.0], [s, 0.0, 0.0], [0.0, 0.0, 0.0]],
            [
                [u / 128.0, (v + s) / 128.0],
                [(u + s) / 128.0, (v + s) / 128.0],
                [(u + s) / 128.0, v / 128.0],
                [u / 128.0, v / 128.0],
            ],
        );
        g.end();
        g.translate(widths[c as usize] as f32, 0.0, 0.0);
    }
    g.pop();
}
/// Read existing saved map data without changing simulation state or map files.
pub fn saved(w: &worldgen::World, e: &worldgen::entity::Entity) -> Option<Data> {
    let item = e.player_state()?.inventory.held()?;
    if item.id != 358 {
        return None;
    }
    let label = format!("map_{}", item.damage);
    let mut data = Data {
        label: label.clone(),
        ..Default::default()
    };
    let path = w
        .storage
        .as_ref()?
        .dir
        .join("data")
        .join(format!("{label}.dat"));
    let bytes = std::fs::read(path).ok()?;
    let root = worldgen::nbt::read_gzip(&bytes).ok()?;
    let n = root.compound("data")?;
    let colors = n.bytes("colors")?;
    if colors.len() == 16384 && colors.iter().all(|v| *v < 56) {
        data.colors.copy_from_slice(colors);
    }
    let scale = n.byte("scale").clamp(0, 4) as i32;
    let px = (e.pos.x - n.int("xCenter") as f64) as f32 / (1 << scale) as f32;
    let pz = (e.pos.z - n.int("zCenter") as f64) as f32 / (1 << scale) as f32;
    let dimension = if w.dimension.is_hell_world() { -1 } else { 0 };
    if px >= -64.0 && px <= 64.0 && pz >= -64.0 && pz <= 64.0 && n.byte("dimension") == dimension {
        let rotation = (e.yaw * 16.0 / 360.0) as f64 + 0.5;
        data.icons.push([
            0,
            ((px * 2.0) as f64 + 0.5) as i32 as i8,
            ((pz * 2.0) as f64 + 0.5) as i32 as i8,
            rotation as i32 as i8,
        ]);
    }
    Some(data)
}
