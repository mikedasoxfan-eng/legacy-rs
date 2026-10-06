//! Literal inclusive bounding-box operations from `StructureBoundingBox`.
use crate::{weighted::Weighted, JavaRandom};
pub mod component;
pub mod locations;
pub(crate) mod long_map_order;
pub mod mineshaft;
pub mod nether10;
pub mod stronghold;
pub mod verify;
pub mod village;
use std::collections::HashMap;

/// `MapGenStructure`'s retained start map, in Java HashMap value iteration order.
#[derive(Default)]
pub struct MineshaftGenerator {
    pub starts: HashMap<i64, (Vec<mineshaft::MineshaftPiece>, BoundingBox)>,
    order: long_map_order::LongMapOrder<i64>,
}
impl MineshaftGenerator {
    pub(crate) fn merge_start(
        &mut self,
        key: i64,
        start: (Vec<mineshaft::MineshaftPiece>, BoundingBox),
    ) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.starts.entry(key) {
            e.insert(start);
            self.order
                .insert((key ^ ((key as u64 >> 32) as i64)) as i32, key);
        }
    }
    pub fn keys(&self) -> Vec<i64> {
        self.order.iter().collect()
    }
    pub fn generate(&mut self, seed: i64, cx: i32, cz: i32) -> i64 {
        let mut r = JavaRandom::new(seed);
        let mx = r.next_long();
        let mz = r.next_long();
        for x in cx.wrapping_sub(8)..=cx.wrapping_add(8) {
            for z in cz.wrapping_sub(8)..=cz.wrapping_add(8) {
                let key = ChunkCoordIntPair { x, z }.packed();
                r.set_seed((x as i64).wrapping_mul(mx) ^ (z as i64).wrapping_mul(mz) ^ seed);
                if !self.starts.contains_key(&key) {
                    r.next_int_unbounded();
                    if r.next_int(100) == 0
                        && r.next_int(80) < x.wrapping_abs().max(z.wrapping_abs())
                    {
                        let start = mineshaft::MineshaftPiece::start(&mut r, x, z);
                        self.starts.insert(key, start);
                        self.order
                            .insert((key ^ ((key as u64 >> 32) as i64)) as i32, key);
                    }
                }
            }
        }
        r.state()
    }
    pub fn populate(&mut self, w: &mut crate::World, r: &mut JavaRandom, cx: i32, cz: i32) -> bool {
        let previous_epoch = w.structure_rng_epoch.replace(w.generator_epoch);
        let x = (cx << 4).wrapping_add(8);
        let z = (cz << 4).wrapping_add(8);
        let clip = BoundingBox::region(x, z, x.wrapping_add(15), z.wrapping_add(15));
        let mut placed = false;
        for key in self.order.iter() {
            let (pieces, bounds) = self.starts.get_mut(&key).unwrap();
            if bounds.intersects_region(x, z, x.wrapping_add(15), z.wrapping_add(15)) {
                pieces.retain_mut(|p| !p.bounds().intersects(clip) || p.generate(w, r, clip));
                placed = true;
            }
        }
        component::random(w, r);
        w.structure_rng_epoch = previous_epoch;
        placed
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChunkCoordIntPair {
    pub x: i32,
    pub z: i32,
}
impl ChunkCoordIntPair {
    pub fn packed(self) -> i64 {
        ((self.x as u32 as u64) | ((self.z as u32 as u64) << 32)) as i64
    }
    pub fn center(self,y:i32)->[i32;3] { [(self.x<<4).wrapping_add(8),y,(self.z<<4).wrapping_add(8)] }
    pub fn java_hash(self) -> i32 {
        self.x ^ self.z
    }
}
#[derive(Clone, Copy, Debug)]
pub struct StructurePieceTreasure {
    pub item: i32,
    pub metadata: i32,
    pub min: i32,
    pub max: i32,
    pub weight: i32,
}
impl Weighted for StructurePieceTreasure {
    fn weight(&self) -> i32 {
        self.weight
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct PieceWeight {
    pub piece_type: &'static str,
    pub weight: i32,
    pub spawned: i32,
    pub limit: i32,
}

/// Components supplied by village, stronghold and mineshaft piece implementations.
pub trait Piece {
    fn bounds(&self) -> BoundingBox;
    fn bounds_mut(&mut self) -> &mut BoundingBox;
    fn component_type(&self) -> i32;
    fn generate(
        &mut self,
        world: &mut crate::World,
        rand: &mut JavaRandom,
        clip: BoundingBox,
    ) -> bool;
}
#[derive(Default)]
pub struct StructureStart {
    pub pieces: Vec<Box<dyn Piece>>,
    pub bounding_box: Option<BoundingBox>,
}
impl StructureStart {
    pub fn update_bounding_box(&mut self) {
        let mut bounds = BoundingBox::empty();
        for p in &self.pieces {
            bounds.encompass(p.bounds());
        }
        self.bounding_box = Some(bounds);
    }
    pub fn generate(&mut self, w: &mut crate::World, r: &mut JavaRandom, clip: BoundingBox) {
        self.pieces
            .retain_mut(|p| !p.bounds().intersects(clip) || p.generate(w, r, clip));
    }
    pub fn move_below_sea_level(&mut self, r: &mut JavaRandom, margin: i32) {
        let ceiling = 63i32.wrapping_sub(margin);
        let bounds = self
            .bounding_box
            .as_mut()
            .expect("uninitialized structure bounds");
        let mut top = bounds.sizes()[1].wrapping_add(1);
        if top < ceiling {
            top = top.wrapping_add(r.next_int(ceiling.wrapping_sub(top)));
        }
        let dy = top.wrapping_sub(bounds.max_y);
        bounds.offset(0, dy, 0);
        for p in &mut self.pieces {
            p.bounds_mut().offset(0, dy, 0);
        }
    }
    pub fn move_to_height10(&mut self,r:&mut JavaRandom,min:i32,max:i32) {
        let bounds=self.bounding_box.as_mut().expect("uninitialized structure bounds");
        let room=max.wrapping_sub(min).wrapping_add(1).wrapping_sub(bounds.sizes()[1]);
        let y=if room>1 {min.wrapping_add(r.next_int(room))} else {min};
        let dy=y.wrapping_sub(bounds.min_y);bounds.offset(0,dy,0);
        for p in &mut self.pieces {p.bounds_mut().offset(0,dy,0);}
    }
    pub fn sizeable(&self) -> bool {
        true
    }
}
impl PieceWeight {
    pub fn can_spawn(self, _depth: i32) -> bool {
        self.limit == 0 || self.spawned < self.limit
    }
    pub fn can_spawn_library(self, depth: i32) -> bool {
        self.can_spawn(depth) && depth > 4
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(i32)]
pub enum Door {
    Opening,
    WoodDoor,
    Grates,
    IronDoor,
}
impl Door {
    pub fn switch_index(self) -> i32 {
        self as i32 + 1
    }
}
#[derive(Clone, Copy, Debug, Default)]
pub struct BlockSelector {
    pub block: i32,
    pub metadata: i32,
}
impl BlockSelector {
    pub fn stronghold_stone(&mut self, r: &mut JavaRandom, _x: i32, _y: i32, _z: i32, wall: bool) {
        if !wall {
            self.block = 0;
            self.metadata = 0;
        } else {
            self.block = 98;
            let f = r.next_float();
            if f < 0.2 {
                self.metadata = 2;
            } else if f < 0.5 {
                self.metadata = 1;
            } else if f < 0.55 {
                self.block = 97;
                self.metadata = 2;
            } else {
                self.metadata = 0;
            }
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct BoundingBox {
    pub min_x: i32,
    pub min_y: i32,
    pub min_z: i32,
    pub max_x: i32,
    pub max_y: i32,
    pub max_z: i32,
}
impl BoundingBox {
    pub fn new(v: [i32; 6]) -> Self {
        Self {
            min_x: v[0],
            min_y: v[1],
            min_z: v[2],
            max_x: v[3],
            max_y: v[4],
            max_z: v[5],
        }
    }
    pub fn values(self) -> [i32; 6] {
        [
            self.min_x, self.min_y, self.min_z, self.max_x, self.max_y, self.max_z,
        ]
    }
    pub fn empty() -> Self {
        Self::new([i32::MAX, i32::MAX, i32::MAX, i32::MIN, i32::MIN, i32::MIN])
    }
    pub fn region(x: i32, z: i32, x1: i32, z1: i32) -> Self {
        Self::new([x, 0, z, x1, 65536, z1])
    }
    pub fn oriented(
        x: i32,
        y: i32,
        z: i32,
        dx: i32,
        dy: i32,
        dz: i32,
        sx: i32,
        sy: i32,
        sz: i32,
        dir: i32,
    ) -> Self {
        // Every arithmetic operation is a Java int operation.
        let a = |x: i32, y: i32| x.wrapping_add(y);
        let sub = |x: i32, y: i32| x.wrapping_sub(y);
        let ymax = a(a(y, sy.wrapping_sub(1)), dy);
        match dir {
            1 => Self::new([
                a(a(sub(x, sz), 1), dz),
                a(y, dy),
                a(z, dx),
                a(x, dz),
                ymax,
                a(a(z, sx.wrapping_sub(1)), dx),
            ]),
            2 => Self::new([
                a(x, dx),
                a(y, dy),
                a(a(sub(z, sz), 1), dz),
                a(a(x, sx.wrapping_sub(1)), dx),
                ymax,
                a(z, dz),
            ]),
            3 => Self::new([
                a(x, dz),
                a(y, dy),
                a(z, dx),
                a(a(x, sz.wrapping_sub(1)), dz),
                ymax,
                a(a(z, sx.wrapping_sub(1)), dx),
            ]),
            _ => Self::new([
                a(x, dx),
                a(y, dy),
                a(z, dz),
                a(a(x, sx.wrapping_sub(1)), dx),
                ymax,
                a(a(z, sz.wrapping_sub(1)), dz),
            ]),
        }
    }
    pub fn intersects(self, b: Self) -> bool {
        self.max_x >= b.min_x
            && self.min_x <= b.max_x
            && self.max_z >= b.min_z
            && self.min_z <= b.max_z
            && self.max_y >= b.min_y
            && self.min_y <= b.max_y
    }
    pub fn intersects_region(self, x: i32, z: i32, x1: i32, z1: i32) -> bool {
        self.max_x >= x && self.min_x <= x1 && self.max_z >= z && self.min_z <= z1
    }
    pub fn encompass(&mut self, b: Self) {
        self.min_x = self.min_x.min(b.min_x);
        self.min_y = self.min_y.min(b.min_y);
        self.min_z = self.min_z.min(b.min_z);
        self.max_x = self.max_x.max(b.max_x);
        self.max_y = self.max_y.max(b.max_y);
        self.max_z = self.max_z.max(b.max_z);
    }
    pub fn offset(&mut self, x: i32, y: i32, z: i32) {
        self.min_x = self.min_x.wrapping_add(x);
        self.max_x = self.max_x.wrapping_add(x);
        self.min_y = self.min_y.wrapping_add(y);
        self.max_y = self.max_y.wrapping_add(y);
        self.min_z = self.min_z.wrapping_add(z);
        self.max_z = self.max_z.wrapping_add(z);
    }
    pub fn contains(self, x: i32, y: i32, z: i32) -> bool {
        x >= self.min_x
            && x <= self.max_x
            && z >= self.min_z
            && z <= self.max_z
            && y >= self.min_y
            && y <= self.max_y
    }
    pub fn center(self)->[i32;3] {
        let [x,y,z]=self.sizes();
        [self.min_x.wrapping_add(x/2),self.min_y.wrapping_add(y/2),self.min_z.wrapping_add(z/2)]
    }
    pub fn sizes(self) -> [i32; 3] {
        [
            self.max_x.wrapping_sub(self.min_x).wrapping_add(1),
            self.max_y.wrapping_sub(self.min_y).wrapping_add(1),
            self.max_z.wrapping_sub(self.min_z).wrapping_add(1),
        ]
    }
}
impl std::fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({}, {}, {}; {}, {}, {})",
            self.min_x, self.min_y, self.min_z, self.max_x, self.max_y, self.max_z
        )
    }
}
