//! Beta 1.8.1 stems and vines. Random calls and neighbour order follow the bytecode.
use super::{behavior, RandSel, World};
use crate::block::*;

const DIR: [(i32, i32); 4] = [(0, 1), (-1, 0), (0, -1), (1, 0)];

/// BlockMushroom's bonemeal path restores both id and metadata on failure.
pub fn grow_mushroom(w:&mut World,x:i32,y:i32,z:i32,rs:RandSel)->bool {
    let id=w.get_block_id(x,y,z);
    let meta=w.get_block_metadata(x,y,z);w.set_block(x,y,z,AIR);
    if matches!(id,39|40) && big_mushroom(w,x,y,z,(id-39) as i32,rs) {true}
    else {w.set_block_and_metadata(x,y,z,id,meta);false}
}
/// WorldGenBigMushroom. Even a fixed type consumes the initial nextInt(2).
pub fn big_mushroom(w:&mut World,x:i32,y:i32,z:i32,kind:i32,rs:RandSel)->bool {
    let mut chosen=w.rng(rs).next_int(2);if kind>=0 {chosen=kind;}
    let height=w.rng(rs).next_int(3)+4;
    if y<1 || y+height+1>w.height10 {return false;}
    let mut clear=true;
    for by in y..=y+height+1 {
        let radius=if by==y {0}else{3};
        for bx in x-radius..=x+radius {if !clear {break;} for bz in z-radius..=z+radius {if !clear {break;}
            if by<0 || by>=w.height10 || !matches!(w.get_block_id(bx,by,bz),AIR|LEAVES) {clear=false;}
        }}
    }
    if !clear {return false;}
    if !matches!(w.get_block_id(x,y-1,z),DIRT|GRASS|110) {return false;}
    if !crate::entity::player::placement::can_place_at(w,39,x,y,z,1) {return false;}
    w.set_block(x,y-1,z,DIRT);
    let top=y+height;let start=if chosen==1 {top-3}else{top};
    for by in start..=top {
        let radius=if chosen==0 {3}else if by<top {2}else{1};
        for bx in x-radius..=x+radius {for bz in z-radius..=z+radius {
            let mut meta=5;
            if bx==x-radius {meta-=1;}if bx==x+radius {meta+=1;}
            if bz==z-radius {meta-=3;}if bz==z+radius {meta+=3;}
            if chosen==0 || by<top {
                if (bx==x-radius||bx==x+radius)&&(bz==z-radius||bz==z+radius) {continue;}
                if bx==x-(radius-1)&&bz==z-radius || bx==x-radius&&bz==z-(radius-1) {meta=1;}
                if bx==x+(radius-1)&&bz==z-radius || bx==x+radius&&bz==z-(radius-1) {meta=3;}
                if bx==x-(radius-1)&&bz==z+radius || bx==x-radius&&bz==z+(radius-1) {meta=7;}
                if bx==x+(radius-1)&&bz==z+radius || bx==x+radius&&bz==z+(radius-1) {meta=9;}
            }
            if meta==5&&by<top {meta=0;}
            // Vanilla tests the base y here, not the current cap layer.
            if (meta!=0||y>=top-1)&&!props(w.get_block_id(bx,by,bz)).opaque_cube {w.set_block_and_metadata(bx,by,bz,(99+chosen) as u8,meta);}
        }}
    }
    for dy in 0..height {if !props(w.get_block_id(x,y+dy,z)).opaque_cube {w.set_block_and_metadata(x,y+dy,z,(99+chosen) as u8,10);}}
    true
}

pub fn stem(w: &mut World, id: u8, x: i32, y: i32, z: i32, rs: RandSel) {
    behavior::check_flower(w, id, x, y, z);
    if w.get_block_light_value(x, y + 1, z) < 9 {
        return;
    }
    let mut growth = 1.0f32;
    let horizontal = w.get_block_id(x - 1, y, z) == id || w.get_block_id(x + 1, y, z) == id;
    let vertical = w.get_block_id(x, y, z - 1) == id || w.get_block_id(x, y, z + 1) == id;
    let diagonal = [(-1, -1), (1, -1), (1, 1), (-1, 1)]
        .into_iter()
        .any(|(dx, dz)| w.get_block_id(x + dx, y, z + dz) == id);
    for bx in x - 1..=x + 1 {
        for bz in z - 1..=z + 1 {
            let mut f = 0.0f32;
            if w.get_block_id(bx, y - 1, bz) == TILLED_FIELD {
                f = if w.get_block_metadata(bx, y - 1, bz) > 0 {
                    3.0
                } else {
                    1.0
                };
            }
            if bx != x || bz != z {
                f /= 4.0;
            }
            growth += f;
        }
    }
    if diagonal || horizontal && vertical {
        growth /= 2.0;
    }
    if w.rng(rs).next_int((25.0f32 / growth) as i32+1) != 0 {
        return;
    }
    let m = w.get_block_metadata(x, y, z);
    if m < 7 {
        w.set_block_metadata_with_notify(x, y, z, m + 1);
        return;
    }
    let fruit = if id == PUMPKIN_STEM { PUMPKIN } else { MELON };
    for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        if w.get_block_id(x + dx, y, z + dz) == fruit {
            return;
        }
    }
    let (dx, dz) = [(-1, 0), (1, 0), (0, -1), (0, 1)][w.rng(rs).next_int(4) as usize];
    if w.get_block_id(x + dx, y, z + dz) == AIR
        && w.get_block_id(x + dx, y - 1, z + dz) == TILLED_FIELD
    {
        w.set_block_with_notify(x + dx, y, z + dz, fruit);
    }
}

pub fn vine_support(id: u8) -> bool {
    id != AIR && props(id).normal_render && mat(material_of(id)).blocks_movement
}

pub fn vine_can_place(w: &mut World, x: i32, y: i32, z: i32, side: i32) -> bool {
    let (dx, dy, dz) = match side {
        1 => (0, 1, 0),
        2 => (0, 0, 1),
        3 => (0, 0, -1),
        4 => (1, 0, 0),
        5 => (-1, 0, 0),
        _ => return false,
    };
    vine_support(w.get_block_id(x + dx, y + dy, z + dz))
}

pub fn vine_neighbor(w: &mut World, x: i32, y: i32, z: i32) {
    let old = w.get_block_metadata(x, y, z);
    let mut m = old;
    for (i, (dx, dz)) in DIR.into_iter().enumerate() {
        let bit = 1 << i;
        if old & bit != 0
            && !vine_support(w.get_block_id(x + dx, y, z + dz))
            && (w.get_block_id(x, y + 1, z) != VINE || w.get_block_metadata(x, y + 1, z) & bit == 0)
        {
            m &= !bit;
        }
    }
    if m == 0 && !vine_support(w.get_block_id(x, y + 1, z)) {
        behavior::drop_block_as_item(w, VINE, x, y, z, old);
        w.set_block_with_notify(x, y, z, AIR);
    } else if m != old {
        w.set_block_metadata_with_notify(x, y, z, m);
    }
}

pub fn vine(w: &mut World, x: i32, y: i32, z: i32) {
    // Vanilla ignores the supplied updateTick Random and draws from World.rand.
    if w.multiplayer_world || w.rand.next_int(4) != 0 {
        return;
    }
    let mut remaining = 5;
    let mut crowded = false;
    'scan: for bx in x - 4..=x + 4 {
        for bz in z - 4..=z + 4 {
            for by in y - 1..=y + 1 {
                if w.get_block_id(bx, by, bz) == VINE {
                    remaining -= 1;
                    if remaining <= 0 {
                        crowded = true;
                        break 'scan;
                    }
                }
            }
        }
    }
    let m = w.get_block_metadata(x, y, z);
    let side = w.rand.next_int(6);
    let direction = [-1, -1, 2, 0, 1, 3][side as usize];
    if side == 1 && y < w.height10.wrapping_sub(1) && w.is_air_block(x, y + 1, z) {
        if crowded {
            return;
        }
        let mut bits = w.rand.next_int(16) & m;
        if bits > 0 {
            for (i, (dx, dz)) in DIR.into_iter().enumerate() {
                if !vine_support(w.get_block_id(x + dx, y + 1, z + dz)) {
                    bits &= !(1 << i);
                }
            }
            if bits > 0 {
                w.set_block_and_metadata_with_notify(x, y + 1, z, VINE, bits);
            }
        }
        return;
    }
    if (2..=5).contains(&side) && m & (1 << direction) == 0 {
        if crowded {
            return;
        }
        let d = direction as usize;
        let (dx, dz) = DIR[d];
        let id = w.get_block_id(x + dx, y, z + dz);
        if id != 0 && props(id).exists {
            if mat(material_of(id)).translucent_solid && props(id).normal_render {
                w.set_block_metadata_with_notify(x, y, z, m | 1 << d);
            }
        } else {
            let a = (d + 1) & 3;
            let b = (d + 3) & 3;
            let (ax, az) = DIR[a];
            let (bx, bz) = DIR[b];
            if m & (1 << a) != 0 && vine_support(w.get_block_id(x + dx + ax, y, z + dz + az)) {
                w.set_block_and_metadata_with_notify(x + dx, y, z + dz, VINE, 1 << a);
            } else if m & (1 << b) != 0 && vine_support(w.get_block_id(x + dx + bx, y, z + dz + bz))
            {
                w.set_block_and_metadata_with_notify(x + dx, y, z + dz, VINE, 1 << b);
            } else if m & (1 << a) != 0
                && w.is_air_block(x + dx + ax, y, z + dz + az)
                && vine_support(w.get_block_id(x + ax, y, z + az))
            {
                w.set_block_and_metadata_with_notify(
                    x + dx + ax,
                    y,
                    z + dz + az,
                    VINE,
                    1 << ((d + 2) & 3),
                );
            } else if m & (1 << b) != 0
                && w.is_air_block(x + dx + bx, y, z + dz + bz)
                && vine_support(w.get_block_id(x + bx, y, z + bz))
            {
                w.set_block_and_metadata_with_notify(
                    x + dx + bx,
                    y,
                    z + dz + bz,
                    VINE,
                    1 << ((d + 2) & 3),
                );
            } else if vine_support(w.get_block_id(x + dx, y + 1, z + dz)) {
                w.set_block_and_metadata_with_notify(x + dx, y, z + dz, VINE, 0);
            }
        }
    } else if y > 1 {
        let below = w.get_block_id(x, y - 1, z);
        if below == AIR {
            let bits = w.rand.next_int(16) & m;
            if bits > 0 {
                w.set_block_and_metadata_with_notify(x, y - 1, z, VINE, bits);
            }
        } else if below == VINE {
            let bits = w.rand.next_int(16) & m;
            let old = w.get_block_metadata(x, y - 1, z);
            if old != old | bits {
                w.set_block_metadata_with_notify(x, y - 1, z, old | bits);
            }
        }
    }
}

pub fn stem_direction(w:&mut World,id:u8,x:i32,y:i32,z:i32)->i32 {
    if w.get_block_metadata(x,y,z)<7{return -1;}let fruit=if id==PUMPKIN_STEM {PUMPKIN}else{MELON};
    for (k,(dx,dz)) in [(-1,0),(1,0),(0,-1),(0,1)].into_iter().enumerate(){if w.get_block_id(x+dx,y,z+dz)==fruit{return k as i32;}}-1
}
