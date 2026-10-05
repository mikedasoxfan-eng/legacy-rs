# Beta 1.8.1 integration, round 6

This round repairs inherited vehicle and renderer inputs, fixes the fireball self-explosion discovered in round 5, and restores the original untamed-wolf despawn predicate exposed by the long special replay. Beta 1.7.3 files, `Mode_entities181.world()`, `WorldEventOracle`, `Oracle.worldBytes`, and client renderer implementations are unchanged. The remaining owner-controlled player/container behavior is diagnosed below; renderer capture integrity is explicitly separate from Rust rendering parity.

## Changes and class status

| Class / boundary | Status and original-source evidence |
| --- | --- |
| EntityFireball.onUpdate / Explosion entity query | Done for the diagnosed self-explosion boundary. Vanilla calls `newExplosion(null, own position, 1.0F, true)` before death and continues movement/smoke. The query includes the fireball. Rust removes the updating object from the map, so `projectile.rs` temporarily publishes that current object and retrieves the explosion mutations before continuing. Zero-distance division retains vanilla NaN motion. No Explosion algorithm, RNG order, damage formula or particle position is approximated. Short `special`, `nbtall` and Nether replays each pass 120 ticks, including the previously failing smoke events. Full-length results are recorded below. Other EntityFireball methods remain its owner's existing port. |
| EntityWolf.canDespawn / EntityLiving.despawnEntity | Done for the diagnosed boundary. Vanilla returns !isWolfTamed(); Rust excluded every wolf alongside non-despawning farm animals. At byte 5954943, frame 950, untamed Wolf id 3 reaches entityAge 601: Java consumes nextInt(800), advancing its RNG from Rust 166086901601364 to Java 170502464446415. creature.rs now excludes only wolves with the tame bit (flags & 4). No AI or randomness is approximated. Full special replay evidence is below. |
| VehicleOracle player initialization | Done for the inherited script input. New `Mode_vehicleintegration181` supplies `PlayerControllerSP` on initial/reloaded players, required by original EntityPlayerSP.onLivingUpdate. The old oracle crashed during rider updates. The copied driver reuses unchanged EntityOracle snapshots, WorldEventOracle, world dumps and VehicleOracle's actual statistics hook/list. |
| Vehicle replay canonical NBT | Done. VehicleOracle calls PlayerOracle.canonical, sorting every compound with Java TreeSet. Sorting the Rust insertion vector and then invoking the production HashMap writer did not produce that fixture format. The driver now uses the existing recursive `write_root_canonical`; actual gameplay saves remain unchanged. The original first difference was frame 0, byte 16417, saved minecart 0: Java's Air short appeared where Rust's FallDistance float appeared. |
| TileEntityRenderer / TileEntitySpecialRenderer fixture inputs | Done. Original Minecraft shares the RenderEngine with TileEntityRenderer; the old manager fixture left it null. `bindTextureByName` consequently did nothing and chest-cart geometry retained texture 1 (terrain). The opt-in integration manager supplies the shared engine and stable original chest texture IDs 31/32. This resolves 2,376 texture differences: all 408 vehicle renderer cases, 17,784 draws and 1,741,710 compared fields are EXACT. This is the inherited checked boundary; packed-light and primitive parity are not added to that Rust checker here. |
| Tessellator complete logical draw capture | Done for acquisition. New `Mode_drawtrace181` captures the RendererOracle emit boundary, including display-list playback, instead of only GL submissions during definitions. Its opt-in sidecar retains primitive mode, enabled UV/color/normal/light arrays, all eight vertex words, texture, GL material/normal, render flags, blend/depth state, current unit-1 coordinates and every matrix operation. Common fields equal the unchanged-format companion byte for byte. Inactive capture changes no serialized output. |
| EffectRenderer.renderParticles / layer-3 callback fixture | Done for original pipeline acquisition. New mode invokes actual renderParticles and func_1187_b, covering 18 particle kinds across 36 sky/block-light/partial-tick scenes. Unlike individual renderParticle calls, the first three layers carry packed light. The footprint starts its own Tessellator batch, resetting the light-array enable: the fixture preserves this vanilla behavior. No Rust particle or renderer port is changed. |
| EntityPickupFX unit-1 input fixture | Done for acquisition. 216 additional actual pickup scenes cover item/arrow sources, three ages, four partial ticks and nine light combinations. The inspector verifies the original current lightmap coordinates and white arrow material. The inherited dark-flat pickup fixture alone had zero coordinates, so could not prove this boundary. |
| RenderItem / EntityPickupFX Rust rendering | Not done. The client renderer is reserved for the integrator. Its inherited checker still compares seven words and omits primitive mode/current lightmap; RawVertex has no packed-light member. This round retains those differences and provides complete fixtures for the owner. |
| World / terrain / Nether / saves / server / portals / respawn / redstone | No generation or world algorithm change. The already-present integration repairs are reconfirmed by the final full suite; no previous-round fix is credited as new work here. |
| EntityOtherPlayerMP eye height / World.positionsToUpdate / SpawnerAnimals identities | Already repaired in round 5 and retained: 1.82F stand-in eye height, active `.identities` and spawning `.spawnidentities` input streams, including player pipeline and sleepworld. No further fixture substitution is needed. |

The complete trace format is little-endian: `B181TRC1`, u32 draw count, then per draw u32 mode/mask/vertex-count; the legacy 48-byte texture/material/normal/flags/blend/depth header; two raw f32 current lightmap coordinates; u32 matrix-operation count and 20 bytes per operation; eight u32 words per vertex. Mask bits are UV=1, color=2, normal=4, packed light=8. Disabled attributes are zero. The companion provides original case inputs and draw grouping. Particle/pickup-light companions have dedicated `B181PIP1` / `B181PKL1` scene headers.

## Verification

The final full `bash tools/verify181.sh` completed with **706 EXACT, 10 failed and 1 unverified heading, 717 labelled sections total**, exit **1** (`SOME CASES DIFFER`). Evidence: `target/integration6-final-verify.log`, `target/integration6-final-exit.txt` and the complete machine-readable index `target/integration6-final-sections.json`, produced by `python tools/b181/summarize_verify181.py`. All five new capture-consistency sections pass (log lines 7460, 7465, 7469, 7473 and 7478).

The unverified entry at line 4590 is only the parent heading `player/gui: hunger, experience, FOV, creative catalog, options, keys and pixels`; its nested checks are individually labelled and all pass. No silent test failure is counted as exact. Counts are not directly comparable to round 5's 683 sections: this checkout already contains additional owner checks, and this round adds ten vehicle labels and five fixture-consistency labels. Of the 706 exact sections, five prove capture integrity rather than Rust renderer parity.

Completed scoped evidence:

- Release builds with `cargo build --release -p worldgen181 -p client` pass (`target/integration6-build.log`, `target/integration6-vehicle-build.log`).
- Fireball short replays: special, nbtall and Nether, each 120 ticks EXACT (`target/integration6-*-rust.log`). The wolf correction additionally passes 1,000 special ticks (`target/integration6-special-wolf-rust.log`); the final full special replay passes all 3,000 ticks.
- Vehicles: rails, train, boats, riding, nbt, longrail and placing, each 240 ticks EXACT. Inventory/NBT: 10,000 operations EXACT. Ridingnbt exposes the owner death-motion issue below (`target/integration6-vehicles-fixed.log`).
- Corrected vehicle renderer input: 408 cases / 17,784 draws / 1,741,710 comparisons / zero differences (`target/integration6-vehiclerenderers-rust.log`).
- Complete object acquisition: 2,004 cases / 9,958 logical draws / 39,992 vertices / 192 vertices with nonzero packed light. Round 5's submission-only sidecar had 9,724 submissions / 39,056 vertices, including definitions but omitting logical playback.
- Complete Fancy and Fast pickup acquisition: each 684 cases / 10,224 draws / 40,896 vertices. Current-lightmap acquisition: 216 scenes / 1,296 draws / 5,184 vertices / 1,152 draws with nonzero current unit-1 coordinates.
- Actual EffectRenderer acquisition: 36 scenes / 144 draws / 2,592 vertices / 108 packed-light draws / 2,204 vertices with nonzero packed light. These counts prove capture structure and source inputs, not Rust parity.
- The inspector rejects truncated captures, wrong primitive counts, nonzero disabled packed-light words and trailing bytes. Bash syntax, Python compilation and `git diff --check` pass.

The completed whole-game sections in the final run are EXACT: raw Overworld/Nether including Far Lands; lighting schedules/grids; all six new-world seeds and seed 25 Fast/Fancy; both save/load directions; the 900-tick server walk in memory and on disk; weather, fire and sapling ticks; all redstone scripts including `pistons_chunk_nbt` and `tile_lifecycle`; four teleporter, two use-portal and three respawn cases; and old-world conversion. The server save check compares 4,622 decompressed chunk payloads and 518 sector layouts. Conversion compares 289 chunk payloads, 32 sector layouts and all 307 progress steps. These payload/layout checks do not imply compressed region files are byte-identical. All 20 full entity replay sections pass.

The five new delimited end-of-suite sections are explicitly named **capture consistency**. Their exactness means fixture integrity, not full Rust renderer correctness. Vehicle checks now label all ten sections, continue after failures, retain the original oracle errors, and explicitly select `--mc b1.8.1` for renderer checking. The previous script stopped at the first difference and suppressed the next oracle's crash output, hiding later checks.

Sections made EXACT by this round's simulation fixes are `entities: special, every frame for 3000 ticks`, `entities: nbtall, every frame for 1000 ticks`, and `entities: nether, every frame for 3000 ticks`. The repaired vehicle harness additionally proves the seven replay scenes listed above, 10,000 inventory operations and the original vehicle renderer inputs; these are newly labelled/reachable checks, not a ten-section improvement against round 5's denominator. The five complete-draw checks add fixture-consistency evidence only.

Initial runs were interrupted before completion as the vehicle canonical/controller/texture fixture failures and then the later wolf despawn mismatch were discovered. `target/integration6-baseline-verify.log`, `target/integration6-before-vehiclefix.log` and `target/integration6-before-wolffix.log` are diagnostic attempts, not final suite evidence.

## Final failure index

Line numbers refer to `target/integration6-final-verify.log`. The owner table below supplies the first differing field and original-source cause.

| Section | Heading / first failure line | Result |
| --- | --- | --- |
| vehicles: ridingnbt, 240 ticks, canonical NBT and final world | 502 / 503 | byte 649240 |
| container clicks, recipes, furnace and dispenser world replay | 518 / 527 | world comparison fails; byte 1057 in retained replay |
| player: movement, every frame and final world | 530 / 531 | byte 636663 |
| player: damage, every frame and final world | 533 / 534 | byte 866689 |
| player: pickup, every frame and final world | 536 / 537 | byte 637885 |
| player: survival, every frame and final world | 569 / 570 | byte 609324 |
| player: bucket, every frame and final world | 572 / 573 | byte 72867 |
| player: combat, every frame and final world | 575 / 576 | byte 33268 |
| remaining entity renderers and GUI item geometry | 925 / 943 | 6,912 differing fields; first case 208, id 53 |
| EntityPickupFX item and arrow renderer draws | 1022 / 1039 | 60,912 differing fields for each of Fancy and Fast; first case 0, id 1 |

## Coordination and owner diagnoses

All three sibling statuses were read before each edit outside tools. Command-local safe.directory exceptions were needed for these read-only checks; global Git configuration was unchanged. None of projectile.rs, creature.rs or vehicle_verify.rs was modified in any sibling. Player/entity and controller/inventory files were actively modified and left untouched. The world-blocks worker was modified during the initial audit; its container/world lifecycle work is left for that owner. No client render directory, worker.rs or app.rs was edited.

Fresh replay diagnoses use `target/integration6-diagnoses.sh`, which waits until the final verifier leaves player/entity replays to avoid overwriting their fixed failure paths. Evidence: `target/integration6-owner-diagnoses.log`. The read-only decoder now handles vehicle headers, rail samples, statistics and nested canonical NBT.

| Owner / replay | First difference and source cause |
| --- | --- |
| Player / entities: movement, damage, pickup, survival, bucket | Fresh diagnoses reconfirm the round-5 source-presence issue: movement byte 636663/frame 521, damage byte 866689/frame 542, pickup byte 637885/frame 521, survival byte 609324/frame 501, bucket byte 72867/frame 44. Original EntityPlayer.onDeath applies horizontal death motion whenever DamageSource is non-null. Rust player_death checks attacker.is_some(), so lava/suffocation and other sources without an attacker lose that motion. The first four posX values are Java -12.600000001490116 / Rust -12.5; bucket is Java 0.3999999985098839 / Rust 0.5. |
| Player / entities: combat | The round-5 self-attacker query issue remains. Original indirect-arrow damage retains the shooter object, including when shooter and victim are the same player. Rust removes that player while applying damage and cannot resolve the attacking object through World.entities, skipping Java's zero-distance knockback RNG branch. Fresh diagnosis: byte 33268, frame 21, player posX Java 0.16494847900878684 / Rust 0.47567818038087295. |
| Player / entities: vehicles ridingnbt | Byte 649240, frame 122, Player id 24 posX: Java -8.799999989569187, Rust -8.699999988079071. This is another occurrence of non-null-source death motion with no attacker. The preceding snapshots match after the fixture repairs. |
| World-blocks: container world replay | Byte 1057, frame 1, furnace index 9 at (6,64,8), slot 0 present: Java 0, Rust 1. Original Chunk.setBlockID/setBlockIDWithMetadata uses local x/z when queuing a replacement tile after lazy lookup returns null during scanning. Burning a negative-chunk furnace queues an empty tile in the positive chunk and replaces this furnace. Preserve this vanilla coordinate bug. Inventory/crafting comparisons are separately exact. |
| Integrator / entity renderers: objects | Original RenderItem scale is 0.5 only for render types 1,19,12,2 and 0.25 otherwise. Rust's normal-block/slab/piston heuristic incorrectly scales stairs (case 208, id 53, first scale operation: Java 0.25 / Rust 0.5). Matrix displacement also depends on the scale. |
| Integrator / entity renderers: pickup | Original EntityPickupFX sets unit-1 coordinates via glMultiTexCoord2f and white GL color. Rust still multiplies material by the scalar brightness in the inherited fixture. First case, id 1: Java material 1.0 / Rust 0.0. The new complete capture retains current lightmap and per-vertex packed light so neither is discarded to claim parity. |

## Delivery

`target/commit-b181-terrain6.sh` stages only this round's explicit files and this report, with the requested logiu author/email and both co-author trailers. `target/commit-b181-terrain.sh` forwards to it. Git metadata was not written and neither script was executed.
