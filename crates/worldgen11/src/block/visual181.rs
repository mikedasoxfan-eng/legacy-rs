//! Immutable block texture/render defaults and the original creative item list.
//! Regenerate visual181.bin with oracle10 mode wbvisual (original release bytecode).
use crate::entity::player::ItemStack;
const DATA: &[u8] = include_bytes!("visual181.bin");
const SIZE: usize = 4 + 6 * 4 + 16 * 7 * 4;
/// Block.func_35274_i, before subclass color overrides.
pub fn base_color181() -> i32 {
    0xffffff
}
/// Block.func_35276_e reads normal-cube state through the supplied access view.
pub fn ambient_occlusion181(
    x: i32, y: i32, z: i32, mut normal_cube: impl FnMut(i32, i32, i32) -> bool,
) -> f32 {
    if normal_cube(x, y, z) { 0.2 } else { 1.0 }
}
/// Block.getBlockBrightness. Registered stairs delegate to their model; fluids dispatch separately.
pub fn base_brightness181(
    id: u8, x: i32, y: i32, z: i32, mut brightness: impl FnMut(i32, i32, i32, i32) -> f32,
) -> f32 {
    brightness(x, y, z, super::light_value(stairs_model181(id).unwrap_or(id)))
}
/// Block.func_35275_c supplies emitted light as the minimum lightmap coordinate.
pub fn base_packed_light181(
    id: u8, x: i32, y: i32, z: i32, mut light: impl FnMut(i32, i32, i32, i32) -> i32,
) -> i32 {
    light(x, y, z, super::light_value(stairs_model181(id).unwrap_or(id)))
}
pub fn stairs_model181(id: u8) -> Option<u8> {match id {53=>Some(5),67=>Some(4),108=>Some(45),109=>Some(98),114=>Some(112),_=>None}}
/// Stairs discard metadata before calling the registered model's texture method.
pub fn stairs_texture181(id: u8, _side: i32, _metadata: i32) -> i32 {
    texture_from_side(stairs_model181(id).expect("stairs block"),0,false)
}
pub const SLAB_TYPES181: [&str;6]=["stone","sand","wood","cobble","brick","smoothStoneBrick"];
/// BlockTrapDoor.setBlockBoundsForBlockRender accepts a signed integer directly.
pub fn trapdoor_bounds181(metadata: i32) -> crate::entity::AxisAlignedBB {
    use crate::entity::AxisAlignedBB as BB;
    if metadata & 4 == 0 { return BB::new(0.0, 0.0, 0.0, 1.0, 0.1875, 1.0); }
    match metadata & 3 {
        0 => BB::new(0.0, 0.0, 0.8125, 1.0, 1.0, 1.0),
        1 => BB::new(0.0, 0.0, 0.0, 1.0, 1.0, 0.1875),
        2 => BB::new(0.8125, 0.0, 0.0, 1.0, 1.0, 1.0),
        _ => BB::new(0.0, 0.0, 0.0, 0.1875, 1.0, 1.0),
    }
}
/// Cactus overrides add in float before promoting to double; selection has an integer top.
pub fn cactus_box181(x: i32, y: i32, z: i32, selected: bool) -> crate::entity::AxisAlignedBB {
    crate::entity::AxisAlignedBB::new(
        (x as f32+0.0625) as f64,y as f64,(z as f32+0.0625) as f64,
        (x.wrapping_add(1) as f32-0.0625) as f64,
        if selected {y.wrapping_add(1) as f64}else{(y.wrapping_add(1) as f32-0.0625) as f64},
        (z.wrapping_add(1) as f32-0.0625) as f64,
    )
}
pub fn cactus_texture181(side: i32) -> i32 {match side {1=>69,0=>71,_=>70}}
pub fn door_state181(metadata:i32)->i32 {if metadata&4==0 {metadata.wrapping_sub(1)&3}else{metadata&3}}
pub fn piston_extension_texture181(side:i32,metadata:i32,override_texture:i32)->Result<i32,()> {
    let facing=metadata&7;
    if side==facing {return Ok(if override_texture>=0 {override_texture}else{if metadata&8!=0 {106}else{107}});}
    if facing>5 {return Err(());}
    Ok(if side==(facing^1) {107}else{108})
}
pub fn door_rotation_bounds181(state:i32)->crate::entity::AxisAlignedBB {
    use crate::entity::AxisAlignedBB as BB;
    match state {0=>BB::new(0.0,0.0,0.0,1.0,1.0,0.1875),1=>BB::new(0.8125,0.0,0.0,1.0,1.0,1.0),2=>BB::new(0.0,0.0,0.8125,1.0,1.0,1.0),3=>BB::new(0.0,0.0,0.0,0.1875,1.0,1.0),_=>BB::new(0.0,0.0,0.0,1.0,2.0,1.0)}
}
pub fn door_texture181(id:u8,side:i32,metadata:i32)->i32 {
    let base=if id==71 {98}else{97};
    if side==0||side==1 {return base;}
    let state=door_state181(metadata);
    if (state==0||state==2)^(side<=3) {return base;}
    let v=state/2+((side&1)^state)+(metadata&4)/4;
    let texture=base-(metadata&8)*2;
    if v&1!=0 {-texture}else{texture}
}
/// Both BlockCloth conversion methods use this same involution, including signed inputs.
pub fn cloth_dye181(value:i32)->i32 { !value & 15 }
pub fn cloth_texture181(metadata:i32)->i32 {
    if metadata==0 {64} else {let v=!(metadata&15);113+((v&8)>>3)+(v&7)*16}
}
pub fn slab_texture181(side:i32,metadata:i32)->i32 {
    match metadata {
        0=>if side<=1 {6}else{5},
        1=>match side {0=>208,1=>176,_=>192},
        2=>4,3=>16,4=>texture_from_side(45,0,false),5=>texture_from_side(98,0,false),_=>6,
    }
}
fn int(at: usize) -> i32 {
    i32::from_le_bytes(DATA[at..at + 4].try_into().unwrap())
}
fn base(id: u8, fancy: bool) -> usize {
    (fancy as usize * 256 + id as usize) * SIZE
}
pub fn render_type(id: u8) -> i32 {
    int(base(id, false))
}
pub fn texture_from_side(id: u8, side: usize, fancy: bool) -> i32 {
    if id == 84 { return jukebox_texture181(side as i32); }
    assert!(side < 6);
    if id == 81 {return cactus_texture181(side as i32);}
    if stairs_model181(id).is_some() {return stairs_texture181(id,side as i32,0);}
    int(base(id, fancy) + 4 + side * 4)
}
pub fn jukebox_texture181(side: i32) -> i32 { 74 + if side == 1 { 1 } else { 0 } }
pub fn texture_result(id: u8, side: usize, metadata: usize, fancy: bool) -> Result<i32, ()> {
    assert!(side < 6 && metadata < 16);
    if let Some(v)=crate::world::blocks10::texture(id,side as i32,metadata as i32){return Ok(v);}
    if id == 81 {return Ok(cactus_texture181(side as i32));}
    if stairs_model181(id).is_some() {return Ok(stairs_texture181(id,side as i32,metadata as i32));}
    if id==35 {return Ok(cloth_texture181(metadata as i32));}
    if matches!(id,43|44) {return Ok(slab_texture181(side as i32,metadata as i32));}
    let v = int(base(id, fancy) + 28 + metadata * 28 + 4 + side * 4);
    if v == i32::MIN {
        Err(())
    } else {
        Ok(v)
    }
}
pub fn texture(id: u8, side: usize, metadata: usize, fancy: bool) -> i32 {
    texture_result(id, side, metadata, fancy)
        .expect("original block texture metadata throws ArrayIndexOutOfBoundsException")
}
/// `Block.getBlockTexture(IBlockAccess, ...)`, including every coordinate-aware
/// override. The callbacks read the supplied access object, not necessarily World.
pub fn texture_at(
    id: u8, side: usize, metadata: usize, fancy: bool,
    x: i32, y: i32, z: i32, mut block_at: impl FnMut(i32,i32,i32)->u8,
) -> Result<i32, ()> {
    if matches!(id,2|110) && side > 1 {
        let above = super::material_of(block_at(x,y+1,z));
        return Ok(if matches!(above, super::MAT_SNOW | super::MAT_CRAFTEDSNOW) {68} else {if id==110 {77}else{3}});
    }
    if matches!(id, 23 | 61 | 62) {
        return Ok(if side <= 1 {62}
            else if side == metadata {if id==23 {46} else if id==62 {61} else {44}}
            else {45});
    }
    if matches!(id,54 | 95) {
        if side<=1 {return Ok(25);}
        let n=block_at(x,y,z-1);let s=block_at(x,y,z+1);
        let w=block_at(x-1,y,z);let e=block_at(x+1,y,z);
        let opaque=super::opaque_cube_lookup;
        if id==95 || (n!=id && s!=id && w!=id && e!=id) {
            let mut facing=3;
            if opaque(n)&&!opaque(s){facing=3;}
            if opaque(s)&&!opaque(n){facing=2;}
            if opaque(w)&&!opaque(e){facing=5;}
            if opaque(e)&&!opaque(w){facing=4;}
            return Ok(if side==facing {27} else {26});
        }
        if n!=id && s!=id {
            if side==4 || side==5 {return Ok(26);}
            let mut offset=if w==id {-1} else {0};
            let nx=if w==id {x-1} else {x+1};
            let nn=block_at(nx,y,z-1);let ns=block_at(nx,y,z+1);
            if side==3 {offset=-1-offset;}
            let mut facing=3;
            if (opaque(n)||opaque(nn))&&!opaque(s)&&!opaque(ns){facing=3;}
            if (opaque(s)||opaque(ns))&&!opaque(n)&&!opaque(nn){facing=2;}
            Ok((if side==facing {42} else {58})+offset)
        } else {
            if side==2 || side==3 {return Ok(26);}
            let mut offset=if n==id {-1} else {0};
            let nz=if n==id {z-1} else {z+1};
            let nw=block_at(x-1,y,nz);let ne=block_at(x+1,y,nz);
            if side==4 {offset=-1-offset;}
            let mut facing=5;
            if (opaque(w)||opaque(nw))&&!opaque(e)&&!opaque(ne){facing=5;}
            if (opaque(e)||opaque(ne))&&!opaque(w)&&!opaque(nw){facing=4;}
            Ok((if side==facing {42} else {58})+offset)
        }
    } else {texture_result(id,side,metadata,fancy)}
}
pub fn texture_at_with_bounds(
    id:u8,side:usize,metadata:usize,fancy:bool,x:i32,y:i32,z:i32,
    bounds:crate::entity::AxisAlignedBB,block_at:impl FnMut(i32,i32,i32)->u8,
) -> Result<i32,()> {
    if matches!(id,29|33) {
        let direction=metadata&7;
        let base=if id==29{106}else{107};
        return Ok(if direction>5{base}else if side==direction {
            if metadata&8==0&&bounds.min_x<=0.0&&bounds.min_y<=0.0&&bounds.min_z<=0.0&&bounds.max_x>=1.0&&bounds.max_y>=1.0&&bounds.max_z>=1.0 {base}else{110}
        }else if side==(direction^1){109}else{108});
    }
    texture_at(id,side,metadata,fancy,x,y,z,block_at)
}
pub fn pane_edge_texture(id:u8)->Option<i32>{match id{101=>Some(85),102=>Some(148),_=>None}}
pub fn opaque_instance(id:u8,fancy:bool)->bool {
    match id {18=>!fancy,43=>true,_=>super::opaque_cube_lookup(id)}
}
/// Neighbor coordinates and current shared block bounds, like shouldSideBeRendered.
pub fn should_side_render(
    id:u8,mut side:i32,bounds:crate::entity::AxisAlignedBB,fancy:bool,
    x:i32,y:i32,z:i32,mut block_at:impl FnMut(i32,i32,i32)->u8,
) -> bool {
    if matches!(id,93|94){return side!=0&&side!=1;}
    let neighbor=block_at(x,y,z);
    if id==90 {
        if neighbor==id{return false;}
        let along_x=(block_at(x-1,y,z)==id&&block_at(x-2,y,z)!=id)
            ||(block_at(x+1,y,z)==id&&block_at(x+2,y,z)!=id);
        let along_z=(block_at(x,y,z-1)==id&&block_at(x,y,z-2)!=id)
            ||(block_at(x,y,z+1)==id&&block_at(x,y,z+2)!=id);
        return (along_x&&matches!(side,4|5))||(along_z&&matches!(side,2|3));
    }
    if matches!(id,8..=11) {
        let material=super::material_of(neighbor);
        if material==super::material_of(id){return false;}
        if side==1{return true;}
        if material==super::MAT_ICE{return false;}
    }
    if (matches!(id,20|79|101|102)||(id==18&&!fancy))&&neighbor==id{return false;}
    if id==79{side=1-side;}
    if matches!(id,43|44|78)&&side==1{return true;}
    let base=match side {
        0 if bounds.min_y>0.0=>true,1 if bounds.max_y<1.0=>true,
        2 if bounds.min_z>0.0=>true,3 if bounds.max_z<1.0=>true,
        4 if bounds.min_x>0.0=>true,5 if bounds.max_x<1.0=>true,
        _=>!opaque_instance(neighbor,fancy),
    };
    base&&(!matches!(id,43|44)||side==0||neighbor!=id)
}
pub fn render_color(id: u8, metadata: usize, grass: &[u32]) -> i32 {
    assert!(metadata < 16);
    if id == 2 {
        lookup(grass, 0.5, 1.0) as i32
    } else {
        int(base(id, false) + 28 + metadata * 28)
    }
}
pub fn creative_stacks() -> Vec<ItemStack> {
    let data=include_bytes!("creative11.bin");
    let value=|at:usize|i32::from_le_bytes(data[at..at+4].try_into().unwrap());
    let count=value(0) as usize;
    (0..count)
        .map(|k| {
            let p = 4 + k * 12;
            ItemStack::new(value(p), value(p + 4), value(p + 8))
        })
        .collect()
}
/// BlockCake keeps integer overflow and float additions before double promotion.
pub fn cake_bounds181(metadata:i32)->crate::entity::AxisAlignedBB {
    crate::entity::AxisAlignedBB::new((1i32.wrapping_add(metadata.wrapping_mul(2)) as f32/16.0) as f64,0.0,0.0625,0.9375,0.5,0.9375)
}
pub fn cake_box181(x:i32,y:i32,z:i32,metadata:i32,selected:bool)->crate::entity::AxisAlignedBB {
    crate::entity::AxisAlignedBB::new(
        (x as f32+1i32.wrapping_add(metadata.wrapping_mul(2)) as f32/16.0) as f64,
        y as f64,(z as f32+0.0625) as f64,
        (x.wrapping_add(1) as f32-0.0625) as f64,
        (if selected {y as f32+0.5} else {y as f32+0.5-0.0625}) as f64,
        (z.wrapping_add(1) as f32-0.0625) as f64)
}
pub fn cake_texture181(side:i32,metadata:i32)->i32 {
    if side==1 {121} else if side==0 {124} else if metadata>0&&side==4 {123} else {122}
}
pub fn lookup(colors: &[u32], temperature: f64, mut rainfall: f64) -> u32 {
    rainfall *= temperature;
    let x = ((1.0 - temperature) * 255.0) as i32;
    let y = ((1.0 - rainfall) * 255.0) as i32;
    colors[(y << 8 | x) as usize]
}
/// The terrain integration supplies the original WorldChunkManager float climate.
pub fn color_multiplier(
    id: u8,
    metadata: i32,
    x: i32,
    _y: i32,
    z: i32,
    grass: &[u32],
    foliage: &[u32],
    mut climate: impl FnMut(i32, i32) -> (f32, f32),
) -> i32 {
    match id {
        111=>2129968,
        55 => 8388608,
        104 | 105 => {
            let r = metadata.wrapping_mul(32);
            let g = 255i32.wrapping_sub(metadata.wrapping_mul(8));
            let b = metadata.wrapping_mul(4);
            r.wrapping_shl(16) | g.wrapping_shl(8) | b
        }
        18 if metadata & 1 != 0 => 6396257,
        18 if metadata & 2 != 0 => 8431445,
        31 if metadata == 0 => 16777215,
        2 | 18 | 31 | 106 => {
            let (t, r) = climate(x, z);
            lookup(
                if matches!(id, 18 | 106) {
                    foliage
                } else {
                    grass
                },
                t as f64,
                r as f64,
            ) as i32
        }
        _ => 16777215,
    }
}

/// Block color dispatch over the 1.0 biome-owned color callbacks. The world
/// owner supplies biome colors, including subclass-specific swamp behavior.
pub fn color_multiplier10(id:u8,metadata:i32,x:i32,y:i32,z:i32,
    mut biome_color:impl FnMut(i32,i32,i32,bool)->i32,water_color:impl FnOnce(i32,i32)->i32)->i32 {
    match id {111=>2129968,55=>8388608,18 if metadata&1!=0=>6396257,18 if metadata&2!=0=>8431445,
        31 if metadata==0=>16777215,2|31=>biome_color(x,y,z,false),18|106=>biome_color(x,y,z,true),
        8|9=>water_color(x,z),104|105=>(metadata*32)<<16 | (255-metadata*8)<<8 | metadata*4,
        _=>16777215}
}

/// Release 1.1 block tint dispatch, averaging each color channel over the
/// original z-major 3x3 biome neighborhood for grass, oak leaves and water.
pub fn color_multiplier11(id:u8,metadata:i32,x:i32,y:i32,z:i32,
    mut biome_color:impl FnMut(i32,i32,i32,bool)->i32,mut water_color:impl FnMut(i32,i32)->i32)->i32 {
    let averaged = matches!(id,2|8|9) || id==18 && metadata&3==0;
    if !averaged {return color_multiplier10(id,metadata,x,y,z,biome_color,water_color);}
    let mut channels=[0i32;3];
    for dz in -1i32..=1 {for dx in -1i32..=1 {
        let bx=x.wrapping_add(dx);let bz=z.wrapping_add(dz);
        let color=if matches!(id,8|9){water_color(bx,bz)}else{biome_color(bx,y,bz,id==18)};
        channels[0]+=(color>>16)&255;channels[1]+=(color>>8)&255;channels[2]+=color&255;
    }}
    ((channels[0]/9)&255)<<16 | ((channels[1]/9)&255)<<8 | ((channels[2]/9)&255)
}

#[cfg(test)]
mod release11_tint_tests {
 use super::*;use std::cell::RefCell;
 fn color(x:i32,y:i32,z:i32,seed:i32,foliage:bool)->i32{x.wrapping_mul(733).wrapping_add(z.wrapping_mul(1237)).wrapping_add(y.wrapping_mul(19)).wrapping_add(seed.wrapping_mul(73471)).wrapping_add(if foliage{0x619fe2}else{0x325711})&0xffffff}
 #[test]
 fn block_neighborhood_tints_match_original_11_bytecode(){for line in include_str!("../fixtures/blocktint11.txt").lines(){let v:Vec<i32>=line.split_whitespace().map(|s|s.parse().unwrap()).collect();let queries=RefCell::new(Vec::new());let actual=color_multiplier11(v[0] as u8,v[1],v[3],64,v[4],|x,y,z,foliage|{queries.borrow_mut().extend([x,z]);color(x,y,z,v[2],foliage)},|x,z|{queries.borrow_mut().extend([x,z]);color(x,0,z,v[2],false)});assert_eq!(actual,v[5],"{line}");assert_eq!(queries.into_inner(),&v[6..],"query order {line}");}}
}
