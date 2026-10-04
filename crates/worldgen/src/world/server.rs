//! The multiplayer server's chunk handling (`ChunkProviderServer`, `PlayerManager`,
//! `PlayerInstance`): chunks load around players and unload when nobody can see them.
//! Singleplayer b1.7.3 never unloads chunks; this is opt-in.

use super::World;
use std::collections::HashSet;

/// `ChunkCoordIntPair.chunkXZ2Int`
pub fn chunk_xz_to_int(x: i32, z: i32) -> i32 {
    (if x < 0 { i32::MIN } else { 0 }) | (x & 0x7fff) << 16 | (if z < 0 { 0x8000 } else { 0 }) | z & 0x7fff
}

/// `java.util.HashSet<Integer>` with Java 8's iteration order (bucket index, then insertion
/// order within a bucket; resizes split buckets stably). Tree bins are not modelled.
#[derive(Default)]
pub struct JavaIntSet {
    table: Vec<Vec<i32>>,
    size: usize,
}

impl JavaIntSet {
    #[inline]
    fn index(k: i32, n: usize) -> usize {
        let h = k as u32;
        ((h ^ (h >> 16)) as usize) & (n - 1)
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn contains(&self, k: i32) -> bool {
        !self.table.is_empty() && self.table[Self::index(k, self.table.len())].contains(&k)
    }

    pub fn insert(&mut self, k: i32) -> bool {
        if self.table.is_empty() {
            self.table = vec![Vec::new(); 16];
        }
        let n = self.table.len();
        let b = &mut self.table[Self::index(k, n)];
        if b.contains(&k) {
            return false;
        }
        b.push(k);
        self.size += 1;
        if self.size > n * 3 / 4 {
            let mut t = vec![Vec::new(); n * 2];
            for bucket in &self.table {
                for &v in bucket {
                    t[Self::index(v, n * 2)].push(v);
                }
            }
            self.table = t;
        }
        true
    }

    pub fn remove(&mut self, k: i32) -> bool {
        if self.table.is_empty() {
            return false;
        }
        let n = self.table.len();
        let b = &mut self.table[Self::index(k, n)];
        if let Some(i) = b.iter().position(|&v| v == k) {
            b.remove(i);
            self.size -= 1;
            true
        } else {
            false
        }
    }

    /// `iterator().next()`
    pub fn first(&self) -> Option<i32> {
        self.table.iter().find_map(|b| b.first().copied())
    }
}

/// Server-side chunk provider state.
#[derive(Default)]
pub struct ServerChunks {
    /// `ChunkProviderServer.field_725_a`: chunks waiting to be unloaded.
    pub dropped: JavaIntSet,
    /// Reverse of `chunk_xz_to_int` for queued chunks.
    keys: std::collections::HashMap<i32, (i32, i32)>,
    /// `ChunkProviderServer.chunkLoadOverride`
    pub load_override: bool,
    /// `World.worldChunkLoadOverride`
    pub world_override: bool,
}

impl World {
    pub fn is_server(&self) -> bool {
        self.server.is_some()
    }

    /// `ChunkProviderServer.loadChunk`
    pub fn server_load_chunk(&mut self, cx: i32, cz: i32) {
        let key = chunk_xz_to_int(cx, cz);
        if let Some(s) = &mut self.server {
            s.dropped.remove(key);
            s.keys.remove(&key);
        }
        if !self.chunk_exists(cx, cz) {
            self.prepare_chunk(cx, cz);
        }
    }

    /// `ChunkProviderServer.func_374_c` (dropChunk): queue for unloading unless near spawn.
    pub fn drop_chunk(&mut self, cx: i32, cz: i32) {
        let (sx, _, sz) = self.spawn;
        let dx = cx * 16 + 8 - sx;
        let dz = cz * 16 + 8 - sz;
        let r = 128;
        if dx < -r || dx > r || dz < -r || dz > r {
            if let Some(s) = &mut self.server {
                let key = chunk_xz_to_int(cx, cz);
                s.dropped.insert(key);
                s.keys.insert(key, (cx, cz));
            }
        }
    }

    /// `unload100OldestChunks` (`ChunkProviderServer.func_361_a`). Singleplayer's queue is
    /// never filled, so this does nothing there, as in the original.
    pub fn unload_oldest_chunks(&mut self) {
        if self.level_saving {
            return;
        }
        for _ in 0..100 {
            let Some(s) = &mut self.server else { return };
            let Some(key) = s.dropped.first() else { break };
            let pos = s.keys.remove(&key).expect("queued chunk position");
            s.dropped.remove(key);
            if self.chunk_exists(pos.0, pos.1) {
                self.save_chunk(pos.0, pos.1);
                self.chunks.remove(&pos);
                if let Some(i) = self.chunk_list.iter().position(|&p| p == pos) {
                    self.chunk_list.remove(i);
                }
                self.unloaded.push(pos);
            }
        }
    }
}

/// `PlayerManager` for one player. Positions are `EntityPlayerMP.posX/posZ`.
pub struct PlayerManager {
    pub radius: i32,
    instances: HashSet<(i32, i32)>,
    /// `field_9155_d` / `field_9154_e`: where chunk tracking was last updated.
    managed: (f64, f64),
}

/// The spiral directions of `PlayerManager.field_22089_e`.
const DIRS: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl PlayerManager {
    /// `new PlayerManager(server, dim, viewRadius)`; the radius must be 3..=15.
    pub fn new(radius: i32) -> Self {
        assert!((3..=15).contains(&radius), "view radius must be 3..15");
        PlayerManager { radius, instances: HashSet::new(), managed: (0.0, 0.0) }
    }

    /// `getPlayerInstance(x, z, true).addPlayer(player)`: creating the instance loads the chunk.
    fn add(&mut self, w: &mut World, cx: i32, cz: i32) {
        if self.instances.insert((cx, cz)) {
            w.server_load_chunk(cx, cz);
        }
    }

    /// `PlayerInstance.removePlayer`: the last viewer leaving queues the chunk for unloading.
    fn remove(&mut self, w: &mut World, cx: i32, cz: i32) {
        if self.instances.remove(&(cx, cz)) {
            w.drop_chunk(cx, cz);
        }
    }

    /// `addPlayer`: load the view square as an outward spiral.
    pub fn add_player(&mut self, w: &mut World, x: f64, z: f64) {
        let cx = (x as i32) >> 4;
        let cz = (z as i32) >> 4;
        self.managed = (x, z);
        let (mut dir, mut ox, mut oz) = (0usize, 0, 0);
        self.add(w, cx, cz);
        for len in 1..=self.radius * 2 {
            for _ in 0..2 {
                let d = DIRS[dir % 4];
                dir += 1;
                for _ in 0..len {
                    ox += d.0;
                    oz += d.1;
                    self.add(w, cx + ox, cz + oz);
                }
            }
        }
        dir %= 4;
        for _ in 0..self.radius * 2 {
            ox += DIRS[dir].0;
            oz += DIRS[dir].1;
            self.add(w, cx + ox, cz + oz);
        }
    }

    fn in_square(&self, x: i32, z: i32, cx: i32, cz: i32) -> bool {
        let (dx, dz) = (x - cx, z - cz);
        dx >= -self.radius && dx <= self.radius && dz >= -self.radius && dz <= self.radius
    }

    /// `func_543_c`: after moving 8+ blocks into another chunk, load the new edge and
    /// release the old one.
    pub fn update_player(&mut self, w: &mut World, x: f64, z: f64) {
        let cx = (x as i32) >> 4;
        let cz = (z as i32) >> 4;
        let dx = self.managed.0 - x;
        let dz = self.managed.1 - z;
        if dx * dx + dz * dz < 64.0 {
            return;
        }
        let ox = (self.managed.0 as i32) >> 4;
        let oz = (self.managed.1 as i32) >> 4;
        let (mx, mz) = (cx - ox, cz - oz);
        if mx == 0 && mz == 0 {
            return;
        }
        for x in cx - self.radius..=cx + self.radius {
            for z in cz - self.radius..=cz + self.radius {
                if !self.in_square(x, z, ox, oz) {
                    self.add(w, x, z);
                }
                if !self.in_square(x - mx, z - mz, cx, cz) {
                    self.remove(w, x - mx, z - mz);
                }
            }
        }
        self.managed = (x, z);
    }

    /// `removePlayer`
    pub fn remove_player(&mut self, w: &mut World) {
        let cx = (self.managed.0 as i32) >> 4;
        let cz = (self.managed.1 as i32) >> 4;
        for x in cx - self.radius..=cx + self.radius {
            for z in cz - self.radius..=cz + self.radius {
                self.remove(w, x, z);
            }
        }
    }

    pub fn is_tracked(&self, cx: i32, cz: i32) -> bool {
        self.instances.contains(&(cx, cz))
    }
}
