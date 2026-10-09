//! RenderEndPortal's sixteen seeded layers and projective texgen inputs.
use super::entity::fixed::{Fixed, Op, matrix};
use glam::{Mat4, Vec4};
use worldgen::JavaRandom;

#[derive(Clone, Debug)]
pub struct Command {pub code:i32, pub args:[f32;4]}
#[derive(Clone, Debug)]
pub struct Layer {
    pub texture_ops: Vec<Op>,
    /// Model-view at the moment the Q eye plane is specified, before restoring the stack.
    pub plane_translation: [f32;3],
}
pub struct Portal {pub gl:Fixed, pub layers:Vec<Layer>, pub commands:Vec<Command>}
impl Portal {
    /// Homogeneous (S,T,Q); division belongs in the fragment shader after interpolation.
    pub fn projective_uv(&self, layer:usize, view:Mat4) -> Vec<[f32;3]> {
        let l=&self.layers[layer];
        let specified=view*Mat4::from_translation(l.plane_translation.into());
        let eye_plane=specified.inverse().transpose()*Vec4::Y;
        let transform=matrix(&l.texture_ops);
        let values:Vec<_>=self.gl.draws[layer].vertices.iter().map(|v| {
            let p=Vec4::new(v.pos[0],v.pos[1],v.pos[2],1.0);
            let q=eye_plane.dot(view*p);
            let tex=transform*Vec4::new(p.x,p.z,1.0,q);
            [tex.x,tex.y,tex.w]
        }).collect();
        [0,1,2,0,2,3].map(|i|values[i]).to_vec()
    }
}
fn command(commands:&mut Vec<Command>, code:i32, args:[f32;4]) {commands.push(Command{code,args});}
fn scalar(commands:&mut Vec<Command>,code:i32,a:i32) {command(commands,code,[a as f32,0.0,0.0,0.0]);}
fn texop(commands:&mut Vec<Command>,ops:&mut Vec<Op>,code:u32,args:[f32;4]) {
    command(commands,code as i32+2,args);ops.push(Op{code,args});
}
pub fn time_fraction(millis:u64) -> f32 {(millis%700000) as f32/700000.0}
pub fn render(position:[f64;3],player:[f64;3],active:[f32;3],times:[f32;16]) -> Portal {
    let mut gl=Fixed::new(1.0);
    gl.enable(1,false);
    let mut commands=Vec::new();let mut layers=Vec::new();
    scalar(&mut commands,5,2896);
    let mut random=JavaRandom::new(31100);
    let [px,py,pz]=player.map(|x|x as f32);
    let [x,y,z]=position;
    for i in 0..16 {
        command(&mut commands,0,[0.0;4]);
        let mut depth=(16-i) as f32;
        let mut scale=1.0/16.0;
        let mut color_scale=1.0/(depth+1.0);
        if i==0 {
            gl.texture=80;scalar(&mut commands,11,80);
            color_scale=0.1;depth=65.0;scale=2.0/16.0;
            gl.enable(16,true);scalar(&mut commands,6,3042);
            gl.src=770;gl.dst=771;command(&mut commands,7,[770.0,771.0,0.0,0.0]);
        }
        if i==1 {
            gl.texture=83;scalar(&mut commands,11,83);
            gl.enable(16,true);scalar(&mut commands,6,3042);
            gl.src=1;gl.dst=1;command(&mut commands,7,[1.0,1.0,0.0,0.0]);scale=0.5;
        }
        let neg=-(y+0.75) as f32;
        let denominator=neg+active[1];
        let projected=denominator/(neg+depth+active[1])+(y+0.75) as f32;
        command(&mut commands,2,[px,projected,pz,0.0]);
        for (coord,mode) in [(8192,9217),(8193,9217),(8194,9217),(8195,9216)] {
            command(&mut commands,12,[coord as f32,9472.0,mode as f32,0.0]);
        }
        for (coord,plane,values) in [(8192,9473,[1.0,0.0,0.0,0.0]),(8193,9473,[0.0,0.0,1.0,0.0]),
            (8194,9473,[0.0,0.0,0.0,1.0]),(8195,9474,[0.0,1.0,0.0,0.0])] {
            command(&mut commands,13,[coord as f32,plane as f32,0.0,0.0]);command(&mut commands,14,values);
        }
        for cap in 3168..=3171 {scalar(&mut commands,6,cap);}
        command(&mut commands,1,[0.0;4]);scalar(&mut commands,8,5890);
        command(&mut commands,0,[0.0;4]);command(&mut commands,9,[0.0;4]);
        let mut ops=Vec::new();
        texop(&mut commands,&mut ops,0,[0.0,times[i],0.0,0.0]);
        texop(&mut commands,&mut ops,1,[scale,scale,scale,0.0]);
        texop(&mut commands,&mut ops,0,[0.5,0.5,0.0,0.0]);
        texop(&mut commands,&mut ops,2,[((i*i*4321+i*9) as f32)*2.0,0.0,0.0,1.0]);
        texop(&mut commands,&mut ops,0,[-0.5,-0.5,0.0,0.0]);
        texop(&mut commands,&mut ops,0,[-px,-pz,-py,0.0]);
        texop(&mut commands,&mut ops,0,[active[0]*depth/denominator,active[2]*depth/denominator,-py,0.0]);
        let mut r=random.next_float()*0.5+0.1;
        let mut g=random.next_float()*0.5+0.4;
        let mut b=random.next_float()*0.5+0.5;
        if i==0 {r=1.0;g=1.0;b=1.0;}
        gl.begin();gl.color([r*color_scale,g*color_scale,b*color_scale,1.0]);
        for p in [[x,y+0.75,z],[x,y+0.75,z+1.0],[x+1.0,y+0.75,z+1.0],[x+1.0,y+0.75,z]] {gl.vertex(p,[0.0;2]);}
        gl.untextured();gl.end();
        layers.push(Layer{texture_ops:ops,plane_translation:[px,projected,pz]});
        command(&mut commands,1,[0.0;4]);scalar(&mut commands,8,5888);
    }
    gl.enable(16,false);scalar(&mut commands,5,3042);
    for cap in 3168..=3171 {scalar(&mut commands,5,cap);}
    gl.enable(1,true);scalar(&mut commands,6,2896);
    Portal{gl,layers,commands}
}
