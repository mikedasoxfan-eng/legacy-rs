//! Release 1.0 blocks round 4: item, block and tile-entity probes on the production code.
use super::*;
use crate::entity::{player::ItemStack, potion::PotionEffect, Entity, EntityKind, EntitySeedSource};

/// Diagnostic: B4DEBUG=1 prints each case's starting offset to stderr.
pub(super) fn case(o: &[u8], what: std::fmt::Arguments) {
    if std::env::var_os("B4DEBUG").is_some() {
        eprintln!("CASE {} {}", o.len(), what);
    }
}
fn bool8(o: &mut Vec<u8>, b: bool) {
    o.push(b as u8);
}
fn d(o: &mut Vec<u8>, v: f64) {
    o.extend((if v.is_nan() { 0x7ff8000000000000 } else { v.to_bits() }).to_le_bytes());
}
fn text(o: &mut Vec<u8>, s: &str) {
    i(o, s.len() as i32);
    o.extend(s.as_bytes());
}
pub(super) fn stack(o: &mut Vec<u8>, s: Option<ItemStack>) {
    bool8(o, s.is_some());
    if let Some(s) = s {
        i(o, s.id);
        i(o, s.count);
        i(o, s.damage);
        bool8(o, s.has_tag());
    }
}
pub(super) fn seeds(w: &mut World) {
    w.rand.set_seed(42);
    w.item_random.set_seed(123);
    w.entity_seed_source = EntitySeedSource::Deterministic(100);
    w.math_random.set_seed(456);
}
pub(super) fn rngs(o: &mut Vec<u8>, w: &World) {
    l(o, w.rand.state());
    l(o, w.item_random.state());
    l(o, w.math_random.state());
}
pub(super) fn entities(o: &mut Vec<u8>, w: &World) {
    i(o, w.loaded_entity_list.len() as i32);
    for id in &w.loaded_entity_list {
        let e = &w.entities[id];
        // EntityList.getEntityString, or the class name for unregistered classes.
        text(o, match e.kind.name() {
            "ThrownEgg" => "EntityEgg",
            "Potion" => "EntityPotion",
            "FishingHook" => "EntityFishHook",
            "LightningBolt" => "EntityLightningBolt",
            name => name,
        });
        for v in [e.pos.x, e.pos.y, e.pos.z, e.motion.x, e.motion.y, e.motion.z] {
            d(o, v);
        }
        f181(o, e.yaw);
        f181(o, e.pitch);
        match &e.kind {
            EntityKind::Item { item, pickup_delay, .. } => {
                stack(o, Some(ItemStack::from(*item)));
                i(o, *pickup_delay);
            }
            EntityKind::Potion(_, damage) => i(o, *damage),
            EntityKind::Arrow(a) => {
                bool8(o, a.critical);
                bool8(o, a.belongs_to_player);
            }
            _ => {}
        }
    }
}
pub(super) fn effects(o: &mut Vec<u8>, e: &Entity) {
    let mut list = e.living.effects.clone();
    list.sort_by_key(|p| p.id);
    i(o, list.len() as i32);
    for p in list {
        i(o, p.id);
        i(o, p.duration);
        i(o, p.amplifier);
    }
}
pub(super) fn player(o: &mut Vec<u8>, p: &Entity) {
    let s = p.player_state().unwrap();
    for slot in s.inventory.main {
        stack(o, slot);
    }
    stack(o, s.item_in_use);
    i(o, if s.item_in_use.is_some() { s.item_use_ticks } else { 0 });
    let held = s.inventory.main[s.inventory.current_item as usize];
    bool8(o, s.item_in_use.is_some_and(|u| held.is_some_and(|h| h.same_object(u))));
    let food = s.food.snapshot();
    i(o, food.level);
    f181(o, food.saturation);
    effects(o, p);
    l(o, p.rand.state());
}
pub(super) fn new_player(w: &mut World) -> Entity {
    Entity::player(w, "Player")
}
pub(super) fn place(p: &mut Entity, x: f64, y: f64, z: f64, yaw: f32, pitch: f32) {
    p.set_location_and_angles(x, y, z, yaw, pitch);
    p.prev_yaw = yaw;
    p.prev_pitch = pitch;
    p.rand.set_seed(77);
}
fn cells(o: &mut Vec<u8>, w: &mut World, lo: i32, hi: i32, bottom: i32, top: i32) {
    for x in lo..=hi {
        for y in bottom..=top {
            for z in lo..=hi {
                o.push(w.get_block_id(x, y, z));
                o.push(w.get_block_metadata(x, y, z) as u8);
            }
        }
    }
}

const RIGHT_IDS: [i32; 31] = [
    325, 326, 327, 335, 333, 332, 344, 368, 373, 374, 111, 261, 322, 260, 282, 297, 319, 320, 349,
    350, 357, 360, 363, 364, 365, 366, 367, 375, 267, 1, 358,
];
const POTIONS: [i32; 7] = [0, 16, 8193, 8204, 16385, 16396, 32767];
fn right_terrain(w: &mut World, t: usize) {
    let cell = [(1, 0), (9, 0), (8, 3), (11, 0), (0, 0), (78, 0), (9, 0), (9, 0)];
    raw(w, 8, 64, 8, cell[t].0, cell[t].1);
    if t == 6 {
        raw(w, 8, 65, 8, 111, 0);
    }
    if t == 7 {
        let mut pig = crate::entity::registry::create(w, "Pig").unwrap();
        pig.set_location_and_angles(8.5, 65.0, 10.5, 0.0, 0.0);
        w.entity_joined_world(pig);
    }
}
pub(super) fn right(o: &mut Vec<u8>) {
    for id in RIGHT_IDS {
        let damages: &[i32] = if id == 373 { &POTIONS } else { &[0] };
        for &damage in damages {
            for t in 0..8 {
                for pitch in [90.0f32, 45.0, 0.0] {
                    for yaw in [0.0f32, 135.0] {
                        for mode in 0..3 {
                            for remote in [false, true] {
                                let mut w = fixture(true);
                                let mut p = new_player(&mut w);
                                right_terrain(&mut w, t);
                                place(&mut p, 8.5, 65.0, 8.5, yaw, pitch);
                                let creative = mode == 2;
                                {
                                    let s = p.player_state_mut().unwrap();
                                    s.capabilities.creative = creative;
                                    s.capabilities.disable_damage = creative;
                                    if mode == 1 {
                                        s.food.write().level = 6;
                                    }
                                    if yaw == 0.0 {
                                        s.inventory.main[5] = Some(ItemStack::new(262, 3, 0));
                                    }
                                }
                                if mode == 1 {
                                    p.add_potion_effect(PotionEffect::new(1, 200, 1));
                                }
                                p.player_state_mut().unwrap().inventory.main[0] =
                                    Some(ItemStack::new(id, if mode == 1 { 2 } else { 1 }, damage));
                                w.multiplayer_world = remote;
                                seeds(&mut w);
                                let used = p.use_item(&mut w);
                                bool8(o, used);
                                player(o, &p);
                                cells(o, &mut w, 6, 10, 62, 67);
                                entities(o, &w);
                                rngs(o, &w);
                                crate::world::events::write_events(&w.drain_events(), o);
                            }
                        }
                    }
                }
            }
        }
    }
}

const FOODS: [i32; 20] = [260, 282, 297, 319, 320, 322, 349, 350, 357, 360, 363, 364, 365, 366, 367, 375, 335, 373, 261, 267];
const EAT_POTIONS: [i32; 13] = [0, 16, 8193, 8194, 8196, 8201, 8204, 8225, 8257, 16385, 16389, 16396, 32767];
pub(super) fn eat(o: &mut Vec<u8>) {
    for id in FOODS {
        let damages: &[i32] = if id == 373 { &EAT_POTIONS } else { &[0] };
        for &damage in damages {
            for amount in [1, 2] {
                for food in [20, 3] {
                    for effects in 0..3 {
                        for remote in [false, true] {
                            let mut w = fixture(true);
                            let mut p = new_player(&mut w);
                            place(&mut p, 8.5, 65.0, 8.5, 30.0, 20.0);
                            p.player_state_mut().unwrap().food.write().level = food;
                            if effects >= 1 {
                                p.add_potion_effect(PotionEffect::new(10, 40, 0));
                                p.add_potion_effect(PotionEffect::new(17, 2000, 1));
                            }
                            if effects == 2 {
                                p.add_potion_effect(PotionEffect::new(1, 9000, 3));
                                p.add_potion_effect(PotionEffect::new(19, 10, 0));
                                p.add_potion_effect(PotionEffect::new(6, 1, 0));
                            }
                            {
                                let s = p.player_state_mut().unwrap();
                                if amount == 2 {
                                    for slot in 1..36 {
                                        s.inventory.main[slot] = Some(ItemStack::new(1, 64, 0));
                                    }
                                }
                                let stack = ItemStack::new(id, amount, damage);
                                s.inventory.main[0] = Some(stack);
                                s.item_in_use = Some(stack);
                                s.item_use_slot = 0;
                                s.item_use_ticks = 1;
                            }
                            w.multiplayer_world = remote;
                            seeds(&mut w);
                            p.finish_item_use(&mut w);
                            player(o, &p);
                            entities(o, &w);
                            rngs(o, &w);
                            crate::world::events::write_events(&w.drain_events(), o);
                        }
                    }
                }
            }
        }
    }
    for remaining in [72000, 71999, 71998, 71997, 71995, 71992, 71990, 71986, 71980, 71970, 71950, 71900, 0, -5] {
        for arrows in 0..3 {
            for creative in [false, true] {
                for damage in [0, 383, 384] {
                    for remote in [false, true] {
                        case(o, format_args!("bow {remaining} {arrows} {creative} {damage} {remote}"));
                        let mut w = fixture(true);
                        let mut p = new_player(&mut w);
                        place(&mut p, 8.5, 65.0, 8.5, -60.0, -10.0);
                        {
                            let s = p.player_state_mut().unwrap();
                            s.capabilities.creative = creative;
                            s.capabilities.disable_damage = creative;
                            if arrows >= 1 {
                                s.inventory.main[7] = Some(ItemStack::new(262, arrows, 0));
                            }
                            let bow = ItemStack::new(261, 1, damage);
                            s.inventory.main[0] = Some(bow);
                            s.item_in_use = Some(bow);
                            s.item_use_slot = 0;
                            s.item_use_ticks = remaining;
                        }
                        w.multiplayer_world = remote;
                        seeds(&mut w);
                        p.stop_item_use(&mut w);
                        player(o, &p);
                        entities(o, &w);
                        rngs(o, &w);
                        crate::world::events::write_events(&w.drain_events(), o);
                    }
                }
            }
        }
    }
}

const USE_ITEMS: [(i32, i32); 53] = [
    (44, 0), (44, 1), (44, 2), (44, 3), (44, 4), (44, 5), (111, 0), (106, 0), (31, 0), (31, 1), (31, 2), (18, 0), (18, 1), (18, 2),
    (6, 0), (6, 1), (6, 2), (17, 0), (17, 1), (17, 2), (98, 0), (98, 1), (98, 2), (98, 3), (35, 0), (35, 14), (295, 0), (361, 0),
    (362, 0), (372, 0), (351, -1), (351, 0), (351, 3), (351, 4), (351, 15), (351, 16), (355, 0), (2256, 0), (2257, 0), (2266, 0),
    (324, 0), (330, 0), (379, 0), (380, 0), (354, 0), (116, 0), (120, 0), (122, 0), (110, 0), (113, 0), (120, 0), (373, 0), (373, 16385),
];
const USE_CONFIGS: [(u8, i32); 23] = [
    (0, 0), (1, 0), (2, 0), (3, 0), (60, 0), (78, 0), (106, 1), (31, 1), (8, 1), (9, 0), (51, 0), (1, 0), (88, 0), (44, 0), (44, 3),
    (104, 3), (59, 2), (6, 1), (39, 0), (115, 1), (110, 0), (84, 0), (84, 1),
];
pub(super) fn item_use(o: &mut Vec<u8>) {
    use crate::world::tile::TileEntity;
    for (id, damage) in USE_ITEMS {
        for side in 0..6 {
            for config in 0..USE_CONFIGS.len() {
                case(o, format_args!("use {id}:{damage} side {side} config {config}"));
                let mut w = fixture(true);
                let mut p = new_player(&mut w);
                place(&mut p, 8.5, 66.0, 5.5, -450.0 + 90.0 * config as f32, 30.0);
                raw(&mut w, 8, 64, 8, USE_CONFIGS[config].0, USE_CONFIGS[config].1);
                if config == 11 {
                    for (dx, dz) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                        raw(&mut w, 8 + dx, 64, 8 + dz, 1, 0);
                    }
                }
                if config == 12 || config == 19 {
                    raw(&mut w, 8, 63, 8, 88, 0);
                }
                if config == 15 || config == 16 {
                    raw(&mut w, 8, 63, 8, 60, 7);
                }
                if config == 18 {
                    raw(&mut w, 8, 63, 8, 3, 0);
                }
                if config == 20 {
                    raw(&mut w, 9, 65, 8, 2, 0);
                }
                if config >= 21 {
                    w.set_block_tile_entity(8, 64, 8, TileEntity::RecordPlayer { record: 0 });
                }
                if config == 22 {
                    if let Some(TileEntity::RecordPlayer { record }) = w.get_block_tile_entity(8, 64, 8) {
                        *record = 2258;
                    }
                }
                let mut s = ItemStack::new(id, [1, 4, 0, 1][config % 4], damage);
                seeds(&mut w);
                bool8(o, p.item_use_on_block181(&mut w, &mut s, 8, 64, 8, side));
                stack(o, Some(s));
                cells(o, &mut w, 6, 10, 62, 67);
                let record = match w.get_block_tile_entity(8, 64, 8) {
                    Some(TileEntity::RecordPlayer { record }) => Some(*record),
                    _ => None,
                };
                bool8(o, record.is_some());
                if let Some(r) = record {
                    i(o, r);
                }
                entities(o, &w);
                rngs(o, &w);
                crate::world::events::write_events(&w.drain_events(), o);
            }
        }
    }
}

const ITEM_DAMAGES: [i32; 12] = [-1, 0, 1, 2, 3, 15, 16, 8193, 8194, 16385, 16396, 32767];
const VISUAL: [i32; 36] = [
    17, 18, 31, 35, 44, 98, 106, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 351, 372, 373, 374, 375, 376,
    377, 378, 379, 380, 381, 382, 2256, 2257, 2258, 2266,
];
fn opt_text(o: &mut Vec<u8>, s: Option<&str>) {
    bool8(o, s.is_some());
    if let Some(s) = s {
        text(o, s);
    }
}
pub(super) fn items(o: &mut Vec<u8>) {
    use crate::entity::player::registry;
    let identity = |s: &str| s.to_string();
    for id in 0..2267 {
        let exists = registry::item(id).is_some();
        bool8(o, exists);
        if !exists {
            continue;
        }
        let ingredient = crate::potion_helper::ingredient(id);
        bool8(o, ingredient.is_some());
        opt_text(o, ingredient);
        i(o, crate::enchantment::item_enchantability(id));
        for damage in ITEM_DAMAGES {
            for ench in [false, true] {
                let s = ItemStack::new(id, 1, damage);
                // ItemStack.func_40706_d rejects tags on stackable items.
                if ench && s.max_stack() != 1 {
                    continue;
                }
                if ench {
                    s.add_enchantment(32, 2);
                }
                i(o, s.rarity10() as i32);
                bool8(o, s.has_effect10());
                bool8(o, s.is_enchantable());
                i(o, registry::use_action(id) as i32);
                i(o, registry::use_duration(id));
                let lines = s.tooltip10(
                    |_| String::new(),
                    |s, lines| {
                        if s.id == 373 {
                            lines.extend(crate::item10::PotionItem::default().tooltip(s.damage, identity));
                        }
                        if let Some(record) = crate::item10::record_tooltip(s.id) {
                            lines.push(record);
                        }
                    },
                    identity,
                );
                i(o, lines.len() as i32);
                for line in &lines[1..] {
                    text(o, line);
                }
                if id == 373 {
                    text(o, &crate::item10::PotionItem::default().display_name(damage, "", identity));
                }
            }
        }
    }
    for id in VISUAL {
        for damage in -2..20 {
            case(o, format_args!("visual {id}:{damage}"));
            let (icon, color) = match crate::item10::item_properties(id, damage) {
                Some((icon, color, _)) => (icon, color),
                None => registry::block_item_properties(id, damage, true).unwrap_or_else(|| panic!("visual item {id}")),
            };
            i(o, icon);
            i(o, color as i32);
            if id < 256 {
                i(o, registry::placed_block_metadata(id, damage));
            }
            match id {
                31 | 35 | 44 => opt_text(o, registry::block_item_name(id, damage).as_deref()),
                351 => opt_text(o, Some(&crate::entity::player::dye::name(damage).unwrap())),
                _ => {}
            }
        }
    }
}

pub(super) fn harvest(o: &mut Vec<u8>) {
    for id in 1..=122u8 {
        if !crate::block::props(id).exists {
            continue;
        }
        for meta in [0, 1, 2, 3, 5, 8, 15] {
            for held in 0..6 {
                for remote in [false, true] {
                    case(o, format_args!("harvest {id}:{meta} held {held} remote {remote}"));
                    let mut w = fixture(true);
                    let mut p = new_player(&mut w);
                    place(&mut p, 8.5, 66.0, 6.5, 0.0, 40.0);
                    if meta & 1 != 0 {
                        raw(&mut w, 8, 63, 8, 0, 0);
                    }
                    raw(&mut w, 8, 64, 8, id, meta);
                    let s = (held != 0).then(|| ItemStack::new([0, 278, 278, 278, 359, 285][held], 1, 0));
                    if let Some(s) = s {
                        match held {
                            2 => s.add_enchantment(33, 1),
                            3 => s.add_enchantment(35, 3),
                            5 => s.add_enchantment(35, 1),
                            _ => {}
                        }
                    }
                    p.player_state_mut().unwrap().inventory.main[0] = s;
                    w.multiplayer_world = remote;
                    seeds(&mut w);
                    p.harvest_block(&mut w, id, 8, 64, 8, meta);
                    cells(o, &mut w, 7, 9, 63, 65);
                    entities(o, &w);
                    rngs(o, &w);
                    crate::world::events::write_events(&w.drain_events(), o);
                    f181(o, p.player_state().unwrap().food.snapshot().exhaustion);
                }
            }
        }
    }    for stored in [89, 73, 18, 1, 59] {
        for held in 0..6 {
            for remote in [false, true] {
                case(o, format_args!("moving piston {stored} held {held} remote {remote}"));
                let mut w = fixture(true);
                let mut p = new_player(&mut w);
                place(&mut p, 8.5, 66.0, 6.5, 0.0, 40.0);
                raw(&mut w, 8, 64, 8, 36, 3);
                w.set_block_tile_entity(8, 64, 8, crate::world::tile::TileEntity::Piston(crate::world::pistons::Piston::new(stored, if stored == 59 { 7 } else { 2 }, 3, true, false)));
                let s = (held != 0).then(|| ItemStack::new([0, 278, 278, 278, 359, 285][held], 1, 0));
                if let Some(s) = s {
                    match held {
                        2 => s.add_enchantment(33, 1),
                        3 => s.add_enchantment(35, 3),
                        5 => s.add_enchantment(35, 1),
                        _ => {}
                    }
                }
                p.player_state_mut().unwrap().inventory.main[0] = s;
                w.multiplayer_world = remote;
                seeds(&mut w);
                p.harvest_block(&mut w, 36, 8, 64, 8, 3);
                cells(o, &mut w, 7, 9, 63, 65);
                entities(o, &w);
                rngs(o, &w);
                crate::world::events::write_events(&w.drain_events(), o);
            }
        }
    }
}

// ------------------------------------------------------------------ block callbacks
fn queue(o: &mut Vec<u8>, w: &World) {
    i(o, w.tick_order.len() as i32);
    for &(time, _, key) in &w.tick_order {
        for v in [key.x, key.y, key.z, key.block] {
            i(o, v);
        }
        l(o, time);
    }
}
fn tick_state(o: &mut Vec<u8>, w: &mut World, x: i32, y: i32, z: i32) {
    for dx in -2..=2 {
        for dy in -3..=2 {
            for dz in -2..=2 {
                o.push(w.get_block_id(x + dx, y + dy, z + dz));
                o.push(w.get_block_metadata(x + dx, y + dy, z + dz) as u8);
            }
        }
    }
    queue(o, w);
    entities(o, w);
    rngs(o, w);
    crate::world::events::write_events(&w.drain_events(), o);
    i(o, w.loaded_tile_entities.len() as i32);
}
fn set_dimension(w: &mut World, dimension: i32) {
    use crate::world::Dimension;
    let d = match dimension {
        -1 => Dimension::Nether,
        1 => Dimension::Sky,
        _ => return,
    };
    w.dimension = d;
    w.light_brightness_table = d.light_brightness_table();
    w.terrain.biome_source = if dimension == -1 { crate::biome::BiomeSource::hell() } else { crate::biome::BiomeSource::sky() };
}
fn egg(o: &mut Vec<u8>) {
    for hook in 0..3 {
        for instant in [false, true] {
            for x in [8, 40] {
                for y in [64, 20] {
                    for below in [1u8, 0, 9, 51, 11, 78] {
                        for remote in [false, true] {
                            case(o, format_args!("egg {hook} {instant} {x} {y} {below} {remote}"));
                            let mut w = fixture(true);
                            if y == 20 {
                                raw(&mut w, x, 10, 8, 1, 0);
                            }
                            raw(&mut w, x, y - 1, 8, below, 0);
                            raw(&mut w, x, y, 8, 122, 5);
                            w.multiplayer_world = remote;
                            w.sand_fall_instantly = instant;
                            seeds(&mut w);
                            match hook {
                                0 => behavior::on_block_added(&mut w, 122, x, y, 8),
                                1 => behavior::on_neighbor_block_change(&mut w, 122, x, y, 8, 1),
                                _ => behavior::update_tick(&mut w, 122, x, y, 8, RandSel::World),
                            }
                            w.sand_fall_instantly = false;
                            tick_state(o, &mut w, x, y, 8);
                            for yy in y - 11..y {
                                o.push(w.get_block_id(x, yy, 8));
                            }
                        }
                    }
                }
            }
        }
    }
}
fn tick_setup(w: &mut World, id: u8, s: i32) {
    match id {
        110 | 2 => {
            if s == 1 {
                raw(w, 8, 65, 8, 1, 0);
            }
            if s == 2 {
                raw(w, 8, 65, 8, 20, 0);
            }
            for (x, y, z) in [(7, 63, 8), (9, 64, 8), (8, 62, 9), (7, 64, 7), (9, 63, 9), (8, 61, 7)] {
                raw(w, x, y, z, 3, 0);
            }
        }
        106 => {
            if s != 3 {
                raw(w, 8, 64, 9, 1, 0);
                raw(w, 8, 65, 9, 1, 0);
            }
            if s == 1 {
                raw(w, 8, 63, 8, 0, 0);
                raw(w, 8, 62, 8, 0, 0);
            }
            if s == 2 {
                raw(w, 9, 65, 8, 1, 0);
                raw(w, 7, 65, 8, 1, 0);
                raw(w, 9, 64, 9, 1, 0);
                raw(w, 7, 64, 9, 1, 0);
            }
        }
        104 | 105 => {
            raw(w, 8, 63, 8, 60, if s == 1 { 7 } else { 0 });
            raw(w, 7, 63, 8, 3, 0);
            raw(w, 9, 63, 8, 2, 0);
            raw(w, 8, 63, 7, 60, 0);
            raw(w, 8, 63, 9, 1, 0);
            if s == 2 {
                raw(w, 9, 64, 8, if id == 104 { 86 } else { 103 }, 0);
            }
            if s == 3 {
                raw(w, 7, 64, 7, 59, 3);
                raw(w, 7, 63, 7, 60, 7);
                raw(w, 9, 64, 9, 104, 2);
                raw(w, 9, 63, 9, 60, 0);
            }
        }
        115 => raw(w, 8, 63, 8, if s == 1 { 3 } else { 88 }, 0),
        51 => {
            raw(w, 8, 63, 8, [7, 87, 1, 0][s as usize], 0);
            raw(w, 9, 64, 8, 5, 0);
            raw(w, 8, 65, 8, 17, 0);
        }
        _ => {}
    }
}
fn random_ticks(o: &mut Vec<u8>) {
    let cases: [(u8, i32); 20] = [
        (115, 0), (115, 2), (115, 3), (110, 0), (2, 0), (106, 1), (106, 2), (106, 4), (106, 8), (106, 5), (106, 15), (106, 0),
        (104, 0), (104, 6), (104, 7), (105, 0), (105, 7), (51, 0), (51, 5), (51, 15),
    ];
    for (id, meta) in cases {
        for s in 0..4 {
            for dimension in [0, -1, 1] {
                for bright in [true, false] {
                    for seed in 0..6i64 {
                        for remote in [false, true] {
                            if dimension != 0 && id != 115 && id != 51 {
                                continue;
                            }
                            if !bright && id != 110 && id != 2 {
                                continue;
                            }
                            case(o, format_args!("tick {id}:{meta} s{s} dim{dimension} bright {bright} seed {seed} remote {remote}"));
                            let mut w = fixture(bright);
                            set_dimension(&mut w, dimension);
                            tick_setup(&mut w, id, s);
                            raw(&mut w, 8, 64, 8, id, meta);
                            w.multiplayer_world = remote;
                            seeds(&mut w);
                            w.rand.set_seed(seed * 31 + 7);
                            behavior::update_tick(&mut w, id, 8, 64, 8, RandSel::World);
                            tick_state(o, &mut w, 8, 64, 8);
                        }
                    }
                }
            }
        }
    }
}
fn added(o: &mut Vec<u8>) {
    use std::sync::atomic::Ordering::Relaxed;
    for id in [86u8, 91] {
        for snow in 0..4 {
            for remote in [false, true] {
                case(o, format_args!("pumpkin {id} {snow} {remote}"));
                let mut w = fixture(true);
                if snow & 1 != 0 {
                    raw(&mut w, 8, 63, 8, 80, 0);
                }
                if snow & 2 != 0 {
                    raw(&mut w, 8, 62, 8, 80, 0);
                }
                raw(&mut w, 8, 64, 8, id, 2);
                w.multiplayer_world = remote;
                seeds(&mut w);
                behavior::on_block_added(&mut w, id, 8, 64, 8);
                tick_state(o, &mut w, 8, 64, 8);
            }
        }
    }
    for flag in [false, true] {
        for dimension in [0, -1, 1] {
            for remote in [false, true] {
                case(o, format_args!("endportal {flag} {dimension} {remote}"));
                let mut w = fixture(true);
                set_dimension(&mut w, dimension);
                raw(&mut w, 8, 64, 8, 119, 0);
                w.multiplayer_world = remote;
                crate::world::blocks10::END_PORTAL_ALLOW.store(flag, Relaxed);
                seeds(&mut w);
                behavior::on_block_added(&mut w, 119, 8, 64, 8);
                crate::world::blocks10::END_PORTAL_ALLOW.store(false, Relaxed);
                tick_state(o, &mut w, 8, 64, 8);
            }
        }
    }
    for frame in 0..3 {
        for dimension in [0, -1, 1] {
            for remote in [false, true] {
                case(o, format_args!("fire {frame} {dimension} {remote}"));
                let mut w = fixture(true);
                set_dimension(&mut w, dimension);
                if frame >= 1 {
                    raw(&mut w, 8, 63, 8, 49, 0);
                }
                if frame == 2 {
                    for x in 7..=10 {
                        raw(&mut w, x, 63, 8, 49, 0);
                        raw(&mut w, x, 67, 8, 49, 0);
                    }
                    for y in 64..=66 {
                        raw(&mut w, 7, y, 8, 49, 0);
                        raw(&mut w, 10, y, 8, 49, 0);
                    }
                }
                raw(&mut w, 8, 64, 8, 51, 0);
                w.multiplayer_world = remote;
                seeds(&mut w);
                behavior::on_block_added(&mut w, 51, 8, 64, 8);
                tick_state(o, &mut w, 8, 64, 8);
            }
        }
    }
}
fn activation(o: &mut Vec<u8>) {
    use crate::entity::player::PlayerEvent;
    for id in [116u8, 117] {
        for tile in [false, true] {
            for remote in [false, true] {
                case(o, format_args!("activate {id} {tile} {remote}"));
                let mut w = fixture(true);
                let mut p = new_player(&mut w);
                raw(&mut w, 8, 64, 8, id, 0);
                if tile {
                    w.get_block_tile_entity(8, 64, 8);
                }
                w.multiplayer_world = remote;
                seeds(&mut w);
                p.player_state_mut().unwrap().events.clear();
                bool8(o, p.activate_block(&mut w, 8, 64, 8));
                let events = &p.player_state().unwrap().events;
                for block in [116u8, 117] {
                    i(o, events.iter().filter(|e| matches!(e, PlayerEvent::OpenContainer { block: b, .. } if *b == block)).count() as i32);
                }
                tick_state(o, &mut w, 8, 64, 8);
            }
        }
    }
}
fn end_collision(o: &mut Vec<u8>) {
    for who in 0..4 {
        for remote in [false, true] {
            case(o, format_args!("end collision {who} {remote}"));
            let mut w = fixture(true);
            raw(&mut w, 8, 64, 8, 119, 0);
            let mut p = new_player(&mut w);
            place(&mut p, 8.5, 64.0, 8.5, 0.0, 0.0);
            let pig = crate::entity::registry::create(&mut w, "Pig").unwrap();
            let mut zombie = crate::entity::registry::create(&mut w, "Zombie").unwrap();
            if who == 1 {
                p.riding = Some(pig.entity_id);
            }
            if who == 2 {
                p.ridden_by = Some(pig.entity_id);
            }
            w.multiplayer_world = remote;
            seeds(&mut w);
            p.player_state_mut().unwrap().stat_log.clear();
            let target = if who == 3 { &mut zombie } else { &mut p };
            target.on_entity_collided_with_block(&mut w, 119, 8, 64, 8);
            // EntityPlayerSP.func_40182_b(1) always reports an End achievement on this path.
            i(o, (!p.player_state().unwrap().stat_log.is_empty()) as i32);
        }
    }
}
fn removal(o: &mut Vec<u8>) {
    use crate::world::tile::TileEntity;
    let fills: [&[i32]; 6] = [
        &[],
        &[0, 373, 1, 8193],
        &[3, 372, 64, 0],
        &[0, 373, 1, 16385, 1, 373, 1, 0, 2, 373, 1, 8204, 3, 331, 45, 0],
        &[3, 348, 0, 0],
        &[1, 373, -3, 5],
    ];
    let tile_id = |w: &mut World| w.chunk(0, 0).tile_entity_ids.get(&(8, 64, 8)).copied();
    for fill in fills {
        for seed in 0..4i64 {
            for hook in 0..2 {
                case(o, format_args!("brewing removal {fill:?} {seed} {hook}"));
                let mut w = fixture(true);
                w.set_block_with_notify(8, 64, 8, 117);
                w.get_block_tile_entity(8, 64, 8);
                let id = tile_id(&mut w).unwrap();
                if let Some(TileEntity::BrewingStand(b)) = w.get_block_tile_entity(8, 64, 8) {
                    for n in fill.chunks(4) {
                        b.items[n[0] as usize] = Some(ItemStack::new(n[1], n[2], n[3]));
                    }
                }
                seeds(&mut w);
                crate::world::blocks10::brewing_stand_random10().set_seed(seed);
                w.set_block_with_notify(8, 64, 8, if hook == 0 { 0 } else { 1 });
                l(o, crate::world::blocks10::brewing_stand_random10().state());
                tick_state(o, &mut w, 8, 64, 8);
                let Some(TileEntity::BrewingStand(b)) = w.tile_object(id).cloned() else { panic!("retained brewing stand") };
                for s in b.items {
                    stack(o, s);
                }
                bool8(o, w.tile_objects[&id].invalid);
            }
        }
    }
    for id in [116u8, 119] {
        for hook in 0..2 {
            case(o, format_args!("tile removal {id} {hook}"));
            use std::sync::atomic::Ordering::Relaxed;
            let mut w = fixture(true);
            crate::world::blocks10::END_PORTAL_ALLOW.store(true, Relaxed);
            w.set_block_with_notify(8, 64, 8, id);
            crate::world::blocks10::END_PORTAL_ALLOW.store(false, Relaxed);
            w.get_block_tile_entity(8, 64, 8);
            let old = tile_id(&mut w);
            seeds(&mut w);
            w.set_block_with_notify(8, 64, 8, if hook == 0 { 0 } else { 116 + 119 - id });
            tick_state(o, &mut w, 8, 64, 8);
            bool8(o, old.is_some());
            if let Some(old) = old {
                bool8(o, w.tile_objects.get(&old).map_or(true, |t| t.invalid));
            }
            let now = w.get_block_tile_entity(8, 64, 8).cloned();
            let now_id = tile_id(&mut w);
            bool8(o, now.is_some() && now_id == old);
            text(o, match now {
                None => "null",
                Some(TileEntity::EnchantmentTable(..)) => "TileEntityEnchantmentTable",
                Some(TileEntity::EndPortal) => "TileEntityEndPortal",
                Some(_) => "other",
            });
        }
    }
}
fn dispense(o: &mut Vec<u8>) {
    use crate::world::tile::TileEntity;
    let damages = [16385, 8193, 16396, 0, 32767, 16384];
    for facing in 2..6 {
        for config in 0..8usize {
            for remote in [false, true] {
                case(o, format_args!("dispense {facing} {config} {remote}"));
                let mut w = fixture(true);
                w.set_block(8, 64, 8, 23);
                w.set_block_metadata(8, 64, 8, facing);
                let mut items = Box::new([None; 9]);
                for n in 0..9 {
                    if (config >> (n % 3)) & 1 != 0 {
                        items[n] = Some(ItemStack::new(373, 1 + (n % 2) as i32, damages[(n + config) % damages.len()]));
                    }
                }
                w.set_block_tile_entity(8, 64, 8, TileEntity::Dispenser(items));
                w.dispenser_random.insert((8, 64, 8), crate::JavaRandom::new(55 + config as i64));
                w.multiplayer_world = remote;
                seeds(&mut w);
                for _ in 0..4 {
                    w.dispense_item(8, 64, 8, RandSel::World);
                    let Some(TileEntity::Dispenser(items)) = w.get_block_tile_entity(8, 64, 8).cloned() else { panic!("dispenser") };
                    for s in *items {
                        stack(o, s);
                    }
                    l(o, w.dispenser_random[&(8, 64, 8)].state());
                    tick_state(o, &mut w, 8, 64, 8);
                }
            }
        }
    }
    for config in 0..4 {
        for remote in [false, true] {
            case(o, format_args!("dispenser neighbor {config} {remote}"));
            let mut w = fixture(true);
            w.set_block(8, 64, 8, 23);
            w.set_block_metadata(8, 64, 8, 3);
            if config & 1 != 0 {
                raw(&mut w, 8, 64, 7, 76, 5);
            }
            if config & 2 != 0 {
                raw(&mut w, 8, 65, 8, 1, 0);
            }
            w.multiplayer_world = remote;
            seeds(&mut w);
            behavior::on_neighbor_block_change(&mut w, 23, 8, 64, 8, 76);
            tick_state(o, &mut w, 8, 64, 8);
        }
    }
}
fn display(o: &mut Vec<u8>) {
    for id in [8u8, 9, 10, 11, 90, 110] {
        for meta in [0, 3] {
            for config in 0..4 {
                for seed in 0..24i64 {
                    let mut w = fixture(true);
                    raw(&mut w, 8, 64, 8, id, meta);
                    if config & 1 != 0 {
                        raw(&mut w, 8, 63, 8, 0, 0);
                    }
                    if config & 2 != 0 {
                        raw(&mut w, 8, 62, 8, 1, 0);
                    }
                    let mut r = crate::JavaRandom::new(seed);
                    w.block_display_tick(id, 8, 64, 8, &mut r);
                    l(o, r.state());
                    crate::world::events::write_events(&w.drain_events(), o);
                }
            }
        }
    }
    for id in 0..=255u8 {
        let p = crate::block::props(id);
        bool8(o, p.exists);
        if p.exists {
            i(o, crate::world::pistons::mobility(id));
            i(o, crate::maps::block_color(id) as i32);
            let m = crate::block::mat(p.material);
            for v in [m.is_liquid, m.is_solid, m.blocks_movement, m.translucent_solid, m.burns, m.ground_cover, m.blocks_grass] {
                bool8(o, v);
            }
        }
    }
}
pub(super) fn ticks(o: &mut Vec<u8>) {
    egg(o);
    random_ticks(o);
    added(o);
    activation(o);
    end_collision(o);
    removal(o);
    dispense(o);
    display(o);
}

pub(super) fn colors(o: &mut Vec<u8>) {
    use crate::biome::Biome;
    use crate::block::visual181::{color_multiplier10, lookup};
    let grass: Vec<u32> = (0..65536u32).map(|k| k.wrapping_mul(997) & 0xffffff).collect();
    let foliage: Vec<u32> = (0..65536u32).map(|k| (k.wrapping_mul(311) + 0x102030) & 0xffffff).collect();
    let biomes = [Biome::Plains, Biome::Swampland, Biome::Forest, Biome::Desert];
    let biome = |x: i32, z: i32| biomes[(x.wrapping_mul(3).wrapping_add(z.wrapping_mul(5)) & 3) as usize];
    let temperature = |x: i32, y: i32, z: i32| (x.wrapping_mul(31).wrapping_add(z.wrapping_mul(17)).wrapping_add(y) & 15) as f32 / 15.0;
    let rainfall = |x: i32, z: i32| (x.wrapping_mul(7).wrapping_sub(z.wrapping_mul(11)) & 15) as f32 / 15.0;
    let mut w = fixture(true);
    for id in [1u8, 2, 8, 9, 10, 11, 18, 31, 55, 104, 105, 106, 110, 111, 115] {
        for meta in 0..16 {
            for x in [-17, -1, 0, 8, 17, 31] {
                for y in [1, 64, 127] {
                    for z in [-17, -1, 0, 9, 30] {
                        raw(&mut w, x, y, z, id, meta);
                        let metadata = w.get_block_metadata(x, y, z);
                        i(o, color_multiplier10(
                            id,
                            metadata,
                            x,
                            y,
                            z,
                            |x, y, z, leaf| {
                                let c = lookup(if leaf { &foliage } else { &grass }, temperature(x, y, z) as f64, rainfall(x, z) as f64);
                                biome(x, z).tint_color10(c) as i32
                            },
                            |x, z| biome(x, z).water_color10(),
                        ));
                    }
                }
            }
        }
    }
}

pub(super) fn tiles(o: &mut Vec<u8>) {
    use crate::world::tile::TileEntity;
    let slot = |s: i8, id: i16, count: i8, damage: i16| {
        let mut n = Compound::new();
        n.put("Slot", Tag::Byte(s));
        n.put("id", Tag::Short(id));
        n.put("Count", Tag::Byte(count));
        n.put("Damage", Tag::Short(damage));
        Tag::Compound(n)
    };
    let w = fixture(true);
    for name in [
        "Cauldron", "EnchantTable", "Airportal", "Furnace", "Chest", "Trap", "Sign", "MobSpawner", "Music", "Piston",
        "RecordPlayer", "cauldron", "Airportal\0", "EnchantmentTable",
    ] {
        for malformed in 0..7 {
            case(o, format_args!("tile {name:?} {malformed}"));
            let mut n = Compound::new();
            n.put("id", Tag::String(name.into()));
            n.put("x", Tag::Int(-123));
            n.put("y", Tag::Int(64));
            n.put("z", Tag::Int(456));
            if malformed == 1 {
                n.put("x", Tag::Byte(1));
            }
            if malformed == 2 {
                n.put("id", Tag::Int(1));
            }
            if malformed == 3 {
                n.put("Items", Tag::Int(1));
            }
            if malformed == 4 {
                n.put("BrewTime", Tag::Int(5));
            }
            if malformed >= 5 {
                let mut list = vec![slot(0, 373, 1, 8193), slot(3, 372, 2, 0), slot(2, 374, 64, 0)];
                if malformed == 6 {
                    list.push(slot(4, 1, 1, 0));
                    list.push(slot(-1, 1, 1, 0));
                }
                n.put("Items", Tag::List(10, list));
                n.put("BrewTime", Tag::Short(if malformed == 5 { 77 } else { -3 }));
            }
            match TileEntity::create_and_load181(&n) {
                Err(e) => {
                    super::round10::error(o, Err(e));
                    o.push(0);
                }
                Ok(tile) => {
                    super::round10::error(o, Ok(()));
                    o.push(tile.is_some() as u8);
                    if let Some((base, tile)) = tile {
                        super::round10::base_snapshot(o, &base, &w);
                        super::round10::nbt(o, &tile.write_nbt181(base.position));
                    }
                }
            }
        }
    }
}

pub(super) fn stack_damage(o: &mut Vec<u8>) {
    for held in 0..4 {
        for target in 0..6 {
            for amount in [0, 1, 2, 7, -1] {
                for victim in 0..2 {
                    case(o, format_args!("damage held {held} target {target} amount {amount} victim {victim}"));
                    let mut w = fixture(true);
                    let mut p = new_player(&mut w);
                    place(&mut p, 8.5, 65.0, 8.5, 20.0, 10.0);
                    let mut zombie = crate::entity::registry::create(&mut w, "Zombie").unwrap();
                    zombie.set_location_and_angles(9.5, 65.0, 8.5, 0.0, 0.0);
                    zombie.rand.set_seed(99);
                    let h = (held != 0).then(|| ItemStack::new(if held == 3 { 1 } else { 276 }, 1, 0));
                    if let Some(h) = h {
                        if held == 1 {
                            h.add_enchantment(34, 1);
                        }
                        if held == 2 {
                            h.add_enchantment(34, 3);
                        }
                    }
                    p.player_state_mut().unwrap().inventory.main[0] = h;
                    p.player_state_mut().unwrap().stat_log.clear();
                    let mut s = match target {
                        0 => h.unwrap_or(ItemStack::new(276, 1, 5)),
                        1 => ItemStack::new(310, 1, 0),
                        2 => ItemStack::new(311, 1, 527),
                        3 => ItemStack::new(257, 1, 249),
                        4 => ItemStack::new(1, 3, 0),
                        _ => ItemStack::new(261, 2, 383),
                    };
                    seeds(&mut w);
                    for _ in 0..3 {
                        if victim == 0 {
                            s.damage_item10(amount, &mut p, &mut w);
                            // The held slot and `s` are one Java object when target 0 aliases it.
                            if target == 0 && h.is_some() {
                                p.player_state_mut().unwrap().inventory.main[0] = Some(s);
                            }
                        } else {
                            s.damage_item10(amount, &mut zombie, &mut w);
                            if target == 0 && h.is_some() {
                                p.player_state_mut().unwrap().inventory.main[0] = Some(s);
                            }
                        }
                        stack(o, Some(s));
                    }
                    rngs(o, &w);
                    l(o, p.rand.state());
                    l(o, zombie.rand.state());
                    crate::world::events::write_events(&w.drain_events(), o);
                    let stats: Vec<i32> = p.player_state().unwrap().stat_log.iter().flat_map(|&(id, amount)| [id, amount]).collect();
                    i(o, stats.len() as i32);
                    for v in stats {
                        i(o, v);
                    }
                }
            }
        }
    }
}
