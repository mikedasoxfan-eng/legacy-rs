//! Pathfinder, including the vanilla point-hash aliasing and heap tie breaks.
use super::{Entity, Vec3D};
use crate::{
    block::*,
    math::{floor_double as floor, floor_float},
    World,
};
use std::collections::HashMap;
#[derive(Clone, Debug)]
pub struct PathEntity {
    pub points: Vec<[i32; 3]>,
    pub index: usize,
}
impl PathEntity {
    pub fn position(&self, width: f32) -> Option<Vec3D> {
        self.points.get(self.index).map(|p| {
            let offset = (width + 1.0) as i32 as f64 * 0.5;
            Vec3D::new(p[0] as f64 + offset, p[1] as f64, p[2] as f64 + offset)
        })
    }
}
#[derive(Clone, Debug)]
struct Point {
    pos: [i32; 3],
    index: i32,
    total: f32,
    next: f32,
    target: f32,
    previous: Option<usize>,
    first: bool,
}
struct Finder<'a> {
    world: &'a mut World,
    points: Vec<Point>,
    map: HashMap<i32, usize>,
    heap: Vec<usize>,
    size: [i32; 3],
    cache: [i32; 4],
    direct:bool,
}
fn hash([x, y, z]: [i32; 3]) -> i32 {
    (y & 255)
        | ((x & 32767) << 8)
        | (z & 32767).wrapping_shl(24)
        | if x < 0 { i32::MIN } else { 0 }
        | if z < 0 { 32768 } else { 0 }
}
impl Finder<'_> {
    fn open(&mut self, pos: [i32; 3]) -> usize {
        let h = hash(pos);
        if let Some(&i) = self.map.get(&h) {
            return i;
        }
        let i = self.points.len();
        self.points.push(Point {
            pos,
            index: -1,
            total: 0.0,
            next: 0.0,
            target: 0.0,
            previous: None,
            first: false,
        });
        self.map.insert(h, i);
        i
    }
    fn distance(&self, a: usize, b: usize) -> f32 {
        let a = self.points[a].pos;
        let b = self.points[b].pos;
        let (x, y, z) = (
            (b[0] - a[0]) as f32,
            (b[1] - a[1]) as f32,
            (b[2] - a[2]) as f32,
        );
        ((x * x + y * y + z * z) as f64).sqrt() as f32
    }
    fn back(&mut self, mut index: usize) {
        let point = self.heap[index];
        let distance = self.points[point].target;
        while index > 0 {
            let parent = (index - 1) >> 1;
            let p = self.heap[parent];
            if distance >= self.points[p].target {
                break;
            }
            self.heap[index] = p;
            self.points[p].index = index as i32;
            index = parent;
        }
        self.heap[index] = point;
        self.points[point].index = index as i32;
    }
    fn forward(&mut self, mut index: usize) {
        let point = self.heap[index];
        let distance = self.points[point].target;
        loop {
            let left = 1 + (index << 1);
            let right = left + 1;
            if left >= self.heap.len() {
                break;
            }
            let ld = self.points[self.heap[left]].target;
            let rd = if right >= self.heap.len() {
                f32::INFINITY
            } else {
                self.points[self.heap[right]].target
            };
            let child = if ld < rd { left } else { right };
            let cd = if ld < rd { ld } else { rd };
            if cd >= distance {
                break;
            }
            let p = self.heap[child];
            self.heap[index] = p;
            self.points[p].index = index as i32;
            index = child;
        }
        self.heap[index] = point;
        self.points[point].index = index as i32;
    }
    fn add(&mut self, p: usize) {
        let i = self.heap.len();
        self.heap.push(p);
        self.points[p].index = i as i32;
        self.back(i);
    }
    fn dequeue(&mut self) -> usize {
        let p = self.heap[0];
        let last = self.heap.pop().unwrap();
        if !self.heap.is_empty() {
            self.heap[0] = last;
            self.forward(0);
        }
        self.points[p].index = -1;
        p
    }
    fn vertical(&mut self, [x, y, z]: [i32; 3]) -> i32 {
        for x in x..x + self.size[0] {
            for y in y..y + self.size[1] {
                for z in z..z + self.size[2] {
                    if !(0..128).contains(&y)
                        || x >> 4 < self.cache[0]
                        || x >> 4 > self.cache[1]
                        || z >> 4 < self.cache[2]
                        || z >> 4 > self.cache[3]
                    {
                        continue;
                    }
                    let (id,meta)=if self.direct {(self.world.get_block_id(x,y,z),self.world.get_block_metadata(x,y,z))}else {let Some(c)=self.world.chunks.get(&(x>>4,z>>4)) else{continue;};(c.block(x&15,y,z&15),c.data.get(x&15,y,z&15))};
                    if id == 0 {
                        continue;
                    }
                    if matches!(id, DOOR_STEEL | DOOR_WOOD) {
                        if meta & 4 == 0 {
                            return 0;
                        }
                    } else {
                        let m = material_of(id);
                        if mat(m).blocks_movement {
                            return 0;
                        }
                        if m == MAT_WATER {
                            return -1;
                        }
                        if m == MAT_LAVA {
                            return -2;
                        }
                    }
                }
            }
        }
        1
    }
    fn safe(&mut self, mut p: [i32; 3], step: i32) -> Option<usize> {
        let mut result = if self.vertical(p) == 1 {
            Some(self.open(p))
        } else {
            None
        };
        if result.is_none() && step > 0 && self.vertical([p[0], p[1] + step, p[2]]) == 1 {
            p[1] += step;
            result = Some(self.open(p));
        }
        if result.is_some() {
            let mut falls = 0;
            let mut v = 0;
            while p[1] > 0 {
                v = self.vertical([p[0], p[1] - 1, p[2]]);
                if v != 1 {
                    break;
                }
                falls += 1;
                if falls >= 4 {
                    return None;
                }
                p[1] -= 1;
                if p[1] > 0 {
                    result = Some(self.open(p));
                }
            }
            if v == -2 {
                return None;
            }
        }
        result
    }
    fn create_path(&self, mut p: usize) -> PathEntity {
        let mut points = Vec::new();
        loop {
            points.push(self.points[p].pos);
            if let Some(prev) = self.points[p].previous {
                p = prev;
            } else {
                break;
            }
        }
        points.reverse();
        PathEntity { points, index: 0 }
    }
    fn find(&mut self, start: usize, end: usize, range: f32) -> Option<PathEntity> {
        self.points[start].total = 0.0;
        self.points[start].next = self.distance(start, end);
        self.points[start].target = self.points[start].next;
        self.add(start);
        let mut closest = start;
        while !self.heap.is_empty() {
            let p = self.dequeue();
            if self.points[p].pos == self.points[end].pos {
                return Some(self.create_path(end));
            }
            if self.distance(p, end) < self.distance(closest, end) {
                closest = p;
            }
            self.points[p].first = true;
            let [x, y, z] = self.points[p].pos;
            let step = i32::from(self.vertical([x, y + 1, z]) == 1);
            let mut options = Vec::new();
            // All safe points are opened before filtering, in exactly this order.
            for coord in [[x, y, z + 1], [x - 1, y, z], [x + 1, y, z], [x, y, z - 1]] {
                options.push(self.safe(coord, step));
            }
            for q in options.into_iter().flatten() {
                if self.points[q].first || self.distance(q, end) >= range {
                    continue;
                }
                let total = self.points[p].total + self.distance(p, q);
                if self.points[q].index < 0 || total < self.points[q].total {
                    self.points[q].previous = Some(p);
                    self.points[q].total = total;
                    self.points[q].next = self.distance(q, end);
                    let old = self.points[q].target;
                    self.points[q].target = total + self.points[q].next;
                    if self.points[q].index >= 0 {
                        let i = self.points[q].index as usize;
                        if self.points[q].target < old {
                            self.back(i);
                        } else {
                            self.forward(i);
                        }
                    } else {
                        self.add(q);
                    }
                }
            }
        }
        if closest == start {
            None
        } else {
            Some(self.create_path(closest))
        }
    }
}
impl World {
    /// Pathfinder(World), used by sleep spawning without a ChunkCache.
    pub(crate) fn entity_path_world(&mut self,e:&Entity,target:&Entity,range:f32)->Option<PathEntity> {
        let mut f=Finder{world:self,points:Vec::new(),map:HashMap::new(),heap:Vec::new(),size:[floor_float(e.width+1.0),floor_float(e.height+1.0),floor_float(e.width+1.0)],cache:[i32::MIN,i32::MAX,i32::MIN,i32::MAX],direct:true};
        let start=f.open([floor(e.bounding_box.min_x),floor(e.bounding_box.min_y),floor(e.bounding_box.min_z)]);
        let end=f.open([floor(target.pos.x-(e.width/2.0) as f64),floor(target.bounding_box.min_y),floor(target.pos.z-(e.width/2.0) as f64)]);f.find(start,end,range)
    }
    pub fn entity_path_to_xyz(
        &mut self,
        e: &Entity,
        x: i32,
        y: i32,
        z: i32,
        range: f32,
    ) -> Option<PathEntity> {
        self.entity_path(
            e,
            Vec3D::new(
                (x as f32 + 0.5) as f64,
                (y as f32 + 0.5) as f64,
                (z as f32 + 0.5) as f64,
            ),
            range,
            8.0,
        )
    }
    pub fn entity_path(
        &mut self,
        e: &Entity,
        target: Vec3D,
        range: f32,
        padding: f32,
    ) -> Option<PathEntity> {
        let x = floor(e.pos.x);
        let z = floor(e.pos.z);
        let r = (range + padding) as i32;
        let cache = [(x - r) >> 4, (x + r) >> 4, (z - r) >> 4, (z + r) >> 4];
        for x in cache[0]..=cache[1] {
            for z in cache[2]..=cache[3] {
                self.provide_chunk(x, z);
            }
        }
        let mut finder = Finder {
            world: self,
            points: Vec::new(),
            map: HashMap::new(),
            heap: Vec::new(),
            size: [
                floor_float(e.width + 1.0),
                floor_float(e.height + 1.0),
                floor_float(e.width + 1.0),
            ],
            cache,
            direct:false,
        };
        let start = finder.open([
            floor(e.bounding_box.min_x),
            floor(e.bounding_box.min_y),
            floor(e.bounding_box.min_z),
        ]);
        let end = finder.open([
            floor(target.x - (e.width / 2.0) as f64),
            floor(target.y),
            floor(target.z - (e.width / 2.0) as f64),
        ]);
        finder.find(start, end, range)
    }
    pub fn get_light_brightness(&mut self, x: i32, y: i32, z: i32) -> f32 {
        let light = self.get_block_light_value(x, y, z);
        self.light_brightness_table[light as usize]
    }
}
