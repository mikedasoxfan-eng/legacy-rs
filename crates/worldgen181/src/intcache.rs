//! Literal `IntCache` pool semantics. The padded, stale tail is observable in biome searches.
use std::cell::RefCell;
thread_local! { pub(crate) static CACHE: RefCell<IntCache> = RefCell::new(IntCache::default()); }
pub(crate) struct IntCache {
    max: usize,
    generation: usize,
    small_free: Vec<Vec<i32>>,
    small_used: Vec<Vec<i32>>,
    large_free: Vec<Vec<i32>>,
    large_used: Vec<Vec<i32>>,
}
pub(crate) struct Allocation {
    small: bool,
    index: usize,
    generation: usize,
}
impl Default for IntCache {
    fn default() -> Self {
        Self {
            max: 256,
            generation: 0,
            small_free: Vec::new(),
            small_used: Vec::new(),
            large_free: Vec::new(),
            large_used: Vec::new(),
        }
    }
}
impl IntCache {
    pub fn reset(&mut self) {
        self.large_free.pop();
        self.small_free.pop();
        self.large_free.append(&mut self.large_used);
        self.small_free.append(&mut self.small_used);
    }
    pub fn allocate(&mut self, n: usize) -> (Vec<i32>, Allocation) {
        let small = n <= 256;
        if !small && n > self.max {
            self.max = n;
            self.generation += 1;
            self.large_free.clear();
            self.large_used.clear();
        }
        let (free, used) = if small {
            (&mut self.small_free, &mut self.small_used)
        } else {
            (&mut self.large_free, &mut self.large_used)
        };
        let v = free
            .pop()
            .unwrap_or_else(|| vec![0; if small { 256 } else { self.max }]);
        let allocation = Allocation {
            small,
            index: used.len(),
            generation: self.generation,
        };
        used.push(Vec::new());
        (v, allocation)
    }
    pub fn finish(&mut self, v: &[i32], a: Allocation) {
        if a.small {
            self.small_used[a.index] = v.to_vec();
        } else if a.generation == self.generation {
            self.large_used[a.index] = v.to_vec();
        }
    }
}
