//! Beta 1.8.1 `WeightedRandom`, `WeightedRandomChoice` and `SpawnListEntry`.
use crate::JavaRandom;
pub trait Weighted { fn weight(&self)->i32; }
#[derive(Clone,Copy,Debug)]
pub struct WeightedRandomChoice {pub weight:i32}
impl Weighted for WeightedRandomChoice {fn weight(&self)->i32 {self.weight}}
#[derive(Clone,Copy,Debug)]
pub struct SpawnListEntry {pub entity:&'static str,pub weight:i32,pub min:i32,pub max:i32}
impl Weighted for SpawnListEntry {fn weight(&self)->i32 {self.weight}}
pub fn total<T:Weighted>(choices:&[T])->i32 {choices.iter().fold(0i32,|a,b|a.wrapping_add(b.weight()))}
pub fn choose_with_total<'a,T:Weighted>(r:&mut JavaRandom,choices:&'a [T],sum:i32)->Option<&'a T> {
    assert!(sum>0,"WeightedRandom: nonpositive total");
    let mut n=r.next_int(sum);for c in choices {n=n.wrapping_sub(c.weight());if n<0{return Some(c);}}None
}
pub fn choose<'a,T:Weighted>(r:&mut JavaRandom,choices:&'a [T])->Option<&'a T> {choose_with_total(r,choices,total(choices))}
pub fn creatures(b:crate::Biome)->Vec<SpawnListEntry> {
    use crate::Biome::*;
    if matches!(b,Ocean|River|Desert|Hell) {return Vec::new();}
    if b==Sky {return vec![SpawnListEntry{entity:"Chicken",weight:10,min:4,max:4}];}
    let mut entries=vec![SpawnListEntry{entity:"Sheep",weight:12,min:4,max:4},SpawnListEntry{entity:"Pig",weight:10,min:4,max:4},SpawnListEntry{entity:"Chicken",weight:10,min:4,max:4},SpawnListEntry{entity:"Cow",weight:8,min:4,max:4}];
    if matches!(b,Forest|Taiga) {entries.push(SpawnListEntry{entity:"Wolf",weight:if b==Forest{5}else{8},min:4,max:4});}entries
}
