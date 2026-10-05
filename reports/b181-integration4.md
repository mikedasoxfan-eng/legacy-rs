# Beta 1.8.1 integration, round 4

Continued the interrupted run's uncommitted edits and `target/integration4-*` diagnostics. No b1.7.3 crate, oracle or verifier was edited. Sibling worktree status was checked before simulation edits; no file modified by another owner was changed. Renderer implementation files remain with their owners.

## Verification

The final `bash tools/verify181.sh` completed with **598 labelled sections: 565 passed, 32 failed, 1 grouping heading without its own result** (exit 1, SOME CASES DIFFER). The group is the player/gui parent heading; its children have their own results. The initial full run is `target/integration4-full-verify.log`; the final run is `target/integration4-final-verify.log`, indexed in `target/integration4-final-sections.json`. The suite has grown since the task's 424/488 baseline, so its denominator differs. These are the recorded full-run counts, before the isolated decoration corrections described below.

Scoped completed proofs:

- All three 2,000-tick rain/thunder replays and the 4,000-tick forest fire are EXACT in the final full run.
- Saplings: **96,000 ticks EXACT**, including every tick's time, LCG, cave-sound counter and World.rand state, all events, and final world bytes, in both the scoped probe (`target/integration4-saplings-96000-rust.log`) and final full run (163.55 seconds for Rust).
- Server: **900-tick walk EXACT** in memory, final time/LCG/sound/RNG state, and disk reload. The final full run has **4,622/4,622 decompressed chunk NBT payloads byte-identical**, including entities, independently confirming the scoped probe (`target/integration4-server-pinned-{rust,nbt}.log`). Sector allocation differs and is not part of the chunk-data comparison.
- Redstone `pistons_chunk_nbt` and `tile_lifecycle`: EXACT (`target/integration4-rs-*.log`), also reached in the full run.
- Existing moving-piston and first-person checks pass without renderer edits: 24,840 cases / 4,591,104 vertex words with zero mismatches, and 19,652 cases / 8,412,376 raw-bit comparisons with zero differences respectively.
- Existing Beta 1.8.1 model fixture: **208 poses, 1,536 parts, 293,376 raw-bit comparisons, zero differences** in the final full run and scoped probe (`target/integration4-models181-rust.log`). The full-suite call now uses the existing `models181` mode.
- Repaired player NBT fixture: **300 ticks and final world EXACT**. Repaired player stats fixture: **400 ticks and final world EXACT**. Both pass in the final full run and scoped probes (`target/integration4-nbt-canonical-rust.log`, `target/integration4-player-new-diagnoses.log`).
- Whole-player suite: **all 23 scenarios EXACT** in the final full run, including haste/fatigue/nausea for 500 ticks each (601,924 / 601,924 / 600,772 bytes). Only harness inputs changed; the original snapshot helpers and Rust simulation were retained.
- Corrected post-run decorations child: **DECORATIONS EXACT**, exit 0 (`target/integration4-decoration-palette.log`). Signs, 384 painting wall/seed replays, map palette plus 480 terrain/marker frames, map file persistence/rollover and Java reads, and 48,000 fishing ticks/events match. Sign/painting/fish rendering has 521,440 / 283,140 / 22,920 comparisons with zero differences.

## Implementation and class status

“Done” below describes the listed integration boundary, not an assertion that every method of a shared class is finished.

| Class / boundary | Status and evidence |
| --- | --- |
| ChunkCoordIntPair | Done for 64-bit coordinate packing and `hashCode = x ^ z`. The old sign/magnitude 32-bit packing was wrong for Beta 1.8.1. Weather/fire/sapling and server walks prove the traversal used here. |
| ChunkProviderServer / PlayerManager | Done for the scripted load, unload and reload boundary. Dropped chunks use Java boxed Long ordering, including comparable tree bins. Unload calls entity removal before saving. The 900-tick walk and all 4,622 saved NBT chunks match. |
| World.updateBlocksAndPlayCaveSounds | Done for the five whole-world ticking sections. Colliding non-comparable ChunkCoordIntPair tree bins depend on VM identity hashes; the new oracle records the actual per-object hashes and Rust replays them without changing their Java iteration order. Every supplied hash must be consumed. Other entity/spawning fixtures still need this input pinned. |
| World / Chunk tile unload list | Done for the checked lifecycle boundary. Chunk unload queues tiles for removal after their final update instead of invalidating them. `pending_unload` models World.field_34900_Q, followed by removal after the tile scan. Redstone lifecycle, hooks replay and server reload pass. |
| WorldGenBigTree | Done for natural population versus sapling construction. Population calls func_517_a(1,1,1), setting leafDistanceLimit to 5; BlockSapling creates a fresh unscaled generator whose value is 4. The original implementation always scaled it. The four-day replay now matches. |
| WorldGenTrees / WorldGenForest / WorldGenTaiga2 | Done for the supplied-Random boundary. Saplings use World.rand directly; population retains the generator RNG. Nested generation no longer temporarily replaces the population stream with a copy of World.rand. Saplings and existing feature/preload checks cover this. |
| NBTTagCompound comparison serialization | Done for the canonical player NBT fixture. Sorting the vector no longer controlled output after the production writer gained Java HashMap order. A separate recursive canonical writer restores the unchanged Java fixture's sorted form. Production save serialization keeps its existing order. |
| ServerOracle | Done for WorldSettings constructors, renamed save/lighting methods, chunk-list extraction and Unsafe player initialization. Math.random and chest/furnace/dispenser drop RNGs are pinned before generation. The server jar remains original bytecode. Java 8u51 is enforced; javac failure is no longer swallowed. |
| PlayerOracle integration driver | A separate Mode_playerintegration181 keeps the original snapshot helpers and script. It replaces obsolete null damage inputs with generic DamageSource, restores a controller on reloaded players, and stores the removed mouseTicksRan repeat clock in the driver. NBT and stats now compare; remaining gameplay differences are owner work. |
| Whole-player potion driver | Done: all 23 scenarios EXACT in the final run. A separate Mode_playertickintegration181 reuses Mode_playertick181.frame/food and its main script. The older Java driver silently ran haste/fatigue/nausea as idle, although Rust applied effects. Supply the same original PotionEffect inputs: ID 3, 80 ticks, amplifier t/100%8 every 100 ticks; ID 4, 80 ticks, amplifier t/100%4 every 100 ticks; ID 9, 300 ticks, amplifier 0 at tick 0. |
| Model oracle selection | Done: the existing models181 fixture includes Enderman, Silverfish and chest models expected by the Rust comparator. No renderer port was changed. |
| Decoration renderer dispatch | Done: corrected child suite DECORATIONS EXACT. The script defined a Beta 1.8.1 beta() wrapper but bypassed it for sign/painting/fish checks, invoking the default Beta 1.7.3 renderer. All three now use the wrapper; no renderer implementation was changed. |
| Map material palette | Done: map palette, 480 terrain/marker frames, NBT and persistence EXACT in the corrected child suite. MATERIAL_COLORS still used 28 Beta 1.7.3 materials. Beta 1.8.1 inserts Material.field_35495_k (vines) after plants; its MapColor is foliageColor (7). Insert this entry and shift the remaining entries, giving all 29 materials. No sibling worker modified maps.rs when this local correction was made. |
| New worlds / raw Nether / light grids / portal / respawn / save conversion | Already repaired in the preceding terrain rounds; retained and rechecked by the complete suite. No new algorithm change was needed here. |

The compare harness's chunk-list reader now recognizes all six tile tags from Oracle.worldBytes, including village furnaces. Its old chest/spawner-only reader ran past the end of the 379,991,316-byte server disk dump. With the fix, all 4,622 reload chunks compare.

The raw server NBT probe initially found 416 differing chunks, exclusively in Entities: 1,311 Motion/Rotation differences and 33 Pos differences. Pinning Math.random and block drop RNGs resolved all of them; these were harness inputs, not a demonstrated terrain mismatch.

## Owner diagnosis

The read-only decoder is `tools/b181/diagnose_replay181.py`; it locates the first different frame, entity and field without changing shared fixtures. Original replays are indexed in `target/integration4-diagnoses.log`; repaired player driver cases are in `target/integration4-player-new-diagnoses.log`.

| Owner / replay | First difference and cause |
| --- | --- |
| Player: movement, pickup | Frame 521, posX: Java -12.600000001490116, Rust -12.5. EntityPlayer.onDeath tests non-null DamageSource; Rust player_death tests attacker.is_some(). Lava damage has no attacker but still has a non-null source. |
| Player: survival, bucket, repaired damage | Same death-source issue. Survival frame 501; bucket frame 44; repaired damage frame 542. |
| Player: dig, digextras | Frames 501 and 1108, stat count 3 versus 2. Missing **16908647 = used(359)**. ItemShears.onBlockDestroyed returns true for leaves/web/tall grass/vines, and ItemStack.onDestroyBlock records use. The Rust controller counts use only for sword/tool ID ranges, omitting shears. |
| Player / entities: combat | Frame 21, player posX 0.16494847900878684 versus 0.47567818038087295. Audit indirect DamageSource attribution and self-shooter knockback while the player is temporarily removed from World.entities. This is a targeted lead, not a completed port proof. |
| Player: API | Repaired oracle reaches comparison. Frame 5, renderYawOffset: 74.80000305175781 versus 92.78338623046875. The obsolete mouseTicksRan reflection previously prevented any comparison. Audit click/swing/controller timing and living animation. |
| Player: pipeline, sleepworld | First RNG divergence at frames 1 and 2 respectively. These drivers still use unpinned active-chunk identities; they need the same recorded identity stream as ticksintegration181 before interpreting later RNG/world differences. |
| Entities: passive, aquatic, hostile, special, spawner, nbt, nbtall, slimes, combat, lightning, rider | First differences are pitch while looking at the stand-in player. EntityOtherPlayerMP.getEyeHeight returns **1.82F** in Beta 1.8.1, but the coordinate-only Rust Target::Player fallback in creature.rs uses **0.12F**. Examples: passive Sheep frame 67, aquatic Slime frame 1, hostile Skeleton frame 1, special Wolf frame 67, spawner Zombie frame 22, nbt Pig frame 67, nbtall/slimes frame 1, combat Wolf frame 51, lightning Creeper/rider Skeleton frame 1. Represent the actual remote-player eye height in these fixtures; do not alter the local EntityPlayerSP eye height. |
| Entities: spawning, client, stormclient, nether | First spawned entity differs at frame 1. Examples: spawning Creeper x=19.5 versus 51.5; client Pig x=-73.5 versus 118.5; stormclient Sheep x=-9.5 versus -89.5; Nether PigZombie x=-44.5 versus 51.5. SpawnerAnimals also builds HashSet<ChunkCoordIntPair> with VM identity tie breaks. Rust already exposes perform_spawning_with_identity; these inherited drivers need the original identity stream pinned. Broader spawn behavior remains owner work. |
| World-blocks: container tile world replay | Frame 1, furnace index 9 at (6,64,8): Java has an empty inventory, Rust retains coal and workbench fuel. Vanilla Chunk.setBlockID/setBlockIDWithMetadata uses **local** x/z when creating a replacement tile after getChunkBlockTileEntity returns null. A burning transition in a negative chunk queues an empty tile in the positive chunk, replacing this furnace. Preserve this vanilla coordinate bug; see the original Chunk.java calls to worldObj.setBlockTileEntity(var1,var2,var3,...). The initial first-byte difference is 1057. Container inventory/crafting-only replay is independently exact. |
| World-blocks: notes/jukebox sounds | Panics in TileEntity::for_block(84). Beta 1.8.1 BlockJukeBox extends BlockContainer and creates TileEntityRecordPlayer. The Rust tile enum has no record-player payload. Port record storage/NBT and BlockJukeBox insertion/ejection together: the full record ID belongs in the tile, while metadata is the occupied flag. Do not restore the old metadata-only jukebox representation. |
| Integrator / entity renderers | Remaining object/pickup/effect failures are indexed in the full log; moving-piston rendering is already exact. Beta 1.8.1 keeps quads and carries the brightness word. Do not reinterpret original draws as Beta 1.7.3 triangles or discard brightness to claim exactness. Renderer implementation belongs to its owners. |

No shared Mode_entities181.world, WorldEventOracle or Oracle.worldBytes serializer was changed. The new player driver copies the simulation script and reuses the existing snapshot/helper methods; its edits concern valid Beta 1.8.1 harness inputs and initialization.

The whole-player haste failure in the initial run was an input mismatch, not a demonstrated potion port defect: Java never applied the effect. The inherited comparator prints lengths as Rust/Java, so the initial 601,924/597,184-byte lengths showed Rust's active effect versus Java's idle script. The new mode supplies all three missing potion cases without changing snapshots. It was added before the final run entered verify_playertick181.sh.

The final full run's unlabelled decorations child initially stopped on 1,920 sign normal differences (of 521,440 values), e.g. Java 0000007f versus Rust ffffff80. Unlike verify_entities3_181.sh, the decoration child omitted --mc b1.8.1 for every renderer command, selecting the default Beta 1.7.3 implementation. Its three commands now use the existing beta() wrapper. The raw full-run counter groups this failure under the preceding conversion heading, although conversion itself matched all 289 chunk payloads. A separate corrected decoration rerun follows the full run; its outcome is reported separately so the recorded full-run counts remain honest.

The first corrected decoration rerun (`target/integration4-decoration-final.log`) passes signs and paintings, then panics in maps.rs at material index 28. The 29-entry palette correction is tested by `target/integration4-decoration-palette.log`, which completes with **DECORATIONS EXACT** and exit 0. The full suite was not restarted for these two isolated post-run corrections.

The initial full run completed with 598 headings: 557 passed, 32 failed, 9 without an explicit result. Five silent headings were failed legacy player oracle invocations, now replaced by the integration mode. Other silent headings included successful comparisons whose pass message was missing or suppressed by the global failure flag, and grouping headings. The log also contains a Windows access-denied build failure after the **CONVERSION EXACT** result (289/289 chunk payloads and 307 progress steps), caused by a concurrent scoped compare process holding compare181.exe. The final full run runs without those scoped probes. The count helper deliberately records visible failures instead of inferring success from silence, and recognizes the settings preload/save regression test's explicit passing result.

## Delivery

`target/commit-b181-terrain4.sh` stages the explicit round-4 files and commits with the required logiu identity and both requested trailers. `target/commit-b181-terrain.sh` is the compatibility copy. Neither script was executed; edits remain uncommitted for integration.

The recorded full-run failures are 11 inherited player replays, 15 inherited entity replays, the container world replay, two object/pickup renderer sections, jukebox sounds, EffectRenderer, and the conversion-labelled section containing the unlabelled decoration failure. The high-priority world-generation/ticking/server/save/portal/respawn/redstone checks all pass. Gameplay and renderer owner work remains; no claim is made that the entire suite passes.

At the final sibling status check, world-blocks/entities had started editing entity/nbt.rs and tile_lifecycle.rs after this integration work had changed them. No further edits were made to those files. Integrate the small unload-queue changes carefully with their later owner work.

Shell syntax checks, Python compilation and git diff --check pass. The decoration rerun rebuilds worldgen181 and client after the final palette edit. Beta 1.7.3 files and shared world/event dump serializers remain unchanged.

## Recorded full-run failure index

Line numbers refer to target/integration4-final-verify.log. Causes and the corrected decoration rerun are described above.

| Section start line | Failed section |
| --- | --- |
| 514 | container clicks, recipes, furnace and dispenser world replay |
| 526 | player: movement, every frame and final world |
| 529 | player: damage, every frame and final world |
| 532 | player: pickup, every frame and final world |
| 543 | player: dig, every frame and final world |
| 558 | player: api, every frame and final world |
| 561 | player: pipeline, every frame and final world |
| 566 | player: sleepworld, every frame and final world |
| 569 | player: survival, every frame and final world |
| 572 | player: bucket, every frame and final world |
| 575 | player: combat, every frame and final world |
| 580 | player: digextras, every frame and final world |
| 599 | entities: passive, every frame for 13000 ticks |
| 602 | entities: aquatic, every frame for 3000 ticks |
| 605 | entities: hostile, every frame for 3000 ticks |
| 608 | entities: special, every frame for 3000 ticks |
| 611 | entities: spawning, every frame for 3000 ticks |
| 614 | entities: spawner, every frame for 3000 ticks |
| 617 | entities: client, every frame for 3000 ticks |
| 620 | entities: nbt, every frame for 1000 ticks |
| 623 | entities: nbtall, every frame for 1000 ticks |
| 626 | entities: slimes, every frame for 3000 ticks |
| 631 | entities: combat, every frame for 3000 ticks |
| 634 | entities: stormclient, every frame for 3000 ticks |
| 637 | entities: lightning, every frame for 1000 ticks |
| 640 | entities: rider, every frame for 3000 ticks |
| 643 | entities: nether, every frame for 3000 ticks |
| 944 | remaining entity renderers and GUI item geometry |
| 1065 | EntityPickupFX item and arrow renderer draws |
| 2470 | note instruments, jukebox insertion/ejection and every step/break sound |
| 2481 | EffectRenderer destruction, hits, tint, eviction and vertex bits |
| 2522 | world conversion from the old chunk-file format |
