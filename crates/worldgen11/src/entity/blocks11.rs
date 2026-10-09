//! Focused release1.1 block behavior oracle vectors.
#[cfg(test)]
mod tests {
 use crate::{entity::{Entity,AxisAlignedBB},world::{chunk::Chunk,events::WorldEvent}};
 fn values(line:&str)->Vec<i64>{line.split_whitespace().map(|s|s.parse().unwrap()).collect()}
 fn bits(b:AxisAlignedBB)->Vec<i64>{[b.min_x,b.min_y,b.min_z,b.max_x,b.max_y,b.max_z].iter().map(|d|d.to_bits() as i64).collect()}
 #[test]
 fn farmland_walking_matches_original_11_bytecode(){let mut w=crate::entity::verify181::world();for line in include_str!("../fixtures/farmwalking11.txt").lines(){let v=values(line);w.set_block_and_metadata(0,63,0,60,7);let mut e=Entity::living(&mut w,"Chicken").unwrap();w.rand.set_seed(v[0]);e.on_entity_walking(&mut w,60,0,63,0);assert_eq!(w.get_block_id(0,63,0),v[1] as u8);assert_eq!(w.rand.next_long(),v[2]);}} #[test]
 fn block_boxes_match_original_11_bytecode(){let mut w=crate::entity::verify181::world();
  for line in include_str!("../fixtures/blockboxes11.txt").lines(){let v=values(line);let id=v[0] as u8;let x=v[2] as i32;let c=(x>>4,x>>4);if !w.chunks.contains_key(&c){w.chunks.insert(c,Box::new(Chunk::new(c.0,c.1,vec![0;32768].into_boxed_slice().try_into().unwrap())));}w.set_block_and_metadata(x,64,x,id,v[1] as i32);let b=w.block_collision_box(id,x,64,x);let mut at=4;assert_eq!(b.is_some(),v[3]!=0,"{line}");if let Some(b)=b{assert_eq!(bits(b),v[at..at+6],"collision {line}");at+=6;}assert_eq!(v[at],1);let b=w.block_selected_box11(id,x,64,x);assert_eq!(bits(b),v[at+1..at+7],"selection {line}");}
 }
 #[test]
 fn farmland_landings_match_original_11_bytecode(){let mut w=crate::entity::verify181::world();
  for line in include_str!("../fixtures/farmland11.txt").lines(){let v=values(line);w.set_block_and_metadata(0,63,0,60,7);let mut e=if v[6]!=0{Entity::living(&mut w,"Chicken").unwrap()}else{Entity::item(&mut w,0.0,64.0,0.0,crate::world::tile::ItemStack::new(1,1,0))};e.y_offset=0.0;e.set_position(0.0,64.0,0.0);e.fall_distance=f32::from_bits(v[1] as u32);w.rand.set_seed(v[0]);e.update_fall_state11(&mut w,-0.25,v[2]!=0);assert_eq!(w.get_block_id(0,63,0),v[3] as u8,"{line}");assert_eq!(e.fall_distance.to_bits(),v[4] as u32,"{line}");assert_eq!(w.rand.next_long(),v[5],"{line}");}
 }
 #[test]
 fn gate_notifications_match_original_11_bytecode(){for line in include_str!("../fixtures/gate11.txt").lines(){let v=values(line);let mut w=crate::entity::verify181::world();if v[2]!=0{w.set_block_and_metadata(1,64,0,76,5);}w.set_block_and_metadata(0,64,0,107,v[0] as i32);w.multiplayer_world=v[3]!=0;w.rand.set_seed(42);w.events.clear();crate::world::behavior::on_neighbor_block_change(&mut w,107,0,64,0,v[1] as u8);assert_eq!(w.get_block_metadata(0,64,0),v[4] as i32,"{line}");assert_eq!(w.events.iter().filter(|e|matches!(e,WorldEvent::Auxiliary{code:1003,..})).count(),v[5] as usize,"{line}");assert_eq!(w.rand.next_long(),v[6],"{line}");}}
 #[test]
 fn powered_rail_additions_match_original_11_bytecode(){for line in include_str!("../fixtures/railadded11.txt").lines(){let v=values(line);let mut w=crate::entity::verify181::world();if v[1]!=0{w.set_block_and_metadata(1,64,0,76,5);}w.editing_blocks=true;w.set_block_and_metadata(0,64,0,27,v[0] as i32);w.editing_blocks=false;w.set_block_metadata(0,64,0,v[0] as i32);w.multiplayer_world=v[2]!=0;w.rand.set_seed(42);w.rail_added(0,64,0);assert_eq!((w.get_block_id(0,64,0),w.get_block_metadata(0,64,0)),(v[3] as u8,v[4] as i32),"{line}");assert_eq!(w.rand.next_long(),v[5],"{line}");}}
 #[test]
 fn web_distance_matches_original_11_bytecode(){let mut w=crate::entity::verify181::world();for line in include_str!("../fixtures/web11.txt").lines(){let v=values(line);let mut e=Entity::living(&mut w,"Chicken").unwrap();e.fall_distance=f32::from_bits(v[0] as u32);e.on_entity_collided_with_block(&mut w,30,0,64,0);assert!(e.in_web);assert_eq!(e.fall_distance.to_bits(),v[1] as u32);}}
}
