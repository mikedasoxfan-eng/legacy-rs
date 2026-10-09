//! Release1.1 ItemMonsterPlacer and EntityList egg metadata.
use super::{Entity,registry};
use super::player::ItemStack;
use crate::World;
pub const ITEM_ID:i32=383;
pub const RENDER_PASSES:u8=2;
pub fn has_multiple_passes()->bool{true}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct EggInfo {pub id:i32,pub base_color:u32,pub spots_color:u32}
pub const EGGS:&[EggInfo]=&[
 EggInfo{id:50,base_color:894731,spots_color:0},EggInfo{id:51,base_color:12698049,spots_color:4802889},EggInfo{id:52,base_color:3419431,spots_color:11013646},
 EggInfo{id:54,base_color:44975,spots_color:7969893},EggInfo{id:55,base_color:5349438,spots_color:8306542},EggInfo{id:56,base_color:16382457,spots_color:12369084},EggInfo{id:57,base_color:15373203,spots_color:5009705},EggInfo{id:58,base_color:1447446,spots_color:0},EggInfo{id:59,base_color:803406,spots_color:11013646},EggInfo{id:60,base_color:7237230,spots_color:3158064},EggInfo{id:61,base_color:16167425,spots_color:16775294},EggInfo{id:62,base_color:3407872,spots_color:16579584},
 EggInfo{id:90,base_color:15771042,spots_color:14377823},EggInfo{id:91,base_color:15198183,spots_color:16758197},EggInfo{id:92,base_color:4470310,spots_color:10592673},EggInfo{id:93,base_color:10592673,spots_color:16711680},EggInfo{id:94,base_color:2243405,spots_color:7375001},EggInfo{id:95,base_color:14144467,spots_color:13545366},EggInfo{id:96,base_color:10489616,spots_color:12040119},EggInfo{id:120,base_color:5651507,spots_color:12422002},
];
/// Actual Java8u51 EntityList egg HashMap key iteration order.
pub const CREATIVE_EGG_IDS:&[i32]=&[96,50,51,52,54,55,56,120,57,58,90,59,91,60,92,61,93,62,94,95];
pub fn info(id:i32)->Option<&'static EggInfo>{EGGS.iter().find(|e|e.id==id)}
pub fn color(id:i32,pass:i32)->u32{info(id).map_or(0xffffff,|e|if pass==0{e.base_color}else{e.spots_color})}
pub fn icon(_metadata:i32,pass:i32)->i32{if pass>0{169}else{153}}
pub fn entity_name(id:i32)->Option<&'static str>{registry::by_id(id).map(|e|e.name)}
pub fn display_name(id:i32,mut translate:impl FnMut(&str)->String)->String{
 let mut name=translate("item.monsterPlacer.name").trim_matches(|c:char|c<='\u{20}').to_owned();
 if let Some(entity)=entity_name(id){name.push(' ');name.push_str(&translate(&format!("entity.{entity}.name")));}name
}
pub fn item()->super::item::Item{let mut item=super::item::Item::defaults(383);item.set_max_stack_size(1).set_has_subtypes(true).set_icon_coord(9,9).set_item_name(Some("monsterPlacer"));item}
/// ItemMonsterPlacer's constructor/consumption/position phase; World spawn follows.
/// This intentionally accepts every concrete EntityList id, including non-egg mobs
/// and nonliving entities. No spawn-initialization or random-yaw callback exists in1.1.
pub fn prepare_spawn(world:&mut World,stack:&mut ItemStack,creative:bool,p:[i32;3],side:i32)->Option<Entity>{
 if world.multiplayer_world{return None;}
 let offset=[(0,-1,0),(0,1,0),(0,0,-1),(0,0,1),(-1,0,0),(1,0,0)][side as usize];
 let x=p[0].wrapping_add(offset.0);let y=p[1].wrapping_add(offset.1);let z=p[2].wrapping_add(offset.2);
 let mut entity=registry::create_by_id(world,stack.damage)?;
 if !creative{stack.count=stack.count.wrapping_sub(1);}
 entity.set_location_and_angles(x as f64+0.5,y as f64,z as f64+0.5,0.0,0.0);
 Some(entity)
}
impl Entity{
 pub fn spawn_egg_on_block11(&mut self,world:&mut World,stack:&mut ItemStack,p:[i32;3],side:i32)->bool{
  let creative=self.player_state().expect("EntityPlayer").capabilities.creative;
  if let Some(entity)=prepare_spawn(world,stack,creative,p,side){world.entity_joined_world(entity);}
  true
 }
}
#[cfg(test)]
mod tests {
 use super::*;
 use crate::{world::chunk::Chunk,entity::player::{registry as items,stat}};
 fn values(line:&str)->Vec<i64>{line.split_whitespace().map(|s|s.parse().unwrap()).collect()}
 fn state(w:&mut World,e:&Entity,o:&mut Vec<u8>){
  crate::entity::verify181::state(w,e,o);
  match &e.kind{
   super::super::EntityKind::Slime{size,jump_delay,squish,prev_squish}|super::super::EntityKind::MagmaCube{size,jump_delay,squish,prev_squish}=>{o.extend(size.to_le_bytes());o.extend(jump_delay.to_le_bytes());for f in [*squish,e.living.slime_squish,*prev_squish]{o.extend(f.to_bits().to_le_bytes());}},
   super::super::EntityKind::Chicken{flap,prev_flap,flap_amount,prev_flap_amount,flap_speed,egg_timer}=>{o.extend(egg_timer.to_le_bytes());for f in [*flap,*prev_flap,*flap_amount,*prev_flap_amount,*flap_speed]{o.extend(f.to_bits().to_le_bytes());}},
   super::super::EntityKind::Squid(s)=>{for f in [s.pitch,s.prev_pitch,s.yaw,s.prev_yaw,s.rotation,s.prev_rotation,s.tentacle_angle,s.prev_tentacle_angle,s.speed,s.rotation_speed,s.yaw_speed,s.motion_x,s.motion_y,s.motion_z]{o.extend(f.to_bits().to_le_bytes());}},
   _=>{}
  }
 }
 #[test]
 fn egg_invalid_side_matches_original_11_bytecode(){
  let mut w=crate::entity::verify181::world();let mut player=Entity::player(&mut w,"fixture");
  for line in include_str!("../fixtures/egginvalidside11.txt").lines(){let columns:Vec<_>=line.split_whitespace().collect();let number=|i:usize|columns[i].parse::<i64>().unwrap();w.multiplayer_world=number(2)!=0;w.rand.set_seed(42);let before=w.loaded_entity_list.len();let mut stack=ItemStack::new(383,3,number(0) as i32);let result=stack.use_item181(&mut player,&mut w,[0,64,0],number(1) as i32);match result{Ok(value)=>{assert_eq!(columns[3],"-");assert_eq!(value as i64,number(4));},Err(error)=>assert_eq!(error.class(),columns[3])};assert_eq!(stack.count as i64,number(5));assert_eq!(w.loaded_entity_list.len(),before);assert_eq!(w.rand.next_long(),number(7));}
 }
 #[test]
 fn egg_constructor_save_tick_matches_original_11_bytecode(){
  let bytes=include_bytes!("../fixtures/eggstate11.bin");let mut at=0usize;
  fn record<'a>(bytes:&'a [u8],at:&mut usize)->&'a [u8]{let n=i32::from_le_bytes(bytes[*at..*at+4].try_into().unwrap()) as usize;*at+=4;let result=&bytes[*at..*at+n];*at+=n;result}
  for joined in [false,true]{for seed in [0,1,42,100,-1,341873128712i64]{for &id in CREATIVE_EGG_IDS{for ticks in [0,1,5,20]{
   let mut w=crate::entity::verify181::world();w.set_entity_determinism(seed,200,0);let mut stack=ItemStack::new(383,3,id);let mut e=prepare_spawn(&mut w,&mut stack,false,[0,64,0],1).unwrap();
   if joined {assert!(w.entity_joined_world(e.clone()));e=w.entities[&e.entity_id].clone();}for _ in 0..ticks{if joined{w.update_entity_with_optional_force(&mut e,true);w.entities.insert(e.entity_id,e.clone());}else{e.on_update(&mut w);}}
   let expected=record(bytes,&mut at);let mut actual=Vec::new();state(&mut w,&e,&mut actual);
   let diff=expected.iter().zip(&actual).position(|(a,b)|a!=b);assert_eq!(actual,expected,"spawn id {id}, seed {seed}, ticks {ticks}, first offset {diff:?}");
   let saved_bytes=record(bytes,&mut at);if saved_bytes.is_empty(){assert!(e.to_nbt().is_none(),"refused save id {id}, seed {seed}, ticks {ticks}");assert!(record(bytes,&mut at).is_empty());continue;}let saved=crate::nbt::read_root(&mut &saved_bytes[..]).unwrap();assert_eq!(e.to_nbt().unwrap(),saved,"saved id {id}, seed {seed}, ticks {ticks}");
   let mut loaded=registry::create_from_nbt_checked(&mut w,Some(&saved)).unwrap().unwrap();let expected=record(bytes,&mut at);let mut actual=Vec::new();state(&mut w,&loaded,&mut actual);let diff=expected.iter().zip(&actual).position(|(a,b)|a!=b);assert_eq!(actual,expected,"reloaded id {id}, seed {seed}, ticks {ticks}, first offset {diff:?}");
   for _ in 0..5{loaded.on_update(&mut w);}let expected=record(bytes,&mut at);let mut actual=Vec::new();state(&mut w,&loaded,&mut actual);let diff=expected.iter().zip(&actual).position(|(a,b)|a!=b);assert_eq!(actual,expected,"reloaded ticks id {id}, seed {seed}, ticks {ticks}, joined {joined}, first offset {diff:?}");
  }}}}
  assert_eq!(at,bytes.len());
 }
 #[test]
 fn egg_metadata_matches_original_11_bytecode(){let item=item();
  for line in include_str!("../fixtures/eggs11.txt").lines(){let columns:Vec<_>=line.split_whitespace().collect();let v:Vec<i32>=columns[..8].iter().map(|s|s.parse().unwrap()).collect();let def=items::item(383).unwrap();assert_eq!((def.max_stack,def.durability,def.subtypes as i32),(v[2],v[3],v[4]));assert_eq!((item.max_stack_size(),item.max_damage(),item.has_subtypes() as i32),(v[2],v[3],v[4]));assert_eq!((icon(v[0],v[1]),color(v[0],v[1]) as i32),(v[6],v[7]),"{line}");assert_eq!(entity_name(v[0]).unwrap_or("-"),columns[8],"{line}");assert_eq!(has_multiple_passes() as i32,v[5]);}
 }
 #[test]
 fn egg_names_match_original_11_bytecode(){
  for line in include_str!("../fixtures/eggnames11.txt").lines(){let(id,expected)=line.split_once('|').unwrap();let id:i32=id.parse().unwrap();let actual=display_name(id,|key|{if key=="item.monsterPlacer.name"{"\tSpawn\n".to_owned()}else{assert_eq!(key,format!("entity.{}.name",entity_name(id).unwrap()));expected.strip_prefix("Spawn ").unwrap().to_owned()}});assert_eq!(actual,expected);}
 }
 #[test]
 fn creative_inventory_matches_original_11_bytecode(){let expected:Vec<_>=include_str!("../fixtures/creative11.txt").lines().map(values).collect();let actual=crate::block::visual181::creative_stacks();assert_eq!(actual.len(),expected.len());for(s,v)in actual.iter().zip(expected){assert_eq!((s.id,s.count,s.damage),(v[0] as i32,v[1] as i32,v[2] as i32));}assert_eq!(actual.iter().filter(|s|s.id==383).map(|s|s.damage).collect::<Vec<_>>(),CREATIVE_EGG_IDS);}
 #[test]
 fn egg_spawns_match_original_11_bytecode(){
  let mut w=crate::World::new(0,300);w.set_entity_determinism(100,200,0);
  for line in include_str!("../fixtures/egguse11.txt").lines(){let v=values(line);w.entities.clear();w.loaded_entity_list.clear();w.chunks.clear();w.multiplayer_world=v[3]!=0;w.rand.set_seed(42);let mut stack=ItemStack::new(383,3,v[0] as i32);let spawned=prepare_spawn(&mut w,&mut stack,v[4]!=0,[v[2] as i32,64,v[2] as i32],v[1] as i32);assert_eq!(stack.count,v[7] as i32,"{line}");assert_eq!(spawned.is_some(),v[8]!=0,"{line}");if let Some(e)=spawned {assert_eq!(registry::by_kind(&e.kind).unwrap().id,v[9] as i32,"{line}");assert_eq!((e.pos.x.to_bits(),e.pos.y.to_bits(),e.pos.z.to_bits(),e.yaw.to_bits(),e.pitch.to_bits()),(v[10] as u64,v[11] as u64,v[12] as u64,v[13] as u32,v[14] as u32),"{line}");let x=crate::math::floor_double(e.pos.x/16.0);let z=crate::math::floor_double(e.pos.z/16.0);if v[5]!=0{w.chunks.insert((x,z),Box::new(Chunk::new(x,z,vec![0;32768].into_boxed_slice().try_into().unwrap())));}assert_eq!(w.entity_joined_world(e),v[5]!=0,"{line}");}assert_eq!(w.rand.next_long(),*v.last().unwrap(),"{line}");}
 }
 #[test]
 fn egg_stack_use_and_statistics_match_original_11_bytecode(){
  let expected=values(include_str!("../fixtures/eggstats11.txt").trim());assert_eq!([stat::used(383),stat::broken(383),stat::crafted(383)],[expected[0] as i32,expected[1] as i32,expected[2] as i32]);
  let mut w=crate::entity::verify181::world();let mut player=Entity::player(&mut w,"fixture");
  for line in include_str!("../fixtures/eggstackuse11.txt").lines(){let v=values(line);w.multiplayer_world=v[3]!=0;player.player_state_mut().unwrap().capabilities.creative=v[2]!=0;player.player_state_mut().unwrap().stat_log.clear();let mut stack=ItemStack::new(383,v[1] as i32,v[0] as i32);let result=stack.use_item181(&mut player,&mut w,[0,64,0],1).unwrap();assert_eq!((result as i64,stack.count as i64),(v[4],v[5]),"{line}");assert_eq!(player.player_state().unwrap().stat_log,[(v[6] as i32,v[7] as i32)],"{line}");}
 }
}
