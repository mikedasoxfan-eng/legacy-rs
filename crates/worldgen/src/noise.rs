//! The two noise families Beta 1.7.3 uses, ported literally (including the
//! y-lattice caching quirk in `PerlinNoise::add_region`, which changes output).
//! Do not "simplify" arithmetic here: operand order is what keeps results bit-exact.

use crate::random::JavaRandom;

/// `NoiseGeneratorPerlin`: improved Perlin noise with a random origin offset.
pub struct PerlinNoise {
    perm: [i32; 512],
    pub x_coord: f64,
    pub y_coord: f64,
    pub z_coord: f64,
}

#[inline]
fn lerp(t: f64, a: f64, b: f64) -> f64 {
    a + t * (b - a)
}

#[inline]
fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

/// Java `(int)d` followed by the "floor if below" fixup used throughout the noise code.
#[inline]
fn floor_i(d: f64) -> i32 {
    let i = d as i32;
    if d < i as f64 {
        i - 1
    } else {
        i
    }
}

#[inline]
fn grad(hash: i32, x: f64, y: f64, z: f64) -> f64 {
    let h = hash & 15;
    let u = if h < 8 { x } else { y };
    let v = if h < 4 {
        y
    } else if h != 12 && h != 14 {
        z
    } else {
        x
    };
    (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
}

/// `func_4110_a`: the 2D gradient variant used for one corner in the flat path.
#[inline]
fn grad2(hash: i32, x: f64, z: f64) -> f64 {
    let h = hash & 15;
    let u = (1 - ((h & 8) >> 3)) as f64 * x;
    let v = if h < 4 {
        0.0
    } else if h != 12 && h != 14 {
        z
    } else {
        x
    };
    (if h & 1 == 0 { u } else { -u }) + (if h & 2 == 0 { v } else { -v })
}

fn shuffled_perm(rand: &mut JavaRandom) -> [i32; 512] {
    let mut perm = [0i32; 512];
    for (i, p) in perm.iter_mut().enumerate().take(256) {
        *p = i as i32;
    }
    for i in 0..256usize {
        let j = (rand.next_int(256 - i as i32) + i as i32) as usize;
        perm.swap(i, j);
        perm[i + 256] = perm[i];
    }
    perm
}

impl PerlinNoise {
    pub fn new(rand: &mut JavaRandom) -> Self {
        let x_coord = rand.next_double() * 256.0;
        let y_coord = rand.next_double() * 256.0;
        let z_coord = rand.next_double() * 256.0;
        let perm = shuffled_perm(rand);
        PerlinNoise { perm, x_coord, y_coord, z_coord }
    }

    #[inline]
    fn p(&self, i: i32) -> i32 {
        self.perm[i as usize]
    }

    /// `generateNoise(x, y, z)`
    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut x = x + self.x_coord;
        let mut y = y + self.y_coord;
        let mut z = z + self.z_coord;
        let xi = floor_i(x);
        let yi = floor_i(y);
        let zi = floor_i(z);
        let xc = xi & 255;
        let yc = yi & 255;
        let zc = zi & 255;
        x -= xi as f64;
        y -= yi as f64;
        z -= zi as f64;
        let u = fade(x);
        let v = fade(y);
        let w = fade(z);
        let a = self.p(xc) + yc;
        let aa = self.p(a) + zc;
        let ab = self.p(a + 1) + zc;
        let b = self.p(xc + 1) + yc;
        let ba = self.p(b) + zc;
        let bb = self.p(b + 1) + zc;
        lerp(
            w,
            lerp(
                v,
                lerp(u, grad(self.p(aa), x, y, z), grad(self.p(ba), x - 1.0, y, z)),
                lerp(u, grad(self.p(ab), x, y - 1.0, z), grad(self.p(bb), x - 1.0, y - 1.0, z)),
            ),
            lerp(
                v,
                lerp(u, grad(self.p(aa + 1), x, y, z - 1.0), grad(self.p(ba + 1), x - 1.0, y, z - 1.0)),
                lerp(u, grad(self.p(ab + 1), x, y - 1.0, z - 1.0), grad(self.p(bb + 1), x - 1.0, y - 1.0, z - 1.0)),
            ),
        )
    }

    /// `func_801_a(x, z)`
    pub fn sample_2d(&self, x: f64, z: f64) -> f64 {
        self.sample(x, z, 0.0)
    }

    /// `func_805_a`: accumulate a grid of samples into `out`, scaled by `1/amplitude`.
    /// Layout is x-major, then z, then y (`out[(x * size_z + z) * size_y + y]`).
    #[allow(clippy::too_many_arguments)]
    pub fn add_region(
        &self,
        out: &mut [f64],
        x0: f64,
        y0: f64,
        z0: f64,
        size_x: i32,
        size_y: i32,
        size_z: i32,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
        amplitude: f64,
    ) {
        let inv = 1.0 / amplitude;
        let mut idx = 0usize;
        if size_y == 1 {
            for ix in 0..size_x {
                let mut x = (x0 + ix as f64) * scale_x + self.x_coord;
                let xi = floor_i(x);
                let xc = xi & 255;
                x -= xi as f64;
                let u = fade(x);
                for iz in 0..size_z {
                    let mut z = (z0 + iz as f64) * scale_z + self.z_coord;
                    let zi = floor_i(z);
                    let zc = zi & 255;
                    z -= zi as f64;
                    let w = fade(z);
                    let a = self.p(xc);
                    let aa = self.p(a) + zc;
                    let b = self.p(xc + 1);
                    let ba = self.p(b) + zc;
                    let n0 = lerp(u, grad2(self.p(aa), x, z), grad(self.p(ba), x - 1.0, 0.0, z));
                    let n1 = lerp(u, grad(self.p(aa + 1), x, 0.0, z - 1.0), grad(self.p(ba + 1), x - 1.0, 0.0, z - 1.0));
                    let n = lerp(w, n0, n1);
                    out[idx] += n * inv;
                    idx += 1;
                }
            }
        } else {
            let mut last_yc = -1;
            let (mut c0, mut c1, mut c2, mut c3) = (0.0, 0.0, 0.0, 0.0);
            for ix in 0..size_x {
                let mut x = (x0 + ix as f64) * scale_x + self.x_coord;
                let xi = floor_i(x);
                let xc = xi & 255;
                x -= xi as f64;
                let u = fade(x);
                for iz in 0..size_z {
                    let mut z = (z0 + iz as f64) * scale_z + self.z_coord;
                    let zi = floor_i(z);
                    let zc = zi & 255;
                    z -= zi as f64;
                    let w = fade(z);
                    for iy in 0..size_y {
                        let mut y = (y0 + iy as f64) * scale_y + self.y_coord;
                        let yi = floor_i(y);
                        let yc = yi & 255;
                        y -= yi as f64;
                        let v = fade(y);
                        // Corner lerps are cached per y-lattice cell and reused with a
                        // stale `y` fraction. That is a bug in the original; keep it.
                        if iy == 0 || yc != last_yc {
                            last_yc = yc;
                            let a = self.p(xc) + yc;
                            let aa = self.p(a) + zc;
                            let ab = self.p(a + 1) + zc;
                            let b = self.p(xc + 1) + yc;
                            let ba = self.p(b) + zc;
                            let bb = self.p(b + 1) + zc;
                            c0 = lerp(u, grad(self.p(aa), x, y, z), grad(self.p(ba), x - 1.0, y, z));
                            c1 = lerp(u, grad(self.p(ab), x, y - 1.0, z), grad(self.p(bb), x - 1.0, y - 1.0, z));
                            c2 = lerp(u, grad(self.p(aa + 1), x, y, z - 1.0), grad(self.p(ba + 1), x - 1.0, y, z - 1.0));
                            c3 = lerp(u, grad(self.p(ab + 1), x, y - 1.0, z - 1.0), grad(self.p(bb + 1), x - 1.0, y - 1.0, z - 1.0));
                        }
                        let n0 = lerp(v, c0, c1);
                        let n1 = lerp(v, c2, c3);
                        let n = lerp(w, n0, n1);
                        out[idx] += n * inv;
                        idx += 1;
                    }
                }
            }
        }
    }
}

/// `NoiseGeneratorOctaves`
pub struct OctavePerlin {
    octaves: Vec<PerlinNoise>,
}

impl OctavePerlin {
    pub fn new(rand: &mut JavaRandom, count: usize) -> Self {
        OctavePerlin { octaves: (0..count).map(|_| PerlinNoise::new(rand)).collect() }
    }

    /// `func_806_a(x, z)`
    pub fn sample_2d(&self, x: f64, z: f64) -> f64 {
        let mut sum = 0.0;
        let mut freq = 1.0;
        for o in &self.octaves {
            sum += o.sample_2d(x * freq, z * freq) / freq;
            freq /= 2.0;
        }
        sum
    }

    /// `generateNoiseOctaves`. Reuses `buf` (zeroed) when present, like the Java code.
    #[allow(clippy::too_many_arguments)]
    pub fn generate(
        &self,
        buf: &mut Vec<f64>,
        x: f64,
        y: f64,
        z: f64,
        size_x: i32,
        size_y: i32,
        size_z: i32,
        scale_x: f64,
        scale_y: f64,
        scale_z: f64,
    ) {
        if buf.is_empty() {
            buf.resize((size_x * size_y * size_z) as usize, 0.0);
        } else {
            buf.iter_mut().for_each(|v| *v = 0.0);
        }
        let mut amp = 1.0;
        for o in &self.octaves {
            o.add_region(buf, x, y, z, size_x, size_y, size_z, scale_x * amp, scale_y * amp, scale_z * amp, amp);
            amp /= 2.0;
        }
    }

    /// `func_4109_a`: the flat (y = 10) variant. The last argument is ignored in the original too.
    #[allow(clippy::too_many_arguments)]
    pub fn generate_2d(&self, buf: &mut Vec<f64>, x: i32, z: i32, size_x: i32, size_z: i32, scale_x: f64, scale_z: f64, _unused: f64) {
        self.generate(buf, x as f64, 10.0, z as f64, size_x, 1, size_z, scale_x, 1.0, scale_z);
    }
}

/// `NoiseGenerator2`: 2D simplex noise, used only for climate.
pub struct SimplexNoise {
    perm: [i32; 512],
    pub x_coord: f64,
    pub y_coord: f64,
    pub z_coord: f64,
}

const GRAD3: [[i32; 3]; 12] = [
    [1, 1, 0],
    [-1, 1, 0],
    [1, -1, 0],
    [-1, -1, 0],
    [1, 0, 1],
    [-1, 0, 1],
    [1, 0, -1],
    [-1, 0, -1],
    [0, 1, 1],
    [0, -1, 1],
    [0, 1, -1],
    [0, -1, -1],
];

#[inline]
fn wrap(d: f64) -> i32 {
    if d > 0.0 {
        d as i32
    } else {
        d as i32 - 1
    }
}

#[inline]
fn dot2(g: &[i32; 3], x: f64, y: f64) -> f64 {
    g[0] as f64 * x + g[1] as f64 * y
}

impl SimplexNoise {
    pub fn new(rand: &mut JavaRandom) -> Self {
        let x_coord = rand.next_double() * 256.0;
        let y_coord = rand.next_double() * 256.0;
        let z_coord = rand.next_double() * 256.0;
        let perm = shuffled_perm(rand);
        SimplexNoise { perm, x_coord, y_coord, z_coord }
    }

    /// `func_4157_a`
    #[allow(clippy::too_many_arguments)]
    pub fn add_region(&self, out: &mut [f64], x0: f64, z0: f64, size_x: i32, size_z: i32, scale_x: f64, scale_z: f64, amplitude: f64) {
        let f2 = 0.5 * (3.0f64.sqrt() - 1.0);
        let g2 = (3.0 - 3.0f64.sqrt()) / 6.0;
        let p = |i: i32| self.perm[i as usize];
        let mut idx = 0usize;
        for ix in 0..size_x {
            let x = (x0 + ix as f64) * scale_x + self.x_coord;
            for iz in 0..size_z {
                let y = (z0 + iz as f64) * scale_z + self.y_coord;
                let s = (x + y) * f2;
                let i = wrap(x + s);
                let j = wrap(y + s);
                let t = (i + j) as f64 * g2;
                let dx0 = x - (i as f64 - t);
                let dy0 = y - (j as f64 - t);
                let (i1, j1) = if dx0 > dy0 { (1, 0) } else { (0, 1) };
                let dx1 = dx0 - i1 as f64 + g2;
                let dy1 = dy0 - j1 as f64 + g2;
                let dx2 = dx0 - 1.0 + 2.0 * g2;
                let dy2 = dy0 - 1.0 + 2.0 * g2;
                let ii = i & 255;
                let jj = j & 255;
                let gi0 = p(ii + p(jj)) % 12;
                let gi1 = p(ii + i1 + p(jj + j1)) % 12;
                let gi2 = p(ii + 1 + p(jj + 1)) % 12;
                let mut t0 = 0.5 - dx0 * dx0 - dy0 * dy0;
                let n0 = if t0 < 0.0 {
                    0.0
                } else {
                    t0 *= t0;
                    t0 * t0 * dot2(&GRAD3[gi0 as usize], dx0, dy0)
                };
                let mut t1 = 0.5 - dx1 * dx1 - dy1 * dy1;
                let n1 = if t1 < 0.0 {
                    0.0
                } else {
                    t1 *= t1;
                    t1 * t1 * dot2(&GRAD3[gi1 as usize], dx1, dy1)
                };
                let mut t2 = 0.5 - dx2 * dx2 - dy2 * dy2;
                let n2 = if t2 < 0.0 {
                    0.0
                } else {
                    t2 *= t2;
                    t2 * t2 * dot2(&GRAD3[gi2 as usize], dx2, dy2)
                };
                out[idx] += 70.0 * (n0 + n1 + n2) * amplitude;
                idx += 1;
            }
        }
    }
}

/// `NoiseGeneratorOctaves2`
pub struct OctaveSimplex {
    octaves: Vec<SimplexNoise>,
}

impl OctaveSimplex {
    pub fn new(rand: &mut JavaRandom, count: usize) -> Self {
        OctaveSimplex { octaves: (0..count).map(|_| SimplexNoise::new(rand)).collect() }
    }

    /// `func_4112_a`
    #[allow(clippy::too_many_arguments)]
    pub fn generate(&self, buf: &mut Vec<f64>, x: f64, z: f64, size_x: i32, size_z: i32, scale_x: f64, scale_z: f64, freq_mul: f64) {
        self.generate_with(buf, x, z, size_x, size_z, scale_x, scale_z, freq_mul, 0.5);
    }

    /// `func_4111_a`
    #[allow(clippy::too_many_arguments)]
    pub fn generate_with(
        &self,
        buf: &mut Vec<f64>,
        x: f64,
        z: f64,
        size_x: i32,
        size_z: i32,
        scale_x: f64,
        scale_z: f64,
        freq_mul: f64,
        amp_mul: f64,
    ) {
        let scale_x = scale_x / 1.5;
        let scale_z = scale_z / 1.5;
        let n = (size_x * size_z) as usize;
        if buf.len() >= n {
            buf.iter_mut().for_each(|v| *v = 0.0);
        } else {
            *buf = vec![0.0; n];
        }
        let mut amp = 1.0;
        let mut freq = 1.0;
        for o in &self.octaves {
            o.add_region(buf, x, z, size_x, size_z, scale_x * freq, scale_z * freq, 0.55 / amp);
            freq *= freq_mul;
            amp *= amp_mul;
        }
    }
}
