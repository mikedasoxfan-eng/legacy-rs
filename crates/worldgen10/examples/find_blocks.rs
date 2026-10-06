//! Find seeds whose vanilla spawn area contains given block ids (e.g. cactus = 81).
use worldgen::World;

fn main() {
    let want: Vec<u8> = std::env::args().skip(1).map(|a| a.parse().unwrap()).collect();
    for seed in 0..200i64 {
        let w = World::new_vanilla(seed, 1);
        let (sx, _, sz) = w.spawn;
        let (scx, scz) = (sx >> 4, sz >> 4);
        let mut counts = vec![0usize; want.len()];
        for cx in scx - 5..=scx + 5 {
            for cz in scz - 5..=scz + 5 {
                for &b in w.chunk(cx, cz).blocks.iter() {
                    if let Some(i) = want.iter().position(|&x| x == b) {
                        counts[i] += 1;
                    }
                }
            }
        }
        if counts.iter().all(|&c| c > 0) {
            println!("seed {seed}: {counts:?}");
            return;
        }
    }
}
