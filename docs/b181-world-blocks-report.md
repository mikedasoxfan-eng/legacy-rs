# Beta 1.8.1 world-blocks handoff

This remains a verified **partial port** after the approximately one-hour round-three stage. Round three adds a ChunkCache view, coordinate-sensitive textures and side visibility, original malformed-array/save-handler behavior, freezing and scheduled-tick fixes, and conversion/file-list helpers. Full World ticking, several save classes, and the remaining block/item audits are unfinished. The per-class table below records current scope; historical round-two player claims belong to the player-gui worker.

## Round-three changes and validation

The release build `cargo build --release -p worldgen181 -p client` succeeds. Nine new original-bytecode modes are wired into `tools/verify_world_blocks181.sh`; the existing delimited block in `tools/verify181.sh` invokes them. No shared oracle dispatch, Beta 1.7.3 source/verifier, entity AI, terrain generator, player-use/hunger engine, or world renderer was changed.

| New check | EXACT byte count | Coverage |
|---|---:|---|
| cache | 896,319 | IDs 0..109, metadata, both leaf modes, four sky-darkening values, height/chunk/30-million boundaries, packed/saved light, materials and lazy chest identity |
| textures | 531,456 | Every registered block's coordinate texture, metadata and six sides; snow-covered grass, furnace/dispenser fronts, locked/double chest neighbor priority |
| sides | 867,128 | All block/neighbor IDs, three bounds, both leaf modes, portal-axis masks, pane edges and extended/contracted piston textures |
| weather | 3,276,800 | 32,768 updates, signed timer extrema, arbitrary float bits/NaNs, partial ticks, Nether suppression, RNG and sky subtraction |
| arrays | 4,512,137 | Constructor lengths, absent/short/long/wrong-type NBT arrays, raw nibble/height preservation, invalid-ID sanitization |
| savelife | 445 | Optional players directory, lazy region directory, session trailing bytes, current/old fallback, absent/wrong Data, bad gzip header, failed rename cleanup and Nether/data paths |
| nextticks | 3,871,336 | Hash/equality/comparison and scheduled queue replays across signed-long overflow, duplicate requests, missing chunks, zero block, force flag, 1,000-entry cap and two interleaved worlds sharing entry IDs |
| frost | 8,217,620 | Sixteen controlled one-chunk snow/freeze worlds, 100 ticks each, horizontal water neighbors, light/rain, injected biome flags, blocks/light/precipitation cache and RNG; original ten biome flags |
| fileclasses | 80,531 | Chunk-file/folder patterns, signed base-36 limits and overflow, region sorting, save-list fields/Java UTF-16 ordering, every registered floor/target combination for gate placement |

The first regression run exposed an inherited merged-checkout omission: IDs 361/362 were registered but absent from the seed planting handler. The minimal existing handler now plants pumpkin/melon stems. Gate placement now checks the occupied target as well as solid floor. The only renderer edit clones the now-variable-length height map to preserve the existing snapshot API.

ChunkCache exports its queried IBlockAccess surface; `World` and the cache expose `block_texture181`/`block_texture` and side-visibility wrappers using shared block bounds. The renderer integrator still needs to consume these APIs. Instance opacity preserves the double-slab constructor quirk: static lookup is false while the slab instance returns true.

Chunk/Nibble/HeightMap arrays now retain original NBT lengths. Missing byte-array tags produce non-null empty arrays, so the original null-only repair test does **not** repair them. Save handlers create regions lazily, accept trailing session-lock bytes and discard `level.dat_new` even when rename fails. The bad-GZIP-header constructor leak is preserved with Windows sharing flags; Java's later garbage-collection/finalizer timing is not modeled, and Rust retains that handle until process exit. Error-printing/exception adapters and the full old loader remain unfinished.

The live checkout already integrates the terrain biome spawn search and original biome rain/snow flags; the round-two handoff below was stale on those points. All ten original biomes, including Taiga, disable snow. The freeze branch now requires a horizontal non-water neighbor. The `frost` replay deliberately fixes the active chunk set to one chunk and does not prove multi-chunk hash iteration. Production still uses the inherited old chunk hash ordering; Beta's x-XOR-z collision/identity ordering remains a blocker for full World EXACT.

Scheduling now uses the Beta coordinate hash, a process-global signed wrapping entry counter, zero scheduled time for block ID zero and the queue/set consistency assertion. The extended original-bytecode replay proves cross-world counter sharing, including duplicate requests consuming IDs. Complete scheduled block behaviors and the tuple representation's equal-ID/mutable-entry edge semantics still need a class-level audit.

Build/replay logs are under `target/world-blocks3-*.log`. The full approximately one-hour verifier is left to the integrator. `target/commit-b181-world-blocks3.sh` stages the explicit round-three files and uses the requested author and both trailers; Git was not written or committed.

Final regression results: all **32 focused checks EXACT**, including every pre-existing world-blocks check in the current focused runner. The existing `verify181.sh` all-block collision/selection section is EXACT for 10,000 cases. The container click/crafting replay is EXACT for 66,146 records. The existing 1,000-tick container-tile section is **blocked by its oracle**, which throws `NoSuchFieldException: lightingToUpdate` in shared `Oracle.worldBytes`; that harness was left untouched. Round-two player-owner `use`, `useticks`, `foodstats` and `blocking` are historical results and are not part of this round's focused runner.

The existing Alpha conversion section's 289 chunk payloads, progress and obsolete-folder cleanup are EXACT. Level NBT is equal apart from wall-clock `LastPlayed`; only 32/289 chunks share the same sector layout, so byte-for-byte region layout is **not** claimed exact. Conversion now uses the original `players=false` save-handler option; a final replay with both player directories removed confirms neither converter creates them. Full-world save/preload, lighting grids, multi-chunk weather/tick, terrain-based teleporter/travel and rendering sections were not made or claimed EXACT in this round. The original terrain teleporter and container-tile probes still have stale reflection names; focused independent probes avoid changing shared dispatch/harnesses.

## Verification

`cargo build --release -p worldgen181 -p client` succeeds (existing dead-code warnings).

Run `bash tools/verify_world_blocks181.sh` for the focused checks, or `--built` after building. The delimited world-blocks block is appended to `tools/verify181.sh` before its final result/exit, so it executes in the full suite. The full approximately one-hour suite was not run.

| New focused check | Result / byte count | Scope |
|---|---:|---|
| plants | EXACT / 24,399,872 | 112 configurations, 128 ticks each; stems, vines, mushroom spreading, leaves, farmland and RNG |
| geometry | EXACT / 1,256,366 | IDs 97..109, 16 metadata, 16 neighbor masks, three neighbor kinds; bounds/collisions; fence/gate connection predicate |
| drops | EXACT / 370,368 | New IDs and old/new stairs, 16 metadata, 32 seeds, chance, RNG and item positions |
| saves | EXACT / 1,464 | 24 level-info cases, clone and gzip persistence |
| lights | EXACT / 10,823,568 | 80 edits, height/occlusion state, sky/block flood light, packed light, provider brightness |
| chests | EXACT / 67,512 | 3,360 facing cases plus 800 animation ticks, adjacency, negative viewers, sounds and RNG |
| helpers | EXACT / 263,588 | floor overflow/NaN, inclusive RNG state, variable-height nibbles, precipitation cache invalidation |
| fluids | EXACT / 717,113 | 51,200 display ticks, ordered particles/sounds, RNG, packed light and face rules |
| teleporter | EXACT / 1,129,872 | Six fixture worlds with creation/search/fallback and both portal axes |
| items | EXACT / 4,280 | Seeds, stem bonemeal, gates, nullable NBT, vine/leaf shearing |
| actions | EXACT / 3,107 | All registered item actions/durations and real bytecode bow shot threshold/critical flag |
| registry | EXACT / 2,936,680 | Item properties, tool strengths, harvestability, block hardness and attack damage |
| definitions | EXACT | All 256 block entries, 29 materials and geometry defaults regenerated from bytecode |
| region cross-read | EXACT / 190,406 | 36 write/overwrite operations; size deltas; original Java reads Rust, Rust reads original Java |

Existing `all Beta block collision/selection states (10000 cases)` is EXACT after the geometry/lazy-tile changes. Existing MathHelper sine-table check remains EXACT (0 / 65,536 differences). No claim is made that the old full-world save, lighting-grid, weather/tick, player-use or travel sections are EXACT. The existing terrain-based teleporter oracle itself fails on its stale `field_4232_a` reflection name; the new independent teleporter suite uses the correct field and exercises the original algorithm.

## Round-two integration and remaining work

Live input now starts and releases bow/sword/food use, suppresses mining/repeated clicks during use, and slows movement input while using. Food state saves with the player. Hunger exhaustion is connected to walking/swimming/diving, blockable damage, and food potion ticking. The minimal DamageSource adapter accepts armor-bypass and exhaustion fields; starvation uses it. The entities owner must map all existing environmental callers, finish jump/sprint/attack/mining exhaustion, player capabilities, full potion behavior and critical-arrow hit/flight replay. No full player simulation EXACT claim is made.

Huge mushrooms and silverfish player-harvest spawning are integrated. `block::visual181` exports original block render types, side/metadata and coordinate-sensitive textures, side visibility, color APIs and creative item stacks. The integrator must call these APIs from RenderBlocks/lightmap/creative UI. Climate floats come from the terrain owner's WorldChunkManager.

`world::spawn181` ports the original biome search, including the replacement-count reservoir bug, and bounded seed-local spawn walk. The current live entry uses terrain's GenLayer biome search. Overworld spawn tests grass; `set_spawn_location` has the original 10000-attempt cap. Original biome flags and the horizontal non-water freeze predicate are integrated. Full World/Chunk ticking, x-XOR-z hash collision iteration and complete map-feature/game-type propagation remain incomplete.

Chest backing arrays start with 36 slots and load as 27; containers expose only 27. Signed tile inventory counts, split/clamp behavior and cache/invalidation paths are ported. Save helpers cover uncompressed/safe-replace operations. Region relocation preserves the original bug: corrected coordinates are written to the outer root and the unchanged Level coordinates are loaded again. Malformed-array preservation and the tested save-handler lifecycle are integrated. Global region-cache behavior, complete conversion, the old save loader and broader save-handler compatibility remain incomplete.

No Beta 1.7.3 source or verifier was edited. Adjacent changes are limited to player/item state and inventory identity/count plumbing, charged-arrow construction/critical flags/gravity, a silverfish constructor/destroy hook, and signed-count fixture conversions in the Beta 1.8.1 renderer check. No entity AI, terrain generator, GUI or world renderer was rewritten.

Git remains unmodified. Use `target/commit-b181-world-blocks3.sh` from a writable Git environment for this round's explicit file list, requested author and both trailers.

## Round-two verification

Final focused validation: all 14 round-one suites and all 13 round-two suites are EXACT. Release build succeeds with existing dead-code warnings.

`tools/verify181.sh` already ends with the delimited world-blocks block calling `tools/verify_world_blocks181.sh --built`; the focused script now includes the new modes below. Every Java probe calls the original Beta 1.8.1 bytecode. The visual table is regenerated independently by the oracle on each replay and compared to output through the Rust texture/color/creative-list APIs.

| Added focused section | Result / bytes | Coverage |
|---|---|---|
| use | EXACT / 467,956 | Food completion, effects/events/RNG, hunger/saturation, bow charge/release/constructor |
| useticks | EXACT / 57,832 | 40-tick use/cancel/alias/mutation/release lifecycle |
| mushrooms | EXACT / 973,792 | 320 bonemeal configurations and 96 direct generator replays |
| chestlife | EXACT / 108,062 | 400 inventory operations, backing-array/NBT and metadata/cache lifecycle |
| math | EXACT / 1,060,373 | Complete public helper surface, random bit patterns and integer extrema |
| storage | EXACT / 63,957 | Uncompressed and safe-replace save helper lifecycle/failures |
| silverfish | EXACT / 65,830 | 64 constructor/event RNG replays plus conversion predicates |
| visual | EXACT / 246,524 | 256 IDs, 16 metadata states, six sides, both leaf graphics settings; creative list |
| colors | EXACT / 253,184 | Synthetic float climate/colormaps, extreme coordinates, all stem direction masks |
| spawn | EXACT / 10,986 | Original biome search/RNG, walk limit and all three provider predicates |
| blocking | EXACT / 30,176 | Sword/blocking, armor durability/remainder, bypass and food exhaustion |
| relocation | EXACT / 657,482 | Original region-load relocation, missing Level/Blocks and invalid IDs |
| foodstats | EXACT / 409,600 | 4,096 persistence cases and 16,384 arithmetic operations, arbitrary float bits/NaNs and signed integer overflow |

The final release build and all focused replays are recorded in `target/world-blocks2-build.log` and `target/world-blocks2-verify.log`. The full approximately one-hour `verify181.sh` run is left to the integrator. Inherited classes without complete current coverage stay Not done below.

## Per-class status

“Not done” includes partial ports and inherited implementations whose complete Beta 1.8.1 coverage has not been established.

| Class | Status | Proof / remaining scope |
|---|---|---|
| World | Not done | lights/helpers/spawn/weather/nextticks/frost EXACT for tested surfaces. Live biome search/flags and global tick-entry IDs are integrated. Active-chunk hash collision/identity ordering, complete block/entity ticking and propagation remain. |
| Chunk | Not done | lights/helpers/chests/arrays EXACT for tested relight/cache/lazy tile and raw constructor/NBT arrays. Full chunk/entity/save lifecycle remains. |
| ChunkCache | Not done | New independent cache EXACT for queried access/light/material/boundary/lazy-tile surface. Renderer hookup and complete null/EmptyChunk/error behavior remain unaudited. |
| CompressedStreamTools | Done | saves + storage EXACT: gzip semantics, uncompressed reads/writes, safe replacement, stale temporary files, missing file, invalid root, failed target deletion and ignored rename result. |
| Particle | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| WorldInfo | Done | saves: game type, missing/default map features, clone, persistence, player dimension and existing level fields. |
| MCHash | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| Teleporter | Done | teleporter: six flat-world replays, search, construction, high arrival, ocean/platform fallback, portal axes, positions and RNG. |
| ChunkFile | Done | fileclasses EXACT: filename/default coordinates, signed base-36 parsing/overflow, getters and region comparison; converter uses this helper. |
| ChunkLoader | Not done | relocation + arrays EXACT for relocation bug, valid/missing/malformed/wrong-type arrays and invalid IDs. Old chunk-directory loader, full entity/tile lifecycle and exception behavior remain. |
| MathHelper | Done | math + helpers EXACT: all public helpers, 20,000 arbitrary float/double bit patterns, integer extrema, negative zero/NaNs, inclusive Random states; prior 65,536-entry sine table remains exact. |
| RegionFile | Done | regions: 36 writes/overwrites, sector reuse and size deltas, Java reads Rust and Rust reads Java. |
| RegionFileCache | Not done | Lazy region creation and explicit local cache flush added. Existing 256-entry per-storage eviction retained; original global cache, sharing/disposal and failure-path replay remain. |
| WorldBlockPositionType | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| MapStorage | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| SaveConverterMcRegion | Not done | fileclasses EXACT helpers now used in scanning/sorting; matching coordinate overflow throws instead of silently skipping. 289-chunk conversion payload/progress/cleanup replay EXACT, level NBT equal except wall-clock LastPlayed; region sector layout differs. Save-handler players=false corrected. Full API/failure/Nether/cache audit remains. |
| NibbleArray | Done | helpers: both constructors, heights 4..8, every nibble and underlying bytes. |
| SaveFormatComparator | Done | fileclasses EXACT: fields/getters, newest timestamp first, signed-long limits, Java UTF-16 comparison including supplementary characters and prefixes. |
| EnumAction | Done | actions: original action ordinals for every registered item. |
| RegionFileChunkBuffer | Done | regions: original compressed-stream close writes checked against Rust compression/write path. |
| IBlockAccess | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| SaveFormatOld | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| WorldProvider | Not done | spawn EXACT for grass predicate; brightness tables retained. Terrain provider construction and map-feature propagation remain. |
| IWorldAccess | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| SaveHandler | Not done | savelife EXACT for optional dirs, session checks, level fallback, failed rename/delete cleanup, DIM-1 and data paths. Player-save API, full failures and Java finalizer timing remain. |
| ChunkFilePattern | Done | fileclasses EXACT: original complete-name regex, lowercase/sign/newline/overflow acceptance; filtering is separate from coordinate parsing. |
| ChunkFolderPattern | Done | fileclasses EXACT: one/two lowercase base-36 characters, directories only; converter uses this helper. |
| EmptyChunk | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| IChunkProvider | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ISaveFormat | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ISaveHandler | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| NextTickListEntry | Not done | nextticks EXACT for tested hash/equality/signed comparison and global-counter queues, including overflow and two interleaved worlds. Queue representation is tuple-based; direct mutable-entry/equal-ID tie semantics still need a class-level audit. |
| WorldProviderHell | Not done | spawn EXACT for nonair, nonbedrock, opaque spawn predicate; lights brightness EXACT. Full provider generation remains with terrain owner. |
| WorldProviderSky | Not done | spawn EXACT for material movement-blocking predicate. Live Sky dimension/provider integration remains. |
| BlockVine | Not done | plants/geometry/drops/items retained; visual/colors EXACT for render type, textures and biome foliage tint. Full inherited class audit/render hookup remains. |
| BlockChest | Not done | chests/chestlife/visual/textures EXACT for tested inventory/metadata/adjacency/cache and coordinate-sensitive double-chest textures. Complete activation replay remains. |
| BlockStem | Done | plants/geometry/drops/items + visual/colors EXACT: growth, fruit, fertilizer, seed drops, bounds, all metadata colors/textures and neighboring-fruit direction priority. |
| TileEntityChest | Done | chests + chestlife EXACT: object identity, 36 initial slots/27 loaded slots, signed counts, clamping/splitting including negative amounts, NBT inventory, cached metadata/block, reset/invalidate/validate, adjacency, lids, viewers, sounds/events. Renderer consumes exported state. |
| ItemDoor | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemDye | Not done | items + mushrooms EXACT for crops/stems and both huge mushrooms, including failed growth/metadata restoration. Edit-permission and full inherited dye surface remain. |
| BlockFluid | Not done | fluids + sides EXACT for random-display RNG/events, packed light, solid faces and same-material/ice/top visibility. Renderer hookup and full flow/tick audit pending. |
| BlockPane | Done | definitions/geometry/drops/visual/sides EXACT for both pane IDs, connection/collision/selection, edge/base textures, opacity/render type and same-ID visibility. Renderer must consume exported APIs. |
| BlockDispenser | Not done | null tile guard added; original algorithm otherwise retained. Full dispenser replay not run. |
| Block | Not done | definitions/registry/geometry/drops cover defaults and new block registration. Base render/light/item-use hooks not fully ported. |
| BlockPistonBase | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockMushroom | Done | plants + mushrooms + visual EXACT: density/spread, placement, bonemeal, huge generator checks/caps/stems, failed restoration, metadata and original RNG state. |
| FoodStats | Done | use/useticks/foodstats EXACT: NBT/defaults, signed overflow and float/NaN arithmetic, exhaustion/saturation depletion, 80-tick healing/starvation and difficulty thresholds. |
| ItemFood | Done | registry/actions/use/useticks EXACT: all 15 foods, hunger/saturation admission, 32-tick use, effects/RNG/sounds/particles, potion IDs/durations/chances and completion consumption. General player potion engine remains with entities owner. |
| BlockFenceGate | Done | geometry/items/visual/fileclasses EXACT for collision, solid-floor and replaceable-target placement, yaw/activation/open-close metadata, opacity/render type. Renderer hookup remains with integrator. |
| ItemBow | Done | registry/actions/use/useticks EXACT: hold, release -20..80 elapsed ticks, threshold, speed, full-charge critical flag, arrow construction floats/RNG, sound and arrow consumption. Projectile simulation beyond construction needs entities coverage. |
| BlockFurnace | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| Item | Not done | registry/actions EXACT for definitions, use action and duration. Use lifecycle and edit permission support incomplete. |
| TileEntityPiston | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockSilverfish | Done | definitions/geometry/drops + silverfish + visual EXACT: metadata textures, conversion predicates, player-destroy spawn constructor/position/RNG and all 20 explosion particles. Silverfish AI/model/sounds belong to entities owner. |
| ItemBlock | Not done | placement supports all new IDs, replacing vines, metadata and surviving-ID guard. Edit permissions and complete placement replay missing. |
| ItemStack | Not done | nullable NBT proof retained; use identity tokens now cancel identical replacements while permitting same-object hotbar aliases (useticks EXACT). Copy-based inventory still lacks fully shared mutation of aliased Java stacks; full item delegation/load audit remains. |
| TileEntity | Not done | chestlife proves metadata/block caches, inventory refresh, resets and invalidate/validate. Full factory/persistence surface for every tile remains. |
| BlockTallGrass | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockJukeBox | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockMushroomCap | Done | definitions/geometry/drops + visual/mushrooms EXACT: every metadata/side texture, drops and huge cap generation. |
| BlockMelon | Done | definitions/geometry/drops + visual EXACT: render defaults, top/side textures, quantity RNG and metadata drops. |
| BlockPistonMoving | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemVine | Not done | side placement and shearing paths ported; no full placement oracle suite. |
| BlockGrass | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockStoneBrick | Done | definitions/geometry/drops + visual EXACT: all metadata/side textures, subtype drops and original creative variants. |
| ItemReed | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemHoe | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockRedstoneWire | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemFlintAndSteel | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockRedstoneRepeater | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemBucket | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemPainting | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemSword | Done | registry/actions/use/useticks/blocking EXACT: properties, use action/duration, hold/cancellation, blocking and armor damage/remainder/exhaustion with bypass flag. Existing environmental attack callers still need DamageSource mapping. |
| BlockContainer | Not done | direct lazy tile factory path ported; full container class coverage pending. |
| BlockLeaves | Not done | plants/items retained; visual/colors EXACT for fancy/fast texture selection, pine/birch flags and float-climate foliage lookup. Full decay/render integration remains. |
| Material | Done | definitions: all 29 material defaults; registry also checks harvest flags and tool behavior. |
| BlockNote | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemRedstone | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| StepSound | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockStairs | Not done | geometry/drops EXACT including new stairs and model metadata drop quirk. Rendering integration pending. |
| ItemBed | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| MaterialWeb | Done | definitions: separate web material with movement disabled while isSolid remains true. |
| ItemSign | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockPistonExtension | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| TileEntityDispenser | Not done | nullable item loading integrated; full tile inventory replay not run. |
| TileEntityFurnace | Not done | nullable item loading integrated; full furnace replay not run. |
| ItemShears | Not done | items/registry cover vine and leaf harvest; tall grass/vines tool cost changed. Full destroy-stat/cost replay pending. |
| ItemSlab | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockFence | Done | geometry: original fence/gate connection predicate for IDs 0..109; existing 10,000-case block geometry replay. |
| ItemLog | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockDoor | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockFire | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockFarmland | Done | plants: water, metadata, crops and both new stems; stem-aware walking hook ported. |
| BlockFlower | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockRedstoneOre | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemSeeds | Not done | items EXACT for all seed IDs, sides, soil and occupied target. Player edit permissions pending. |
| ItemSoup | Done | actions/use/useticks EXACT: 32-tick completion, hunger/saturation and bowl replacement. |
| PistonBlockTextures | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockCake | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockCloth | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockPressurePlate | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockStep | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockTrapDoor | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockWeb | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemCloth | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemMap | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| TileEntitySign | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockBed | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockCactus | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockMobSpawner | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockReed | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockSign | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockSnowBlock | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemLeaves | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemRecord | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
