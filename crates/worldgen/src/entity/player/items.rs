use super::*;
use crate::{block::*,entity::MovingObjectPosition};
impl Entity {
    pub fn use_on_block(&mut self,w:&mut World,x:i32,y:i32,z:i32,side:i32)->bool {
        if self.activate_block(w,x,y,z) {return true;}
        let Some(mut stack)=self.held_item() else{return false;};
        let used=match stack.id {
            1..=96=>self.place_block_item(w,&mut stack,x,y,z,side),
            324|330=>self.place_door(w,&mut stack,x,y,z,side),
            355=>self.place_bed(w,&mut stack,x,y,z,side),
            351=>self.dye_block(w,&mut stack,x,y,z),
            259=>{let (x,y,z)=super::placement::offset(x,y,z,side);if w.get_block_id(x,y,z)==0 {w.item_random.next_float();w.set_block_with_notify(x,y,z,51);}stack.damage_item(1);true},
            290..=294=>{let id=w.get_block_id(x,y,z);if (side==0||w.get_block_id(x,y+1,z)!=0||id!=2)&&id!=3 {false}else{w.set_block_with_notify(x,y,z,60);stack.damage_item(1);true}},
            295=>{if side==1&&w.get_block_id(x,y,z)==60&&w.get_block_id(x,y+1,z)==0 {w.set_block_with_notify(x,y+1,z,59);stack.count-=1;true}else{false}},
            338|354|356=>{let block=match stack.id {338=>83,354=>92,_=>93};let (x,y,z)=if w.get_block_id(x,y,z)==78 {(x,y,z)}else{super::placement::offset(x,y,z,side)};if w.get_block_id(x,y,z)==0&&super::placement::can_place_at(w,block,x,y,z,side) {w.set_block_with_notify(x,y,z,block);self.block_placed(w,block,x,y,z,side);stack.count-=1;true}else{false}},
            _=>false,
        };
        let inv=&mut self.player_state_mut().unwrap().inventory;inv.main[inv.current_item as usize]=if stack.count<=0 {None}else{Some(stack)};used
    }
    pub fn use_item(&mut self,w:&mut World)->bool {
        let Some(mut stack)=self.held_item() else{return false;};let original=stack;
        let heal=match stack.id {260=>4,282=>10,297=>5,319=>3,320=>8,322=>42,349=>2,350=>5,357=>1,_=>0};
        if heal>0 {stack.count-=1;self.heal(heal);if stack.id==282 {stack=ItemStack::new(281,1,0);}}
        else if stack.id==261 {if self.player_state_mut().unwrap().inventory.consume(262) {w.item_random.next_float();let arrow=Entity::arrow_from_owner(w,self);w.entity_joined_world(arrow);}}
        else if matches!(stack.id,332|344) {stack.count-=1;w.item_random.next_float();let projectile=Entity::throwable(w,self,stack.id==344);w.entity_joined_world(projectile);}
        else if matches!(stack.id,325..=327|335) {
            let full=match stack.id {326=>8,327=>10,335=>-1,_=>0};
            let start=Vec3D::new(self.prev_pos.x+(self.pos.x-self.prev_pos.x),self.prev_pos.y+(self.pos.y-self.prev_pos.y)+1.62-self.y_offset as f64,self.prev_pos.z+(self.pos.z-self.prev_pos.z));let look=self.look(1.0);
            if let Some(MovingObjectPosition::Block{x,y,z,side,..})=w.ray_trace_blocks_options(start,start.add(look.x*5.0,look.y*5.0,look.z*5.0),full==0,false) {
                if full==0 {let mat=w.get_block_material(x,y,z);if matches!(mat,MAT_WATER|MAT_LAVA)&&w.get_block_metadata(x,y,z)==0 {w.set_block_with_notify(x,y,z,0);stack=ItemStack::new(if mat==MAT_WATER{326}else{327},1,0);}}
                else if full<0 {stack=ItemStack::new(325,1,0);}
                else {let (x,y,z)=super::placement::offset(x,y,z,side);if w.get_block_id(x,y,z)==0||!mat(w.get_block_material(x,y,z)).is_solid {if w.dimension.is_hell_world()&&full==8 {w.rand.next_float();w.rand.next_float();for _ in 0..24 {w.math_random.next_double();}}else{w.set_block_and_metadata_with_notify(x,y,z,full as u8,0);}stack=ItemStack::new(325,1,0);}}
            }
        }
        let changed=stack.id!=original.id||stack.count!=original.count;
        let inv=&mut self.player_state_mut().unwrap().inventory;inv.main[inv.current_item as usize]=if stack.count==0 {None}else{Some(stack)};changed
    }
}
