#[derive(Clone,Debug)]
pub struct VillagerSpawn10 {pub position:[f64;3],pub profession:i32}
// Village piece factory, pending-road expansion and population.
use super::{component::Component, long_map_order::LongMapOrder, BoundingBox as B, Piece};
use crate::{BiomeSource, JavaRandom, World};
use std::collections::HashMap;
#[path = "village_generated.rs"]
mod generated;
const NAMES: [&str; 12] = [
    "ComponentVillageHouse4_Garden",
    "ComponentVillageChurch",
    "ComponentVillageHouse1",
    "ComponentVillageWoodHut",
    "ComponentVillageHall",
    "ComponentVillageField",
    "ComponentVillageField2",
    "ComponentVillageHouse2",
    "ComponentVillageHouse3",
    "ComponentVillageTorch",
    "ComponentVillageStartPiece",
    "ComponentVillagePathGen",
];
const SIZES: [[i32; 3]; 10] = [
    [5, 6, 5],
    [5, 12, 9],
    [9, 9, 6],
    [4, 6, 5],
    [9, 7, 11],
    [13, 4, 9],
    [7, 4, 9],
    [10, 6, 7],
    [9, 7, 12],
    [3, 4, 2],
];
#[derive(Clone, Debug)]
pub struct VillagePiece {
    pub component: Component,
    pub kind: usize,
    pub height: i32,
    pub flag: bool,
    pub style: i32,
    pub length: i32,
    pub villagers10:i32,
}
impl VillagePiece {
    fn new(kind: usize, depth: i32, bounds: B, orientation: i32, r: &mut JavaRandom) -> Self {
        let flag = if kind == 0 || kind == 3 {
            r.next_bool()
        } else {
            true
        };
        let style = if kind == 3 { r.next_int(3) } else { 0 };
        Self {
            component: Component {
                bounds,
                orientation,
                depth,
            },
            kind,
            villagers10:0,
            height: -1,
            flag,
            style,
            length: bounds.sizes()[0].max(bounds.sizes()[2]),
        }
    }
    fn spawn_villagers10(&mut self,w:&mut World,clip:B,x:i32,y:i32,z:i32,count:i32) {
        for n in self.villagers10..count {
            let (x,y,z)=self.component.position(x+n,y,z);
            if !clip.contains(x,y,z) {break;}
            self.villagers10+=1;
            let profession=match self.kind {1=>2,2=>1,7=>3,4 if n==0=>4,_=>0};
            w.villager_spawns10.push(VillagerSpawn10 {position:[x as f64+0.5,y as f64,z as f64+0.5],profession});
        }
    }
    pub fn name(&self) -> &'static str {
        NAMES[self.kind]
    }
}
fn inclusive(r: &mut JavaRandom, lo: i32, hi: i32) -> i32 {
    if lo >= hi {
        lo
    } else {
        r.next_int(hi - lo + 1) + lo
    }
}
fn top_solid(w: &mut World, x: i32, z: i32) -> i32 {
    let mut y = 127;
    while y > 0 {
        let m = w.get_block_material(x, y, z);
        if crate::block::mat(m).blocks_movement && m != crate::block::MAT_LEAVES {
            return y + 1;
        }
        y -= 1;
    }
    -1
}
fn average_height(w: &mut World, clip: B, b: B) -> i32 {
    let mut sum: i32 = 0;
    let mut count = 0;
    for z in b.min_z..=b.max_z {
        for x in b.min_x..=b.max_x {
            if clip.contains(x, 64, z) {
                sum = sum.wrapping_add(top_solid(w, x, z).max(63));
                count += 1;
            }
        }
    }
    if count == 0 {
        -1
    } else {
        sum / count
    }
}
impl Piece for VillagePiece {
    fn bounds(&self) -> B {
        self.component.bounds
    }
    fn bounds_mut(&mut self) -> &mut B {
        &mut self.component.bounds
    }
    fn component_type(&self) -> i32 {
        self.component.depth
    }
    fn generate(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) -> bool {
        match self.kind {
            0 => self.generate_0(w, r, clip),
            1 => self.generate_1(w, r, clip),
            2 => self.generate_2(w, r, clip),
            3 => self.generate_3(w, r, clip),
            4 => self.generate_4(w, r, clip),
            5 => self.generate_5(w, r, clip),
            6 => self.generate_6(w, r, clip),
            7 => self.generate_7(w, r, clip),
            8 => self.generate_8(w, r, clip),
            9 => self.generate_9(w, r, clip),
            10 => self.generate_10(w, r, clip),
            11 => self.generate_11(w, r, clip),
            _ => unreachable!(),
        }
    }
}
#[derive(Clone, Debug)]
struct Weight {
    kind: usize,
    weight: i32,
    spawned: i32,
    limit: i32,
}
pub struct VillageStart {
    pub pieces: Vec<VillagePiece>,
    pub bounds: B,
    pub sizeable: bool,
    weights: Vec<Weight>,
    previous: Option<usize>,
    roads: Vec<usize>,
    houses: Vec<usize>,
    size: i32,
}
impl VillageStart {
    pub fn new(biomes: &BiomeSource, r: &mut JavaRandom, cx: i32, cz: i32) -> Self {
        Self::with_size(biomes, r, cx, cz, 0)
    }
    pub fn with_size(
        biomes: &BiomeSource,
        r: &mut JavaRandom,
        cx: i32,
        cz: i32,
        size: i32,
    ) -> Self {
        let mut weights = Vec::new();
        for (kind, (weight, lo, hi)) in [
            (4, 2 + size, 4 + size * 2),
            (20, size, 1 + size),
            (20, size, 2 + size),
            (3, 2 + size, 5 + size * 3),
            (15, size, 2 + size),
            (3, 1 + size, 4 + size),
            (3, 2 + size, 4 + size * 2),
            (15, 0, 1 + size),
            (8, size, 3 + size * 2),
        ]
        .into_iter()
        .enumerate()
        {
            let limit = inclusive(r, lo, hi);
            if limit != 0 {
                weights.push(Weight {
                    kind,
                    weight,
                    spawned: 0,
                    limit,
                });
            }
        }
        let x = (cx << 4) + 2;
        let z = (cz << 4) + 2;
        let orientation = r.next_int(4);
        let b = B::new([x, 64, z, x + 5, 78, z + 5]);
        let mut s = Self {
            pieces: vec![VillagePiece::new(10, 0, b, orientation, r)],
            bounds: b,
            sizeable: false,
            weights,
            previous: None,
            roads: Vec::new(),
            houses: Vec::new(),
            size,
        };
        for (x, z, d) in [
            (b.min_x - 1, b.min_z + 1, 1),
            (b.max_x + 1, b.min_z + 1, 3),
            (b.min_x + 1, b.min_z - 1, 2),
            (b.min_x + 1, b.max_z + 1, 0),
        ] {
            s.road(biomes, r, x, b.max_y - 4, z, d, 0);
        }
        while !s.roads.is_empty() || !s.houses.is_empty() {
            let index = if !s.roads.is_empty() {
                let n = r.next_int(s.roads.len() as i32) as usize;
                s.roads.remove(n)
            } else {
                let n = r.next_int(s.houses.len() as i32) as usize;
                s.houses.remove(n)
            };
            if s.pieces[index].kind == 11 {
                s.build_road(index, biomes, r);
            }
        }
        s.bounds = B::empty();
        let mut houses = 0;
        for p in &s.pieces {
            s.bounds.encompass(p.bounds());
            if p.kind != 11 {
                houses += 1;
            }
        }
        s.sizeable = houses > 2;
        s
    }
    fn intersects(&self, b: B) -> bool {
        self.pieces.iter().any(|p| p.bounds().intersects(b))
    }
    fn viable(b: B, biomes: &BiomeSource) -> bool {
        let x = (b.min_x + b.max_x) / 2;
        let z = (b.min_z + b.max_z) / 2;
        biomes.viable(
            x,
            z,
            (b.max_x - b.min_x).max(b.max_z - b.min_z) / 2 + 4,
            &[crate::Biome::Plains, crate::Biome::Desert],
        )
    }
    fn factory(
        &self,
        kind: usize,
        r: &mut JavaRandom,
        x: i32,
        y: i32,
        z: i32,
        d: i32,
        depth: i32,
    ) -> Option<VillagePiece> {
        let [sx, sy, sz] = SIZES[kind];
        let b = B::oriented(x, y, z, 0, 0, 0, sx, sy, sz, d);
        if (kind != 9 && b.min_y <= 10) || self.intersects(b) {
            None
        } else {
            Some(VillagePiece::new(kind, depth, b, d, r))
        }
    }
    fn choose_house(
        &mut self,
        r: &mut JavaRandom,
        x: i32,
        y: i32,
        z: i32,
        d: i32,
        depth: i32,
    ) -> Option<VillagePiece> {
        let total = if self
            .weights
            .iter()
            .any(|p| p.limit > 0 && p.spawned < p.limit)
        {
            self.weights.iter().map(|p| p.weight).sum()
        } else {
            -1
        };
        if total <= 0 {
            return None;
        }
        for _ in 0..5 {
            let mut pick = r.next_int(total);
            for i in 0..self.weights.len() {
                let p = self.weights[i].clone();
                pick -= p.weight;
                if pick < 0 {
                    if p.spawned >= p.limit
                        || (self.previous == Some(p.kind) && self.weights.len() > 1)
                    {
                        break;
                    }
                    if let Some(piece) = self.factory(p.kind, r, x, y, z, d, depth) {
                        self.weights[i].spawned += 1;
                        self.previous = Some(p.kind);
                        if self.weights[i].spawned >= p.limit {
                            self.weights.remove(i);
                        }
                        return Some(piece);
                    }
                }
            }
        }
        self.factory(9, r, x, y, z, d, depth)
    }
    fn house(
        &mut self,
        biomes: &BiomeSource,
        r: &mut JavaRandom,
        x: i32,
        y: i32,
        z: i32,
        d: i32,
        depth: i32,
    ) -> Option<usize> {
        let root = self.pieces[0].bounds();
        if depth > 50 || (x - root.min_x).abs() > 112 || (z - root.min_z).abs() > 112 {
            return None;
        }
        let p = self.choose_house(r, x, y, z, d, depth + 1)?;
        if !Self::viable(p.bounds(), biomes) {
            return None;
        }
        let i = self.pieces.len();
        self.pieces.push(p);
        self.houses.push(i);
        Some(i)
    }
    fn road(
        &mut self,
        biomes: &BiomeSource,
        r: &mut JavaRandom,
        x: i32,
        y: i32,
        z: i32,
        d: i32,
        depth: i32,
    ) -> Option<usize> {
        let root = self.pieces[0].bounds();
        if depth > 3 + self.size || (x - root.min_x).abs() > 112 || (z - root.min_z).abs() > 112 {
            return None;
        }
        let mut length = 7 * inclusive(r, 3, 5);
        while length >= 7 {
            let b = B::oriented(x, y, z, 0, 0, 0, 3, 3, length, d);
            if !self.intersects(b) {
                if b.min_y <= 10 || !Self::viable(b, biomes) {
                    return None;
                }
                let i = self.pieces.len();
                self.pieces.push(VillagePiece::new(11, depth, b, d, r));
                self.roads.push(i);
                return Some(i);
            }
            length -= 7;
        }
        None
    }
    fn build_road(&mut self, index: usize, biomes: &BiomeSource, r: &mut JavaRandom) {
        let p = self.pieces[index].clone();
        let b = p.bounds();
        let d = p.component.orientation;
        let depth = p.component.depth;
        let mut any = false;
        for side in 0..2 {
            let mut n = r.next_int(5);
            while n < p.length - 8 {
                let (x, z, dir) = match (d, side) {
                    (0 | 2, 0) => (b.min_x - 1, b.min_z + n, 1),
                    (0 | 2, _) => (b.max_x + 1, b.min_z + n, 3),
                    (1 | 3, 0) => (b.min_x + n, b.min_z - 1, 2),
                    (1 | 3, _) => (b.min_x + n, b.max_z + 1, 0),
                    _ => unreachable!(),
                };
                if let Some(i) = self.house(biomes, r, x, b.min_y, z, dir, depth) {
                    let b = self.pieces[i].bounds();
                    n += b.sizes()[0].max(b.sizes()[2]);
                    any = true;
                }
                n += 2 + r.next_int(5);
            }
        }
        if any && r.next_int(3) > 0 {
            let (x, z, dir) = match d {
                0 => (b.min_x - 1, b.max_z - 2, 1),
                1 => (b.min_x, b.min_z - 1, 2),
                2 => (b.min_x - 1, b.min_z, 1),
                _ => (b.max_x - 2, b.min_z - 1, 2),
            };
            self.road(biomes, r, x, b.min_y, z, dir, depth);
        }
        if any && r.next_int(3) > 0 {
            let (x, z, dir) = match d {
                0 => (b.max_x + 1, b.max_z - 2, 3),
                1 => (b.min_x, b.max_z + 1, 0),
                2 => (b.max_x + 1, b.min_z, 3),
                _ => (b.max_x - 2, b.max_z + 1, 0),
            };
            self.road(biomes, r, x, b.min_y, z, dir, depth);
        }
    }
}
#[derive(Default)]
pub struct VillageGenerator {
    pub starts: HashMap<i64, VillageStart>,
    order: LongMapOrder<i64>,
}
impl VillageGenerator {
    pub(crate) fn merge_start(&mut self, key: i64, start: VillageStart) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.starts.entry(key) {
            e.insert(start);
            self.order.insert((key ^ (key >> 32)) as i32, key);
        }
    }
    pub fn keys(&self) -> Vec<i64> {
        self.order.iter().collect()
    }
    pub fn generate(
        &mut self,
        seed: i64,
        cx: i32,
        cz: i32,
        biomes: &BiomeSource,
        wr: &mut JavaRandom,
    ) -> i64 {
        let mut r = JavaRandom::new(seed);
        let mx = r.next_long();
        let mz = r.next_long();
        for x in cx - 8..=cx + 8 {
            for z in cz - 8..=cz + 8 {
                let key = super::ChunkCoordIntPair { x, z }.packed();
                r.set_seed((x as i64).wrapping_mul(mx) ^ (z as i64).wrapping_mul(mz) ^ seed);
                if !self.starts.contains_key(&key) {
                    r.next_int_unbounded();
                    if super::locations::village(seed, x, z, biomes, wr) {
                        self.starts
                            .insert(key, VillageStart::new(biomes, &mut r, x, z));
                        self.order.insert((key ^ (key >> 32)) as i32, key);
                    }
                }
            }
        }
        r.state()
    }
    pub fn populate(&mut self, w: &mut World, r: &mut JavaRandom, cx: i32, cz: i32) -> bool {
        let previous_epoch = w.structure_rng_epoch.replace(w.generator_epoch);
        let x = cx * 16 + 8;
        let z = cz * 16 + 8;
        let clip = B::region(x, z, x + 15, z + 15);
        let mut placed = false;
        for key in self.order.iter() {
            let s = self.starts.get_mut(&key).unwrap();
            if s.sizeable && s.bounds.intersects_region(x, z, x + 15, z + 15) {
                s.pieces
                    .retain_mut(|p| !p.bounds().intersects(clip) || p.generate(w, r, clip));
                placed = true;
            }
        }
        super::component::random(w, r);
        w.structure_rng_epoch = previous_epoch;
        placed
    }
}
