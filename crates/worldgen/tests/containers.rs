use worldgen::{
    World,
    container::{ContainerCommand, ContainerKind, ItemStack},
    entity::Entity,
    world::{
        chunk::{Chunk, idx},
        tile::TileEntity,
    },
};
fn fixture() -> (World, i32) {
    let mut w = World::new(0, 300);
    w.set_entity_determinism(100, 200, 0);
    for x in -3..=3 {
        for z in -3..=3 {
            let mut c = Chunk::new(x, z, vec![0; 32768].into_boxed_slice().try_into().unwrap());
            c.loaded = true;
            c.populated = true;
            for bx in 0..16 {
                for bz in 0..16 {
                    c.blocks[idx(bx, 63, bz)] = 1;
                }
            }
            w.chunks.insert((x, z), Box::new(c));
        }
    }
    let mut p = Entity::player(&mut w, "Player");
    p.set_location_and_angles(0.5, 64.0, 0.5, 0.0, 0.0);
    let id = p.entity_id;
    w.entity_joined_world(p);
    (w, id)
}
#[test]
fn double_chest_order_and_window_messages_update_real_inventory() {
    let (mut w, id) = fixture();
    w.set_block_with_notify(1, 64, 1, 54);
    w.set_block_with_notify(2, 64, 1, 54);
    if let Some(TileEntity::Chest(slots)) = w.get_block_tile_entity(1, 64, 1) {
        slots[0] = Some(ItemStack::new(264, 32, 0).into());
    }
    let snapshot = w
        .container_command(
            id,
            ContainerCommand::OpenBlock {
                block: 54,
                position: (2, 64, 1),
            },
        )
        .unwrap();
    assert_eq!(snapshot.kind, ContainerKind::Chest { rows: 6 });
    assert_eq!(snapshot.contents[0], Some(ItemStack::new(264, 32, 0)));
    let window = snapshot.window;
    let stale = w
        .container_command(
            id,
            ContainerCommand::Click {
                window: window + 1,
                slot: 0,
                button: 0,
                shift: true,
            },
        )
        .unwrap();
    assert_eq!(stale.contents[0], snapshot.contents[0]);
    let moved = w
        .container_command(
            id,
            ContainerCommand::Click {
                window,
                slot: 0,
                button: 0,
                shift: true,
            },
        )
        .unwrap();
    assert!(moved.contents[0].is_none());
    assert_eq!(moved.contents[89], snapshot.contents[0]);
    assert!(
        matches!(w.get_block_tile_entity(1,64,1),Some(TileEntity::Chest(items)) if items[0].is_none())
    );
    assert!(w.container_usable(id));
    w.entities
        .get_mut(&id)
        .unwrap()
        .set_position(40.0, 64.0, 40.0);
    assert!(!w.container_usable(id));
    assert!(
        w.container_command(id, ContainerCommand::Close { window })
            .is_none()
    );
    assert!(w.container_snapshot(id).is_none());
}
#[test]
fn close_drops_cursor_and_crafting_inputs_as_entities() {
    let (mut w, id) = fixture();
    let snapshot = w
        .container_command(id, ContainerCommand::OpenInventory)
        .unwrap();
    let session = w.container_sessions.get_mut(&id).unwrap();
    let c = session.active.as_mut().unwrap();
    c.matrix[0] = Some(ItemStack::new(5, 4, 0));
    c.matrix_changed();
    w.entities
        .get_mut(&id)
        .unwrap()
        .player_state_mut()
        .unwrap()
        .inventory
        .cursor = Some(ItemStack::new(264, 3, 0));
    w.container_command(
        id,
        ContainerCommand::Close {
            window: snapshot.window,
        },
    );
    assert!(
        w.entities[&id]
            .player_state()
            .unwrap()
            .inventory
            .cursor
            .is_none()
    );
    let mut items: Vec<_> = w
        .entities
        .values()
        .filter_map(|e| {
            if let worldgen::entity::EntityKind::Item { item, .. } = e.kind {
                Some((item.id, item.count))
            } else {
                None
            }
        })
        .collect();
    items.sort();
    assert_eq!(items, vec![(5, 4), (264, 3)]);
}
#[test]
fn machine_nbt_roundtrip_preserves_signed_fields_and_recomputes_fuel() {
    use worldgen::container::tiles::Furnace;
    let f = Furnace {
        items: [
            Some(ItemStack::new(15, 12, 0)),
            Some(ItemStack::new(263, 3, 1)),
            Some(ItemStack::new(265, 8, 0)),
        ],
        burn: 777,
        cook: 103,
        current_fuel: 99,
    };
    let restored = Furnace::from_nbt(&f.to_nbt(8, 64, 8));
    assert_eq!(restored.items, f.items);
    assert_eq!(
        (restored.burn, restored.cook, restored.current_fuel),
        (777, 103, 1600)
    );
}

#[test]
fn machine_tiles_survive_chunk_serialization() {
    use worldgen::{
        container::tiles::Furnace,
        storage::{chunk_from_nbt, chunk_to_nbt},
    };
    let (mut w, _) = fixture();
    w.set_block_with_notify(2, 64, 2, 62);
    w.set_block_with_notify(4, 64, 2, 23);
    let furnace = Furnace {
        items: [
            Some(ItemStack::new(15, 12, 0)),
            Some(ItemStack::new(263, 3, 1)),
            Some(ItemStack::new(265, 8, 0)),
        ],
        burn: 777,
        cook: 103,
        current_fuel: 99,
    };
    w.set_block_tile_entity(2, 64, 2, TileEntity::Furnace(furnace.clone()));
    let mut items = Box::new([None; 9]);
    items[8] = Some(ItemStack::new(351, -1, -32768));
    w.set_block_tile_entity(4, 64, 2, TileEntity::Dispenser(items.clone()));
    let chunk = chunk_from_nbt(&chunk_to_nbt(w.chunks[&(0, 0)].as_ref(), 123)).unwrap();
    let restored = match &chunk.tile_entities[&(2, 64, 2)] {
        TileEntity::Furnace(f) => f,
        _ => panic!("furnace not restored"),
    };
    assert_eq!(restored.items, furnace.items);
    assert_eq!(
        (restored.burn, restored.cook, restored.current_fuel),
        (777, 103, 1600)
    );
    assert_eq!(
        chunk.tile_entities[&(4, 64, 2)],
        TileEntity::Dispenser(items)
    );
    assert!(chunk.raw_tile_entities.is_empty());
}

#[test]
fn leaving_workbench_range_closes_window_and_drops_matrix() {
    use worldgen::{container::ContainerEvent, entity::player::PlayerEvent};
    let (mut w, id) = fixture();
    w.set_block_with_notify(1, 64, 1, 58);
    let snapshot = w
        .container_command(
            id,
            ContainerCommand::OpenBlock {
                block: 58,
                position: (1, 64, 1),
            },
        )
        .unwrap();
    let c = w
        .container_sessions
        .get_mut(&id)
        .unwrap()
        .active
        .as_mut()
        .unwrap();
    c.matrix[0] = Some(ItemStack::new(5, 4, 0));
    c.matrix_changed();
    w.entities
        .get_mut(&id)
        .unwrap()
        .set_position(20.5, 64.0, 0.5);
    w.update_entities();
    assert!(w.container_snapshot(id).is_none());
    assert!(
        w.take_player_events(id)
            .iter().any(|event|matches!(event,PlayerEvent::Container(ContainerEvent::Closed{window}) if *window==snapshot.window))
    );
    assert!(w.entities.values().any(|e|matches!(e.kind,worldgen::entity::EntityKind::Item{item,..} if item.id==5&&item.count==4)));
}

#[test]
fn furnace_snapshot_reproduces_progress_read_side_effect() {
    use worldgen::container::tiles::Furnace;
    let (mut w, id) = fixture();
    w.set_block_with_notify(1, 64, 1, 62);
    w.set_block_tile_entity(
        1,
        64,
        1,
        TileEntity::Furnace(Furnace {
            burn: 100,
            cook: 80,
            ..Default::default()
        }),
    );
    let snapshot = w
        .container_command(
            id,
            ContainerCommand::OpenBlock {
                block: 62,
                position: (1, 64, 1),
            },
        )
        .unwrap();
    assert_eq!(snapshot.progress, [80, 100, 200]);
    assert!(
        matches!(w.get_block_tile_entity(1,64,1),Some(TileEntity::Furnace(f)) if f.current_fuel==200)
    );
}

#[test]
fn chest_nbt_byte_counts_are_signed_in_container_views() {
    let (mut w, id) = fixture();
    w.set_block_with_notify(1, 64, 1, 54);
    if let Some(TileEntity::Chest(items)) = w.get_block_tile_entity(1, 64, 1) {
        items[0] = Some(ItemStack::new(264, -1, 0).into());
    }
    let snapshot = w
        .container_command(
            id,
            ContainerCommand::OpenBlock {
                block: 54,
                position: (1, 64, 1),
            },
        )
        .unwrap();
    assert_eq!(snapshot.contents[0], Some(ItemStack::new(264, -1, 0)));
}
