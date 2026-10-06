//! Release 1.0 ChunkProviderEnd. Density arithmetic follows the original Java source.
use crate::{
    biome::{Biome, BiomeSource},
    noise::OctavePerlin,
    overworld::{index, RawChunk, CHUNK_VOLUME},
    JavaRandom,
};
pub struct EndGenerator {
    rand: JavaRandom,
    min: OctavePerlin,
    max: OctavePerlin,
    main: OctavePerlin,
    pub scale: OctavePerlin,
    pub depth: OctavePerlin,
    pub biome_source: BiomeSource,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EndEntitySpawn {
    Crystal { x: f64, y: f64, z: f64, yaw: f32 },
    Dragon { yaw: f32 },
}

/// BiomeEndDecorator.decorate_do. It uses World.rand, without reseeding it.
pub fn populate(w: &mut crate::World, cx: i32, cz: i32) {
    w.sand_fall_instantly = true;
    let previous = w.population_uses_world10;
    w.population_uses_world10 = true;
    crate::decorator10::begin(Biome::Sky);
    let (x0, z0) = (cx.wrapping_mul(16), cz.wrapping_mul(16));
    crate::populate::generate_ores(w, x0, z0);
    if w.prand().next_int(5) == 0 {
        let x = x0 + w.prand().next_int(16) + 8;
        let z = z0 + w.prand().next_int(16) + 8;
        let y = w.get_top_solid_or_liquid_block(x, z);
        generate_spike(w, x, y, z);
    }
    if x0 == 0 && z0 == 0 {
        // Construct before drawing the yaw: the constructor allocates the dragon's
        // seven parts and their Randoms, as in BiomeEndDecorator.decorate_do.
        let mut dragon = crate::entity::Entity::living(w, "EnderDragon")
            .expect("Release dragon constructor");
        let yaw = w.prand().next_float() * 360.0;
        dragon.set_location_and_angles(0.0, 128.0, 0.0, yaw, 0.0);
        w.entity_joined_world(dragon);
        w.end_entity_spawns10.push(EndEntitySpawn::Dragon { yaw });
    }
    crate::decorator10::finish(Biome::Sky);
    w.population_uses_world10 = previous;
    w.sand_fall_instantly = false;
}

/// WorldGenSpikes: validate the whole footprint before changing any blocks.
pub fn generate_spike(w: &mut crate::World, x: i32, y: i32, z: i32) -> bool {
    if w.get_block_id(x, y, z) != 0 || w.get_block_id(x, y - 1, z) != 121 { return false; }
    let height = w.prand().next_int(32) + 6;
    let radius = w.prand().next_int(4) + 1;
    for bx in x - radius..=x + radius {
        for bz in z - radius..=z + radius {
            let (dx, dz) = (bx - x, bz - z);
            if dx * dx + dz * dz <= radius * radius + 1 && w.get_block_id(bx, y - 1, bz) != 121 {
                return false;
            }
        }
    }
    for by in y..(y + height).min(128) {
        for bx in x - radius..=x + radius {
            for bz in z - radius..=z + radius {
                let (dx, dz) = (bx - x, bz - z);
                if dx * dx + dz * dz <= radius * radius + 1 {
                    w.set_block_with_notify(bx, by, bz, 49);
                }
            }
        }
    }
    let mut crystal = crate::entity::Entity::ender_crystal(w, crate::entity::Vec3D::default());
    let yaw = w.prand().next_float() * 360.0;
    let position = crate::entity::Vec3D::new(
        (x as f32 + 0.5) as f64, (y + height) as f64, (z as f32 + 0.5) as f64);
    crystal.set_location_and_angles(position.x, position.y, position.z, yaw, 0.0);
    w.entity_joined_world(crystal);
    w.end_entity_spawns10.push(EndEntitySpawn::Crystal {
        x: (x as f32 + 0.5) as f64, y: (y + height) as f64, z: (z as f32 + 0.5) as f64, yaw,
    });
    w.set_block_with_notify(x, y + height, z, 7);
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn end_island_and_void_are_end_stone_only() {
        let mut gen = EndGenerator::new(42);
        let origin = gen.generate(0, 0);
        assert!(origin.blocks.iter().any(|&id| id == 121));
        assert!(origin.blocks.iter().all(|&id| id == 0 || id == 121));
        let distant = gen.generate(100, 100);
        assert!(distant.blocks.iter().all(|&id| id == 0));
        let again = gen.generate(0, 0);
        assert_eq!(origin.blocks, again.blocks);
    }
    #[test]
    fn accepted_spike_has_circular_footprint_cap_and_crystal_request() {
        let mut w = crate::World::with_dimension(42, 7, crate::world::Dimension::End);
        w.chunks.insert((0, 0), Box::new(crate::world::chunk::Chunk::new(0, 0, vec![0; CHUNK_VOLUME].into_boxed_slice())));
        for x in 0..16 { for z in 0..16 { w.chunk_mut(0, 0).blocks[index(x, 63, z)] = 121; } }
        let mut expected = w.prand().clone();
        let height = expected.next_int(32) + 6;
        let radius = expected.next_int(4) + 1;
        let yaw = expected.next_float() * 360.0;
        assert!(generate_spike(&mut w, 8, 64, 8));
        for x in 3..14 { for z in 3..14 {
            let inside = (x - 8) * (x - 8) + (z - 8) * (z - 8) <= radius * radius + 1;
            assert_eq!(w.get_block_id(x, 64, z), if inside {49} else {0});
        } }
        assert_eq!(w.get_block_id(8, 64 + height, 8), 7);
        assert_eq!(w.end_entity_spawns10, vec![EndEntitySpawn::Crystal {x: 8.5, y: (64 + height) as f64, z: 8.5, yaw}]);
        let crystal = w.entities.values().find(|e| matches!(e.kind, crate::entity::EntityKind::EnderCrystal { .. })).unwrap();
        // Entity.setLocationAndAngles adds the crystal's one-block yOffset.
        assert_eq!(crystal.pos, crate::entity::Vec3D::new(8.5, (65 + height) as f64, 8.5));
        assert_eq!(crystal.yaw, yaw);
        assert!(w.loaded_entity_list.contains(&crystal.entity_id));
        assert!(w.chunk(0, 0).entity_lists.iter().any(|ids| ids.contains(&crystal.entity_id)));
        assert_eq!(w.prand().next_int_unbounded(), expected.next_int_unbounded());
    }
    #[test]
    fn origin_population_joins_one_dragon_and_allocates_seven_parts() {
        let mut w = crate::World::with_dimension(42, 7, crate::world::Dimension::End);
        w.entity_seed_source = crate::entity::EntitySeedSource::Deterministic(500);
        w.next_entity_id = 100;
        // Empty resident chunks make ore/spike attempts fail without recursive population.
        for x in 0..=2 { for z in 0..=2 {
            let mut c = crate::world::chunk::Chunk::new(x, z, vec![0; CHUNK_VOLUME].into_boxed_slice());
            c.loaded = true;
            c.populated = true;
            w.chunks.insert((x, z), Box::new(c));
        } }
        populate(&mut w, 0, 0);
        assert_eq!(w.loaded_entity_list, [100]);
        assert_eq!(w.next_entity_id, 108);
        let dragon = &w.entities[&100];
        assert_eq!(dragon.pos, crate::entity::Vec3D::new(0.0, 128.0, 0.0));
        let crate::entity::EntityKind::Dragon(state) = &dragon.kind else { panic!("missing dragon") };
        assert_eq!(state.parts, [101, 102, 103, 104, 105, 106, 107]);
        for id in state.parts {
            assert!(matches!(w.entities[&id].kind, crate::entity::EntityKind::DragonPart { parent: 100, .. }));
            assert!(!w.loaded_entity_list.contains(&id));
        }
        assert!(matches!(w.entity_seed_source, crate::entity::EntitySeedSource::Deterministic(508)));
        populate(&mut w, 1, 0);
        assert_eq!(w.loaded_entity_list, [100]);
        assert_eq!(w.next_entity_id, 108);
        assert!(!w.sand_fall_instantly);
        assert!(!w.population_uses_world10);
    }
    #[test]
    fn spike_rejection_does_not_place_blocks_or_consume_yaw() {
        let mut w = crate::World::new(42, 7);
        w.chunks.insert((0, 0), Box::new(crate::world::chunk::Chunk::new(0, 0, vec![0; CHUNK_VOLUME].into_boxed_slice())));
        w.chunk_mut(0, 0).blocks[index(8, 63, 8)] = 121;
        let mut expected = w.prand().clone();
        expected.next_int(32);
        expected.next_int(4);
        assert!(!generate_spike(&mut w, 8, 64, 8));
        assert_eq!(w.prand().next_int_unbounded(), expected.next_int_unbounded());
        assert_eq!(w.get_block_id(8, 64, 8), 0);
        assert!(w.end_entity_spawns10.is_empty());
    }
}
impl EndGenerator {
    pub fn new(seed: i64) -> Self {
        let mut rand = JavaRandom::new(seed);
        let min = OctavePerlin::new(&mut rand, 16);
        let max = OctavePerlin::new(&mut rand, 16);
        let main = OctavePerlin::new(&mut rand, 8);
        let scale = OctavePerlin::new(&mut rand, 10);
        let depth = OctavePerlin::new(&mut rand, 16);
        Self {
            rand,
            min,
            max,
            main,
            scale,
            depth,
            biome_source: BiomeSource::sky(),
        }
    }
    pub fn rand_mut(&mut self) -> &mut JavaRandom {
        &mut self.rand
    }
    pub fn generate(&mut self, cx: i32, cz: i32) -> RawChunk {
        self.rand.set_seed(
            (cx as i64)
                .wrapping_mul(341873128712)
                .wrapping_add((cz as i64).wrapping_mul(132897987541)),
        );
        let mut blocks: Box<[u8; CHUNK_VOLUME]> =
            vec![0; CHUNK_VOLUME].into_boxed_slice().try_into().unwrap();
        let mut biomes = Vec::new();
        self.biome_source
            .biomes(&mut biomes, cx.wrapping_mul(16), cz.wrapping_mul(16), 16, 16);
        let d = self.density(cx.wrapping_mul(2), cz.wrapping_mul(2));
        for a in 0..2 {
            for b in 0..2 {
                for c in 0..32 {
                    let at = |a: usize, b: usize, c: usize| d[(a * 3 + b) * 33 + c];
                    let mut p = at(a, b, c);
                    let mut q = at(a, b + 1, c);
                    let mut r = at(a + 1, b, c);
                    let mut s = at(a + 1, b + 1, c);
                    let dp = (at(a, b, c + 1) - p) * 0.25;
                    let dq = (at(a, b + 1, c + 1) - q) * 0.25;
                    let dr = (at(a + 1, b, c + 1) - r) * 0.25;
                    let ds = (at(a + 1, b + 1, c + 1) - s) * 0.25;
                    for y in 0..4 {
                        let mut left = p;
                        let mut right = q;
                        let dl = (r - p) * 0.125;
                        let dt = (s - q) * 0.125;
                        for x in 0..8 {
                            let mut value = left;
                            let step = (right - left) * 0.125;
                            for z in 0..8 {
                                blocks[index(a * 8 + x, c * 4 + y, b * 8 + z)] =
                                    if value > 0.0 { 121 } else { 0 };
                                value += step;
                            }
                            left += dl;
                            right += dt;
                        }
                        p += dp;
                        q += dq;
                        r += dr;
                        s += ds;
                    }
                }
            }
        }
        self.surface(cx, cz, &mut blocks[..], &biomes);
        RawChunk {
            x: cx,
            z: cz,
            blocks,
            biomes,
        }
    }
    fn density(&self, x: i32, z: i32) -> Vec<f64> {
        let mut scale = Vec::new();
        let mut depth = Vec::new();
        let mut main = Vec::new();
        let mut min = Vec::new();
        let mut max = Vec::new();
        self.scale
            .generate_2d(&mut scale, x, z, 3, 3, 1.121, 1.121, 0.5);
        self.depth
            .generate_2d(&mut depth, x, z, 3, 3, 200.0, 200.0, 0.5);
        let hs = 684.412 * 2.0;
        let vs = 684.412;
        self.main.generate(
            &mut main,
            x as f64,
            0.0,
            z as f64,
            3,
            33,
            3,
            hs / 80.0,
            vs / 160.0,
            hs / 80.0,
        );
        self.min
            .generate(&mut min, x as f64, 0.0, z as f64, 3, 33, 3, hs, vs, hs);
        self.max
            .generate(&mut max, x as f64, 0.0, z as f64, 3, 33, 3, hs, vs, hs);
        // Both 2D fields are sampled but their height transform never affects the result.
        let mut out = vec![0.0; 297];
        for a in 0..3 {
            for b in 0..3 {
                let fx = (a as i32).wrapping_add(x) as f32;
                let fz = (b as i32).wrapping_add(z) as f32;
                let radius = ((fx * fx + fz * fz) as f64).sqrt() as f32;
                let island = (100.0f32 - radius * 8.0f32).clamp(-100.0, 80.0);
                for y in 0..33 {
                    let i = (a * 3 + b) * 33 + y;
                    let lo = min[i] / 512.0;
                    let hi = max[i] / 512.0;
                    let blend = (main[i] / 10.0 + 1.0) / 2.0;
                    let mut n = if blend < 0.0 {
                        lo
                    } else if blend > 1.0 {
                        hi
                    } else {
                        lo + (hi - lo) * blend
                    };
                    n -= 8.0;
                    n += island as f64;
                    if y > 14 {
                        let f = ((y - 14) as f32 / 64.0f32) as f64;
                        n = n * (1.0 - f) + -3000.0 * f;
                    }
                    if y < 8 {
                        let f = ((8 - y) as f32 / 7.0f32) as f64;
                        n = n * (1.0 - f) + -30.0 * f;
                    }
                    out[i] = n;
                }
            }
        }
        out
    }
    // func_40381_b has constant thickness 1 and only replaces stone (id 1).
    // The density pass writes end stone, so this pass consumes no RNG and makes no changes.
    fn surface(&mut self, _cx: i32, _cz: i32, blocks: &mut [u8], _biomes: &[Biome]) {
        for z in 0..16 {
            for x in 0..16 {
                let mut left = -1;
                for y in (0..128).rev() {
                    let i = index(x, y, z);
                    match blocks[i] {
                        0 => left = -1,
                        1 => {
                            if left == -1 { left = 1; blocks[i] = 121; }
                            else if left > 0 { left -= 1; blocks[i] = 121; }
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}
