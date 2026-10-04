//! `ChunkProviderGenerate`: the overworld generator ("RandomLevelSource").
//!
//! This file covers the per-chunk, order-independent part (`provideChunk`):
//! density terrain, biome surface, bedrock and caves. Population lives in `populate`.

use crate::biome::{Biome, BiomeSource};
use crate::block;
use crate::caves::CaveCarver;
use crate::noise::OctavePerlin;
use crate::random::JavaRandom;

pub const CHUNK_VOLUME: usize = 16 * 16 * 128;

/// Block index for chunk-local coordinates, matching the Java layout (`x << 11 | z << 7 | y`).
#[inline]
pub fn index(x: usize, y: usize, z: usize) -> usize {
    x << 11 | z << 7 | y
}

/// A freshly generated, unpopulated chunk.
pub struct RawChunk {
    pub x: i32,
    pub z: i32,
    pub blocks: Box<[u8; CHUNK_VOLUME]>,
    /// Biomes for the chunk's columns, index `x * 16 + z`.
    pub biomes: Vec<Biome>,
}

pub struct OverworldGenerator {
    seed: i64,
    rand: JavaRandom,
    min_limit: OctavePerlin,  // field_912_k
    max_limit: OctavePerlin,  // field_911_l
    main: OctavePerlin,       // field_910_m
    surface: OctavePerlin,    // field_909_n (sand / gravel)
    stone_depth: OctavePerlin, // field_908_o
    pub scale: OctavePerlin,  // field_922_a
    pub depth: OctavePerlin,  // field_921_b
    pub forest: OctavePerlin, // mobSpawnerNoise (tree density)
    pub biome_source: BiomeSource,
    caves: CaveCarver,

    density: Vec<f64>,
    sand_noise: Vec<f64>,
    gravel_noise: Vec<f64>,
    stone_noise: Vec<f64>,
    main_noise: Vec<f64>,
    min_noise: Vec<f64>,
    max_noise: Vec<f64>,
    scale_noise: Vec<f64>,
    depth_noise: Vec<f64>,
    biomes: Vec<Biome>,
}

impl OverworldGenerator {
    pub fn new(seed: i64) -> Self {
        let mut rand = JavaRandom::new(seed);
        let min_limit = OctavePerlin::new(&mut rand, 16);
        let max_limit = OctavePerlin::new(&mut rand, 16);
        let main = OctavePerlin::new(&mut rand, 8);
        let surface = OctavePerlin::new(&mut rand, 4);
        let stone_depth = OctavePerlin::new(&mut rand, 4);
        let scale = OctavePerlin::new(&mut rand, 10);
        let depth = OctavePerlin::new(&mut rand, 16);
        let forest = OctavePerlin::new(&mut rand, 8);
        OverworldGenerator {
            seed,
            rand,
            min_limit,
            max_limit,
            main,
            surface,
            stone_depth,
            scale,
            depth,
            forest,
            biome_source: BiomeSource::new(seed),
            caves: CaveCarver::default(),
            density: Vec::new(),
            // The Java fields start as `new double[256]`, not null.
            sand_noise: vec![0.0; 256],
            gravel_noise: vec![0.0; 256],
            stone_noise: vec![0.0; 256],
            main_noise: Vec::new(),
            min_noise: Vec::new(),
            max_noise: Vec::new(),
            scale_noise: Vec::new(),
            depth_noise: Vec::new(),
            biomes: Vec::new(),
        }
    }

    pub fn seed(&self) -> i64 {
        self.seed
    }

    pub fn rand_mut(&mut self) -> &mut JavaRandom {
        &mut self.rand
    }

    /// `provideChunk`
    pub fn generate(&mut self, cx: i32, cz: i32) -> RawChunk {
        self.rand
            .set_seed((cx as i64).wrapping_mul(341873128712).wrapping_add((cz as i64).wrapping_mul(132897987541)));
        let mut blocks: Box<[u8; CHUNK_VOLUME]> = vec![0u8; CHUNK_VOLUME].into_boxed_slice().try_into().unwrap();
        let mut biomes = std::mem::take(&mut self.biomes);
        self.biome_source.biomes(&mut biomes, cx * 16, cz * 16, 16, 16);
        self.terrain(cx, cz, &mut blocks[..]);
        self.surface(cx, cz, &mut blocks[..], &biomes);
        self.caves.carve(self.seed, cx, cz, &mut blocks[..]);
        let out = RawChunk { x: cx, z: cz, blocks, biomes: biomes[..256].to_vec() };
        self.biomes = biomes;
        out
    }

    /// `generateTerrain`: trilinear upsample of a 5x17x5 density grid into stone/water/ice.
    fn terrain(&mut self, cx: i32, cz: i32, blocks: &mut [u8]) {
        let cell = 4i32;
        let sea_level = 64i32;
        let sx = (cell + 1) as usize;
        let sy = 17usize;
        let sz = (cell + 1) as usize;
        let mut density = std::mem::take(&mut self.density);
        self.density_field(&mut density, cx * cell, 0, cz * cell, sx as i32, sy as i32, sz as i32);
        let temps = &self.biome_source.temperature;

        let d = |x: usize, z: usize, y: usize| density[(x * sz + z) * sy + y];
        for i in 0..cell as usize {
            for j in 0..cell as usize {
                for k in 0..16usize {
                    let fy = 0.125;
                    let mut d000 = d(i, j, k);
                    let mut d010 = d(i, j + 1, k);
                    let mut d100 = d(i + 1, j, k);
                    let mut d110 = d(i + 1, j + 1, k);
                    let s000 = (d(i, j, k + 1) - d000) * fy;
                    let s010 = (d(i, j + 1, k + 1) - d010) * fy;
                    let s100 = (d(i + 1, j, k + 1) - d100) * fy;
                    let s110 = (d(i + 1, j + 1, k + 1) - d110) * fy;
                    for l in 0..8usize {
                        let fx = 0.25;
                        let mut a = d000;
                        let mut b = d010;
                        let ax = (d100 - d000) * fx;
                        let bx = (d110 - d010) * fx;
                        for m in 0..4usize {
                            let mut idx = (m + i * 4) << 11 | (j * 4) << 7 | (k * 8 + l);
                            let fz = 0.25;
                            let mut v = a;
                            let vz = (b - a) * fz;
                            for n in 0..4usize {
                                let t = temps[(i * 4 + m) * 16 + j * 4 + n];
                                let y = (k * 8 + l) as i32;
                                let mut id = block::AIR;
                                if y < sea_level {
                                    if t < 0.5 && y >= sea_level - 1 {
                                        id = block::ICE;
                                    } else {
                                        id = block::WATER_STILL;
                                    }
                                }
                                if v > 0.0 {
                                    id = block::STONE;
                                }
                                blocks[idx] = id;
                                idx += 128;
                                v += vz;
                            }
                            a += ax;
                            b += bx;
                        }
                        d000 += s000;
                        d010 += s010;
                        d100 += s100;
                        d110 += s110;
                    }
                }
            }
        }
        self.density = density;
    }

    /// `func_4061_a`
    #[allow(clippy::too_many_arguments)]
    fn density_field(&mut self, out: &mut Vec<f64>, x: i32, y: i32, z: i32, sx: i32, sy: i32, sz: i32) {
        if out.is_empty() {
            *out = vec![0.0; (sx * sy * sz) as usize];
        }
        let h_scale = 684.412;
        let v_scale = 684.412;
        self.scale.generate_2d(&mut self.scale_noise, x, z, sx, sz, 1.121, 1.121, 0.5);
        self.depth.generate_2d(&mut self.depth_noise, x, z, sx, sz, 200.0, 200.0, 0.5);
        self.main.generate(
            &mut self.main_noise,
            x as f64,
            y as f64,
            z as f64,
            sx,
            sy,
            sz,
            h_scale / 80.0,
            v_scale / 160.0,
            h_scale / 80.0,
        );
        self.min_limit
            .generate(&mut self.min_noise, x as f64, y as f64, z as f64, sx, sy, sz, h_scale, v_scale, h_scale);
        self.max_limit
            .generate(&mut self.max_noise, x as f64, y as f64, z as f64, sx, sy, sz, h_scale, v_scale, h_scale);

        let temps = &self.biome_source.temperature;
        let humid = &self.biome_source.humidity;
        let mut i3 = 0usize;
        let mut i2 = 0usize;
        let step = 16 / sx;
        for ix in 0..sx {
            let bx = ix * step + step / 2;
            for iz in 0..sz {
                let bz = iz * step + step / 2;
                let t = temps[(bx * 16 + bz) as usize];
                let h = humid[(bx * 16 + bz) as usize] * t;
                let mut dry = 1.0 - h;
                dry *= dry;
                dry *= dry;
                dry = 1.0 - dry;
                let mut scale = (self.scale_noise[i2] + 256.0) / 512.0;
                scale *= dry;
                if scale > 1.0 {
                    scale = 1.0;
                }
                let mut depth = self.depth_noise[i2] / 8000.0;
                if depth < 0.0 {
                    depth = -depth * 0.3;
                }
                depth = depth * 3.0 - 2.0;
                if depth < 0.0 {
                    depth /= 2.0;
                    if depth < -1.0 {
                        depth = -1.0;
                    }
                    depth /= 1.4;
                    depth /= 2.0;
                    scale = 0.0;
                } else {
                    if depth > 1.0 {
                        depth = 1.0;
                    }
                    depth /= 8.0;
                }
                if scale < 0.0 {
                    scale = 0.0;
                }
                scale += 0.5;
                depth = depth * sy as f64 / 16.0;
                let center = sy as f64 / 2.0 + depth * 4.0;
                i2 += 1;

                for iy in 0..sy {
                    let mut falloff = (iy as f64 - center) * 12.0 / scale;
                    if falloff < 0.0 {
                        falloff *= 4.0;
                    }
                    let lo = self.min_noise[i3] / 512.0;
                    let hi = self.max_noise[i3] / 512.0;
                    let mix = (self.main_noise[i3] / 10.0 + 1.0) / 2.0;
                    let mut v = if mix < 0.0 {
                        lo
                    } else if mix > 1.0 {
                        hi
                    } else {
                        lo + (hi - lo) * mix
                    };
                    v -= falloff;
                    if iy > sy - 4 {
                        let f = ((iy - (sy - 4)) as f32 / 3.0f32) as f64;
                        v = v * (1.0 - f) + -10.0 * f;
                    }
                    out[i3] = v;
                    i3 += 1;
                }
            }
        }
    }

    /// `replaceBlocksForBiome`: top/filler layers, beaches, sandstone and bedrock.
    fn surface(&mut self, cx: i32, cz: i32, blocks: &mut [u8], biomes: &[Biome]) {
        let sea_level = 64i32;
        let s = 1.0 / 32.0;
        self.surface
            .generate(&mut self.sand_noise, (cx * 16) as f64, (cz * 16) as f64, 0.0, 16, 16, 1, s, s, 1.0);
        self.surface
            .generate(&mut self.gravel_noise, (cx * 16) as f64, 109.0134, (cz * 16) as f64, 16, 1, 16, s, 1.0, s);
        self.stone_depth.generate(
            &mut self.stone_noise,
            (cx * 16) as f64,
            (cz * 16) as f64,
            0.0,
            16,
            16,
            1,
            s * 2.0,
            s * 2.0,
            s * 2.0,
        );

        // Variable roles follow the Java: `a` indexes z-ish noise columns, `b` the x row.
        for a in 0..16usize {
            for b in 0..16usize {
                let biome = biomes[a + b * 16];
                let sand = self.sand_noise[a + b * 16] + self.rand.next_double() * 0.2 > 0.0;
                let gravel = self.gravel_noise[a + b * 16] + self.rand.next_double() * 0.2 > 3.0;
                let depth = (self.stone_noise[a + b * 16] / 3.0 + 3.0 + self.rand.next_double() * 0.25) as i32;
                let mut run = -1i32;
                let mut top = biome.top_block();
                let mut filler = biome.filler_block();

                for y in (0..128i32).rev() {
                    let idx = (b * 16 + a) * 128 + y as usize;
                    if y <= self.rand.next_int(5) {
                        blocks[idx] = block::BEDROCK;
                        continue;
                    }
                    let cur = blocks[idx];
                    if cur == block::AIR {
                        run = -1;
                    } else if cur == block::STONE {
                        if run == -1 {
                            if depth <= 0 {
                                top = block::AIR;
                                filler = block::STONE;
                            } else if y >= sea_level - 4 && y <= sea_level + 1 {
                                top = biome.top_block();
                                filler = biome.filler_block();
                                if gravel {
                                    top = block::AIR;
                                    filler = block::GRAVEL;
                                }
                                if sand {
                                    top = block::SAND;
                                    filler = block::SAND;
                                }
                            }
                            if y < sea_level && top == block::AIR {
                                top = block::WATER_STILL;
                            }
                            run = depth;
                            blocks[idx] = if y >= sea_level - 1 { top } else { filler };
                        } else if run > 0 {
                            run -= 1;
                            blocks[idx] = filler;
                            if run == 0 && filler == block::SAND {
                                run = self.rand.next_int(4);
                                filler = block::SANDSTONE;
                            }
                        }
                    }
                }
            }
        }
    }
}
