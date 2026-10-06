//! Original-bytecode fixture for IInventory's b181 callbacks and base Slot behavior.
use super::*;
fn int(out: &mut Vec<u8>, n: i32) {
    out.extend(n.to_le_bytes());
}
fn stack(out: &mut Vec<u8>, s: Option<ItemStack>) {
    for n in s.map_or([-1, 0, 0, 0], |s| [s.id, s.count, s.damage, s.animations]) {
        int(out, n);
    }
}
pub fn replay() -> Vec<u8> {
    let mut out = Vec::new();
    for mode in 0..3 {
        let mut inv = InventoryPlayer::default();
        let mut c = Container::new(
            if mode == 0 {
                ContainerKind::Dispenser
            } else {
                ContainerKind::Player
            },
            0,
        );
        let size = if mode == 0 {
            9
        } else if mode == 1 {
            4
        } else {
            1
        };
        for t in 0..800 {
            let index = t % size;
            let slot = if mode == 0 {
                index
            } else if mode == 1 {
                index + 1
            } else {
                0
            };
            let count = [-1, 0, 1, 2, 63, 64, 65, 127][t % 8];
            let s = if t % 5 == 0 {
                None
            } else {
                Some(ItemStack::new(
                    [1, 35, 260, 267][t % 4],
                    count,
                    (t % 16) as i32,
                ))
            };
            c.put_stack(&mut inv, slot, s);
            // Basic/crafting/result openInventory and closeInventory are literal no-ops.
            out.push(c.stack(&inv, slot).is_some() as u8);
            stack(&mut out, c.stack(&inv, slot));
            let spec = c.slots[slot];
            int(&mut out, spec.stack_limit());
            int(&mut out, spec.background_icon());
            // InventoryCraftResult is exercised through a base Slot, rather than SlotCrafting.
            out.push(
                Slot {
                    kind: SlotKind::Normal,
                    ..spec
                }
                .is_item_valid(ItemStack::new(267, 1, 0)) as u8,
            );
            out.push(1);
            stack(&mut out, c.decrement(&mut inv, slot, (t % 7) as i32 - 1));
            for n in 0..size {
                stack(
                    &mut out,
                    c.stack(
                        &inv,
                        if mode == 0 {
                            n
                        } else if mode == 1 {
                            n + 1
                        } else {
                            0
                        },
                    ),
                );
            }
            if mode == 1 {
                stack(&mut out, c.result);
            }
        }
    }
    out
}
