# Documentation

Design notes and verification coverage for each part of the port. The top-level
[README](../README.md) is the overview; these go into detail: what each subsystem ports,
which original classes it follows, how it is checked against the original bytecode, and the
original quirks it keeps on purpose.

## Subsystems

| Document | Covers |
|---|---|
| [Container screens](containers.md) | Inventory, crafting, chests, furnaces and dispensers: the world-thread container sessions and the screen contract |
| [Pistons and tile entities](pistons.md) | Normal and sticky pistons, note blocks, the tile-entity object lifecycle, moving-piston rendering |
| [World effects](effects.md) | Particles, world and entity sounds, records, the ordered event stream |
| [First-person view](first-person.md) | Camera, FOV and fog, the hand and held items, block outline and cracks, third person and the player model |
| [Vehicles](vehicles.md) | Minecarts, boats, riding, chest-cart inventories, their renderers |
| [Decorations and fishing](decorations.md) | Signs, paintings, maps, fishing |
| [Texture packs, chat, anaglyph](texture-packs-chat-anaglyph.md) | Texture-pack loading and live refresh, the chat screen, the 3D anaglyph mode |
| [Video settings](video-settings.md) | Fancy/Fast, smooth lighting, render distance, frame-rate limit, GUI scale |
| [Weather](weather.md) | Rain and snow rendering, splashes, lightning, sky darkening |

## Inside the source tree

| Document | Covers |
|---|---|
| [crates/worldgen/src/entity](../crates/worldgen/src/entity/README.md) | Entity physics, AI, spawning and NBT |
| [crates/worldgen/src/entity/player](../crates/worldgen/src/entity/player/README.md) | The player simulation |
| [crates/worldgen/src/container](../crates/worldgen/src/container/README.md) | Container logic and recipes |
| [crates/client/src/render/entity](../crates/client/src/render/entity/README.md) | Mob, item and entity renderers |

## Verification

Every document's claims are backed by `tools/verify.sh` (the simulation and renderer inputs,
bit for bit) and `tools/verify_gui.sh` (screens, pixel for pixel, against the real client).
See [How exact is exact?](../README.md#how-exact-is-exact) for how the oracle works.
