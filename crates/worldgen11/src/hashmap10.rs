//! IntHashMap and LongHashMap, including vanilla's duplicate-insertion bug.
pub use crate::world::mchash181::{compute_hash, Entry as IntHashMapEntry, MCHash181 as IntHashMap};

impl<T: PartialEq> PartialEq for IntHashMapEntry<T> {
    fn eq(&self, other: &Self) -> bool { self.key == other.key && self.value == other.value }
}
impl<T> IntHashMapEntry<T> {
    pub fn hash_code(&self) -> i32 { compute_hash(self.key) }
}
impl<T: std::fmt::Display> std::fmt::Display for IntHashMapEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=", self.key)?;
        if let Some(value) = &self.value { value.fmt(f) } else { f.write_str("null") }
    }
}

#[derive(Clone, Debug)]
pub struct LongHashMapEntry<T> {
    pub key: i64,
    pub value: Option<T>,
    pub next: Option<usize>,
    pub hash: i32,
}
impl<T: PartialEq> PartialEq for LongHashMapEntry<T> {
    fn eq(&self, other: &Self) -> bool { self.key == other.key && self.value == other.value }
}
impl<T> LongHashMapEntry<T> {
    pub fn hash_code(&self) -> i32 { long_hash(self.key) }
}
impl<T: std::fmt::Display> std::fmt::Display for LongHashMapEntry<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}=", self.key)?;
        if let Some(value) = &self.value { value.fmt(f) } else { f.write_str("null") }
    }
}
pub fn long_hash(key: i64) -> i32 { compute_hash((key ^ ((key as u64 >> 32) as i64)) as i32) }

pub struct LongHashMap<T> {
    pub slots: Vec<Option<usize>>,
    pub entries: Vec<LongHashMapEntry<T>>,
    pub count: usize,
    pub threshold: usize,
    pub version_stamp: i32,
}
impl<T> Default for LongHashMap<T> {
    fn default() -> Self {
        Self { slots: vec![None; 16], entries: Vec::new(), count: 0, threshold: 12, version_stamp: 0 }
    }
}
impl<T: Clone> LongHashMap<T> {
    pub fn lookup_entry(&self, key: i64) -> Option<usize> {
        let mut entry = self.slots[long_hash(key) as usize & (self.slots.len() - 1)];
        while let Some(id) = entry {
            if self.entries[id].key == key { return Some(id); }
            entry = self.entries[id].next;
        }
        None
    }
    pub fn lookup(&self, key: i64) -> Option<&T> {
        self.lookup_entry(key).and_then(|id| self.entries[id].value.as_ref())
    }
    pub fn contains(&self, key: i64) -> bool { self.lookup_entry(key).is_some() }
    pub fn add(&mut self, key: i64, value: Option<T>) {
        let hash = long_hash(key);
        let slot = hash as usize & (self.slots.len() - 1);
        let mut entry = self.slots[slot];
        while let Some(id) = entry {
            if self.entries[id].key == key { self.entries[id].value = value.clone(); }
            entry = self.entries[id].next;
        }
        self.version_stamp = self.version_stamp.wrapping_add(1);
        let id = self.entries.len();
        self.entries.push(LongHashMapEntry { key, value, hash, next: self.slots[slot] });
        self.slots[slot] = Some(id);
        let count = self.count;
        self.count += 1;
        if count >= self.threshold { self.grow(); }
    }
    fn grow(&mut self) {
        if self.slots.len() == 1 << 30 { self.threshold = i32::MAX as usize; return; }
        let mut slots = vec![None; self.slots.len() * 2];
        for old in &mut self.slots {
            let mut entry = old.take();
            while let Some(id) = entry {
                entry = self.entries[id].next;
                let slot = self.entries[id].hash as usize & (slots.len() - 1);
                self.entries[id].next = slots[slot];
                slots[slot] = Some(id);
            }
        }
        self.threshold = (slots.len() as f32 * 0.75f32) as usize;
        self.slots = slots;
    }
    pub fn remove_entry(&mut self, key: i64) -> Option<usize> {
        let slot = long_hash(key) as usize & (self.slots.len() - 1);
        let mut entry = self.slots[slot];
        let mut previous: Option<usize> = None;
        while let Some(id) = entry {
            let next = self.entries[id].next;
            if self.entries[id].key == key {
                self.version_stamp = self.version_stamp.wrapping_add(1);
                self.count -= 1;
                if let Some(previous) = previous { self.entries[previous].next = next; }
                else { self.slots[slot] = next; }
                return Some(id);
            }
            previous = entry;
            entry = next;
        }
        None
    }
    pub fn remove(&mut self, key: i64) -> Option<T> {
        self.remove_entry(key).and_then(|id| self.entries[id].value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn duplicate_keys_remain_after_one_remove_and_resize_reverses_chains() {
        let mut map = LongHashMap::default();
        let key = i64::MIN + 123;
        for value in 0..13 { map.add(key, Some(value)); }
        assert_eq!(map.count, 13);
        assert_eq!(map.slots.len(), 32);
        assert_eq!(map.lookup(key), Some(&12));
        assert_eq!(map.lookup_entry(key), Some(0)); // transfer reverses the entire chain
        assert_eq!(map.remove(key), Some(12));
        assert!(map.contains(key));
        assert_eq!(map.count, 12);
    }
    #[test]
    fn int_map_retains_key_set_when_cleared() {
        let mut map = IntHashMap::<i32>::default();
        map.add_key(-1, None);
        map.add_key(-1, Some(3));
        assert_eq!(map.count, 2);
        assert_eq!(map.keys.len(), 1);
        map.clear_map();
        assert_eq!(map.count, 0);
        assert_eq!(map.keys.len(), 1);
        assert!(!map.contains(-1));
    }
}
