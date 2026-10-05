//! The b1.8.1 lightmap (`EntityRenderer.func_35808_e`): a 16x16 texture indexed by block light
//! (x) and sky light (y) that every lit vertex samples through its brightness coordinates.

use worldgen::math;

/// `EntityRenderer`'s torch flicker state (`func_35807_d`, once per tick).
#[derive(Clone, Copy, Debug, Default)]
pub struct Flicker {
    /// field_35816_f / field_35821_h: the random walk
    pub fx: f32,
    pub fz: f32,
    /// field_35819_e / field_35817_g: what the lightmap reads
    pub x: f32,
    pub z: f32,
}

impl Flicker {
    /// `func_35807_d`, with `Math.random()` supplied by the caller (four draws per axis).
    pub fn tick(&mut self, mut random: impl FnMut() -> f64) {
        self.fx = (self.fx as f64 + (random() - random()) * random() * random()) as f32;
        self.fz = (self.fz as f64 + (random() - random()) * random() * random()) as f32;
        self.fx = (self.fx as f64 * 0.9) as f32;
        self.fz = (self.fz as f64 * 0.9) as f32;
        self.x += (self.fx - self.x) * 1.0f32;
        self.z += (self.fz - self.z) * 1.0f32;
    }
}

/// `World.func_35464_b`: the sky light factor for the lightmap.
pub fn sky_factor(celestial: f32, rain: f32, thunder: f32) -> f32 {
    let mut v = 1.0f32 - (math::cos(celestial * std::f32::consts::PI * 2.0f32) * 2.0f32 + 0.2f32);
    if v < 0.0 {
        v = 0.0;
    }
    if v > 1.0 {
        v = 1.0;
    }
    v = 1.0f32 - v;
    v = (v as f64 * (1.0 - (rain * 5.0f32) as f64 / 16.0)) as f32;
    v = (v as f64 * (1.0 - (thunder * 5.0f32) as f64 / 16.0)) as f32;
    v * 0.8f32 + 0.2f32
}

/// `func_35808_e`: ARGB lightmap texels, index `sky * 16 + block`.
/// `flash`: `World.field_27172_i > 0` (lightning lights the sky fully); `gamma`: the brightness
/// option (`field_35380_M`).
pub fn compute(table: &[f32; 16], sky: f32, flicker_x: f32, flash: bool, gamma: f32) -> [u32; 256] {
    let mut out = [0u32; 256];
    for (i, texel) in out.iter_mut().enumerate() {
        let var3 = sky * 0.95f32 + 0.05f32;
        let mut var4 = table[i / 16] * var3;
        let var5 = table[i % 16] * (flicker_x * 0.1f32 + 1.5f32);
        if flash {
            var4 = table[i / 16];
        }
        let var6 = var4 * (sky * 0.65f32 + 0.35f32);
        let var7 = var4 * (sky * 0.65f32 + 0.35f32);
        let var10 = var5 * ((var5 * 0.6f32 + 0.4f32) * 0.6f32 + 0.4f32);
        let var11 = var5 * (var5 * var5 * 0.6f32 + 0.4f32);
        let mut r = var6 + var5;
        let mut g = var7 + var10;
        let mut b = var4 + var11;
        r = r * 0.96f32 + 0.03f32;
        g = g * 0.96f32 + 0.03f32;
        b = b * 0.96f32 + 0.03f32;
        if r > 1.0 {
            r = 1.0;
        }
        if g > 1.0 {
            g = 1.0;
        }
        if b > 1.0 {
            b = 1.0;
        }
        let mut ir = 1.0f32 - r;
        let mut ig = 1.0f32 - g;
        let mut ib = 1.0f32 - b;
        ir = 1.0f32 - ir * ir * ir * ir;
        ig = 1.0f32 - ig * ig * ig * ig;
        ib = 1.0f32 - ib * ib * ib * ib;
        r = r * (1.0f32 - gamma) + ir * gamma;
        g = g * (1.0f32 - gamma) + ig * gamma;
        b = b * (1.0f32 - gamma) + ib * gamma;
        r = r * 0.96f32 + 0.03f32;
        g = g * 0.96f32 + 0.03f32;
        b = b * 0.96f32 + 0.03f32;
        r = r.clamp(0.0, 1.0);
        g = g.clamp(0.0, 1.0);
        b = b.clamp(0.0, 1.0);
        let (r, g, b) = ((r * 255.0f32) as i32, (g * 255.0f32) as i32, (b * 255.0f32) as i32);
        *texel = (255u32 << 24) | (r as u32) << 16 | (g as u32) << 8 | b as u32;
    }
    out
}

/// RGBA8 bytes of the ARGB texels, for the GPU texture.
pub fn rgba(texels: &[u32; 256]) -> Vec<u8> {
    texels.iter().flat_map(|&c| [(c >> 16) as u8, (c >> 8) as u8, c as u8, (c >> 24) as u8]).collect()
}

/// `--lightmapcheck file`: compare against `oracle.sh lightmap` (cases of table, sky, flicker,
/// flash and gamma, each followed by the 256 original texels).
pub fn check(path: &str) -> bool {
    let data = std::fs::read(path).expect("read lightmap dump");
    let mut o = 0usize;
    let mut i32_ = || {
        let v = i32::from_le_bytes(data[o..o + 4].try_into().unwrap());
        o += 4;
        v
    };
    let n = i32_();
    let mut bad = 0;
    for case in 0..n {
        let mut table = [0f32; 16];
        for t in table.iter_mut() {
            *t = f32::from_bits(i32_() as u32);
        }
        let sky = f32::from_bits(i32_() as u32);
        let flicker = f32::from_bits(i32_() as u32);
        let flash = i32_() != 0;
        let gamma = f32::from_bits(i32_() as u32);
        let celestial = f32::from_bits(i32_() as u32);
        let rain = f32::from_bits(i32_() as u32);
        let thunder = f32::from_bits(i32_() as u32);
        let java_sky = f32::from_bits(i32_() as u32);
        let rust_sky = sky_factor(celestial, rain, thunder);
        if rust_sky.to_bits() != java_sky.to_bits() {
            if bad < 5 {
                eprintln!("case {case}: sky factor java {java_sky} rust {rust_sky}");
            }
            bad += 1;
        }
        let rust = compute(&table, sky, flicker, flash, gamma);
        for (k, r) in rust.iter().enumerate() {
            let j = i32_() as u32;
            if j != *r {
                if bad < 5 {
                    eprintln!("case {case} texel {k}: java {j:08x} rust {r:08x}");
                }
                bad += 1;
            }
        }
    }
    println!("lightmap: {n} cases, {bad} differences");
    bad == 0
}
