//! The procedural `TextureFX` animations that overwrite terrain.png tiles every tick:
//! still/flowing water, still/flowing lava, fire and portal.
//! They use `Math.random()` in the original, so any RNG is equally faithful.

use worldgen::math;
use worldgen::JavaRandom;

const PI_F: f32 = std::f64::consts::PI as f32;

/// Tiny xorshift standing in for `Math.random()`.
pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Rng {
        Rng(seed | 1)
    }
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        (self.0 >> 11) as f64 / (1u64 << 53) as f64
    }
}

pub enum Kind {
    Water,
    WaterFlow { tick: i32 },
    Lava,
    LavaFlow { tick: i32 },
    Flames,
    Portal { tick: i32, frames: Vec<[u8; 1024]> },
}

pub struct TextureFx {
    pub icon: u32,
    pub tile_size: u32,
    pub image: [u8; 1024],
    kind: Kind,
    a: Vec<f32>,
    b: Vec<f32>,
    c: Vec<f32>,
    d: Vec<f32>,
}

impl TextureFx {
    fn new(icon: u32, tile_size: u32, kind: Kind, n: usize) -> TextureFx {
        TextureFx { icon, tile_size, image: [0; 1024], kind, a: vec![0.0; n], b: vec![0.0; n], c: vec![0.0; n], d: vec![0.0; n] }
    }

    /// The list `Minecraft.startGame` registers for terrain.png, in order.
    pub fn terrain_set() -> Vec<TextureFx> {
        vec![
            TextureFx::new(205, 1, Kind::Water, 256),
            TextureFx::new(237, 1, Kind::Lava, 256),
            TextureFx::new(206, 2, Kind::WaterFlow { tick: 0 }, 256),
            TextureFx::new(238, 2, Kind::LavaFlow { tick: 0 }, 256),
            TextureFx::new(14, 1, Kind::Portal { tick: 0, frames: portal_frames() }, 0),
            TextureFx::new(31, 1, Kind::Flames, 320),
            TextureFx::new(47, 1, Kind::Flames, 320),
        ]
    }

    pub fn tick(&mut self, rng: &mut Rng) {
        match &mut self.kind {
            Kind::Water => water(&mut self.a, &mut self.b, &mut self.c, &mut self.d, rng, &mut self.image, None),
            Kind::WaterFlow { tick } => {
                *tick += 1;
                let t = *tick;
                water(&mut self.a, &mut self.b, &mut self.c, &mut self.d, rng, &mut self.image, Some(t));
            }
            Kind::Lava => lava(&mut self.a, &mut self.b, &mut self.c, &mut self.d, rng, &mut self.image, None),
            Kind::LavaFlow { tick } => {
                *tick += 1;
                let t = *tick;
                lava(&mut self.a, &mut self.b, &mut self.c, &mut self.d, rng, &mut self.image, Some(t));
            }
            Kind::Flames => flames(&mut self.a, &mut self.b, rng, &mut self.image),
            Kind::Portal { tick, frames } => {
                *tick += 1;
                self.image.copy_from_slice(&frames[(*tick & 31) as usize]);
            }
        }
    }
}

/// `TextureWaterFX` (flow = None) and `TextureWaterFlowFX` (flow = Some(tick)).
fn water(g: &mut Vec<f32>, h: &mut Vec<f32>, i: &mut [f32], j: &mut [f32], rng: &mut Rng, img: &mut [u8; 1024], flow: Option<i32>) {
    for x in 0..16i32 {
        for y in 0..16i32 {
            let mut s = 0.0f32;
            if flow.is_some() {
                for yy in y - 2..=y {
                    s += g[((x & 15) + (yy & 15) * 16) as usize];
                }
                h[(x + y * 16) as usize] = s / 3.2f32 + i[(x + y * 16) as usize] * 0.8f32;
            } else {
                for xx in x - 1..=x + 1 {
                    s += g[((xx & 15) + (y & 15) * 16) as usize];
                }
                h[(x + y * 16) as usize] = s / 3.3f32 + i[(x + y * 16) as usize] * 0.8f32;
            }
        }
    }
    let (decay, chance) = if flow.is_some() { (0.3f32, 0.2) } else { (0.1f32, 0.05) };
    for k in 0..256 {
        i[k] += j[k] * 0.05f32;
        if i[k] < 0.0 {
            i[k] = 0.0;
        }
        j[k] -= decay;
        if rng.next() < chance {
            j[k] = 0.5;
        }
    }
    std::mem::swap(g, h);
    for k in 0..256i32 {
        let idx = match flow {
            Some(t) => (k - t * 16) & 255,
            None => k,
        };
        let v = g[idx as usize].clamp(0.0, 1.0);
        let v2 = v * v;
        let o = (k * 4) as usize;
        img[o] = (32.0f32 + v2 * 32.0f32) as i32 as u8;
        img[o + 1] = (50.0f32 + v2 * 64.0f32) as i32 as u8;
        img[o + 2] = 255;
        img[o + 3] = (146.0f32 + v2 * 50.0f32) as i32 as u8;
    }
}

/// `TextureLavaFX` / `TextureLavaFlowFX`.
fn lava(g: &mut Vec<f32>, h: &mut Vec<f32>, i: &mut [f32], j: &mut [f32], rng: &mut Rng, img: &mut [u8; 1024], flow: Option<i32>) {
    for x in 0..16i32 {
        for y in 0..16i32 {
            let mut s = 0.0f32;
            let oy = (math::sin(y as f32 * PI_F * 2.0f32 / 16.0f32) * 1.2f32) as i32;
            let ox = (math::sin(x as f32 * PI_F * 2.0f32 / 16.0f32) * 1.2f32) as i32;
            for xx in x - 1..=x + 1 {
                for yy in y - 1..=y + 1 {
                    s += g[((xx + oy & 15) + (yy + ox & 15) * 16) as usize];
                }
            }
            let at = |a: i32, b: i32| i[((a & 15) + (b & 15) * 16) as usize];
            h[(x + y * 16) as usize] = s / 10.0f32 + (at(x, y) + at(x + 1, y) + at(x + 1, y + 1) + at(x, y + 1)) / 4.0f32 * 0.8f32;
            let k = (x + y * 16) as usize;
            i[k] += j[k] * 0.01f32;
            if i[k] < 0.0 {
                i[k] = 0.0;
            }
            j[k] -= 0.06f32;
            if rng.next() < 0.005 {
                j[k] = 1.5;
            }
        }
    }
    std::mem::swap(g, h);
    for k in 0..256i32 {
        let idx = match flow {
            Some(t) => (k - t / 3 * 16) & 255,
            None => k,
        };
        let v = (g[idx as usize] * 2.0f32).clamp(0.0, 1.0);
        let o = (k * 4) as usize;
        img[o] = (v * 100.0f32 + 155.0f32) as i32 as u8;
        img[o + 1] = (v * v * 255.0f32) as i32 as u8;
        img[o + 2] = (v * v * v * v * 128.0f32) as i32 as u8;
        img[o + 3] = 255;
    }
}

/// `TextureFlamesFX`
fn flames(g: &mut Vec<f32>, h: &mut Vec<f32>, rng: &mut Rng, img: &mut [u8; 1024]) {
    for x in 0..16i32 {
        for y in 0..20i32 {
            let mut n = 18;
            let mut s = g[(x + (y + 1) % 20 * 16) as usize] * n as f32;
            for xx in x - 1..=x + 1 {
                for yy in y..=y + 1 {
                    if xx >= 0 && yy >= 0 && xx < 16 && yy < 20 {
                        s += g[(xx + yy * 16) as usize];
                    }
                    n += 1;
                }
            }
            h[(x + y * 16) as usize] = s / (n as f32 * 1.06f32);
            if y >= 19 {
                h[(x + y * 16) as usize] =
                    (rng.next() * rng.next() * rng.next() * 4.0 + rng.next() * 0.1f32 as f64 + 0.2f32 as f64) as f32;
            }
        }
    }
    std::mem::swap(g, h);
    for k in 0..256 {
        let v = (g[k] * 1.8f32).clamp(0.0, 1.0);
        let o = k * 4;
        img[o] = (v * 155.0f32 + 100.0f32) as i32 as u8;
        img[o + 1] = (v * v * 255.0f32) as i32 as u8;
        img[o + 2] = (v.powi(10) * 255.0f32) as i32 as u8;
        img[o + 3] = if v < 0.5 { 0 } else { 255 };
    }
}

/// `TexturePortalFX` constructor: 32 precomputed frames from `new Random(100)`.
fn portal_frames() -> Vec<[u8; 1024]> {
    let mut r = JavaRandom::new(100);
    let mut frames = vec![[0u8; 1024]; 32];
    for (f, frame) in frames.iter_mut().enumerate() {
        for x in 0..16 {
            for y in 0..16 {
                let mut v = 0.0f32;
                for k in 0..2 {
                    let off = (k * 8) as f32;
                    let mut a = (x as f32 - off) / 16.0f32 * 2.0f32;
                    let mut b = (y as f32 - off) / 16.0f32 * 2.0f32;
                    if a < -1.0 {
                        a += 2.0;
                    }
                    if a >= 1.0 {
                        a -= 2.0;
                    }
                    if b < -1.0 {
                        b += 2.0;
                    }
                    if b >= 1.0 {
                        b -= 2.0;
                    }
                    let d = a * a + b * b;
                    let mut t = (b as f64).atan2(a as f64) as f32
                        + (f as f32 / 32.0f32 * PI_F * 2.0f32 - d * 10.0f32 + (k * 2) as f32) * (k * 2 - 1) as f32;
                    t = (math::sin(t) + 1.0f32) / 2.0f32;
                    t /= d + 1.0f32;
                    v += t * 0.5f32;
                }
                v += r.next_float() * 0.1f32;
                let blue = (v * 100.0f32 + 155.0f32) as i32;
                let red = (v * v * 200.0f32 + 55.0f32) as i32;
                let green = (v * v * v * v * 255.0f32) as i32;
                let alpha = (v * 100.0f32 + 155.0f32) as i32;
                let o = (y * 16 + x) * 4;
                frame[o] = red as u8;
                frame[o + 1] = green as u8;
                frame[o + 2] = blue as u8;
                frame[o + 3] = alpha as u8;
            }
        }
    }
    frames
}
