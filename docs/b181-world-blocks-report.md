# Beta 1.8.1 world-blocks handoff

## Round-seven changes and validation

Seven additional block classes are Done: **BlockFurnace, BlockContainer, BlockNote, BlockRedstoneOre, BlockFlower, BlockWeb and BlockSnowBlock**. The class table below records the original-bytecode proofs. The larger tile classes remain Not done: the new inventory proof deliberately does not hide their delegated ItemStack exception/alias gaps. Existing integration credits remain authoritative.

| Added check | EXACT bytes | Coverage |
|---|---:|---|
| furnace7 | 61,322 | Both furnaces: 1,024 default-facing scenes; remote/local and valid/invalid retained tiles; repeated factory callbacks; 76 signed/extreme yaw cases; 1,280 particle callbacks at signed/large/boundary coordinates; 48 active/inactive, immediate/deferred, local/remote swaps with retained inventories, identity, loaded/pending lists and caches |
| inventory7 | 210,517 | 800 real furnace/dispenser set/split/clear/dirty operations; byte-exact uncompressed NBT, signed counts/timers, duplicate/invalid slots and unregistered positive item IDs; 48 identity/distance/NaN/infinity interaction cases; 6,561 scaled-counter overflow cases; every registered fuel; 1,620 remote/local smelting/tick cases with signed/extreme input counts and cook times |
| container7 | 44,710 | Ten original factories and 3,000 direct superclass callbacks on the real subclasses (MethodHandles.findSpecial calls the original BlockContainer bytecode): addition, removal, lazy lookup, invalid/retained identity and chest event forwarding, both remote states and signed/extreme event/value inputs |
| machines7 | 15,639,484 | Unmodified ContainerOracle.tiles: 1,000 ticks of twelve furnaces and a dispenser, entity state and ordered events each frame, counters/inventories, world fingerprint each frame and byte-exact final world |
| ore7 | 515,072 | 2,560 direct redstone-ore click/walk/activate/update/display scenes: both variants, both remote states, metadata, all 64 opaque-neighbor masks, separate callback/world RNG and exact particle events |
| thin7 | 3,639,374 | 28,160 flower stay/neighbor/tick cases across all registered floors, all 16 light levels and remote states; 512 snow-block light/metadata cases; 40 web contacts (pig, spider, cave spider, item) preserving web/fall/motion fields; 90 note events with signed/extreme instrument and note integers |

The first machine divergence was at byte 1,057, on its first tile tick. Original Chunk.setBlockID and setBlockIDWithMetadata perform a container lookup/cache refresh after onBlockAdded. While scanning tiles, a fresh factory tile is queued and an invalid old tile can make that lookup return null. Vanilla then passes *chunk-local* x/z to World.setBlockTileEntity, overwriting unrelated containers in the origin chunk. Rust omitted that entire tail and used an ensure-existing furnace factory, retaining three inventories that Java clears. The literal factory and setter-tail ports make the complete machine replay EXACT. Both setters also preserve their original remote onBlockAdded guard. This is a concrete World/Chunk ticking root cause, beyond the previously credited normal save/light cases; no active-chunk iteration or terrain ordering was changed.

Furnaces now create fresh tiles on every addition, retain displaced objects, guard default direction and GUI activation remotely, tick with the actual remote flag, and invoke TileEntity.onInventoryChanged after a state transition so the cached metadata is refreshed. Cook/input/output/fuel arithmetic wraps like Java; scaled burn-time division preserves MIN_VALUE / -1. New machine inventory APIs preserve the differing dirty hooks (furnace setters/decrements do not mark dirty; dispenser operations do). Original dcmpl/ifle admits a NaN interaction distance; the literal predicate is verified by both original tile classes (bytecode: target/TileEntityFurnace7-bytecode.txt).

BlockContainer's fresh factory path and tile-event forwarding are production APIs. Note and jukebox additions now use that path. Chest events forward to the actual tile override. Note event arithmetic wraps the integer subtraction before converting to double. Redstone ore now runs its particle/lighting callback on right-click and still returns false; click and walk use the same callback. The inherited flower, snow-block and web implementations required no behavioral rewrite; their full class overrides are now audited and covered by direct callbacks plus the existing tables, placement, drops and all-block geometry proofs.

TileEntityFurnace and TileEntityDispenser stay Not done. The original ItemStack nullable-NBT helper throws ArrayIndexOutOfBoundsException for negative short item IDs; the existing owner API returns null. Item classes are owned by the entities worker this round and were not edited. Fully shared retained ItemStack aliases, arbitrary malformed NBT getter/cast failures, and detached dispenser Random ownership also remain unproved. The new inventory helpers and counter/cache corrections are exact for the documented fixtures; they do not establish full container-session/player simulation compatibility.

Source diffs for the nine audited block/tile classes are in target/audit7-*.diff. New Java modes call original bytecode; no shared oracle dispatch or fixture was changed. Adjacent entity-namespace edits only route block callbacks, expose existing contact/walk callbacks to the checks, and delegate furnace ticking to its block-owned implementation. No entity AI, item class, player-use/hunger engine, terrain generator, client renderer, Beta 1.7.3 source or verifier was edited.

Final focused regression: **all 73 suites EXACT** (67 retained plus six new), recorded in `target/world-blocks7-verify-final.log`. `cargo build --release -p worldgen181 -p client` succeeds (`target/world-blocks7-build-final.log`). The delimited world-blocks section of `tools/verify181.sh` already calls the extended focused runner; no other verify181 section was edited. Its formerly failing container machine section is now EXACT through the unchanged `containertiles` fixture. The approximately one-hour full verifier remains the integrator's run. Broader affected regression is also **EXACT**: 66,146 container click/crafting records, the 1,000-tick machine replay, 10,000 collision/ray cases, all 96 step/break sound properties plus notes/records, 89,600 ambient block callbacks with ordered events/RNG, and all 21 redstone scripts (`target/world-blocks7-regressions-final.log`). `git diff --check`, shell syntax validation and explicit commit-file/protected-path checks pass. Git remains unstaged/uncommitted; target/commit-b181-world-blocks7.sh contains the explicit changed-file list, requested author and both trailers.

Next owned work: finish the tile NBT/alias and retained dispenser RNG gaps with the entities owner's ItemStack API; then complete the remaining redstone wire/repeater, moving-piston/extension, pressure-plate, door/trapdoor and other block callbacks identified in the class table. ChunkLoader and SaveConverterMcRegion retain their save/failure gaps. No new whole-world tick, terrain, player or renderer coverage is claimed.

## Previous round-six handoff

Round six was resumed from the existing uncommitted edits after the interrupted run. The integrator's credits in `../codex/notes/world-blocks-next.md` are authoritative: those classes were not re-audited or reworked during the continuation. The remaining priority classes BlockDispenser, Block and BlockPistonBase now have class-level original-bytecode checks. Other incomplete classes remain identified below.

## Resumed round-six changes and validation

The baseline continuation reran all 64 pre-existing focused suites: **EXACT** (`target/world-blocks6-resume-baseline.log`). Three additional suites bring the focused runner to **67 suites**. The new probes use Mojang Java 8u51 and call the original Beta 1.8.1 bytecode; no shared oracle fixture or dispatch was changed.

| Added check | EXACT bytes | Coverage |
|---|---:|---|
| dispenser6 | 141,336 | 2,670 cases: default facing for all support masks/metadata/remote states; valid/invalid and remote activation; repeated addition with retained old tile/inventory; signed/extreme yaw; both power levels, neighbor scheduling and actual update callbacks with valid/invalid tiles |
| base6 | 570,024 | 630 access cases: emitted-light minimum and exact callback arguments for brightness/packed light, normal-cube ambient occlusion, default white color; 3,840 base harvest cases with all metadata, remote/damage-disabled states, exhaustion limits/NaN/infinity, statistics, drops/events/RNG |
| piston6 | 828,142 | 6,904 cases: both piston types, horizontal/vertical placement thresholds and signed/extreme yaw; remote guards, ignored neighbor updates, metadata sentinel, all twelve power-query positions, direct added/neighbor callbacks, moving-tile fields and ordered events/RNG |

BlockDispenser now creates a fresh factory tile on each addition, retains the previous tile's validity and inventory, suppresses default facing changes in remote worlds, and suppresses remote inventory activation. Its update/neighbor methods still run their original power logic without an invented remote guard. Existing dispenser4/removal5/textures/definitions proofs complete the class methods.

Block's access-independent helpers in `block::visual181` expose base brightness, packed-light minimum, ambient occlusion and white default color for renderer integration. Subclass dispatch remains with the existing callers. Base harvest suppresses exhaustion remotely while still recording the mining statistic, as EntityPlayer.addExhaustion does in the original. The existing exhaustion API and item implementations were not changed. Definitions/registry/visual/sides/drops6 plus the all-block collision/ray replay cover the other base surfaces.

BlockPistonBase now suppresses state updates remotely at every entry point; onBlockAdded also avoids a remote tile lookup. Existing piston5/push6/visual/sides/geometry proofs cover extension, retraction, push limits, entity displacement, bounds and textures. No credited Chunk/ChunkCache, fluid, vine, chest, terrain, whole-world ticking, renderer or item-class implementation was reworked during the continuation.

The broader `pistons_chunk_nbt` replay exposed a fixture adapter omission from the interrupted unload changes: compare181's `chunkreload` replaced its chunk map before preserving tiles still referenced by the update list. The adapter now calls the same tile-detachment API as the production provider immediately before discarding the old chunk. Old moving tiles retain their 0.5 progress while the newly decoded tiles start at the saved lastProgress value of 0.0; the complete replay is EXACT. No original-bytecode output was changed or filtered.

Final regression: **all 67 focused suites EXACT**, plus **all 21 redstone scripts**, **10,000 collision/ray cases**, music/sound events and **66,146 container click/crafting records EXACT**. Results are in `target/world-blocks6-resume-verify-final.log` and `target/world-blocks6-resume-regressions.log`. The separate 1,000-tick machine replay is not EXACT: it first differs at byte 1,057 (Rust 15,723,484 bytes, Java 15,639,484 bytes). Removing the continuation's dispenser factory replacement produces byte-identical Rust machine output (`target/world-blocks6-resume-container-baseline.log`); the other continuation guards affect only remote worlds, while this fixture is local. This failure predates the continuation. Its broader furnace/dispenser/tile/world-event discrepancy remains an integration follow-up, distinct from the focused class proofs and container click checks. The release build succeeds (`target/world-blocks6-resume-build-final.log`). Only the delimited world-blocks section of verify181.sh is extended through its existing focused runner; the full approximately one-hour whole-game verifier remains the integrator's run. The explicit file list in `target/commit-b181-world-blocks6.sh` includes the interrupted edits and continuation, requested identity and both trailers. Git remains unstaged and uncommitted.

Next uncredited work: resolve the full machine tile replay, finish BlockContainer/TileEntity factory and cache coverage, then the remaining furnace, redstone-wire/repeater and block-specific callbacks. ChunkLoader and SaveConverterMcRegion retain the save/failure gaps listed in the per-class table. Item classes remain owned by the entities worker; credited whole-world, chunk and rendering work remains with integration.

## Round-six changes and validation

Final regression: **all 64 focused suites EXACT** (54 existing plus ten new), recorded in `target/world-blocks6-verify-final.log`. `cargo build --release -p worldgen181 -p client` succeeds (`target/world-blocks6-build-final.log`). The existing `verify181.sh` all-block collision/ray section remains EXACT for 10,000 cases, the container click/crafting replay remains EXACT for 66,146 records, and its original `musicevents` section remains EXACT (`target/world-blocks6-blocks.log`, `target/world-blocks6-containers.log`, `target/world-blocks6-musicevents.log`). The delimited world-blocks section already invokes the extended runner from `tools/verify181.sh`; shared oracle dispatch and fixtures remain untouched. `git diff --check` and shell syntax validation of the commit script pass.

| Added check | EXACT bytes | Coverage |
|---|---:|---|
| slabs6 | 180,224 | 2,048 additions: both slab instances, four lower block types and every pair of metadata values |
| trapdoors6 | 1,570,792 | Both remote states, all metadata, 110 support IDs, four changed-block IDs and 32 direct powered transitions; blocks, drop entities, sounds/events and RNG |
| doors6 | 471,488 | 4,096 cases: both doors, remote states, metadata, missing halves/floors/power, neighbor/powered/activation callbacks |
| attached6 | 201,984 | 2,048 button/lever neighbor cases: metadata, 32 support masks, remote states, drop/sound/RNG results |
| drops6 | 3,055,488 | 27,904 actual drop callbacks: IDs 1..109, metadata, remote states, eight chances including NaN/infinity; complete item construction and world RNG |
| conversion6 | 10,912 | Eight controlled Alpha conversions; both dimensions, empty worlds, duplicate coordinates, existing region payloads, process-global foreign cache handles, progress/cleanup, WorldInfo fields, decompressed payloads and sector offsets |
| push6 | 2,299,392 | 576 piston displacement scenes: six directions, extension/retraction, six stored blocks, four tile ticks, nine entities including no-clip, obstacles; exact positions/motions/bounds/collision flags, events and RNG |
| unload6 | 2,560 | Six note/jukebox/chest worlds: repeated unload, queued payload mutation through the chunk, one final update, reload and subsequent unload; list order/queue counts, retained validity, payloads, chest animation, sounds and RNG |
| music6 | 2,041,044 | 14,400 note cases plus 384 jukebox cases; signed pitch extrema, metadata, remote states, floors/roofs, activation/click/trigger/redstone/direct-play callbacks, set/eject/remove; cached metadata, retained tile state, entities/events/RNG |
| materials6 | 2,880 | Both cloth dye mappings, cloth/slab textures with signed/extreme metadata and sides, all six original slab names |

BlockStep.onBlockAdded merges matching lower slabs even when the newly added block is a double slab. The constructor/static opacity discrepancy and original shouldSideBeRendered behavior remain preserved by the existing definitions/sides checks. Wool and slab texture methods now expose literal signed-input helpers; the renderer's existing bounded texture API calls those helpers. No item class was edited.

Powered doors and trapdoors emit the original auxiliary sound. Trapdoor neighbor processing continues its power check after support loss removes the block. Door drops are suppressed in remote worlds. Button/lever support processing preserves short-circuit candidate queries followed by the original repeated support queries; malformed orientations are not replaced with guessed attachments. Base drops suppress remote execution; bed heads bypass even quantity/chance RNG. Redstone wire drops item 331. Original bytecode uses `fcmpl`/`ifle` for chance, so NaN admits the drop, despite the decompiler displaying a misleading <= expression (`target/Block6-bytecode.txt`).

Chunk.onChunkUnload marks tiles unloaded without invalidating them. World removes the queued identities after the tile update pass and before pending insertions. Queued tiles retain a shared payload with a retained chunk; a provider discarding the chunk detaches its payload after saving. Pending/invalid tile processing checks chunk existence before touching chunk storage, avoiding unintended chunk generation. Complete entity unloading, Java map iteration/identity and arbitrary retained external tile references remain unproved.

BlockNote activation/click and jukebox set/eject callbacks preserve multiplayer guards. Note pitch arithmetic promotes the signed byte to int before modulo, then calls TileEntity.onInventoryChanged to refresh cached metadata. The new direct jukebox setter preserves the misleading MCP ejectRecord name's actual insertion semantics; the existing item wrapper continues to emit its own music event. No player-use engine was changed.

Conversion now uses RegionFileCache's process-global handles and clears the cache after each dimension, including an empty list. It preserves filesystem enumeration order before the original stable region sort. This proves the eight controlled layouts in conversion6. Java/Rust compressors, broad historical region sector layouts, directory alias/case/permissions, filesystem exception adapters and SoftReference reclamation remain unproved; SaveConverterMcRegion is not marked complete.

Source differences for 122 world/block/tile classes are in `target/audit6-*.diff`, with `target/audit6-summary.txt`. Normalization applies mapped identifiers while retaining Java keywords and primitive types. The inherited World active-chunk hash still uses the old packed coordinate hash where Beta uses x XOR z; Java 8 tree-bin ties require actual object identity. This remains an integration root cause, with no substitute iteration approximation added. Terrain, entity AI, player item use/hunger, all item classes, Beta 1.7.3 code/verifiers, renderer files and shared Oracle.java/WorldEventOracle/Mode_entities181 fixtures were untouched. Adjacent entity-namespace edits only delegate block callbacks and the Chunk tile-unload hook.

The full approximately one-hour verifier remains the integrator's run. Per-class statuses below distinguish complete block overrides from tested surfaces of incomplete larger classes. Git was not staged or committed. `target/commit-b181-world-blocks6.sh` stages the explicit round-six files and uses the requested author and both trailers.

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
| World | Done | Integrator credit: whole-world tick/new-world replays EXACT; existing focused lights/weather/spawn/nextticks/frost and unload6 proofs retained. |
| Chunk | Done | Integrator credit: light/array/cache/chest checks, terrain byte-identical saves for six seeds and redstone replays EXACT; unload6 retained. |
| ChunkCache | Done | Integrator credit: cache/emptycache4 plus renderer exact chunk-mesh access checks. |
| CompressedStreamTools | Done | saves + storage EXACT: gzip semantics, uncompressed reads/writes, safe replacement, stale temporary files, missing file, invalid root, failed target deletion and ignored rename result. |
| Particle | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| WorldInfo | Done | saves: game type, missing/default map features, clone, persistence, player dimension and existing level fields. |
| MCHash | Done | mchash5 EXACT for the literal standalone map adapter: duplicate/null entries, growth/removal/clear, live integer-key set and signed version overflow. Not substituted into other owners' entity maps. |
| Teleporter | Done | teleporter: six flat-world replays, search, construction, high arrival, ocean/platform fallback, portal axes, positions and RNG. |
| ChunkFile | Done | fileclasses EXACT: filename/default coordinates, signed base-36 parsing/overflow, getters and region comparison; converter uses this helper. |
| ChunkLoader | Not done | legacy5/relocation/arrays EXACT for legacy paths, normal/malformed arrays, invalid IDs, overwrite/temp-file failure and relocation. Full entity/tile reconstruction, compressed-byte equality and exception/permission adapters remain. |
| MathHelper | Done | math + helpers EXACT: all public helpers, 20,000 arbitrary float/double bit patterns, integer extrema, negative zero/NaNs, inclusive Random states; prior 65,536-entry sine table remains exact. |
| RegionFile | Done | regions: 36 writes/overwrites, sector reuse and size deltas, Java reads Rust and Rust reads Java. |
| RegionFileCache | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| WorldBlockPositionType | Out of scope | Integrator note: multiplayer-only; this port targets singleplayer. |
| MapStorage | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| SaveConverterMcRegion | Not done | conversion6 EXACT for eight controlled worlds: global region-cache reuse/flush, empty dimensions, duplicate coordinates, existing payloads, progress/cleanup/WorldInfo/payloads/sector offsets. Larger layouts, compression/file alias/failure compatibility remain. |
| NibbleArray | Done | helpers: both constructors, heights 4..8, every nibble and underlying bytes. |
| SaveFormatComparator | Done | fileclasses EXACT: fields/getters, newest timestamp first, signed-long limits, Java UTF-16 comparison including supplementary characters and prefixes. |
| EnumAction | Done | actions: original action ordinals for every registered item. |
| RegionFileChunkBuffer | Done | regions: original compressed-stream close writes checked against Rust compression/write path. |
| IBlockAccess | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| SaveFormatOld | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| WorldProvider | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| IWorldAccess | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| SaveHandler | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| ChunkFilePattern | Done | fileclasses EXACT: original complete-name regex, lowercase/sign/newline/overflow acceptance; filtering is separate from coordinate parsing. |
| ChunkFolderPattern | Done | fileclasses EXACT: one/two lowercase base-36 characters, directories only; converter uses this helper. |
| EmptyChunk | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| IChunkProvider | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| ISaveFormat | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| ISaveHandler | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| NextTickListEntry | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| WorldProviderHell | Done | Integrator credit: whole-world tick/new-world replays EXACT; spawn/lights retained. |
| WorldProviderSky | Done | Integrator credit: existing focused and whole-game checks accepted; remaining finalizer/GC/file-alias/interface-only scope resolved by integration. |
| BlockVine | Done | Integrator credit: vine181 EXACT; existing placement/fire/harvest/visual checks retained. No continuation implementation edits. |
| BlockChest | Done | Integrator credit: chests/chestlife/activation and player-gui chest hooks cover every method; chestopen5/removal5 retained. |
| BlockStem | Done | plants/geometry/drops/items + visual/colors EXACT: growth, fruit, fertilizer, seed drops, bounds, all metadata colors/textures and neighboring-fruit direction priority. |
| TileEntityChest | Done | chests + chestlife EXACT: object identity, 36 initial slots/27 loaded slots, signed counts, clamping/splitting including negative amounts, NBT inventory, cached metadata/block, reset/invalidate/validate, adjacency, lids, viewers, sounds/events. Renderer consumes exported state. |
| ItemDoor | Done | registry/placement4/placepred4/doors4 EXACT for properties, both edit cells, replaceable cells, yaw/hinge priorities, notifications and consumption. |
| ItemDye | Not done | items/mushrooms/placement4 EXACT for owned plant/huge-mushroom hooks and base edit check. Entity-facing/sheep behavior remains with entities owner. |
| BlockFluid | Done | Integrator credit: whole-world tick/new-world replays EXACT; fluids/sides/flow5 retained. |
| BlockPane | Done | definitions/geometry/drops/visual/sides EXACT for both pane IDs, connection/collision/selection, edge/base textures, opacity/render type and same-ID visibility. Renderer must consume exported APIs. |
| BlockDispenser | Done | dispenser6 + dispenser4/activation4/removal5/definitions/textures EXACT: all class overrides, fresh factory tiles, facing/yaw, remote and null guards, power scheduling/ticks, projectiles, drops and RNG. |
| Block | Done | base6 + definitions/registry/visual/colors/sides/drops6 and all-block collision/ray EXACT: new lightmap/AO/color methods, harvest exhaustion/statistics, registered defaults and base geometry/drop behavior. Item delegates remain with the entities owner. |
| BlockPistonBase | Done | piston6 + piston5/push6/definitions/visual/sides/geometry EXACT: placement, every power query, remote/ignore guards, callbacks, extension/retraction/limits, texture/bounds and entity displacement. No changed or added class override remains unported. |
| BlockMushroom | Done | plants + mushrooms + visual EXACT: density/spread, placement, bonemeal, huge generator checks/caps/stems, failed restoration, metadata and original RNG state. |
| FoodStats | Done | use/useticks/foodstats EXACT: NBT/defaults, signed overflow and float/NaN arithmetic, exhaustion/saturation depletion, 80-tick healing/starvation and difficulty thresholds. |
| ItemFood | Done | registry/actions/use/useticks EXACT: all 15 foods, hunger/saturation admission, 32-tick use, effects/RNG/sounds/particles, potion IDs/durations/chances and completion consumption. General player potion engine remains with entities owner. |
| BlockFenceGate | Done | geometry/items/visual/fileclasses EXACT for collision, solid-floor and replaceable-target placement, yaw/activation/open-close metadata, opacity/render type. Renderer hookup remains with integrator. |
| ItemBow | Done | registry/actions/use/useticks EXACT: hold, release -20..80 elapsed ticks, threshold, speed, full-charge critical flag, arrow construction floats/RNG, sound and arrow consumption. Projectile simulation beyond construction needs entities coverage. |
| BlockFurnace | Done | furnace7 + furnace4/activation4/removal5/definitions/textures/drops6 and machines7 EXACT: every override, factory/retained identity, facing/yaw, remote/null guards, swaps/caches, particles and removal. No changed or added class override remains unported. |
| Item | Not done | registry/actions EXACT for definitions, use action and duration. Use lifecycle and edit permission support incomplete. |
| TileEntityPiston | Not done | piston5 plus push6 EXACT for tested NBT/interpolation/collision, nonfinite comparison behavior and 576 actual entity displacement scenes. Raw stored IDs still narrow to u8; malformed state, huge/nonfinite scans and full world lifecycle remain. |
| BlockSilverfish | Done | definitions/geometry/drops + silverfish + visual EXACT: metadata textures, conversion predicates, player-destroy spawn constructor/position/RNG and all 20 explosion particles. Silverfish AI/model/sounds belong to entities owner. |
| ItemBlock | Not done | placement4/placepred4 EXACT for tested placement, replaceability, survival guard and base edit hook. Full every-block/height/collision replay remains. |
| ItemStack | Not done | nullable NBT proof retained; use identity tokens now cancel identical replacements while permitting same-object hotbar aliases (useticks EXACT). Copy-based inventory still lacks fully shared mutation of aliased Java stacks; full item delegation/load audit remains. |
| TileEntity | Not done | container7 + inventory7/chestlife/chestopen5/unload6/music6 cover generic event forwarding, tested caches, identity/invalidation and dirty/unload lifecycle. Full factory/persistence/failure and retained external identity surface remains. |
| BlockTallGrass | Done | Integrator credit: whole-world tick/new-world replays EXACT; growth4/harvest4/visual/colors retained. |
| BlockJukeBox | Not done | music6 plus musicevents/record4 EXACT for remote/local activation, set/eject/remove, typed lazy tiles/null cases, cache refresh, retained tile state and drops/events/RNG. Raw out-of-range record drop IDs narrow in the shared ItemStack; exception/alias edges remain. |
| BlockMushroomCap | Done | definitions/geometry/drops + visual/mushrooms EXACT: every metadata/side texture, drops and huge cap generation. |
| BlockMelon | Done | definitions/geometry/drops + visual EXACT: render defaults, top/side textures, quantity RNG and metadata drops. |
| BlockPistonMoving | Not done | piston5/activation4 EXACT for tested tile collision and null-tile activation. Full bounds/selection/removal and invalid tile/NBT audit remains. |
| ItemVine | Not done | placement4/geometry/items/registry EXACT for tested side placement/properties/shearing. Full metadata/target boundary audit remains. |
| BlockGrass | Done | Integrator credit: whole-world tick/new-world replays EXACT; growth4/visual/colors retained. |
| BlockStoneBrick | Done | definitions/geometry/drops + visual EXACT: all metadata/side textures, subtype drops and original creative variants. |
| ItemReed | Done | registry/placement4/placepred4 EXACT for all three items, snow/vine replacement, base permission, signed count, surviving-ID callbacks, sound and consumption. |
| ItemHoe | Done | registry/placement4 EXACT for all hoes, original dirt/grass precedence, occupied/side checks, base permission, sound/replacement/damage. |
| BlockRedstoneWire | Not done | drops6 fixes and proves item 331 across all metadata/chance/remote cases. Full power propagation, slope and scheduled callback audit/replay remain. |
| ItemFlintAndSteel | Done | registry/placement4 EXACT for offset, base permission, air/occupied target, original item-RNG sound, fire placement and damage. |
| BlockRedstoneRepeater | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemBucket | Not done | Minimal original filled-bucket creative preservation fix ported. Full ray trace/collection/Nether/cow-milk/permission replay remains. |
| ItemPainting | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemSword | Done | registry/actions/use/useticks/blocking EXACT: properties, use action/duration, hold/cancellation, blocking and armor damage/remainder/exhaustion with bypass flag. Existing environmental attack callers still need DamageSource mapping. |
| BlockContainer | Done | container7 + definitions EXACT: original superclass addition/removal/event callbacks, constructor flags, all ten factories including null moving-piston factory, lazy/invalid tiles, retained identity and signed chest events. |
| BlockLeaves | Done | Integrator credit: whole-world tick/new-world replays EXACT; growth4/harvest4/visual/colors retained. |
| Material | Done | definitions: all 29 material defaults; registry also checks harvest flags and tool behavior. |
| BlockNote | Done | thin7 + container7/music6/musicevents/activation4/definitions/textures EXACT: all overrides, remote/null guards, fresh factory, pitch/trigger/redstone callbacks and signed/extreme direct sound/particle events. |
| ItemRedstone | Done | registry/placement4 EXACT for snow/offset/occupied target, base permission, normal floor, true-without-placement return, consumption and absence of placement sound. |
| StepSound | Done | sounds4 EXACT for every registered block 1..109 step/break names and volume/pitch bits, including glass and metal; musicevents retained. |
| BlockStairs | Not done | geometry/drops EXACT including new stairs and model metadata drop quirk. Rendering integration pending. |
| ItemBed | Not done | placement4 EXACT for tested face/yaw/floor/occupied placement; both base edit hooks added. Full paired-cell boundary/collision audit remains. |
| MaterialWeb | Done | definitions: separate web material with movement disabled while isSolid remains true. |
| ItemSign | Not done | placement4 EXACT for tested wall/floor placement and suppressed GUI editor. Full permission/NBT/entity-facing audit remains. |
| BlockPistonExtension | Not done | piston5 proves extension/retraction participation in all six directions. Complete standalone bounds/selection/removal callback audit remains. |
| TileEntityDispenser | Not done | inventory7 + dispenser4/removal5/machines7 EXACT for real set/decrement/dirty callbacks, inventory NBT/slots/signed counts, identity and NaN distance, selection/consumption/RNG. Delegated NBT exception/alias behavior and retained detached Random ownership remain. |
| TileEntityFurnace | Not done | inventory7 + furnace4/removal5/machines7 EXACT for real inventory callbacks, NBT bytes/counts/timers/slots, fuels, counters/overflow, remote ticks, identity/cache/dirty behavior and NaN distance. Delegated negative-ID/malformed NBT exceptions and shared retained ItemStack aliases remain. |
| ItemShears | Done | registry/shears4/harvest4 EXACT for strengths/harvestability, all block destroy costs/statistics, signed durability extrema and leaves/tall-grass/vine drops. |
| ItemSlab | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockFence | Done | geometry: original fence/gate connection predicate for IDs 0..109; existing 10,000-case block geometry replay. |
| ItemLog | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockDoor | Not done | doors6 plus doors4/placepred4/geometry/drops6 EXACT for both door halves, power/activation/neighbor loss, remote suppression, hinges, placement, bounds and drops/sounds/RNG. Full render-dirty/IWorldAccess and malformed side/yaw edges remain. |
| BlockFire | Done | Integrator credit: whole-world tick/new-world replays EXACT; fire4 retained. |
| BlockFarmland | Done | plants: water, metadata, crops and both new stems; stem-aware walking hook ported. |
| BlockFlower | Done | thin7 + placepred4/growth4/definitions/all-block geometry EXACT: both flowers, all registered floors/light levels, stay/placement, direct neighbor/tick removal/drop/RNG and complete render/collision defaults. Source diff is a method rename only. |
| BlockRedstoneOre | Done | ore7 + definitions/visual/drops6/all-block geometry EXACT: both variants, every click/walk/activate/tick/display override, all opaque-neighbor masks, world/callback RNG, particles, light transitions and drops. |
| ItemSeeds | Done | registry/items/placement4 EXACT for all three seeds, sides, soil/occupied cells, both base edit hooks, crop/stem IDs and consumption. |
| ItemSoup | Done | actions/use/useticks EXACT: 32-tick completion, hunger/saturation and bowl replacement. |
| PistonBlockTextures | Done | piston5 EXACT for all 24 original constants (opposite side and X/Y/Z offsets), plus actual six-direction extension/retraction and interpolation. |
| BlockCake | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockCloth | Done | materials6 EXACT for both signed dye mappings and signed/extreme texture inputs; visual/definitions/registry/drops6 prove constructor/defaults, all world metadata, textures and metadata-preserving drops. No changed or added class override remains unported. |
| BlockPressurePlate | Not done | plates4 EXACT for all floors and both metadata states, fence support and removal; existing geometry EXACT. Full entity-trigger/power/tick audit remains. |
| BlockStep | Done | slabs6 + materials6 + definitions/visual/sides/geometry/drops6 EXACT: both constructors, six names, signed textures, instance/static opacity quirk, matching metadata merges for both instances, side culling/bounds, quantity/id/damage drops. No changed or added class override remains unported. |
| BlockTrapDoor | Not done | trapdoors6 + placepred4/geometry/drops6 EXACT for every support ID/metadata/remote state, unsupported removal, continued power processing, direct powered sound, placement predicates and bounds/drop surfaces. Item bounds, direct placement/click/activation and complete ray replay remain. |
| BlockWeb | Done | thin7 + definitions/visual/drops6/all-block geometry EXACT: web material/defaults, null collision/nonopaque crossed render/string drops and actual entity callback with spider/cave-spider overrides. |
| ItemCloth | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemMap | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| TileEntitySign | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockBed | Not done | drops6 proves head halves bypass quantity/chance RNG; doors/placement checks retained. Sleeping/paired removal/texture/render-dirty and full inherited class audit remain. |
| BlockCactus | Not done | placepred4/growth4 EXACT for tested floor/neighbor/stay/growth and RNG. Full inherited class audit remains. |
| BlockMobSpawner | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockReed | Not done | placement4/placepred4/growth4 EXACT for tested water/support/placement/growth and RNG. Full inherited class audit remains. |
| BlockSign | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockSnowBlock | Done | thin7 + growth4/definitions/drops6/all-block geometry EXACT: crafted-snow material, constructor/ticking, all 16 light/metadata states and remote guards inherited by drops, four snowball drops and geometry. |
| ItemLeaves | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemRecord | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| TileEntityRecordPlayer | Done | record4 EXACT for signed/extreme NBT, omitted nonpositive Record, reload/chunk factory; musicevents EXACT for live records. |
