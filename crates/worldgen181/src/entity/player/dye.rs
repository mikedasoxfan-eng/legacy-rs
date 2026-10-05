use super::*;
impl Entity {
    pub(super) fn dye_block(
        &mut self,
        w: &mut World,
        stack: &mut ItemStack,
        x: i32,
        y: i32,
        z: i32,
    ) -> bool {
        if stack.damage != 15 {
            return false;
        }
        match w.get_block_id(x, y, z) {
            6 => {
                // ItemDye: Block.sapling.growTree(world, x, y, z, world.rand)
                crate::world::ticks_blocks::grow_tree(w, x, y, z, crate::world::RandSel::World);
                stack.count -= 1;
                true
            }
            59 | 104 | 105 => {
                w.set_block_metadata_with_notify(x, y, z, 7);
                stack.count -= 1;
                true
            }
            2 => {
                stack.count -= 1;
                'attempt: for attempt in 0..128 {
                    let (mut bx, mut by, mut bz) = (x, y + 1, z);
                    for _ in 0..attempt / 16 {
                        bx += w.item_random.next_int(3) - 1;
                        by += (w.item_random.next_int(3) - 1) * w.item_random.next_int(3) / 2;
                        bz += w.item_random.next_int(3) - 1;
                        if w.get_block_id(bx, by - 1, bz) != 2 || w.is_block_normal_cube(bx, by, bz)
                        {
                            continue 'attempt;
                        }
                    }
                    if w.get_block_id(bx, by, bz) == 0 {
                        if w.item_random.next_int(10) != 0 {
                            w.set_block_and_metadata_with_notify(bx, by, bz, 31, 1);
                        } else {
                            let id = if w.item_random.next_int(3) != 0 {
                                37
                            } else {
                                38
                            };
                            w.set_block_with_notify(bx, by, bz, id);
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }
}
