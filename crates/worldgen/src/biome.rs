//! `BiomeGenBase` lookup and `WorldChunkManager` (temperature/humidity → biome).

use crate::block;
use crate::noise::OctaveSimplex;
use crate::random::JavaRandom;
use std::sync::OnceLock;

/// Discriminants match the oracle's biome id order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Biome {
    Rainforest = 0,
    Swampland = 1,
    SeasonalForest = 2,
    Forest = 3,
    Savanna = 4,
    Shrubland = 5,
    Taiga = 6,
    Desert = 7,
    Plains = 8,
    IceDesert = 9,
    Tundra = 10,
    Hell = 11,
    Sky = 12,
}

impl Biome {
    pub fn top_block(self) -> u8 {
        match self {
            Biome::Desert | Biome::IceDesert => block::SAND,
            _ => block::GRASS,
        }
    }

    pub fn filler_block(self) -> u8 {
        match self {
            Biome::Desert | Biome::IceDesert => block::SAND,
            _ => block::DIRT,
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Biome::Rainforest => "Rainforest",
            Biome::Swampland => "Swampland",
            Biome::SeasonalForest => "Seasonal Forest",
            Biome::Forest => "Forest",
            Biome::Savanna => "Savanna",
            Biome::Shrubland => "Shrubland",
            Biome::Taiga => "Taiga",
            Biome::Desert => "Desert",
            Biome::Plains => "Plains",
            Biome::IceDesert => "Ice Desert",
            Biome::Tundra => "Tundra",
            Biome::Hell => "Hell",
            Biome::Sky => "Sky",
        }
    }

    /// `BiomeGenBase.getBiome(temperature, humidity)`
    pub fn from_climate(temp: f32, humid: f32) -> Biome {
        let h = humid * temp;
        if temp < 0.1 {
            Biome::Tundra
        } else if h < 0.2 {
            if temp < 0.5 {
                Biome::Tundra
            } else if temp < 0.95 {
                Biome::Savanna
            } else {
                Biome::Desert
            }
        } else if h > 0.5 && temp < 0.7 {
            Biome::Swampland
        } else if temp < 0.5 {
            Biome::Taiga
        } else if temp < 0.97 {
            if h < 0.35 {
                Biome::Shrubland
            } else {
                Biome::Forest
            }
        } else if h < 0.45 {
            Biome::Plains
        } else if h < 0.9 {
            Biome::SeasonalForest
        } else {
            Biome::Rainforest
        }
    }

    /// `BiomeGenBase.getBiomeFromLookup`
    pub fn lookup(temp: f64, humid: f64) -> Biome {
        static TABLE: OnceLock<[Biome; 4096]> = OnceLock::new();
        let table = TABLE.get_or_init(|| {
            let mut t = [Biome::Plains; 4096];
            for i in 0..64 {
                for j in 0..64 {
                    t[i + j * 64] = Biome::from_climate(i as f32 / 63.0, j as f32 / 63.0);
                }
            }
            t
        });
        let i = (temp * 63.0) as i32;
        let j = (humid * 63.0) as i32;
        table[(i + j * 64) as usize]
    }
}

/// `WorldChunkManager`. Holds the scratch arrays the Java class exposes publicly,
/// because the terrain generator reads `temperature`/`humidity` from the last call.
pub struct BiomeSource {
    temp_noise: OctaveSimplex,
    humid_noise: OctaveSimplex,
    detail_noise: OctaveSimplex,
    pub temperature: Vec<f64>,
    pub humidity: Vec<f64>,
    detail: Vec<f64>,
}

impl BiomeSource {
    pub fn new(seed: i64) -> Self {
        BiomeSource {
            temp_noise: OctaveSimplex::new(&mut JavaRandom::new(seed.wrapping_mul(9871)), 4),
            humid_noise: OctaveSimplex::new(&mut JavaRandom::new(seed.wrapping_mul(39811)), 4),
            detail_noise: OctaveSimplex::new(&mut JavaRandom::new(seed.wrapping_mul(543321)), 2),
            temperature: Vec::new(),
            humidity: Vec::new(),
            detail: Vec::new(),
        }
    }

    /// `loadBlockGeneratorData`. Index order is `x * depth + z`.
    /// Note the original passes `width` for both noise dimensions; harmless for squares.
    pub fn biomes(&mut self, out: &mut Vec<Biome>, x: i32, z: i32, width: i32, depth: i32) {
        let n = (width * depth) as usize;
        if out.len() < n {
            *out = vec![Biome::Plains; n];
        }
        self.temp_noise.generate(&mut self.temperature, x as f64, z as f64, width, width, 0.025f32 as f64, 0.025f32 as f64, 0.25);
        self.humid_noise.generate(&mut self.humidity, x as f64, z as f64, width, width, 0.05f32 as f64, 0.05f32 as f64, 1.0 / 3.0);
        self.detail_noise.generate(&mut self.detail, x as f64, z as f64, width, width, 0.25, 0.25, 0.5882352941176471);
        let mut i = 0usize;
        for _ in 0..width {
            for _ in 0..depth {
                let d = self.detail[i] * 1.1 + 0.5;
                let mut w = 0.01;
                let mut inv = 1.0 - w;
                let mut t = (self.temperature[i] * 0.15 + 0.7) * inv + d * w;
                w = 0.002;
                inv = 1.0 - w;
                let mut h = (self.humidity[i] * 0.15 + 0.5) * inv + d * w;
                t = 1.0 - (1.0 - t) * (1.0 - t);
                if t < 0.0 {
                    t = 0.0;
                }
                if h < 0.0 {
                    h = 0.0;
                }
                if t > 1.0 {
                    t = 1.0;
                }
                if h > 1.0 {
                    h = 1.0;
                }
                self.temperature[i] = t;
                self.humidity[i] = h;
                out[i] = Biome::lookup(t, h);
                i += 1;
            }
        }
    }

    /// `getBiomeGenAt(x, z)`: a 1x1 `biomes` call (clobbers the scratch arrays like the original).
    pub fn biome_at(&mut self, x: i32, z: i32) -> Biome {
        let mut out = Vec::new();
        self.biomes(&mut out, x, z, 1, 1);
        out[0]
    }

    /// `getTemperatures`: temperature only, without humidity. Does not touch `self.temperature`.
    pub fn temperatures(&mut self, out: &mut Vec<f64>, x: i32, z: i32, width: i32, depth: i32) {
        let n = (width * depth) as usize;
        if out.len() < n {
            *out = vec![0.0; n];
        }
        self.temp_noise.generate(out, x as f64, z as f64, width, depth, 0.025f32 as f64, 0.025f32 as f64, 0.25);
        self.detail_noise.generate(&mut self.detail, x as f64, z as f64, width, depth, 0.25, 0.25, 0.5882352941176471);
        let mut i = 0usize;
        for _ in 0..width {
            for _ in 0..depth {
                let d = self.detail[i] * 1.1 + 0.5;
                let w = 0.01;
                let inv = 1.0 - w;
                let mut t = (out[i] * 0.15 + 0.7) * inv + d * w;
                t = 1.0 - (1.0 - t) * (1.0 - t);
                if t < 0.0 {
                    t = 0.0;
                }
                if t > 1.0 {
                    t = 1.0;
                }
                out[i] = t;
                i += 1;
            }
        }
    }
}
