use worldgen::{
    entity::{Entity, EntityKind},
    nbt::{Compound, Tag},
    storage::Storage,
    world::chunk::Chunk,
    World,
};

fn empty_chunk(x: i32, z: i32) -> Chunk {
    let mut c = Chunk::new(x, z, vec![0; 32768].into_boxed_slice().try_into().unwrap());
    c.populated = true;
    c.loaded = true;
    c.sky_light.0.fill(255);
    c.height_map.fill(64);
    c
}

#[test]
fn region_roundtrip_simulates_supported_entities_and_preserves_unknown_tags() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/entity-storage-test")
        .join(format!(
            "{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    let mut w = World::new(0, 1);
    w.set_entity_determinism(100, 200, 0);
    w.storage = Some(Storage::open(&dir).unwrap());
    w.chunks.insert((0, 0), Box::new(empty_chunk(0, 0)));
    w.chunk_list.push((0, 0));
    let mut unsupported = Compound::new();
    unsupported.put("id", Tag::String("Painting".into()));
    unsupported.put("Custom", Tag::ByteArray(vec![0, 255, 17]));
    w.chunk_mut(0, 0)
        .raw_entities
        .push(Tag::Compound(unsupported.clone()));
    let mut pig = Entity::living(&mut w, "Pig").unwrap();
    pig.set_location_and_angles(3.25, 64.0, 4.5, 123.0, -7.0);
    pig.kind = EntityKind::Pig { saddled: true };
    pig.living.health = 6;
    let expected = pig.to_nbt().unwrap();
    assert!(w.entity_joined_world(pig));
    w.world_time = 120;
    assert!(w.save_chunks(true));
    drop(w);
    let mut w = World::new(0, 1);
    w.set_entity_determinism(100, 200, 0);
    w.storage = Some(Storage::open(&dir).unwrap());
    w.load_chunk(0, 0);
    assert_eq!(w.loaded_entity_list.len(), 1);
    let id = w.loaded_entity_list[0];
    assert_eq!(w.entities[&id].to_nbt().unwrap(), expected);
    assert_eq!(w.chunk(0, 0).entity_lists[4], vec![id]);
    assert_eq!(
        w.chunk(0, 0).raw_entities,
        vec![Tag::Compound(unsupported.clone())]
    );
    w.world_time = 240;
    w.set_block_with_notify(8, 64, 8, 52);
    w.spawner_animations.insert((8, 64, 8), (123.0, 122.0));
    w.server = Some(Default::default());
    w.spawn = (1000, 64, 1000);
    w.drop_chunk(0, 0);
    w.unload_oldest_chunks();
    w.update_entities();
    assert!(w.entities.is_empty());
    assert!(w.loaded_tile_entities.is_empty());
    assert!(w.spawner_animations.is_empty());
    w.server_load_chunk(0, 0);
    assert_eq!(w.loaded_entity_list.len(), 1);
    assert!(w.loaded_tile_entities.contains(&(8, 64, 8)));
    assert!(w.spawner_animations.is_empty());
    assert!(w.save_chunks(true));
    drop(w);
    let mut s = Storage::open(&dir).unwrap();
    let saved = s.load_chunk(0, 0).unwrap();
    assert_eq!(saved.raw_entities.len(), 2);
    assert!(saved.raw_entities.contains(&Tag::Compound(unsupported)));
    assert!(saved.raw_entities.contains(&Tag::Compound(expected)));
}

#[test]
fn force_gate_refiling_and_unloaded_removal_keep_chunk_membership_consistent() {
    let mut w = World::new(0, 1);
    w.set_entity_determinism(100, 200, 0);
    w.chunks.insert((0, 0), Box::new(empty_chunk(0, 0)));
    w.chunks.insert((1, 0), Box::new(empty_chunk(1, 0)));
    let mut pig = Entity::living(&mut w, "Pig").unwrap();
    pig.set_position(1.0, 64.0, 1.0);
    let id = pig.entity_id;
    assert!(w.entity_joined_world(pig));
    w.update_entities();
    assert_eq!(w.entities[&id].ticks_existed, 0); // Missing chunks within 32 blocks.
    let mut pig = w.entities.remove(&id).unwrap();
    pig.set_position(17.0, 130.0, 1.0);
    w.update_entity_with_optional_force(&mut pig, false);
    assert_eq!((pig.chunk_x, pig.chunk_y, pig.chunk_z), (1, 7, 0));
    assert!(w.chunk(0, 0).entity_lists[4].is_empty());
    assert_eq!(w.chunk(1, 0).entity_lists[7], vec![id]);
    w.entities.insert(id, pig);
    w.unloaded_entity_list.push(id);
    w.update_entities();
    assert!(w.loaded_entity_list.is_empty());
    assert!(w.entities.is_empty());
    assert!(w.chunk(1, 0).entity_lists[7].is_empty());
    let mut rejected = Entity::living(&mut w, "Cow").unwrap();
    rejected.set_position(1000.0, 64.0, 0.0);
    assert!(!w.entity_joined_world(rejected));
    assert_eq!(w.chunks.len(), 2);
}
