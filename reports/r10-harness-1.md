# Minecraft 1.0 harness — round 1

Ported the original-bytecode oracle and all 31 inherited verification suites. The release fixtures load the original client/server 1.0 jars and release libraries, enforce Mojang Java 8u51, and compile in separate main, GUI, visual and server groups. The master runner retains its six-job parallel queue and ordered pflush; dedicated replay, setup, server, biome, independent-save and headless player/GUI rerun suites are also included.

The current section baseline is [r10-harness-baseline.md](r10-harness-baseline.md). It contains 1560 comparisons: 578 EXACT and 982 DIFFERS. The evidence column separates completed comparisons from oracle/checker failures. Every differing row records its first field, first pixel, or the concrete run failure that prevented a field comparison. Some bulk or variable records retain byte-level evidence; these are explicit diagnostic limitations. Supplemental rows supersede matching master rows and are labeled. Capture setup headings that completed successfully are represented by their per-image child results.

## Verification

| Check | Result | Evidence |
| --- | --- | --- |
| Full workspace cargo build --release | Passed | target/build10-final.log |
| cargo test -p worldgen10 | Passed | target/test10-final.log |
| Main, GUI, visual and server Java compilation | Passed, zero errors | target/compile10-final.log and target/javac10-*.log |
| Original RNG hook preservation | Passed for Entity, Explosion and World | Entire original class restored and compared after allowing only the seven-byte RNG substitution plus appended constant-pool entries |
| 1.0 client smoke | Passed; screenshot inspected | target/r10-smoke.png, target/smoke-client10.log |
| Shell syntax | Passed for the master, all focused suites and oracle launchers | bash -n |
| coverage.json | Unchanged, empty object | tools/r10/coverage.json |

The smoke command was `target/release/beta --mc 1.0 --seed 42 --no-save --shot target/r10-smoke.png --shot-ticks 60`. The image showed generated terrain and the player HUD. The inherited client branding and release behavior differences remain the client worker’s responsibility.

Four release test files needed fixture/assertion maintenance: NBT roundtrips now compare canonical compounds rather than insertion order; jukebox state is checked in the live RecordPlayer tile; Nether fixtures use valid data/block-light arrays, recognize empty missing arrays, the release zero Overworld ambient light floor, and owned query rectangles. No simulation or client implementation was changed. These small cross-area test changes are listed in the commit script.

## Runtime and reproducibility

The measured full verify10.sh took **3804.33 seconds (63.41 minutes)** with the default six parallel jobs; its exit code was 1 because the baseline contains differences.

| Suite | Seconds | Exit |
| --- | ---: | ---: |
| verify10.sh | 3804.33 | 1 |
| verify_alphafix10.sh | 1.50 | 1 |
| verify_biomes10.sh | 6.24 | 1 |
| verify_bucketfix10.sh | 138.97 | 1 |
| verify_constantfix10.sh | 435.90 | 1 |
| verify_entities10.sh | 205.45 | 1 |
| verify_gui10.sh | 130.10 | 1 |
| verify_harnessfix10.sh | 109.43 | 1 |
| verify_ingame10.sh | 26.52 | 1 |
| verify_latefix10.sh | 138.54 | 1 |
| verify_minecraft10.sh | 30.21 | 1 |
| verify_playerguifix10.sh | 142.77 | 1 |
| verify_playertick10.sh | 68.63 | 1 |
| verify_regions10.sh | 71.47 | 1 |
| verify_render10.sh | 426.75 | 1 |
| verify_replays10.sh | 69.74 | 1 |
| verify_serverfix10.sh | 316.24 | 1 |
| verify_statsync10.sh | 10.08 | 1 |
| verify_tracefix10.sh | 4.62 | 0 |

The full run exposed release spawning HashMap and fixture initialization changes; the corrected frame suite was rerun separately and supersedes the initial replay failures in the baseline table. A first-person shell invocation was interrupted by a launcher edit during discovery; the setup rerun and final focused rendering run provide stable results. Supplemental rechecks initialize the new server air watcher, renderer options and default texture pack, avoid stale trapdoor tiles, and expose original biome-search exceptions. The biome search iterates the full cached GenLayer array, including stale scratch entries; the harness preserves that original failure. A source-pair audit exposed constants and wire headers damaged by the initial broad version substitution. All 29 affected fixture sources were restored from the Beta originals, and their headless groups were rerun by verify_constantfix10; the final rendering suite also uses the restored constants. The source-pair audit now runs with compile_check.sh. Independent saves were regenerated before their region comparisons to avoid subsequent read/load checks changing the saved directories. The headless player/GUI modes were rechecked after reconciling inherited dump basenames and dynamic executable names. Those supplemental times are shown separately rather than added to the observed master runtime.

Commands: `python tools/r10/baseline.py` runs full/render/in-game/GUI suites; repeated `--suite` options select focused suites; `--report-only` rebuilds the table. Logs and section JSON are under target/r10/baseline. Original dumps and invocation sidecars are under tools/oracle10/out. Rust checker output, including inherited client diagnostic paths, is isolated by invocation under target/r10/compare; captures are under shots10. The inherited Rust fixture mode arguments and binary magic are retained, while executable names and client selection use release 1.0.

The harness no longer waits indefinitely for the release non-daemon file-I/O thread: client/server runners flush the original threaded I/O, close region caches and exit. Fixture colorizer images are read from the release jar and cached as cloned immutable pixels. The original launcher and release jar colorizer images were verified identical. GUI clock patches use release sources; GUI runs use real LWJGL, separate from headless recording stubs.

The safe rename/copy script is tools/r10/port_harness.py; it preserves numbered MCP identifiers, RNG seeds and wire magic. It is an initial-copy utility and must not be rerun over hand-fixed sources. tools/r10/port_verify.py regenerates all 31 inherited suites. tools/r10/diagnose_dump10.py and diagnose_replay10.py supply frame schemas, fixed terrain coordinates and opt-in scalar fixture traces without replacing original game methods.

Final setup rechecks also restore the release four-argument bucket ray hook, represent milk?s absent ray by zero-valued reserved legacy ray slots, and initialize original OpenGlHelper texture units before pickup lightmap capture. The milk fixture compares the original right-click return and world effects; its inherited envelope does not include the new drinking timer. Mouse recording stubs expose the release GUI fixtures? reflected buffers. Draw-trace companions retain .trace181/.draws181 suffixes and binary headers because those are inherited serialization identifiers. All five complete draw capture consistency checks now pass. Window wrapping passes all 32 comparisons. Event and chunk bulk diagnostics decode the remaining differences from saved dumps without changing original game methods.

## Remaining oracle/checker limitations

| Section | Failure |
| --- | --- |
| chunk meshes: seed 1, world rand 1, fancy, AO on | Checker failed before completing comparison: no entry found for key; thread 'main' (38372) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| chunk meshes: seed 1, world rand 1, fast, AO on | Checker failed before completing comparison: no entry found for key; thread 'main' (26996) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| chunk meshes: seed 1, world rand 1, fancy, AO off | Checker failed before completing comparison: no entry found for key; thread 'main' (33120) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| chunk meshes: seed 1, world rand 1, fast, AO off | Checker failed before completing comparison: no entry found for key; thread 'main' (29592) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| chunk meshes: seed -1, world rand -1, fancy, AO on | Checker failed before completing comparison: no entry found for key; thread 'main' (32212) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| chunk meshes: seed -1, world rand -1, fast, AO on | Checker failed before completing comparison: no entry found for key; thread 'main' (36708) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| chunk meshes: seed -1, world rand -1, fancy, AO off | Checker failed before completing comparison: no entry found for key; thread 'main' (18956) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| chunk meshes: seed -1, world rand -1, fast, AO off | Checker failed before completing comparison: no entry found for key; thread 'main' (34444) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: (rechecked by verify_render10) |
| server mode: 900-tick walk (real World.tick), unload/reload through region files | Checker failed before completing comparison: chunk identity stream exhausted; thread 'main' (37960) panicked at crates\worldgen10\src\world\tick.rs:26:83: (rechecked by verify_serverfix10) |
| biomes seed 0 | Oracle failed before comparison: java.lang.ArrayIndexOutOfBoundsException: 44672; at net.minecraft.src.WorldChunkManager.func_35556_a(WorldChunkManager.java:264) (rechecked by verify_biomes10) |
| biomes seed 3257840388504953787 | Oracle failed before comparison: java.lang.ArrayIndexOutOfBoundsException: 36915; at net.minecraft.src.WorldChunkManager.func_35556_a(WorldChunkManager.java:264) (rechecked by verify_biomes10) |
| biomes seed -4172144997902289642 | Oracle failed before comparison: java.lang.ArrayIndexOutOfBoundsException: 52428; at net.minecraft.src.WorldChunkManager.func_35556_a(WorldChunkManager.java:264) (rechecked by verify_biomes10) |
| biomes seed 9223372036854775807 | Oracle failed before comparison: java.lang.ArrayIndexOutOfBoundsException: 19660; at net.minecraft.src.WorldChunkManager.func_35556_a(WorldChunkManager.java:264) (rechecked by verify_biomes10) |
| biomes seed -9223372036854775808 | Oracle failed before comparison: java.lang.ArrayIndexOutOfBoundsException: 32768; at net.minecraft.src.WorldChunkManager.func_35556_a(WorldChunkManager.java:264) (rechecked by verify_biomes10) |
| entities10: xprender10 | byte 612, EntityOracle.rngState(e.rand) = 187788727644900 (Mode_entities10.java:41, field starts 612, 8 bytes, Long); Rust 201480731191965; another mode in this section failed: java.lang.UnsupportedOperationException: ModelBase random-part selection was removed in Minecraft 1.0; at net.minecraft.src.Harness10.removedRandomModel(Harness10.java:57) (rechecked by verify_entities10) |
| player-gui 1.0 round 5: queued click/use transitions | Oracle failed before comparison: java.lang.NullPointerException; at net.minecraft.client.Minecraft.clickMiddleMouseButton(Minecraft.java:2201) (rechecked by verify_constantfix10) |
| player-gui 1.0 round 4: controllers, picking and creative reach | Oracle failed before comparison: java.lang.NullPointerException; at net.minecraft.client.Minecraft.clickMiddleMouseButton(Minecraft.java:2201) (rechecked by verify_constantfix10) |
| entities3: factorybranches | byte 873, Harness10.removedOrbPhase(a) = 0.0 (Mode_entities10.java:39, field starts 873, 4 bytes, Float); Rust 4.624325752258301; another mode in this section failed: java.lang.UnsupportedOperationException: ModelBase random-part selection was removed in Minecraft 1.0; at net.minecraft.src.Harness10.removedRandomModel(Harness10.java:57) |
| entities6: sleep6 | Oracle failed before comparison: java.lang.UnsupportedOperationException: performSleepSpawning was removed in Minecraft 1.0; at net.minecraft.src.Harness10.removedSleepSpawning(Harness10.java:50) (rechecked by verify_constantfix10) |
| entities6: spawninputs6 (real JVM identity hashes) | Checker failed before completing comparison: missing recorded JVM identity; thread 'main' (29672) panicked at crates\worldgen10\src\entity\round6_verify181.rs:447:47: (rechecked by verify_constantfix10) |
| minecraft 1.0: original runTick event queues, clicks, screen/focus callbacks and world policy | Oracle failed before comparison: java.lang.NullPointerException; at net.minecraft.client.Minecraft.clickMiddleMouseButton(Minecraft.java:2201) (rechecked by verify_constantfix10) |
| minecraft 1.0: retained clickMouse and sendClickBlockToController effects | Oracle failed before comparison: java.lang.NullPointerException; at net.minecraft.client.Minecraft.clickMiddleMouseButton(Minecraft.java:2201) (rechecked by verify_constantfix10) |
| player: pipeline, every frame and final world | Checker failed before completing comparison: chunk identity stream exhausted; thread 'main' (30392) panicked at crates\worldgen10\src\world\tick.rs:26:83: (rechecked by verify_replays10) |
| player: night, every frame and final world | Oracle failed before comparison: java.lang.UnsupportedOperationException: performSleepSpawning was removed in Minecraft 1.0; at net.minecraft.src.Harness10.removedSleepSpawning(Harness10.java:50) (rechecked by verify_replays10) |
| player: sleepworld, every frame and final world | Checker failed before completing comparison: chunk identity stream exhausted; thread 'main' (36240) panicked at crates\worldgen10\src\world\tick.rs:26:83: (rechecked by verify_replays10) |
| entities: client, every frame for 3000 ticks | Checker failed before completing comparison: chunk identity stream exhausted; thread 'main' (4080) panicked at crates\worldgen10\src\world\tick.rs:26:83: (rechecked by verify_replays10) |
| entities: stormclient, every frame for 3000 ticks | Checker failed before completing comparison: chunk identity stream exhausted; thread 'main' (36468) panicked at crates\worldgen10\src\world\tick.rs:26:83: (rechecked by verify_replays10) |
| entities10: xprender10 | byte 612, EntityOracle.rngState(e.rand) = 187788727644900 (Mode_entities10.java:41, field starts 612, 8 bytes, Long); Rust 201480731191965; another mode in this section failed: java.lang.UnsupportedOperationException: ModelBase random-part selection was removed in Minecraft 1.0; at net.minecraft.src.Harness10.removedRandomModel(Harness10.java:57) |
| chunk meshes: seed 1, world rand 1, fancy, AO on | Checker failed before completing comparison: no entry found for key; thread 'main' (38372) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| chunk meshes: seed 1, world rand 1, fast, AO on | Checker failed before completing comparison: no entry found for key; thread 'main' (26996) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| chunk meshes: seed 1, world rand 1, fancy, AO off | Checker failed before completing comparison: no entry found for key; thread 'main' (33120) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| chunk meshes: seed 1, world rand 1, fast, AO off | Checker failed before completing comparison: no entry found for key; thread 'main' (29592) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| chunk meshes: seed -1, world rand -1, fancy, AO on | Checker failed before completing comparison: no entry found for key; thread 'main' (32212) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| chunk meshes: seed -1, world rand -1, fast, AO on | Checker failed before completing comparison: no entry found for key; thread 'main' (36708) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| chunk meshes: seed -1, world rand -1, fancy, AO off | Checker failed before completing comparison: no entry found for key; thread 'main' (18956) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |
| chunk meshes: seed -1, world rand -1, fast, AO off | Checker failed before completing comparison: no entry found for key; thread 'main' (34444) panicked at C:\Users\logiu\Desktop\Projects\minecraft\wt181-r10harness\crates\worldgen10\src\world\mod.rs:477:21: |

Removed-subject fixtures compile and fail explicitly: SpawnerAnimals.performSleepSpawning and ModelBase random-part selection have no release equivalents. Reserved dump slots for the removed XP-orb phase/fireball shake are zero to preserve the inherited checker envelope. Version differences, new item/entity counts and unsupported Rust checker cases remain area-worker work; they are not approximated by the harness.

## Harness class table

One row per Java fixture/stub source. Ported denotes a compiling release harness adaptation, not a claim that its Rust comparison is exact. Partial rows identify fixtures that deliberately retain removed subjects. The baseline identifies runtime limitations of individual modes.

| Class | Status | Original-bytecode check / adaptation |
| --- | --- | --- |
| ContainerOracle | Ported | Container/recipe ground truth: invokes original classes, never recompiles them. |
| DecorationOracle | Ported | Original sign, painting, map and fishing bytecode fixtures. |
| EntityOracle | Ported | Entity oracle cases always execute the original jar, except the independent Entity/Explosion RNG seed hooks. |
| FirstPersonOracle | Ported | Original b1.7.3 camera, ItemRenderer, RenderPlayer and targeting bytecode. |
| Harness10 | Partial | Fixture access for 1.0 state that is no longer public in the original jar. Removed-subject branches fail explicitly; other branches run on original release bytecode. |
| Mode_arrows10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_bedmodel10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_biomecache10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_biomes10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_bipedequipment5_10 | Ported | Arbitrary held stacks through original RenderBiped and ItemRenderer. |
| Mode_chesthooks10 | Ported | ContainerChest's open/close callbacks through single and directional double chests. |
| Mode_chests10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_chestworld | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_chunkcache10 | Ported | Direct-mapped cache: identities, callback order, eviction, save throttles and progress. |
| Mode_combat10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_component10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_controller10 | Ported | Original controllers: transitions, placement restoration, reach and physical pick policy. |
| Mode_controllerinput10 | Ported | Original Minecraft click/held/queued input callbacks, including repeated use and cooldowns. |
| Mode_controllerlife10 | Ported | Original controller factory, flip, no-op lifecycle, reset and partial-time boundaries. |
| Mode_controllermode10 | Ported | Original creative initialization uses the supplied player and retains existing stacks/flying. |
| Mode_creative | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_displayloop10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_drawintegration10 | Ported | Companion capture of the actual 1.8.1 Tessellator GL submissions. The legacy fixtures stay readable; this sidecar retains primitive mode and all eight words, including packed light. It is a fixture, not an exactness asse |
| Mode_drawtrace10 | Ported | Complete logical draws, including display-list playback. Unlike the legacy seven-word format, this retains primitive mode and both lightmap inputs. A companion legacy file supplies the original case inputs without change |
| Mode_effectdispatch8_10 | Ported | Probe only EffectRenderer's original bytecode, with observable virtual FX callbacks. |
| Mode_effectlifecycle4_10 | Ported | Original EffectRenderer layer caps, same-tick pruning and clear/reuse. |
| Mode_emitters10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_entities10 | Ported | Focused 1.0 entity replay; every call executes the original bytecode. |
| Mode_entityapi10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_entitybranches10 | Ported | Branch matrices executed on original entity bytecode, with pinned RNGs. |
| Mode_entityintegration10 | Ported | Inherited script with VM identity inputs recorded, unchanged snapshots. |
| Mode_entitylist10_10 | Ported | Concrete original EntityList constructors and malformed NBT, including null World. |
| Mode_entityround4_10 | Ported | Round 4 calls original entity bytecode; shared fixtures/recorders are unchanged. |
| Mode_entityround5_10 | Ported | Round 5 status, AI and vehicle calls run the original deobfuscated bytecode. |
| Mode_entityround6_10 | Partial | Round 6 singleplayer branch proofs; original classes are never replaced. Removed-subject branches fail explicitly; other branches run on original release bytecode. |
| Mode_entityspawninputs6_10 | Ported | Keep real JVM identities as replay inputs, with original HashSet/Spawner bytecode. |
| Mode_explosion10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_features10 | Ported | Isolated feature fixtures; all generators below come from the original jar. |
| Mode_generaliving10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_generaliving4_10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_generalstats10 | Ported | Original general-stat list and virtual drawString arguments, without GPU rounding. |
| Mode_guichest10 | Ported | ContainerChest's open/close callbacks through single and directional double chests. |
| Mode_guiclose10 | Ported | Actual GuiContainer.onGuiClosed and actual container callbacks, including vanilla double drops. |
| Mode_guislot8_10 | Ported | Original GuiSlot callbacks. Only input buffers and timestamps are supplied by the fixture. |
| Mode_guiutility10 | Ported | Original scale rounding, button hit boundaries and font measurement/wrapping. |
| Mode_inventorybounds10 | Ported | Exhausts the signed-short item-id domain through actual InventoryPlayer.readFromNBT. |
| Mode_inventorygui10 | Ported | Calls the original inventory screen constructor, init/update and hidden button callbacks. |
| Mode_inventoryinterfaces10 | Ported | Calls original IInventory and Slot methods, including the b10 open/close additions. |
| Mode_itembase10_10 | Ported | Actual Item bytecode, including constructor edits and inherited defaults. |
| Mode_itemblock10_10 | Ported | Original ItemBlock with virtual dependencies; no World/block bytecode replaced. |
| Mode_itemblock6_10 | Ported | Registered block-item placements, full height/face/replacement/collision matrix. |
| Mode_itembucket7_10 | Partial | Original ItemBucket, with controlled ray results plus the real World ray tracer. Milk starts drinking without a ray; absent legacy ray slots are zero. Drinking timer is outside this envelope. |
| Mode_itemdoor6_10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_itemdye6_10 | Ported | Every ItemDye override, including exceptional damage values and both RNGs. |
| Mode_itemlog7_10 | Ported | ItemLog's stored constructor Block, independent of its inherited placement Block. |
| Mode_itemmap8_10 | Ported | ItemMap's static/stack lookups, creation and real onUpdate callbacks. |
| Mode_itemmetadata7_10 | Ported | All signed-short icon/color/name/placement inputs plus full int boundaries. |
| Mode_itemmetaplace7_10 | Ported | Registered block-item placements, full height/face/replacement/collision matrix. |
| Mode_itempiston11_10 | Ported | Actual ItemBlock/World/BlockPistonMoving bytecode, including malformed tiles. |
| Mode_itemplacement7_10 | Ported | Original ItemBed/ItemSign callbacks, including failed World writes and editor calls. |
| Mode_itemstack11_10 | Ported | Every original ItemStack method; virtual Items observe the same mutable alias. |
| Mode_itemvine7_10 | Ported | ItemVine's captured Block and independent subtype flag, including custom callbacks. |
| Mode_javadigits6_10 | Ported | Character.digit tables of the required Mojang 8u51 runtime. |
| Mode_keyregistry10 | Ported | Global KeyBinding registry, including stale lookup until reset and retained old settings. |
| Mode_lightmap | Ported | oracle.sh lightmap out.bin: the original EntityRenderer.func_35808_e (1.0 lightmap) over times of day, rain, thunder, lightning flashes, torch flicker values, brightness settings and both dimensions. Per case: the 16 bri |
| Mode_listentities | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_mainmenu10_10 | Ported | Original constructor RNG, complete singleplayer buttons and update/key/action callbacks. |
| Mode_managervisual9_10 | Ported | Real registered renderers, including the newly routed thrown-item sprites. |
| Mode_minecraftdebug10 | Ported | Actual Minecraft.debugInfoEntities -> EffectRenderer + World delegation. |
| Mode_minecraftpreload10 | Ported | Invokes original preloadWorld, including null loading screen, player coordinates, optional load-provider focus and reduced-controller branches. |
| Mode_modelregistry10 | Partial | Fixture support; compile and exercise through the owning release verification mode. Removed-subject branches fail explicitly; other branches run on original release bytecode. |
| Mode_modelregistry3_10 | Partial | Original constructor registration, including vehicle and sign models. Removed-subject branches fail explicitly; other branches run on original release bytecode. |
| Mode_models10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_newliving10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_options10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_particlefactory5_10 | Ported | Calls original RenderGlobal.spawnParticle; constructors and factory guard/order remain original. |
| Mode_particlefactory6_10 | Ported | Calls original RenderGlobal.spawnParticle; constructors and factory guard/order remain original. |
| Mode_particles10 | Ported | Runs the original EntityFX classes and reads original Tessellator buffers. |
| Mode_particles2_10 | Ported | Runs the original EntityFX classes and reads original Tessellator buffers. |
| Mode_playercombat10 | Ported | EntityPlayer's literal melee policy, including refused hits and potion mining. |
| Mode_playercore | Ported | Calls the original FoodStats and EntityPlayer XP code, with isolated health callbacks. |
| Mode_playerdamage8_10 | Ported | Original player damage/death callbacks, especially source-without-attacker and self shooter. |
| Mode_playerdata | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_playerintegration10 | Partial | The inherited player script with explicit generic DamageSource inputs. Beta 1.8.1 no longer accepts the old null-source damage calls. All snapshots use the unchanged PlayerOracle/EntityOracle fixtures. The removed Minecr Removed-subject branches fail explicitly; other branches run on original release bytecode. |
| Mode_playerlist10 | Ported | Execute original PlayerList bytecode, including duplicate insertion and table order. |
| Mode_playerliving4_10 | Ported | RenderPlayer body/armor/items/arrows; label is suppressed for the viewing player. |
| Mode_playerliving6_10 | Ported | RenderPlayer body/armor/items/arrows; label is suppressed for the viewing player. |
| Mode_playernbt10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_playersp10 | Ported | SP action/FOV boundaries and real onUpdate with nonzero view and death. |
| Mode_playerstate10 | Ported | Direct original player callbacks and inventory/NBT boundary methods; no copied policy. |
| Mode_playertick10 | Ported | Whole original EntityPlayerSP.onUpdate, without the legacy World dump schema. |
| Mode_playertickintegration10 | Ported | Original whole-player driver and snapshots, with the potion inputs used by the Rust replay. The older driver silently treated these scenarios as idle. |
| Mode_portalworld | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_provider10 | Ported | Original singleplayer provider scanning and signed-Long HashSet removals. |
| Mode_recipeaudit10 | Ported | Initialize the original registries in client order before inspecting recipes. |
| Mode_rename8_10 | Ported | Calls actual original rename init/input/save callbacks and missing-save failure. |
| Mode_renderengine10_10 | Ported | Exercise the actual getTexture branch order, new blurclamp prefix and flag reset. Texture upload is the observing adapter; retained texturecheck covers conversion. |
| Mode_renderengine11_10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_renderintegration10 | Ported | Supply the colorizer images Minecraft loads before these renderer calls. |
| Mode_renderlist10 | Ported | RenderList (1.0 keeps the camera offset in double): the glTranslatef each list emits, for WorldRenderer origins near spawn, far out and at the world border. |
| Mode_rendermanager9_10 | Ported | Original RenderManager bytecode. Virtual probes observe its dependencies, without replacing the class or recording any renderer's drawing algorithm. |
| Mode_replayidentities10 | Ported | Record the actual VM identity input before the original HashSet inserts it. Separate streams preserve the independent active and spawning set lifetimes. |
| Mode_runtick10 | Ported | Calls the unmodified original Minecraft.runTick. Only platform/render/world tick callbacks are replaced; inventory, item use, clicks and controllers run the original bytecode. The fixture carries its event inputs with ea |
| Mode_screenshot10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_select8_10 | Ported | Original saved-world controller policy, labels, selection latch and delete callbacks. |
| Mode_spawn10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_stackdata11_10 | Ported | ItemStack constructors, scalar mutations/copies/equality and NBT failure order. |
| Mode_stacknbt11_10 | Ported | Original ItemStack's virtual compound calls and retained alias on failure. |
| Mode_statfile10_10 | Ported | Calls original static parsing/serialization. Malformed schema exceptions are observed at the API boundary, independently of StatsSyncher's file catch. |
| Mode_statfile11_10 | Ported | Original constructor failures, malformed UTF-16 and Windows charset boundaries. |
| Mode_stats8_10 | Ported | Original stats list constructors, row callbacks, header geometry and full sorting cycles. |
| Mode_statsorder10 | Ported | Calls the original comparators, including missing-stat precedence and signed overflow. |
| Mode_statsync10 | Ported | Real StatFileWriter/StatsSyncher and both original worker threads, with barriers only at observation points so disk timing cannot change the trace. |
| Mode_strongholds10 | Ported | Stronghold graphs, retained map/RNG order, clipped blocks and chest inventories. Only the harness is compiled here: all structure classes come from Mojang's jar. |
| Mode_structures10 | Ported | Real original-bytecode mineshaft starts, pieces, state and clipped block generation. |
| Mode_structurestarts10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_terrain | Ported | Original bytecode biome maps and terrain with an explicit structures switch. |
| Mode_terrainnbt10 | Ported | Exact compound serialization through original NBTTagCompound, including tree bins. |
| Mode_terrainround2 | Ported | Sky population, cross-world biome generator lifetime, sky colours and dungeon tiles. |
| Mode_terrainsave10 | Ported | Independently generated saves with the otherwise time-seeded item/entity RNGs pinned. |
| Mode_terrainutils10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_ticksintegration10 | Ported | Original whole-world tick replay, recording the VM identity hashes used by Java 8 HashMap tree bins. World/event serialization is the shared fixture. |
| Mode_tilemanager9_10 | Ported | Original tile manager bytecode, including HashMap iteration with real Class hashes. |
| Mode_tilespecial9_10 | Ported | Execute the original base-class methods with observable resource callbacks. |
| Mode_vehiclerenders10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_vine10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbactivation4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbarrays | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbattached6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbbase6 | Ported | New Block access methods and the base harvest callback use original bytecode. |
| Mode_wbbed9 | Ported | Original BlockBed bytecode, with its separately owned sleep/explosion callees recorded. |
| Mode_wbblocking | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbcache | Ported | Calls the original ChunkCache, including its different World boundary rules. |
| Mode_wbcactus8 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbcake10 | Ported | BlockCake.eatCakeSlice (1.0: canEat(false), FoodStats.addStats(2, 0.1F)) through both of its callers, blockActivated (right click) and onBlockClicked (left click). |
| Mode_wbcake9 | Ported | Full BlockCake surface; the range recorder isolates explicit block callbacks. Existing WorldEventOracle continues to record every sound/particle callback. |
| Mode_wbchestlife | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbchestopen5 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbcolors | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbcontainer7 | Ported | Invoke original BlockContainer superclass methods on each real subclass. |
| Mode_wbconversion6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbdispenser4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbdispenser6 | Ported | BlockDispenser callbacks complement dispenser4, removal5 and the visual tables. |
| Mode_wbdispensertile10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbdoor9 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbdoors4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbdoors6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbdrops6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbemptycache4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbextension9 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbfileclasses | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbfire4 | Ported | Original fire ticks around the new vine fuel, retaining age, scheduling and RNG. |
| Mode_wbflow5 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbfoodstats | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbfrost | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbfurnace4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbfurnace7 | Ported | Complete BlockFurnace callbacks; existing furnace4/removal5 cover ticks/drops. |
| Mode_wbfurnacetile10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbgrowth4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbharvest4 | Ported | Shears and ordinary harvest for leaves, tall grass and vines at every metadata value. |
| Mode_wbinventory7 | Ported | TileEntityFurnace/Dispenser inventory, NBT, scaling and interaction methods. |
| Mode_wbjukebox10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wblegacy5 | Ported | Original old ChunkLoader paths, reads, saves, locks and dimension selection. |
| Mode_wbmachine10 | Ported | Calls real tile methods; the companion modes select one class at a time. |
| Mode_wbmachines7 | Ported | Keep the shared 1000-tick fixture unchanged. |
| Mode_wbmaterials6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbmath | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbmchash5 | Ported | Original MCHash duplicate insertion, rehashing, null values and live key set. |
| Mode_wbmushrooms | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbmusic6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbnextticks | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wboldformat4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbore7 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbpiston5 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbpiston6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbpistontile10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbplacement4 | Ported | Calls original Item.onItemUse; compares its result, stack, blocks, RNG and callbacks. |
| Mode_wbplacepred4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbplate9 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbplates4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbpush6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbrecord4 | Ported | Original jukebox tile NBT, insertion, ejection and null-tile behavior. |
| Mode_wbreed8 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbregioncache4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbregions | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbregistry | Ported | Initialize Block before Item (the original client startup order). |
| Mode_wbrelocation | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbremoval5 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbrepeater8 | Ported | Complete direct callbacks of both original repeater instances. |
| Mode_wbsavelife | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbshears4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbsides | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbsign8 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbsilverfish | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbslabs6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbsounds4 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbspawn | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbspawner8 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbsponge10 | Ported | BlockSponge (unchanged since b1.7.3): onBlockAdded reads the 5x5x5 materials, onBlockRemoval notifies all 125 positions. Water and an unsupported torch inside the area show the notifications. |
| Mode_wbstairs8 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbstorage | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbtextures | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbthin7 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbtileentity10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbtrapdoor8 | Ported | Every remaining BlockTrapDoor override calls original bytecode. |
| Mode_wbtrapdoors6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbunload6 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbuse | Ported | Original Item methods, player use completion, and FoodStats, without terrain. |
| Mode_wbuseticks | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbvisual | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbweather | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wbwire8 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_wolfdespawn8_10 | Ported | EntityWolf.canDespawn overrides EntityAnimal: only tamed wolves opt out. |
| Mode_worldblocks10 | Ported | Focused world/block checks that call the original 1.0 bytecode. |
| Mode_worldcallbacks10 | Ported | Executes the original create-world input, button and launch callbacks with a recording client. |
| Mode_worldseed10 | Ported | Executes GuiCreateWorld.actionPerformed; only inherently clock-random seeds are classified. |
| Mode_xprender10 | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| Mode_yesno8_10 | Ported | Actual confirmation callbacks with arbitrary parents, signed world IDs and button IDs. |
| ModelShot | Ported | Optional visual oracle. Actual Mojang RenderLiving subclasses and model display lists draw into a GL pbuffer. The only stubs supply textures, brightness and render state, replacing the world/AI. models mode in Oracle.jav |
| Oracle | Ported | Ground-truth dumper. Runs Mojang's original b1.7.3 bytecode (deobfuscated.jar, names remapped only) and writes data the Rust port is compared against. Lives in net.minecraft.src so it can call protected World methods. ra |
| OracleHooks | Ported | The only override in the original Entity is its Random field initializer. |
| OracleRunner10 | Ported | Finish fixture I/O and exit despite the 1.0 non-daemon File IO Thread. |
| ParticleOracle | Ported | Runs the original EntityFX classes and reads original Tessellator buffers. |
| PistonRendererOracle | Ported | Execute the original TileEntityRendererPiston, recording its Tessellator bits. |
| PlayerOracle | Partial | Player tests call the original jar's inventory, entity and controller methods. Removed-subject branches fail explicitly; other branches run on original release bytecode. |
| RedstoneOracle | Ported | Scripted block worlds for redstone, rails, note blocks and pistons: a flat stone world (chunks -3..3, floor at y 63), commands from a text file, the original World.tick in between. Commands: place x y z id meta \| remove  |
| RendererOracle | Ported | Execute original renderer bytecode and record GL inputs + original Tessellator bytes. |
| ShowcaseOracle | Ported | Mesh tools/oracle/showcase.txt (see make_showcase.py) with the original RenderBlocks. |
| SoundOracle | Ported | Exercises Mojang's original MusInputStream bytecode, with no audio device. |
| StatsOracle | Ported | StatList as the original registers it: the general/block/item lists the statistics screen shows, and which stat each block/item index adds to (after replaceSimilarBlocks). |
| TeleportOracle | Ported | The original Teleporter after a dimension change: preload around the arrival point as Minecraft.func_6255_d does, then find or build the portal. |
| Trace10 | Ported | Opt-in dump-field locator. Never patches or replaces original game methods. |
| VehicleOracle | Ported | Vehicle replays execute the original EntityMinecart/Boat and World bytecode. |
| WorldEventOracle | Ported | Records original IWorldAccess calls without constructing an EffectRenderer. |
| Keyboard (headless) | Ported | Queued LWJGL events, isolated to Minecraft replay modes. Physical state is polled state after all queued events, not state at the current event. |
| Mouse (headless) | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| ContextCapabilities (headless) | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| GL11 (headless) | Ported | No GL context: record the original Tessellator immediately before draw resets it. |
| GL12 (headless) | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| GL13 (headless) | Ported | Headless GL13 for 1.0's lightmap: tracks the active texture unit and the lightmap coordinates (texture unit 1) so recorders can read them; no GL context needed. |
| GLContext (headless) | Ported | Capabilities queried by the original renderer; no native context exists. |
| Display (headless-input) | Ported | Fixture support; compile and exercise through the owning release verification mode. |
| GuiLightmapFixture (gui) | Ported | Controlled texture-unit-1 input for original GuiContainer pixel tests. |
| GuiPanoramaProbe10 (gui) | Ported | Optional observations between actual original cube and blur methods. |
| GuiShot (gui) | Ported | Runs the original Release 1.0 client (real window, real OpenGL) through a scene script and saves screenshots, for pixel comparison with the Rust port's `beta --guishot`. See crates/client10/src/gui/shot.rs for the script |
| ServerOracle (server/src) | Ported | Ground truth for server-mode chunk handling, running the original 1.0 server classes (RetroMCP's deobfuscated server jar): WorldServer, ChunkProviderServer and PlayerManager, with a stand-in player walking a scripted pat |
| ServerRunner10 (server/src) | Ported | Flush the original asynchronous region writer before ending a server fixture. |
| GuiMainMenu (generated GUI clock patch) | Ported | Original release source; only System.currentTimeMillis reads use GuiShot.millis. Exercise splash animation, achievement scrolling and notification timing with fixed capture clocks. |
| GuiAchievements (generated GUI clock patch) | Ported | Original release source; only System.currentTimeMillis reads use GuiShot.millis. Exercise splash animation, achievement scrolling and notification timing with fixed capture clocks. |
| GuiAchievement (generated GUI clock patch) | Ported | Original release source; only System.currentTimeMillis reads use GuiShot.millis. Exercise splash animation, achievement scrolling and notification timing with fixed capture clocks. |

No Beta crate, original oracle, or original verification script was edited. coverage.json remains `{}`. No commit was made. `target/commit-r10-harness.sh` stages these changes and uses the required identity and co-author trailers.
