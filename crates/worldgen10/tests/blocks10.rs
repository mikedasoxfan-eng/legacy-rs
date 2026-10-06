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
