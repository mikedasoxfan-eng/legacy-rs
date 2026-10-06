//! TextureCompassFX / TextureWatchFX. Source icons always come from the jar, even with a pack.
use super::gpu::Image;

pub struct ItemFx {
    compass: [u8; 1024],
    watch: [u8; 1024],
    dial: [u8; 1024],
    angles: [[f64; 2]; 2],
    pub images: [[u8; 1024]; 2],
}
pub const ICONS: [u32; 2] = [54, 70];
impl ItemFx {
    pub fn new(items: &Image, dial: &Image) -> Self {
        let tile = |icon: u32| {
            let mut out = [0; 1024];
            for y in 0..16 {
                let start = (((icon / 16 * 16 + y) * items.width + icon % 16 * 16) * 4) as usize;
                out[(y * 64) as usize..(y * 64 + 64) as usize].copy_from_slice(&items.rgba[start..start + 64]);
            }
            out
        };
        let mut fx = Self {
            compass: tile(ICONS[0]),
            watch: tile(ICONS[1]),
            dial: dial.rgba[..1024].try_into().unwrap(),
            angles: [[0.0; 2]; 2],
            images: [[0; 1024]; 2],
        };
        fx.tick([0.0; 2], false); // RenderEngine.registerTextureFX invokes onTick once.
        fx
    }
    pub fn tick(&mut self, target: [f64; 2], anaglyph: bool) {
        for (i, target) in target.into_iter().enumerate() {
            let a = &mut self.angles[i];
            let mut d = target - a[0];
            while d < -std::f64::consts::PI {
                d += std::f64::consts::PI * 2.0;
            }
            while d >= std::f64::consts::PI {
                d -= std::f64::consts::PI * 2.0;
            }
            d = d.clamp(-1.0, 1.0);
            a[1] += d * 0.1;
            a[1] *= 0.8;
            a[0] += a[1];
        }
        self.images[0] = self.compass;
        let sin = self.angles[0][0].sin();
        let cos = self.angles[0][0].cos();
        for i in -4..=4 {
            let x = (8.5 + cos * i as f64 * 0.3) as i32;
            let y = (7.5 - sin * i as f64 * 0.3 * 0.5) as i32;
            self.images[0][((y * 16 + x) * 4) as usize..((y * 16 + x) * 4 + 4) as usize]
                .copy_from_slice(&[100, 100, 100, 255]);
        }
        for i in -8..=16 {
            let x = (8.5 + sin * i as f64 * 0.3) as i32;
            let y = (7.5 + cos * i as f64 * 0.3 * 0.5) as i32;
            self.images[0][((y * 16 + x) * 4) as usize..((y * 16 + x) * 4 + 4) as usize].copy_from_slice(&if i >= 0 {
                [255, 20, 20, 255]
            } else {
                [100, 100, 100, 255]
            });
        }
        let sin = self.angles[1][0].sin();
        let cos = self.angles[1][0].cos();
        for i in 0..256 {
            let c = &self.watch[i * 4..i * 4 + 4];
            self.images[1][i * 4..i * 4 + 4].copy_from_slice(c);
            if c[0] == c[2] && c[1] == 0 && c[2] > 0 {
                let x = -(i as i32 % 16) as f64 / 15.0 + 0.5;
                let y = (i / 16) as f64 / 15.0 - 0.5;
                let dx = ((x * cos + y * sin + 0.5) * 16.0) as i32;
                let dy = ((y * cos - x * sin + 0.5) * 16.0) as i32;
                let d = &self.dial
                    [(((dx & 15) + (dy & 15) * 16) * 4) as usize..(((dx & 15) + (dy & 15) * 16) * 4 + 4) as usize];
                for k in 0..3 {
                    self.images[1][i * 4 + k] = (d[k] as i32 * c[0] as i32 / 255) as u8;
                }
                self.images[1][i * 4 + 3] = d[3];
            }
        }
        if anaglyph {
            for image in &mut self.images {
                crate::texture_packs::anaglyph(image);
            }
        }
    }
    pub fn targets(pos: [f64; 3], yaw: f32, spawn: (i32, i32, i32), angle: f32) -> [f64; 2] {
        [
            (yaw - 90.0) as f64 * std::f64::consts::PI / 180.0
                - (spawn.2 as f64 - pos[2]).atan2(spawn.0 as f64 - pos[0]),
            (-angle * std::f64::consts::PI as f32 * 2.0) as f64,
        ]
    }
}
