# Pistons, note blocks and tile lifecycle

The simulation ports the original singleplayer `World`, `Chunk`, piston blocks,
`TileEntityPiston`, `BlockNote` and `TileEntityNote`. The oracle calls Mojang's
original Beta 1.7.3 bytecode under Java 8u51; decompiled source is used to guide
the port, never as the reference implementation.

## Verification

Run `bash tools/verify.sh quick` while iterating, or `bash tools/verify.sh` for the
full regression suite. Both run every `tools/oracle/redstone/*.txt` script and
the moving-piston renderer comparison. A passing run ends `ALL CASES EXACT`.

For one script:

```bash
bash tools/oracle/oracle.sh redstone "$(cygpath -aw tools/oracle/redstone/pistons_pulses.txt)" piston_case.bin
target/release/compare.exe redstone tools/oracle/redstone/pistons_pulses.txt tools/oracle/out/piston_case.bin
```

For the renderer alone:

```bash
bash tools/oracle/oracle.sh pistonrender pistonrender.bin
target/release/beta.exe --pistoncheck tools/oracle/out/pistonrender.bin
```

`PistonRendererOracle` captures the original `TileEntityRendererPiston`'s actual
Tessellator output. The 24,840 cases compare 3,935,232 raw words: vertex position,
UV and packed RGBA. Cases cover all six facings, 23 stored block IDs, extending
heads, retracting heads and bases, three progress phases, five partial ticks,
negative coordinates and chunk edges, and all four Fancy/Fast and AO settings.

Redstone cases include normal and sticky pistons pushing lengths 1 through 12
and blocked length 13 in every facing; height limits; quasi-connectivity;
obsidian, bedrock and tile blockers; destroyable blocks and their drops;
pistons pushing pistons; failed pulls; pulses before and after completion;
torch clocks; pushed item and mob position, velocity and bounding-box bits;
placement facing; collision and selection bounds; breaking heads, bases and
moving blocks; and chunk/NBT round trips during movement. Note cases exercise
all 25 pitches and five instruments, activation, clicking, redstone edges,
occlusion, sound/particle event bits and NBT clamping.

## Completed validation (2026-10-04)

The full `tools/verify.sh` run ended `ALL CASES EXACT`, including all 21 redstone
scripts (19 added here), the 24,840-case piston renderer check, 65,706 container
records, and the 1,000-tick container tile replay. The final same-chunk identity
probe and all six entity-push directions were included in this run.
`cargo check --workspace`, the release build, and `cargo test -p worldgen --lib`
also passed. The local full-suite transcript is `target/verify-pistons-full.log`.

## Extended script commands

The existing `place`, `remove`, `use`, `tick`, and `check` commands retain their
meaning. `tick` and ticked `check` run `updateEntities` before `World.tick`, as
the singleplayer client does. Added commands are:

| Command | Purpose |
|---|---|
| `state` | Loaded tile order, invalid flags, complete piston/note state and entity double bits |
| `events` | Drain and compare sound and note-particle event bits |
| `click x y z` | Original block-click hook |
| `reload x y z` | Write/read the tile's NBT and replace it |
| `chunkcycle cx cz` | Unload/load the same Chunk object, retaining invalid identities |
| `chunkreload cx cz` | Save/unload/load a chunk through original ChunkLoader NBT |
| `setnote x y z value` | Create a note tile through NBT (byte conversion and clamping) |
| `lookup x y z` | Lazy tile lookup, recording the note or null |
| `scan` | Tile update pass (fixtures without entities) |
| `probe x y z action target value` | A one-shot Note subclass runs a mutation inside the tile scan |
| `item mx my mz id count damage` | Spawn an item; coordinates are integer thousandths |
| `mob mx my mz id` | Spawn a mob at integer-thousandth coordinates |
| `placedby x y z mx my mz yaw` | Invoke the placed-by hook with a positioned player |
| `itemplace x y z side id mx my mz yaw` | Place using the original ItemPiston path |
| `properties` | Compare mobility, hardness bits and container flags for all block IDs |
| `boxes x y z` | Compare collision and state selection bounds as raw double bits |

Probe actions remove a tile (0), set a new note tile (1), set a block (2), remove
then perform two lazy lookups (3), or remove the block (4). The probe invalidates
itself afterward. Rust keeps these test callbacks outside the product tile enum.

## Ownership and rendering

Tile IDs represent Java object identity independently of the chunk position map.
The existing chunk inventory APIs continue to use attached tile payloads;
overwritten, deferred and unloaded objects retain their payloads in the identity
registry until their last owner is removed. Invalid loaded entries survive until
the scan reaches them. Insertions during scanning are deferred, and post-scan
insertion checks that the current block is a container.

The worker sends a separate `BlockTiles` snapshot with two arm-length meshes,
an optional stationary base and queued note/piston events. Frame rendering uses
the original float translation added to the original double vertex positions
before rounding to float. It uses the existing terrain texture and entity GPU
pipeline. The client plays the sound events; note events also retain the note
value for particle consumers.

Vanilla quirks retained include previous-progress NBT, the unsaved render-head
and previous-note-power flags, quick-pulse sticky behavior, shifted sticky
retraction sound coordinates, replacement objects that keep ticking, and invalid
cleanup removing a newer chunk-map entry at the same coordinates.
