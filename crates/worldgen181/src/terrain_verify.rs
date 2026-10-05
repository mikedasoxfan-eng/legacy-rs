//! Fixtures shared with Mode_features181. Only results, metadata and RNG states are compared.
use crate::{features::{self,TreeKind},features181,world::{World,chunk::Chunk},JavaRandom};
pub fn biomes(seed:i64)->Vec<u8> {
    crate::intcache::CACHE.with(|c| *c.borrow_mut()=Default::default());
    let mut m=crate::BiomeSource::new(seed);let mut out=Vec::new();
    for [x,z,w,h] in [[0,0,16,16],[-19,31,17,13],[-1,-1,1,1],[512,-1024,32,16],[-33,-41,9,7]] {
        out.extend(m.block_biomes(x,z,w,h).into_iter().map(|b|b as u8));
        let mut t=Vec::new();m.temperatures(&mut t,x,z,w,h);for v in t {out.extend_from_slice(&(v as f32).to_be_bytes());}
        for v in m.rainfalls(x,z,w,h) {out.extend_from_slice(&(v as f32).to_be_bytes());}
        out.extend(m.generation_biomes(x,z,w,h).into_iter().map(|b|b as u8));
    }
    let allowed=[crate::Biome::Forest,crate::Biome::Swampland,crate::Biome::Taiga];let mut rand=JavaRandom::new(seed);
    for radius in [4,64,256,16,0] {
        let p=m.find_position(-17,31,radius,&allowed,&mut rand);out.push(p.is_some() as u8);
        if let Some((x,z))=p {out.extend_from_slice(&x.to_be_bytes());out.extend_from_slice(&z.to_be_bytes());}
        out.extend_from_slice(&rand.state().to_be_bytes());out.push(m.viable(-17,31,radius,&allowed) as u8);
    }
    for [x,z] in [[-21,79],[-21,79],[1025,-512],[0,0]] {out.push(m.biome_at(x,z) as u8);out.extend_from_slice(&(m.raw_temperature(x,z) as f32).to_be_bytes());}
    out
}
pub fn feature(name:&str,seed:i64,fixture:i32)->Vec<u8> {
    let mut w=World::new(0,300);w.sand_fall_instantly=true;
    for cx in -3..=3 {for cz in -3..=3 {
        let mut blocks=vec![0;32768];
        for x in 0..16 {for z in 0..16 {for y in 0..=63 {blocks[(x<<11)|(z<<7)|y]=if y<61 {1} else if y<63 {3} else {2};}}}
        let mut c=Chunk::new(cx,cz,blocks.into_boxed_slice().try_into().unwrap());c.populated=true;c.loaded=true;
        w.chunks.insert((cx,cz),Box::new(c));
    }}
    if fixture==1 {for x in 3..=13 {for z in 3..=13 {let c=w.chunk_mut(x>>4,z>>4);c.blocks[((x&15)<<11|(z&15)<<7|64) as usize]=9;}}}
    if fixture==2 {w.chunk_mut(0,0).blocks[8<<11|8<<7|66]=1;}
    for cx in -3..=3 {for cz in -3..=3 {w.chunk_mut(cx,cz).init_height_and_sky();}}
    *w.prand()=JavaRandom::new(seed);
    let y=if name=="ore" {32} else if fixture==3 {123} else {64};
    let result=if let Some(id)=name.strip_prefix("decorate") {crate::populate::decorate(&mut w,crate::Biome::from_id(id.parse().unwrap()),0,0);true} else {match name {
        "oak"=>features::tree(&mut w,TreeKind::Oak,8,y,8),"birch"=>features::tree(&mut w,TreeKind::Birch,8,y,8),
        "taiga1"=>features::tree(&mut w,TreeKind::Spruce1,8,y,8),"taiga2"=>features::tree(&mut w,TreeKind::Spruce2,8,y,8),
        "big"=>features::tree(&mut w,TreeKind::BigOak,8,y,8),"swamp"=>features181::swamp_tree(&mut w,8,y,8),
        "brown"=>features181::big_mushroom(&mut w,Some(0),8,y,8),"red"=>features181::big_mushroom(&mut w,Some(1),8,y,8),
        "clay"=>features::clay(&mut w,4,8,y,8),"sand"=>features::sand(&mut w,7,12,8,y,8),"gravel"=>features::sand(&mut w,6,13,8,y,8),
        "ore"=>features::minable(&mut w,14,8,8,y,8),_=>panic!("unknown feature {name}"),
    }};
    let mut out=vec![result as u8];out.extend_from_slice(&w.prand().state().to_be_bytes());out.extend_from_slice(&w.rand.state().to_be_bytes());
    for cx in -3..=3 {for cz in -3..=3 {let c=w.chunk(cx,cz);out.extend_from_slice(&c.blocks[..]);out.extend_from_slice(&c.data.0[..]);}}
    out
}
