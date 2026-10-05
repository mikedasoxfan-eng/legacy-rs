# Beta 1.8.1 terrain worker report — round 2

The second terrain stage adds real mineshafts and villages, component helpers, Sky population/dimension support, global biome tree lifetime, and expanded original-bytecode checks. **The requested task remains incomplete:** concrete strongholds and provider/cache lifecycle changes still need work. This report supersedes the round-1 status.

## Verification evidence

The oracle runs Mojang's original deobfuscated Beta 1.8.1 bytecode on Java 8u51. All b1.7.3 crates, oracle and verification scripts remain untouched.

`tools/verify_terrain181.sh --built`: **157/157 EXACT**, resolving the old two Taiga decorator failures and dungeon metadata failure. `tools/verify_terrain2_181.sh --built`: **78/78 EXACT** in the full suite run. Each new mode runs seeds 0, 42, 3257840388504953787, -4172144997902289642, Long.MAX_VALUE and Long.MIN_VALUE. Full log: `target/terrain2-full-verify.log`.

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
| H2 | terrainround2 locations: six stronghold candidate/biome relocation/RNG checks. This does not prove stronghold pieces. |
| W2 | Focused existing populated preload/grid/save/region sections, rerun after oracle fixes; results recorded below. |

The full `tools/verify181.sh` run completed with **SOME CASES DIFFER**. Earlier populated-world sections could not dump worlds because the inherited harness cast the Beta 1.8.1 chunk cache to a Map and accessed an obsolete lighting queue. Both accessors are now corrected. Renderer/player/entity failures also remain in the full suite; no complete-suite success is claimed. Current focused results are in `target/terrain2-world-checks.log`.

W2 final rerun: **22 EXACT checks** — nine Nether grids (three locations × each/end/none lighting), all six standard preloads, Fast/Fancy seed-25 preloads, all three Overworld grids (24×24 each, 16×16 end, 16×16 none), Java-save/Rust-load and Rust-save/Java-load. These include generated structures and full chunk blocks/metadata/light/height, tile state, provider/world RNG and scheduled ticks where the existing format includes them. Save comparisons intentionally omit unsaved scheduled ticks.

The original Alpha-to-McRegion conversion section is **EXACT**: all 289 decompressed chunk payloads identical and 307 progress steps match. An additional comparison of independently generated Java/Rust populated save bytes is **not exact** (0/289 byte-identical payloads). Diagnostic parsing finds blocks, metadata, light, height and tile contents identical; compound tag serialization order differs throughout, tile-entity list order differs in six chunks, and entity Motion/Rotation differ in six chunks. Ignoring compound order gives 278/289 identical chunks; sorting tiles proves all tile contents match. No serialization or entity approximation was introduced. Those remaining serialization/entity differences are outside this worker's new structure bodies and remain for the respective owners.

Final release build `cargo build --release -p worldgen181 -p client` succeeds (`target/terrain2-release-final.log`). Client dead-code warnings remain; the terrain library adds no warnings. Bash syntax and `git diff --check` pass.

After that final build, all 18 village layout/map/block comparisons were rerun and passed (`target/terrain2-final-villages.log`). The final script and source retain the generalized village size parameter; the original provider's size-zero path is the one covered by V2.

## Integration details

- Raw Overworld generation registers mineshaft/village starts, retains Java 8 signed-Long HashMap iteration (including tree buckets), and initializes stronghold candidate coordinates. Population calls mineshafts then villages, preserves structure/provider RNG sharing across nested loads, and suppresses lake RNG draws in sizeable villages.
- Mineshaft room entrance boxes intentionally retain vanilla's untranslated coordinates after vertical start adjustment. Structure loot keeps overwrite-on-collision and oversized-stack split behavior; no approximations introduced.
- Village generation retains piece-weight limits, repeated selection attempts, pending-road/house order, terrain-height adjustment, path surface rules and all concrete building bodies. `tools/b181/port_village_bodies.py` performs strict translation from local original source; unsupported statements fail instead of producing placeholders.
- `Dimension::Sky`, fixed Sky biome manager, raw generator, old Sky population sequence, spawn predicate, saved dimension selection and celestial angle are wired. Renderer fog/cloud/camera use of Sky remains integrator work.
- Biome BigTree generators are retained globally across Worlds, matching vanilla biome singleton identity.
- Minimal world-block dependencies: dispatch plain torch callbacks through existing literal torch methods before player hooks (needed by structures), and allow reeds on sand (new Beta 1.8.1 behavior, needed by populated terrain). No other block-worker rewrites.
- Minimal shared oracle fixes: serialize `ChunkProvider.chunkList`, write zero for the removed lighting queue, and filter vanilla's invalid-stronghold-biome diagnostic from binary color output. New test modes are separate Java files.

## Per-class status

Done means literal implementation with the listed original-bytecode coverage; incomplete providers stay Not done even when narrower fixtures match.

| Class | Status | Evidence / remaining work |
| --- | --- | --- |
| ChunkProviderGenerate | Not done | R + W2: raw terrain and several full populated worlds exact. Mineshaft/village registration and population wired, village lake suppression preserved. Stronghold pieces and third population call remain absent. |
| ChunkProviderSky | Done | K + Y2: original raw Sky terrain; six flat population fixtures and six real 5x5 floating-island grids with blocks, metadata, both light arrays, height maps, populated flags and both RNG states. World Dimension::Sky and saved dimension selection wired. |
| BiomeDecorator | Done | D2: all 20 original decorator fixtures exact; Taiga dependency resolved by the merged lighting implementation. Existing literal order retained. |
| WorldChunkManager | Done | B + C: query order, climate floats, biome search, viability, cached queries, fixed managers |
| MapGenRavine | Done | R: all original raw terrain cases, including both Far Lands directions |
| MapGenCaves | Done | R: all original raw terrain cases, including both Far Lands directions |
| WorldGenBigMushroom | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| WorldGenSwamp | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| BiomeGenBase | Done | U + D2 + A + T2: properties, spawn lists, decoration, sky-color bits and biome-global retained BigTree identity across four sequential Worlds. |
| GenLayer | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| WorldGenTaiga2 | Done | F: original feature, four fixtures, blocks/metadata/both RNG states |
| GenLayerZoom | Done | L: four original layer endpoints, five signed seeds and non-square/negative/Far Lands queries |
| ChunkProvider | Not done | Existing lifecycle retained; changed provider/cache lifecycle not audited or proven exact. |
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
| ChunkProviderLoadOrGenerate | Not done | Existing loader retained; changed lifecycle not audited or proven exact. |
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
| ComponentStrongholdLibrary | Not done | Concrete stronghold implementation remains absent. |
| StructureStrongholdPieces | Not done | Concrete stronghold implementation remains absent. |
| ComponentStronghold | Not done | Concrete stronghold implementation remains absent. |
| ComponentStrongholdRoomCrossing | Not done | Concrete stronghold implementation remains absent. |
| ComponentMineshaftCross | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| ComponentVillageHouse1 | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillageHall | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillageChurch | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| StructureBoundingBox | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| ComponentMineshaftRoom | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| ComponentVillagePathGen | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdCrossing | Not done | Concrete stronghold implementation remains absent. |
| ComponentVillageWoodHut | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillageHouse2 | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdStairs | Not done | Concrete stronghold implementation remains absent. |
| ComponentMineshaftStairs | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| ComponentVillageWell | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillage | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdCorridor | Not done | Concrete stronghold implementation remains absent. |
| ComponentVillageField | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdStraight | Not done | Concrete stronghold implementation remains absent. |
| ComponentVillageField2 | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| MapGenStronghold | Not done | H2: all three candidate coordinates, biome relocation and final RNG exact for six seeds. Actual stronghold starts, piece factory and population remain absent. |
| StructureMineshaftPieces | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| StructureStart | Done | S: aggregation, sea-level translation, intersection filtering, failed-component removal, RNG states (64 starts per seed) |
| StructureVillageStart | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentStrongholdPrison | Not done | Concrete stronghold implementation remains absent. |
| ComponentStrongholdStairsStraight | Not done | Concrete stronghold implementation remains absent. |
| ComponentStrongholdLeftTurn | Not done | Concrete stronghold implementation remains absent. |
| ComponentVillageTorch | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| MapGenVillage | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| MapGenStructure | Not done | M2 + V2 prove retained registration maps, Java 8 HashMap<Long> iteration, chunk-key arithmetic, generation scan/RNG and mineshaft/village population. Stronghold specialization and structure-location API remain incomplete. |
| ComponentStrongholdRightTurn | Not done | Concrete stronghold implementation remains absent. |
| StructureStrongholdStones | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| EnumDoorHelper | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| ComponentVillageStartPiece | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| StructureStrongholdStart | Not done | Concrete stronghold implementation remains absent. |
| StructureStrongholdPieceWeight | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructureVillagePieceWeight | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructurePieceTreasure | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructurePieceBlockSelector | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| StructureMineshaftStart | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| ComponentStrongholdStairs2 | Not done | Concrete stronghold implementation remains absent. |
| MapGenMineshaft | Done | M2: real starts/piece order/bounds/state, retained candidate maps, generated blocks/metadata/chest inventories/CaveSpider spawners, six seeds exact. |
| StructureStrongholdPieceWeight2 | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| EnumDoor | Done | U: constructors/fields, inclusive boxes, orientation/overflow, key/hash, stone selector RNG, piece limit/depth gates, enum order |
| ComponentVillageRoadPiece | Done | V2: real starts/piece order/bounds/state/sizeable flag, retained nonempty real-biome candidate maps, generated blocks/metadata/loot, six seeds exact. Vanilla provider size zero checked. |
| ComponentVillageHouse4_Garden | Done | V2: real layout and generated blocks across six seeds; additional village house required by the factory. |

## Remaining work

1. Implement every concrete stronghold piece/factory/start, portal-room retry, registration/population and location API, with real graph/block/tile oracle checks. Candidate coordinates alone are implemented.
2. Audit and port changed ChunkProvider and ChunkProviderLoadOrGenerate cache/unload lifecycle. Their round-1 implementation is retained and unproven.
3. Resolve the independently generated save serialization/entity differences described above; rerun the full suite with the corrected harness and coordinate renderer/player/entity failures with their owners. The focused populated grids and both save-load directions are already exact.
4. Integrator: wire Sky world rendering/camera behavior to the new dimension and biome color APIs.

`target/commit-b181-terrain2.sh` stages this worker's explicit file list and uses the requested author/co-author trailers. The sandbox blocks Git writes; the script was not executed.
