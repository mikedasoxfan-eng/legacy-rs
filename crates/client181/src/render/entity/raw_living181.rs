//! Original draw inputs for Enderman, Silverfish and the two spider renderers.
use super::{fixed::Fixed, living::{MobKind,RenderState}, models::Model, special, objects::{self,ObjectKind,ObjectState}};
use worldgen::{JavaRandom,math::PI_F32};
pub fn embedded_arrows(g: &mut Fixed, m: &Model, id: i32, count: i32, partial: f32) {
    let mut r=JavaRandom::new(id as i64);
    for _ in 0..count {
        g.push();
        let p=m.random_part(&mut r);
        special::part(g,p,true);
        let mut v: [f32;3]=std::array::from_fn(|_|r.next_float());
        let xyz: [f32;3]=std::array::from_fn(|i|(p.raw_bounds[i]+(p.raw_bounds[i+3]-p.raw_bounds[i])*v[i])/16.0);
        g.translate(xyz[0],xyz[1],xyz[2]);
        for v in &mut v {*v=*v*2.0-1.0;*v *= -1.0;}
        let horizontal=(v[0]*v[0]+v[2]*v[2]).sqrt();
        let yaw=(worldgen::jmath::java_atan2(v[0] as f64,v[2] as f64)*180.0/PI_F32 as f64) as f32;
        let pitch=(worldgen::jmath::java_atan2(v[1] as f64,horizontal as f64)*180.0/PI_F32 as f64) as f32;
        let state=ObjectState {kind:ObjectKind::Arrow {yaw,pitch,shake:0,visible:true},position:[0.0;3],partial,brightness:g.material[0]};
        let arrow=objects::render(&state,[0.0;3],[0.0;2]);
        for mut draw in arrow.draws {
            let mut ops=g.ops.clone();ops.extend(draw.ops);draw.ops=ops;
            draw.material=g.material;draw.flags=(draw.flags & (64|128|512)) | (g.flags & !(64|128|512)) | 2;
            draw.src=g.src;draw.dst=g.dst;draw.depth=g.depth;
            g.draws.push(draw);
        }
        g.texture=3;g.normal=[0.0,0.0,0.05625];g.enable(2,false);
        g.pop();
    }
}
pub fn render(s:&RenderState,camera:[f64;3],normalize:bool)->Fixed {
    let mut g=Fixed::new(s.brightness);
    if normalize {g.enable(4,true);}
    let mut m=Model::new(super::living::model_kind(s.kind));
    m.on_ground=s.attack;m.is_riding=s.riding;m.carrying=s.carrying;m.screaming=s.screaming;
    let pose=[s.limb_swing,s.limb_amount.min(1.0),s.age,s.yaw-s.body_yaw,s.pitch,0.0625];m.set_rotation_angles(pose);
    g.push();g.translate((s.position[0]-camera[0]+s.scream_jitter[0]) as f32,(s.position[1]-camera[1]) as f32,(s.position[2]-camera[2]+s.scream_jitter[1]) as f32);
    g.rotate(180.0-s.body_yaw,0.0,1.0,0.0);
    if s.death_time>0 {let f=(((s.death_time as f32+s.partial-1.0)/20.0*1.6) as f64).sqrt() as f32;g.rotate(f.min(1.0)*if matches!(s.kind,MobKind::Enderman){90.0}else{180.0},0.0,0.0,1.0);}
    g.enable(2,true);g.scale(-1.0,-1.0,1.0);
    if matches!(s.kind,MobKind::Spider|MobKind::CaveSpider) {let scale=if s.kind==MobKind::CaveSpider {0.7}else{1.0};g.scale(scale,scale,scale);}
    g.translate(0.0,-24.0*0.0625-0.0078125,0.0);
    g.texture=match s.kind {MobKind::Enderman=>40,MobKind::Silverfish=>42,MobKind::Spider=>43,MobKind::CaveSpider=>45,_=>unreachable!()};
    special::model(&mut g,&m);
    let eyes=s.kind!=MobKind::Silverfish;
    if eyes {g.texture=if s.kind==MobKind::Enderman {41}else{44};g.enable(16,true);g.src=1;g.dst=1;g.material=[1.0;4];special::model(&mut g,&m);g.enable(16,false);}
    embedded_arrows(&mut g,&m,s.entity_id,s.arrow_count,s.partial);
    if s.kind==MobKind::Enderman && s.carried_block>0 {
        g.enable(2,true);g.push();g.translate(0.0,11.0/16.0,-12.0/16.0);g.rotate(20.0,1.0,0.0,0.0);g.rotate(45.0,0.0,1.0,0.0);g.scale(0.5,-0.5,0.5);g.material=[1.0;4];g.texture=1;
        super::inventory::inventory(&mut g,s.carried_block,s.carried_data,1.0,false);g.pop();g.enable(2,false);
    }
    if s.hurt_time>0 || s.death_time>0 {
        g.enable(8,false);g.enable(16,true);g.src=770;g.dst=771;g.depth=514;g.material=[s.overlay_brightness,0.0,0.0,0.4];
        special::model(&mut g,&m);
        if eyes {g.texture=if s.kind==MobKind::Enderman {41}else{44};g.src=1;g.dst=1;g.material=[s.overlay_brightness,0.0,0.0,0.4];special::model(&mut g,&m);}
        g.depth=515;g.enable(16,false);g.enable(8,true);
    }
    g.enable(2,false);g.pop();g
}

/// Lightmap coordinate transitions made by the original eye/carried-block callbacks.
pub fn packed_brightnesses(draws:&[super::fixed::Draw], base:i32)->Vec<i32> {
 let mut value=base;draws.iter().map(|d| {if matches!(d.texture,41|44){value=61680;}else if d.texture==1 {value=base;}value}).collect()
}
