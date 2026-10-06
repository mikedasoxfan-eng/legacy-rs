//! Beta 1.8.1 PlayerList / PlayerListEntry (the original long-key hash table).
//! Inserting an existing key updates every matching entry AND adds another entry.
//! This vanilla bug, bucket order and resize threshold are intentional.
#[derive(Clone, Debug)]
pub struct PlayerListEntry<V> {
    pub key: i64,
    pub value: V,
    pub hash: i32,
}
impl<V: PartialEq> PartialEq for PlayerListEntry<V> {
    fn eq(&self, other:&Self)->bool { self.key==other.key && self.value==other.value }
}
impl<V: Eq> Eq for PlayerListEntry<V> {}
impl<V: std::fmt::Display> std::fmt::Display for PlayerListEntry<Option<V>> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}=",self.key)?;
        match &self.value {Some(value)=>write!(f,"{value}"),None=>write!(f,"null")}
    }
}
pub fn hash(key: i64) -> i32 {
    let mut h=(key as u64 ^ (key as u64 >> 32)) as u32;
    h ^= (h >> 20) ^ (h >> 12);
    (h ^ (h >> 7) ^ (h >> 4)) as i32
}
pub struct PlayerList<V> {
    buckets: Vec<Vec<PlayerListEntry<V>>>,
    size: i32,
    threshold: i32,
    modifications: i32,
}
impl<V: Clone> Default for PlayerList<V> {
    fn default() -> Self {Self {buckets:(0..16).map(|_|Vec::new()).collect(),size:0,threshold:12,modifications:0}}
}
impl<V: Clone> PlayerList<V> {
    pub fn len(&self) -> i32 { self.size }
    pub fn is_empty(&self) -> bool { self.size == 0 }
    fn bucket(&self,key:i64)->usize { hash(key) as u32 as usize & (self.buckets.len()-1) }
    pub fn entry(&self,key:i64)->Option<&PlayerListEntry<V>> {self.buckets[self.bucket(key)].iter().find(|entry|entry.key==key)}
    pub fn get(&self,key:i64)->Option<&V> {self.entry(key).map(|entry|&entry.value)}
    pub fn contains(&self,key:i64)->bool {self.entry(key).is_some()}
    pub fn insert(&mut self,key:i64,value:V) {
        let bucket=self.bucket(key);
        for entry in &mut self.buckets[bucket] {if entry.key==key {entry.value=value.clone();}}
        self.modifications=self.modifications.wrapping_add(1);
        self.buckets[bucket].insert(0,PlayerListEntry {key,value,hash:hash(key)});
        let previous=self.size;self.size=self.size.wrapping_add(1);
        if previous>=self.threshold {self.resize();}
    }
    fn resize(&mut self) {
        let length=self.buckets.len();
        if length==1<<30 {self.threshold=i32::MAX;return;}
        let mut buckets:Vec<Vec<PlayerListEntry<V>>>=(0..length*2).map(|_|Vec::new()).collect();
        for old in &mut self.buckets {
            for entry in old.drain(..) {let bucket=entry.hash as u32 as usize & (buckets.len()-1);buckets[bucket].insert(0,entry);}
        }
        self.buckets=buckets;
        self.threshold=(self.buckets.len() as f32 * (12.0f32/16.0)) as i32;
    }
    pub fn remove_entry(&mut self,key:i64)->Option<PlayerListEntry<V>> {
        let bucket=self.bucket(key);
        let index=self.buckets[bucket].iter().position(|entry|entry.key==key)?;
        self.modifications=self.modifications.wrapping_add(1);self.size=self.size.wrapping_sub(1);
        Some(self.buckets[bucket].remove(index))
    }
    pub fn remove(&mut self,key:i64)->Option<V> {self.remove_entry(key).map(|entry|entry.value)}
    pub fn buckets(&self)->&[Vec<PlayerListEntry<V>>] {&self.buckets}
    pub fn threshold(&self)->i32 {self.threshold}
    pub fn modifications(&self)->i32 {self.modifications}
}
