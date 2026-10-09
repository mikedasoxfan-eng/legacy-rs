use worldgen::{
    container::{recipes::CraftingManager, ContainerCommand, ContainerKind, ItemStack},
    entity::Entity,
    world::{
        chunk::{idx, Chunk},
        tile::TileEntity,
    },
    JavaRandom, World,
};
fn fixture() -> (World, i32) {
    let mut w = World::new(0, 300);
    w.set_entity_determinism(100, 200, 0);
    for x in -2..=2 {
        for z in -2..=2 {
            let mut chunk = Chunk::new(x, z, vec![0; 32768].into_boxed_slice().try_into().unwrap());
            chunk.loaded = true;
            chunk.populated = true;
            w.chunks.insert((x, z), Box::new(chunk));
        }
    }
    let mut player = Entity::player(&mut w, "Player");
    player.set_location_and_angles(8.5, 64.0, 8.5, 0.0, 0.0);
    let id = player.entity_id;
    w.entity_joined_world(player);
    (w, id)
}
#[test]
fn brewing_window_uses_whole_slot_decrement_and_saves_tags() {
    let (mut w, id) = fixture();
    w.set_block_with_notify(9, 64, 9, 117);
    let potion = ItemStack::new(373, 1, 8194);
    let mut tag = worldgen::nbt::Compound::new();
    tag.put("Label", worldgen::nbt::Tag::String("kept".into()));
    potion.set_tag_checked(Some(tag));
    if let Some(TileEntity::BrewingStand(b)) = w.get_block_tile_entity(9, 64, 9) {
        b.items[0] = Some(potion);
        b.items[3] = Some(ItemStack::new(353, 12, 0));
        b.brew_time = 400;
    }
    let shot = w
        .container_command(
            id,
            ContainerCommand::OpenBlock {
                block: 117,
                position: (9, 64, 9),
            },
        )
        .unwrap();
    assert_eq!(shot.kind, ContainerKind::BrewingStand);
    assert_eq!(shot.progress[0], 400);
    w.container_command(
        id,
        ContainerCommand::Click {
            window: shot.window,
            slot: 3,
            button: 1,
            shift: false,
        },
    );
    assert_eq!(
        w.entities[&id]
            .player_state()
            .unwrap()
            .inventory
            .cursor
            .unwrap()
            .count,
        12
    );
    assert!(
        matches!(w.get_block_tile_entity(9,64,9),Some(TileEntity::BrewingStand(b)) if b.items[3].is_none())
    );
    let nbt = worldgen::storage::chunk_to_nbt(w.chunk(0, 0), 0);
    let saved = worldgen::storage::chunk_from_nbt(&nbt).unwrap();
    let Some(TileEntity::BrewingStand(b)) = saved.tile_entities.get(&(9, 64, 9)) else {
        panic!("brewing tile missing")
    };
    assert_eq!(b.brew_time, 400);
    assert!(b.items[0].unwrap().tags_equal(&potion));
}
#[test]
fn enchanting_window_spends_levels_and_retains_enchantments_after_transfer() {
    let (mut w, id) = fixture();
    w.set_block_with_notify(9, 64, 9, 116);
    for dx in -2i32..=2 {
        for dz in -2i32..=2 {
            if dx.abs() == 2 || dz.abs() == 2 {
                w.chunk_mut(0, 0).blocks[idx(9 + dx, 64, 9 + dz)] = 47;
                w.chunk_mut(0, 0).blocks[idx(9 + dx, 65, 9 + dz)] = 47;
            }
        }
    }
    let sword = ItemStack::new(276, 1, 0);
    {
        let p = w.entities.get_mut(&id).unwrap().player_state_mut().unwrap();
        p.experience.level = 100;
        p.inventory.cursor = Some(sword);
    }
    let shot = w
        .container_command(
            id,
            ContainerCommand::OpenBlock {
                block: 116,
                position: (9, 64, 9),
            },
        )
        .unwrap();
    w.container_sessions
        .get_mut(&id)
        .unwrap()
        .active
        .as_mut()
        .unwrap()
        .enchanting
        .as_mut()
        .unwrap()
        .rng = JavaRandom::new(42);
    let shot = w
        .container_command(
            id,
            ContainerCommand::Click {
                window: shot.window,
                slot: 0,
                button: 0,
                shift: false,
            },
        )
        .unwrap();
    let cost = shot.progress[2];
    assert!(cost > 0);
    assert!(w.enchant_container10(id, shot.window, 2));
    assert_eq!(
        w.entities[&id].player_state().unwrap().experience.level,
        100 - cost
    );
    let shot = w
        .container_command(
            id,
            ContainerCommand::Click {
                window: shot.window,
                slot: 0,
                button: 0,
                shift: true,
            },
        )
        .unwrap();
    assert!(shot.contents[0].is_none());
    let sword = shot
        .contents
        .iter()
        .flatten()
        .find(|s| s.id == 276)
        .unwrap();
    assert!(sword.is_enchanted());
    assert_eq!(
        sword.enchantments(),
        ItemStack::from_nbt(&sword.to_nbt()).enchantments()
    );
}
#[test]
fn eye_insertion_completes_all_rotations_and_rejects_a_missing_eye() {
    for direction in 0..4 {
        for missing in [false, true] {
            let (mut w, id) = fixture();
            let player = w.entities.remove(&id).unwrap();
            let dx = [0, -1, 0, 1];
            let dz = [1, 0, -1, 0];
            let side = (direction + 1) % 4;
            let at = |offset: i32, forward: i32| {
                (
                    8 + dx[side] * offset + dx[direction] * forward,
                    64,
                    8 + dz[side] * offset + dz[direction] * forward,
                )
            };
            let mut frames = Vec::new();
            for o in -1..=1 {
                frames.push(at(o, 0));
                frames.push(at(o, 4));
            }
            for o in [-2, 2] {
                for f in 1..=3 {
                    frames.push(at(o, f));
                }
            }
            for &(x, y, z) in &frames {
                w.chunk_mut(x >> 4, z >> 4).blocks[idx(x & 15, y, z & 15)] = 120;
                w.chunk_mut(x >> 4, z >> 4)
                    .data
                    .set(x & 15, y, z & 15, direction as i32 + 4);
            }
            w.chunk_mut(0, 0).data.set(8, 64, 8, direction as i32);
            if missing {
                let (x, y, z) = at(0, 4);
                w.chunk_mut(x >> 4, z >> 4)
                    .data
                    .set(x & 15, y, z & 15, direction as i32);
            }
            let mut eye = ItemStack::new(381, 2, 0);
            assert!(player.insert_ender_eye10(&mut w, &mut eye, 8, 64, 8));
            assert_eq!(eye.count, 1);
            for o in -1..=1 {
                for f in 1..=3 {
                    let (x, y, z) = at(o, f);
                    assert_eq!(w.get_block_id(x, y, z), if missing { 0 } else { 119 });
                }
            }
        }
    }
}
#[test]
fn food_recipes_are_shapeless_and_repair_discards_tags() {
    let c = CraftingManager::instance();
    let mut grid = [None; 9];
    grid[0] = Some(ItemStack::new(39, 1, 0));
    grid[4] = Some(ItemStack::new(281, 1, 0));
    grid[8] = Some(ItemStack::new(40, 1, 0));
    assert_eq!(c.find_matching_recipe(&grid, 3).unwrap().id, 282);
    let mut pumpkin = [None; 9];
    pumpkin[5] = Some(ItemStack::new(86, 1, 12));
    let seed = c.find_matching_recipe(&pumpkin, 3).unwrap();
    assert_eq!((seed.id, seed.count), (361, 4));
    let sword = ItemStack::new(276, 1, 1000);
    sword.add_enchantment(16, 2);
    let mut grid = [None; 9];
    grid[0] = Some(sword);
    grid[8] = Some(ItemStack::new(276, 1, 1000));
    let repaired = c.find_matching_recipe(&grid, 3).unwrap();
    assert_eq!(repaired.damage, 283);
    assert!(!repaired.has_tag());
    for (block, item, damage) in [(16, 263, 0), (73, 331, 0), (21, 351, 4)] {
        let s = c.smelting_result(block).unwrap();
        assert_eq!((s.id, s.damage), (item, damage));
    }
}

#[test]
fn splash_and_pearl_item_dispatch_preserves_guards_and_item_rng() {
    use worldgen::entity::EntityKind;
    for (item, damage, creative, remote) in [
        (373, 16384 | 8194, false, false),
        (373, 16384 | 8194, false, true),
        (368, 0, false, false),
        (368, 0, true, false),
        (373, 8194, false, false),
    ] {
        let (mut w, id) = fixture();
        w.multiplayer_world = remote;
        w.item_random.set_seed(42);
        let mut p = w.entities.remove(&id).unwrap();
        p.set_creative(creative);
        p.player_state_mut().unwrap().inventory.main[0] = Some(ItemStack::new(item, 2, damage));
        let throw = (item == 373 && damage & 16384 != 0) || (item == 368 && !creative);
        assert_eq!(p.use_item(&mut w), throw);
        assert_eq!(p.held_item().unwrap().count, if throw { 1 } else { 2 });
        let mut expected = JavaRandom::new(42);
        if throw { expected.next_float(); }
        assert_eq!(w.item_random.state(), expected.state());
        let projectiles: Vec<_> = w.entities.values().collect();
        assert_eq!(projectiles.len(), usize::from(throw && !remote));
        if let Some(projectile) = projectiles.first() {
            assert!(match &projectile.kind {
                EntityKind::Potion(state, meta) => item == 373 && *meta == damage && state.owner == Some(id),
                EntityKind::EnderPearl(state) => item == 368 && state.owner == Some(id),
                _ => false,
            });
        }
        if item == 373 && !throw {
            assert_eq!(p.player_state().unwrap().item_use_ticks, 32);
        }
    }
}

#[test]
fn dispenser_launches_splash_metadata_and_keeps_drinkable_potions_as_items() {
    use worldgen::{entity::EntityKind, world::RandSel};
    for damage in [8194, 16384 | 8194] {
        let (mut w, _) = fixture();
        w.set_block_with_notify(9, 64, 9, 23);
        if let Some(TileEntity::Dispenser(items)) = w.get_block_tile_entity(9, 64, 9) {
            items[0] = Some(ItemStack::new(373, 2, damage));
        }
        w.dispense_item(9, 64, 9, RandSel::World);
        assert_eq!(w.container_effects[0].0, if damage & 16384 != 0 {1002} else {1000});
        assert_eq!(w.container_effects[1].0, 2000);
        assert!(w.entities.values().any(|e| match &e.kind {
            EntityKind::Potion(_, meta) => damage & 16384 != 0 && *meta == damage,
            EntityKind::Item { item, .. } => damage & 16384 == 0 && item.damage as i32 == damage,
            _ => false,
        }));
    }
}

#[test]
fn pumpkin_added_consumes_snow_blocks_and_constructs_a_snowman() {
    use worldgen::entity::EntityKind;
    let (mut w, _) = fixture();
    w.set_block_with_notify(9, 62, 9, 80);
    w.set_block_with_notify(9, 63, 9, 80);
    w.set_block_with_notify(9, 64, 9, 86);
    for y in 62..=64 { assert_eq!(w.get_block_id(9, y, 9), 0); }
    let snowman = w.entities.values().find(|e| matches!(e.kind, EntityKind::Snowman)).unwrap();
    assert_eq!(snowman.pos, worldgen::entity::Vec3D::new(9.5, 64.0 - 1.95, 9.5));
}

#[test]
fn unbreaking_uses_the_held_item_and_one_world_draw_for_the_entire_damage_call() {
    let (mut w, id) = fixture();
    let mut p = w.entities.remove(&id).unwrap();
    let sword = ItemStack::new(276, 1, 0);
    sword.add_enchantment(34, 3);
    p.player_state_mut().unwrap().inventory.main[0] = Some(sword);
    let mut armor = ItemStack::new(310, 1, 0);
    w.rand.set_seed(42); // nextInt(4) == 2: skip all 7 damage, including to armor.
    let mut expected = JavaRandom::new(42);
    assert_eq!(expected.next_int(4), 2);
    assert!(!armor.damage_item10(7, &mut p, &mut w));
    assert_eq!(armor.damage, 0);
    assert_eq!(w.rand.state(), expected.state());
    armor.damage_item10(-3, &mut p, &mut w);
    assert_eq!(armor.damage, -3);
    assert_eq!(w.rand.state(), expected.state());
    p.player_state_mut().unwrap().inventory.main[0] = None;
    armor.damage_item10(7, &mut p, &mut w);
    assert_eq!(armor.damage, 4);
    assert_eq!(w.rand.state(), expected.state());
}

#[test]
fn tag_references_share_mutation_but_copy_and_split_clone_compounds() {
    use std::sync::{Arc, Mutex};
    use worldgen::nbt::{Compound, Tag};
    let tag = Arc::new(Mutex::new(Compound::new()));
    let mut a = ItemStack::new(276, 2, 0);
    let b = ItemStack::new(267, 1, 0);
    a.set_tag_handle(Some(tag.clone()));
    b.set_tag_handle(Some(tag.clone()));
    let copy = a.copy_stack181();
    let split = a.split(1);
    a.add_enchantment(16, 2);
    assert_eq!(b.enchantments(), a.enchantments());
    assert_eq!(tag.lock().unwrap().list("ench").len(), 1);
    assert!(copy.enchantments().is_empty());
    assert!(split.enchantments().is_empty());
    tag.lock().unwrap().put("NaN", Tag::Float(f32::NAN));
    assert!(a.tags_equal(&b)); // Java Map.equals short-circuits shared values.
}

#[test]
fn enchantment_level_stops_before_later_malformed_entries() {
    use worldgen::nbt::{Compound, Tag};
    let stack = ItemStack::new(276, 1, 0);
    let mut enchantment = Compound::new();
    enchantment.put("id", Tag::Short(16));
    enchantment.put("lvl", Tag::Short(3));
    let mut tag = Compound::new();
    tag.put("ench", Tag::List(10, vec![Tag::Compound(enchantment), Tag::Int(7)]));
    stack.set_tag(Some(tag));
    assert_eq!(worldgen::enchantment::level(16, Some(stack)), 3);
    assert!(std::panic::catch_unwind(|| worldgen::enchantment::level(20, Some(stack))).is_err());
    let mut wrong = Compound::new();
    wrong.put("ench", Tag::Int(7));
    stack.set_tag(Some(wrong));
    assert!(std::panic::catch_unwind(|| stack.enchantments()).is_err());
}

#[test]
fn enchantment_slot_recomputes_twice_on_put_and_rejects_incoming_shift_clicks() {
    use worldgen::container::{Container, InventoryPlayer};
    let mut c = Container::new(ContainerKind::Enchantment, 1);
    let mut inventory = InventoryPlayer::default();
    c.bookshelves = 30;
    c.enchanting.as_mut().unwrap().rng.set_seed(42);
    let sword = ItemStack::new(276, 1, 0);
    c.put_stack(&mut inventory, 0, Some(sword));
    let mut expected = worldgen::container::enchanting::Enchanting::new(42);
    expected.item = Some(sword);
    expected.changed(false, 30);
    expected.changed(false, 30);
    assert_eq!(c.enchanting.as_ref().unwrap().rng.state(), expected.rng.state());
    assert_eq!(c.enchanting.as_ref().unwrap().name_seed, expected.name_seed);
    assert_eq!(c.progress, expected.costs);
    c.put_stack(&mut inventory, 0, None);
    inventory.main[9] = Some(sword);
    assert!(c.slot_click(&mut inventory, 1, 0, true).is_none());
    assert!(c.tile[0].is_none());
    assert!(inventory.main[9].unwrap().same_object(sword));
}

#[test]
fn colored_item_retains_configurable_name_array_and_full_damage_callbacks() {
    use std::sync::{Arc, Mutex};
    let mut item = worldgen::item10::ColoredItem::new(Some(31), true);
    let names = Arc::new(Mutex::new(vec![Some("custom".to_owned()), None]));
    item.set_names(Some(names.clone()));
    let stack = ItemStack::new(31, 1, 0);
    assert_eq!(item.name(Some(&stack), || Some("tile.tallgrass".into())), Some("tile.tallgrass.custom".into()));
    names.lock().unwrap()[0] = Some("changed".into());
    assert_eq!(item.name(Some(&stack), || None), Some("null.changed".into()));
    assert_eq!(item.name(Some(&stack), || {
        names.lock().unwrap()[0] = Some("during-base".into());
        Some("base".into())
    }), Some("base.during-base".into()));
    assert_eq!(item.color(-999, |block, damage| { assert_eq!((block, damage), (31, -999)); 123 }), 123);
    item.set_names(None);
    assert_eq!(item.name(None, || Some("base".into())), Some("base".into()));
}

#[test]
fn potion_cache_retains_mutable_lists_and_drinking_copies_the_current_effects() {
    use std::sync::Arc;
    use worldgen::item10::PotionItem;
    let mut item = PotionItem::default();
    assert!(item.effects(0).is_none());
    let effects = item.effects(8194).unwrap();
    assert!(Arc::ptr_eq(&effects, &item.effects(8194).unwrap()));
    effects.lock().unwrap()[0].amplifier = 7;
    let mut applied = Vec::new();
    let result = item.drink(ItemStack::new(373, 1, 8194), false,
        |effect| applied.push(effect), |_| panic!("last potion replaces the stack"));
    assert_eq!(result.id, 374);
    assert_eq!(applied[0].amplifier, 7);
    effects.lock().unwrap().clear();
    assert!(item.effects(8194).unwrap().lock().unwrap().is_empty());
    assert_eq!(applied[0].amplifier, 7);
    // Java String.trim removes ASCII controls, while retaining NBSP.
    assert_eq!(item.display_name(0, "unused", |_| "\u{20}\u{a0}water\u{a0}\n".into()), "\u{a0}water\u{a0}");
}

#[test]
fn mushroom_spread_uses_release_probability_and_preserves_failed_trials() {
    use worldgen::world::{behavior, RandSel};
    let (mut w, _) = fixture();
    for x in 3..=13 { for z in 3..=13 {
        w.chunk_mut(0, 0).blocks[idx(x, 63, z)] = 110;
    }}
    w.chunk_mut(0, 0).blocks[idx(8, 64, 8)] = 39;
    w.rand.set_seed(19); // nextInt(25) == 0, nextInt(100) == 25.
    behavior::update_tick(&mut w, 39, 8, 64, 8, RandSel::World);
    // All five proposals run. The final proposal hits the soil at (7,63,6).
    assert_eq!(w.rand.state(), 55_509_055_100_821);
    assert_eq!(w.get_block_id(7, 63, 6), 110);
    w.height10 = 64;
    assert!(!behavior::flower_can_stay(&mut w, 39, 8, 64, 8));
    w.height10 = 65;
    assert!(behavior::flower_can_stay(&mut w, 39, 8, 64, 8));
}

#[test]
fn giant_mushroom_rejects_stone_soil_and_restores_the_original_metadata() {
    use worldgen::world::{plants181, RandSel};
    let (mut w, _) = fixture();
    w.chunk_mut(0, 0).blocks[idx(8, 63, 8)] = 1;
    w.chunk_mut(0, 0).blocks[idx(8, 64, 8)] = 39;
    w.chunk_mut(0, 0).data.set(8, 64, 8, 3);
    w.rand.set_seed(42);
    assert!(!plants181::grow_mushroom(&mut w, 8, 64, 8, RandSel::World));
    assert_eq!((w.get_block_id(8,64,8),w.get_block_metadata(8,64,8)), (39,3));
    let mut expected = JavaRandom::new(42);
    expected.next_int(2);
    expected.next_int(3);
    assert_eq!(w.rand.state(), expected.state());
    w.chunk_mut(0, 0).blocks[idx(8, 63, 8)] = 110;
    assert!(plants181::grow_mushroom(&mut w, 8, 64, 8, RandSel::World));
    assert_eq!(w.get_block_id(8,63,8), 3);
    assert_eq!(w.get_block_id(8,64,8), 99);
}

#[test]
fn neighbor_brightness_includes_double_slabs_and_nether_stairs() {
    use worldgen::world::chunk::LightKind;
    let (mut w, _) = fixture();
    w.chunk_mut(0,0).block_light.set(8,64,8,2);
    w.chunk_mut(0,0).block_light.set(8,65,8,11);
    for (block, light) in [(43,11), (114,11), (1,2)] {
        w.chunk_mut(0,0).blocks[idx(8,64,8)] = block;
        assert_eq!(w.saved_light_with_neighbors181(LightKind::Block,8,64,8), light);
    }
}
