//! Columns near spawn whose top block is leaves (for the fire tick scenario).
use worldgen::World;

fn main() {
    let a: Vec<i64> = std::env::args().skip(1).map(|a| a.parse().unwrap()).collect();
    let w = World::new_vanilla(a[0], a[1]);
    let (sx, _, sz) = w.spawn;
    let mut found = Vec::new();
    for r in (0..96).step_by(4) {
        for (dx, dz) in [(r, 0), (-r, 0), (0, r), (0, -r), (r, r), (-r, -r)] {
            let h = w.get_height_value(sx + dx, sz + dz);
            if h > 0 && w.chunk((sx + dx) >> 4, (sz + dz) >> 4).block((sx + dx) & 15, h - 1, (sz + dz) & 15) == 18 {
                found.push(format!("{dx}:{dz}"));
            }
        }
    }
    println!("{}", found.join(","));
}
