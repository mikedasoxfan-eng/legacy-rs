//! The four Beta 1.8.1 mineshaft pieces and their recursive factory.
use super::{component::Component, BoundingBox as B, Piece, StructurePieceTreasure as T};
use crate::{block, JavaRandom, World};

const LOOT: [T; 11] = [
    T {
        item: 265,
        metadata: 0,
        min: 1,
        max: 5,
        weight: 10,
    },
    T {
        item: 266,
        metadata: 0,
        min: 1,
        max: 3,
        weight: 5,
    },
    T {
        item: 331,
        metadata: 0,
        min: 4,
        max: 9,
        weight: 5,
    },
    T {
        item: 351,
        metadata: 4,
        min: 4,
        max: 9,
        weight: 5,
    },
    T {
        item: 264,
        metadata: 0,
        min: 1,
        max: 2,
        weight: 3,
    },
    T {
        item: 263,
        metadata: 0,
        min: 3,
        max: 8,
        weight: 10,
    },
    T {
        item: 297,
        metadata: 0,
        min: 1,
        max: 3,
        weight: 15,
    },
    T {
        item: 257,
        metadata: 0,
        min: 1,
        max: 1,
        weight: 1,
    },
    T {
        item: 66,
        metadata: 0,
        min: 4,
        max: 8,
        weight: 1,
    },
    T {
        item: 362,
        metadata: 0,
        min: 2,
        max: 4,
        weight: 10,
    },
    T { item:361, metadata:0, min:2, max:4, weight:10 },
];
#[derive(Clone, Debug)]
pub enum Kind {
    Room {
        entrances: Vec<B>,
    },
    Corridor {
        rails: bool,
        spiders: bool,
        spawner: bool,
        sections: i32,
    },
    Cross {
        direction: i32,
        tall: bool,
    },
    Stairs,
}
#[derive(Clone, Debug)]
pub struct MineshaftPiece {
    pub component: Component,
    pub kind: Kind,
}
impl MineshaftPiece {
    pub fn name(&self) -> &'static str {
        match self.kind {
            Kind::Room { .. } => "ComponentMineshaftRoom",
            Kind::Corridor { .. } => "ComponentMineshaftCorridor",
            Kind::Cross { .. } => "ComponentMineshaftCross",
            Kind::Stairs => "ComponentMineshaftStairs",
        }
    }
    fn build(index: usize, pieces: &mut Vec<Self>, root: B, r: &mut JavaRandom) {
        // Java recurses immediately after inserting each child, so take a snapshot of
        // the parent's immutable coordinates, keeping room entrance writes by index.
        let p = pieces[index].clone();
        let b = p.component.bounds;
        let depth = p.component.depth;
        let add = |pieces: &mut Vec<Self>, r: &mut JavaRandom, x, y, z, d, level| {
            Self::add(pieces, root, r, x, y, z, d, level)
        };
        match p.kind {
            Kind::Room { .. } => {
                let sy = (b.sizes()[1] - 4).max(1);
                for side in 0..4 {
                    let length = b.sizes()[if side < 2 { 0 } else { 2 }];
                    let mut n = 0;
                    while n < length {
                        n += r.next_int(length);
                        if n + 3 > length {
                            break;
                        }
                        let y = b.min_y + r.next_int(sy) + 1;
                        let (x, z, d) = match side {
                            0 => (b.min_x + n, b.min_z - 1, 2),
                            1 => (b.min_x + n, b.max_z + 1, 0),
                            2 => (b.min_x - 1, b.min_z + n, 1),
                            _ => (b.max_x + 1, b.min_z + n, 3),
                        };
                        if let Some(child) = add(pieces, r, x, y, z, d, depth) {
                            let c = pieces[child].component.bounds;
                            let entry = match side {
                                0 => B::new([
                                    c.min_x,
                                    c.min_y,
                                    b.min_z,
                                    c.max_x,
                                    c.max_y,
                                    b.min_z + 1,
                                ]),
                                1 => B::new([
                                    c.min_x,
                                    c.min_y,
                                    b.max_z - 1,
                                    c.max_x,
                                    c.max_y,
                                    b.max_z,
                                ]),
                                2 => B::new([
                                    b.min_x,
                                    c.min_y,
                                    c.min_z,
                                    b.min_x + 1,
                                    c.max_y,
                                    c.max_z,
                                ]),
                                _ => B::new([
                                    b.max_x - 1,
                                    c.min_y,
                                    c.min_z,
                                    b.max_x,
                                    c.max_y,
                                    c.max_z,
                                ]),
                            };
                            if let Kind::Room { entrances } = &mut pieces[index].kind {
                                entrances.push(entry);
                            }
                        }
                        n += 4;
                    }
                }
            }
            Kind::Corridor { .. } => {
                let choice = r.next_int(4);
                let d = p.component.orientation;
                let (x, z, dir) = match (d, choice) {
                    (0, 0 | 1) => (b.min_x, b.max_z + 1, d),
                    (0, 2) => (b.min_x - 1, b.max_z - 3, 1),
                    (0, _) => (b.max_x + 1, b.max_z - 3, 3),
                    (1, 0 | 1) => (b.min_x - 1, b.min_z, d),
                    (1, 2) => (b.min_x, b.min_z - 1, 2),
                    (1, _) => (b.min_x, b.max_z + 1, 0),
                    (2, 0 | 1) => (b.min_x, b.min_z - 1, d),
                    (2, 2) => (b.min_x - 1, b.min_z, 1),
                    (2, _) => (b.max_x + 1, b.min_z, 3),
                    (3, 0 | 1) => (b.max_x + 1, b.min_z, d),
                    (3, 2) => (b.max_x - 3, b.min_z - 1, 2),
                    (3, _) => (b.max_x - 3, b.max_z + 1, 0),
                    _ => unreachable!(),
                };
                let y = b.min_y - 1 + r.next_int(3);
                add(pieces, r, x, y, z, dir, depth);
                if depth < 10 {
                    if d != 0 && d != 2 {
                        let mut x = b.min_x + 3;
                        while x + 3 <= b.max_x {
                            match r.next_int(5) {
                                0 => {
                                    add(pieces, r, x, b.min_y, b.min_z - 1, 2, depth + 1);
                                }
                                1 => {
                                    add(pieces, r, x, b.min_y, b.max_z + 1, 0, depth + 1);
                                }
                                _ => {}
                            }
                            x += 5;
                        }
                    } else {
                        let mut z = b.min_z + 3;
                        while z + 3 <= b.max_z {
                            match r.next_int(5) {
                                0 => {
                                    add(pieces, r, b.min_x - 1, b.min_y, z, 1, depth + 1);
                                }
                                1 => {
                                    add(pieces, r, b.max_x + 1, b.min_y, z, 3, depth + 1);
                                }
                                _ => {}
                            }
                            z += 5;
                        }
                    }
                }
            }
            Kind::Cross { direction, tall } => {
                let targets = [
                    (b.min_x + 1, b.min_z - 1, 2),
                    (b.min_x - 1, b.min_z + 1, 1),
                    (b.max_x + 1, b.min_z + 1, 3),
                    (b.min_x + 1, b.max_z + 1, 0),
                ];
                let order: &[usize] = match direction {
                    0 => &[3, 1, 2],
                    1 => &[0, 3, 1],
                    2 => &[0, 1, 2],
                    _ => &[0, 3, 2],
                };
                for &n in order {
                    let (x, z, d) = targets[n];
                    add(pieces, r, x, b.min_y, z, d, depth);
                }
                if tall {
                    for (x, z, d) in targets {
                        if r.next_bool() {
                            add(pieces, r, x, b.min_y + 4, z, d, depth);
                        }
                    }
                }
            }
            Kind::Stairs => {
                let d = p.component.orientation;
                let (x, z) = match d {
                    0 => (b.min_x, b.max_z + 1),
                    1 => (b.min_x - 1, b.min_z),
                    2 => (b.min_x, b.min_z - 1),
                    _ => (b.max_x + 1, b.min_z),
                };
                add(pieces, r, x, b.min_y, z, d, depth);
            }
        }
    }
    fn add(
        pieces: &mut Vec<Self>,
        root: B,
        r: &mut JavaRandom,
        x: i32,
        y: i32,
        z: i32,
        d: i32,
        depth: i32,
    ) -> Option<usize> {
        if depth > 8
            || x.wrapping_sub(root.min_x).wrapping_abs() > 80
            || z.wrapping_sub(root.min_z).wrapping_abs() > 80
        {
            return None;
        }
        let choice = r.next_int(100);
        let mut b = B::new([x, y, z, x, y + 2, z]);
        let kind;
        if choice >= 80 {
            if r.next_int(4) == 0 {
                b.max_y += 4;
            }
            match d {
                0 => {
                    b.min_x = x - 1;
                    b.max_x = x + 3;
                    b.max_z = z + 4;
                }
                1 => {
                    b.min_x = x - 4;
                    b.min_z = z - 1;
                    b.max_z = z + 3;
                }
                2 => {
                    b.min_x = x - 1;
                    b.max_x = x + 3;
                    b.min_z = z - 4;
                }
                _ => {
                    b.max_x = x + 4;
                    b.min_z = z - 1;
                    b.max_z = z + 3;
                }
            }
            if pieces.iter().any(|p| p.component.bounds.intersects(b)) {
                return None;
            }
            kind = Kind::Cross {
                direction: d,
                tall: b.sizes()[1] > 3,
            };
        } else if choice >= 70 {
            b.min_y = y - 5;
            match d {
                0 => {
                    b.max_x = x + 2;
                    b.max_z = z + 8;
                }
                1 => {
                    b.min_x = x - 8;
                    b.max_z = z + 2;
                }
                2 => {
                    b.max_x = x + 2;
                    b.min_z = z - 8;
                }
                _ => {
                    b.max_x = x + 8;
                    b.max_z = z + 2;
                }
            }
            if pieces.iter().any(|p| p.component.bounds.intersects(b)) {
                return None;
            }
            kind = Kind::Stairs;
        } else {
            let mut sections = r.next_int(3) + 2;
            while sections > 0 {
                let n = sections * 5 - 1;
                match d {
                    0 => {
                        b.max_x = x + 2;
                        b.max_z = z + n;
                    }
                    1 => {
                        b.min_x = x - n;
                        b.max_z = z + 2;
                    }
                    2 => {
                        b.max_x = x + 2;
                        b.min_z = z - n;
                    }
                    _ => {
                        b.max_x = x + n;
                        b.max_z = z + 2;
                    }
                }
                if !pieces.iter().any(|p| p.component.bounds.intersects(b)) {
                    break;
                }
                sections -= 1;
            }
            if sections == 0 {
                return None;
            }
            let rails = r.next_int(3) == 0;
            let spiders = !rails && r.next_int(23) == 0;
            kind = Kind::Corridor {
                rails,
                spiders,
                spawner: false,
                sections,
            };
        }
        let index = pieces.len();
        pieces.push(Self {
            component: Component {
                bounds: b,
                orientation: if matches!(kind, Kind::Cross { .. }) {
                    -1
                } else {
                    d
                },
                depth: depth + 1,
            },
            kind,
        });
        Self::build(index, pieces, root, r);
        Some(index)
    }
    pub fn start(r: &mut JavaRandom, cx: i32, cz: i32) -> (Vec<Self>, B) {
        let x = (cx << 4) + 2;
        let z = (cz << 4) + 2;
        let b = B::new([
            x,
            50,
            z,
            x + 7 + r.next_int(6),
            54 + r.next_int(6),
            z + 7 + r.next_int(6),
        ]);
        let mut pieces = vec![Self {
            component: Component {
                bounds: b,
                orientation: -1,
                depth: 0,
            },
            kind: Kind::Room {
                entrances: Vec::new(),
            },
        }];
        Self::build(0, &mut pieces, b, r);
        let mut bounds = B::empty();
        for p in &pieces {
            bounds.encompass(p.component.bounds);
        }
        let mut top = bounds.sizes()[1] + 1;
        if top < 53 {
            top += r.next_int(53 - top);
        }
        let dy = top - bounds.max_y;
        bounds.offset(0, dy, 0);
        for p in &mut pieces {
            p.component.bounds.offset(0, dy, 0);
        }
        (pieces, bounds)
    }
}
impl Piece for MineshaftPiece {
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
        let c = self.component;
        let b = c.bounds;
        if c.liquids(w, clip) {
            return false;
        }
        match &mut self.kind {
            Kind::Room { entrances } => {
                c.fill(
                    w,
                    clip,
                    [b.min_x, b.min_y, b.min_z, b.max_x, b.min_y, b.max_z],
                    3,
                    0,
                    true,
                );
                c.fill(
                    w,
                    clip,
                    [
                        b.min_x,
                        b.min_y + 1,
                        b.min_z,
                        b.max_x,
                        (b.min_y + 3).min(b.max_y),
                        b.max_z,
                    ],
                    0,
                    0,
                    false,
                );
                for e in entrances {
                    c.fill(
                        w,
                        clip,
                        [e.min_x, e.max_y - 2, e.min_z, e.max_x, e.max_y, e.max_z],
                        0,
                        0,
                        false,
                    );
                }
                c.dome(
                    w,
                    clip,
                    [b.min_x, b.min_y + 4, b.min_z, b.max_x, b.max_y, b.max_z],
                    0,
                    false,
                );
            }
            Kind::Stairs => {
                c.fill(w, clip, [0, 5, 0, 2, 7, 1], 0, 0, false);
                c.fill(w, clip, [0, 0, 7, 2, 2, 8], 0, 0, false);
                for n in 0..5 {
                    c.fill(
                        w,
                        clip,
                        [0, 5 - n - if n < 4 { 1 } else { 0 }, 2 + n, 2, 7 - n, 2 + n],
                        0,
                        0,
                        false,
                    );
                }
            }
            Kind::Cross { tall, .. } => {
                if *tall {
                    for (y0, y1) in [(b.min_y, b.min_y + 2), (b.max_y - 2, b.max_y)] {
                        c.fill(
                            w,
                            clip,
                            [b.min_x + 1, y0, b.min_z, b.max_x - 1, y1, b.max_z],
                            0,
                            0,
                            false,
                        );
                        c.fill(
                            w,
                            clip,
                            [b.min_x, y0, b.min_z + 1, b.max_x, y1, b.max_z - 1],
                            0,
                            0,
                            false,
                        );
                    }
                    c.fill(
                        w,
                        clip,
                        [
                            b.min_x + 1,
                            b.min_y + 3,
                            b.min_z + 1,
                            b.max_x - 1,
                            b.min_y + 3,
                            b.max_z - 1,
                        ],
                        0,
                        0,
                        false,
                    );
                } else {
                    c.fill(
                        w,
                        clip,
                        [b.min_x + 1, b.min_y, b.min_z, b.max_x - 1, b.max_y, b.max_z],
                        0,
                        0,
                        false,
                    );
                    c.fill(
                        w,
                        clip,
                        [b.min_x, b.min_y, b.min_z + 1, b.max_x, b.max_y, b.max_z - 1],
                        0,
                        0,
                        false,
                    );
                }
                for (x, z) in [
                    (b.min_x + 1, b.min_z + 1),
                    (b.min_x + 1, b.max_z - 1),
                    (b.max_x - 1, b.min_z + 1),
                    (b.max_x - 1, b.max_z - 1),
                ] {
                    c.fill(w, clip, [x, b.min_y, z, x, b.max_y, z], 5, 0, false);
                }
            }
            Kind::Corridor {
                rails,
                spiders,
                spawner,
                sections,
            } => {
                let end = *sections * 5 - 1;
                c.fill(w, clip, [0, 0, 0, 2, 1, end], 0, 0, false);
                c.random_fill(w, clip, r, 0.8, [0, 2, 0, 2, 2, end], 0, 0, false);
                if *spiders {
                    c.random_fill(w, clip, r, 0.6, [0, 0, 0, 2, 1, end], 30, 0, false);
                }
                for n in 0..*sections {
                    let z = 2 + n * 5;
                    c.fill(w, clip, [0, 0, z, 0, 1, z], 85, 0, false);
                    c.fill(w, clip, [2, 0, z, 2, 1, z], 85, 0, false);
                    if super::component::random(w, r).next_int(4) != 0 {
                        c.fill(w, clip, [0, 2, z, 2, 2, z], 5, 0, false);
                    } else {
                        c.fill(w, clip, [0, 2, z, 0, 2, z], 5, 0, false);
                        c.fill(w, clip, [2, 2, z, 2, 2, z], 5, 0, false);
                    }
                    for (chance, x, dz) in [
                        (0.1, 0, -1),
                        (0.1, 2, -1),
                        (0.1, 0, 1),
                        (0.1, 2, 1),
                        (0.05, 0, -2),
                        (0.05, 2, -2),
                        (0.05, 0, 2),
                        (0.05, 2, 2),
                    ] {
                        c.random_put(w, clip, r, chance, 30, 0, x, 2, z + dz);
                    }
                    c.random_put(w, clip, r, 0.05, 50, 0, 1, 2, z - 1);
                    c.random_put(w, clip, r, 0.05, 50, 0, 1, 2, z + 1);
                    if super::component::random(w, r).next_int(100) == 0 {
                        let count = 3 + super::component::random(w, r).next_int(4);
                        c.chest(w, clip, r, 2, 0, z - 1, &LOOT, count);
                    }
                    if super::component::random(w, r).next_int(100) == 0 {
                        let count = 3 + super::component::random(w, r).next_int(4);
                        c.chest(w, clip, r, 0, 0, z + 1, &LOOT, count);
                    }
                    if *spiders && !*spawner {
                        let dz = z - 1 + super::component::random(w, r).next_int(3);
                        let (x, y, z) = c.position(1, 0, dz);
                        if clip.contains(x, y, z) {
                            *spawner = true;
                            w.set_block_with_notify(x, y, z, 52);
                            if let Some(crate::world::tile::TileEntity::MobSpawner {
                                mob, ..
                            }) = w.get_block_tile_entity(x, y, z)
                            {
                                *mob = "CaveSpider".into();
                            }
                        }
                    }
                }
                if *rails {
                    for z in 0..=end {
                        let id = c.get(w, clip, 1, -1, z);
                        if id > 0 && block::opaque_cube_lookup(id as u8) {
                            c.random_put(w, clip, r, 0.7, 66, c.rotate(66, 0), 1, 0, z);
                        }
                    }
                }
            }
        }
        true
    }
}
