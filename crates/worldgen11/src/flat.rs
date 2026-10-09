//! Release 1.1 ChunkProviderFlat: four layers and village population only.
use crate::{block, overworld::{index, RawChunk, CHUNK_VOLUME}, Biome, JavaRandom, World};

pub struct FlatGenerator { rand: JavaRandom }
impl FlatGenerator {
    pub fn new(seed: i64) -> Self { Self { rand: JavaRandom::new(seed) } }
    pub fn rand_mut(&mut self) -> &mut JavaRandom { &mut self.rand }
    pub fn generate(&mut self, x: i32, z: i32) -> RawChunk {
        let mut blocks: Box<[u8; CHUNK_VOLUME]> = vec![0; CHUNK_VOLUME].into_boxed_slice().try_into().unwrap();
        for cx in 0..16 { for cz in 0..16 {
            blocks[index(cx, 0, cz)] = block::BEDROCK;
            blocks[index(cx, 1, cz)] = block::DIRT;
            blocks[index(cx, 2, cz)] = block::DIRT;
            blocks[index(cx, 3, cz)] = block::GRASS;
        }}
        RawChunk { x, z, blocks, biomes: vec![Biome::Plains; 256] }
    }
}

pub fn populate(w: &mut World, cx: i32, cz: i32) {
    let seed = w.seed;
    let random = w.prand();
    random.set_seed(seed);
    let mx = random.next_long() / 2 * 2 + 1;
    let mz = random.next_long() / 2 * 2 + 1;
    random.set_seed((cx as i64).wrapping_mul(mx).wrapping_add((cz as i64).wrapping_mul(mz)) ^ seed);
    if w.info.map_features {
        let mut villages = std::mem::take(&mut w.villages);
        // Nested chunk generation must use the same flat village size.
        w.villages.size = villages.size;
        let mut random = w.prand().clone();
        villages.populate(w, &mut random, cx, cz);
        *w.prand() = random;
        for key in w.villages.keys() {
            let start = w.villages.starts.remove(&key).unwrap();
            villages.merge_start(key, start);
        }
        w.villages = villages;
    }
}
