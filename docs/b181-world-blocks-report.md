# Beta 1.8.1 world-blocks handoff

This remains a verified **partial port** after the approximately one-hour round-five stage. SaveFormatOld now selects the old chunk-directory loader; pistons destroy vines; container removal preserves original retained-stack mutations; chest construction preserves null-neighbor aliasing. Full World/Chunk ticking, conversion compatibility and classes marked Not done below remain unfinished.

## Round-five changes and validation

All **54 focused suites EXACT** (48 existing plus six new). The new independent original-bytecode modes are wired into `tools/verify_world_blocks181.sh`, already called by the delimited world-blocks block in `tools/verify181.sh`. The release build `cargo build --release -p worldgen181 -p client` succeeds. Logs: `target/world-blocks5-verify-final.log`, `target/world-blocks5-build-final.log`; `target/world-blocks5-final-probes.log` rechecks all six final probes after adding the direct piston-constant comparison and cleaning the readonly-path fixture. The existing 10,000-case block collision/selection and 66,146-record container click/crafting replays remain EXACT against the retained original-bytecode fixtures (`target/world-blocks5-blocks.log`, `target/world-blocks5-containers.log`). The approximately one-hour full verifier was left to integration; no new full-world ticking/lighting/save EXACT claim is made.

| Added check | EXACT bytes | Coverage |
|---|---:|---|
| legacy5 | 614,900 | Original ChunkLoader signed base-36/extreme paths, creation flags, missing/malformed/corrupt chunk inputs, invalid IDs, relocation, provider loader choices, save/overwrite/temp-file failure/session lock, uncompressed saved NBT and physical size-delta predicates |
| piston5 | 379,694 | All 24 PistonBlockTextures constants; both types, six directions, 24 configurations; actual extension/retraction tile ticks, push limit, immovable/container/new blocks/vines/height limits; 108 interpolation/NBT/collision cases including signed zero, NaN and infinities |
| removal5 | 9,053 | Chest/dispenser/both furnace IDs, eight signed counts; hidden chest slot 35, retained invalid tile inventories, dropped item positions/motions, events and RNG |
| mchash5 | 3,207,268 | 1,024 actual map operations; duplicate/null entries, growth/chain order, lookup/contains, both removals, live key-set edits, retained keys after clearMap and signed version overflow |
| chestopen5 | 164,541 | 3,072 original BlockChest activations versus production container construction; neighbor/invalid-tile masks, blocked lids, nested inventory order/size/contents and null-neighbor aliasing |
| flow5 | 393,216 | 16,384 original fluid velocity calculations; four fluid IDs, every metadata, registered neighbor block/material IDs, lower fluids and upper walls; exact vector double bits |

`LegacyChunkLoader181` preserves separate mkdir attempts, mask directories, signed base-36 filenames and fixed `tmp_chunk.dat`. Storage retains its legacy/McRegion choice across dimensions. Malformed arrays and the outer-root relocation bug use the existing decoder. The oracle compares decompressed NBT and size-delta relationships: gzip implementations differ, so compressed bytes and absolute SizeOnDisk equality are **not** claimed exact. Full entity/tile reconstruction, malformed numeric NBT exceptions, file alias/permissions and Java exception/print adapters remain unproved.

Pistons destroy vines rather than pushing them. Progress uses Java's explicit comparison to preserve NaN. Nonfinite interpolation/collision state is covered; infinite-progress updates are excluded because the original scans enormous entity bounds. Entity displacement and malformed raw block IDs (currently narrowed to u8) remain unproved.

Chest removal visits only 27 exposed slots and leaves consumed stacks at zero in retained tiles. Furnace/dispenser removal also mutates retained stacks. Chest adjacency lookups now run west/east/north/south before the original cache-reset order; invalidation refreshes adjacency before setting the invalid flag. Container construction applies the nested InventoryLargeChest wrappers in source order, duplicating the other inventory for a null half. Player activation still emits the existing event for later container construction; synchronous inventory identity and fully shared ItemStack aliases remain incomplete.

`world::mchash181::MCHash181` is a literal standalone class adapter, preserving duplicate insertion after updating matches and clearMap's retained live key set. It is not substituted into other owners' entity maps. Active-chunk ordering instead uses ChunkCoordIntPair/Java HashSet: production still uses the inherited `server::chunk_xz_to_int` order where Beta uses x XOR z. Collision-heavy Java 8 tree bins also depend on object identity. Integration must resolve this before claiming multi-chunk World ticking EXACT.

Source comparison artifacts `target/audit5-*.diff` normalize only mapped field/method identifiers. No Beta 1.7.3 crate/verifier, shared oracle dispatch/fixture, entity AI, terrain or renderer was changed. Adjacent production edits are limited to block-owned container removal hooks and container construction. The added flow replay verified inherited code without a production change.

Git was not written. `target/commit-b181-world-blocks5.sh` stages the explicit round-five files and uses the requested author and both trailers in a writable Git environment.

## Round-four changes and validation

The release build `cargo build --release -p worldgen181 -p client` succeeds. Sixteen new independent original-bytecode modes are wired into `tools/verify_world_blocks181.sh`, already invoked by the delimited world-blocks block in `tools/verify181.sh`. Final focused regression: all **48 suites EXACT** (32 existing plus 16 new). See `target/world-blocks4-verify-final.log`; the all-block and container replay results below were also rechecked.

| Check | EXACT bytes | Coverage |
|---|---:|---|
| placement4 | 616,327 | 29 items, six faces, 12 configurations; result, signed counts, blocks/metadata, world/item RNG, events; snow/vines, seeds, doors, beds, signs, hoe, flint |
| placepred4 | 184,140 | 31 block kinds, 110 floors, nine targets, six faces |
| doors4 | 140,616 | Both doors, four facings, 81 neighbor combinations; hinges/notifications/drops |
| harvest4 | 3,456 | Leaves/tall grass/vines, 16 metadata, shears/unarmed drops/statistics |
| shears4 | 8,772 | IDs 0..109, six signed/extreme durability values, destroy statistics/break behavior |
| fire4 | 1,528,320 | 24 vine/plank/netherrack scenes, 80 updates; blocks/scheduling/RNG |
| growth4 | 6,724,608 | Eleven old plant/snow/ice blocks, 16 light/floor/neighbor scenes, 48 updates; cells/drop counts/RNG |
| plates4 | 9,680 | Both plates, all 110 floors, powered/unpowered metadata; placement/support-loss events |
| sounds4 | 5,192 | Every registered block 1..109, step/break names and exact volume/pitch bits |
| emptycache4 | 16,560 | Missing server chunks, rectangle/height boundaries, IDs/meta/light/material/null tiles |
| furnace4 | 714,240 | 12 fuels, six inputs, eight timer/output scenes, 24 real tile ticks; NBT, inventory, signed timers, identity/invalidation, dirty state/facing |
| dispenser4 | 257,700 | Six facings, 16 inventory masks, eight dispenses; slots/RNG, projectile/item construction and events |
| activation4 | 461 | Dispenser/note/chest/both furnaces, valid/invalid tiles, roof obstruction, GUI count/dirty state/RNG/events |
| record4 | 487 | Signed record NBT extrema, reload/factory, both records with valid/invalid tiles and occupied metadata |
| regioncache4 | 30,107 | Shared handles/size deltas across directories, 36 writes/reads, global 256-file flush and explicit cleanup |
| oldformat4 | 378 | Five legacy slots, missing/wrong Data fallback, rename, optional player dirs, deletion and no-op APIs |

Vanilla quirks are retained: ItemRedstone returns true on an unsupported floor without consumption or placement sound. ItemReed replaces vines without offset and only calls placement callbacks if its block survives notifications. The base player edit hook literally returns true; owned block/door/bed/dye/hoe/flint/seed handlers call it. Mushrooms can be placed on an opaque floor in bright light and later fail their stay test. Doors accept two replaceable cells. Plates accept fences; exhaustive support-loss probing also exposed and fixed unsupported ladder metadata zero. Tall grass gives its block when sheared. Shears count successful leaves/web/tall-grass/vine destroy hooks and preserve signed durability behavior. New block sounds use original wood/grass/metal/glass properties. Vine fire values are 15 encouragement and 100 flammability.

RegionFileCache is now process-wide and synchronized. Storage instances share handles and consumed size deltas, lazily create region directories, and flush globally at 256 regions or explicit flush. Storage drop no longer closes regions. Strong references still replace Java SoftReference GC; Java Windows File alias/case semantics and failed-close paths remain unproved. SaveFormatOld implements normal list/read/fallback/rename/delete and loader-directory APIs, including detached missing Data. Its returned Storage still uses McRegion: original SaveHandler's old chunk-directory loader and broader failure adapters remain unported, so the class is not complete.

Jukeboxes now use a typed RecordPlayer tile with lazy factory and chunk NBT loading/saving. Only positive IDs are written. Insertion/ejection call the tile inventory-change API before metadata changes, guard null tiles and retain original float-to-double drop offsets. The existing music section, previously blocked by the missing factory, is EXACT for notes, jukebox events/drop construction and step/break properties. The only adjacent entity verification change handles the new enum variant by explicitly rejecting it, like shared Oracle.worldBytes; record4 checks it independently. No entity class or AI was changed. The filled-bucket handler received the minimal original creative stack-preservation fix; broader bucket behavior remains unverified.

The existing all-block collision/selection section remains EXACT for 10,000 cases. The container click/crafting replay remains EXACT for 66,146 records. No Beta 1.7.3 source/verifier, shared Oracle.java, player-use/hunger engine, terrain generator, GUI or world renderer was edited. The full approximately one-hour verifier is left to the integrator. Round-three terrain-teleporter/container-tile stale-reflection failures remain; full-world saves, lighting grids, multi-chunk ticking/travel/rendering are not claimed EXACT.

Highest-priority remaining work: full World/Chunk ticking; x-XOR-z collision/identity iteration; complete map-feature/game-type propagation; old chunk-directory loading; conversion sector layout/failures; remaining piston/fluid/block/item audits. The current hash helper cannot reproduce Java 8 identity-dependent tree-bin ties; no approximation was substituted. Build/replay logs are under `target/world-blocks4-*.log`. Git was not staged or committed; `target/commit-b181-world-blocks4.sh` has the final explicit file list, requested author and both trailers.


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
| ChunkCache | Not done | cache + emptycache4 EXACT for queried access/light/material/boundaries/lazy tiles and server EmptyChunk reads. Renderer hookup, actual null-provider and error behavior remain. |
| CompressedStreamTools | Done | saves + storage EXACT: gzip semantics, uncompressed reads/writes, safe replacement, stale temporary files, missing file, invalid root, failed target deletion and ignored rename result. |
| Particle | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| WorldInfo | Done | saves: game type, missing/default map features, clone, persistence, player dimension and existing level fields. |
| MCHash | Done | mchash5 EXACT for the literal standalone map adapter: duplicate/null entries, growth/removal/clear, live integer-key set and signed version overflow. Not substituted into other owners' entity maps. |
| Teleporter | Done | teleporter: six flat-world replays, search, construction, high arrival, ocean/platform fallback, portal axes, positions and RNG. |
| ChunkFile | Done | fileclasses EXACT: filename/default coordinates, signed base-36 parsing/overflow, getters and region comparison; converter uses this helper. |
| ChunkLoader | Not done | legacy5/relocation/arrays EXACT for legacy paths, normal/malformed arrays, invalid IDs, overwrite/temp-file failure and relocation. Full entity/tile reconstruction, compressed-byte equality and exception/permission adapters remain. |
| MathHelper | Done | math + helpers EXACT: all public helpers, 20,000 arbitrary float/double bit patterns, integer extrema, negative zero/NaNs, inclusive Random states; prior 65,536-entry sine table remains exact. |
| RegionFile | Done | regions: 36 writes/overwrites, sector reuse and size deltas, Java reads Rust and Rust reads Java. |
| RegionFileCache | Not done | regioncache4 EXACT: process-global synchronized path cache, sharing/size deltas, lazy dirs and global 256-entry/explicit flush. SoftReference GC, Java File aliases and failure paths remain. |
| WorldBlockPositionType | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| MapStorage | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| SaveConverterMcRegion | Not done | fileclasses EXACT helpers now used in scanning/sorting; matching coordinate overflow throws instead of silently skipping. 289-chunk conversion payload/progress/cleanup replay EXACT, level NBT equal except wall-clock LastPlayed; region sector layout differs. Save-handler players=false corrected. Full API/failure/Nether/cache audit remains. |
| NibbleArray | Done | helpers: both constructors, heights 4..8, every nibble and underlying bytes. |
| SaveFormatComparator | Done | fileclasses EXACT: fields/getters, newest timestamp first, signed-long limits, Java UTF-16 comparison including supplementary characters and prefixes. |
| EnumAction | Done | actions: original action ordinals for every registered item. |
| RegionFileChunkBuffer | Done | regions: original compressed-stream close writes checked against Rust compression/write path. |
| IBlockAccess | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| SaveFormatOld | Not done | oldformat4/legacy5 EXACT for normal list/read/fallback/rename/delete and correct old ChunkLoader/provider selection. Broader failure/exception compatibility remains. |
| WorldProvider | Not done | spawn EXACT for grass predicate; brightness tables retained. Terrain provider construction and map-feature propagation remain. |
| IWorldAccess | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| SaveHandler | Not done | savelife EXACT for optional dirs, session checks, level fallback, failed rename/delete cleanup, DIM-1 and data paths. Player-save API, full failures and Java finalizer timing remain. |
| ChunkFilePattern | Done | fileclasses EXACT: original complete-name regex, lowercase/sign/newline/overflow acceptance; filtering is separate from coordinate parsing. |
| ChunkFolderPattern | Done | fileclasses EXACT: one/two lowercase base-36 characters, directories only; converter uses this helper. |
| EmptyChunk | Not done | emptycache4 EXACT for missing-chunk cache IDs, metadata, light and null tiles. Full mutation/entity/save surface remains unaudited. |
| IChunkProvider | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ISaveFormat | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ISaveHandler | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| NextTickListEntry | Not done | nextticks EXACT for tested hash/equality/signed comparison and global-counter queues, including overflow and two interleaved worlds. Queue representation is tuple-based; direct mutable-entry/equal-ID tie semantics still need a class-level audit. |
| WorldProviderHell | Not done | spawn EXACT for nonair, nonbedrock, opaque spawn predicate; lights brightness EXACT. Full provider generation remains with terrain owner. |
| WorldProviderSky | Not done | spawn EXACT for material movement-blocking predicate. Live Sky dimension/provider integration remains. |
| BlockVine | Not done | placement4/fire4/harvest4 EXACT for tested placement, fuel and shearing; prior plants/geometry/drops/visual/colors retained. Full inherited class audit remains. |
| BlockChest | Not done | chests/chestlife/visual/textures/activation4/chestopen5/removal5 EXACT for tested facing, inventory/cache/lid, nested/null-neighbor wrappers and scattering. Synchronous activation inventory identity and stack-alias semantics remain. |
| BlockStem | Done | plants/geometry/drops/items + visual/colors EXACT: growth, fruit, fertilizer, seed drops, bounds, all metadata colors/textures and neighboring-fruit direction priority. |
| TileEntityChest | Done | chests + chestlife EXACT: object identity, 36 initial slots/27 loaded slots, signed counts, clamping/splitting including negative amounts, NBT inventory, cached metadata/block, reset/invalidate/validate, adjacency, lids, viewers, sounds/events. Renderer consumes exported state. |
| ItemDoor | Done | registry/placement4/placepred4/doors4 EXACT for properties, both edit cells, replaceable cells, yaw/hinge priorities, notifications and consumption. |
| ItemDye | Not done | items/mushrooms/placement4 EXACT for owned plant/huge-mushroom hooks and base edit check. Entity-facing/sheep behavior remains with entities owner. |
| BlockFluid | Not done | fluids/sides/flow5 EXACT for random display, light/faces and tested velocity-vector calculations. Full dynamic/stationary tick and inherited audit remains. |
| BlockPane | Done | definitions/geometry/drops/visual/sides EXACT for both pane IDs, connection/collision/selection, edge/base textures, opacity/render type and same-ID visibility. Renderer must consume exported APIs. |
| BlockDispenser | Not done | dispenser4/activation4/removal5 EXACT for selection/consumption/RNG, projectile construction/events, null activation and retained-inventory scattering. Full scheduled power/cache audit remains. |
| Block | Not done | definitions/registry/geometry/drops cover defaults and new block registration. Base render/light/item-use hooks not fully ported. |
| BlockPistonBase | Not done | piston5 EXACT for both types/all faces, real extension/retraction, 0..13 pushes, immovable/container/new blocks and height limits; vine mobility corrected. Full textures/placement/callback/entity-collision audit remains. |
| BlockMushroom | Done | plants + mushrooms + visual EXACT: density/spread, placement, bonemeal, huge generator checks/caps/stems, failed restoration, metadata and original RNG state. |
| FoodStats | Done | use/useticks/foodstats EXACT: NBT/defaults, signed overflow and float/NaN arithmetic, exhaustion/saturation depletion, 80-tick healing/starvation and difficulty thresholds. |
| ItemFood | Done | registry/actions/use/useticks EXACT: all 15 foods, hunger/saturation admission, 32-tick use, effects/RNG/sounds/particles, potion IDs/durations/chances and completion consumption. General player potion engine remains with entities owner. |
| BlockFenceGate | Done | geometry/items/visual/fileclasses EXACT for collision, solid-floor and replaceable-target placement, yaw/activation/open-close metadata, opacity/render type. Renderer hookup remains with integrator. |
| ItemBow | Done | registry/actions/use/useticks EXACT: hold, release -20..80 elapsed ticks, threshold, speed, full-charge critical flag, arrow construction floats/RNG, sound and arrow consumption. Projectile simulation beyond construction needs entities coverage. |
| BlockFurnace | Not done | furnace4/activation4/removal5 EXACT for tested ticks/swaps/facing/identity/dirty state/null activation and scattering. Complete cache/inherited audit remains. |
| Item | Not done | registry/actions EXACT for definitions, use action and duration. Use lifecycle and edit permission support incomplete. |
| TileEntityPiston | Not done | piston5 EXACT for real progress/update/invalidation, NBT progress and finite/nonfinite interpolation/collision. Entity displacement, raw block-ID narrowing and malformed NBT remain. |
| BlockSilverfish | Done | definitions/geometry/drops + silverfish + visual EXACT: metadata textures, conversion predicates, player-destroy spawn constructor/position/RNG and all 20 explosion particles. Silverfish AI/model/sounds belong to entities owner. |
| ItemBlock | Not done | placement4/placepred4 EXACT for tested placement, replaceability, survival guard and base edit hook. Full every-block/height/collision replay remains. |
| ItemStack | Not done | nullable NBT proof retained; use identity tokens now cancel identical replacements while permitting same-object hotbar aliases (useticks EXACT). Copy-based inventory still lacks fully shared mutation of aliased Java stacks; full item delegation/load audit remains. |
| TileEntity | Not done | chestlife/chestopen5 prove tested metadata/block caches, resets and invalidate/validate ordering. Full factory/persistence surface remains. |
| BlockTallGrass | Not done | growth4/harvest4 EXACT for tested stay/growth/drop/shears hooks across all metadata. Full inherited class audit remains. |
| BlockJukeBox | Not done | musicevents/record4 EXACT for insertion/ejection/events/drop construction, typed lazy tiles and null cases. Multiplayer guard and complete removal/failure replay remain. |
| BlockMushroomCap | Done | definitions/geometry/drops + visual/mushrooms EXACT: every metadata/side texture, drops and huge cap generation. |
| BlockMelon | Done | definitions/geometry/drops + visual EXACT: render defaults, top/side textures, quantity RNG and metadata drops. |
| BlockPistonMoving | Not done | piston5/activation4 EXACT for tested tile collision and null-tile activation. Full bounds/selection/removal and invalid tile/NBT audit remains. |
| ItemVine | Not done | placement4/geometry/items/registry EXACT for tested side placement/properties/shearing. Full metadata/target boundary audit remains. |
| BlockGrass | Not done | growth4 EXACT for tested light/support/neighbor spread/decay and RNG. Full inherited class audit remains. |
| BlockStoneBrick | Done | definitions/geometry/drops + visual EXACT: all metadata/side textures, subtype drops and original creative variants. |
| ItemReed | Done | registry/placement4/placepred4 EXACT for all three items, snow/vine replacement, base permission, signed count, surviving-ID callbacks, sound and consumption. |
| ItemHoe | Done | registry/placement4 EXACT for all hoes, original dirt/grass precedence, occupied/side checks, base permission, sound/replacement/damage. |
| BlockRedstoneWire | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemFlintAndSteel | Done | registry/placement4 EXACT for offset, base permission, air/occupied target, original item-RNG sound, fire placement and damage. |
| BlockRedstoneRepeater | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemBucket | Not done | Minimal original filled-bucket creative preservation fix ported. Full ray trace/collection/Nether/cow-milk/permission replay remains. |
| ItemPainting | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemSword | Done | registry/actions/use/useticks/blocking EXACT: properties, use action/duration, hold/cancellation, blocking and armor damage/remainder/exhaustion with bypass flag. Existing environmental attack callers still need DamageSource mapping. |
| BlockContainer | Not done | direct lazy tile factory path ported; full container class coverage pending. |
| BlockLeaves | Not done | growth4/harvest4 EXACT for tested decay and all-metadata shears drops; prior plants/visual/colors retained. Full inherited class audit remains. |
| Material | Done | definitions: all 29 material defaults; registry also checks harvest flags and tool behavior. |
| BlockNote | Not done | musicevents/activation4 EXACT for notes/instruments/events and null activation. Full NBT/inherited edge audit remains. |
| ItemRedstone | Done | registry/placement4 EXACT for snow/offset/occupied target, base permission, normal floor, true-without-placement return, consumption and absence of placement sound. |
| StepSound | Done | sounds4 EXACT for every registered block 1..109 step/break names and volume/pitch bits, including glass and metal; musicevents retained. |
| BlockStairs | Not done | geometry/drops EXACT including new stairs and model metadata drop quirk. Rendering integration pending. |
| ItemBed | Not done | placement4 EXACT for tested face/yaw/floor/occupied placement; both base edit hooks added. Full paired-cell boundary/collision audit remains. |
| MaterialWeb | Done | definitions: separate web material with movement disabled while isSolid remains true. |
| ItemSign | Not done | placement4 EXACT for tested wall/floor placement and suppressed GUI editor. Full permission/NBT/entity-facing audit remains. |
| BlockPistonExtension | Not done | piston5 proves extension/retraction participation in all six directions. Complete standalone bounds/selection/removal callback audit remains. |
| TileEntityDispenser | Not done | dispenser4/removal5 EXACT for selection/consumption/RNG and retained inventory after removal. Full generic inventory/NBT/cache/failure audit remains. |
| TileEntityFurnace | Not done | furnace4/removal5 EXACT for signed NBT timers/fuel/output, ticks/identity/invalidation/dirty state and retained inventory after removal. Full inventory/cache/failure audit remains. |
| ItemShears | Done | registry/shears4/harvest4 EXACT for strengths/harvestability, all block destroy costs/statistics, signed durability extrema and leaves/tall-grass/vine drops. |
| ItemSlab | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockFence | Done | geometry: original fence/gate connection predicate for IDs 0..109; existing 10,000-case block geometry replay. |
| ItemLog | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockDoor | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockFire | Not done | fire4 EXACT for vine fuel and tested 80-tick clear-weather replays. Full weather/provider/display/class audit remains. |
| BlockFarmland | Done | plants: water, metadata, crops and both new stems; stem-aware walking hook ported. |
| BlockFlower | Not done | placepred4/growth4 EXACT for tested placement/stay behavior, including mushroom placement versus light-sensitive stay. Full inherited audit remains. |
| BlockRedstoneOre | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemSeeds | Done | registry/items/placement4 EXACT for all three seeds, sides, soil/occupied cells, both base edit hooks, crop/stem IDs and consumption. |
| ItemSoup | Done | actions/use/useticks EXACT: 32-tick completion, hunger/saturation and bowl replacement. |
| PistonBlockTextures | Done | piston5 EXACT for all 24 original constants (opposite side and X/Y/Z offsets), plus actual six-direction extension/retraction and interpolation. |
| BlockCake | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockCloth | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockPressurePlate | Not done | plates4 EXACT for all floors and both metadata states, fence support and removal; existing geometry EXACT. Full entity-trigger/power/tick audit remains. |
| BlockStep | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockTrapDoor | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockWeb | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemCloth | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemMap | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| TileEntitySign | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockBed | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockCactus | Not done | placepred4/growth4 EXACT for tested floor/neighbor/stay/growth and RNG. Full inherited class audit remains. |
| BlockMobSpawner | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockReed | Not done | placement4/placepred4/growth4 EXACT for tested water/support/placement/growth and RNG. Full inherited class audit remains. |
| BlockSign | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockSnowBlock | Not done | growth4 plus existing geometry/drops for tested light/snow behavior. Full inherited class audit remains. |
| ItemLeaves | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemRecord | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| TileEntityRecordPlayer | Done | record4 EXACT for signed/extreme NBT, omitted nonpositive Record, reload/chunk factory; musicevents EXACT for live records. |
