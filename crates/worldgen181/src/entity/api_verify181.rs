//! Bytecode fixtures for DataWatcher and ChunkCoordinates.
use super::data_watcher::*;
use std::{rc::Rc, cell::RefCell};
fn i(o:&mut Vec<u8>,n:i32){o.extend(n.to_le_bytes());}
fn text(o:&mut Vec<u8>,s:&str){i(o,s.len() as i32);o.extend(s.as_bytes());}
fn value(kind:i32,n:i32)->Value {
 match kind {
  0=>Value::Byte(n.wrapping_mul(17) as i8),1=>Value::Short(n.wrapping_mul(1009) as i16),2=>Value::Int(n.wrapping_mul(123456789)),
  3=>Value::Float([0.0,-0.0,f32::NAN,f32::INFINITY,-1.25][n.rem_euclid(5) as usize]),
  4=>{let mut s=vec![65,0,0xd83d,0xde00,0xd800];s.extend(n.to_string().encode_utf16());Value::String(s)},
  5=>Value::Item(Rc::new(RefCell::new(WatchItem{id:35,count:n.wrapping_mul(17),damage:n.wrapping_mul(1009) as i16 as i32}))),
  _=>Value::Coordinates(ChunkCoordinates{x:n.wrapping_mul(123456789),y:n-17,z:n.wrapping_neg().wrapping_mul(1009)})
 }
}
fn snapshot(w:&DataWatcher,o:&mut Vec<u8>){let bytes=w.write().unwrap();i(o,bytes.len() as i32);o.extend(bytes);o.push(w.object_changed as u8);let list=w.objects();i(o,list.len() as i32);for v in list {i(o,v.id);i(o,v.object_type);o.push(v.watching as u8);}}
pub fn replay(kind:&str)->Vec<u8> {
 let mut o=Vec::new();
 if kind=="registry" {
  fn optional(o:&mut Vec<u8>,v:Option<&str>) {if let Some(v)=v {text(o,v);}else {i(o,-1);}}
  for id in -10..110 {let entry=super::registry::by_id(id);optional(&mut o,entry.map(|e|e.class));optional(&mut o,entry.map(|e|e.name));}
  for name in ["Mob","Monster","Player","Fireball","ThrownEgg","FishingHook","LightningBolt","Enderman","Silverfish","CaveSpider","XPOrb","unknown","Zombie"] {optional(&mut o,super::registry::by_name(name).map(|e|e.class));}
 } else if kind=="light" {
  let mut w=super::verify181::world();use crate::world::chunk::LightKind;
  for x in -20..=40 {for z in -20..=40 {for y in 0..128 {w.set_light_value(LightKind::Sky,x,y,z,(x+y+z)&15);w.set_light_value(LightKind::Block,x,y,z,(x*3+y*7+z*5)&15);}}}
  for name in ["XPOrb","Pig","Enderman","Ghast","Slime"] {let mut e=if name=="XPOrb" {super::Entity::xp_orb_base(&mut w)}else {super::Entity::living(&mut w,name).unwrap()};
   for x in [-17.01,-0.1,0.0,15.99,33.5,30000001.0] {for y in [-4.0,0.25,63.5,127.99,256.0] {for offset in [0.0,-0.75,1.62] {for partial in [0.0,0.25,0.75,1.0] {e.y_offset=offset;e.set_position(x,y,-0.1);i(&mut o,e.packed_brightness(&mut w,partial));let v=e.brightness(&mut w);o.extend(v.to_bits().to_le_bytes());}}}}
  }
 } else if kind=="coordinates" {
  let mut r=crate::JavaRandom::new(42);
  for _ in 0..10000 {let p=ChunkCoordinates{x:r.next_int_unbounded(),y:r.next_int_unbounded(),z:r.next_int_unbounded()};let q=ChunkCoordinates{x:r.next_int_unbounded(),y:r.next_int_unbounded(),z:r.next_int_unbounded()};i(&mut o,p.hash_code());i(&mut o,p.compare(q));let d=p.sq_distance_to(q.x,q.y,q.z);o.extend((if d.is_nan(){0x7ff8000000000000}else{d.to_bits()}).to_le_bytes());o.push((p==q) as u8);o.push(1);}
 } else {
  for variant in 0..32 {
   let mut w=DataWatcher::default();for j in 0..31 {let id=(j+variant)%31;w.add(id,value(id%7,id+variant)).unwrap();}w.add(-1,Value::Byte(-128)).unwrap();snapshot(&w,&mut o);
   let first=w.write().unwrap();
   for id in 0..31 {w.update(id,value(id%7,id+variant)).unwrap();}snapshot(&w,&mut o);
   for id in 0..31 {w.update(id,value(id%7,id+variant+1)).unwrap();}snapshot(&w,&mut o);
   let list=read_list(&mut first.as_slice()).unwrap().unwrap().into_iter().map(Option::unwrap).collect::<Vec<_>>();w.update_from_list(&list);snapshot(&w,&mut o);
   for id in [32,100,0] {text(&mut o,&w.add(id,Value::Byte(0)).unwrap_err());}
  }
  let bytes=write_list(None).unwrap();i(&mut o,bytes.len() as i32);o.extend(bytes);o.push(read_list(&mut [127].as_slice()).unwrap().is_none() as u8);
  let list=read_list(&mut [224,127].as_slice()).unwrap().unwrap();i(&mut o,list.len() as i32);o.push(list[0].is_none() as u8);
 }
 o
}
