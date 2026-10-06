//! ChunkProviderSky retains its pre-GenLayer population order and lava threshold.
use crate::{block, features as f, features::TreeKind, Biome, World};
pub fn populate(w: &mut World, cx: i32, cz: i32) {
    w.sand_fall_instantly = true;
    let x0 = cx * 16;
    let z0 = cz * 16;
    let biome = w.terrain.biome_source.biome_at(x0 + 16, z0 + 16);
    let seed = w.seed;
    let r = w.prand();
    r.set_seed(seed);
    let mx = r.next_long() / 2 * 2 + 1;
    let mz = r.next_long() / 2 * 2 + 1;
    r.set_seed(
        (cx as i64)
            .wrapping_mul(mx)
            .wrapping_add((cz as i64).wrapping_mul(mz))
            ^ seed,
    );
    fn pos(w: &mut World, x0: i32, z0: i32, offset: i32, high: i32) -> (i32, i32, i32) {
        let x = x0 + w.prand().next_int(16) + offset;
        let y = w.prand().next_int(high);
        let z = z0 + w.prand().next_int(16) + offset;
        (x, y, z)
    }
    if w.prand().next_int(4) == 0 {
        let (x, y, z) = pos(w, x0, z0, 8, 128);
        f::lakes(w, 9, x, y, z);
    }
    if w.prand().next_int(8) == 0 {
        let x = x0 + w.prand().next_int(16) + 8;
        let n = w.prand().next_int(120) + 8;
        let y = w.prand().next_int(n);
        let z = z0 + w.prand().next_int(16) + 8;
        if y < 64 || w.prand().next_int(10) == 0 {
            f::lakes(w, 11, x, y, z);
        }
    }
    for _ in 0..8 {
        let (x, y, z) = pos(w, x0, z0, 8, 128);
        f::dungeon(w, x, y, z);
    }
    for _ in 0..10 {
        let (x, y, z) = pos(w, x0, z0, 0, 128);
        f::clay(w, 32, x, y, z);
    }
    for (id, size, count, high) in [
        (3, 32, 20, 128),
        (13, 32, 10, 128),
        (16, 16, 20, 128),
        (15, 8, 20, 64),
        (14, 8, 2, 32),
        (73, 7, 8, 16),
        (56, 7, 1, 16),
    ] {
        for _ in 0..count {
            let (x, y, z) = pos(w, x0, z0, 0, high);
            f::minable(w, id, size, x, y, z);
        }
    }
    let x = x0 + w.prand().next_int(16);
    let y = w.prand().next_int(16) + w.prand().next_int(16);
    let z = z0 + w.prand().next_int(16);
    f::minable(w, 21, 6, x, y, z);
    let noise = w
        .sky
        .as_mut()
        .expect("Sky provider")
        .forest
        .sample_2d(x0 as f64 * 0.5, z0 as f64 * 0.5);
    let density = ((noise / 8.0 + w.prand().next_double() * 4.0 + 4.0) / 3.0) as i32;
    let mut count = if w.prand().next_int(10) == 0 { 1 } else { 0 };
    if biome == Biome::Forest {
        count += density + 5;
    }
    if biome == Biome::Desert || biome == Biome::Plains {
        count -= 20;
    }
    for _ in 0..count {
        let x = x0 + w.prand().next_int(16) + 8;
        let z = z0 + w.prand().next_int(16) + 8;
        let kind = crate::populate::tree_for_biome(w, biome);
        let y = w.get_height_value(x, z);
        if matches!(kind, TreeKind::BigOak) {
            crate::biome::big_tree(biome, w, x, y, z);
        } else {
            f::tree(w, kind, x, y, z);
        }
    }
    for _ in 0..2 {
        let (x, y, z) = pos(w, x0, z0, 8, 128);
        f::flowers(w, 37, x, y, z);
    }
    for (chance, id) in [(2, 38), (4, 39), (8, 40)] {
        if w.prand().next_int(chance) == 0 {
            let (x, y, z) = pos(w, x0, z0, 8, 128);
            f::flowers(w, id, x, y, z);
        }
    }
    for _ in 0..10 {
        let (x, y, z) = pos(w, x0, z0, 8, 128);
        f::reed(w, x, y, z);
    }
    if w.prand().next_int(32) == 0 {
        let (x, y, z) = pos(w, x0, z0, 8, 128);
        f::pumpkin(w, x, y, z);
    }
    if biome == Biome::Desert {
        for _ in 0..10 {
            let (x, y, z) = pos(w, x0, z0, 8, 128);
            f::cactus(w, x, y, z);
        }
    }
    for _ in 0..50 {
        let x = x0 + w.prand().next_int(16) + 8;
        let n = w.prand().next_int(120) + 8;
        let y = w.prand().next_int(n);
        let z = z0 + w.prand().next_int(16) + 8;
        f::liquid_spring(w, block::WATER_MOVING, x, y, z);
    }
    for _ in 0..20 {
        let x = x0 + w.prand().next_int(16) + 8;
        let n = w.prand().next_int(112) + 8;
        let n = w.prand().next_int(n) + 8;
        let y = w.prand().next_int(n);
        let z = z0 + w.prand().next_int(16) + 8;
        f::liquid_spring(w, block::LAVA_MOVING, x, y, z);
    }
    w.sand_fall_instantly = false;
}
