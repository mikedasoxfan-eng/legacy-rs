//! Repeated damage with attribution and immunity-window branches.
use super::{Entity,damage::{DamageKind,DamageSource}};
pub fn replay()->Vec<u8> {
 let mut o=Vec::new();
 for name in ["Pig","Cow","Chicken","Sheep","Wolf","Zombie","Skeleton","Spider","CaveSpider","Creeper","PigZombie","Ghast","Squid","Slime","Enderman","Silverfish"] {
  for source in 0..6 {for hearts in [0,11,20] {for amount in [0,1,4,25] {
   let mut w=super::verify181::world();let mut e=Entity::living(&mut w,name).unwrap();e.set_location_and_angles(-3.5,64.0,4.5,35.0,0.0);e.living.hearts_life=hearts;e.living.health=20;
   let mut attacker=if source==2 {Entity::living(&mut w,"Zombie").unwrap()} else {Entity::player(&mut w,"Player")};attacker.set_position(0.5,65.62,0.5);let id=attacker.entity_id;w.entity_joined_world(attacker);
   let arrow=Entity::arrow(&mut w,0.0,0.0,0.0);
   let damage=match source {
    0=>DamageSource::new(DamageKind::Generic),1=>DamageSource::new(DamageKind::OnFire),2=>DamageSource::entity(DamageKind::Mob,Some(id)),3=>DamageSource::entity(DamageKind::Player,Some(id)),_=>DamageSource::indirect(DamageKind::Arrow,arrow.entity_id,if source==4 {Some(id)} else {None}),
   };
   for j in 0..3 {o.push(e.attack_from(&mut w,damage,amount+j) as u8);super::verify181::state(&mut w,&e,&mut o);}
  }}}
 }
 o
}

pub fn explosion_replay()->Vec<u8> {
 let mut o=Vec::new();
 for size in [0.25,1.0,2.0,4.0] {for c in 0..4 {
  let mut w=super::verify181::world();let mut ids=Vec::new();for (j,name) in ["Pig","Wolf","Slime"].into_iter().enumerate() {let mut e=Entity::living(&mut w,name).unwrap();let j=j as f64;e.set_position(-3.5+j*0.9,85.25+j*0.125,4.5-j*0.75);ids.push(e.entity_id);w.entity_joined_world(e);}
  let ex=w.new_explosion(None,super::Vec3D::new(-3.5+c as f64*0.137,85.25+c as f64*0.25,4.5-c as f64*0.073),size,c%2!=0);
  o.extend((ex.destroyed_blocks.len() as i32).to_le_bytes());for (x,y,z) in ex.destroyed_blocks {for v in [x,y,z] {o.extend(v.to_le_bytes());}}
  for id in ids {let e=w.entities[&id].clone();super::verify181::state(&mut w,&e,&mut o);}
 }}o
}
