//! Beta 1.8.1 MCHash. addKey deliberately inserts duplicate entries, even
//! after updating every existing matching value. clearMap leaves the key set.
use crate::structures::long_map_order::LongMapOrder;

#[derive(Clone, Debug)]
pub struct Entry<T> {
    pub key: i32,
    pub value: Option<T>,
    pub next: Option<usize>,
    pub hash: i32,
}
pub struct MCHash181<T> {
    pub slots: Vec<Option<usize>>,
    pub entries: Vec<Entry<T>>,
    pub count: usize,
    pub threshold: usize,
    pub version_stamp: i32,
    pub keys: LongMapOrder<i32>,
}
pub fn compute_hash(mut value: i32) -> i32 {
    value ^= ((value as u32 >> 20) ^ (value as u32 >> 12)) as i32;
    value ^ ((value as u32 >> 7) ^ (value as u32 >> 4)) as i32
}
impl<T: Clone> Default for MCHash181<T> {
    fn default() -> Self {
        Self {
            slots: vec![None; 16],
            entries: Vec::new(),
            count: 0,
            threshold: 12,
            version_stamp: 0,
            keys: LongMapOrder::default(),
        }
    }
}
impl<T: Clone> MCHash181<T> {
    pub fn lookup_entry(&self, key: i32) -> Option<usize> {
        let mut entry = self.slots[compute_hash(key) as usize & (self.slots.len() - 1)];
        while let Some(id) = entry {
            if self.entries[id].key == key {
                return Some(id);
            }
            entry = self.entries[id].next;
        }
        None
    }
    pub fn lookup(&self, key: i32) -> Option<&T> {
        self.lookup_entry(key)
            .and_then(|id| self.entries[id].value.as_ref())
    }
    pub fn contains(&self, key: i32) -> bool {
        self.lookup_entry(key).is_some()
    }
    pub fn add_key(&mut self, key: i32, value: Option<T>) {
        self.keys.insert(key, key);
        let hash = compute_hash(key);
        let slot = hash as usize & (self.slots.len() - 1);
        let mut entry = self.slots[slot];
        while let Some(id) = entry {
            if self.entries[id].key == key {
                self.entries[id].value = value.clone();
            }
            entry = self.entries[id].next;
        }
        self.version_stamp = self.version_stamp.wrapping_add(1);
        let id = self.entries.len();
        self.entries.push(Entry {
            key,
            value,
            next: self.slots[slot],
            hash,
        });
        self.slots[slot] = Some(id);
        let old_count = self.count;
        self.count += 1;
        if old_count >= self.threshold {
            self.grow();
        }
    }
    fn grow(&mut self) {
        if self.slots.len() == 1 << 30 {
            self.threshold = i32::MAX as usize;
            return;
        }
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
    /// Package-local removeEntry does not update field_35861_f.
    pub fn remove_entry(&mut self, key: i32) -> Option<usize> {
        let slot = compute_hash(key) as usize & (self.slots.len() - 1);
        let mut entry = self.slots[slot];
        let mut previous: Option<usize> = None;
        while let Some(id) = entry {
            let next = self.entries[id].next;
            if self.entries[id].key == key {
                self.version_stamp = self.version_stamp.wrapping_add(1);
                self.count -= 1;
                if let Some(previous) = previous {
                    self.entries[previous].next = next;
                } else {
                    self.slots[slot] = next;
                }
                return Some(id);
            }
            previous = entry;
            entry = next;
        }
        None
    }
    pub fn remove_object(&mut self, key: i32) -> Option<T> {
        self.keys.remove(key, key);
        self.remove_entry(key)
            .and_then(|id| self.entries[id].value.clone())
    }
    pub fn clear_map(&mut self) {
        self.version_stamp = self.version_stamp.wrapping_add(1);
        self.slots.fill(None);
        self.count = 0;
    }
}
