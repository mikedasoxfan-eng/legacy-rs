# Terrain round 3 owner handoff

Stronghold graph/block/tile generation and real populated grids around all three starts are exact across six signed seeds. The singleplayer provider and alternate cache have original-bytecode lifecycle checks. All six independently generated saves are exact: 2,103 decompressed chunk NBT payloads byte-identical, including entities, with both cross-load directions exact for every seed. The prior entity Motion/Rotation save differences were unpinned harness randomness, rather than a demonstrated entity implementation defect.

Release build passes. No b1.7.3 source, oracle or verifier was edited. The complete `tools/verify181.sh` run finished with **SOME CASES DIFFER**. All 331 focused terrain checks passed (round 1: 157, round 2: 78, round 3: 96). No full-suite pass is claimed. The authoritative log is `target/terrain3-full-verify.log`.

| Owner | Result requiring follow-up |
| --- | --- |
| Integrator | Wire Sky fog/cloud/camera/render behavior to the existing dimension and biome color APIs. Beta 1.8.1 has no portal-room retry or nearest-stronghold API; preserve this version rather than adding later behavior. |
| Integrator / entity renderers | Moving piston renderer: 1,285,876 differing words of 2,564,352. First-person replay: 1,572 differences of 8,411,656 comparisons. Normal terrain meshes, block showcases, living renderers and entity shadow/fire geometry are exact in the completed run. Legacy model/object checks, pickup rendering, sign rendering, sound replay, EffectRenderer and ambient-weather checks also fail or panic; see the index. |
| Entities | Broad passive/aquatic/hostile/special/spawning/spawner/client/NBT/slime/hook/combat/storm/lightning/rider/Nether replays differ. Of the 42 focused entity simulations, 41 are exact and Silverfish allies differs at byte 7012 (17,498 Rust bytes / 17,540 Java bytes). The geometry, models, particles and XP-render focused checks are exact. Natural spawning still emits three unreachable old-biome-alias warnings; audit that adapter, as already identified in `docs/entities181-report.md`. |
| Player-GUI / world-blocks | Movement, pickup, digging, sleeping-world, survival, bucket, combat and dig-extras whole-world replays differ. Container tile replay also differs; inventory/container-only operations are a different boundary. See `reports/b181-player-gui.md`. |
| World-blocks / simulation integration | Thunder/rain/fire/sapling whole-world ticking differs in RNG state, blocks, lighting, metadata and scheduled ticks. The pistons_chunk_nbt and tile_lifecycle redstone replays differ. The focused worldblocks181 items replay also differs at byte 300, in the seed-planting section before NBT checks. The flat fixture uses preloaded populated chunks, so it does not invoke terrain or structure generation. Do not infer that the isolated exact terrain/provider checks certify these combined simulations. See `docs/b181-world-blocks-report.md`. |
| Integrator / server harness | The server-walk oracle does not compile: WorldInfo/WorldServer now require WorldSettings; SaveHandler.func_22096_c and WorldServer.func_6156_d are obsolete. Five javac errors, then ClassNotFoundException; no dump is produced. `target/terrain3-server-oracle-diagnostic.log` captures the failure. The full run labels its missing-file comparison and resulting disk comparison DIFFERS, but these are not valid server algorithm proofs. |

This worker did not rewrite player, entity or renderer behavior. The new deterministic save harness is a separate Mode_terrainsave181 class; it only pins RNGs and calls original bytecode. The comparison reads the same 27 visible chest slots as Oracle.worldBytes while retaining vanilla's 36-slot constructor backing array.

The final playercore, creative, option/key checks and all 23 focused GUI screenshots are exact. Skin-download socket errors are environment diagnostics; both GUI groups still finish pixel EXACT.

Full-run failure index (line numbers refer to `target/terrain3-full-verify.log`; detailed byte/state diagnostics remain there):

| Section | Log line | First failure |
| --- | ---: | --- |
| Vehicle replay (early verifier) | 470 | vehicles differ at byte Some(16417); lengths 18294527/18294527 |
| container clicks, recipes, furnace and dispenser world replay | 578 | thread 'main' (37620) panicked at crates\worldgen181\src\bin\container_compare.rs:26:13: |
| player: movement, every frame and final world | 582 | player differs at byte Some(636663); lengths 4734442/4734442 |
| player: pickup, every frame and final world | 585 | player differs at byte Some(637885); lengths 4765992/4765992 |
| player: dig, every frame and final world | 594 | player differs at byte Some(833791); lengths 6160237/6160229 |
| player: sleepworld, every frame and final world | 612 | player differs at byte Some(2318); lengths 51556499/51556013 |
| player: survival, every frame and final world | 614 | player differs at byte Some(609324); lengths 4689463/4689463 |
| player: bucket, every frame and final world | 616 | player differs at byte Some(72867); lengths 4146957/4146957 |
| player: combat, every frame and final world | 618 | player differs at byte Some(33268); lengths 5088890/5073736 |
| player: digextras, every frame and final world | 622 | player differs at byte Some(6032949); lengths 14218804/14137788 |
| entities: passive, every frame for 13000 ticks | 639 | entities differ at byte Some(204812); lengths 89752884/89752884 |
| entities: aquatic, every frame for 3000 ticks | 641 | entities differ at byte Some(5469); lengths 61769320/61769320 |
| entities: hostile, every frame for 3000 ticks | 643 | entities differ at byte Some(3541); lengths 58793328/58246988 |
| entities: special, every frame for 3000 ticks | 645 | entities differ at byte Some(277134); lengths 72004480/72612882 |
| entities: spawning, every frame for 3000 ticks | 647 | entities differ at byte Some(120); lengths 169357687/170201767 |
| entities: spawner, every frame for 3000 ticks | 649 | entities differ at byte Some(9341); lengths 55934299/55934299 |
| entities: client, every frame for 3000 ticks | 651 | entities differ at byte Some(7533); lengths 78501133/78568059 |
| entities: nbt, every frame for 1000 ticks | 653 | entities differ at byte Some(183217); lengths 53979327/53979327 |
| entities: nbtall, every frame for 1000 ticks | 655 | entities differ at byte Some(9253); lengths 58658858/58658858 |
| entities: slimes, every frame for 3000 ticks | 657 | entities differ at byte Some(3645); lengths 203140893/208646216 |
| entities: hooks, every frame for 1000 ticks | 659 | entities differ at byte Some(10883); lengths 10267807/10360288 |
| entities: combat, every frame for 3000 ticks | 661 | entities differ at byte Some(160661); lengths 60820704/60820704 |
| entities: stormclient, every frame for 3000 ticks | 663 | entities differ at byte Some(3301); lengths 180486178/178595430 |
| entities: lightning, every frame for 1000 ticks | 665 | entities differ at byte Some(4591); lengths 54566727/54566727 |
| entities: rider, every frame for 3000 ticks | 667 | entities differ at byte Some(4245); lengths 68099612/68049032 |
| entities: nether, every frame for 3000 ticks | 669 | entities differ at byte Some(6306); lengths 162678622/167899828 |
| server mode: 900-tick walk (real World.tick), unload/reload through region files | 833 | thread 'main' (35088) panicked at crates\worldgen181\src\bin\compare.rs:1037:36: |
| ticks: 2 3 -77 3000 2000 thunder 0 0 | 878 | tick state differs: |
| ticks: 1 7 999 100 2000 rain 40 -30 | 957 | tick state differs: |
| ticks: -1 5 4242 0 2000 rain -100 60 | 1041 | tick state differs: |
| ticks: forest fire, seed 6 | 1121 | tick state differs: |
| ticks: saplings growing, seed 6 | 1208 | tick state differs: |
| moving piston renderer (original TileEntityRendererPiston vertex bits) | 1304 | piston renderer: 24840 cases, 2564352 vertex words, 1285876 mismatches |
| redstone: pistons_chunk_nbt | 1317 | redstone differs at line 3: |
| redstone: tile_lifecycle | 1364 | redstone differs at line 53: |
| mob models: boxes, UVs, successive poses and wolf living animations | 1405 | thread 'main' (22164) panicked at crates\client181\src\render\entity\check.rs:163:5: |
| remaining entity renderers and GUI item geometry | 1439 | thread 'main' (3696) panicked at crates\client181\src\render\entity\object_check.rs:51:9: |
| first-person view, player and block targeting | 1562 | first person: 19652 cases, 8411656 raw-bit comparisons, 1572 differences |
| EntityPickupFX item and arrow renderer draws | 2268 | renderers: 684 cases, 10224 draws, 693000 bit comparisons, 60588 differ |
| note instruments, jukebox insertion/ejection and every step/break sound | 2974 | thread 'main' (29684) panicked at crates\worldgen181\src\world\tile.rs:45:18: |
| EffectRenderer destruction, hits, tint, eviction and vertex bits | 2985 | thread 'main' (38576) panicked at crates\client181\src\world_effects.rs:260:46: |
| ambient weather vertices | 2990 | thread 'main' (23452) panicked at crates\client181\src\render\weather_check.rs:58:5: |
| sign renderer (decorations) | 3574 | sign renderers: 521440 values, 1920 differences |
| world-blocks b1.8.1: items | 3596 | worldblocks181 items differs at Some(300); Java/Rust 4280/4280 bytes |
| entities181: allies | 4887 | DIFF: allies byte 7012 (Rust 17498 bytes, Java 17540 bytes); actual C:/Users/logiu/Desktop/Projects/minecraft/wt181-terrain/tools/oracle181/out/entity181_allies.rust.bin |
