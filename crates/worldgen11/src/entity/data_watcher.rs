//! Beta 1.8.1 DataWatcher wire format and Java object equality semantics.
use crate::world::java_hash::JavaHashSet;
use std::{io::{self, Read}, sync::{Arc, Mutex},};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChunkCoordinates { pub x: i32, pub y: i32, pub z: i32, }
impl ChunkCoordinates {
    pub fn hash_code(self) -> i32 {
        self.x.wrapping_add(self.z).wrapping_shl(8i32.wrapping_add(self.y) as u32).wrapping_shl(16)
    }
    pub fn compare(self, other: Self) -> i32 {
        if self.y != other.y { self.y.wrapping_sub(other.y) }
        else if self.z != other.z { self.z.wrapping_sub(other.z) }
        else { self.x.wrapping_sub(other.x) }
    }
    /// Despite the name, getSqDistanceTo returns a square root; int overflow is retained.
    pub fn sq_distance_to(self, x: i32, y: i32, z: i32) -> f64 {
        let a=self.x.wrapping_sub(x);let b=self.y.wrapping_sub(y);let c=self.z.wrapping_sub(z);
        (a.wrapping_mul(a).wrapping_add(b.wrapping_mul(b)).wrapping_add(c.wrapping_mul(c)) as f64).sqrt()
    }
}
#[derive(Clone, Debug)]
pub struct WatchItem { pub id: i32, pub count: i32, pub damage: i32, }
#[derive(Clone, Debug)]
pub enum Value {
    Byte(i8), Short(i16), Int(i32), Float(f32),
    /// Java strings can contain unpaired UTF-16 surrogates.
    String(Vec<u16>), Item(Arc<Mutex<WatchItem>>), Coordinates(ChunkCoordinates),
}
fn float_bits(v: f32) -> u32 { if v.is_nan() { 0x7fc00000 } else { v.to_bits() } }
impl PartialEq for Value {
    fn eq(&self, b: &Self) -> bool {
        match (self,b) {
            (Self::Byte(a),Self::Byte(b)) => a==b, (Self::Short(a),Self::Short(b)) => a==b,
            (Self::Int(a),Self::Int(b)) => a==b, (Self::Float(a),Self::Float(b)) => float_bits(*a)==float_bits(*b),
            (Self::String(a),Self::String(b)) => a==b, (Self::Item(a),Self::Item(b)) => Arc::ptr_eq(a,b),
            (Self::Coordinates(a),Self::Coordinates(b)) => a==b, _ => false,
        }
    }
}
impl Value {
    pub fn kind(&self) -> i32 { match self { Self::Byte(_)=>0,Self::Short(_)=>1,Self::Int(_)=>2,Self::Float(_)=>3,Self::String(_)=>4,Self::Item(_)=>5,Self::Coordinates(_)=>6, } }
}
#[derive(Clone, Debug)]
pub struct WatchableObject { pub object_type: i32, pub id: i32, pub value: Value, pub watching: bool, }
impl WatchableObject {
    pub fn new(id: i32, value: Value) -> Self { Self {object_type:value.kind(),id,value,watching:true,} }
}
#[derive(Clone, Default)]
pub struct DataWatcher {
    order: JavaHashSet<i32>,
    objects: std::collections::BTreeMap<i32, WatchableObject>,
    pub object_changed: bool,
}
impl DataWatcher {
    pub fn add(&mut self, id: i32, value: Value) -> Result<(), String> {
        if id>31 { return Err(format!("Data value id is too big with {id}! (Max is 31)")); }
        if self.objects.contains_key(&id) { return Err(format!("Duplicate id value for {id}!")); }
        self.order.insert(id,id); self.objects.insert(id,WatchableObject::new(id,value)); Ok(())
    }
    pub fn get(&self, id: i32) -> Option<&WatchableObject> { self.objects.get(&id) }
    pub fn byte(&self, id: i32) -> Option<i8> { if let Value::Byte(v) = self.get(id)?.value {
            Some(v)
        } else {
            None
        }
    }
    pub fn short(&self, id: i32) -> Option<i16> {
        if let Value::Short(v)=self.get(id)?.value {Some(v)} else {None} }
    pub fn int(&self, id: i32) -> Option<i32> { if let Value::Int(v)=self.get(id)?.value {Some(v)} else {None} }
    pub fn string(&self, id: i32) -> Option<&[u16]> { if let Value::String(v)=&self.get(id)?.value {Some(v)} else {None} }
    pub fn update(&mut self, id: i32, value: Value) -> Result<(), String> {
        let object=self.objects.get_mut(&id).ok_or_else(|| format!("Missing watched id {id}"))?;
        if value != object.value { object.value=value;object.watching=true;self.object_changed=true; }
        Ok(())
    }
    pub fn update_from_list(&mut self, list: &[WatchableObject]) {
        for object in list { if let Some(existing)=self.objects.get_mut(&object.id) {existing.value=object.value.clone();} }
    }
    pub fn objects(&self) -> Vec<WatchableObject> { self.order.iter().map(|id|self.objects[&id].clone()).collect() }
    pub fn write(&self) -> io::Result<Vec<u8>> { write_list(Some(&self.objects())) }
}
pub fn write_list(list: Option<&[WatchableObject]>) -> io::Result<Vec<u8>> {
    let mut out=Vec::new();
    for object in list.unwrap_or(&[]) {
        out.push(((object.object_type<<5 | object.id & 31)&255) as u8);
        if object.object_type != object.value.kind() {return Err(io::Error::new(io::ErrorKind::InvalidData,"Watcher type mismatch",));}
        match &object.value {
            Value::Byte(v)=>out.push(*v as u8), Value::Short(v)=>out.extend(v.to_be_bytes()),
            Value::Int(v)=>out.extend(v.to_be_bytes()), Value::Float(v)=>out.extend(float_bits(*v).to_be_bytes()),
            Value::String(v)=> { if v.len()>32767 {return Err(io::Error::new(io::ErrorKind::InvalidData,"String too big"));}out.extend((v.len() as i16).to_be_bytes());for c in v {out.extend(c.to_be_bytes());} }
            Value::Item(v)=> {let v=v.lock().unwrap();out.extend((v.id as i16).to_be_bytes());out.push(v.count as u8);out.extend((v.damage as i16).to_be_bytes());}
            Value::Coordinates(v)=> {for v in [v.x,v.y,v.z] {out.extend(v.to_be_bytes());}}
        }
    }
    out.push(127);Ok(out)
}
pub fn read_list(input: &mut impl Read) -> io::Result<Option<Vec<Option<WatchableObject>>>> {
    fn bytes<const N:usize>(r:&mut impl Read)->io::Result<[u8;N]>{let mut b=[0;N];r.read_exact(&mut b)?;Ok(b)}
    let mut list=None;
    loop {
        let header=bytes::<1>(input)?[0];if header==127 {return Ok(list);}
        let kind=(header&224)>>5;let id=(header&31) as i32;
        let value=match kind {
            0=>Some(Value::Byte(bytes::<1>(input)?[0] as i8)),1=>Some(Value::Short(i16::from_be_bytes(bytes(input)?))),
            2=>Some(Value::Int(i32::from_be_bytes(bytes(input)?))),3=>Some(Value::Float(f32::from_bits(u32::from_be_bytes(bytes(input,)?)))),
            4=> {let len=i16::from_be_bytes(bytes(input)?);if !(0..=64).contains(&len){return Err(io::Error::new(io::ErrorKind::InvalidData,"Invalid watcher string length",));}let mut chars=Vec::new();for _ in 0..len{chars.push(u16::from_be_bytes(bytes(input)?));}Some(Value::String(chars))}
            5=>Some(Value::Item(Arc::new(Mutex::new(WatchItem{id:i16::from_be_bytes(bytes(input)?) as i32,count:bytes::<1>(input)?[0] as i8 as i32,damage:i16::from_be_bytes(bytes(input)?) as i32,})))),
            6=>Some(Value::Coordinates(ChunkCoordinates{x:i32::from_be_bytes(bytes(input)?),y:i32::from_be_bytes(bytes(input)?),z:i32::from_be_bytes(bytes(input)?),})),
            _=>None,
        };
        list.get_or_insert_with(Vec::new).push(value.map(|value|WatchableObject {object_type:kind as i32,id,value,watching:true,}));
    }
}

impl std::fmt::Debug for DataWatcher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DataWatcher").field("objects", &self.objects)
            .field("object_changed", &self.object_changed).finish()
    }
}
