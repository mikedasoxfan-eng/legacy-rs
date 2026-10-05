//! Find seeds where Fancy vs Fast graphics changes generated terrain (snow on leaves).
use worldgen::World;

fn main() {
    for seed in 0..40i64 {
        let mut fast = World::new(seed, 1);
        fast.find_initial_spawn();
        fast.preload_spawn_area();
        let mut fancy = World::new(seed, 1);
        fancy.leaves_opaque = false;
        fancy.find_initial_spawn();
        fancy.preload_spawn_area();
        let diff: usize = fast
            .chunks
            .iter()
            .map(|(k, c)| c.blocks.iter().zip(fancy.chunks[k].blocks.iter()).filter(|(a, b)| a != b).count())
            .sum();
        if diff > 0 {
            println!("seed {seed}: {diff} blocks differ between Fast and Fancy");
        }
    }
}
