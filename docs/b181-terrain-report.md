# Beta 1.8.1 terrain worker report - round 3

Round 3 implements the remaining concrete strongholds, the changed singleplayer provider lifecycle, the alternate direct-mapped chunk cache, and Java compound/tile-list serialization order. The normal World now registers and populates strongholds. All assigned terrain/structure classes have the scoped original-bytecode checks listed below. The complete suite finished with **SOME CASES DIFFER** in other simulation/rendering areas and the legacy server harness; no complete-suite pass is claimed.

## Verification evidence

All oracle classes are harnesses over Mojang's original deobfuscated Beta 1.8.1 bytecode, run on Java 8u51. No b1.7.3 crate, oracle or verification script was edited.

- H3: `tools/verify_terrain3_181.sh --built`: **96/96 EXACT** in the completed full run (`target/terrain3-full-verify.log`): 78 graph/block/provider/cache/NBT fixtures plus 18 independent-save and cross-load checks. The initial 78-case run is in `target/terrain3-round3-final.log`. Six seeds: 0, 42, 3257840388504953787, -4172144997902289642, Long.MAX_VALUE and Long.MIN_VALUE.
- Strongholds: six modes per seed (layout, retained map, blocks, wet/liquid rejection, all direct concrete pieces in four orientations with split clips, and real grids). Each layout serializes 16 full starts. Each real grid loads 7x7 chunks around each of three candidate locations and compares every loaded chunk's blocks, metadata, sky/block light, height, population flag, surviving graph and both RNG states. This proves registration/population alongside mineshafts and villages.
- P3: three provider modes per seed: signed-Long HashSet insertion/removal including equal-hash tree bins, automatic unload/scan, and saves. The scan fixture uses 161 chunks, 70 passes, changing spawn/player positions and height, explicit drops, load-vs-provide cancellation and all four out-of-range boundaries. Saves check partial/full behavior with and without entities, neverSave and signed world times.
- Q3: three alternate-cache modes per seed: normal, null loader and failing loader. The oracle dumps chunk object identity, all flags and last-save time, every loader/generator callback in order, ring aliases, saved progress and RNG. The Java jar has no constructor for this unused legacy class; the harness allocates it via Unsafe and supplies its original private dependencies.
- E3: six original NBT byte fixtures, each containing eight shuffled compounds with real save keys plus 64 colliding Java String keys, replacements and nested compounds. Every serialized byte matches, including resizing and tree-bin iteration. Empty lists now write type byte 1, as vanilla does.
- S3: all six independently generated saves are **EXACT**, **2,103/2,103 decompressed chunk NBT payloads byte-identical**, including entities, tile order and empty lists. Both save-load directions pass for every seed (18 checks; `target/terrain3-six-saves-final.log`). Sector allocation/order and timestamps are not chunk data and need not be identical.
- Round-1: **157/157 EXACT**; round-2: **78/78 EXACT**. Both save-load directions are exact with the corrected 27-slot chest inventory comparison (`target/terrain3-save-direction-final.log`). The complete suite finished with **SOME CASES DIFFER**. All three appended terrain suites passed (157 + 78 + 96 = **331 focused checks EXACT**). Logs: `target/terrain3-round1-final.log`, `target/terrain3-round2-final.log`, `target/terrain3-full-verify.log`. `target/terrain3-world-final.log` records the initial diagnostic run before the chest comparison and save RNG corrections; its failure summary is superseded by the later logs.

| Key | Original-bytecode check |
| --- | --- |
| R | Seven original raw Overworld cases, 649 chunks, including structures enabled and Far Lands; all exact. Raw calls register starts; they do not place structure blocks. |
| N | Six original raw Nether cases, 505 chunks; all exact. |
| L | Four GenLayer endpoints, five seeds, negative/non-square/Far Lands queries. |
| B | biomes181: biome cache/query/climate float bits, padded search, viability and RNG. |
| C | biomecache181: cleanup/clock rollback, Hell/Sky fixed biome searches and RNG. |
| U | terrainutils181: biome properties/spawn lists, weighted selection/overflow, bounding boxes and structure utility values. |
| S | structurestarts181: 64 synthetic starts per seed, bounds, translations, intersection filtering and failed-piece removal. |
| K | Six raw Sky terrain cases, 141 chunks including Far Lands. |
| F | features181: all standalone trees, mushrooms, sand/clay/gravel/ore/lakes/dungeon fixtures. |
| D2 | features181 decorate0..9, dry and water fixtures: all 20 exact. |
| A | features181 animals0..9: 40 exact cases including nonempty groups, entity type/position/yaw/sheep color and RNG. |
| C2 | component181: six comprehensive StructureComponent helper dumps. |
| M2 | structures181 layout/map/blocks: six seeds each, real mineshaft graphs, Java HashMap order, chest/spawner tiles. |
| V2 | structures181 village-layout/village-map/village-blocks: six seeds each, 16 starts per layout, 55 map queries including real biome candidate regions, clipped block and loot fixtures. |
| Y2 | terrainround2 sky/skygrid: six seeds each, flat population plus real floating-island chunks and light/height/RNG data. |
| J2 | terrainround2 dungeon: six successful dungeon fixtures, full inventories and spawner tile state. |
| T2 | terrainround2 lifetime/colors: six seeds each; forest decoration across four successive Worlds, all biome sky-color float inputs and dimension angle bits. |
| S3 | terrainsave181: six independently generated saves (2,103 chunks), plus both cross-load directions. |
| H2 | terrainround2 locations: six stronghold candidate/biome relocation/RNG checks. This does not prove stronghold pieces. |
| W2 | Focused existing populated preload/grid/save/region sections, rerun after oracle fixes; results recorded below. |

The existing full-suite raw Overworld/Nether cases, all 22 populated-grid/preload/cross-save sections, terrain mesh comparisons, Alpha conversion (289 chunks and 307 progress steps), and lightmap checks are EXACT. Broader ticking, item-use, player/entity and selected renderer failures remain; the owner handoff lists them without treating a missing server-oracle dump as an algorithm comparison.

## Integration details

- **Version correction:** Beta 1.8.1 has no portal-room piece, no portal-room retry, and no nearest-structure/stronghold location method on MapGenStructure or IChunkProvider. Local original source and the original jar both establish this. The port retains this version's three candidate coordinates and starts; later-release APIs were not invented.
- StructureStrongholdPieces' RightTurn branch invokes the inherited LeftTurn static factory and creates a LeftTurn. This vanilla bug is preserved. Direct RightTurn generation is independently verified as well.
- The previously missing IntCache reset on an aligned biome-cache hit is now present. Its padded/stale tails change stronghold relocation; the new real grids exposed and proved the fix.
- Normal ChunkProvider uses packed 64-bit chunk keys, spawn-protected drops, the 100-drop limit and rotating 10-chunk player-distance scan. onChunkUnload precedes save. Load cancels drops; provide cache hits do not. Signed-Long tree-bin removal follows the locally installed OpenJDK 8 source and original-bytecode HashSet fixtures.
- Alternate cache code is a separate library IChunkProvider implementation with a CacheHost boundary for loader/generator/world callbacks. It is not selected for the normal World, matching vanilla.
- NBTTagCompound writes use Java String hashing and UTF-16 comparison for tree ordering. TileEntities follow the chunk map's Java hash order instead of coordinate sorting. The old six item Motion/Rotation differences arose because the save harness did not pin Math.random or Entity.rand. A separate original-bytecode save fixture pins these and chest randomness; all six seeds now give 2,103/2,103 byte-identical decompressed chunk payloads, including entities. No entity behavior was rewritten. The six-seed save extension also verifies both load directions.
- The existing world comparison now compares the 27 visible chest inventory slots, matching Oracle.worldBytes. It preserves the original 36-slot constructor backing array and 27-slot post-load array. This repairs a harness false mismatch.
- Existing concrete-piece helpers and new block IDs are used directly. No new renderer, player or entity behavior was introduced. Sky renderer/camera integration remains integrator work.
- `TERRAIN_COMPARE_DUMP=<path>` optionally writes the Rust binary fixture for diagnosing a failed terrain comparison.

## Per-class status

Done means literal implementation with the listed original-bytecode coverage. The finite fixtures describe the scope of the proof.

| Class | Status | Evidence / remaining work |
| --- | --- | --- |
| ChunkProviderGenerate | Done | R + W2 + H3: literal density/surface/caves/ravines retained; stronghold/mineshaft/village registration and population in vanilla order. Six real grids around all three strongholds exact, including all block, metadata, light and height arrays, graph state and both RNG states. |
| ChunkProviderSky | Done | K + Y2: original raw Sky terrain; six flat population fixtures and six real 5x5 floating-island grids with blocks, metadata, both light arrays, height maps, populated flags and both RNG states. World Dimension::Sky and saved dimension selection wired. |
| BiomeDecorator | Done | D2: all 20 original decorator fixtures exact; Taiga dependency resolved by the merged lighting implementation. Existing literal order retained. |
| WorldChunkManager | Done | B + C + H3: query order, climate floats, biome search, viability, cached queries, fixed managers |
| MapGenRavine | Done | R: all original raw terrain cases, including both Far Lands directions |
| MapGenCaves | Done | R: all original raw terrain cases, including both Far Lands directions |
| WorldGenBigMushroom | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| WorldGenSwamp | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| BiomeGenBase | Done | U + D2 + A + T2: properties, spawn lists, decoration, sky-color bits and biome-global retained BigTree identity across four sequential Worlds. |
| GenLayer | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| WorldGenTaiga2 | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| GenLayerZoom | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| ChunkProvider | Done | P3: six signed-Long collision/removal fixtures, 70-pass automatic scan/unload replays, drop cancellation vs provide hits, spawn/player-distance protection, 100-unload/10-scan limits, boundary EmptyChunk behavior and 24-save throttling/neverSave/null-loader/time fixtures; W3 save-load verification. |
| WorldGenTaiga1 | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| WorldGenForest | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| WorldGenTrees | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| WorldGenBigTree | Done | F + T2: isolated generation and four sequential Worlds using the same biome generator, all six signed seeds exact. |
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
| ChunkProviderLoadOrGenerate | Done | Q3: six seeds each in normal/null-loader/failing-loader fixtures, original-bytecode chunk identities and every callback, direct-map aliasing/eviction, last-query shortcut outside moved windows, spawn-search bypass, population, two-save throttle, full-save progress and loader/generator ticking. Host callbacks supplied through CacheHost; normal World continues to use ChunkProvider, as vanilla does. |
| BiomeGenSwamp | Done | U + D + A: exact biome properties and spawn lists; all 20 decorator fixtures now exact (D2) |
| GenLayerDownfall | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| GenLayerTemperature | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| BiomeGenDesert | Done | U + D + A: exact biome properties and spawn lists; all 20 decorator fixtures now exact (D2) |
| MapGenBase | Done | R: all original raw terrain cases, including both Far Lands directions |
| BiomeGenPlains | Done | U + D + A: exact biome properties and spawn lists; all 20 decorator fixtures now exact (D2) |
| ChunkCoordIntPair | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| BiomeGenOcean | Done | U + D + A: exact biome properties and spawn lists; all 20 decorator fixtures now exact (D2) |
| BiomeGenRiver | Done | U + D + A: exact biome properties and spawn lists; all 20 decorator fixtures now exact (D2) |
| SpawnListEntry | Done | U + A: totals, collection/array selection, explicit totals, zero/negative weights, overflow, biome entries and population |
| WeightedRandomChoice | Done | U + A: totals, collection/array selection, explicit totals, zero/negative weights, overflow, biome entries and population |
| BiomeGenForest | Done | U + D + A: exact biome properties and spawn lists; all 20 decorator fixtures now exact (D2) |
| BiomeGenHills | Done | U + D + A: exact biome properties and spawn lists; all 20 decorator fixtures now exact (D2) |
| WorldGenDungeons | Done | F + J2: successful fixture blocks, metadata, all chest slots/item damage/counts, spawner EntityId/Delay and both RNG states; all six signed seeds exact. Merged chest behavior resolves old metadata differences. |
| BiomeGenTaiga | Done | U + D + A: exact biome properties and spawn lists; all 20 decorator fixtures now exact (D2) |
| NoiseGeneratorPerlin | Done | R: all original raw terrain cases, including both Far Lands directions |
| WorldGenLakes | Done | F: water/lava lakes, five fixtures, blocks/metadata/both RNG states |
| BiomeGenHell | Done | U + D + A: exact biome properties and spawn lists; all 20 decorator fixtures now exact (D2) |
| BiomeGenSky | Done | U + A + T2: original constant sky-color override and fixed-biome generation. Client Sky fog/cloud/camera wiring remains integrator work. |
| WorldGenMinable | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| StructureComponent | Done | C2: transforms, metadata rotations, liquids, doors, random/selector fills, dome, clear-up/fill-down, chest generation including split oversized stacks, blocks/metadata and RNG states; six seeds exact. |
| ComponentMineshaftCorridor | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| StructureVillagePieces | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillageHouse3 | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdLibrary | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| StructureStrongholdPieces | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| ComponentStronghold | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| ComponentStrongholdRoomCrossing | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| ComponentMineshaftCross | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| ComponentVillageHouse1 | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillageHall | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillageChurch | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| StructureBoundingBox | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| ComponentMineshaftRoom | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| ComponentVillagePathGen | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdCrossing | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| ComponentVillageWoodHut | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillageHouse2 | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdStairs | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| ComponentMineshaftStairs | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| ComponentVillageWell | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillage | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdCorridor | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| ComponentVillageField | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdStraight | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| ComponentVillageField2 | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| MapGenStronghold | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| StructureMineshaftPieces | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| StructureStart | Done | S: aggregation, sea-level translation, intersection filtering, failed-component removal, RNG states (64 starts per seed) |
| StructureVillageStart | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdPrison | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| ComponentStrongholdStairsStraight | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| ComponentStrongholdLeftTurn | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| ComponentVillageTorch | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| MapGenVillage | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| MapGenStructure | Done | M2 + V2 + H3: all three concrete map generators, retained Java HashMap order, registration RNG, sizeable/intersection gates, failed-component removal and real populated stronghold grids exact. |
| ComponentStrongholdRightTurn | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| StructureStrongholdStones | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| EnumDoorHelper | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| ComponentVillageStartPiece | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| StructureStrongholdStart | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| StructureStrongholdPieceWeight | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructureVillagePieceWeight | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructurePieceTreasure | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructurePieceBlockSelector | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructureMineshaftStart | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| ComponentStrongholdStairs2 | Done | H3: original-bytecode layout/map/block/wet/component/grid fixtures across six signed seeds; real graph order/bounds/fields, clipped blocks/metadata, chest slots and RNG states exact. Right-turn factory retains its inherited LeftTurn factory bug. |
| MapGenMineshaft | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| StructureStrongholdPieceWeight2 | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| EnumDoor | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| ComponentVillageRoadPiece | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillageHouse4_Garden | Done | V2: real layout and generated blocks across six seeds; additional village house required by the factory. |
| NBTTagCompound | Done | E3 + S3: original String hashing, resizing/tree-bin iteration, replacement-key writes and all independent saved chunk bytes. Reads now use put semantics for duplicate keys as vanilla does. |
| NBTTagList | Done | E3 + S3: empty lists always emit byte type 1; actual saved entity/tile lists are byte-identical. |
| McRegionChunkLoader | Done for terrain serialization | S3: six independently populated saves, all 2,103 chunk payloads exact, plus both cross-load directions. Existing region allocator/cross-read implementation retained. |


Final release build `cargo build --release -p worldgen181 -p client` succeeds (`target/terrain3-release-final.log`). The existing three unreachable biome-pattern warnings and 26 client dead-code warnings remain. Bash syntax and `git diff --check` pass.

## Remaining work

All assigned stronghold, provider/cache and independent-save work is resolved with the scoped checks above. Sky renderer/camera integration remains integrator work. Whole-world/player/entity/rendering failures and the obsolete server harness are indexed with owner follow-ups in `reports/b181-terrain3-owner-handoff.md`.

Git writes are blocked by the sandbox. `target/commit-b181-terrain3.sh` stages this worker's explicit file list and commits with the requested identity and both co-author trailers. It has not been executed.
