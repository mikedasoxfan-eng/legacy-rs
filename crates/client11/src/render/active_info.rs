//! Release ActiveRenderInfo: camera-relative near-plane origin and liquid surface lookup.
use worldgen::{entity::{Entity, Vec3D}, World, math};
use super::camera::Medium;

#[derive(Clone, Copy, Debug, Default)]
pub struct Info {
    pub offset: [f32; 3],
    pub rotation: [f32; 5],
}
impl Info {
    /// LWJGL GLU's float Gaussian elimination, including its original pivot comparison.
    /// `viewport` is the OpenGL viewport; the center uses integer division in vanilla.
    pub fn update(&mut self, model: [f32;16], projection: [f32;16], viewport: [i32;4], yaw:f32, pitch:f32, front:bool) {
        let mut combined = [0.0;16];
        for i in 0..4 {for j in 0..4 {
            combined[i*4+j] = model[i*4]*projection[j] + model[i*4+1]*projection[4+j]
                + model[i*4+2]*projection[8+j] + model[i*4+3]*projection[12+j];
        }}
        let mut inverse = [0.0;16];
        inverse[0]=1.0;inverse[5]=1.0;inverse[10]=1.0;inverse[15]=1.0;
        let mut valid = true;
        for i in 0..4 {
            let mut swap=i;
            for j in i+1..4 {if combined[j*4+i].abs() > combined[i*4+i].abs() {swap=j;}}
            if swap!=i {for k in 0..4 {combined.swap(i*4+k,swap*4+k);inverse.swap(i*4+k,swap*4+k);}}
            if combined[i*4+i]==0.0 {valid=false;break;}
            let t=combined[i*4+i];
            for k in 0..4 {combined[i*4+k]/=t;inverse[i*4+k]/=t;}
            for j in 0..4 {if j!=i {
                let t=combined[j*4+i];
                for k in 0..4 {combined[j*4+k]-=combined[i*4+k]*t;inverse[j*4+k]-=inverse[i*4+k]*t;}
            }}
        }
        if valid {
            let x=((viewport[0]+viewport[2])/2) as f32;
            let y=((viewport[1]+viewport[3])/2) as f32;
            let input=[(x-viewport[0] as f32)/viewport[2] as f32*2.0-1.0,
                (y-viewport[1] as f32)/viewport[3] as f32*2.0-1.0,-1.0,1.0];
            let output: [f32;4]=std::array::from_fn(|i| input[0]*inverse[i]+input[1]*inverse[4+i]+input[2]*inverse[8+i]+input[3]*inverse[12+i]);
            if output[3]!=0.0 {let inv=1.0/output[3];self.offset=std::array::from_fn(|i|output[i]*inv);}
        }
        self.set_rotation(yaw,pitch,front);
    }
    pub fn set_rotation(&mut self, yaw: f32, pitch: f32, front: bool) {
        let sign = if front {-1.0} else {1.0};
        let x = math::cos(yaw * std::f32::consts::PI / 180.0) * sign;
        let z = math::sin(yaw * std::f32::consts::PI / 180.0) * sign;
        let yz = -z * math::sin(pitch * std::f32::consts::PI / 180.0) * sign;
        let xy = x * math::sin(pitch * std::f32::consts::PI / 180.0) * sign;
        self.rotation = [x, math::cos(pitch * std::f32::consts::PI / 180.0), z, yz, xy];
    }
    pub fn position(&self, e: &Entity, partial: f64) -> Vec3D {
        Vec3D::new(
            e.prev_pos.x + (e.pos.x - e.prev_pos.x) * partial + self.offset[0] as f64,
            e.prev_pos.y + (e.pos.y - e.prev_pos.y) * partial + e.eye_height() as f64 + self.offset[1] as f64,
            e.prev_pos.z + (e.pos.z - e.prev_pos.z) * partial + self.offset[2] as f64,
        )
    }
    pub fn block(&self, e: &Entity, partial: f64, w: &mut World) -> u8 {
        let p = self.position(e, partial);
        let (x,y,z) = (math::floor_double(p.x),math::floor_double(p.y),math::floor_double(p.z));
        let mut id = w.get_block_id(x,y,z);
        if matches!(id,8..=11) {
            let meta = w.get_block_metadata(x,y,z) as i32;
            let height = (if meta >= 8 {0} else {meta} + 1) as f32 / 9.0 - 1.0 / 9.0;
            if p.y >= ((y + 1) as f32 - height) as f64 {id = w.get_block_id(x,y+1,z);}
        }
        id
    }
    pub fn medium(&self, e: &Entity, partial: f64, w: &mut World) -> Medium {
        match self.block(e,partial,w) {8|9 => Medium::Water,10|11 => Medium::Lava,_=>Medium::Air}
    }
}
