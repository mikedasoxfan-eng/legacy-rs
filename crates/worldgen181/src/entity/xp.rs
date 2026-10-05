//! EntityXPOrb: constructor, attraction, physics, lifetime and pickup.
use super::{Entity, EntityKind, Vec3D, block_table};
use crate::{World, math::floor_double as floor, block::*};
#[derive(Clone, Debug)]
pub struct XPOrbState { pub ticks: i32, pub age: i32, pub pickup_delay: i32, pub health: i32, pub value: i32, pub phase: f32 }
impl Default for XPOrbState { fn default() -> Self { Self { ticks:0, age:0, pickup_delay:0, health:5, value:0, phase:0.0 } } }
pub const XP_VALUES: [i32;11] = [1,3,7,17,37,73,149,307,617,1237,2477];
pub fn split_xp(value: i32) -> i32 { XP_VALUES.into_iter().rev().find(|&v| value >= v).unwrap_or(1) }
pub fn texture_index(value: i32) -> i32 { XP_VALUES.iter().rposition(|&v| value >= v).unwrap_or(0) as i32 }
impl Entity {
    pub fn xp_orb_base(w: &mut World) -> Self {
        let phase = (w.math_random.next_double() * std::f64::consts::PI * 2.0) as f32;
        let mut e = Self::base(w, EntityKind::XPOrb(XPOrbState { phase, ..Default::default() }));
        e.width=0.25; e.height=0.25; e.y_offset=e.height/2.0;
        e
    }
    pub fn xp_orb(w: &mut World, pos: Vec3D, value: i32) -> Self {
        let mut e=Self::xp_orb_base(w);
        e.width=0.5; e.height=0.5; e.y_offset=e.height/2.0;
        e.set_position(pos.x,pos.y,pos.z);
        e.yaw=(w.math_random.next_double()*360.0) as f32;
        e.motion.x=((w.math_random.next_double()*0.2f32 as f64-0.1f32 as f64) as f32*2.0) as f64;
        e.motion.y=((w.math_random.next_double()*0.2) as f32*2.0) as f64;
        e.motion.z=((w.math_random.next_double()*0.2f32 as f64-0.1f32 as f64) as f32*2.0) as f64;
        if let EntityKind::XPOrb(s)=&mut e.kind { s.value=value; }
        e
    }
    pub(super) fn update_xp_orb(&mut self,w:&mut World) {
        self.on_entity_update(w);
        if let EntityKind::XPOrb(s)=&mut self.kind { if s.pickup_delay>0 {s.pickup_delay-=1;} }
        self.prev_pos=self.pos; self.motion.y-=0.03f32 as f64;
        if w.get_block_material(floor(self.pos.x),floor(self.pos.y),floor(self.pos.z))==MAT_LAVA {
            self.motion.y=0.2f32 as f64;
            self.motion.x=((self.rand.next_float()-self.rand.next_float())*0.2) as f64;
            self.motion.z=((self.rand.next_float()-self.rand.next_float())*0.2) as f64;
            let pitch=2.0+self.rand.next_float()*0.4;
            w.play_sound_at_entity(self,"random.fizz",0.4,pitch);
        }
        self.push_out_of_blocks(w,self.pos.x,(self.bounding_box.min_y+self.bounding_box.max_y)/2.0,self.pos.z);
        if let Some(i)=w.closest_player(self.pos,8.0) {
            let (x,y,z)=w.players[i];
            let eye=w.player_entities.get(i).copied().flatten().and_then(|id|w.entities.get(&id)).map_or(0.12,|p|p.eye_height());
            let dx=(x-self.pos.x)/8.0;let dy=(y+eye as f64-self.pos.y)/8.0;let dz=(z-self.pos.z)/8.0;
            let length=(dx*dx+dy*dy+dz*dz).sqrt();let mut force=1.0-length;
            if force>0.0 { force*=force;self.motion.x+=dx/length*force*0.1;self.motion.y+=dy/length*force*0.1;self.motion.z+=dz/length*force*0.1; }
        }
        self.move_entity(w,self.motion.x,self.motion.y,self.motion.z);
        let mut friction=0.98f32;
        if self.on_ground {
            friction=0.1f32*0.1f32*58.8f32;
            let block=w.get_block_id(floor(self.pos.x),floor(self.bounding_box.min_y)-1,floor(self.pos.z));
            if block>0 { friction=block_table::SLIPPERINESS[block as usize]*0.98; }
        }
        self.motion.x*=friction as f64;self.motion.y*=0.98f32 as f64;self.motion.z*=friction as f64;
        if self.on_ground {self.motion.y*=-0.9f32 as f64;}
        if let EntityKind::XPOrb(s)=&mut self.kind {s.ticks+=1;s.age+=1;if s.age>=6000 {self.dead=true;}}
    }
    pub fn pickup_xp_orb(&mut self,w:&mut World,player:&mut Entity) {
        let EntityKind::XPOrb(s)=&self.kind else{return};
        let Some(p)=player.player_state_mut() else{return};
        if s.pickup_delay==0 && p.xp_pickup_cooldown==0 {
            p.xp_pickup_cooldown=2; let value=s.value;
            let pitch=0.5*((self.rand.next_float()-self.rand.next_float())*0.7+1.0);
            w.play_sound_at_entity(self,"random.pop",0.2,pitch);
            if let Some(p)=player.player_state_mut() {p.events.push(super::player::PlayerEvent::Pickup{entity:self.entity_id,count:1,source:Box::new(self.clone())});}
            player.add_experience(value);
            self.dead=true;
        }
    }
    pub fn add_experience(&mut self,value:i32) {
        if let Some(p)=self.player_state_mut() {
            p.experience=p.experience.wrapping_add(value);p.total_experience=p.total_experience.wrapping_add(value);
            while p.experience >= p.experience_level.wrapping_add(1).wrapping_mul(10) {
                p.experience-=p.experience_level.wrapping_add(1).wrapping_mul(10);p.experience_level+=1;
            }
        }
    }
}
