# Beta 1.8.1 terrain worker report

This is a completed terrain stage, **not a complete implementation of the task**. The approximately 75-minute allocation was used for terrain, biome layers, new features, population decoration/spawning, and shared structure foundations. Concrete village/stronghold/mineshaft generation remains absent.

## Verification evidence

All checks execute Mojang's original deobfuscated bytecode on Java 8u51. New modes are separate Java classes; Oracle.java and the b1.7.3 crates/oracle/check scripts were not edited.

| Key | Exact check |
| --- | --- |
| R | Original verify181 raw-terrain section: seven cases, 649 chunks, including structures-enabled Java raw calls. Terrain-only mode independently matches the same seven cases. Structure blocks are placed during population, so these matches do not prove structure generation. |
| N | Original verify181 raw-Nether section: six cases, 505 chunks; blocks, metadata and biome/climate output match. |
| L | terrain/layers: all four GenLayer endpoints; five seeds including both signed-long extremes, negative/non-square queries and Far Lands. |
| B | biomes181: mixed query/cache sequence, exact float bits, padded IntCache biome search, viability and RNG states, five seeds. |
| C | biomecache181: expiry, backwards-clock cleanup, cleanup gate; Hell/Sky fixed maps/climate/search and RNG, five seeds. |
| U | terrainutils181: all biome properties and three spawn lists, weighted collection/array selection and overflow, 512 bounding-box cases and 256 selector/piece-limit/key/treasure cases per seed, five seeds. |
| S | structurestarts181: 64 synthetic component lists per seed through the original StructureStart methods; five seeds. These prove the base Start class, not concrete structures. |
| K | terrain/sky: six raw Sky cases, 141 chunks, including positive and negative Far Lands. |
| F | features181: oak/birch/two taigas/big oak/swamp/both giant mushrooms/clay/sand/gravel/ore, seed 42 with dry/water/obstruction/height-limit fixtures. Water/lava lakes additionally checked on a fifth fixture. Output includes result, blocks, metadata and both RNG states. |
| D | features181 decorate0..9: dry and water fixtures. 18 match; two Taiga fixtures fail as documented. |
| A | features181 animals0..9: seeds 4096, 4097, 4100, 4105. All 40 match, including actual nonempty groups, RNG states, entity type/position/yaw and sheep color. |

Spawn-only checks also match for five seeds: (56,64,157), (-244,64,-428), (184,64,256), (112,64,252), (91,64,210). The existing populated preload, grid, savepreload/readsave and region-file sections are **not claimed exact** and were not run in full; they require the missing structure population and world/block work.

`cargo build --release -p worldgen181 -p client` passes. Client warnings are inherited dead-code warnings. Full verify181 (~1 hour) is reserved for the integrator. The new `tools/verify_terrain181.sh --built` runs the new checks under the common verification lock and intentionally exits nonzero on the documented mismatches. Its invocation is in the delimited terrain block at the end of verify181.sh; the final verdict/exit was moved into that block so the appended checks execute.

The final complete run of the new script finished with **154 EXACT checks and 3 differing cases**: the two Taiga decorator fixtures and the successful dungeon chest fixture. It exited 1 as intended. Evidence is in `target/verify-terrain181-final.log`; the successful release build is in `target/terrain-final-build.log`. `bash -n` and `git diff --check` also pass.

## Integration details and limitations

- Population now uses sea level 63, the literal decorator order (including the duplicated sand and yellow-flower bugs), changed ore Y offsets, and generation animal spawning. Java's two RNGs and the rectangular-region width bug are preserved.
- Minimal block dependencies added locally: ids 99/100 properties, id106 vine material/properties and the support callback needed by swamp generation; plain lifecycle dispatch accepts these ids. Full block behavior remains owned by the blocks worker.
- World/client weather helpers now read the new biome rain/snow flags. Original b1.8.1 registers no snowy biome, including Taiga. This is a small caller migration needed after changing the biome ids.
- The original biome-cache cleanup call is wired at the start of World.tick, before weather updates.
- Taiga decorator failures have identical RNG output and tree blocks; only grass placement differs. The old world lighting implementation is the suspected dependency, not a proven diagnosis. No placement approximation was introduced.
- Successful dungeon fixture: block bytes and RNG match; two chest metadata bytes differ. Chest orientation must be ported by the block worker. The harness does not yet serialize dungeon inventories/spawner tile data.
- Structure utility types are usable foundations, but no village/stronghold/mineshaft start registration, piece factories, real pieces, chest generation or population calls exist. The requested oracle dumping *real* starts/pieces/generated structure blocks remains to be added. StructureComponent is not presented as implemented merely because a trait exists.
- Sky raw generation is public and exact; it is not yet a selectable World dimension and Sky population remains unported.
- Compatibility aliases for old biome names remain to keep unported callers compiling. Ambient entity spawning still needs migration to the new spawn lists by the entities worker.

## Per-class status

“Done” below means the implemented class behavior has the indicated exact oracle coverage. “Not done” includes partial work and unimplemented integration; passing a narrower fixture is not treated as proof of a whole unfinished provider.

| Class | Status | Evidence / remaining work |
| --- | --- | --- |
| ChunkProviderGenerate | Not done | Raw density/surface/caves/ravines exact (R). New decoration and animal spawning implemented. Structure starts/pieces/calls still absent; complete population is not exact. |
| ChunkProviderSky | Not done | Raw floating-island density/surface/caves exact (K). Public SkyGenerator added; Sky dimension integration and its old population sequence remain. |
| BiomeDecorator | Not done | 18/20 isolated seed-42 decorator fixtures exact (D). Taiga misses four grass blocks on dry fixture and one on water fixture; RNG states match. Lighting-dependent placement unresolved. |
| WorldChunkManager | Done | B + C: query order, climate floats, biome search, viability, cached queries, fixed managers |
| MapGenRavine | Done | R: all original raw terrain cases, including both Far Lands directions |
| MapGenCaves | Done | R: all original raw terrain cases, including both Far Lands directions |
| WorldGenBigMushroom | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| WorldGenSwamp | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| BiomeGenBase | Not done | IDs/heights/climate/top/filler/colors/weather flags/all spawn lists exact (U). Decoration wired. Sky color override and JVM-global retained generator lifecycle not fully integrated. |
| GenLayer | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| WorldGenTaiga2 | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| GenLayerZoom | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| ChunkProvider | Not done | Existing lifecycle retained; changed provider/cache lifecycle not audited or proven exact. |
| WorldGenTaiga1 | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| WorldGenForest | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| WorldGenTrees | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| WorldGenBigTree | Not done | Four standalone fixtures and Forest decoration exact (F/D). Retained first height limit is kept per biome per World; original JVM-wide biome generator identity across Worlds remains. |
| GenLayerZoomVoronoi | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| WeightedRandom | Done | U + A: totals, collection/array selection, explicit totals, zero/negative weights, overflow, biome entries and population |
| ChunkProviderHell | Done | N: all six original raw Nether cases (505 chunks) |
| BiomeCache | Done | B + C: cache construction/query order, expiry, clock rollback, cleanup gate |
| IntCache | Done | L + B: original allocation padding and stale tails, including reservoir search/spawn |
| GenLayerSmoothZoom | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| GenLayerZoomFuzzy | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| GenLayerSmooth | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| WorldChunkManagerHell | Done | C + N: fixed Hell/Sky climate, maps, positive/negative search membership, viability, RNG state |
| GenLayerIsland | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| GenLayerRiver | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| WorldGenSand | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| WorldGenClay | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| MapGenCavesHell | Done | N: all six original raw Nether cases (505 chunks) |
| BiomeCacheBlock | Done | B + C: cache construction/query order, expiry, clock rollback, cleanup gate |
| GenLayerRiverMix | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| GenLayerDownfallMix | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| GenLayerTemperatureMix | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| NoiseGeneratorOctaves | Done | R: all original raw terrain cases, including both Far Lands directions |
| GenLayerVillageLandscape | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| LayerIsland | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| GenLayerRiverInit | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| ChunkProviderLoadOrGenerate | Not done | Existing loader retained; changed lifecycle not audited or proven exact. |
| BiomeGenSwamp | Done | U + D + A: exact biome properties and spawn lists; matching decorator cases except Taiga lighting dependency noted below |
| GenLayerDownfall | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| GenLayerTemperature | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| BiomeGenDesert | Done | U + D + A: exact biome properties and spawn lists; matching decorator cases except Taiga lighting dependency noted below |
| MapGenBase | Done | R: all original raw terrain cases, including both Far Lands directions |
| BiomeGenPlains | Done | U + D + A: exact biome properties and spawn lists; matching decorator cases except Taiga lighting dependency noted below |
| ChunkCoordIntPair | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| BiomeGenOcean | Done | U + D + A: exact biome properties and spawn lists; matching decorator cases except Taiga lighting dependency noted below |
| BiomeGenRiver | Done | U + D + A: exact biome properties and spawn lists; matching decorator cases except Taiga lighting dependency noted below |
| SpawnListEntry | Done | U + A: totals, collection/array selection, explicit totals, zero/negative weights, overflow, biome entries and population |
| WeightedRandomChoice | Done | U + A: totals, collection/array selection, explicit totals, zero/negative weights, overflow, biome entries and population |
| BiomeGenForest | Done | U + D + A: exact biome properties and spawn lists; matching decorator cases except Taiga lighting dependency noted below |
| BiomeGenHills | Done | U + D + A: exact biome properties and spawn lists; matching decorator cases except Taiga lighting dependency noted below |
| WorldGenDungeons | Not done | Failure fixtures exact; successful fixture differs only in two chest metadata bytes (F). b1.8.1 chest orientation is a block-worker dependency. Loot/tile contents need an expanded check. |
| BiomeGenTaiga | Done | U + D + A: exact biome properties and spawn lists; matching decorator cases except Taiga lighting dependency noted below |
| NoiseGeneratorPerlin | Done | R: all original raw terrain cases, including both Far Lands directions |
| WorldGenLakes | Done | F: water/lava lakes, five fixtures, blocks/metadata/both RNG states |
| BiomeGenHell | Done | U + D + A: exact biome properties and spawn lists; matching decorator cases except Taiga lighting dependency noted below |
| BiomeGenSky | Not done | Properties/spawn entries exact (U/A). getSkyColorByTemp constant override still requires renderer integration. |
| WorldGenMinable | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| StructureComponent | Not done | Only the Piece contract is present. Coordinate transforms, block fills, metadata rotation, doors, liquids, chest loot placement and other helpers remain. |
| ComponentMineshaftCorridor | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| StructureVillagePieces | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillageHouse3 | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdLibrary | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| StructureStrongholdPieces | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStronghold | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdRoomCrossing | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentMineshaftCross | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillageHouse1 | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillageHall | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillageChurch | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| StructureBoundingBox | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| ComponentMineshaftRoom | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillagePathGen | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdCrossing | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillageWoodHut | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillageHouse2 | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdStairs | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentMineshaftStairs | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillageWell | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillage | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdCorridor | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillageField | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdStraight | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillageField2 | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| MapGenStronghold | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| StructureMineshaftPieces | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| StructureStart | Done | S: aggregation, sea-level translation, intersection filtering, failed-component removal, RNG states (64 starts per seed) |
| StructureVillageStart | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdPrison | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdStairsStraight | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdLeftTurn | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentVillageTorch | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| MapGenVillage | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| MapGenStructure | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdRightTurn | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| StructureStrongholdStones | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| EnumDoorHelper | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| ComponentVillageStartPiece | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| StructureStrongholdStart | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| StructureStrongholdPieceWeight | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructureVillagePieceWeight | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructurePieceTreasure | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructurePieceBlockSelector | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructureMineshaftStart | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| ComponentStrongholdStairs2 | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| MapGenMineshaft | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |
| StructureStrongholdPieceWeight2 | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| EnumDoor | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| ComponentVillageRoadPiece | Not done | Not ported. Concrete structure generation/piece implementation and oracle coverage remain. |

## Next work

1. Port StructureComponent helpers and all concrete pieces/factories, MapGenStructure/start maps and the three provider calls. Add real starts/pieces/bounds/block oracle dumps.
2. Resolve Taiga placement through the world lighting port and dungeon metadata through the new chest behavior; extend dungeon tile/loot checks.
3. Finish Sky dimension/population, BiomeGenSky sky color and retained biome-generator lifetime across worlds.
4. Run populated preload/grids, save round trips and region comparisons, then the full verify181 suite.

`target/commit-b181-terrain.sh` stages only this worker's explicit file list and uses the requested author and co-author trailers. Existing WIP worker snapshots are left intact.
