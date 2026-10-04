use super::*;
use crate::entity::MovingObjectPosition;
impl Entity {
    pub fn view_position(&self,partial:f32)->Vec3D {
        if partial==1.0 {self.pos}else {self.prev_pos.add((self.pos.x-self.prev_pos.x)*partial as f64,(self.pos.y-self.prev_pos.y)*partial as f64,(self.pos.z-self.prev_pos.z)*partial as f64)}
    }
    pub fn look(&self,partial:f32)->Vec3D {
        let (yaw,pitch)=if partial==1.0 {(self.yaw,self.pitch)}else{(self.prev_yaw+(self.yaw-self.prev_yaw)*partial,self.prev_pitch+(self.pitch-self.prev_pitch)*partial)};
        let yaw=-yaw*(math::PI_F32/180.0)-math::PI_F32;let pitch=-pitch*(math::PI_F32/180.0);
        let c=math::cos(yaw);let s=math::sin(yaw);let p=-math::cos(pitch);Vec3D::new((s*p) as f64,math::sin(pitch) as f64,(c*p) as f64)
    }
    /// EntityRenderer.getMouseOver, with SP block reach 4 and entity reach 3.
    pub fn mouse_over(&self,w:&mut World,partial:f32)->Option<MovingObjectPosition> {
        let start=self.view_position(partial);let look=self.look(partial);
        let block=w.ray_trace_blocks(start,start.add(look.x*4.0,look.y*4.0,look.z*4.0));
        let reach=block.as_ref().map_or(4.0,|hit|match hit {MovingObjectPosition::Block{hit,..}=>hit.distance(start),_=>4.0}).min(3.0);
        let end=start.add(look.x*reach,look.y*reach,look.z*reach);
        let mut pointed=None;let mut distance=0.0;
        for id in w.entities_within_aabb(Some(self.entity_id),self.bounding_box.add_coord(look.x*reach,look.y*reach,look.z*reach).expand(1.0,1.0,1.0)) {
            let e=&w.entities[&id];if !e.can_be_collided_with() {continue;}
            let b=e.bounding_box.expand(0.1f32 as f64,0.1f32 as f64,0.1f32 as f64);
            if b.contains(start) {if 0.0<distance||distance==0.0 {pointed=Some(id);distance=0.0;}}
            else if let Some(MovingObjectPosition::Block{hit,..})=b.ray_trace(start,end) {let d=start.distance(hit);if d<distance||distance==0.0 {pointed=Some(id);distance=d;}}
        }
        pointed.map(|id|MovingObjectPosition::Entity{id,hit:w.entities[&id].pos}).or(block)
    }
}
