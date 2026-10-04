# Beta 1.7.3 world effects

World presentation is now an ordered `WorldEvent` queue. Enqueuing and draining do not
consume random numbers. Sound callbacks retain nullable names, entity yOffset, volume
and pitch; particle callbacks retain position and velocity doubles. Record, auxiliary,
direct SoundManager, and controller hit callbacks preserve their original ordering.
The worker drains once after the player/world tick. The client consumes the events before
its particle tick; audio stays on the window thread. No GUI files were changed.

## Implemented

Supported world/entity/player sound and particle sites include living/hurt/death,
footsteps and landing, water entry/drowning, fire/fluid fizz, explosions, projectiles,
slime contact/jumping/landing, wolves, ghasts, chicken eggs, lightning, spawners,
portal trigger/travel, item and arrow pickup, digging/hits/breaking/placement,
buttons/levers/plates/doors, cave ambience, and ambient block display callbacks.
Notes support all five instruments, pitch changes/wrapping, redstone activation and
saved tile pitch (region-storage regression tested). Jukeboxes support both records, insertion, ejection and dropped items.
Beta 1.7.3 has no chest open/close or food chewing sound at those requested sites.

`EffectRenderer` has four ordered layers and the original 4,000-particle cap per layer.
Ports include EntityFX, smoke/large smoke, flame, explode, rain/splash, bubble, portal,
note, red dust, heart, lava, snowball/slime sprites, snow shovel, digging, footstep and
pickup. Constructors, updates, collision boxes, float/double narrowing, age/death,
brightness, texture selection, fragment UVs, vertex colors, and billboard rotation
follow the original. Lava smoke is inserted and ticked in the same layer pass; the
original eviction/index behavior is preserved. Block hits retain the block singleton's
mutable bounds at the callback. Destroy effects use the original 4x4x4 grid and tint.

Layers use particles.png, terrain.png, gui/items.png and misc/footprint.png. Lit pickup
and footstep effects precede normal particles; pickup delegates item/arrow draws to the
existing bytecode-checked renderers. The stale interpPos origin used by the original
lit pass, its three-tick squared interpolation, and RenderManager's float casts remain.
Leaf fragments retain their construction-time Fancy/Fast atlas choice; dropped and
pickup leaf items read the current choice. Both choices are bytecode-checked.
Fancy pickup shadows and fire use the existing effect geometry with the passed pickup
render position. Runtime entity/renderer randoms use Java time seeds; oracle streams
are explicitly seeded. Changing worlds clears particles while preserving their RNGs.

World sounds pass through the existing RenderGlobal distance gate and SoundManager.
Portal effects bypass the world gate as direct SoundManager calls. Records use the
streaming pool, set the existing Now playing HUD message, stop background music,
ignore the supplied pitch, and retain the
original fixed 64-block attenuation and half sound-volume setting. Existing mixing,
resource naming, Ogg decoding and positional-sound behavior are retained. The original
MusInputStream transformation now decodes encrypted .mus record resources as well.

## Verification

Run `bash tools/verify.sh`. The original Mojang jar runs on the launcher's Java 8u51.
Only deterministic Random initializer hooks are patched; original particle/block/
renderer methods execute unchanged. The existing world, entity and player replays now
compare their entire ordered callback stream every frame, including float/double bits,
in addition to all previous state, random, world and NBT comparisons. A new 360-tick
slime-contact replay exercises player damage followed by the slime attack sound.

Additional original-bytecode checks are included in both full and quick verification:

* 89,600 ambient block display calls across 14 block IDs, all metadata, four neighbor
  scenes and 100 calls per scene; callback bytes and final RNG states.
* All note instruments and pitch wrapping, obstructed notes, both record insertion/
  ejection callbacks and dropped-item states; all 96 step/break sound properties.
* 144 particle cases through 202 frames, with complete state and RNG comparisons and
  465,408 vertices at four partial ticks. Another 64 pickup cases compare 1,024 transforms.
* 256 destroy/hit/tint/bounds scenes through 20 updates, totaling 650,240 vertices,
  plus mixed-layer updates, lava smoke insertion, death removal and cap eviction;
  96 actual RenderGlobal name/distance cases include exactly 16 blocks, the next double,
  unknown names, NaN and infinity. Original bytecode admits NaN through this gate
  despite the decompiler displaying a <= comparison; that behavior is retained.
* 1,368 actual pickup item/arrow render cases in Fancy and Fast graphics:
  20,448 draws and 1,386,000 raw bit comparisons.
* Original MusInputStream bytecode: eight filenames (escaping and Unicode included),
  524,296 transformed bytes; both installed records decoded from .mus and .ogg and
  exercised through headless streaming start/stop.
* Existing weather, shadows/fire, model, entity-renderer, world, storage, light and
  terrain checks remain enabled.

The checks establish exactness for those inputs, before GPU arithmetic. They do not
establish pixel-identical rasterization or an identical OpenAL/cpal audio waveform.
Live scripted wgpu mining and movement were also checked for successful rendering.

## Boundaries

Existing unsupported simulation remains unsupported: boats/minecarts/fishing,
full piston/redstone machinery, furnace ticking and filled dispenser operation.
Their unavailable callback sites cannot fire until those systems are ported. Ambient
particles for their existing block representations are implemented where applicable.
The client still lacks the existing requested dimension-switch integration.

RenderGlobal auxiliary effects read World.rand in the original single-threaded client.
This threaded client replays a captured clone (including Gaussian cache), preserving
all simulation fingerprints and preventing audio/render work from advancing World.rand.
The callback streams themselves are checked against a recording IWorldAccess. Feedback
from a live RenderGlobal into later simulation RNG, shared global Math.random across
threads, moving beyond the finite 41x41 particle terrain snapshot, and end-to-end pickup
shadow lighting are not proven bit-exact. Particle class/draw checks use the original
explicit seeded streams and a complete collision/light snapshot for each tested scene.

Two finished stages can be committed with `bash target/commit-effects.sh` when Git
metadata is writable. It uses captured bytes and a temporary index, preserves unrelated
staged work, refuses uncaptured later edits and records completed commit IDs. Author
and both requested co-author trailers are set by the script.

## Recorded results

`target/verify-effects-complete.log` and `target/verify-effects-final-quick.log`
end with `ALL CASES EXACT`. The final Fast-leaf extensions were then checked directly:
`target/settings-fx-check.log` (256 cases, 650,240 vertices),
`target/pickup-final-check.log` and `target/pickup-fast-check.log` (684 cases each),
and `target/renderers-final-check.log` (the unchanged 1,828 legacy renderer cases).
Every comparison reports zero differing fields/bits. The final release build passed,
as did all 19 tests in `target/effects-tests-complete.log`. Sound resource/stream checks
are in `target/effects-sound-final.log`; live captures are in `target/effects-live-final`.
