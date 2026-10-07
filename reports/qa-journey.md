# QA journey: dimensions, the dragon, the credits, death and respawn (Minecraft 1.0)

Scenarios: `crates/client10/src/playtest/journey.rs` (plus a crash-audit test module in the same
file). Tests: `tools/playtest10/journey_*.txt`. Oracle capture of the original 1.0 client in the
End: `tools/oracle10/journey/endshot.sh` (`EndShot.java`).

Scenarios can now have an *arrival part* (`journey::ARRIVALS`, run by `playtest::arrived` from the
worker right after `Minecraft.usePortal` switched worlds), so the End is always reached through a
real lit End portal and only then prepared (a wounded or dying dragon, a walkway).

## Results

| test | result | bug found + fix |
|---|---|---|
| `end_portal` (existing) | EXACT | It passed only because the End's loading time ran on the script clock while W was held; with the clock fixed (below) it passes as written. |
| `journey_end_idle` | EXACT (no crash; 400 ticks in the End) | Dragon flies, crystals heal it (150 -> 160 -> 172 -> 184 logged), boss bar shows. The End's fog/clear colour followed the Overworld's time of day and blue biome sky (fixed, see Part C). The dragon sometimes knocks the player into the void, as in the original, so later positions are not checked. |
| `journey_dragon_exit` | EXACT | **Exit portal had no portal blocks in the End**: `dragon_exit_portal` did not set `BlockEndPortal.field_41051_a`, so every End portal block it placed was removed again by `BlockEndPortal.onBlockAdded` (worldType != 0). Fixed. Screens checked: death beams, XP orbs (player went to level 54), bedrock/torch/egg/portal pool, black fade-in, logo, poem ("Player?"), Escape -> Overworld spawn with sword, 16 pearls, potion and the XP kept. |
| `journey_nether_round_trip` | EXACT | Walk into a built Nether portal, 80 ticks, Nether (portal built near x/8, z/8), step out, cooldown, step in, back in the Overworld. No game bug; with `--no-save` the Overworld is regenerated on return, so the Teleporter builds a new portal near 8x the Nether position instead of finding the original (expected given no save; noted in the test). |
| `journey_end_death` | EXACT | **Dying in the End respawned the player in the Nether**: the worker's respawn used `use_portal` (Nether<->Overworld toggle) whenever `is_hell_world()`, which is true for the End. Now `if !canRespawnHere() usePortal(0)` as `Minecraft.respawn`. Player respawns at the Overworld spawn (dimension 0). |
| `journey_hardcore_death` | EXACT | Hardcore GuiGameOver ("Game over!", hardcore notice, only "Delete world"). No bug. |
| `journey_end_view` | EXACT | Same three views as the oracle capture; pixel colours match the original (Part C). Shots are taken early: in a 200-tick version the dragon knocked the player off the platform. |
| unit `playtest::journey::audit::every_block_and_metadata_meshes` | pass | Every block 0..=122 x metadata 0..=15, packed and isolated, Overworld/Nether/End, fancy/fast, AO on/off. |
| unit `playtest::journey::audit::every_entity_renders` | pass | Every EntityList entity plus XP orbs, lightning, falling blocks, items of every id/damage (world and GUI icon + durability bar), thrown/item potions, the dying dragon: CPU side of the renderer. |
| unit `playtest::journey::audit::end_fog_color_is_world_provider_end_s` | pass | End fog equals WorldProviderEnd.getFogColor pulled toward the black End sky. |
| unit `entity::dragon::tests::exit_portal_keeps_its_portal_blocks_in_the_end` (worldgen10) | pass | Regression test for the exit portal bug. |

## Part A: event audit

`PlayerEvent` (worldgen10 entity/player.rs) - all variants handled: Stat, Chat, EditSign,
OpenVehicleInventory/OpenContainer (worker `activation_request`), Container (Closed/Achievement/
Crafted in app; Drop and MapCreated are consumed on the world thread in
`deliver_container_events`), Pickup, ChangeDimension and ChangeDimensionTo (worker), WinGame,
MagicCriticalHit, CriticalHit. `SpawnVehicle` is informational only (the vehicle entity is already
spawned by the simulation), so ignoring it is correct. `ToWorker` and `FromWorker`: every variant is
matched. `BlockTiles.events` is dropped on purpose (note/piston sounds and particles also go through
`world.events`); `container_effects` is a test log (dispensers also call `auxiliary_effect`).
`WorldEvent::Auxiliary` codes 1000-1009 and 2000-2004 checked against `RenderGlobal.playAuxSFX`.

Gaps found and fixed:
- `world_effects.rs`: aux 1005 (jukebox) only knew records 2256 "13" and 2257 "cat"; the nine other
  1.0 discs (2258-2266, `Item.recordBlocks`..`record11`) played silence and showed no "Now playing".
  Now all eleven (`ItemRecord.recordName`).
- Worker respawn in the End -> Nether (above).
- The End used the Overworld's celestial angle, biome and fog: compass and clock did not spin there
  (`TextureCompassFX`/`TextureWatchFX` spin when `worldProvider.isNether`, true for the End), and the
  fog/clear colour was wrong (Part C).
- `worker::biome_source`: the End used the Overworld's biome source; now the sky biome
  (`WorldChunkManagerHell(BiomeGenBase.sky, 0.5, 0)`), the Nether keeps hell.

Panics on 1.0 paths: the unit tests above found none in the renderer. The only panic the block
test hit was in the simulation (a powered piston with metadata facing 6/7 indexes past the
direction table) - the original throws the same ArrayIndexOutOfBounds there and no game path
produces that metadata, so it is not changed.

## Fixes (game bugs)

| file | what | follows |
|---|---|---|
| `crates/worldgen10/src/entity/dragon.rs` | `dragon_exit_portal` sets `END_PORTAL_ALLOW` around the build, so the exit portal keeps its portal pool in the End | `EntityDragon.func_41012_a` (`BlockEndPortal.field_41051_a = true/false`) |
| `crates/client10/src/worker.rs` | respawn (death and end credits) leaves any dimension where you can't respawn for the Overworld | `Minecraft.respawn`: `if (!worldProvider.canRespawnHere()) usePortal(0)` |
| `crates/client10/src/worker.rs` | after a world switch no ticks are caught up; at most 10 ticks per loop (was 20) | `Timer.updateTimer` (frame over 1 s: no catch-up; `elapsedTicks` capped at 10) |
| `crates/client10/src/worker.rs` | End biome source = sky | `WorldProviderEnd.registerWorldChunkManager` |
| `crates/client10/src/app.rs`, `render/sky.rs` | End celestial angle 0, End fog colour, End temperature source | `WorldProviderEnd.calculateCelestialAngle/getFogColor`, `BiomeGenEnd.getSkyColorByTemp` = 0, `EntityRenderer.updateFogColor` |
| `crates/client10/src/client_textures.rs` | compass and clock spin in the End | `TextureCompassFX`/`TextureWatchFX` (`worldProvider.isNether`) |
| `crates/client10/src/world_effects.rs` | all 11 record names | `RenderGlobal.playAuxSFX` 1005 / `ItemRecord.recordName` |

Test infrastructure (not game behaviour): the script clock stops while a loading screen is up
(`app.rs`; the original runs no ticks inside `changeWorld`), `playtest::arrived` + worker calls,
script shot lines also print dimension, health, XP level, dragon health, XP orbs, yaw/pitch.

## Part C: the End's colours

Captured the original: `tools/oracle10/journey/endshot.sh` starts a seed-7 world in the real 1.0
client, calls `Minecraft.usePortal(1)` (same arrival platform at 100, 0) and saves three views
(`island` yaw 90, `ground` pitch 45, `sky` yaw 270 pitch -30); `journey_end_view` takes the same
views in the port. The original End **is** dark and greenish: `EntityRenderer.updateLightmap` for
worldType 1 is `0.22/0.28/0.25 + blockLight * 0.75` (End stone has no sky light), so unlit end
stone comes out around RGB 31,37,25 in both. Region means (java / port): island foreground
22.9,28.6,19.5 / 23.2,29.0,19.7; ground near 29.4,36.9,24.9 / 29.9,37.4,25.3; obsidian 3.8,4.3,6.2
/ 3.8,4.3,6.2; sky 11.2,10.6,11.3 / 11.2,10.6,11.3; sampled pixels identical. What did differ was
the fog/clear colour: the port used the Overworld's (time of day, blue biome sky); the original's
is `0x8080A0 * 0.15` pulled toward the black End sky (printed by the oracle: 0.0531, 0.0531,
0.0664), now matched. So there is no pale-beige End to restore: 1.0 renders it this dark.

## Verification

- All 7 play tests pass (`python tools/playtest10/run.py`), twice in a row.
- `cargo test -p client10 --lib` (29) and the worldgen10 dragon tests pass.
- Focused oracle suites after the worldgen10 change, all EXACT (108 checks):
  `verify_r10entities2.sh --built dragonhistory dragondamage dragonflight dragondeath endereye`,
  `verify_r10entities3.sh --built` (incl. player-end-travel, player-respawn-copy),
  `verify_r10blocks3.sh --built`, `verify_r10blocks4.sh --built` (incl. the END_PORTAL_ALLOW cases).
