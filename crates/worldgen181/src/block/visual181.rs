//! Immutable block texture/render defaults and the original creative item list.
//! Regenerate visual181.bin with oracle181 mode wbvisual (original bytecode).
use crate::entity::player::ItemStack;
const DATA: &[u8] = include_bytes!("visual181.bin");
const SIZE: usize = 4 + 6 * 4 + 16 * 7 * 4;
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
    assert!(side < 6);
    int(base(id, fancy) + 4 + side * 4)
}
pub fn texture_result(id: u8, side: usize, metadata: usize, fancy: bool) -> Result<i32, ()> {
    assert!(side < 6 && metadata < 16);
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
    if id == 2 && side > 1 {
        let above = super::material_of(block_at(x,y+1,z));
        return Ok(if matches!(above, super::MAT_SNOW | super::MAT_CRAFTEDSNOW) {68} else {3});
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
    let at = SIZE * 512;
    let count = int(at) as usize;
    (0..count)
        .map(|k| {
            let p = at + 4 + k * 12;
            ItemStack::new(int(p), int(p + 4), int(p + 8))
        })
        .collect()
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
    y: i32,
    z: i32,
    grass: &[u32],
    foliage: &[u32],
    mut climate: impl FnMut(i32, i32) -> (f32, f32),
) -> i32 {
    match id {
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
            let (mut x, mut z) = (x, z);
            if id == 31 {
                let mut n = x
                    .wrapping_mul(3129871)
                    .wrapping_add(z.wrapping_mul(6129781))
                    .wrapping_add(y) as i64;
                n = n
                    .wrapping_mul(n)
                    .wrapping_mul(42317861)
                    .wrapping_add(n.wrapping_mul(11));
                x = x.wrapping_add(((n >> 14 & 31) - 16) as i32);
                z = z.wrapping_add(((n >> 24 & 31) - 16) as i32);
            }
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
