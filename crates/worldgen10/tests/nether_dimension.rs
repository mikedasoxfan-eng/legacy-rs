//! Provider behavior that a terrain-only comparison cannot exercise.

use worldgen::biome::{Biome, BiomeSource};
use worldgen::block;
use worldgen::nbt::{Compound, Tag};
use worldgen::overworld::CHUNK_VOLUME;
use worldgen::world::chunk::{Chunk, LightKind};
use worldgen::world::Dimension;
use worldgen::World;

#[test]
fn hell_biome_queries_and_temperature_output_are_fixed() {
    // 1.0 WorldChunkManagerHell: getTemperatures/getRainfall fill only width*height of the given array.
    let mut source = BiomeSource::hell();
    assert_eq!(source.biome_at(-12345, 67890), Biome::Hell);
    let mut biomes = Vec::new();
    source.biomes(&mut biomes, -12345, 67890, 3, 2);
    assert_eq!(biomes, vec![Biome::Hell; 6]);
    assert_eq!(source.raw_temperature(999999, -999999), 1.0);
    assert_eq!(source.raw_rainfall(999999, -999999), 0.0);
    let mut temperatures = vec![0.5; 4];
    source.temperatures(&mut temperatures, 0, 0, 1, 2);
    assert_eq!(temperatures, vec![1.0, 1.0, 0.5, 0.5]);
}

#[test]
fn nether_chunks_start_with_uninitialized_height_and_light() {
    let mut w = World::with_dimension(0, 1, Dimension::Nether);
    w.load_chunk(0, 0);
    let c = w.chunk(0, 0);
    assert!(c.blocks.iter().any(|&b| b == block::NETHERRACK));
    assert_eq!(&*c.height_map, &[0; 256]);
    assert!(c.sky_light.0.iter().all(|&v| v == 0));
    assert!(c.block_light.0.iter().all(|&v| v == 0));
    assert!(w.light_queue.is_empty());
    assert_eq!(w.light_brightness_table[0].to_bits(), 0.1f32.to_bits());
    assert_eq!(w.light_brightness_table[15].to_bits(), 1.0f32.to_bits());
    assert_eq!(World::new(0, 1).light_brightness_table[0].to_bits(), 0.0f32.to_bits());
}

#[test]
fn only_metadata_writes_skip_relighting_in_hell() {
    let mut w = World::with_dimension(0, 1, Dimension::Nether);
    let air = vec![0; CHUNK_VOLUME].into_boxed_slice().try_into().unwrap();
    w.chunks.insert((0, 0), Box::new(Chunk::new(0, 0, air)));
    assert!(w.set_block_and_metadata(4, 70, 4, block::NETHERRACK, 3));
    assert_eq!(w.chunk(0, 0).height(4, 4), 0);
    assert!(w.set_block(5, 70, 4, block::NETHERRACK));
    assert_eq!(w.chunk(0, 0).height(5, 4), 71);
    // Release 1.0 lowers the height without writing sky light in a no-sky world.
    assert!(w.set_block(5, 70, 4, block::AIR));
    assert_eq!(w.chunk(0, 0).height(5, 4), 0);
    assert_eq!(w.get_saved_light_value(LightKind::Sky, 5, 69, 4), 0);
    assert!(w.light_queue.iter().all(|u| u.kind == LightKind::Block));
    let n = w.light_queue.len();
    w.schedule_lighting_update(LightKind::Sky, 4, 64, 4, 4, 80, 4);
    w.neighbor_light_propagation_changed(LightKind::Sky, 4, 64, 4, 15);
    assert_eq!(w.light_queue.len(), n);
}

#[test]
fn missing_saved_light_arrays_are_non_null_and_keep_the_provider_flag() {
    let mut nbt = Compound::new();
    nbt.put("Blocks", Tag::ByteArray(vec![0; CHUNK_VOLUME]));
    // NBTTagCompound.getByteArray returns a non-null empty array for absent tags;
    // NibbleArray.isValid checks null, so Java does not enter the repair path.
    let hell = worldgen::storage::chunk_from_nbt_for_dimension(&nbt, Dimension::Nether).unwrap();
    let overworld = worldgen::storage::chunk_from_nbt(&nbt).unwrap();
    assert!(hell.sky_light.0.iter().all(|&v| v == 0));
    assert!(overworld.sky_light.0.is_empty());
    assert!(hell.has_no_sky10);
    assert!(!overworld.has_no_sky10);
    assert_eq!(hell.height_map, overworld.height_map);
}
