//! Integrator checks that sit next to the world-blocks replays.
use super::*;

/// Mode_wbcake181: BlockCake.eatCakeSlice through right click (blockActivated) and left click
/// (onBlockClicked), for every slice, hunger, saturation and creative state.
pub(super) fn cake(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    let mut p = crate::entity::Entity::player(&mut w, "oracle");
    for meta in 0..8 {
        for food in [0, 7, 19, 20] {
            for sat in [0.0f32, 4.5, 20.0] {
                for creative in [false, true] {
                    for hook in 0..2 {
                        raw(&mut w, 8, 63, 8, 1, 0);
                        raw(&mut w, 8, 64, 8, 92, meta);
                        let s = p.player_state_mut().unwrap();
                        s.food.level = food;
                        s.food.saturation = sat;
                        s.capabilities.disable_damage = creative;
                        if hook == 0 {
                            p.block_clicked(&mut w, 92, 8, 64, 8);
                        } else {
                            o.push(p.activate_block(&mut w, 8, 64, 8) as u8);
                        }
                        let s = p.player_state().unwrap();
                        i(o, s.food.level);
                        i(o, s.food.saturation.to_bits() as i32);
                        i(o, w.get_block_id(8, 64, 8) as i32);
                        i(o, w.get_block_metadata(8, 64, 8));
                    }
                }
            }
        }
    }
}

/// Mode_wbsponge181: BlockSponge placed and removed next to water and an unsupported torch.
pub(super) fn sponge(o: &mut Vec<u8>) {
    let mut w = fixture(true);
    for layout in 0..4 {
        for x in 6..=10 {
            for y in 62..=66 {
                for z in 6..=10 {
                    raw(&mut w, x, y, z, if y < 64 { 1 } else { 0 }, 0);
                }
            }
        }
        if layout & 1 != 0 {
            raw(&mut w, 7, 64, 8, 9, 0);
        }
        if layout & 2 != 0 {
            raw(&mut w, 9, 64, 9, 0, 0);
            raw(&mut w, 9, 63, 9, 0, 0);
            raw(&mut w, 9, 64, 9, 50, 5);
        }
        super::round6::seed(&mut w);
        w.loaded_entity_list.clear();
        w.set_block_with_notify(8, 64, 8, 19);
        super::round6::result(o, &mut w);
        w.set_block_with_notify(8, 64, 8, 0);
        super::round6::result(o, &mut w);
    }
}
