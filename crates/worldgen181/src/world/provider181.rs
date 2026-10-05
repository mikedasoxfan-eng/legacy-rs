//! The singleplayer Beta 1.8.1 ChunkProvider unload queue and rotating scan.
use super::World;
use crate::{
    entity::Vec3D,
    structures::{long_map_order::LongMapOrder, ChunkCoordIntPair},
};
#[derive(Default)]
pub struct ProviderChunks {
    pub(crate) dropped: LongMapOrder<i64>,
    pub scan_cursor: usize,
}
fn hash(k: i64) -> i32 {
    (k ^ ((k as u64 >> 32) as i64)) as i32
}
impl ProviderChunks {
    pub fn dropped(&self) -> Vec<i64> {
        self.dropped.iter().collect()
    }
    pub fn cancel(&mut self, x: i32, z: i32) {
        let k = ChunkCoordIntPair { x, z }.packed();
        self.dropped.remove(hash(k), k);
    }
}
impl World {
    /// ChunkProvider.func_35391_d; spawn protection uses wrapped Java int math.
    pub fn provider_drop_chunk(&mut self, x: i32, z: i32) {
        let dx = x
            .wrapping_mul(16)
            .wrapping_add(8)
            .wrapping_sub(self.spawn.0);
        let dz = z
            .wrapping_mul(16)
            .wrapping_add(8)
            .wrapping_sub(self.spawn.2);
        if !(-128..=128).contains(&dx) || !(-128..=128).contains(&dz) {
            let k = ChunkCoordIntPair { x, z }.packed();
            self.provider.dropped.insert(hash(k), k);
        }
    }
    /// ChunkProvider.loadChunk cancels a pending drop even on a cache hit.
    pub fn provider_load_chunk(&mut self, x: i32, z: i32) {
        self.provider.cancel(x, z);
        self.provide_chunk(x, z);
    }
    pub(crate) fn unload_provider_chunks(&mut self) {
        for _ in 0..100 {
            let Some(k) = self.provider.dropped.iter().next() else {
                break;
            };
            let p = (k as i32, (k as u64 >> 32) as i32);
            // onChunkUnload precedes saveChunk in the original provider.
            self.unload_chunk_entities(p.0, p.1);
            self.save_chunk(p.0, p.1);
            self.provider.dropped.remove(hash(k), k);
            self.detach_unloaded_chunk_tiles181(p.0,p.1);
            self.chunks.remove(&p);
            if let Some(i) = self.chunk_list.iter().position(|&v| v == p) {
                self.chunk_list.remove(i);
            }
            self.unloaded.push(p);
        }
        for _ in 0..10 {
            if self.provider.scan_cursor >= self.chunk_list.len() {
                self.provider.scan_cursor = 0;
                break;
            }
            let (x, z) = self.chunk_list[self.provider.scan_cursor];
            self.provider.scan_cursor += 1;
            if self
                .closest_player(
                    Vec3D::new((x << 4) as f64 + 8.0, 64.0, (z << 4) as f64 + 8.0),
                    288.0,
                )
                .is_none()
            {
                self.provider_drop_chunk(x, z);
            }
        }
    }
}
