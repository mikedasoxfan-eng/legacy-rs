//! EntitySheep's release1.1 grazing state machine; superclass work stays in living.rs.
use super::{Entity,EntityKind};
use crate::{World, math, block::{GRASS,DIRT}};

impl Entity {
    pub(super) fn sheep_after_action11(&mut self, world: &mut World) {
        if !matches!(self.kind,EntityKind::Sheep {..}) {return;}
        let timer = self.living.sheep_graze_timer;
        if self.living.path.is_none() && timer <= 0 &&
            ((self.living.growing_age < 0 && self.rand.next_int(50)==0) || self.rand.next_int(1000)==0) {
            let x=math::floor_double(self.pos.x);let y=math::floor_double(self.pos.y);let z=math::floor_double(self.pos.z);
            if (world.get_block_id(x,y,z)==31 && world.get_block_metadata(x,y,z)==1) || world.get_block_id(x,y-1,z)==GRASS {
                self.living.sheep_graze_timer=40;
                // World.setEntityState is a no-op in the original singleplayer World.
            }
        } else if timer==4 {
            let x=math::floor_double(self.pos.x);let y=math::floor_double(self.pos.y);let z=math::floor_double(self.pos.z);
            let eaten = if world.get_block_id(x,y,z)==31 {
                world.auxiliary_effect(2001,(x,y,z),31+256);
                world.set_block_with_notify(x,y,z,0);
                true
            } else if world.get_block_id(x,y-1,z)==GRASS {
                world.auxiliary_effect(2001,(x,y-1,z),GRASS as i32);
                world.set_block_with_notify(x,y-1,z,DIRT);
                true
            } else {false};
            if eaten {
                if let EntityKind::Sheep {sheared,..}=&mut self.kind {*sheared=false;}
                self.sync_mob_watcher();
                if self.living.growing_age < 0 {self.set_growing_age(self.living.growing_age.wrapping_add(1200).min(0));}
            }
        }
    }
    pub(super) fn sheep_after_living11(&mut self) {
        if matches!(self.kind,EntityKind::Sheep {..}) && self.living.sheep_graze_timer > 0 {self.living.sheep_graze_timer-=1;}
    }
    pub fn sheep_head_position11(&self, partial:f32)->f32 {
        let timer=self.living.sheep_graze_timer;
        if timer<=0 {0.0} else if (4..=36).contains(&timer) {1.0} else if timer<4 {(timer as f32-partial)/4.0}
        else {-((timer-40) as f32-partial)/4.0}
    }
    pub fn sheep_head_angle11(&self, partial:f32)->f32 {
        let timer=self.living.sheep_graze_timer;
        if timer>4 && timer<=36 {
            let progress=((timer-4) as f32-partial)/32.0;
            math::PI_F32*0.2+math::PI_F32*0.07*math::sin(progress*28.7)
        } else if timer>0 {math::PI_F32*0.2} else {self.pitch/(180.0/math::PI_F32)}
    }
}
#[cfg(test)]
mod tests {
 use super::*;
 #[test]
 fn sheep_tick_growth_order_matches_original_11_bytecode(){let mut w=crate::entity::verify181::world();
  for line in include_str!("../fixtures/sheep_ticks11.txt").lines(){let v:Vec<i64>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();w.set_block_and_metadata(0,64,0,if v[2]>=2{31}else{0},if v[2]==3{2}else{1});w.set_block_with_notify(0,63,0,if v[2]==1{GRASS}else{DIRT});let mut e=Entity::living(&mut w,"Sheep").unwrap();e.set_position(0.0,64.0,0.0);e.living.entity_age=100;e.set_growing_age(v[0] as i32);e.kind=EntityKind::Sheep{sheared:true,fleece_color:0};e.living.sheep_graze_timer=v[1] as i32;e.rand.set_seed(42);e.living_update(&mut w);assert_eq!((e.living.sheep_graze_timer,e.living.growing_age,matches!(e.kind,EntityKind::Sheep{sheared:true,..}) as i64,w.get_block_id(0,64,0),w.get_block_id(0,63,0)),(v[3] as i32,v[4] as i32,v[5],v[6] as u8,v[7] as u8),"{line}");assert_eq!(e.rand.next_long(),v[8],"{line}");}
 } #[test]
 fn sheep_actions_match_original_11_bytecode(){
  let mut w=crate::entity::verify181::world();
  for line in include_str!("../fixtures/sheep_actions11.txt").lines(){
   let v:Vec<i64>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();
   w.set_block_and_metadata(0,64,0,if v[3]>=2 {31}else{0},if v[3]==3 {2}else{1});
   w.set_block_with_notify(0,63,0,if v[3]==1 {GRASS}else{DIRT});w.events.clear();
   let mut e=Entity::living(&mut w,"Sheep").unwrap();e.set_position(0.0,64.0,0.0);e.living.entity_age=100;e.set_growing_age(v[1] as i32);
   e.kind=EntityKind::Sheep{sheared:true,fleece_color:0};e.living.sheep_graze_timer=v[2] as i32;e.rand.set_seed(v[0]);
   e.creature_action_state(&mut w);e.sheep_after_action11(&mut w);
   assert_eq!((e.living.sheep_graze_timer,e.living.growing_age,matches!(e.kind,EntityKind::Sheep{sheared:true,..}) as i64,w.get_block_id(0,64,0),w.get_block_id(0,63,0)),(v[4] as i32,v[5] as i32,v[6],v[7] as u8,v[8] as u8),"{line}");
   let sfx=w.events.iter().filter_map(|e|if let crate::world::events::WorldEvent::Auxiliary{data,..}=e {Some(*data)}else{None}).last().unwrap_or(0);
   assert_eq!(sfx,v[9] as i32,"{line}");assert_eq!(e.rand.next_long(),v[10],"RNG {line}");
  }
 }
 #[test]
 fn sheep_head_boundaries_match_original_11_bytecode(){let mut w=crate::entity::verify181::world();let mut e=Entity::living(&mut w,"Sheep").unwrap();e.pitch=37.5;
  for line in include_str!("../fixtures/sheep_heads11.txt").lines(){let v:Vec<i64>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();e.living.sheep_graze_timer=v[0] as i32;let p=f32::from_bits(v[1] as u32);assert_eq!((e.sheep_head_position11(p).to_bits(),e.sheep_head_angle11(p).to_bits()),(v[2] as u32,v[3] as u32),"{line}");}
 }
 #[test]
 fn sheep_status_jump_and_movement_match_original_11_bytecode(){
  let mut w=crate::entity::verify181::world();
  for line in include_str!("../fixtures/sheep_status11.txt").lines(){
   let v:Vec<i64>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();
   let mut e=Entity::living(&mut w,"Sheep").unwrap();e.living.sheep_graze_timer=v[0] as i32;
   e.handle_health_update(&mut w,v[1] as i8);e.motion=super::super::Vec3D::new(0.125,0.25,-0.375);e.yaw=37.5;e.sprinting=v[2]!=0;e.rand.set_seed(42);
   assert_eq!(e.living.sheep_graze_timer,v[3] as i32,"{line}");assert_eq!(e.movement_ceased11() as i64,v[4],"{line}");
   e.jump11();assert_eq!([e.motion.x.to_bits() as i64,e.motion.y.to_bits() as i64,e.motion.z.to_bits() as i64],v[5..8],"{line}");
   assert_eq!(e.velocity_changed as i64,v[8],"{line}");assert_eq!(e.rand.next_long(),v[9],"{line}");
  }
 }
 #[test]
 fn magma_drops_match_original_11_bytecode(){let mut w=crate::entity::verify181::world();
  for line in include_str!("../fixtures/magma11.txt").lines(){let v:Vec<i64>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();w.entities.clear();w.loaded_entity_list.clear();let mut e=Entity::living(&mut w,"LavaSlime").unwrap();if let EntityKind::MagmaCube{size,..}=&mut e.kind {*size=v[1] as i32;}e.rand.set_seed(v[0]);e.drop_few_items_with_looting(&mut w,false,v[2] as i32);let items:Vec<_>=w.entities.values().filter_map(|e|e.dropped_item_stack()).collect();assert_eq!(items.len(),v[3] as usize,"{line}");assert!(items.iter().all(|s|s.id==378 && s.count==1));assert_eq!(e.rand.next_long(),v[4],"{line}");}
 }
 #[test]
 fn shearing_matches_original_11_bytecode(){let mut w=crate::entity::verify181::world();
  for line in include_str!("../fixtures/shearing11.txt").lines(){let v:Vec<i64>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();w.entities.clear();w.loaded_entity_list.clear();w.multiplayer_world=v[2]!=0;let mut sheep=Entity::living(&mut w,"Sheep").unwrap();sheep.kind=EntityKind::Sheep{sheared:v[3]!=0,fleece_color:7};sheep.set_growing_age(if v[1]!=0 {-1}else{0});sheep.rand.set_seed(v[0]);let id=sheep.entity_id;w.entity_joined_world(sheep);let mut p=Entity::player(&mut w,"fixture");p.player_state_mut().unwrap().inventory.main[0]=Some(crate::entity::player::inventory::ItemStack::new(359,1,0));let result=p.use_on_entity(&mut w,id);let sheep=&mut w.entities.get_mut(&id).unwrap();assert_eq!(matches!(sheep.kind,EntityKind::Sheep{sheared:true,..}) as i64,v[6],"{line}");assert_eq!(sheep.rand.next_long(),v[8],"{line}");assert_eq!(result as i64,v[7],"{line}");assert_eq!(p.held_item().unwrap().damage,v[5] as i32,"{line}");let items:Vec<_>=w.entities.values().filter_map(|e|e.dropped_item_stack()).collect();assert_eq!(items.len(),v[4] as usize,"{line}");assert!(items.iter().all(|s|s.id==35 && s.damage==7));}
 }
}
