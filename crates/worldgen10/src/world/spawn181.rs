//! Beta 1.8.1 spawn search. GenLayer biome data is supplied by the terrain owner.
use crate::JavaRandom;
pub fn biome_position(
    x: i32,
    z: i32,
    radius: i32,
    biomes: &[i32],
    eligible: &[i32],
    rand: &mut JavaRandom,
) -> Option<(i32, i32)> {
    let min_x = x.wrapping_sub(radius) >> 2;
    let min_z = z.wrapping_sub(radius) >> 2;
    let width = (x.wrapping_add(radius) >> 2)
        .wrapping_sub(min_x)
        .wrapping_add(1);
    let mut selected = None;
    let mut count = 0;
    for (k, &biome) in biomes.iter().enumerate() {
        if eligible.contains(&biome) && (selected.is_none() || rand.next_int(count + 1) == 0) {
            selected = Some((
                min_x.wrapping_add(k as i32 % width).wrapping_shl(2),
                min_z.wrapping_add(k as i32 / width).wrapping_shl(2),
            ));
            // Vanilla increments only on replacement, not on every eligible cell.
            count += 1;
        }
    }
    selected
}
pub fn initial_position(
    seed: i64,
    biomes: &[i32],
    mut can_spawn: impl FnMut(i32, i32) -> bool,
) -> (i32, i32, i32) {
    let mut rand = JavaRandom::new(seed);
    let (mut x, mut z) = biome_position(0, 0, 256, biomes, &[4, 6, 5], &mut rand).unwrap_or((0, 0));
    let mut attempts = 0;
    while !can_spawn(x, z) {
        x = x.wrapping_add(rand.next_int(64) - rand.next_int(64));
        z = z.wrapping_add(rand.next_int(64) - rand.next_int(64));
        attempts += 1;
        if attempts == 1000 {
            break;
        }
    }
    (x, 64, z)
}
impl crate::World {
    /// Complete spawn entry point once the terrain owner supplies its GenLayer rectangle.
    pub fn find_initial_spawn181(&mut self, biomes: &[i32]) {
        if let Some(s) = &mut self.server {
            s.world_override = true;
        }
        self.spawn = initial_position(self.seed, biomes, |x, z| self.can_coordinate_be_spawn(x, z));
        self.info.spawn = self.spawn;
        if let Some(s) = &mut self.server {
            s.world_override = false;
        }
    }
}
