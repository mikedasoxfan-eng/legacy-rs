//! Beta 1.8.1 stronghold factories, randomized pending graph and retained starts.
use super::{
    component::{self, Component},
    long_map_order::LongMapOrder,
    BlockSelector, BoundingBox as B, Door, Piece, StructurePieceTreasure as T,
};
use crate::{BiomeSource, JavaRandom, World};
use std::collections::HashMap;
#[path = "stronghold_generated.rs"]
mod generated;
#[path = "stronghold10.rs"]
mod release10;
const NAMES: [&str; 13] = [
    "ComponentStrongholdStraight",
    "ComponentStrongholdPrison",
    "ComponentStrongholdLeftTurn",
    "ComponentStrongholdRightTurn",
    "ComponentStrongholdRoomCrossing",
    "ComponentStrongholdStairsStraight",
    "ComponentStrongholdStairs",
    "ComponentStrongholdCrossing",
    "ComponentStrongholdLibrary",
    "ComponentStrongholdCorridor",
    "ComponentStrongholdStairs2",
    "ComponentStrongholdChestCorridor",
    "ComponentStrongholdPortalRoom",
];
const BOXES: [[i32; 6]; 9] = [
    [-1, -1, 0, 5, 5, 7],
    [-1, -1, 0, 9, 5, 11],
    [-1, -1, 0, 5, 5, 5],
    [-1, -1, 0, 5, 5, 5],
    [-4, -1, 0, 11, 7, 11],
    [-1, -7, 0, 5, 11, 8],
    [-1, -7, 0, 5, 11, 5],
    [-4, -3, 0, 10, 9, 11],
    [-4, -1, 0, 14, 11, 15],
];
const fn loot(item: i32, min: i32, max: i32, weight: i32) -> T {
    T {
        item,
        metadata: 0,
        min,
        max,
        weight,
    }
}
const ROOM_LOOT: [T; 7] = [
    loot(265, 1, 5, 10),
    loot(266, 1, 3, 5),
    loot(331, 4, 9, 5),
    loot(263, 3, 8, 10),
    loot(297, 1, 3, 15),
    loot(260, 1, 3, 15),
    loot(257, 1, 1, 1),
];
const LIBRARY_LOOT: [T; 4] = [
    loot(340, 1, 3, 20),
    loot(339, 2, 7, 20),
    loot(358, 1, 1, 1),
    loot(345, 1, 1, 1),
];
#[derive(Clone, Debug)]
pub struct StrongholdPiece {
    pub component: Component,
    pub kind: usize,
    pub door: Door,
    pub flags: [bool; 4],
    pub style: i32,
    pub length: i32,
}
impl StrongholdPiece {
    pub fn new(kind: usize, depth: i32, bounds: B, orientation: i32, r: &mut JavaRandom) -> Self {
        let door = if kind == 9 || kind == 10 || kind == 12 {
            Door::Opening
        } else {
            match r.next_int(5) {
                2 => Door::WoodDoor,
                3 => Door::Grates,
                4 => Door::IronDoor,
                _ => Door::Opening,
            }
        };
        let mut flags = [false; 4];
        match kind {
            0 => {
                flags[0] = r.next_int(2) == 0;
                flags[1] = r.next_int(2) == 0;
            }
            7 => {
                for f in &mut flags[..3] { *f = r.next_bool(); }
                flags[3] = r.next_int(3) > 0;
            }
            8 => flags[0] = bounds.sizes()[1] > 6,
            10 => flags[0] = true,
            _ => {}
        }
        let style = if kind == 4 { r.next_int(5) } else { 0 };
        let length = if orientation == 0 || orientation == 2 {
            bounds.sizes()[2]
        } else {
            bounds.sizes()[0]
        };
        Self {
            component: Component {
                bounds,
                orientation,
                depth,
            },
            kind,
            door,
            flags,
            style,
            length,
        }
    }
    pub fn name(&self) -> &'static str {
        NAMES[self.kind]
    }
    fn entrance(
        &self,
        w: &mut World,
        _r: &mut JavaRandom,
        clip: B,
        door: Door,
        x: i32,
        y: i32,
        z: i32,
    ) {
        let c = self.component;
        if door == Door::Opening {
            c.fill(w, clip, [x, y, z, x + 2, y + 2, z], 0, 0, false);
            return;
        }
        if door == Door::Grates {
            c.put(w, clip, 0, 0, x + 1, y, z);
            c.put(w, clip, 0, 0, x + 1, y + 1, z);
            for [dx, dy] in [[0, 0], [0, 1], [0, 2], [1, 2], [2, 2], [2, 1], [2, 0]] {
                c.put(w, clip, 101, 0, x + dx, y + dy, z);
            }
        } else {
            for [dx, dy] in [[0, 0], [0, 1], [0, 2], [1, 2], [2, 2], [2, 1], [2, 0]] {
                c.put(w, clip, 98, 0, x + dx, y + dy, z);
            }
            let id = if door == Door::WoodDoor { 64 } else { 71 };
            c.put(w, clip, id, 0, x + 1, y, z);
            c.put(w, clip, id, 8, x + 1, y + 1, z);
            if door == Door::IronDoor {
                c.put(w, clip, 77, c.rotate(77, 4), x + 2, y + 1, z + 1);
                c.put(w, clip, 77, c.rotate(77, 3), x + 2, y + 1, z - 1);
            }
        }
    }
    fn chest(
        &self,
        w: &mut World,
        clip: B,
        r: &mut JavaRandom,
        x: i32,
        y: i32,
        z: i32,
        loot: &[T],
        count: i32,
    ) {
        self.component.chest(w, clip, r, x, y, z, loot, count);
    }
}
impl Piece for StrongholdPiece {
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
            6 | 10 => self.generate_6(w, r, clip),
            7 => self.generate_7(w, r, clip),
            8 => self.generate_8(w, r, clip),
            9 => self.generate_9(w, r, clip),
            11 => self.generate_chest10(w, r, clip),
            12 => self.generate_portal10(w, r, clip),
            _ => unreachable!(),
        }
    }
}
pub struct StrongholdStart {
    pub pieces: Vec<StrongholdPiece>,
    pub bounds: B,
    weights: Vec<Weight>,
    pub previous: Option<usize>,
    pending: Vec<usize>,
    forced: Option<usize>,
    pub portal_room: Option<usize>,
    pub portal_center10: Option<[i32;3]>,
}
struct Weight {
    kind: usize,
    weight: i32,
    spawned: i32,
    limit: i32,
}
impl StrongholdStart {
    pub fn new(r: &mut JavaRandom, cx: i32, cz: i32) -> Self {
        let x = (cx << 4).wrapping_add(2);
        let z = (cz << 4).wrapping_add(2);
        let orientation = r.next_int(4);
        let root = StrongholdPiece::new(
            10,
            0,
            B::new([x, 64, z, x.wrapping_add(4), 74, z.wrapping_add(4)]),
            orientation,
            r,
        );
        let weights = [(0,40,0),(1,5,5),(2,20,0),(3,20,0),(4,10,6),(5,5,5),(6,5,5),(7,5,4),(11,5,4),(8,10,2),(12,20,1)]
            .into_iter().map(|(kind, weight, limit)| Weight { kind, weight, limit, spawned: 0 }).collect();
        let mut s = Self {
            pieces: vec![root],
            bounds: B::empty(),
            weights,
            previous: None,
            pending: vec![],
            forced: None,
            portal_room: None,
            portal_center10: None,
        };
        s.expand(0, r);
        while !s.pending.is_empty() {
            let i = r.next_int(s.pending.len() as i32) as usize;
            let p = s.pending.remove(i);
            s.expand(p, r);
        }
        for p in &s.pieces {
            s.bounds.encompass(p.bounds());
        }
        let mut top = s.bounds.sizes()[1].wrapping_add(1);
        if top < 53 {
            top = top.wrapping_add(r.next_int(53 - top));
        }
        let dy = top.wrapping_sub(s.bounds.max_y);
        s.bounds.offset(0, dy, 0);
        for p in &mut s.pieces {
            p.component.bounds.offset(0, dy, 0);
        }
        s.portal_center10=s.portal_room.map(|i|s.pieces[i].bounds().center());
        s
    }
    fn factory(
        &self,
        kind: usize,
        r: &mut JavaRandom,
        x: i32,
        y: i32,
        z: i32,
        dir: i32,
        depth: i32,
    ) -> Option<StrongholdPiece> {
        let [dx, dy, dz, sx, sy, sz] = match kind { 11 => [-1,-1,0,5,5,7], 12 => [-4,-1,0,11,8,16], _ => BOXES[kind] };
        let mut b = B::oriented(x, y, z, dx, dy, dz, sx, sy, sz, dir);
        let valid = |b: B| b.min_y > 10 && !self.pieces.iter().any(|p| p.bounds().intersects(b));
        if !valid(b) {
            if kind != 8 {
                return None;
            }
            b = B::oriented(x, y, z, dx, dy, dz, sx, 6, sz, dir);
            if !valid(b) {
                return None;
            }
        }
        // Vanilla invokes the inherited LeftTurn static factory for RightTurn.
        Some(StrongholdPiece::new(
            if kind == 3 { 2 } else { kind },
            depth,
            b,
            dir,
            r,
        ))
    }
    fn corridor(&self, x: i32, y: i32, z: i32, dir: i32) -> Option<B> {
        let b = B::oriented(x, y, z, -1, -1, 0, 5, 5, 4, dir);
        let p = self.pieces.iter().find(|p| p.bounds().intersects(b))?;
        if p.bounds().min_y == b.min_y {
            for n in (1..=3).rev() {
                if !p
                    .bounds()
                    .intersects(B::oriented(x, y, z, -1, -1, 0, 5, 5, n - 1, dir))
                {
                    return Some(B::oriented(x, y, z, -1, -1, 0, 5, 5, n, dir));
                }
            }
        }
        None
    }
    fn select(
        &mut self,
        r: &mut JavaRandom,
        x: i32,
        y: i32,
        z: i32,
        dir: i32,
        depth: i32,
    ) -> Option<StrongholdPiece> {
        if !self
            .weights
            .iter()
            .any(|w| w.limit > 0 && w.spawned < w.limit)
        {
            return None;
        }
        let total = self.weights.iter().map(|w| w.weight).sum();
        if let Some(kind) = self.forced.take() {
            if let Some(piece) = self.factory(kind, r, x, y, z, dir, depth) { return Some(piece); }
        }
        for _ in 0..5 {
            let mut n = r.next_int(total);
            for i in 0..self.weights.len() {
                let w = &self.weights[i];
                n -= w.weight;
                if n < 0 {
                    if (w.limit > 0 && w.spawned >= w.limit)
                        || (w.kind == 8 && depth <= 4)
                        || (w.kind == 12 && depth <= 5)
                        || self.previous == Some(w.kind)
                    {
                        break;
                    }
                    let kind = w.kind;
                    if let Some(p) = self.factory(kind, r, x, y, z, dir, depth) {
                        self.weights[i].spawned += 1;
                        self.previous = Some(kind);
                        if self.weights[i].limit != 0
                            && self.weights[i].spawned >= self.weights[i].limit
                        {
                            self.weights.remove(i);
                        }
                        return Some(p);
                    }
                }
            }
        }
        let b = self.corridor(x, y, z, dir)?;
        if b.min_y > 1 {
            Some(StrongholdPiece::new(9, depth, b, dir, r))
        } else {
            None
        }
    }
    fn attach(&mut self, r: &mut JavaRandom, x: i32, y: i32, z: i32, dir: i32, depth: i32) {
        if depth > 50
            || x.wrapping_sub(self.pieces[0].bounds().min_x).wrapping_abs() > 112
            || z.wrapping_sub(self.pieces[0].bounds().min_z).wrapping_abs() > 112
        {
            return;
        }
        if let Some(p) = self.select(r, x, y, z, dir, depth + 1) {
            self.pending.push(self.pieces.len());
            self.pieces.push(p);
        }
    }
    fn connect(&mut self, p: &StrongholdPiece, r: &mut JavaRandom, side: i32, a: i32, b: i32) {
        let c = p.component;
        let bb = c.bounds;
        let d = c.orientation;
        let (x, y, z, dir) = match (side, d) {
            (0, 0) => (
                bb.min_x.wrapping_add(a),
                bb.min_y.wrapping_add(b),
                bb.max_z.wrapping_add(1),
                d,
            ),
            (0, 1) => (
                bb.min_x.wrapping_sub(1),
                bb.min_y.wrapping_add(b),
                bb.min_z.wrapping_add(a),
                d,
            ),
            (0, 2) => (
                bb.min_x.wrapping_add(a),
                bb.min_y.wrapping_add(b),
                bb.min_z.wrapping_sub(1),
                d,
            ),
            (0, 3) => (
                bb.max_x.wrapping_add(1),
                bb.min_y.wrapping_add(b),
                bb.min_z.wrapping_add(a),
                d,
            ),
            (1, 0 | 2) => (
                bb.min_x.wrapping_sub(1),
                bb.min_y.wrapping_add(a),
                bb.min_z.wrapping_add(b),
                1,
            ),
            (1, 1 | 3) => (
                bb.min_x.wrapping_add(b),
                bb.min_y.wrapping_add(a),
                bb.min_z.wrapping_sub(1),
                2,
            ),
            (2, 0 | 2) => (
                bb.max_x.wrapping_add(1),
                bb.min_y.wrapping_add(a),
                bb.min_z.wrapping_add(b),
                3,
            ),
            (2, 1 | 3) => (
                bb.min_x.wrapping_add(b),
                bb.min_y.wrapping_add(a),
                bb.max_z.wrapping_add(1),
                0,
            ),
            _ => return,
        };
        self.attach(r, x, y, z, dir, c.depth);
    }
    fn expand(&mut self, index: usize, r: &mut JavaRandom) {
        let p = self.pieces[index].clone();
        match p.kind {
            0 => {
                self.connect(&p, r, 0, 1, 1);
                if p.flags[0] {
                    self.connect(&p, r, 1, 1, 2);
                }
                if p.flags[1] {
                    self.connect(&p, r, 2, 1, 2);
                }
            }
            1 | 5 | 6 | 11 => self.connect(&p, r, 0, 1, 1),
            10 => { self.forced = Some(7); self.connect(&p, r, 0, 1, 1); }
            12 => self.portal_room = Some(index),
            2 | 3 => {
                let normal = p.component.orientation != 2 && p.component.orientation != 3;
                let side = if normal ^ (p.kind == 3) { 2 } else { 1 };
                self.connect(&p, r, side, 1, 1);
            }
            4 => {
                self.connect(&p, r, 0, 4, 1);
                self.connect(&p, r, 1, 1, 4);
                self.connect(&p, r, 2, 1, 4);
            }
            7 => {
                self.connect(&p, r, 0, 5, 1);
                let (low, high) = if matches!(p.component.orientation, 1 | 2) { (5, 3) } else { (3, 5) };
                for (i, (side, a, b)) in [(1, low, 1), (1, high, 7), (2, low, 1), (2, high, 7)]
                    .into_iter()
                    .enumerate()
                {
                    if p.flags[i] {
                        self.connect(&p, r, side, a, b);
                    }
                }
            }
            _ => {}
        }
    }
    pub fn populate(&mut self, w: &mut World, r: &mut JavaRandom, clip: B) {
        self.pieces
            .retain_mut(|p| !p.bounds().intersects(clip) || p.generate(w, r, clip));
    }
}
#[derive(Default)]
pub struct StrongholdGenerator {
    pub starts: HashMap<i64, StrongholdStart>,
    order: LongMapOrder<i64>,
    pub candidates: Option<[(i32, i32); 3]>,
}
impl StrongholdGenerator {
    pub(crate) fn merge_start(&mut self, key: i64, start: StrongholdStart) {
        if let std::collections::hash_map::Entry::Vacant(e) = self.starts.entry(key) {
            e.insert(start);
            self.order
                .insert((key ^ ((key as u64 >> 32) as i64)) as i32, key);
        }
    }
    pub fn keys(&self) -> Vec<i64> {
        self.order.iter().collect()
    }
    pub fn generate(&mut self, seed: i64, cx: i32, cz: i32, biomes: &BiomeSource) -> i64 {
        let mut r = JavaRandom::new(seed);
        let mx = r.next_long();
        let mz = r.next_long();
        for x in cx.wrapping_sub(8)..=cx.wrapping_add(8) {
            for z in cz.wrapping_sub(8)..=cz.wrapping_add(8) {
                let key = super::ChunkCoordIntPair { x, z }.packed();
                r.set_seed((x as i64).wrapping_mul(mx) ^ (z as i64).wrapping_mul(mz) ^ seed);
                if !self.starts.contains_key(&key) {
                    r.next_int_unbounded();
                    if self.candidates.is_none() {
                        self.candidates = Some(super::locations::strongholds(seed, biomes, &mut r));
                    }
                    if self.candidates.unwrap().contains(&(x, z)) {
                        loop {
                            let s = StrongholdStart::new(&mut r, x, z);
                            if !s.pieces.is_empty() && s.portal_room.is_some() { self.merge_start(key, s); break; }
                        }
                    }
                }
            }
        }
        r.state()
    }
    pub fn contains(&self,x:i32,y:i32,z:i32)->bool {
        self.starts.values().any(|s|s.bounds.intersects_region(x,z,x,z)&&s.pieces.iter().any(|p|p.bounds().contains(x,y,z)))
    }
    /// MapGenStructure.func_40484_a retains its original integer-distance bug.
    pub fn nearest(&mut self,seed:i64,biomes:&BiomeSource,x:i32,y:i32,z:i32)->Option<[i32;3]> {
        let mut r=JavaRandom::new(seed);let mx=r.next_long();let mz=r.next_long();
        let (cx,cz)=(x>>4,z>>4);let key=super::ChunkCoordIntPair{x:cx,z:cz}.packed();
        if !self.starts.contains_key(&key) {
            r.set_seed((cx as i64).wrapping_mul(mx)^(cz as i64).wrapping_mul(mz)^seed);r.next_int_unbounded();
            if self.candidates.is_none() {self.candidates=Some(super::locations::strongholds(seed,biomes,&mut r));}
            if self.candidates.unwrap().contains(&(cx,cz)) {loop {
                let s=StrongholdStart::new(&mut r,cx,cz);
                if !s.pieces.is_empty()&&s.portal_room.is_some() {self.merge_start(key,s);break;}
            }}
        }
        let distance=|p:[i32;3]| {let dx=p[0].wrapping_sub(x);let dy=p[1].wrapping_sub(y);let dz=p[2].wrapping_sub(z);
            dx.wrapping_add(dx.wrapping_mul(dy).wrapping_mul(dy)).wrapping_add(dz.wrapping_mul(dz)) as f64};
        let mut best=None;let mut score=f64::MAX;
        for key in self.order.iter() {let s=&self.starts[&key];let p=s.portal_center10.unwrap_or_else(||s.pieces[0].bounds().center());let d=distance(p);
            if d<score {score=d;best=Some(p);}
        }
        if best.is_some() {return best;}
        if let Some(candidates)=self.candidates {for (cx,cz) in candidates {
            let p=super::ChunkCoordIntPair{x:cx,z:cz}.center(64);let d=distance(p);if d<score {score=d;best=Some(p);}
        }}
        best
    }
    pub fn populate(&mut self, w: &mut World, r: &mut JavaRandom, cx: i32, cz: i32) -> bool {
        let epoch = w.structure_rng_epoch.replace(w.generator_epoch);
        let x = (cx << 4).wrapping_add(8);
        let z = (cz << 4).wrapping_add(8);
        let clip = B::region(x, z, x.wrapping_add(15), z.wrapping_add(15));
        let mut placed = false;
        for key in self.order.iter() {
            let s = self.starts.get_mut(&key).unwrap();
            if s.bounds
                .intersects_region(x, z, x.wrapping_add(15), z.wrapping_add(15))
            {
                s.populate(w, r, clip);
                placed = true;
            }
        }
        component::random(w, r);
        w.structure_rng_epoch = epoch;
        placed
    }
}
