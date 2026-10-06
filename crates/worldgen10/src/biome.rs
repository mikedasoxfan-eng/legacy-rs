//! Beta 1.8.1 biome definitions and `WorldChunkManager` layer queries.
use crate::{
    block,
    genlayer::{self, Layer},
    random::JavaRandom,
};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
// BiomeGenBase's singleton generators outlive every World/provider.
// (The enum below identifies the corresponding singleton.)
#[repr(u8)]
pub enum Biome {
    Ocean = 0,
    Plains = 1,
    Desert = 2,
    ExtremeHills = 3,
    Forest = 4,
    Taiga = 5,
    Swampland = 6,
    River = 7,
    Hell = 8,
    Sky = 9,
    FrozenOcean = 10,
    FrozenRiver = 11,
    IcePlains = 12,
    IceMountains = 13,
    MushroomIsland = 14,
    MushroomIslandShore = 15,
}
static BIG_TREES: std::sync::LazyLock<Mutex<Vec<Option<crate::features::BigTree>>>> =
    std::sync::LazyLock::new(|| {
        Mutex::new(
            (0..16)
                .map(|_| Some(crate::features::BigTree::new()))
                .collect(),
        )
    });
pub(crate) fn big_tree(biome: Biome, w: &mut crate::World, x: i32, y: i32, z: i32) -> bool {
    let mut tree = BIG_TREES.lock().unwrap()[biome as usize]
        .take()
        .expect("recursive biome big tree");
    tree.set_scale(1.0, 1.0, 1.0);
    let result = tree.generate(w, x, y, z);
    BIG_TREES.lock().unwrap()[biome as usize] = Some(tree);
    result
}
impl Biome {
    pub fn from_id(id: i32) -> Self {
        match id {
            0 => Self::Ocean,
            1 => Self::Plains,
            2 => Self::Desert,
            3 => Self::ExtremeHills,
            4 => Self::Forest,
            5 => Self::Taiga,
            6 => Self::Swampland,
            7 => Self::River,
            8 => Self::Hell,
            9 => Self::Sky,
            10 => Self::FrozenOcean,
            11 => Self::FrozenRiver,
            12 => Self::IcePlains,
            13 => Self::IceMountains,
            14 => Self::MushroomIsland,
            15 => Self::MushroomIslandShore,
            _ => panic!("invalid biome {id}"),
        }
    }
    pub fn top_block(self) -> u8 {
        if matches!(self, Self::MushroomIsland | Self::MushroomIslandShore) { return 110; }
        if self == Self::Sky { return block::DIRT; }
        if self == Self::Desert {
            block::SAND
        } else {
            block::GRASS
        }
    }
    pub fn filler_block(self) -> u8 {
        if self == Self::Desert {
            block::SAND
        } else {
            block::DIRT
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Self::Ocean => "Ocean",
            Self::Plains => "Plains",
            Self::Desert => "Desert",
            Self::ExtremeHills => "Extreme Hills",
            Self::Forest => "Forest",
            Self::Taiga => "Taiga",
            Self::Swampland => "Swampland",
            Self::River => "River",
            Self::Hell => "Hell",
            Self::Sky => "Sky",
            Self::FrozenOcean => "FrozenOcean",
            Self::FrozenRiver => "FrozenRiver",
            Self::IcePlains => "Ice Plains",
            Self::IceMountains => "Ice Mountains",
            Self::MushroomIsland => "MushroomIsland",
            Self::MushroomIslandShore => "MushroomIslandShore",
        }
    }
    pub fn heights(self) -> (f32, f32) {
        match self {
            Self::Ocean => (-1.0, 0.4),
            Self::FrozenOcean => (-1.0, 0.5),
            Self::IceMountains => (0.2, 1.8),
            Self::MushroomIsland => (0.2, 1.0),
            Self::MushroomIslandShore => (-1.0, 0.1),
            Self::Desert => (0.1, 0.2),
            Self::ExtremeHills => (0.2, 1.8),
            Self::Taiga => (0.1, 0.4),
            Self::Swampland => (-0.2, 0.1),
            Self::River | Self::FrozenRiver => (-0.5, 0.0),
            _ => (0.1, 0.3),
        }
    }
    pub fn climate(self) -> (f32, f32) {
        match self {
            Self::Plains => (0.8, 0.4),
            Self::Desert | Self::Hell => (2.0, 0.0),
            Self::FrozenOcean | Self::FrozenRiver | Self::IcePlains | Self::IceMountains => (0.0, 0.5),
            Self::MushroomIsland | Self::MushroomIslandShore => (0.9, 1.0),
            Self::ExtremeHills => (0.2, 0.3),
            Self::Forest => (0.7, 0.8),
            Self::Taiga => (0.3, 0.8),
            Self::Swampland => (0.8, 0.9),
            _ => (0.5, 0.5),
        }
    }
    pub fn temperature(self) -> f32 {
        self.climate().0
    }
    pub fn rainfall(self) -> f32 {
        self.climate().1
    }
    pub fn color(self) -> i32 {
        [
            112, 9286496, 16421912, 6316128, 353825, 747097, 522674, 255, 16711680, 8421631,
            9474208, 10526975, 16777215, 10526880, 16711935, 10486015,
        ][self as usize]
    }
    pub fn foliage_color(self) -> i32 {
        match self {
            Self::Forest | Self::Taiga => 5159473,
            Self::Swampland => 9154376,
            _ => 5169201,
        }
    }
    pub fn water_color10(self)->i32 {if self==Self::Swampland {14745456} else {16777215}}
    pub fn tint_color10(self,color:u32)->u32 {if self==Self::Swampland {((color&16711422)+5115470)/2} else {color}}
    pub fn enable_snow(self) -> bool {
        false
    }
    pub fn enable_rain(self) -> bool {
        !matches!(self, Self::Desert | Self::Hell | Self::Sky)
    }
    // Compatibility names for the still-unported weather/population callers.
    #[allow(non_upper_case_globals)]
    pub const Rainforest: Self = Self::Swampland;
    #[allow(non_upper_case_globals)]
    pub const SeasonalForest: Self = Self::Forest;
    #[allow(non_upper_case_globals)]
    pub const IceDesert: Self = Self::Desert;
    #[allow(non_upper_case_globals)]
    pub const Tundra: Self = Self::Taiga;
}
pub struct BiomeSource {
    fixed: Option<Biome>,
    layers: [Arc<Layer>; 4],
    pub temperature: Vec<f64>,
    pub humidity: Vec<f64>,
    cache: Mutex<Cache>,
}
#[derive(Default)]
struct Cache {
    blocks: HashMap<(i32, i32), CacheBlock>,
    last_cleanup: i64,
}
struct CacheBlock {
    biomes: Vec<Biome>,
    temperature: Vec<f64>,
    rainfall: Vec<f64>,
    last_access: i64,
}
fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64
}
impl Cache {
    fn block<'a>(&'a mut self, source: &BiomeSource, x: i32, z: i32) -> &'a mut CacheBlock {
        let b = self
            .blocks
            .entry((x >> 4, z >> 4))
            .or_insert_with(|| source.cache_block((x >> 4) << 4, (z >> 4) << 4));
        b.last_access = now_ms();
        b
    }
}
impl BiomeSource {
    pub fn new(seed: i64) -> Self {
        Self {
            fixed: None,
            layers: genlayer::stack(seed),
            temperature: Vec::new(),
            humidity: Vec::new(),
            cache: Mutex::new(Cache::default()),
        }
    }
    pub fn hell() -> Self {
        let mut s = Self::new(0);
        s.fixed = Some(Biome::Hell);
        s
    }
    pub fn sky() -> Self {
        let mut s = Self::new(0);
        s.fixed = Some(Biome::Sky);
        s
    }
    pub(crate) fn structure_fixture_plains() -> Self {
        let mut s = Self::new(0);
        s.fixed = Some(Biome::Plains);
        s
    }
    /// Coarse biome map for density blending (`func_35557_b`).
    pub fn generation_biomes(&self, x: i32, z: i32, w: i32, h: i32) -> Vec<Biome> {
        self.map(0, x, z, w, h)
    }
    fn map(&self, layer: usize, x: i32, z: i32, w: i32, h: i32) -> Vec<Biome> {
        if let Some(b) = self.fixed {
            vec![b; (w * h) as usize]
        } else {
            self.layers[layer]
                .get(x, z, w, h)
                .into_iter()
                .take((w * h) as usize)
                .map(Biome::from_id)
                .collect()
        }
    }
    /// Voronoi block map. Java uses `x + z * width`, including non-square queries.
    pub fn biomes(&mut self, out: &mut Vec<Biome>, x: i32, z: i32, w: i32, h: i32) {
        if let Some(biome) = self.fixed {
            let len = (w * h) as usize;
            if out.len() < len { out.resize(len, biome); }
            out[..len].fill(biome);
            let temperature = if biome == Biome::Sky { 0.5 } else { 1.0 };
            if self.temperature.len() < len { self.temperature.resize(len, temperature); }
            if self.humidity.len() < len { self.humidity.resize(len, 0.0); }
            self.temperature[..len].fill(temperature);
            self.humidity[..len].fill(0.0);
            return;
        }
        if w == 16 && h == 16 && x & 15 == 0 && z & 15 == 0 && self.fixed.is_none() {
            // func_35555_a resets IntCache even when the biome cache already has
            // this chunk. The pool's stale tails affect stronghold relocation.
            crate::intcache::CACHE.with(|cache| cache.borrow_mut().reset());
            let mut cache = self.cache.lock().unwrap();
            let b = cache.block(self, x, z);
            *out = b.biomes.clone();
            self.temperature = b.temperature.clone();
            self.humidity = b.rainfall.clone();
        } else {
            *out = self.map(1, x, z, w, h);
            self.temperature = self.climate_values(true, x, z, w, h);
            self.humidity = self.climate_values(false, x, z, w, h);
        }
    }
    fn cache_block(&self, x: i32, z: i32) -> CacheBlock {
        let temperature = self.climate_values(true, x, z, 16, 16);
        let rainfall = self.climate_values(false, x, z, 16, 16);
        let biomes = self.map(1, x, z, 16, 16);
        CacheBlock {
            biomes,
            temperature,
            rainfall,
            last_access: 0,
        }
    }
    pub fn biome_at(&mut self, x: i32, z: i32) -> Biome {
        if let Some(b) = self.fixed {
            return b;
        }
        self.cache.lock().unwrap().block(self, x, z).biomes[((x & 15) + (z & 15) * 16) as usize]
    }
    pub fn block_biomes(&self, x: i32, z: i32, w: i32, h: i32) -> Vec<Biome> {
        if self.fixed.is_none() && w == 16 && h == 16 && x & 15 == 0 && z & 15 == 0 {
            self.cache.lock().unwrap().block(self, x, z).biomes.clone()
        } else {
            self.map(1, x, z, w, h)
        }
    }
    fn climate_values(&self, temp: bool, x: i32, z: i32, w: i32, h: i32) -> Vec<f64> {
        if let Some(b) = self.fixed {
            return vec![
                if temp {
                    if b == Biome::Sky {
                        0.5
                    } else {
                        1.0
                    }
                } else {
                    0.0
                };
                (w * h) as usize
            ];
        }
        self.layers[if temp { 2 } else { 3 }]
            .get(x, z, w, h)
            .into_iter()
            .take((w * h) as usize)
            .map(|v| (v as f32 / 65536.0).min(1.0) as f64)
            .collect()
    }
    /// WorldChunkManager.func_35554_b / func_40540_a (height is currently ignored in Java).
    pub fn temperature_at_height(&self, x: i32, _y: i32, z: i32) -> f32 {
        self.raw_temperature(x, z) as f32
    }
    pub fn raw_temperature(&self, x: i32, z: i32) -> f64 {
        if let Some(b) = self.fixed {
            return if b == Biome::Sky { 0.5 } else { 1.0 };
        }
        self.cache.lock().unwrap().block(self, x, z).temperature
            [((x & 15) + (z & 15) * 16) as usize]
    }
    pub fn raw_rainfall(&self, x: i32, z: i32) -> f64 {
        if self.fixed.is_some() {
            0.0
        } else {
            self.cache.lock().unwrap().block(self, x, z).rainfall
                [((x & 15) + (z & 15) * 16) as usize]
        }
    }
    pub fn cleanup_cache(&self) {
        self.cleanup_cache_at(now_ms());
    }
    pub fn cleanup_cache_at(&self, now: i64) {
        let mut cache = self.cache.lock().unwrap();
        let elapsed = now.wrapping_sub(cache.last_cleanup);
        if elapsed > 7500 || elapsed < 0 {
            cache.last_cleanup = now;
            cache.blocks.retain(|_, b| {
                let age = now.wrapping_sub(b.last_access);
                (0..=30000).contains(&age)
            });
        }
    }
    pub(crate) fn cache_snapshot(&self) -> Vec<(i32, i32)> {
        let mut keys: Vec<_> = self.cache.lock().unwrap().blocks.keys().copied().collect();
        keys.sort();
        keys
    }
    pub(crate) fn cache_fixture_times(&self, last: i64, entries: &[((i32, i32), i64)]) {
        let mut c = self.cache.lock().unwrap();
        c.last_cleanup = last;
        for (key, time) in entries {
            c.blocks.get_mut(key).unwrap().last_access = *time;
        }
    }
    pub fn temperatures(&mut self, out: &mut Vec<f64>, x: i32, z: i32, w: i32, h: i32) {
        let values=self.climate_values(true,x,z,w,h);
        if out.len()<values.len() {out.resize(values.len(),0.0);}
        out[..values.len()].copy_from_slice(&values);
    }
    pub fn rainfalls(&self, x: i32, z: i32, w: i32, h: i32) -> Vec<f64> {
        self.climate_values(false, x, z, w, h)
    }
    pub fn viable(&self, x: i32, z: i32, r: i32, allowed: &[Biome]) -> bool {
        let (x0, z0, x1, z1) = ((x - r) >> 2, (z - r) >> 2, (x + r) >> 2, (z + r) >> 2);
        if let Some(b) = self.fixed {
            return allowed.contains(&b);
        }
        let w = x1 - x0 + 1;
        let h = z1 - z0 + 1;
        self.layers[0]
            .get_without_reset(x0, z0, w, h)
            .into_iter()
            .take((w * h) as usize)
            .all(|b| allowed.contains(&Biome::from_id(b)))
    }
    /// Vanilla increments the reservoir count only when it replaces the selected position.
    pub fn find_position(
        &self,
        x: i32,
        z: i32,
        r: i32,
        allowed: &[Biome],
        rand: &mut JavaRandom,
    ) -> Option<(i32, i32)> {
        if let Some(b) = self.fixed {
            return if allowed.contains(&b) {
                Some((
                    x - r + rand.next_int(r * 2 + 1),
                    z - r + rand.next_int(r * 2 + 1),
                ))
            } else {
                None
            };
        }
        let (x0, z0, x1, z1) = ((x - r) >> 2, (z - r) >> 2, (x + r) >> 2, (z + r) >> 2);
        let w = x1 - x0 + 1;
        let h = z1 - z0 + 1;
        let values = if let Some(b) = self.fixed {
            vec![b as i32; (w * h) as usize]
        } else {
            self.layers[0].get_without_reset(x0, z0, w, h)
        };
        let mut found = None;
        let mut count = 0;
        for (i, b) in values.iter().enumerate() {
            if allowed.contains(&Biome::from_id(*b))
                && (found.is_none() || rand.next_int(count + 1) == 0)
            {
                found = Some(((x0 + i as i32 % w) << 2, (z0 + i as i32 / w) << 2));
                count += 1;
            }
        }
        found
    }
}

pub fn hsb_to_rgb(hue: f32, sat: f32, bri: f32) -> u32 {
    let (r, g, b);
    if sat == 0.0 {
        let v = (bri * 255.0f32 + 0.5f32) as i32;
        r = v;
        g = v;
        b = v;
    } else {
        let h = (hue - hue.floor()) * 6.0f32;
        let f = h - h.floor();
        let p = bri * (1.0f32 - sat);
        let q = bri * (1.0f32 - sat * f);
        let t = bri * (1.0f32 - sat * (1.0f32 - f));
        let c = |v: f32| (v * 255.0f32 + 0.5f32) as i32;
        (r, g, b) = match h as i32 {
            0 => (c(bri), c(t), c(p)),
            1 => (c(q), c(bri), c(p)),
            2 => (c(p), c(bri), c(t)),
            3 => (c(p), c(q), c(bri)),
            4 => (c(t), c(p), c(bri)),
            5 => (c(bri), c(p), c(q)),
            _ => (0, 0, 0),
        };
    }
    0xff000000 | ((r as u32) << 16) | ((g as u32) << 8) | b as u32
}

impl Biome {
    pub fn sky_color_by_temp(self, temp: f32) -> i32 {
        if self == Self::Sky {
            return 0;
        }
        let t = (temp / 3.0f32).clamp(-1.0, 1.0);
        hsb_to_rgb(
            224.0f32 / 360.0f32 - t * 0.05f32,
            0.5f32 + t * 0.1f32,
            1.0f32,
        ) as i32
    }
}
