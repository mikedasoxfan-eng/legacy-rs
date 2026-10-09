//! Features introduced in Beta 1.8.1.
use crate::{
    block::{self, opaque_cube_lookup},
    world::World,
};

pub fn swamp_tree(w: &mut World, x: i32, mut y: i32, z: i32) -> bool {
    let height = w.prand().next_int(4) + 5;
    while w.get_block_material(x, y - 1, z) == block::MAT_WATER {
        y -= 1;
    }
    if y < 1 || y + height + 1 > 128 {
        return false;
    }
    let mut clear = true;
    for by in y..=y + 1 + height {
        let radius = if by == y {
            0
        } else if by >= y + 1 + height - 2 {
            3
        } else {
            1
        };
        for bx in x - radius..=x + radius {
            if !clear {
                break;
            }
            for bz in z - radius..=z + radius {
                if !clear {
                    break;
                }
                if !(0..128).contains(&by) {
                    clear = false;
                    continue;
                }
                let id = w.get_block_id(bx, by, bz);
                if id != block::AIR
                    && id != block::LEAVES
                    && (!(id == block::WATER_MOVING || id == block::WATER_STILL) || by > y)
                {
                    clear = false;
                }
            }
        }
    }
    if !clear {
        return false;
    }
    let ground = w.get_block_id(x, y - 1, z);
    if (ground != block::GRASS && ground != block::DIRT) || y >= 128 - height - 1 {
        return false;
    }
    w.set_block(x, y - 1, z, block::DIRT);
    for by in y - 3 + height..=y + height {
        let dy = by - (y + height);
        let radius = 2 - dy / 2;
        for bx in x - radius..=x + radius {
            for bz in z - radius..=z + radius {
                if ((bx - x).abs() != radius
                    || (bz - z).abs() != radius
                    || w.prand().next_int(2) != 0 && dy != 0)
                    && !opaque_cube_lookup(w.get_block_id(bx, by, bz))
                {
                    w.set_block(bx, by, bz, block::LEAVES);
                }
            }
        }
    }
    for dy in 0..height {
        let id = w.get_block_id(x, y + dy, z);
        if matches!(
            id,
            block::AIR | block::LEAVES | block::WATER_MOVING | block::WATER_STILL
        ) {
            w.set_block(x, y + dy, z, block::LOG);
        }
    }
    for by in y - 3 + height..=y + height {
        let radius = 2 - (by - (y + height)) / 2;
        for bx in x - radius..=x + radius {
            for bz in z - radius..=z + radius {
                if w.get_block_id(bx, by, bz) == block::LEAVES {
                    for (dx, dz, meta) in [(-1, 0, 8), (1, 0, 2), (0, -1, 1), (0, 1, 4)] {
                        if w.prand().next_int(4) == 0
                            && w.get_block_id(bx + dx, by, bz + dz) == block::AIR
                        {
                            vine(w, bx + dx, by, bz + dz, meta);
                        }
                    }
                }
            }
        }
    }
    true
}
fn vine(w: &mut World, x: i32, mut y: i32, z: i32, meta: i32) {
    w.set_block_and_metadata_with_notify(x, y, z, 106, meta);
    let mut remaining = 4;
    loop {
        y -= 1;
        if w.get_block_id(x, y, z) != block::AIR || remaining <= 0 {
            break;
        }
        w.set_block_and_metadata_with_notify(x, y, z, 106, meta);
        remaining -= 1;
    }
}
/// Minimum BlockVine support callback needed while swamp trees place hanging vines.
pub fn vine_neighbor(w: &mut World, x: i32, y: i32, z: i32) {
    let old = w.get_block_metadata(x, y, z);
    let mut meta = old;
    let supports = |id: u8| {
        id != 0
            && block::props(id).normal_render
            && block::mat(block::material_of(id)).blocks_movement
    };
    if old > 0 {
        for (i, (dx, dz)) in [(0, 1), (-1, 0), (0, -1), (1, 0)].into_iter().enumerate() {
            let bit = 1 << i;
            if old & bit != 0
                && !supports(w.get_block_id(x + dx, y, z + dz))
                && (w.get_block_id(x, y + 1, z) != 106
                    || w.get_block_metadata(x, y + 1, z) & bit == 0)
            {
                meta &= !bit;
            }
        }
    }
    if meta == 0 && !supports(w.get_block_id(x, y + 1, z)) {
        w.set_block_with_notify(x, y, z, 0);
    } else if meta != old {
        w.set_block_metadata_with_notify(x, y, z, meta);
    }
}
/// MapGenWaterlily.generate. BlockFlower.canPlaceBlockAt checks the block ID
/// below (still water), rather than BlockLilyPad.canBlockStay's metadata test.
pub fn waterlily(w: &mut World, x: i32, y: i32, z: i32) -> bool {
    for _ in 0..10 {
        let bx = x + w.prand().next_int(8) - w.prand().next_int(8);
        let by = y + w.prand().next_int(4) - w.prand().next_int(4);
        let bz = z + w.prand().next_int(8) - w.prand().next_int(8);
        if w.get_block_id(bx, by, bz) == 0 && w.get_block_id(bx, by - 1, bz) == block::WATER_STILL {
            w.set_block(bx, by, bz, 111);
        }
    }
    true
}

pub fn big_mushroom(w: &mut World, kind: Option<i32>, x: i32, y: i32, z: i32) -> bool {
    let random_kind = w.prand().next_int(2);
    let kind = kind.unwrap_or(random_kind);
    let height = w.prand().next_int(3) + 4;
    if y < 1 || y + height + 1 > 128 {
        return false;
    }
    let mut clear = true;
    for by in y..=y + 1 + height {
        let radius = if by == y { 0 } else { 3 };
        for bx in x - radius..=x + radius {
            if !clear {
                break;
            }
            for bz in z - radius..=z + radius {
                if !clear {
                    break;
                }
                if !(0..128).contains(&by)
                    || !matches!(w.get_block_id(bx, by, bz), block::AIR | block::LEAVES)
                {
                    clear = false;
                }
            }
        }
    }
    let below = w.get_block_id(x, y - 1, z);
    if !clear || !matches!(below, 2 | 3 | 110) || !opaque_cube_lookup(below) {
        return false;
    }
    let current = w.get_block_id(x, y, z);
    if current != block::AIR && !block::mat(w.get_block_material(x, y, z)).ground_cover {
        return false;
    }
    w.set_block(x, y - 1, z, block::DIRT);
    let top = y + height;
    let start = if kind == 1 { top - 3 } else { top };
    for by in start..=top {
        let radius = if kind == 0 {
            3
        } else if by < top {
            2
        } else {
            1
        };
        for bx in x - radius..=x + radius {
            for bz in z - radius..=z + radius {
                let mut meta = 5;
                if bx == x - radius {
                    meta -= 1;
                }
                if bx == x + radius {
                    meta += 1;
                }
                if bz == z - radius {
                    meta -= 3;
                }
                if bz == z + radius {
                    meta += 3;
                }
                if kind == 0 || by < top {
                    if (bx == x - radius || bx == x + radius)
                        && (bz == z - radius || bz == z + radius)
                    {
                        continue;
                    }
                    if (bx == x - (radius - 1) && bz == z - radius)
                        || (bx == x - radius && bz == z - (radius - 1))
                    {
                        meta = 1;
                    }
                    if (bx == x + (radius - 1) && bz == z - radius)
                        || (bx == x + radius && bz == z - (radius - 1))
                    {
                        meta = 3;
                    }
                    if (bx == x - (radius - 1) && bz == z + radius)
                        || (bx == x - radius && bz == z + (radius - 1))
                    {
                        meta = 7;
                    }
                    if (bx == x + (radius - 1) && bz == z + radius)
                        || (bx == x + radius && bz == z + (radius - 1))
                    {
                        meta = 9;
                    }
                }
                if meta == 5 && by < top {
                    meta = 0;
                }
                // The source tests the base y, rather than the cap y, in this condition.
                if (meta != 0 || y >= top - 1) && !opaque_cube_lookup(w.get_block_id(bx, by, bz)) {
                    w.set_block_and_metadata(bx, by, bz, 99 + kind as u8, meta);
                }
            }
        }
    }
    for dy in 0..height {
        if !opaque_cube_lookup(w.get_block_id(x, y + dy, z)) {
            w.set_block_and_metadata(x, y + dy, z, 99 + kind as u8, 10);
        }
    }
    true
}
