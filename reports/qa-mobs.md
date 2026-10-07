# QA: mobs (1.0 client play tests)

Scenarios: `crates/client10/src/playtest/mobs.rs`. Scripts: `tools/playtest10/mobs_*.txt`.
Run: `cargo build --release -q -p client && python tools/playtest10/run.py mobs_blaze ...`
All tests below pass (no panic/hang, screenshots taken, expected positions). I looked at the screenshots of each.

| test | result | bug found + fix |
|---|---|---|
| mobs_blaze | OK. Blaze hovers, fires (flame visible at t60), fireballs hit the player (hearts drop, fire overlay), smoke trail renders | none |
| mobs_magma_cube | OK. Size-4 and default cube render with lava particles, hop, kill the player by t300 (survival) | none |
| mobs_mooshroom | OK. Mooshroom on mycelium in a fence pen; shears right-click turns it into a (brown) cow, red mushrooms drop and are picked up | none (test bug fixed on the way: 1.0 animals hop 1-high walls, so the pen uses fences) |
| mobs_mooshroom_stew | OK. Bowl right-click gives mushroom stew in the same slot, Mooshroom stays | none |
| mobs_snow_golem | OK. Pumpkin placed on two snow blocks by right-clicking the top snow block (aimed down from a 2-high pillar) -> BlockPumpkin.onBlockAdded makes a SnowMan, it wanders and leaves a snow trail | none |
| mobs_villager | OK. Villager wanders 300 ticks | none |
| mobs_enderman_silverfish_cavespider | OK. All three render and run their AI for 300 ticks | none |
| mobs_breeding | OK. Two cows in adjacent pen cells; each fed 1 wheat by right-click (hearts); mating completes about 60 ticks later (hearts stop) and a small calf is visible | none |
| mobs_wolf_tame | OK. Smoke on failed bone attempts, hearts + red collar + sitting on the successful one; bone stack shrinks only while wild | none |
| mobs_nether_mobs | OK. Lit Nether portal (obsidian 4x5, portal 2x3); standing in it ~80 ticks teleports to the Nether (x/8, z/8), walk out, 300 ticks standing there, red Nether fog, lava falls. No ghast/pigman happened to spawn in view in 300 ticks (random) | none. Test bug: walking THROUGH the portal does not teleport (needs ~80 ticks inside, as in Java) |
| mobs_fortress | PARTIAL. Seed 7 has a fortress with start chunk (1,4) (found by porting MapGenNetherBridge.canSpawnStructureAtCoords); a portal at Overworld (150,550) lands at Nether (24, y10, 83) which is inside the fortress' x/z footprint but under it (Teleporter picks a cave floor at y 10, the fortress is at y 64-73). Fortress interior not reached by this test | none |
| mobs_all_entities_soak | OK. All 36 entities `registry::create` can make (everything but EnderDragon, "Mob", "Monster") spawned in a 5-wide grid, 200 ticks, every renderer drew (zombie giant, ghast, snow golem, blaze, crystal, cows, ...). Dragon skipped: it only lives in the End (covered by the journey tests) | none |
| mobs_spawners | OK. Blaze, Cave Spider and Silverfish spawner blocks in a roofed dark chamber: spinning mob in the cage, flame/smoke, mobs spawn and attack. (In daylight nothing spawns: EntityMob.getCanSpawnHere needs darkness, same as Java) | none |
| mobs_killbox | OK. Diamond sword vs weakened Enderman, Zombie Pigman, Blaze and size-2 Magma Cube: attack packets, death animation, XP bar, pickup path run without a problem | none |

## Game bugs fixed

None found in the mobs area: every scenario ran without a panic or an unhandled event, and each behaviour
(blaze fire, mooshroom shearing/stew, snow golem creation, villager, enderman, silverfish, cave spider,
breeding, wolf taming, Nether portal travel, spawner blocks, combat, the full entity registry) matched
EntityBlaze / EntityMooshroom / BlockPumpkin / EntityAnimal / EntityWolf / BlockPortal / TileEntityMobSpawner.

## Notes for later
- Reaching a fortress interior needs either walking in the Nether from the arrival cave or a scenario that can
  start the player in the Nether directly (not available).
- Screen coordinates: ~6.7 mouse counts per degree at default sensitivity (0.15 deg per count).
