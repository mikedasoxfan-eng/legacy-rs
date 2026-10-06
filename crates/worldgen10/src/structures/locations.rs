//! Literal structure candidate selection, independent of concrete piece factories.
use crate::{Biome, BiomeSource, JavaRandom};
#[derive(Default)]
pub struct Locations {
    pub strongholds: Option<[(i32, i32); 3]>,
}
pub fn village(
    seed: i64,
    x: i32,
    z: i32,
    biomes: &BiomeSource,
    world_rand: &mut JavaRandom,
) -> bool {
    let rx = if x < 0 { x.wrapping_sub(31) } else { x } / 32;
    let rz = if z < 0 { z.wrapping_sub(31) } else { z } / 32;
    world_rand.set_seed(
        (rx as i64)
            .wrapping_mul(341873128712)
            .wrapping_add((rz as i64).wrapping_mul(132897987541))
            .wrapping_add(seed)
            .wrapping_add(10387312),
    );
    let candidate_x = rx.wrapping_mul(32).wrapping_add(world_rand.next_int(24));
    let candidate_z = rz.wrapping_mul(32).wrapping_add(world_rand.next_int(24));
    x == candidate_x
        && z == candidate_z
        && biomes.viable(
            x.wrapping_mul(16).wrapping_add(8),
            z.wrapping_mul(16).wrapping_add(8),
            0,
            &[Biome::Plains, Biome::Desert],
        )
}
pub fn strongholds(seed: i64, biomes: &BiomeSource, r: &mut JavaRandom) -> [(i32, i32); 3] {
    r.set_seed(seed);
    let mut angle = r.next_double() * std::f64::consts::PI * 2.0;
    let mut out = [(0, 0); 3];
    for p in &mut out {
        let radius = (1.25 + r.next_double()) * 32.0;
        let x = (crate::jmath::java_cos(angle) * radius + 0.5).floor() as i32;
        let z = (crate::jmath::java_sin(angle) * radius + 0.5).floor() as i32;
        *p = biomes
            .find_position(
                (x << 4) + 8,
                (z << 4) + 8,
                112,
                &[
                    Biome::Desert,
                    Biome::Forest,
                    Biome::ExtremeHills,
                    Biome::Swampland,
                ],
                r,
            )
            .map_or((x, z), |(x, z)| (x >> 4, z >> 4));
        angle += std::f64::consts::PI * 2.0 / 3.0;
    }
    out
}
