# Beta 1.8.1 integration, round 5

This round fixes integration inputs without changing Beta 1.7.3 files or the shared `Mode_entities181.world()`, `WorldEventOracle`, or `Oracle.worldBytes` snapshots. No client renderer implementation was edited. The starting checkout already includes later owner work than the round-4 report; improvements already present there are distinguished from this round's changes.

## Changes and class status

| Class / boundary | Status and proof |
| --- | --- |
| EntityOtherPlayerMP fixture / Entity.target_position | Done for the inherited coordinate-only remote target: the original Beta 1.8.1 override returns **1.82F**. A scoped, thread-local fixture guard supplies that eye height during `object_bytes`; it restores the prior value even during unwinding. The actual local player and normal fallback retain **0.12F**. The full passive, aquatic, hostile and NBT replay results below prove the covered boundary. |
| World.positionsToUpdate | Done for the inherited player and entity drivers. `Mode_replayidentities181` records `System.identityHashCode` before each original HashSet insertion, retaining Java's own insertion/traversal behavior. Both the `pipeline` and `sleepworld` drivers use this helper. Rust uses the existing active-chunk replay stream and verifies complete consumption. |
| SpawnerAnimals.eligibleChunksForSpawning | Done for VM identity inputs. The new entity driver copies the inherited script, reuses its original snapshots and world serializer, and records a separate spawning identity stream. Rust's original `perform_spawning_with_identity` receives the recorded hashes, including repeated coordinate objects and persistent set capacity. Normal play retains its existing deterministic local identities. Full spawning replay and the remaining client/Nether results below cover this boundary; surviving gameplay differences are listed separately. |
| Minecraft.runTick input driver | Done for attack/use press timing. The old integration driver replayed Beta 1.7.3's removed `mouseTicksRan` five-tick repeat. The original Beta 1.8.1 uses one-shot attack presses and **field_35001_ab = 6** on use, decremented before input handling; it repeats held use only when that delay expires and no item is being used. The driver now calls the original methods in that order and releases active use through `PlayerControllerSP.func_35638_c`. Its obsolete compatibility clock stays zero, matching Rust's unused field. **API: 200 ticks EXACT; pipeline: 100 ticks EXACT**, including final world bytes. |
| World.tick peaceful-spawn cadence | Done: the original Beta 1.8.1 gates peaceful spawning with `spawnPeacefulMobs && worldInfo.getWorldTime() % 400L == 0L`; Beta 1.7.3 passes `spawnPeacefulMobs` directly. Rust retained the older condition. Porting the literal predicate resolves the frame-2 RNG mismatch in both client variants. **Client and stormclient: 30 ticks EXACT** in scoped checks, followed by the full-length checks in the final suite. |
| compare181 replay sidecars | Done: active `.identities` and spawning `.spawnidentities` streams are big-endian 32-bit VM inputs. Truncation, exhaustion and unused entries assert. Old fixtures without sidecars remain usable. The verifier selects `entityintegration181` and cleans up both sidecars. |
| Tessellator / GL11 fixture acquisition | Done for an opt-in complete GL submission sidecar; Rust renderer parity remains integrator work. `drawintegration181` runs the original object, pickup or effect fixture and writes a companion in the inherited legacy format plus `.draws181`. Each record preserves primitive mode, first vertex, enabled-array mask, texture, GL color and **all eight vertex words**, including packed light at word 7. Disabled attributes are zero. The GL hook returns immediately when capture is inactive. |
| Minecraft renderer colorizer inputs | Done: `Mode_renderintegration181` initializes grass and foliage colors from the installed original jar before invoking the inherited object/pickup drivers. The old fixture left these static buffers zero, making grass black. The full verifier selects this wrapper. After supplying the missing input, the first object difference moves from case 4 (grass tint) to case 208 (stairs scale). The complete capture mode supplies the same colorizer inputs. |
| Renderer object/pickup comparison | Not complete. Existing Rust object checking still consumes seven words and `RawVertex` has no packed-light field. Its scale/material handling has demonstrated differences. Do not discard brightness or reinterpret original quads as Beta 1.7.3 triangles to mark these exact. |
| Inherited EffectRenderer comparison | Done for the existing checked boundary: **256 block/metadata cases, 650,240 vertices, zero differing fields**, plus tint, eviction and updates. This is an inherited improvement over round 4. The fixture calls individual `renderParticle` methods instead of `EffectRenderer.renderParticles`, so it bypasses that method's `func_35835_b(p.func_35115_a(partial))` light setup; complete pipeline/lightmap parity is outside this proof. |
| Terrain / WorldGen / server / saves / portals / respawn / redstone | No new generation algorithm change. These were exact in round 4 and are covered again by the required final full run, including the world-tick spawn-cadence correction. |

The original-source comparison used `EntityOtherPlayerMP.java`, `Minecraft.runTick`/`clickMouse`, `SpawnerAnimals.performSpawning`, `Tessellator.addVertex`, and `EffectRenderer.renderParticles`. The Beta 1.7.3 remote-player class has no eye-height override; Beta 1.8.1 explicitly adds 1.82F. Literal float values and vanilla control flow are retained.

## Verification

The required final `bash tools/verify181.sh` completed with **683 labelled sections: 670 passed, 12 failed, 1 without an explicit section result** (exit **1**). The log is `target/integration5-final-verify.log`; the complete section index is `target/integration5-final-sections.json`, generated by the existing read-only summary tool. Headings without their own result are not counted as passes.

Headings without their own result: player/gui: hunger, experience, FOV, creative catalog, options, keys and pixels.

Scoped checks and fixture audits:

- `cargo build --release -p worldgen181 -p client` succeeds (`target/integration5-build-bash.log`), followed by the final spawn-condition rebuild (`target/integration5-spawnfix-build.log`).
- Passive entities: **100 ticks EXACT**; spawning with recorded identities: **10 ticks EXACT**; sleepworld: **10 ticks and world EXACT**.
- Corrected API: **200 ticks and world EXACT** (`tools/oracle181/out/i5-api-fixed.bin`). Corrected pipeline: **100 ticks and world EXACT** (`target/integration5-pipeline-fixed-rust.log`).
- The current checkout's inherited notes/jukebox port is **EXACT** in the scoped music check (`target/integration5-music-rust.log`): record insertion/ejection and drops, note events, and all 96 step/break sound properties. This round made no music simulation edit. Likewise, inherited player digging/shears corrections already pass both dig scenarios; they are not credited to this round's replay fixes.
- Full capture acquisition, validated by `tools/b181/inspect_draws181.py` (`target/integration5-draw-captures.jsonl`): objects **9,724 quad submissions / 39,056 vertices**, including **192 vertices with nonzero packed light**; Fancy pickup **10,224 quad submissions / 40,896 vertices**; inherited effect fixture **162,560 quad submissions / 650,240 vertices**. These counts validate capture structure, **not** Rust exactness.
- Shell syntax checks, Python compilation and `git diff --check` pass.

Round-5 repairs turn these previously failing inherited sections EXACT in the final full run:

- **Player:** api (200 ticks), pipeline (100), sleepworld (150).
- **Entities:** passive (13,000 ticks), aquatic (3,000), hostile (3,000), spawning (3,000), spawner (3,000), client (3,000), nbt (1,000), slimes (3,000), combat (3,000), stormclient (3,000), lightning (1,000), rider (3,000).

The final full run also reconfirms both save-format directions, **900 server ticks** in memory and on disk, and **4,622/4,622 decompressed saved chunk NBT payloads byte-identical**. All three 2,000-tick weather cases, the 4,000-tick forest fire and **96,000 sapling ticks** are EXACT; Rust's four-day sapling comparison took **146.98 seconds**. Raw terrain, raw Nether, every light-grid schedule, every new-world seed including Fast/Fancy seed 25, and all 16 chunk-mesh graphics/AO combinations pass.

Redstone **pistons_chunk_nbt** and **tile_lifecycle** are EXACT, as are the other scripted redstone sections, four teleporter cases, both use-portal cases and all three respawn cases. First-person rendering passes **19,652 cases / 8,412,376 raw-bit comparisons / zero differences**. The existing model, living-renderer, shadow/fire, showcase and Nether-mesh checks pass without renderer implementation edits.

Two initial full attempts (`target/integration5-full-verify.log` and `target/integration5-before-spawnfix.log`) were interrupted as the API/capture work and then the newly diagnosed spawning condition changed. A temporary capture method-name compile error affected several oracle invocations in the first attempt; it was corrected before the final full run. Neither initial attempt is used for final counts.

## Owner diagnoses and coordination

Sibling status was checked before simulation edits with `git -C ../wt181-<owner> status --short`, using command-local `safe.directory` exceptions for the read-only checks. No global Git configuration was changed. The files edited here were not modified by a sibling at that initial check.

At the subsequent check the entities worker had begun modifying `entity/spawning.rs`. No further edit was made to that file after that check; merge its owner changes with the small optional replay-stream addition. Their `living.rs`, `mod.rs`, `nbt.rs`, player sleep implementation and renderer files were left untouched. World-blocks owns the still-modified world, tile lifecycle, controller, music and conversion files; player-gui owns the still-modified player and GUI files.

Fresh first-difference diagnostics are generated by `target/integration5-diagnoses.sh` into `target/integration5-diagnoses.log`, using the read-only existing `tools/b181/diagnose_replay181.py`. The script waits until the full run leaves its replay section to avoid colliding with compare181's fixed failed-dump paths.

Fresh diagnoses after fixing the input streams are in `target/integration5-owner-diagnoses.log`. The completed full-run failure index is at the end of this report.

| Owner / replay | First differing field and original-source cause |
| --- | --- |
| Player: movement / pickup | Frame **521**, posX: Java **-12.600000001490116**, Rust **-12.5**. `EntityPlayer.onDeath(DamageSource)` applies horizontal death motion whenever the source is non-null; `player_death` instead tests `attacker.is_some()`. Lava has a source with no attacker. |
| Player: damage / survival / bucket | The same source-presence error: frames **542 / 501 / 44**. Bucket posX is **0.3999999985098839 / 0.5**. The player implementation is actively modified by player-gui, so it was left untouched. |
| Player / entities: player combat | Frame **21**, posX: **0.16494847900878684 / 0.47567818038087295**. The script creates an arrow owned by the player and explicitly damages that same player with the indirect source. Java retains the shooter object; Rust removes the player from `World.entities` during the call and `attack_from` resolves its attacker through that map. The self-attacker position is therefore absent. Preserve Java's zero-distance knockback branch and its random draws when the attacking object is the current victim. |
| Entities: special | Frame **24**, event **70**, **smoke.x**: Java **NaN**, Rust **12.179724010702952**. `EntityFireball.onUpdate` calls `newExplosion(null, own position, 1.0F, true)` before setting dead and continuing movement/smoke emission. Java's explosion includes this fireball in its entity query; its direction length is zero, so all three knockback components become NaN. Rust's current fireball is absent from the entity map during `update_fireball`, so its explosion cannot mutate the current object. The vanilla NaN must propagate into the later smoke event; do not sanitize it. |
| Entities: nbtall / nether | Same fireball self-explosion cause. NBT-all: frame **100**, event **72**, smoke.x **NaN / 32.23277277448477**. Nether: frame **40**, event **85**, smoke.x **NaN / 3.157097473104608**. The preceding positions, RNG states and events match. |
| Integrator / entity renderer: objects | With real colorizer inputs, first difference **case 208, id 53**, draw 0, scale operation 2: **0.25 / 0.5**. `RenderItem.doRenderItem` selects 0.5 only for render types **1, 19, 12, 2**, and 0.25 otherwise. Rust `objects.rs` uses a normal-block/slab/piston heuristic, incorrectly scaling stairs. All axes and the subsequent per-copy displacement depend on this scale. Scoped result: **2,004 cases, 9,958 draws, 648,700 comparisons, 7,164 differences** (`target/integration5-colorized-objects-rust.log`). |
| Integrator / entity renderer: pickup | First difference **case 0, id 1**, GL material RGB: **1.0 / 0.0**. `EntityPickupFX.renderParticle` supplies packed light through `glMultiTexCoord2f` and explicitly sets white `glColor4f(1,1,1,1)`; the inherited Rust object path still multiplies by the scalar brightness supplied in the old fixture. Scoped Fancy result: **684 cases, 10,224 draws, 693,000 comparisons, 60,912 differences** (`target/integration5-colorized-pickup-rust.log`). Preserve the lightmap input as well as the white material. |

The decoder now includes the event name and tag in a field diagnosis, distinguishing these smoke NaNs from an explosion-particle ordering mismatch. This changes no recorded data.

The inherited container tile replay still first differs at **byte 1057**. Round 4 traced this to `Chunk.setBlockID/setBlockIDWithMetadata` using local x/z when replacing an absent furnace tile: a burning transition in a negative chunk can queue an empty tile in the positive chunk. Those implementation files are actively modified by world-blocks, so this round leaves that correction with its owner. Container inventory/crafting comparisons are independent.

## Delivery

`target/commit-b181-terrain5.sh` stages only the explicit changed files and this report, and uses the required logiu identity and both requested co-author trailers. `target/commit-b181-terrain.sh` forwards to it for compatibility. Neither script was executed; changes remain uncommitted for integration.


## Final full-run failure index

Line numbers refer to `target/integration5-final-verify.log`. Owner causes are described above; the full JSON index retains the first visible failure for every section.

| Section start line | Failed section |
| --- | --- |
| 496 | container clicks, recipes, furnace and dispenser world replay |
| 508 | player: movement, every frame and final world |
| 511 | player: damage, every frame and final world |
| 514 | player: pickup, every frame and final world |
| 547 | player: survival, every frame and final world |
| 550 | player: bucket, every frame and final world |
| 553 | player: combat, every frame and final world |
| 582 | entities: special, every frame for 3000 ticks |
| 593 | entities: nbtall, every frame for 1000 ticks |
| 608 | entities: nether, every frame for 3000 ticks |
| 904 | remaining entity renderers and GUI item geometry |
| 1009 | EntityPickupFX item and arrow renderer draws |
