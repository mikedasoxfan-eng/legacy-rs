//! Release 1.1's integer biome graph. Arrays use `x + z * width`.
//! Java's wrapping LCG and the climate mix layers' unseeded second parents are intentional.
use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc,
};

const A: i64 = 6364136223846793005;
const B: i64 = 1442695040888963407;

// Cardinal neighbors are north, east, west, south. Diagonals are irrelevant.
fn hills_biome(center: i32, neighbors: [i32; 4]) -> i32 {
    if neighbors.iter().any(|&id| id != center) { return center; }
    match center { 2 => 17, 4 => 18, 5 => 19, 1 => 4, 12 => 13, _ => center }
}
fn shore_biome(center: i32, neighbors: [i32; 4]) -> i32 {
    if center == 14 {
        if neighbors.contains(&0) { 15 } else { center }
    } else if !matches!(center, 0 | 7 | 6 | 3) {
        if neighbors.contains(&0) { 16 } else { center }
    } else if center == 3 && neighbors.iter().any(|&id| id != center) {
        20
    } else { center }
}

// Shared by the actual getInts loop and the controlled-parent oracle fixture.
fn neighborhood_cell(kind: Kind, r: &mut Rng, center: i32, neighbors: [i32;4]) -> (i32,i64) {
    let value=match kind {
        Kind::Hills => if r.next(3)==0 { hills_biome(center,neighbors) } else {center},
        Kind::Shore => shore_biome(center,neighbors),
        Kind::SwampRivers => if center==6 && r.next(6)==0 {7} else {center},
        _ => unreachable!(),
    };
    (value,r.state)
}

/// Controlled original-parent fixture, using the same cell implementation as Layer::get.
/// Returns valid output prefix, final cell RNG state and initialized layer world seed.
#[doc(hidden)]
pub fn neighborhood_fixture(kind: u8, salt: i64, seed: i64, x: i32, z: i32, w: i32, h: i32, parent: &[i32]) -> (Vec<i32>,i64,i64) {
    let kind=match kind {0=>Kind::Hills,1=>Kind::Shore,2=>Kind::SwampRivers,_=>panic!("unknown layer")};
    let l=layer(kind,salt,None,None);l.init(seed);let world=l.world.load(Ordering::Relaxed);
    let pw=w+2;assert_eq!(parent.len(),((w+2)*(h+2)) as usize);let mut values=Vec::new();let mut state=0;
    for j in 0..h {for i in 0..w {
        let at=|dx,dz|parent[(i+dx+(j+dz)*pw) as usize];
        let mut r=Rng::new(world,x.wrapping_add(i),z.wrapping_add(j));
        let (value,s)=neighborhood_cell(kind,&mut r,at(1,1),[at(1,0),at(2,1),at(0,1),at(1,2)]);
        values.push(value);state=s;
    }}
    (values,state,world)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hills_require_all_cardinal_neighbors() {
        for (base, hill) in [(2,17), (4,18), (5,19), (1,4), (12,13)] {
            assert_eq!(hills_biome(base, [base; 4]), hill);
            for edge in 0..4 {
                let mut neighbors = [base; 4]; neighbors[edge] = 0;
                assert_eq!(hills_biome(base, neighbors), base);
            }
        }
        for base in [0,3,6,7,8,9,10,11,13,14,15,16,17,18,19,20] {
            assert_eq!(hills_biome(base,[base;4]),base);
        }
    }
    #[test]
    fn shores_preserve_swamps_rivers_and_extreme_hills_rules() {
        for edge in 0..4 {
            let mut coast = [1;4]; coast[edge] = 0;
            assert_eq!(shore_biome(14, coast),15);
            for base in [1,2,4,5,10,11,12,13,15,16,17,18,19,20] {
                assert_eq!(shore_biome(base,coast),16);
            }
            for base in [0,6,7] { assert_eq!(shore_biome(base,coast),base); }
            let mut hills = [3;4]; hills[edge] = 4;
            assert_eq!(shore_biome(3,hills),20);
        }
        assert_eq!(shore_biome(14,[14;4]),14);
        assert_eq!(shore_biome(3,[3;4]),3);
        assert_eq!(shore_biome(1,[1;4]),1);
    }
    #[test]
    fn biome_queries_agree_across_negative_coordinate_tiles() {
        let layers = stack(-7654321);
        for l in &layers[..2] {
            let whole = l.get(-37, -23, 28, 20);
            for (x, z, width, height) in [(-37, -23, 13, 9), (-24, -23, 15, 9), (-37, -14, 28, 11)] {
                let part = l.get(x, z, width, height);
                for dz in 0..height { for dx in 0..width {
                    assert_eq!(part[(dx + dz * width) as usize], whole[((x + 37 + dx) + (z + 23 + dz) * 28) as usize]);
                } }
            }
            assert!(whole.iter().all(|&id| (0..=20).contains(&id)));
        }
    }
}
fn advance(s: i64, add: i64) -> i64 {
    s.wrapping_mul(s.wrapping_mul(A).wrapping_add(B))
        .wrapping_add(add)
}
#[derive(Clone, Copy)]
enum Kind {
    Land,
    Island,
    Snow,
    MushroomIsland,
    Shore,
    Hills,
    SwampRivers,
    Zoom,
    Fuzzy,
    Voronoi,
    SmoothZoom,
    RiverInit,
    River,
    Smooth,
    Landscape,
    RiverMix,
    Temperature,
    Downfall,
    TemperatureMix(i32),
    DownfallMix(i32),
}
pub struct Layer {
    kind: Kind,
    base: i64,
    world: AtomicI64,
    parent: Option<Arc<Layer>>,
    other: Option<Arc<Layer>>,
}
struct Rng {
    state: i64,
    world: i64,
}
impl Rng {
    fn new(world: i64, x: i32, z: i32) -> Self {
        let mut state = world;
        for v in [x, z, x, z] {
            state = advance(state, v as i64);
        }
        Self { state, world }
    }
    fn next(&mut self, bound: i32) -> i32 {
        let mut n = ((self.state >> 24) % bound as i64) as i32;
        if n < 0 {
            n += bound;
        }
        self.state = advance(self.state, self.world);
        n
    }
    fn choose(&mut self, a: i32, b: i32) -> i32 {
        if self.next(2) == 0 {
            a
        } else {
            b
        }
    }
    fn mode(&mut self, a: i32, b: i32, c: i32, d: i32) -> i32 {
        if b == c && c == d {
            b
        } else if (a == b && a == c)
            || (a == b && a == d)
            || (a == c && a == d)
            || (a == b && c != d)
            || (a == c && b != d)
            || (a == d && b != c)
        {
            a
        } else if (b == c && a != d) || (b == d && a != c) {
            b
        } else if c == d && a != b {
            c
        } else {
            [a, b, c, d][self.next(4) as usize]
        }
    }
}
fn layer(
    kind: Kind,
    salt: i64,
    parent: Option<Arc<Layer>>,
    other: Option<Arc<Layer>>,
) -> Arc<Layer> {
    let mut base = salt;
    for _ in 0..3 {
        base = advance(base, salt);
    }
    Arc::new(Layer {
        kind,
        base,
        world: AtomicI64::new(0),
        parent,
        other,
    })
}
fn unary(kind: Kind, salt: i64, parent: Arc<Layer>) -> Arc<Layer> {
    layer(kind, salt, Some(parent), None)
}
fn zoom(mut p: Arc<Layer>, salt: i64, count: i32, kind: Kind) -> Arc<Layer> {
    for i in 0..count {
        p = unary(kind, salt + i as i64, p);
    }
    p
}
pub fn stack(seed: i64) -> [Arc<Layer>; 4] {
    let mut p = layer(Kind::Land, 1, None, None);
    p = unary(Kind::Fuzzy, 2000, p);
    p = unary(Kind::Island, 1, p);
    p = unary(Kind::Zoom, 2001, p);
    p = unary(Kind::Island, 2, p);
    p = unary(Kind::Snow, 2, p);
    p = unary(Kind::Zoom, 2002, p);
    p = unary(Kind::Island, 3, p);
    p = unary(Kind::Zoom, 2003, p);
    p = unary(Kind::Island, 4, p);
    p = unary(Kind::MushroomIsland, 5, p);
    let river = unary(Kind::RiverInit, 100, p.clone());
    let river = zoom(river, 1000, 6, Kind::Zoom);
    let river = unary(Kind::Smooth, 1000, unary(Kind::River, 1, river));
    let mut biome = zoom(unary(Kind::Landscape, 200, p), 1000, 2, Kind::Zoom);
    biome = unary(Kind::Hills, 1000, biome);
    let mut temp = unary(Kind::Temperature, 0, biome.clone());
    let mut rain = unary(Kind::Downfall, 0, biome.clone());
    for i in 0..4 {
        biome = unary(Kind::Zoom, 1000 + i as i64, biome);
        if i == 0 {
            biome = unary(Kind::Island, 3, biome);
        }
        if i == 1 {
            biome = unary(Kind::Shore, 1000, biome);
            biome = unary(Kind::SwampRivers, 1000, biome);
        }
        temp = layer(
            Kind::TemperatureMix(i),
            0,
            Some(biome.clone()),
            Some(unary(Kind::SmoothZoom, 1000 + i as i64, temp)),
        );
        rain = layer(
            Kind::DownfallMix(i),
            0,
            Some(biome.clone()),
            Some(unary(Kind::SmoothZoom, 1000 + i as i64, rain)),
        );
    }
    let biome = layer(
        Kind::RiverMix,
        100,
        Some(unary(Kind::Smooth, 1000, biome)),
        Some(river),
    );
    let temp = zoom(temp, 1000, 2, Kind::SmoothZoom);
    let rain = zoom(rain, 1000, 2, Kind::SmoothZoom);
    let voronoi = unary(Kind::Voronoi, 10, biome.clone());
    for p in [&biome, &temp, &rain, &voronoi] {
        p.init(seed);
    }
    [biome, voronoi, temp, rain]
}
pub fn climate(id: i32, temperature: bool) -> i32 {
    let (t, r) = crate::biome::Biome::from_id(id).climate();
    ((if temperature { t } else { r }) * 65536.0) as i32
}
impl Layer {
    fn init(&self, seed: i64) {
        if let Some(p) = &self.parent {
            p.init(seed);
        }
        if matches!(self.kind, Kind::RiverMix) {
            self.other.as_ref().unwrap().init(seed);
        }
        let mut s = seed;
        for _ in 0..3 {
            s = advance(s, self.base);
        }
        self.world.store(s, Ordering::Relaxed);
    }
    pub fn get(&self, x: i32, z: i32, w: i32, h: i32) -> Vec<i32> {
        crate::intcache::CACHE.with(|cache| {
            let mut cache = cache.borrow_mut();
            cache.reset();
            self.get_cached(x, z, w, h, &mut cache)
        })
    }
    pub(crate) fn get_without_reset(&self, x: i32, z: i32, w: i32, h: i32) -> Vec<i32> {
        crate::intcache::CACHE.with(|cache| self.get_cached(x, z, w, h, &mut cache.borrow_mut()))
    }
    fn get_cached(
        &self,
        mut x: i32,
        mut z: i32,
        w: i32,
        h: i32,
        cache: &mut crate::intcache::IntCache,
    ) -> Vec<i32> {
        let parent = |x, z, w, h, cache: &mut crate::intcache::IntCache| {
            self.parent.as_ref().unwrap().get_cached(x, z, w, h, cache)
        };
        let (mut out, allocation);
        let rng = |x, z| Rng::new(self.world.load(Ordering::Relaxed), x, z);
        match self.kind {
            Kind::Land => {
                (out, allocation) = cache.allocate((w * h) as usize);
                for j in 0..h {
                    for i in 0..w {
                        out[(i + j * w) as usize] =
                            if rng(x.wrapping_add(i), z.wrapping_add(j)).next(10) == 0 {
                                1
                            } else {
                                0
                            };
                    }
                }
                if x > -w && x <= 0 && z > -h && z <= 0 {
                    out[(-x - z * w) as usize] = 1;
                }
            }
            Kind::Zoom | Kind::Fuzzy | Kind::SmoothZoom => {
                let px = x >> 1;
                let pz = z >> 1;
                let pw = (w >> 1) + 3;
                let ph = (h >> 1) + 3;
                let p = parent(px, pz, pw, ph, cache);
                let stride = pw * 2;
                let (mut big, big_allocation) = cache.allocate((pw * 2 * ph * 2) as usize);
                for j in 0..ph - 1 {
                    for i in 0..pw - 1 {
                        let a = p[(i + j * pw) as usize];
                        let b = p[(i + 1 + j * pw) as usize];
                        let c = p[(i + (j + 1) * pw) as usize];
                        let d = p[(i + 1 + (j + 1) * pw) as usize];
                        let mut r = rng(
                            i.wrapping_add(px).wrapping_shl(1),
                            j.wrapping_add(pz).wrapping_shl(1),
                        );
                        let k = (i * 2 + j * 2 * stride) as usize;
                        big[k] = a;
                        if matches!(self.kind, Kind::SmoothZoom) {
                            big[k + stride as usize] = a + (c - a) * r.next(256) / 256;
                            big[k + 1] = a + (b - a) * r.next(256) / 256;
                            let v = a + (b - a) * r.next(256) / 256;
                            let u = c + (d - c) * r.next(256) / 256;
                            big[k + 1 + stride as usize] = v + (u - v) * r.next(256) / 256;
                        } else {
                            big[k + stride as usize] = r.choose(a, c);
                            big[k + 1] = r.choose(a, b);
                            big[k + 1 + stride as usize] = if matches!(self.kind, Kind::Fuzzy) {
                                [a, b, c, d][r.next(4) as usize]
                            } else {
                                r.mode(a, b, c, d)
                            };
                        }
                    }
                }
                cache.finish(&big, big_allocation);
                (out, allocation) = cache.allocate((w * h) as usize);
                for j in 0..h {
                    let start = ((j + (z & 1)) * stride + (x & 1)) as usize;
                    out[(j * w) as usize..((j + 1) * w) as usize]
                        .copy_from_slice(&big[start..start + w as usize]);
                }
            }
            Kind::Voronoi => {
                x = x.wrapping_sub(2);
                z = z.wrapping_sub(2);
                let px = x >> 2;
                let pz = z >> 2;
                let pw = (w >> 2) + 3;
                let ph = (h >> 2) + 3;
                let p = parent(px, pz, pw, ph, cache);
                let stride = pw * 4;
                let (mut big, big_allocation) = cache.allocate((stride * ph * 4) as usize);
                for j in 0..ph - 1 {
                    for i in 0..pw - 1 {
                        let mut jitter = [(0.0, 0.0); 4];
                        for (k, (dx, dz)) in
                            [(0, 0), (1, 0), (0, 1), (1, 1)].into_iter().enumerate()
                        {
                            let mut r =
                                rng((i + px + dx).wrapping_shl(2), (j + pz + dz).wrapping_shl(2));
                            jitter[k] = (
                                (r.next(1024) as f64 / 1024.0 - 0.5) * 3.6 + dx as f64 * 4.0,
                                (r.next(1024) as f64 / 1024.0 - 0.5) * 3.6 + dz as f64 * 4.0,
                            );
                        }
                        let values = [
                            p[(i + j * pw) as usize],
                            p[(i + 1 + j * pw) as usize],
                            p[(i + (j + 1) * pw) as usize],
                            p[(i + 1 + (j + 1) * pw) as usize],
                        ];
                        for dz in 0..4 {
                            for dx in 0..4 {
                                let distances = jitter.map(|(jx, jz)| {
                                    (dz as f64 - jz) * (dz as f64 - jz)
                                        + (dx as f64 - jx) * (dx as f64 - jx)
                                });
                                let k = if distances[0] < distances[1]
                                    && distances[0] < distances[2]
                                    && distances[0] < distances[3]
                                {
                                    0
                                } else if distances[1] < distances[0]
                                    && distances[1] < distances[2]
                                    && distances[1] < distances[3]
                                {
                                    1
                                } else if distances[2] < distances[0]
                                    && distances[2] < distances[1]
                                    && distances[2] < distances[3]
                                {
                                    2
                                } else {
                                    3
                                };
                                big[((j * 4 + dz) * stride + i * 4 + dx) as usize] = values[k];
                            }
                        }
                    }
                }
                cache.finish(&big, big_allocation);
                (out, allocation) = cache.allocate((w * h) as usize);
                for j in 0..h {
                    let start = ((j + (z & 3)) * stride + (x & 3)) as usize;
                    out[(j * w) as usize..((j + 1) * w) as usize]
                        .copy_from_slice(&big[start..start + w as usize]);
                }
            }
            Kind::Island | Kind::Snow | Kind::MushroomIsland | Kind::Shore | Kind::Hills | Kind::SwampRivers | Kind::River | Kind::Smooth => {
                let pw = w + 2;
                let p = parent(x.wrapping_sub(1), z.wrapping_sub(1), pw, h + 2, cache);
                (out, allocation) = cache.allocate((w * h) as usize);
                for j in 0..h {
                    for i in 0..w {
                        let at = |dx, dz| p[(i + dx + (j + dz) * pw) as usize];
                        let mut c = at(1, 1);
                        let mut r = rng(x.wrapping_add(i), z.wrapping_add(j));
                        if matches!(self.kind, Kind::Island) {
                            let corners = [at(0, 0), at(2, 0), at(0, 2), at(2, 2)];
                            if c == 0 && corners.iter().any(|&v| v != 0) {
                                let mut chosen = 1;
                                let mut count = 1;
                                for value in corners {
                                    if value != 0 {
                                        if r.next(count) == 0 { chosen = value; }
                                        count += 1;
                                    }
                                }
                                c = if r.next(3) == 0 { chosen } else if chosen == 12 { 10 } else { 0 };
                            } else if c > 0 && corners.contains(&0) && r.next(5) == 0 {
                                c = if c == 12 { 10 } else { 0 };
                            }
                        } else if matches!(self.kind, Kind::Snow) {
                            if c != 0 { c = if r.next(5) == 0 { 12 } else { 1 }; }
                        } else if matches!(self.kind, Kind::MushroomIsland) {
                            if c == 0 && [at(0, 0), at(2, 0), at(0, 2), at(2, 2)] == [0; 4] && r.next(100) == 0 { c = 14; }
                        } else if matches!(self.kind, Kind::Shore) {
                            let neighbors = [at(1, 0), at(2, 1), at(0, 1), at(1, 2)];
                            c = neighborhood_cell(self.kind,&mut r,c,neighbors).0;
                        } else if matches!(self.kind, Kind::Hills) {
                            let neighbors = [at(1, 0), at(2, 1), at(0, 1), at(1, 2)];
                            c = neighborhood_cell(self.kind,&mut r,c,neighbors).0;
                        } else if matches!(self.kind, Kind::SwampRivers) {
                            c = neighborhood_cell(self.kind,&mut r,c,[0;4]).0;
                        } else {
                            let a = at(0, 1);
                            let b = at(2, 1);
                            let d = at(1, 0);
                            let e = at(1, 2);
                            if matches!(self.kind, Kind::River) {
                                c = if c != 0
                                    && a != 0
                                    && b != 0
                                    && d != 0
                                    && e != 0
                                    && c == a
                                    && c == b
                                    && c == d
                                    && c == e
                                {
                                    -1
                                } else {
                                    7
                                };
                            } else if a == b && d == e {
                                c = r.choose(a, d);
                            } else {
                                if a == b {
                                    c = a;
                                }
                                if d == e {
                                    c = d;
                                }
                            }
                        }
                        out[(i + j * w) as usize] = c;
                    }
                }
            }
            Kind::RiverInit | Kind::Landscape | Kind::Temperature | Kind::Downfall => {
                let p = parent(x, z, w, h, cache);
                (out, allocation) = cache.allocate((w * h) as usize);
                for j in 0..h {
                    for i in 0..w {
                        let k = (i + j * w) as usize;
                        let mut r = rng(x.wrapping_add(i), z.wrapping_add(j));
                        out[k] = match self.kind {
                            Kind::RiverInit => {
                                if p[k] > 0 {
                                    r.next(2) + 2
                                } else {
                                    0
                                }
                            }
                            Kind::Landscape => {
                                match p[k] {
                                    0 | 14 => p[k],
                                    1 => [2, 4, 3, 6, 1, 5][r.next(6) as usize],
                                    _ => 12,
                                }
                            }
                            Kind::Temperature => climate(p[k], true),
                            _ => climate(p[k], false),
                        };
                    }
                }
            }
            Kind::RiverMix | Kind::TemperatureMix(_) | Kind::DownfallMix(_) => {
                let p = parent(x, z, w, h, cache);
                let other = self.other.as_ref().unwrap().get_cached(x, z, w, h, cache);
                (out, allocation) = cache.allocate((w * h) as usize);
                for k in 0..(w * h) as usize {
                    out[k] = match self.kind {
                        Kind::RiverMix => {
                            if p[k] == 0 {
                                p[k]
                            } else if other[k] >= 0 {
                                match p[k] { 12 => 11, 14 | 15 => 15, _ => other[k] }
                            } else {
                                p[k]
                            }
                        }
                        Kind::TemperatureMix(n) => {
                            other[k] + (climate(p[k], true) - other[k]) / (n * 2 + 1)
                        }
                        Kind::DownfallMix(n) => {
                            other[k] + (climate(p[k], false) - other[k]) / (n + 1)
                        }
                        _ => unreachable!(),
                    };
                }
            }
        }
        cache.finish(&out, allocation);
        out
    }
}
