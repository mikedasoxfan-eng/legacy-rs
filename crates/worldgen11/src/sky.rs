//! `ChunkProviderSky`: the Beta 1.8.1 floating-island raw chunk generator.
pub use crate::end::populate;
use crate::{
    biome::{Biome, BiomeSource},
    caves::CaveCarver,
    noise::OctavePerlin,
    overworld::{index, RawChunk, CHUNK_VOLUME},
    JavaRandom,
};
pub struct SkyGenerator {
    seed: i64,
    rand: JavaRandom,
    min: OctavePerlin,
    max: OctavePerlin,
    main: OctavePerlin,
    sand: OctavePerlin,
    stone: OctavePerlin,
    pub scale: OctavePerlin,
    pub depth: OctavePerlin,
    pub forest: OctavePerlin,
    pub biome_source: BiomeSource,
    caves: CaveCarver,
}
impl SkyGenerator {
    pub fn new(seed: i64) -> Self {
        let mut rand = JavaRandom::new(seed);
        let min = OctavePerlin::new(&mut rand, 16);
        let max = OctavePerlin::new(&mut rand, 16);
        let main = OctavePerlin::new(&mut rand, 8);
        let sand = OctavePerlin::new(&mut rand, 4);
        let stone = OctavePerlin::new(&mut rand, 4);
        let scale = OctavePerlin::new(&mut rand, 10);
        let depth = OctavePerlin::new(&mut rand, 16);
        let forest = OctavePerlin::new(&mut rand, 8);
        Self {
            seed,
            rand,
            min,
            max,
            main,
            sand,
            stone,
            scale,
            depth,
            forest,
            biome_source: BiomeSource::sky(),
            caves: Default::default(),
        }
    }
    pub fn rand_mut(&mut self) -> &mut JavaRandom {
        &mut self.rand
    }
    pub fn generate(&mut self, cx: i32, cz: i32) -> RawChunk {
        self.rand.set_seed(
            (cx as i64)
                .wrapping_mul(341873128712)
                .wrapping_add((cz as i64).wrapping_mul(132897987541)),
        );
        let mut blocks: Box<[u8; CHUNK_VOLUME]> =
            vec![0; CHUNK_VOLUME].into_boxed_slice().try_into().unwrap();
        let mut biomes = Vec::new();
        self.biome_source
            .biomes(&mut biomes, cx * 16, cz * 16, 16, 16);
        let d = self.density(cx * 2, cz * 2);
        for a in 0..2 {
            for b in 0..2 {
                for c in 0..32 {
                    let at = |a: usize, b: usize, c: usize| d[(a * 3 + b) * 33 + c];
                    let mut p = at(a, b, c);
                    let mut q = at(a, b + 1, c);
                    let mut r = at(a + 1, b, c);
                    let mut s = at(a + 1, b + 1, c);
                    let dp = (at(a, b, c + 1) - p) * 0.25;
                    let dq = (at(a, b + 1, c + 1) - q) * 0.25;
                    let dr = (at(a + 1, b, c + 1) - r) * 0.25;
                    let ds = (at(a + 1, b + 1, c + 1) - s) * 0.25;
                    for y in 0..4 {
                        let mut left = p;
                        let mut right = q;
                        let dl = (r - p) * 0.125;
                        let dt = (s - q) * 0.125;
                        for x in 0..8 {
                            let mut value = left;
                            let step = (right - left) * 0.125;
                            for z in 0..8 {
                                blocks[index(a * 8 + x, c * 4 + y, b * 8 + z)] =
                                    if value > 0.0 { 1 } else { 0 };
                                value += step;
                            }
                            left += dl;
                            right += dt;
                        }
                        p += dp;
                        q += dq;
                        r += dr;
                        s += ds;
                    }
                }
            }
        }
        self.surface(cx, cz, &mut blocks[..], &biomes);
        self.caves.carve(self.seed, cx, cz, &mut blocks[..]);
        RawChunk {
            x: cx,
            z: cz,
            blocks,
            biomes,
        }
    }
    fn density(&self, x: i32, z: i32) -> Vec<f64> {
        let mut scale = Vec::new();
        let mut depth = Vec::new();
        let mut main = Vec::new();
        let mut min = Vec::new();
        let mut max = Vec::new();
        self.scale
            .generate_2d(&mut scale, x, z, 3, 3, 1.121, 1.121, 0.5);
        self.depth
            .generate_2d(&mut depth, x, z, 3, 3, 200.0, 200.0, 0.5);
        let hs = 684.412 * 2.0;
        let vs = 684.412;
        self.main.generate(
            &mut main,
            x as f64,
            0.0,
            z as f64,
            3,
            33,
            3,
            hs / 80.0,
            vs / 160.0,
            hs / 80.0,
        );
        self.min
            .generate(&mut min, x as f64, 0.0, z as f64, 3, 33, 3, hs, vs, hs);
        self.max
            .generate(&mut max, x as f64, 0.0, z as f64, 3, 33, 3, hs, vs, hs);
        // Both 2D fields are sampled but their height transform never affects the result.
        let mut out = vec![0.0; 297];
        for a in 0..3 {
            for b in 0..3 {
                for y in 0..33 {
                    let i = (a * 3 + b) * 33 + y;
                    let lo = min[i] / 512.0;
                    let hi = max[i] / 512.0;
                    let blend = (main[i] / 10.0 + 1.0) / 2.0;
                    let mut n = if blend < 0.0 {
                        lo
                    } else if blend > 1.0 {
                        hi
                    } else {
                        lo + (hi - lo) * blend
                    };
                    n -= 8.0;
                    if y > 1 {
                        let f = ((y - 1) as f32 / 31.0f32) as f64;
                        n = n * (1.0 - f) + -30.0 * f;
                    }
                    if y < 8 {
                        let f = ((8 - y) as f32 / 7.0f32) as f64;
                        n = n * (1.0 - f) + -30.0 * f;
                    }
                    out[i] = n;
                }
            }
        }
        out
    }
    fn surface(&mut self, cx: i32, cz: i32, blocks: &mut [u8], biomes: &[Biome]) {
        let mut sand = vec![0.0; 256];
        let mut gravel = vec![0.0; 256];
        let mut stone = vec![0.0; 256];
        let scale = 1.0 / 32.0;
        self.sand.generate(
            &mut sand,
            (cx * 16) as f64,
            (cz * 16) as f64,
            0.0,
            16,
            16,
            1,
            scale,
            scale,
            1.0,
        );
        self.sand.generate(
            &mut gravel,
            (cx * 16) as f64,
            109.0,
            (cz * 16) as f64,
            16,
            1,
            16,
            scale,
            1.0,
            scale,
        );
        self.stone.generate(
            &mut stone,
            (cx * 16) as f64,
            (cz * 16) as f64,
            0.0,
            16,
            16,
            1,
            scale * 2.0,
            scale * 2.0,
            scale * 2.0,
        );
        for z in 0..16 {
            for x in 0..16 {
                let biome = biomes[z + x * 16];
                let thickness =
                    (stone[z + x * 16] / 3.0 + 3.0 + self.rand.next_double() * 0.25) as i32;
                let mut left = -1;
                let mut top = biome.top_block();
                let mut filler = biome.filler_block();
                for y in (0..128).rev() {
                    let i = index(x, y, z);
                    match blocks[i] {
                        0 => left = -1,
                        1 => {
                            if left == -1 {
                                if thickness <= 0 {
                                    top = 0;
                                    filler = 1;
                                }
                                left = thickness;
                                blocks[i] = top;
                            } else if left > 0 {
                                left -= 1;
                                blocks[i] = filler;
                                if left == 0 && filler == 12 {
                                    left = self.rand.next_int(4);
                                    filler = 24;
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
