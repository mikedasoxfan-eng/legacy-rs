//! The alternate, 32x32 direct-mapped ChunkProviderLoadOrGenerate.
//! Vanilla's normal World uses ChunkProvider; this cache remains an independent
//! IChunkProvider implementation, with its loader/generator callbacks supplied by a host.
use super::chunk::Chunk;
use std::{cell::RefCell, rc::Rc};
pub type ChunkHandle = Rc<RefCell<Chunk>>;
/// IChunkLoader, underlying IChunkProvider and World callbacks. Loader failures
/// are represented as None after the host has reported the exception.
pub trait CacheHost {
    fn world_time(&self) -> i64;
    fn finding_spawn(&self) -> bool;
    fn has_loader(&self) -> bool;
    fn has_generator(&self) -> bool;
    fn load(&mut self, x: i32, z: i32) -> Result<Option<ChunkHandle>, String>;
    fn generate(&mut self, x: i32, z: i32) -> ChunkHandle;
    fn translate(&mut self, c: &ChunkHandle);
    fn on_load(&mut self, c: &ChunkHandle);
    fn on_unload(&mut self, c: &ChunkHandle);
    fn populate(&mut self, x: i32, z: i32);
    fn save(&mut self, c: &ChunkHandle);
    fn save_extra(&mut self, c: &ChunkHandle);
    fn flush(&mut self);
    fn tick_loader(&mut self);
    fn unload_generator(&mut self) -> bool;
}
pub struct LegacyChunkCache {
    pub blank: ChunkHandle,
    pub chunks: Vec<Option<ChunkHandle>>,
    pub last_x: i32,
    pub last_z: i32,
    pub last: Option<ChunkHandle>,
    pub current_x: i32,
    pub current_z: i32,
}
impl LegacyChunkCache {
    pub fn new(blank: ChunkHandle) -> Self {
        Self {
            blank,
            chunks: vec![None; 1024],
            last_x: 0,
            last_z: 0,
            last: None,
            current_x: 0,
            current_z: 0,
        }
    }
    pub fn set_current_chunk_over(&mut self, x: i32, z: i32) {
        self.current_x = x;
        self.current_z = z;
    }
    pub fn can_chunk_exist(&self, x: i32, z: i32) -> bool {
        x >= self.current_x.wrapping_sub(15)
            && z >= self.current_z.wrapping_sub(15)
            && x <= self.current_x.wrapping_add(15)
            && z <= self.current_z.wrapping_add(15)
    }
    fn index(x: i32, z: i32) -> usize {
        ((x & 31) + (z & 31) * 32) as usize
    }
    pub fn chunk_exists(&self, x: i32, z: i32) -> bool {
        if !self.can_chunk_exist(x, z) {
            return false;
        }
        if x == self.last_x && z == self.last_z && self.last.is_some() {
            return true;
        }
        self.chunks[Self::index(x, z)].as_ref().is_some_and(|c| {
            Rc::ptr_eq(c, &self.blank) || {
                let c = c.borrow();
                c.x == x && c.z == z
            }
        })
    }
    pub fn load_chunk(&mut self, h: &mut impl CacheHost, x: i32, z: i32) -> ChunkHandle {
        self.provide_chunk(h, x, z)
    }
    pub fn provide_chunk(&mut self, h: &mut impl CacheHost, x: i32, z: i32) -> ChunkHandle {
        // The last queried chunk wins even after the current window moves.
        if x == self.last_x && z == self.last_z {
            if let Some(c) = &self.last {
                return c.clone();
            }
        }
        if !h.finding_spawn() && !self.can_chunk_exist(x, z) {
            return self.blank.clone();
        }
        let i = Self::index(x, z);
        if !self.chunk_exists(x, z) {
            if let Some(c) = self.chunks[i].clone() {
                h.on_unload(&c);
                self.save(h, &c);
                if h.has_loader() {
                    h.save_extra(&c);
                }
            }
            let loaded = if !h.has_loader() {
                Some(self.blank.clone())
            } else {
                match h.load(x, z) {
                    Ok(c) => {
                        if let Some(c) = &c {
                            c.borrow_mut().last_save_time = h.world_time();
                        }
                        c
                    }
                    Err(_) => Some(self.blank.clone()),
                }
            };
            let c = loaded.unwrap_or_else(|| {
                if h.has_generator() {
                    let c = h.generate(x, z);
                    h.translate(&c);
                    c
                } else {
                    self.blank.clone()
                }
            });
            self.chunks[i] = Some(c.clone());
            h.on_load(&c);
            self.check_population(h, x, z);
        }
        self.last_x = x;
        self.last_z = z;
        self.last = self.chunks[i].clone();
        self.last.as_ref().unwrap().clone()
    }
    fn check_population(&mut self, h: &mut impl CacheHost, x: i32, z: i32) {
        let populated = self.chunks[Self::index(x, z)]
            .as_ref()
            .unwrap()
            .borrow()
            .populated;
        if !populated
            && self.chunk_exists(x.wrapping_add(1), z.wrapping_add(1))
            && self.chunk_exists(x, z.wrapping_add(1))
            && self.chunk_exists(x.wrapping_add(1), z)
        {
            self.populate(h, x, z);
        }
        if self.chunk_exists(x.wrapping_sub(1), z)
            && !self
                .provide_chunk(h, x.wrapping_sub(1), z)
                .borrow()
                .populated
            && self.chunk_exists(x.wrapping_sub(1), z.wrapping_add(1))
            && self.chunk_exists(x, z.wrapping_add(1))
            && self.chunk_exists(x.wrapping_sub(1), z.wrapping_add(1))
        {
            self.populate(h, x.wrapping_sub(1), z);
        }
        if self.chunk_exists(x, z.wrapping_sub(1))
            && !self
                .provide_chunk(h, x, z.wrapping_sub(1))
                .borrow()
                .populated
            && self.chunk_exists(x.wrapping_add(1), z.wrapping_sub(1))
            && self.chunk_exists(x.wrapping_add(1), z.wrapping_sub(1))
            && self.chunk_exists(x.wrapping_add(1), z)
        {
            self.populate(h, x, z.wrapping_sub(1));
        }
        if self.chunk_exists(x.wrapping_sub(1), z.wrapping_sub(1))
            && !self
                .provide_chunk(h, x.wrapping_sub(1), z.wrapping_sub(1))
                .borrow()
                .populated
            && self.chunk_exists(x, z.wrapping_sub(1))
            && self.chunk_exists(x.wrapping_sub(1), z)
        {
            self.populate(h, x.wrapping_sub(1), z.wrapping_sub(1));
        }
    }
    pub fn populate(&mut self, h: &mut impl CacheHost, x: i32, z: i32) {
        let c = self.provide_chunk(h, x, z);
        let populated = c.borrow().populated;
        if !populated {
            c.borrow_mut().populated = true;
            if h.has_generator() {
                h.populate(x, z);
                if !Rc::ptr_eq(&c, &self.blank) {
                    c.borrow_mut().modified = true;
                }
            }
        }
    }
    fn save(&self, h: &mut impl CacheHost, c: &ChunkHandle) {
        if h.has_loader() {
            c.borrow_mut().last_save_time = h.world_time();
            h.save(c);
        }
    }
    fn needs_saving(c: &ChunkHandle, time: i64, all: bool) -> bool {
        let c = c.borrow();
        !c.never_save
            && (c.modified
                || (c.has_entities
                    && if all {
                        time != c.last_save_time
                    } else {
                        time >= c.last_save_time.wrapping_add(600)
                    }))
    }
    pub fn save_chunks(
        &mut self,
        h: &mut impl CacheHost,
        all: bool,
        mut progress: Option<&mut dyn FnMut(i32)>,
    ) -> bool {
        let count = if progress.is_some() {
            self.chunks
                .iter()
                .flatten()
                .filter(|c| Self::needs_saving(c, h.world_time(), all))
                .count()
        } else {
            0
        };
        let mut saved = 0;
        let mut done = 0;
        for c in self.chunks.iter().flatten() {
            if all && !c.borrow().never_save && h.has_loader() {
                h.save_extra(c);
            }
            if Self::needs_saving(c, h.world_time(), all) {
                self.save(h, c);
                c.borrow_mut().modified = false;
                saved += 1;
                if saved == 2 && !all {
                    return false;
                }
                if let Some(p) = progress.as_mut() {
                    done += 1;
                    if done % 10 == 0 {
                        p(done * 100 / count as i32);
                    }
                }
            }
        }
        if all && h.has_loader() {
            h.flush();
        }
        true
    }
    pub fn unload100_oldest_chunks(&mut self, h: &mut impl CacheHost) -> bool {
        if h.has_loader() {
            h.tick_loader();
        }
        h.unload_generator()
    }
    pub fn can_save(&self) -> bool {
        true
    }
    pub fn make_string(&self) -> String {
        format!("ChunkCache: {}", self.chunks.len())
    }
}
