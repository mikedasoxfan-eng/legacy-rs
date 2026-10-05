//! Literal client weather algorithms from EntityRenderer and RenderLightningBolt.
//! Terrain is an immutable worker snapshot; these Randoms never advance World.rand.
use worldgen::{math, Biome, JavaRandom};

#[derive(Clone, Copy, Debug, Default)]
pub struct Weather {
    pub rain: f32,
    pub prev_rain: f32,
    pub thunder: f32,
    pub prev_thunder: f32,
    pub flash: i32,
}
impl Weather {
    pub fn rain_at(self, partial: f32) -> f32 {
        self.prev_rain + (self.rain - self.prev_rain) * partial
    }
    pub fn thunder_at(self, partial: f32) -> f32 {
        (self.prev_thunder + (self.thunder - self.prev_thunder) * partial) * self.rain_at(partial)
    }
}

/// Replace the worker's position-list adapter with entity boltVertex seeds when available.
#[derive(Clone, Copy, Debug)]
pub struct Lightning {
    pub pos: [f64; 3],
    pub bolt_vertex: i64,
}

pub fn snows(b: Biome) -> bool {
    matches!(b, Biome::Taiga | Biome::IceDesert | Biome::Tundra)
}
pub fn rains(b: Biome) -> bool {
    !snows(b) && !matches!(b, Biome::Desert | Biome::Hell | Biome::Sky)
}

#[derive(Clone)]
pub struct Column {
    pub x: i32,
    pub z: i32,
    pub top: i32,
    pub biome: Biome,
    pub particle_rain: bool,
    pub blocks: [u8; 128],
    pub meta: [u8; 128],
    pub brightness: [f32; 129],
}
#[derive(Default)]
pub struct Snapshot {
    pub columns: Vec<Column>,
}
impl Snapshot {
    pub fn column(&self, x: i32, z: i32) -> Option<&Column> {
        let first = self.columns.first()?;
        let (dx, dz) = (x - first.x, z - first.z);
        let width=(self.columns.len() as f64).sqrt() as i32;
        if !(0..width).contains(&dx) || !(0..width).contains(&dz) {
            return None;
        }
        self.columns.get((dx * width + dz) as usize)
    }
    pub(crate) fn brightness(&self, p: [f64; 3]) -> f32 {
        self.column(math::floor_double(p[0]), math::floor_double(p[2]))
            .map_or(0.0, |c| c.brightness[math::floor_double(p[1]).clamp(0, 128) as usize])
    }
}

/// GL current color is float, unlike Tessellator's packed byte color. Preserve it.
#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct WeatherVertex {
    pub pos: [f32; 3],
    pub uv: [f32; 2],
    pub color: [f32; 4],
}

/// java.util.Random.nextGaussian, with the cached second sample cleared on setSeed.
pub struct ClientRandom {
    pub random: JavaRandom,
    gaussian: Option<f64>,
}
impl ClientRandom {
    pub fn new(seed: i64) -> Self {
        Self { random: JavaRandom::new(seed), gaussian: None }
    }
    pub fn set_seed(&mut self, seed: i64) {
        self.random.set_seed(seed);
        self.gaussian = None;
    }
    pub(crate) fn gaussian(&mut self) -> f64 {
        if let Some(v) = self.gaussian.take() {
            return v;
        }
        loop {
            let a = 2.0 * self.random.next_double() - 1.0;
            let b = 2.0 * self.random.next_double() - 1.0;
            let s = a * a + b * b;
            if s >= 1.0 || s == 0.0 {
                continue;
            }
            let k = (-2.0 * strict_log(s) / s).sqrt();
            self.gaussian = Some(b * k);
            return a * k;
        }
    }
}

/// fdlibm's __ieee754_log, as used by Random.nextGaussian's StrictMath.log.
/// Its argument here is always a positive normal double below one.
fn strict_log(x: f64) -> f64 {
    const HI: f64 = 6.93147180369123816490e-1;
    const LO: f64 = 1.90821492927058770002e-10;
    const LG: [f64; 7] = [
        6.666666666666735130e-1,
        3.999999999940941908e-1,
        2.857142874366239149e-1,
        2.222219843214978396e-1,
        1.818357216161805012e-1,
        1.531383769920937332e-1,
        1.479819860511658591e-1,
    ];
    let bits = x.to_bits();
    let mut hx = (bits >> 32) as i32;
    let mut k = (hx >> 20) - 1023;
    hx &= 0xfffff;
    let i = (hx + 0x95f64) & 0x100000;
    let x = f64::from_bits((bits & 0xffffffff) | (((hx | (i ^ 0x3ff00000)) as u64) << 32));
    k += i >> 20;
    let f = x - 1.0;
    let dk = k as f64;
    if (0xfffff & (2 + hx)) < 3 {
        if f == 0.0 {
            return if k == 0 { 0.0 } else { dk * HI + dk * LO };
        }
        let r = f * f * (0.5 - 0.33333333333333333 * f);
        return if k == 0 { f - r } else { dk * HI - ((r - dk * LO) - f) };
    }
    let s = f / (2.0 + f);
    let z = s * s;
    let w = z * z;
    let t1 = w * (LG[1] + w * (LG[3] + w * LG[5]));
    let t2 = z * (LG[0] + w * (LG[2] + w * (LG[4] + w * LG[6])));
    let r = t2 + t1;
    if ((hx - 0x6147a) | (0x6b851 - hx)) > 0 {
        let half = 0.5 * f * f;
        if k == 0 {
            f - (half - s * (half + r))
        } else {
            dk * HI - ((half - (s * (half + r) + dk * LO)) - f)
        }
    } else if k == 0 {
        f - s * (f - r)
    } else {
        dk * HI - ((s * (f - r) - dk * LO) - f)
    }
}

fn column_seed(x: i32, z: i32) -> i32 {
    x.wrapping_mul(x)
        .wrapping_mul(3121)
        .wrapping_add(x.wrapping_mul(45238971))
        .wrapping_add(z.wrapping_mul(z).wrapping_mul(418711))
        .wrapping_add(z.wrapping_mul(13761))
}

/// Snow pass then rain pass, x-major, crossed quads (Beta does not face them at camera).
pub fn precipitation(
    s: &Snapshot,
    cam: [f64; 3],
    tick: i32,
    partial: f32,
    rain: f32,
    fancy: bool,
    rng: &mut ClientRandom,
) -> [Vec<WeatherVertex>; 2] {
    let mut out = [Vec::new(), Vec::new()];
    if rain <= 0.0 {
        return out;
    }
    let cy = math::floor_double(cam[1]);
    let (cx, cz) = (math::floor_double(cam[0]), math::floor_double(cam[2]));
    let radius = if fancy { 10 } else { 5 };
    for (pass, verts) in out.iter_mut().enumerate() {
        for c in &s.columns {
            if (c.x - cx).abs() > radius || (c.z - cz).abs() > radius {
                continue;
            }
            if !(if pass == 0 { snows(c.biome) } else { rains(c.biome) }) {
                continue;
            }
            let top = if pass == 0 { c.top.max(0) } else { c.top };
            let lo = (cy - radius).max(top);
            let hi = (cy + radius).max(top);
            if lo == hi {
                continue;
            }
            let seed = column_seed(c.x, c.z);
            rng.set_seed(seed as i64);
            let (u, scroll, drift, bright, alpha);
            let dx = (c.x as f32 + 0.5f32) as f64 - cam[0];
            let dz = (c.z as f32 + 0.5f32) as f64 - cam[2];
            let distance = (dx * dx + dz * dz).sqrt() as f32 / radius as f32;
            if pass == 0 {
                let t = tick as f32 + partial;
                scroll = ((tick & 511) as f32 + partial) / 512.0f32;
                u = rng.random.next_float() + t * 0.01f32 * rng.gaussian() as f32;
                drift = rng.random.next_float() + t * rng.gaussian() as f32 * 0.001f32;
                bright = c.brightness[top.max(cy).clamp(0, 128) as usize];
                alpha = ((1.0f32 - distance * distance) * 0.3f32 + 0.5f32) * rain;
            } else {
                scroll =
                    ((tick.wrapping_add(seed) & 31) as f32 + partial) / 32.0f32 * (3.0f32 + rng.random.next_float());
                u = 0.0;
                drift = 0.0;
                bright = c.brightness[128] * 0.85f32 + 0.15f32;
                alpha = ((1.0f32 - distance * distance) * 0.5f32 + 0.5f32) * rain;
            }
            let color = [bright, bright, bright, alpha];
            let v0 = lo as f32 / 4.0f32 + scroll + drift;
            let v1 = hi as f32 / 4.0f32 + scroll + drift;
            let (x, z) = (c.x as f64, c.z as f64);
            for q in [
                [
                    [x, lo as f64, z + 0.5],
                    [x + 1.0, lo as f64, z + 0.5],
                    [x + 1.0, hi as f64, z + 0.5],
                    [x, hi as f64, z + 0.5],
                ],
                [
                    [x + 0.5, lo as f64, z],
                    [x + 0.5, lo as f64, z + 1.0],
                    [x + 0.5, hi as f64, z + 1.0],
                    [x + 0.5, hi as f64, z],
                ],
            ] {
                for (p, uv) in q.into_iter().zip([[u, v0], [1.0f32 + u, v0], [1.0f32 + u, v1], [u, v1]]) {
                    verts.push(WeatherVertex { pos: std::array::from_fn(|i| (p[i] - cam[i]) as f32), uv, color });
                }
            }
        }
    }
    out
}

pub struct Splash {
    pub pos: [f64; 3],
    pub smoke: bool,
}
pub struct RainSound {
    pub pos: [f64; 3],
    pub volume: f32,
    pub pitch: f32,
}

/// EntityRenderer.addRainParticles, including reservoir sampling and sound-counter RNG.
pub fn add_rain_particles(
    s: &Snapshot,
    cam: [f64; 3],
    tick: i32,
    rain: f32,
    fancy: bool,
    rng: &mut ClientRandom,
    sound_counter: &mut i32,
) -> (Vec<Splash>, Option<RainSound>) {
    let rain = if fancy { rain } else { rain / 2.0f32 };
    let mut out = Vec::new();
    if rain == 0.0 {
        return (out, None);
    }
    rng.set_seed(tick as i64 * 312987231i64);
    let (cx, cy, cz) = (math::floor_double(cam[0]), math::floor_double(cam[1]), math::floor_double(cam[2]));
    let mut count = 0;
    let mut sound_pos = [0.0; 3];
    for _ in 0..(100.0f32 * rain * rain) as i32 {
        let x = cx + rng.random.next_int(10) - rng.random.next_int(10);
        let z = cz + rng.random.next_int(10) - rng.random.next_int(10);
        let Some(c) = s.column(x, z) else {
            continue;
        };
        let y = c.top;
        let id = if (1..=128).contains(&y) { c.blocks[(y - 1) as usize] } else { 0 };
        if y <= cy + 10 && y >= cy - 10 && c.particle_rain {
            let fx = rng.random.next_float();
            let fz = rng.random.next_float();
            if id > 0 {
                let pos = [
                    (x as f32 + fx) as f64,
                    (y as f32 + 0.1f32) as f64 - super::rprops(id).bounds[1] as f64,
                    (z as f32 + fz) as f64,
                ];
                let smoke = matches!(id, 10 | 11);
                if !smoke {
                    count += 1;
                    if rng.random.next_int(count) == 0 {
                        sound_pos = pos;
                    }
                }
                out.push(Splash { pos, smoke });
            }
        }
    }
    let mut sound = None;
    if count > 0 {
        let draw = rng.random.next_int(3);
        let old = *sound_counter;
        *sound_counter = sound_counter.wrapping_add(1);
        if draw < old {
            *sound_counter = 0;
            let roof = s.column(cx, cz).is_some_and(|c| c.top > cy);
            let muffled = sound_pos[1] > cam[1] + 1.0 && roof;
            sound = Some(RainSound {
                pos: sound_pos,
                volume: if muffled { 0.1 } else { 0.2 },
                pitch: if muffled { 0.5 } else { 1.0 },
            });
        }
    }
    (out, sound)
}

/// RenderLightningBolt's four shells and three branches; triangle strips expanded.
pub fn lightning(bolt: Lightning, cam: [f64; 3]) -> Vec<WeatherVertex> {
    let pos: [f64; 3] = std::array::from_fn(|i| bolt.pos[i] - cam[i]);
    let mut xs = [0.0; 8];
    let mut zs = [0.0; 8];
    let (mut x, mut z) = (0.0, 0.0);
    let mut rng = JavaRandom::new(bolt.bolt_vertex);
    for i in (0..8).rev() {
        xs[i] = x;
        zs[i] = z;
        x += (rng.next_int(11) - 5) as f64;
        z += (rng.next_int(11) - 5) as f64;
    }
    let mut out = Vec::new();
    for shell in 0..4 {
        rng.set_seed(bolt.bolt_vertex);
        for branch in 0..3 {
            let start = 7 - branch;
            let end = if branch == 0 { 0 } else { start - 2 };
            let (mut bx, mut bz) = (xs[start] - x, zs[start] - z);
            for segment in (end..=start).rev() {
                let (px, pz) = (bx, bz);
                let bound = if branch == 0 { 11 } else { 31 };
                bx += (rng.next_int(bound) - bound / 2) as f64;
                bz += (rng.next_int(bound) - bound / 2) as f64;
                let mut upper = 0.1 + shell as f64 * 0.2;
                let mut lower = upper;
                if branch == 0 {
                    upper *= segment as f64 * 0.1 + 1.0;
                    lower *= (segment as i32 - 1) as f64 * 0.1 + 1.0;
                }
                let mut strip = Vec::new();
                for corner in 0..5 {
                    let mut ux = pos[0] + 0.5 - upper;
                    let mut uz = pos[2] + 0.5 - upper;
                    let mut lx = pos[0] + 0.5 - lower;
                    let mut lz = pos[2] + 0.5 - lower;
                    if corner == 1 || corner == 2 {
                        ux += upper * 2.0;
                        lx += lower * 2.0;
                    }
                    if corner == 2 || corner == 3 {
                        uz += upper * 2.0;
                        lz += lower * 2.0;
                    }
                    // Tessellator.setColorRGBA_F truncates these to bytes.
                    let color = [114.0 / 255.0, 114.0 / 255.0, 127.0 / 255.0, 76.0 / 255.0];
                    strip.push(WeatherVertex {
                        pos: [(lx + bx) as f32, (pos[1] + (segment * 16) as f64) as f32, (lz + bz) as f32],
                        uv: [0.0; 2],
                        color,
                    });
                    strip.push(WeatherVertex {
                        pos: [(ux + px) as f32, (pos[1] + ((segment + 1) * 16) as f64) as f32, (uz + pz) as f32],
                        uv: [0.0; 2],
                        color,
                    });
                }
                for i in 2..strip.len() {
                    if i % 2 == 0 {
                        out.extend_from_slice(&[strip[i - 2], strip[i - 1], strip[i]]);
                    } else {
                        out.extend_from_slice(&[strip[i - 1], strip[i - 2], strip[i]]);
                    }
                }
            }
        }
    }
    out
}

struct Particle {
    pos: [f64; 3],
    prev: [f64; 3],
    motion: [f64; 3],
    bounds: [f64; 6],
    smoke: bool,
    color: f32,
    scale: f32,
    age: i32,
    max_age: i32,
    texture: i32,
    dead: bool,
    in_web: bool,
}

pub struct Particles {
    particles: Vec<Particle>,
    math_random: JavaRandom,
    entity_seeds: JavaRandom,
}
impl Particles {
    /// `EffectRenderer.getStatistics`: particles alive.
    pub fn len(&self) -> usize {
        self.particles.len()
    }

    pub fn new(seed: i64) -> Self {
        Self {
            particles: Vec::new(),
            math_random: JavaRandom::new(seed ^ 0x4d415448),
            entity_seeds: JavaRandom::new(seed ^ 0x4658),
        }
    }
    /// Constructors consume Math.random and each entity's Random, independently of
    /// EntityRenderer.random. Vanilla initializes those nondeterministically too.
    pub fn spawn(&mut self, splashes: Vec<Splash>) {
        for s in splashes {
            let mut r = JavaRandom::new(self.entity_seeds.next_long());
            let m = &mut self.math_random;
            let mut motion = std::array::from_fn(|_| ((m.next_double() * 2.0 - 1.0) as f32 * 0.4f32) as f64);
            let speed = (m.next_double() + m.next_double() + 1.0) as f32 * 0.15f32;
            let length = (motion[0] * motion[0] + motion[1] * motion[1] + motion[2] * motion[2]).sqrt() as f32;
            for v in &mut motion {
                *v = *v / length as f64 * speed as f64 * 0.4f32 as f64;
            }
            motion[1] += 0.1f32 as f64;
            let _jitter_x = r.next_float() * 3.0f32;
            let _jitter_y = r.next_float() * 3.0f32;
            let mut scale = (r.next_float() * 0.5f32 + 0.5f32) * 2.0f32;
            let _base_age = (4.0f32 / (r.next_float() * 0.9f32 + 0.1f32)) as i32;
            let (color, texture);
            if s.smoke {
                for v in &mut motion {
                    *v *= 0.1f32 as f64;
                }
                color = (m.next_double() * 0.3f32 as f64) as f32;
                scale *= 12.0f32 / 16.0f32;
                texture = 0;
            } else {
                motion[0] *= 0.3f32 as f64;
                motion[1] = (m.next_double() as f32 * 0.2f32 + 0.1f32) as f64;
                motion[2] *= 0.3f32 as f64;
                color = 1.0;
                texture = 19 + r.next_int(4);
            }
            let max_age = (8.0 / (m.next_double() * 0.8 + 0.2)) as i32;
            // EntityRainFX.setSize(.01,.01) does NOT resize the .2-wide bounding box!
            let half = (0.2f32 / 2.0f32) as f64;
            let p = s.pos;
            let bounds = [p[0] - half, p[1] - half, p[2] - half, p[0] + half, p[1] - half + 0.2f32 as f64, p[2] + half];
            self.particles.push(Particle {
                pos: p,
                prev: p,
                motion,
                bounds,
                smoke: s.smoke,
                color,
                scale,
                age: 0,
                max_age,
                texture,
                dead: false,
                in_web: false,
            });
        }
    }
    pub fn tick(&mut self, terrain: &Snapshot) {
        for p in &mut self.particles {
            p.prev = p.pos;
            if p.smoke {
                let age = p.age;
                p.age += 1;
                if age >= p.max_age {
                    p.dead = true;
                }
                p.texture = 7 - p.age * 8 / p.max_age;
                p.motion[1] += 0.004;
            } else {
                p.motion[1] -= 0.06f32 as f64;
            }
            let ground = move_particle(p, terrain);
            if p.smoke && p.pos[1] == p.prev[1] {
                p.motion[0] *= 1.1;
                p.motion[2] *= 1.1;
            }
            for v in &mut p.motion {
                *v *= if p.smoke { 0.96f32 as f64 } else { 0.98f32 as f64 };
            }
            if !p.smoke {
                let age = p.max_age;
                p.max_age -= 1;
                if age <= 0 {
                    p.dead = true;
                }
            }
            if ground {
                if !p.smoke && self.math_random.next_double() < 0.5 {
                    p.dead = true;
                }
                p.motion[0] *= 0.7f32 as f64;
                p.motion[2] *= 0.7f32 as f64;
            }
            if !p.smoke {
                let y = math::floor_double(p.pos[1]);
                if let Some(c) = terrain.column(math::floor_double(p.pos[0]), math::floor_double(p.pos[2])) {
                    if (0..128).contains(&y) {
                        let mat = worldgen::block::mat(worldgen::block::material_of(c.blocks[y as usize]));
                        if mat.is_liquid || mat.is_solid {
                            let mut meta = c.meta[y as usize] as i32;
                            if meta >= 8 {
                                meta = 0;
                            }
                            let air = (meta + 1) as f32 / 9.0f32;
                            if p.pos[1] < ((y + 1) as f32 - air) as f64 {
                                p.dead = true;
                            }
                        }
                    }
                }
            }
        }
        self.particles.retain(|p| !p.dead);
    }
    pub fn vertices(
        &self,
        terrain: &Snapshot,
        cam: [f64; 3],
        yaw: f32,
        pitch: f32,
        partial: f32,
    ) -> Vec<WeatherVertex> {
        let (cy, sy) = (math::cos(yaw * math::PI_F32 / 180.0f32), math::sin(yaw * math::PI_F32 / 180.0f32));
        let sp = math::sin(pitch * math::PI_F32 / 180.0f32);
        let cp = math::cos(pitch * math::PI_F32 / 180.0f32);
        let (rx, ry, rz, ux, uz) = (cy, cp, sy, -sy * sp, cy * sp);
        let mut out = Vec::new();
        for p in &self.particles {
            let pos: [f32; 3] =
                std::array::from_fn(|i| (p.prev[i] + (p.pos[i] - p.prev[i]) * partial as f64 - cam[i]) as f32);
            let size = 0.1f32
                * (p.scale
                    * if p.smoke {
                        ((p.age as f32 + partial) / p.max_age as f32 * 32.0f32).clamp(0.0, 1.0)
                    } else {
                        1.0
                    });
            let u0 = (p.texture % 16) as f32 / 16.0f32;
            let u1 = u0 + 0.999f32 / 16.0f32;
            let v0 = (p.texture / 16) as f32 / 16.0f32;
            let v1 = v0 + 0.999f32 / 16.0f32;
            let bright_pos = [p.pos[0], p.pos[1] - 0.1f32 as f64 + (p.bounds[4] - p.bounds[1]) * 0.66, p.pos[2]];
            let c = ((p.color * terrain.brightness(bright_pos) * 255.0f32) as i32).clamp(0, 255) as f32 / 255.0f32;
            let points = [
                [pos[0] - rx * size - ux * size, pos[1] - ry * size, pos[2] - rz * size - uz * size],
                [pos[0] - rx * size + ux * size, pos[1] + ry * size, pos[2] - rz * size + uz * size],
                [pos[0] + rx * size + ux * size, pos[1] + ry * size, pos[2] + rz * size + uz * size],
                [pos[0] + rx * size - ux * size, pos[1] - ry * size, pos[2] + rz * size - uz * size],
            ];
            for (pos, uv) in points.into_iter().zip([[u1, v1], [u1, v0], [u0, v0], [u0, v1]]) {
                out.push(WeatherVertex { pos, uv, color: [c, c, c, 1.0] });
            }
        }
        out
    }
}

fn intersects(a: [f64; 6], b: [f64; 6]) -> bool {
    (0..3).all(|i| a[i + 3] > b[i] && a[i] < b[i + 3])
}

fn move_particle(p: &mut Particle, s: &Snapshot) -> bool {
    let mut original = p.motion;
    if p.in_web {
        p.in_web = false;
        original[0] *= 0.25;
        original[1] *= 0.05f32 as f64;
        original[2] *= 0.25;
        p.motion = [0.0; 3];
    }
    let mut sweep = p.bounds;
    for i in 0..3 {
        if original[i] < 0.0 {
            sweep[i] += original[i];
        } else {
            sweep[i + 3] += original[i];
        }
    }
    let mut boxes = Vec::new();
    for x in math::floor_double(sweep[0])..math::floor_double(sweep[3] + 1.0) {
        for z in math::floor_double(sweep[2])..math::floor_double(sweep[5] + 1.0) {
            let Some(c) = s.column(x, z) else {
                continue;
            };
            for y in math::floor_double(sweep[1]) - 1..math::floor_double(sweep[4] + 1.0) {
                if !(0..128).contains(&y) {
                    continue;
                }
                for b in collision_boxes(c.blocks[y as usize], c.meta[y as usize], [x, y, z]) {
                    if intersects(b, sweep) {
                        boxes.push(b);
                    }
                }
            }
        }
    }
    let mut delta = original;
    for axis in [1, 0, 2] {
        for b in &boxes {
            if !(0..3).filter(|&i| i != axis).all(|i| p.bounds[i + 3] > b[i] && p.bounds[i] < b[i + 3]) {
                continue;
            }
            if delta[axis] > 0.0 && p.bounds[axis + 3] <= b[axis] {
                delta[axis] = delta[axis].min(b[axis] - p.bounds[axis + 3]);
            }
            if delta[axis] < 0.0 && p.bounds[axis] >= b[axis + 3] {
                delta[axis] = delta[axis].max(b[axis + 3] - p.bounds[axis]);
            }
        }
        p.bounds[axis] += delta[axis];
        p.bounds[axis + 3] += delta[axis];
        if delta[axis] != original[axis] {
            p.motion[axis] = 0.0;
        }
    }
    p.pos = [(p.bounds[0] + p.bounds[3]) / 2.0, p.bounds[1] + 0.1f32 as f64, (p.bounds[2] + p.bounds[5]) / 2.0];
    // Block.onEntityCollidedWithBlock scans the contracted bounding box.
    for x in math::floor_double(p.bounds[0] + 0.001)..=math::floor_double(p.bounds[3] - 0.001) {
        for z in math::floor_double(p.bounds[2] + 0.001)..=math::floor_double(p.bounds[5] - 0.001) {
            if let Some(c) = s.column(x, z) {
                for y in math::floor_double(p.bounds[1] + 0.001)..=math::floor_double(p.bounds[4] - 0.001) {
                    if !(0..128).contains(&y) {
                        continue;
                    }
                    match c.blocks[y as usize] {
                        30 => p.in_web = true,
                        88 => {
                            p.motion[0] *= 0.4;
                            p.motion[2] *= 0.4;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    delta[1] != original[1] && original[1] < 0.0
}

/// Block collision boxes used by ambient particles. No step-up (EntityFX.stepHeight=0).
fn collision_boxes(id: u8, meta: u8, pos: [i32; 3]) -> Vec<[f64; 6]> {
    if matches!(id, 0 | 6 | 8..=11 | 27 | 28 | 30..=32 | 36 | 37..=40 | 50 | 51 | 55 | 59 | 63 | 66 | 68..=70 | 72 | 75..=77 | 83 | 90)
    {
        return Vec::new();
    }
    let mut b = super::rprops(id).bounds;
    let mut shapes = Vec::new();
    match id {
        29 | 33 => b = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        34 => {
            let pair = match meta & 7 {
                0 => [[0.0, 0.0, 0.0, 1.0, 0.25, 1.0], [0.375, 0.25, 0.375, 0.625, 1.0, 0.625]],
                1 => [[0.0, 0.75, 0.0, 1.0, 1.0, 1.0], [0.375, 0.0, 0.375, 0.625, 0.75, 0.625]],
                2 => [[0.0, 0.0, 0.0, 1.0, 1.0, 0.25], [0.25, 0.375, 0.25, 0.75, 0.625, 1.0]],
                3 => [[0.0, 0.0, 0.75, 1.0, 1.0, 1.0], [0.25, 0.375, 0.0, 0.75, 0.625, 0.75]],
                4 => [[0.0, 0.0, 0.0, 0.25, 1.0, 1.0], [0.375, 0.25, 0.25, 0.625, 0.75, 1.0]],
                5 => [[0.75, 0.0, 0.0, 1.0, 1.0, 1.0], [0.0, 0.375, 0.25, 0.75, 0.625, 0.75]],
                _ => return shapes,
            };
            for b in pair {
                shapes.push(std::array::from_fn(|i| pos[i % 3] as f64 + b[i]));
            }
            return shapes;
        }
        53 | 67 => {
            let pair = match meta {
                0 => [[0.0, 0.0, 0.0, 0.5, 0.5, 1.0], [0.5, 0.0, 0.0, 1.0, 1.0, 1.0]],
                1 => [[0.0, 0.0, 0.0, 0.5, 1.0, 1.0], [0.5, 0.0, 0.0, 1.0, 0.5, 1.0]],
                2 => [[0.0, 0.0, 0.0, 1.0, 0.5, 0.5], [0.0, 0.0, 0.5, 1.0, 1.0, 1.0]],
                3 => [[0.0, 0.0, 0.0, 1.0, 1.0, 0.5], [0.0, 0.0, 0.5, 1.0, 0.5, 1.0]],
                _ => return shapes,
            };
            for b in pair {
                shapes.push(std::array::from_fn(|i| pos[i % 3] as f64 + b[i]));
            }
            return shapes;
        }
        60 => b = [0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        78 => {
            if meta & 7 < 3 {
                return shapes;
            }
            b[4] = 0.5;
        }
        85 => b = [0.0, 0.0, 0.0, 1.0, 1.5, 1.0],
        81 => b = [0.0625, 0.0, 0.0625, 0.9375, 0.9375, 0.9375],
        88 => b[4] = 0.875,
        92 => b = [(1 + meta as i32 * 2) as f32 / 16.0, 0.0, 0.0625, 0.9375, 0.4375, 0.9375],
        64 | 71 | 65 | 96 => {
            let width = if id == 65 { 0.125 } else { 0.1875 };
            let side = if id == 65 {
                match meta {
                    2 => 2,
                    3 => 0,
                    4 => 1,
                    5 => 3,
                    _ => 0,
                }
            } else if id == 96 {
                if meta & 4 == 0 {
                    b = [0.0, 0.0, 0.0, 1.0, width, 1.0];
                    4
                } else {
                    [2, 0, 1, 3][(meta & 3) as usize]
                }
            } else {
                if meta & 4 == 0 {
                    meta.wrapping_sub(1) & 3
                } else {
                    meta & 3
                }
            };
            b = match side {
                0 => [0.0, 0.0, 0.0, 1.0, 1.0, width],
                1 => [1.0 - width, 0.0, 0.0, 1.0, 1.0, 1.0],
                2 => [0.0, 0.0, 1.0 - width, 1.0, 1.0, 1.0],
                3 => [0.0, 0.0, 0.0, width, 1.0, 1.0],
                _ => b,
            };
        }
        _ => {}
    }
    // Cactus/cake/fence use float addition before widening in vanilla.
    shapes.push(std::array::from_fn(|i| {
        if matches!(id, 81 | 92) || (id == 85 && i == 4) {
            (pos[i % 3] as f32 + b[i]) as f64
        } else {
            pos[i % 3] as f64 + b[i] as f64
        }
    }));
    shapes
}
