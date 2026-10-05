use worldgen::{
    entity::{
        Entity,
        player::{InventoryPlayer, controller::PlayerControllerSP, inventory::ItemStack},
    },
    nbt::Tag,
};
fn i(out: &mut Vec<u8>, value: i32) {
    out.extend(value.to_le_bytes());
}
fn stack(out: &mut Vec<u8>, value: Option<ItemStack>) {
    for n in value.map_or([-1, 0, 0, 0], |s| [s.id, s.count, s.damage, s.animations]) {
        i(out, n);
    }
}
fn inventory(out: &mut Vec<u8>, inv: &InventoryPlayer) {
    for s in inv.main.iter().chain(&inv.armor) {
        stack(out, *s);
    }
    stack(out, inv.cursor);
    i(out, inv.current_item);
    out.push(inv.changed as u8);
    let Tag::List(_, tags) = inv.to_nbt() else {
        unreachable!()
    };
    let mut payload = vec![if tags.is_empty() { 1 } else { 10 }];
    payload.extend((tags.len() as i32).to_be_bytes());
    // Sorted compound keys, as in PlayerOracle.canonical; production NBT uses Java HashMap order.
    for t in tags {
        let Tag::Compound(n) = t else { unreachable!() };
        for (key, ty) in [("Count", 1), ("Damage", 2), ("Slot", 1), ("id", 2)] {
            payload.push(ty);
            payload.extend((key.len() as u16).to_be_bytes());
            payload.extend(key.as_bytes());
            if ty == 1 {
                payload.push(n.byte(key) as u8);
            } else {
                payload.extend(n.short(key).to_be_bytes());
            }
        }
        payload.push(0);
    }
    i(out, payload.len() as i32);
    out.extend(payload);
}
fn tag(tags: &mut Vec<Tag>, slot: i32, id: i32, count: i32, damage: i32) {
    let mut n = ItemStack::new(id, count, damage).to_nbt();
    n.put("Slot", Tag::Byte(slot as i8));
    tags.push(Tag::Compound(n));
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    let mut out = Vec::new();
    let mut w = worldgen::entity::verify181::world();
    let mut p = Entity::player(&mut w, "Player");
    let mut v = InventoryPlayer::default();
    v.current_item = 8;
    v.changed = true;
    v.cursor = Some(ItemStack::new(35, 3, 14));
    let counts = [-128, -1, 0, 1, 64, 127, 128, 255];
    let damages = [-32768, -1, 0, 32767, 32768, 65535];
    for c in 0..4516 {
        let mut tags = Vec::new();
        tag(
            &mut tags,
            c % 256,
            c % 2258,
            counts[c as usize % 8],
            damages[c as usize % 6],
        );
        tag(&mut tags, 0, 260, c % 127, 0);
        tag(&mut tags, 0, if c % 2 == 0 { 400 } else { 261 }, 1, 0);
        tag(&mut tags, 100, 310, 1, c % 400);
        tag(&mut tags, 103, 313, -1, -1);
        v.read_nbt(&tags);
        inventory(&mut out, &v);
        for id in [260, 261, 262, 310, 313, 400, c % 2258] {
            out.push(v.contains_item(id) as u8);
        }
    }
    for slot in 0..36 {
        for count in [-1, 0, 1] {
            v.main = [None; 36];
            v.armor = [None; 4];
            v.current_item = 5;
            v.main[slot] = Some(ItemStack::new(262, count, 0));
            v.armor[0] = Some(ItemStack::new(260, 1, 0));
            out.push(v.contains_item(262) as u8);
            out.push(v.contains_item(260) as u8);
            v.set_current_item(262);
            i(&mut out, v.current_item);
        }
    }
    let mut r = worldgen::JavaRandom::new(5181);
    for c in 0..6000 {
        p.living.health = r.next_int(80) - 20;
        p.living.prev_health = r.next_int(80) - 20;
        p.living.hearts_life = r.next_int(40);
        p.living.last_damage = r.next_int(20);
        p.living.hurt_time = r.next_int(20);
        p.living.max_hurt_time = r.next_int(20);
        p.set_creative(c % 2 == 0);
        p.player_state_mut().unwrap().inventory.armor[0] = Some(ItemStack::new(313, 1, 0));
        p.start_item_use(ItemStack::new(267, 1, 0), 72000);
        p.set_player_health(r.next_int(80) - 20);
        for n in [
            p.living.health,
            p.living.prev_health,
            p.living.hearts_life,
            p.living.last_damage,
            p.living.hurt_time,
            p.living.max_hurt_time,
        ] {
            i(&mut out, n);
        }
        out.extend(p.player_state().unwrap().food.exhaustion.to_le_bytes());
        stack(&mut out, p.player_state().unwrap().inventory.armor[0]);
        let points = r.next_int_unbounded();
        let total = r.next_int_unbounded();
        let level = r.next_int_unbounded();
        p.set_player_experience(points, total, level);
        let xp = p.player_state().unwrap().experience;
        for n in [xp.points, xp.total, xp.level, xp.bar_cap()] {
            i(&mut out, n);
        }
    }
    let mut controller = PlayerControllerSP::default();
    for _ in 0..10000 {
        controller.cur_block_damage = (r.next_int(2001) - 1000) as f32 / 317.0;
        controller.prev_block_damage = (r.next_int(2001) - 1000) as f32 / 319.0;
        let partial = (r.next_int(2001) - 1000) as f32 / 321.0;
        let damage = controller.damage_partial(partial);
        out.extend(damage.to_le_bytes());
        out.extend(damage.to_le_bytes());
    }
    std::fs::write(&args[1], &out).unwrap();
    let expected = std::fs::read(&args[2]).unwrap();
    if out != expected {
        let at = out
            .iter()
            .zip(&expected)
            .position(|(a, b)| a != b)
            .unwrap_or(out.len().min(expected.len()));
        eprintln!(
            "PLAYER STATE DIFFERS at byte {at}, lengths {}/{}",
            out.len(),
            expected.len()
        );
        std::process::exit(1);
    }
    println!(
        "PLAYER STATE EXACT: 4516 inventory NBT cases, 108 membership/pick cases, 6000 health/XP callbacks, 10000 mining interpolations ({} bytes)",
        out.len()
    );
}
