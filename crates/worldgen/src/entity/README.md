# Beta 1.7.3 singleplayer entities

`Entity` holds shared physics and living/creature state. `EntityKind` carries the
class-specific state for items, falling sand/gravel, primed TNT, arrows,
fireballs, lightning, pig, sheep, cow, chicken, squid, wolf, zombie, skeleton,
spider, creeper, slime, ghast, pig zombie, and giant.

## Client integration

Add camera/player positions to `World.players`, then call
`World::client_tick(player_index, client_tick_number)` once per game tick. It
joins the surrounding chunks every 30 ticks, updates entities and tile entities,
ticks the world (including natural spawning), and consumes random display draws.
Players are non-ticking, invulnerable targets whose attacks count as successful.
`living::Target` distinguishes entity and player combat sources;
`World::damage_entity` is the public combat entry point.

Worlds default to Normal difficulty and allow hostile and peaceful spawning.
`set_allowed_mob_spawns` controls both categories. Entity construction consumes
an independent `EntitySeedSource`; it supports time seeds, a deterministic
counter, and a caller-supplied closure. `set_entity_determinism` also seeds the
world-owned Math.random stream and independent chest/explosion streams.

Positions, previous positions, body rotation, limb and swing state, health/hurt/
death timers, and all kind-specific animation fields are public. Interpolation
helpers expose swing progress, creeper flash, and wolf interest/shake angles.
No rendering code is included.

## Persistence

Chunk loading constructs supported entities from their vanilla EntityList NBT
names and puts them in the eight chunk slices and world update order. Saving
writes their current state in slice order. Unsupported entity and tile tags are
preserved. Fireball and lightning have no EntityList registration in this Beta
client and are not saved, matching vanilla. Transient AI paths, entity RNG,
projectile owners, and animation state are not persisted by vanilla either.

## Exactness checks

Run `bash tools/verify.sh`; `quick` retains every entity case while shortening
the existing terrain suite. The oracle requires Mojang Java 8u51 and executes
the original client jar. The seed-hook builder recompiles the requested Entity
initializer as a check, then patches only its original Random initializer in the
original class file. Explosion's independent Random initializer gets the same
deterministic treatment. All other bytecode is retained, with a byte-for-byte
reverse-patch assertion. Math.random, entity IDs, World.rand, and chest Random
are seeded explicitly after constructing the real player stand-in.

The suite compares every frame, sorted by entity ID, including position/motion/
rotation bits, bounding boxes, shared and kind-specific state, RNG seeds, weather,
and spawner state. Each replay also compares the existing final world dump:
blocks, metadata, light, height maps, tile contents, scheduled ticks, and lighting
queue size. Cases cover:

* 10,000 geometry/Gaussian/atan2 cases and 10,000 block collision/ray cases across
  every Beta block ID and metadata, retaining singleton block-bound mutations.
* Item fluid/friction/fire/despawn, falling sand/gravel, TNT explosions and drops,
  and arrows through their 1,200-tick ground lifetime.
* 13,000 ticks of passive movement and chicken egg laying.
* 3,000 ticks each of aquatic mobs, hostiles, wolves/ghasts, natural spawning,
  dungeon spawning, client tick order, mounted skeletons/spiders, Nether spawning,
  slime-chunk spawning/splitting, and wolf/pig-zombie combat anger.
* 1,000 ticks of lightning, pig conversion and powered creepers; an additional 3,000-tick
  storm replay exercises the public client pipeline and weather entities. Block hooks
  including chest scattering, digging below sand, TNT ignition, plates, cactus,
  web, soul sand, farmland, and glowing ore display RNG.
* Repeated NBT reconstruction for all 18 persistable kinds, including saddled pigs,
  sheared sheep, tamed wolves, powered creepers, arrows and TNT.

Rust integration tests additionally exercise actual McRegion save/load with
unknown-tag preservation, missing-chunk force gates, re-filing into the top
chunk slice, rejection of joins into missing chunks, unloaded-list removal, and spawner animation reset on chunk reload.
Existing world/terrain/lighting/storage/mesh/server comparisons remain enabled.

Exactness is established for these replay inputs, not every possible game state.
The player inventory/interaction/network simulation and unsupported entities
(boats, minecarts, paintings, and the real player) remain outside this task.
Existing unsupported block machinery such as full redstone/piston circuits and
furnace ticking remains outside the entity port; its existing checks remain.
