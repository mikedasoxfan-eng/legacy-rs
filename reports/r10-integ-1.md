# Minecraft 1.0 integration fixes, round 1

Worktree `wt181-r10integ` (branch `w181-r10integ`), with `b1.8.1` merged in (renderfx, world rounds 4 and 5).
I reran every section individually after the last change. All 25 player replays and all 20 entity replays in
`verify10.sh` are EXACT, not only the ones on the list. `cargo test -p worldgen10` passes, the oracle compiles,
and the 1.0 smoke screenshot is fine.

| section | EXACT now / still differs | root cause + fix |
|---|---|---|
| statistics file | EXACT | Harness bug. `beta --statsfile` went through `r10_client`/`compare10.py`, which runs the binary in an isolated work directory, so the relative `rust/stats/...` files were never written in `$sf`. The section now calls the client directly (`statsbeta`). The original's "999999 is not a valid stat" and "CHECKSUM MISMATCH" lines are only printed; the rows filter already ignored them. |
| decorations10: fishing | EXACT | 1.0 rod breaks call `EntityLiving.func_41005_b` (broken-item particles), which draws from the angler's `rand`. In the Java fixture the angler was created before `OracleHooks.reset`, so its seed carried over from the previous cast. Both sides now seed the angler with `reset(90+seed)` / `Deterministic(90+seed)`. |
| entities4: fishing | EXACT | Same oracle mode and fix as above. |
| player-gui 1.0 round 4: statistics comparators | EXACT | `stats_table.rs` was still the 1.8.1 StatList, and `tools/oracle10/statlist.txt` was stale. Regenerated from `oracle.sh statlist` on 1.0 with `tools/oracle10/gen_stats_table.py`; this adds the mined/used stats for 1.0 blocks 110-122 and items 369+. |
| player-gui 1.0: options and keybindings | EXACT | Same stats-table fix. |
| gui10: b10_controls_duplicate, inventory_signed_counts, b10_lightmap3_chest, b10_creative_to_inventory | these 4 shots are EXACT; other shots in their groups still differ | These 4 are pixel-exact, also in r10-full-3. They are only blamed because they are the last shot of a group, and the group's "SOME GUI CAPTURES DIFFER" line is attributed to them. The shots that actually differ are renderer ones: `b10_lightmap*_hurt` and `inventory_hurt*` (hurt-tint overlay on the inventory player model, about 4.7k px, max 22), `inventory_bow` (458 px), `b10_creative_first/scroll/cursor` and `b10_inventory_to_creative` (3 px, max 1, at item icon 351,215), and `b10_lightmap1_items` (1 px). These belong to the render worker (RenderLiving hurt overlay, RenderItem/RenderBlocks inventory icons, ItemRenderer bow pose). |
| entities3: factorybranches | EXACT | factorybranches itself was EXACT. The failure came from the following `modelregistry3` check, which called `ModelBase.func_35393_a` (random part). That method was removed in 1.0, so I dropped that part from `Mode_modelregistry10/3_10` and from Rust `registry_check`. The check also compared the last cube's bounds, but `Harness10.modelBound` reads `cubeList.get(0)`; fixed. I added `==` headers for the four checks after the loop (model registry, new living renderers, signs, fish) so failures are attributed correctly. "new living renderers" still differs (texture binding java 0 / rust 7): render worker. |
| entities6: sleep6 | EXACT | `SpawnerAnimals.performSleepSpawning` was removed in 1.0. `Harness10.removedSleepSpawning` now returns false (nothing spawns) instead of throwing, and the Rust side writes false. |
| entities6: spawninputs6 | EXACT | 1.0 `eligibleChunksForSpawning` is a HashMap. An edge chunk is put (as true) only when absent, so no identity hash is recorded for it. Rust consumed an identity for every chunk, so the recorded stream ran out (panic). `perform_spawning_with_identity` now skips edge chunks that are already present. |
| entities7: vineconstructor7 | EXACT | 1.8.1 `ItemVine` became 1.0 `ItemColored(id, subtypes)`, and its colour is now `Block.getRenderColor(damage)`. The fixture now builds `ItemColored` and probes `getRenderColor(int)`. `VineItem::color` takes the damage. |
| entities8: Wolf despawning | EXACT | Already exact after the merge; no change. |
| minecraft 1.0: runTick event queues ... | EXACT | Four causes. (1) Fixture: in creative, an attack press and a pick press in the same tick pick the block that was just broken, and 1.0 then NPEs on `Item.itemsList[0]`, a crash path the Done standard excludes. The scenario-6 attack key is now a release on that tick. (2) 1.0 `clickMouse(1)` sets `rightClickDelayTimer = 4` (was 6). (3) 1.0 runTick drains attack/use/pick presses while an item is in use, in both `minecraft_input::run_tick` and `apply_player_tick_input`. (4) Mob-spawn policy uses `theWorld.difficultySetting`, which is 1 in multiplayer. Clicking the middle mouse button now ports 1.0 `setCurrentItem(id, meta, hasSubtypes, creative)`. |
| minecraft 1.0: retained clickMouse and sendClickBlockToController effects | EXACT | Same middle-click NPE: the fixture pick-pressed a block that held-attack had already broken. The pick press is now skipped when the target is air, on both sides. The fixture now drains presses while an item is in use, as 1.0 does. |
| minecraft 1.0: retained saved-world controller selection | EXACT | Exact after the merge; no change. |
| entities10: entitylist10 | EXACT | The Rust side walked the 1.0 registry while the fixture walked 28 hard-coded 1.8.1 names. Both now walk all 39 `EntityList.addMapping` entries in id order. The factory NBT model was extended to the 1.0 readers: EntityAnimal `Age`/`InLove` ints, Villager `Profession`, LavaSlime `Size`, Fireball/SmallFireball tiles (no `shake`), ThrownEnderpearl, Enderman `carriedData`, and EnderCrystal (id 200) is not living. `EntityEnderCrystal(World)` keeps the 0.6-wide box from the Entity constructor, because `setSize` does not move it; added `ender_crystal_base`. |
| entities11: stacknbt11 | EXACT | No change (suite is all EXACT). |
| player: damage | EXACT | 1.0 moved `damageRemainder` from EntityPlayer to EntityLiving (`field_40129_bA`). The trace wrote the stale `PlayerState.damage_remainder`; it now writes `living.damage_remainder`. |
| player: activate | EXACT | The 1.0 BlockDoor upper half still forwards only power-providing neighbour changes (`canProvidePower()`). The Rust upper half forwarded every change, which toggled the door back and played 1003 three times. Blocks area; a one-line fix. |
| player: night | EXACT | performSleepSpawning was removed in 1.0. The fixture threw; both sides now skip it. |
| player: digextras | EXACT | Same door fix. |
| player: slimecontact | EXACT | EntitySlime faces the player with `faceEntity(p, 10, 20)`. Rust passed pitch speed 0, because the slime's `getVerticalFaceSpeed()==0` only applies to the base AI. |
| entities: aquatic, nbtall | EXACT | Already exact. |
| entities: spawning, slimes, client | EXACT | Fixture: `EntityOracle.FlatProvider.func_40377_a` returned an empty spawn list, so 1.0 `World.func_40474_a` never spawned anything. It now returns the biome list, like ChunkProviderGenerate/Hell. |
| entities: lightning, stormclient | EXACT | 1.0 `Entity.isBurning()` is `fire > 0 \|\| getEntityFlag(0)`. Rust only used the flag in multiplayer, so a cow that died in the tick after water put its fire out dropped raw instead of cooked beef. Also, `onStruckByLightning` now uses `func_40046_d(8)` (160 ticks, was 300); this also fixed entities4: lightning4. |
| entities: special | EXACT | 1.0 `getLook(1.0F)` multiplies by the folded constant `(float)Math.PI/180.0F`. Rust computed `yaw*PI/180`, which rounds differently and moved ghast fireball spawn points by one sine-table step. |
| entities: nether | EXACT | 1.0 `Chunk.getBlockLightValue` ignores skylight when `worldProvider.hasNoSky`. The Rust replay fixture built nether chunks without setting `has_no_sky10`, so wander-path light weights differed. |

Also fixed while there:
- decorations10 maps no longer panics. `MATERIAL_COLORS` gained the 1.0 dragon-egg material (index 25); world/maps area, small fix.
- entities10 `models10` no longer asserts on two-cube parts: the check now compares every cube's corners.

## Still differing in the suites I touched (not on my list; owners noted)

- **Render worker:**
  - entities3/4/10 "new living renderers" (texture binding).
  - entities4 `livingrenderers`, `playerliving4_181` and entities6 `playerliving6_10` (object_check.rs:51 "matrix op count").
  - entities4 weather_check.rs:77 vertex.
  - entities10 `models10` (`silverfishBodyParts[0]` missing).
  - The gui10 pixel shots listed above.
- **Blocks/items worker:**
  - entities7 `metadata7` (`getIconFromDamage` 39 vs 14), `buckethit7` and `bucketray7` (`v.xCoord`).
  - entities10 `itemblock10` (events.size 3 vs 2).

## 1.0 classes whose port code changed

- **EntityLiving:** `isBurning`, `getLook(1.0F)` via `look_vector`, damageRemainder trace.
- **Entity:** `onStruckByLightning`.
- **EntitySlime:** faceEntity pitch speed.
- **EntityEnderCrystal:** World-only constructor.
- **EntityList:** factory NBT validation for the 1.0 entity readers.
- **SpawnerAnimals:** edge-chunk put.
- **BlockDoor:** upper-half `canProvidePower`. Blocks area.
- **Minecraft:** `clickMouse` delay 4, runTick press draining while using, `clickMiddleMouseButton` subtypes, world mob-spawn difficulty.
- **InventoryPlayer:** `setCurrentItem(id, meta, subtypes, creative)` is now used.
- **ItemColored:** was ItemVine.
- **StatList:** regenerated table.
- **MapColor/Material:** map colours. World area.

Fixtures changed (oracle10): `Harness10`, `EntityOracle.FlatProvider`, `DecorationOracle.fishing`, `Mode_controllerinput10`, `Mode_runtick10`, `Mode_entitylist10_10`, `Mode_itemvine7_10`, `Mode_modelregistry10`, `Mode_modelregistry3_10`, and `statlist.txt`.

Suites changed: `verify10.sh` (statistics file section) and `verify_entities3_10.sh` (headers).
