use super::*;
pub const DYE_COLOR_NAMES: [&str; 16] = ["black", "red", "green", "brown", "blue", "purple", "cyan", "silver", "gray", "pink", "lime", "yellow", "lightBlue", "magenta", "orange", "white"];
pub const DYE_COLORS: [i32; 16] = [1973019,11743532,3887386,5320730,2437522,8073150,2651799,2651799,4408131,14188952,4312372,14602026,6719955,12801229,15435844,15790320];
/// ItemDye clamps damage before indexing its 16 color variants.
pub fn icon(damage: i32) -> i32 { let damage=damage.clamp(0,15); 78+damage%8*16+damage/8 }
pub fn name(damage: i32) -> Result<String, &'static str> {
    DYE_COLOR_NAMES.get(damage.clamp(0,15) as usize).map(|s| format!("item.dyePowder.{s}")).ok_or("ArrayIndexOutOfBoundsException")
}
impl Entity {
    pub(super) fn dye_block(
        &mut self,
        w: &mut World,
        stack: &mut ItemStack,
        x: i32,
        y: i32,
        z: i32,
    ) -> bool {
        if !self.can_edit_block181(x,y,z) || stack.damage != 15 {
            return false;
        }
        if w.multiplayer_world {
            return matches!(w.get_block_id(x,y,z), 2 | 6 | 39 | 40 | 59 | 104 | 105);
        }
        match w.get_block_id(x, y, z) {
            39 | 40 => {
                if crate::world::plants181::grow_mushroom(w,x,y,z,crate::world::RandSel::World) {stack.count=stack.count.wrapping_sub(1);}
                true
            }
            6 => {
                // ItemDye: Block.sapling.growTree(world, x, y, z, world.rand)
                crate::world::ticks_blocks::grow_tree(w, x, y, z, crate::world::RandSel::World);
                stack.count = stack.count.wrapping_sub(1);
                true
            }
            59 | 104 | 105 => {
                w.set_block_metadata_with_notify(x, y, z, 7);
                stack.count = stack.count.wrapping_sub(1);
                true
            }
            2 => {
                stack.count = stack.count.wrapping_sub(1);
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
