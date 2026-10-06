use worldgen::{
    entity::{
        player::{controller::PlayerControllerSP, inventory::ItemStack},
        Entity,
    },
    JavaRandom,
};
fn i(o: &mut Vec<u8>, v: i32) {
    o.extend(v.to_le_bytes());
}
fn stack(o: &mut Vec<u8>, s: Option<ItemStack>) {
    for v in s.map_or([-1, 0, 0, 0], |s| [s.id, s.count, s.damage, s.animations]) {
        i(o, v);
    }
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    let mut out = vec![];
    let mut r = JavaRandom::new(6281);
    let mut w = worldgen::entity::verify181::world();
    let mut owner = Entity::player(&mut w, "Player");
    let mut supplied = Entity::player(&mut w, "Supplied");
    for c in 0..10000 {
        let creative = c % 2 == 0;
        let s = supplied.player_state_mut().unwrap();
        s.capabilities.disable_damage = c & 1 != 0;
        s.capabilities.flying = c & 2 != 0;
        s.capabilities.allow_flying = c & 4 != 0;
        s.capabilities.creative = c & 8 != 0;
        for slot in 0..36 {
            s.inventory.main[slot] = if (c + slot) % 4 == 0 {
                None
            } else {
                Some(ItemStack::new(
                    [260, 261, 35, 278][slot as usize % 4],
                    r.next_int(256) - 128,
                    r.next_int(1000) - 100,
                ))
            };
        }
        owner.player_state_mut().unwrap().inventory.main[0] =
            Some(ItemStack::new(264, 91, c as i32));
        supplied.initialize_player_mode(creative);
        let damage = (r.next_int(2001) - 1000) as f32 / 317.0;
        let previous = (r.next_int(2001) - 1000) as f32 / 319.0;
        let mut controller = PlayerControllerSP::default();
        controller.cur_block_damage = damage;
        controller.prev_block_damage = previous;
        let music = controller.update_for_mode(creative);
        let s = supplied.player_state().unwrap();
        let b = s.capabilities;
        for v in [b.disable_damage, b.flying, b.allow_flying, b.creative] {
            out.push(v as u8);
        }
        for s in s.inventory.main {
            stack(&mut out, s);
        }
        stack(&mut out, owner.held_item());
        i(&mut out, music as i32);
        out.extend(controller.prev_block_damage.to_le_bytes());
        out.extend(PlayerControllerSP::block_reach(creative).to_le_bytes());
        for v in [
            creative,
            PlayerControllerSP::should_draw_hud(creative),
            false,
            !creative,
            !creative,
            creative,
            PlayerControllerSP::has_extended_entity_reach(creative),
        ] {
            out.push(v as u8);
        }
    }
    std::fs::write(&a[1], &out).unwrap();
    let expected = std::fs::read(&a[2]).unwrap();
    if out != expected {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "CONTROLLER MODE DIFFERS byte {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        std::process::exit(1);
    }
    println!(
        "CONTROLLER MODE EXACT: 10000 original initialization/update/flag cases ({} bytes)",
        out.len()
    );
}
