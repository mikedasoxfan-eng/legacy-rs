use worldgen::{
    storage::Storage,
    world::chunk::{idx, Chunk},
    World,
};

#[test]
fn note_pitch_and_jukebox_record_survive_region_storage() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/effects-storage-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
    let mut world = World::new(0, 1);
    world.storage = Some(Storage::open(&dir).unwrap());
    for x in -1..=1 {
        for z in -1..=1 {
            let mut chunk = Chunk::new(x, z, vec![0; 32768].into_boxed_slice().try_into().unwrap());
            chunk.loaded = true;
            chunk.populated = true;
            world.chunks.insert((x, z), Box::new(chunk));
        }
    }
    world.chunk_list = vec![(0, 0)];
    world.chunk_mut(0, 0).blocks[idx(8, 64, 8)] = 25;
    world.chunk_mut(0, 0).blocks[idx(9, 64, 8)] = 84;
    world.get_block_tile_entity(8, 64, 8);
    world.chunk_mut(0, 0).modified = false;
    for _ in 0..6 {
        world.activate_note(8, 64, 8);
    }
    assert!(
        world.chunk(0, 0).modified,
        "pitch changes must dirty the chunk"
    );
    assert!(world.insert_record(9, 64, 8, 2257));
    world.save_world(true);
    drop(world);
    let mut storage = Storage::open(&dir).unwrap();
    let chunk = storage.load_chunk(0, 0).unwrap();
    let note = match chunk.tile_entities.get(&(8, 64, 8)) {
        Some(worldgen::world::tile::TileEntity::Note(n)) => n.note,
        other => panic!("note block tile entity: {other:?}"),
    };
    let record = match chunk.tile_entities.get(&(9, 64, 8)) {
        Some(worldgen::world::tile::TileEntity::RecordPlayer { record }) => *record,
        other => panic!("jukebox tile entity: {other:?}"),
    };
    assert_eq!(note, 6);
    assert_eq!(record, 2257);
    assert_eq!(chunk.data.get(9, 64, 8), 1);
}
