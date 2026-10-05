//! Original-bytecode replays for decorations and fishing.
use crate::{World,JavaRandom,entity::{Entity,EntitySeedSource},world::{tile::TileEntity,chunk::{Chunk,idx}}};
fn i(o:&mut Vec<u8>,v:i32){o.extend_from_slice(&v.to_le_bytes());}
fn text(o:&mut Vec<u8>,s:&str){i(o,s.len() as i32);o.extend_from_slice(s.as_bytes());}
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
