//! AxisAlignedBB / Vec3D. MathHelper's square roots deliberately narrow to float.
use crate::math;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Vec3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3D {
    /// Pooled setComponents preserves negative zero (unlike the Java constructor).
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn helper(x: f64, y: f64, z: f64) -> Self {
        Self::new(
            if x == 0.0 { 0.0 } else { x },
            if y == 0.0 { 0.0 } else { y },
            if z == 0.0 { 0.0 } else { z },
        )
    }
    /// Vanilla subtract is argument minus receiver.
    pub fn subtract(self, other: Self) -> Self {
        Self::new(other.x - self.x, other.y - self.y, other.z - self.z)
    }
    pub fn add(self, x: f64, y: f64, z: f64) -> Self {
        Self::new(self.x + x, self.y + y, self.z + z)
    }
    pub fn normalize(self) -> Self {
        let n = self.length();
        if n < 1.0e-4 {
            Self::default()
        } else {
            Self::new(self.x / n, self.y / n, self.z / n)
        }
    }
    pub fn cross(self, v: Self) -> Self {
        Self::new(
            self.y * v.z - self.z * v.y,
            self.z * v.x - self.x * v.z,
            self.x * v.y - self.y * v.x,
        )
    }
    pub fn square_distance(self, v: Self) -> f64 {
        let d = self.subtract(v);
        d.x * d.x + d.y * d.y + d.z * d.z
    }
    pub fn distance(self, v: Self) -> f64 {
        (self.square_distance(v).sqrt() as f32) as f64
    }
    pub fn length(self) -> f64 {
        ((self.x * self.x + self.y * self.y + self.z * self.z).sqrt() as f32) as f64
    }
    pub fn intermediate(self, v: Self, axis: usize, value: f64) -> Option<Self> {
        let d = self.subtract(v);
        let delta = [d.x, d.y, d.z][axis];
        if delta * delta < 1.0e-7f32 as f64 {
            return None;
        }
        let t = (value - [self.x, self.y, self.z][axis]) / delta;
        if t >= 0.0 && t <= 1.0 {
            Some(self.add(d.x * t, d.y * t, d.z * t))
        } else {
            None
        }
    }
    pub fn rotate_x(&mut self, angle: f32) {
        let c = math::cos(angle) as f64;
        let s = math::sin(angle) as f64;
        let y = self.y * c + self.z * s;
        let z = self.z * c - self.y * s;
        self.y = y;
        self.z = z;
    }
    pub fn rotate_y(&mut self, angle: f32) {
        let c = math::cos(angle) as f64;
        let s = math::sin(angle) as f64;
        let x = self.x * c + self.z * s;
        let z = self.z * c - self.x * s;
        self.x = x;
        self.z = z;
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AxisAlignedBB {
    pub min_x: f64,
    pub min_y: f64,
    pub min_z: f64,
    pub max_x: f64,
    pub max_y: f64,
    pub max_z: f64,
}
impl AxisAlignedBB {
    pub const fn new(
        min_x: f64,
        min_y: f64,
        min_z: f64,
        max_x: f64,
        max_y: f64,
        max_z: f64,
    ) -> Self {
        Self {
            min_x,
            min_y,
            min_z,
            max_x,
            max_y,
            max_z,
        }
    }
    pub fn add_coord(self, x: f64, y: f64, z: f64) -> Self {
        let mut b = self;
        if x < 0.0 {
            b.min_x += x;
        }
        if x > 0.0 {
            b.max_x += x;
        }
        if y < 0.0 {
            b.min_y += y;
        }
        if y > 0.0 {
            b.max_y += y;
        }
        if z < 0.0 {
            b.min_z += z;
        }
        if z > 0.0 {
            b.max_z += z;
        }
        b
    }
    pub fn expand(self, x: f64, y: f64, z: f64) -> Self {
        Self::new(
            self.min_x - x,
            self.min_y - y,
            self.min_z - z,
            self.max_x + x,
            self.max_y + y,
            self.max_z + z,
        )
    }
    pub fn contract(self, x: f64, y: f64, z: f64) -> Self {
        Self::new(
            self.min_x + x,
            self.min_y + y,
            self.min_z + z,
            self.max_x - x,
            self.max_y - y,
            self.max_z - z,
        )
    }
    pub fn translated(self, x: f64, y: f64, z: f64) -> Self {
        Self::new(
            self.min_x + x,
            self.min_y + y,
            self.min_z + z,
            self.max_x + x,
            self.max_y + y,
            self.max_z + z,
        )
    }
    pub fn offset(&mut self, x: f64, y: f64, z: f64) {
        *self = self.translated(x, y, z);
    }
    pub fn intersects(self, b: Self) -> bool {
        b.max_x > self.min_x
            && b.min_x < self.max_x
            && b.max_y > self.min_y
            && b.min_y < self.max_y
            && b.max_z > self.min_z
            && b.min_z < self.max_z
    }
    pub fn contains(self, v: Vec3D) -> bool {
        v.x > self.min_x
            && v.x < self.max_x
            && v.y > self.min_y
            && v.y < self.max_y
            && v.z > self.min_z
            && v.z < self.max_z
    }
    pub fn average_edge_length(self) -> f64 {
        ((self.max_x - self.min_x) + (self.max_y - self.min_y) + (self.max_z - self.min_z)) / 3.0
    }
    pub fn calculate_x_offset(self, b: Self, mut delta: f64) -> f64 {
        if b.max_y > self.min_y
            && b.min_y < self.max_y
            && b.max_z > self.min_z
            && b.min_z < self.max_z
        {
            if delta > 0.0 && b.max_x <= self.min_x {
                let d = self.min_x - b.max_x;
                if d < delta {
                    delta = d;
                }
            }
            if delta < 0.0 && b.min_x >= self.max_x {
                let d = self.max_x - b.min_x;
                if d > delta {
                    delta = d;
                }
            }
        }
        delta
    }
    pub fn calculate_y_offset(self, b: Self, mut delta: f64) -> f64 {
        if b.max_x > self.min_x
            && b.min_x < self.max_x
            && b.max_z > self.min_z
            && b.min_z < self.max_z
        {
            if delta > 0.0 && b.max_y <= self.min_y {
                let d = self.min_y - b.max_y;
                if d < delta {
                    delta = d;
                }
            }
            if delta < 0.0 && b.min_y >= self.max_y {
                let d = self.max_y - b.min_y;
                if d > delta {
                    delta = d;
                }
            }
        }
        delta
    }
    pub fn calculate_z_offset(self, b: Self, mut delta: f64) -> f64 {
        if b.max_x > self.min_x
            && b.min_x < self.max_x
            && b.max_y > self.min_y
            && b.min_y < self.max_y
        {
            if delta > 0.0 && b.max_z <= self.min_z {
                let d = self.min_z - b.max_z;
                if d < delta {
                    delta = d;
                }
            }
            if delta < 0.0 && b.min_z >= self.max_z {
                let d = self.max_z - b.min_z;
                if d > delta {
                    delta = d;
                }
            }
        }
        delta
    }
    pub fn ray_trace(self, start: Vec3D, end: Vec3D) -> Option<MovingObjectPosition> {
        let mut best: Option<(Vec3D, i32)> = None;
        for (axis, plane, side) in [
            (0, self.min_x, 4),
            (0, self.max_x, 5),
            (1, self.min_y, 0),
            (1, self.max_y, 1),
            (2, self.min_z, 2),
            (2, self.max_z, 3),
        ] {
            if let Some(v) = start.intermediate(end, axis, plane) {
                let inside = match axis {
                    0 => {
                        v.y >= self.min_y
                            && v.y <= self.max_y
                            && v.z >= self.min_z
                            && v.z <= self.max_z
                    }
                    1 => {
                        v.x >= self.min_x
                            && v.x <= self.max_x
                            && v.z >= self.min_z
                            && v.z <= self.max_z
                    }
                    _ => {
                        v.x >= self.min_x
                            && v.x <= self.max_x
                            && v.y >= self.min_y
                            && v.y <= self.max_y
                    }
                };
                if inside
                    && best.map_or(true, |(b, _)| {
                        start.square_distance(v) < start.square_distance(b)
                    })
                {
                    best = Some((v, side));
                }
            }
        }
        best.map(|(hit, side)| MovingObjectPosition::Block {
            x: 0,
            y: 0,
            z: 0,
            side,
            hit,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MovingObjectPosition {
    Block {
        x: i32,
        y: i32,
        z: i32,
        side: i32,
        hit: Vec3D,
    },
    Entity {
        id: i32,
        hit: Vec3D,
    },
}

/// A handle preserves observable Java pool aliasing across reset/reuse. Long-lived
/// Entity.boundingBox uses an owned box, exactly as getBoundingBox does in Java.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PoolHandle(pub usize);
#[derive(Clone, Debug)]
pub struct GeometryPool<T> {
    values: Vec<T>,
    used: usize,
}
impl<T: Copy> Default for GeometryPool<T> {
    fn default() -> Self {
        Self {
            values: Vec::new(),
            used: 0,
        }
    }
}
impl<T: Copy> GeometryPool<T> {
    pub fn reset(&mut self) {
        self.used = 0;
    }
    pub fn clear(&mut self) {
        self.values.clear();
        self.used = 0;
    }
    pub fn allocate(&mut self, value: T) -> PoolHandle {
        let i = self.used;
        self.used += 1;
        if i == self.values.len() {
            self.values.push(value);
        } else {
            self.values[i] = value;
        }
        PoolHandle(i)
    }
    pub fn get(&self, h: PoolHandle) -> T {
        self.values[h.0]
    }
    pub fn get_mut(&mut self, h: PoolHandle) -> &mut T {
        &mut self.values[h.0]
    }
}
