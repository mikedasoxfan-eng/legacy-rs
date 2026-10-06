mod support;
use worldgen::{
    entity::{
        player::{ItemStack, PlayerEvent, PlayerInput, PlayerTarget, PlayerTickInput},
        Entity, MovingObjectPosition, Vec3D,
    },
    storage::Storage,
    world::chunk::{idx, Chunk},
    World,
};

fn flat_world() -> World {
    let mut w = World::new(0, 1);
    w.set_entity_determinism(100, 200, 0);
    w.set_allowed_mob_spawns(false, false);
    w.spawn = (0, 63, 0);
    for x in -12..=12 {
        for z in -12..=12 {
            let mut c = Chunk::new(x, z, vec![0; 32768].into_boxed_slice().try_into().unwrap());
            c.loaded = true;
            c.populated = true;
            c.sky_light.0.fill(255);
            c.height_map.fill(64);
            for bx in 0..16 {
                for bz in 0..16 {
                    c.blocks[idx(bx, 63, bz)] = 1;
                }
            }
            w.chunks.insert((x, z), Box::new(c));
        }
    }
    w
}

fn canonical(nbt: &worldgen::nbt::Compound) -> Vec<u8> {
    let mut bytes = Vec::new();
    worldgen::nbt::write_root_canonical(&mut bytes, nbt).unwrap();
    bytes
}

#[test]
fn public_tick_moves_player_and_exposes_hud_and_container_requests() {
    let mut w = flat_world();
    let id = w.spawn_player("Alex");
    assert_eq!(w.entities[&id].yaw, -180.0);
    for tick in 1..=40 {
        w.player_tick(
            id,
            PlayerTickInput {
                movement: PlayerInput {
                    forward: true,
                    ..Default::default()
                },
                ..Default::default()
            },
            tick,
        );
    }
    let p = &w.entities[&id];
    assert_eq!(p.ticks_existed, 40);
    assert!(p.pos.z < -3.0);
    assert_eq!(w.players[0], (p.pos.x, p.pos.y, p.pos.z));
    assert_eq!(w.world_time, 40);
    let hud = w.player_hud(id, 0.5).unwrap();
    assert_eq!((hud.health, hud.air, hud.armor), (20, 300, 0));
    assert!(hud.camera_yaw > 0.0 && hud.distance_walked > 0.0);
    w.chunk_mut(0, 0).blocks[idx(8, 64, 8)] = 58;
    let hit = MovingObjectPosition::Block {
        x: 8,
        y: 64,
        z: 8,
        side: 1,
        hit: Vec3D::new(8.0, 65.0, 8.0),
    };
    w.apply_player_tick_input(
        id,
        PlayerTickInput {
            target: PlayerTarget::ObjectMouseOver(Some(hit)),
            right_pressed: true,
            ..Default::default()
        },
        41,
    );
    let events: Vec<_> = w
        .take_player_events(id)
        .into_iter()
        .filter(|e| !matches!(e, PlayerEvent::Stat { .. }))
        .collect();
    assert!(matches!(
        events.as_slice(),
        [PlayerEvent::OpenContainer {
            block: 58,
            position: (8, 64, 8)
        }]
    ));
    assert!(w.take_player_events(id).is_empty());
}

#[test]
fn level_dat_roundtrip_keeps_player_out_of_chunk_entities() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/player-storage-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    let mut w = flat_world();
    w.storage = Some(Storage::open(&dir).unwrap());
    let id = w.spawn_player("Alex");
    let p = w.entities.get_mut(&id).unwrap();
    p.living.health = 7;
    let s = p.player_state_mut().unwrap();
    s.inventory.main[35] = Some(ItemStack::new(35, 63, 14));
    s.inventory.armor[2] = Some(ItemStack::new(312, 1, 103));
    s.inventory.current_item = 8;
    s.spawn = Some((10, 64, -5));
    s.score = 123;
    let expected = p.player_nbt().unwrap();
    assert!(p.to_nbt().is_none());
    w.chunk_list = vec![(0, 0)];
    w.chunk_mut(0, 0).modified = true;
    w.save_world_indirectly();
    drop(w);
    let mut storage = Storage::open(&dir).unwrap();
    let info = storage.load_level().unwrap();
    assert_eq!(
        support::canonical(info.player.as_ref().unwrap()),
        support::canonical(&expected)
    );
    assert!(storage.load_chunk(0, 0).unwrap().raw_entities.is_empty());
    let mut w = flat_world();
    w.info = info;
    let id = w.spawn_player("Alex");
    assert_eq!(canonical(&w.entities[&id].player_nbt().unwrap()), canonical(&expected));
    assert_eq!(
        w.entities[&id]
            .player_state()
            .unwrap()
            .inventory
            .current_item,
        0
    );
    assert!(w.info.player.is_none());
    assert_eq!(w.player_entities, vec![Some(id)]);
}

#[test]
fn dead_player_remains_available_to_hud_and_level_save() {
    let mut w = flat_world();
    let mut p = Entity::player(&mut w, "Player");
    p.living.health = 0;
    p.living.death_time = 19;
    let id = p.entity_id;
    w.entity_joined_world(p);
    for _ in 0..4 {
        w.update_entities();
    }
    assert!(w.entities[&id].dead);
    assert!(!w.loaded_entity_list.contains(&id));
    assert_eq!(w.player_hud(id, 1.0).unwrap().health, 0);
    w.save_level();
    assert_eq!(w.info.player.as_ref().unwrap().short("Health"), 0);
}
