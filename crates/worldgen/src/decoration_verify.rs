//! Original-bytecode replays for decorations and fishing.
use crate::{World,JavaRandom,entity::{Entity,EntitySeedSource},world::{tile::TileEntity,chunk::{Chunk,idx}}};
fn i(o:&mut Vec<u8>,v:i32){o.extend_from_slice(&v.to_le_bytes());}
fn text(o:&mut Vec<u8>,s:&str){i(o,s.len() as i32);o.extend_from_slice(s.as_bytes());}
fn painting_state(o:&mut Vec<u8>,p:&Entity,w:&mut World) {
    let crate::entity::EntityKind::Painting(s)=&p.kind else{unreachable!()};
    for v in [s.art as i32,s.direction,s.tile[0],s.tile[1],s.tile[2],s.check_counter]{i(o,v);}
    for v in [p.pos.x,p.pos.y,p.pos.z,p.bounding_box.min_x,p.bounding_box.min_y,p.bounding_box.min_z,p.bounding_box.max_x,p.bounding_box.max_y,p.bounding_box.max_z]{o.extend_from_slice(&v.to_bits().to_le_bytes());}
    for v in [p.yaw,p.prev_yaw]{o.extend_from_slice(&v.to_bits().to_le_bytes());}
    for v in [p.dead as i32,p.been_attacked as i32,p.painting_valid_surface(w) as i32]{i(o,v);}o.extend_from_slice(&p.rand.state().to_le_bytes());i(o,w.loaded_entity_list.len() as i32);
}
fn raw(w:&mut World,x:i32,y:i32,z:i32,id:u8) {w.chunk_mut(x>>4,z>>4).blocks[idx(x&15,y,z&15)]=id;}
pub fn paintings()->Vec<u8> {
    let mut o=Vec::new();let widths=[8,1,2,1,2,4,4,4];let heights=[8,1,1,2,2,2,3,4];
    for scene in 0..12{for dir in 0..4{for seed in 0..8{
        let mut w=flat_world();w.entity_seed_source=EntitySeedSource::Deterministic(100+seed);
        let width=widths[scene%8];let height=heights[scene%8];let material=match scene{8=>20,9=>18,10=>44,11=>0,_=>1};
        for a in 0..width{for b in 0..height{let x=if dir==0||dir==2{a-width/2}else{0};let z=if dir==0||dir==2{0}else{a-width/2};raw(&mut w,x,67+b-height/2,z,material);}}
        let mut p=Entity::painting(&mut w,0,67,0,dir);painting_state(&mut o,&p,&mut w);
        let n=p.to_nbt().unwrap();let q=Entity::from_nbt(&mut w,&n).unwrap();painting_state(&mut o,&q,&mut w);
        if scene%3==0 {w.entity_joined_world(p.clone());let overlap=Entity::painting(&mut w,0,67,0,dir);painting_state(&mut o,&overlap,&mut w);}
        for y in 64..73{for a in -4..=4{raw(&mut w,if dir==0||dir==2{a}else{0},y,if dir==0||dir==2{0}else{a},0);}}
        for _ in 0..103{p.on_update(&mut w);painting_state(&mut o,&p,&mut w);}
        p.attack(&mut w,None,0);painting_state(&mut o,&p,&mut w);
    }}}o
}
fn sign(o:&mut Vec<u8>,s:Option<&crate::sign::Sign>){i(o,s.is_some() as i32);if let Some(s)=s{for s in &s.text{text(o,s);}}}
pub fn flat_world()->World {
    let mut w=World::new(0,300);w.entity_seed_source=EntitySeedSource::Deterministic(100);w.math_random=JavaRandom::new(200);
    for x in -3..=3 {for z in -3..=3 {let mut b=vec![0u8;32768];for bx in 0..16{for bz in 0..16{b[idx(bx,63,bz)]=1;}}let mut c=Chunk::new(x,z,b.into_boxed_slice().try_into().unwrap());c.populated=true;c.loaded=true;w.chunks.insert((x,z),Box::new(c));}}
    w
}
pub fn signs(allowed:&str)->Vec<u8> {
    let mut w=flat_world();let mut p=Entity::player(&mut w,"Player");let mut o=Vec::new();
    for support in [1,20,18,44,85,0,9] {for side in 0..6 {for yaw in [-720.0,-181.0,-180.0,-168.75,-0.1,0.0,11.25,179.99,180.0,359.99,720.0] {
        w.chunk_mut(0,0).blocks[idx(0,64,0)]=support;p.yaw=yaw;p.player_state_mut().unwrap().events.clear();
        let (x,y,z)=crate::entity::player::placement::offset(0,64,0,side);
        if y!=63 {w.set_block(x,y,z,0);}
        let mut stack=crate::entity::player::ItemStack::new(323,1,0);let used=p.place_sign(&mut w,&mut stack,0,64,0,side);
        for v in [used as i32,stack.count,p.player_state().unwrap().events.len() as i32,w.get_block_id(x,y,z) as i32,w.get_block_metadata(x,y,z)]{i(&mut o,v);}
        let s=if matches!(w.get_block_id(x,y,z),63|68) {match w.get_block_tile_entity(x,y,z){Some(TileEntity::Sign(s))=>Some(s),_=>None}}else{None};sign(&mut o,s.as_deref());
        w.set_block_with_notify(0,64,0,0);i(&mut o,w.get_block_id(x,y,z) as i32);o.extend_from_slice(&w.rand.state().to_le_bytes());
        w.set_block(x,y,z,0);if y==63 {w.chunk_mut(x>>4,z>>4).blocks[idx(x&15,y,z&15)]=1;}
    }}}
    let mut editor=crate::sign::Editor::default();let mut r=JavaRandom::new(1729);
    for _ in 0..2000 {let key=[200,208,28,14,1,0,0,0,0][r.next_int(9) as usize];let c=char::from_u32(r.next_int(256) as u32).unwrap();editor.key_typed(c,key,allowed.contains(c));editor.update_counter+=1;i(&mut o,editor.edit_line as i32);i(&mut o,editor.update_counter);sign(&mut o,Some(&editor.sign));}
    let mut n=crate::nbt::Compound::new();for j in 1..=4 {n.put(&format!("Text{j}"),crate::nbt::Tag::String("01234567890123456789012345".into()));}sign(&mut o,Some(&crate::sign::Sign::from_nbt(&n)));o
}
