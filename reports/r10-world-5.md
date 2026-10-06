# Release 1.0 world port, round 5: inherited world-level sections of verify10.sh

I re-ran every section in `codex/notes/r10world5-sections.txt` on the current branch (rounds 3–4 merged) through
the suites that contain it: `tools/verify_terrain10.sh`, `verify_terrain2_10.sh`, `verify_terrain3_10.sh`,
`verify_biomes10.sh`, and the `run`/`run_extra` cases of `tools/verify10.sh`, called with the same oracle and
compare arguments. Most sections already passed after rounds 3–4. The rest are fixed below. No new suite was
needed: the fixes are in existing suites, which `verify10.sh` already runs.

## Fixes

1. **biomes seed N (ORACLE FAILED).** This is a real 1.0 behaviour. `WorldChunkManager.func_35556_a` (find a
   biome position) loops over `var12.length`, which is the whole IntCache array including stale padding, not
   just `w*h` entries. It does this without first resetting IntCache. A stale value that is not a biome id,
   for example a river-init value, makes `BiomeGenBase.biomeList[...]` throw ArrayIndexOutOfBoundsException.
   The fixture (`Mode_biomes10`) ran into this exception in the original bytecode.
   - Fixture fix: it now records the throw as an outcome and keeps going.
   - Port fix: `BiomeSource::find_position10` scans the full padded array the same way. Ids 16..255 (null
     biomes) never match. An id outside 0..256 returns `Err` after the same random draws Java made.
     `find_position` keeps a panic for that case, as in game, where it would crash world generation.
2. **BiomeDecorator 9 fixture 0/1.** `BiomeGenBase.biomeList[9]` is BiomeGenEnd, and its decorator is a
   `BiomeEndDecorator`: ores, then a spike on 1 in 5 chunks, then the dragon at chunk 0,0. The port sent biome 9
   through the overworld decorator.
   - `populate::decorate` now calls `end::decorate` for the Sky biome.
   - `end::decorate` uses the caller's Random. The switch to `World.rand` moved to `end::populate`, because only
     `ChunkProviderEnd.populate` passes `World.rand`.
3. **Rust save / Java load (all seeds), and the related "independently populated region NBT" sections.**
   - **File IO flush:** the `compare10 terrain-makesave` and `serverwalk` fixtures exited without waiting for
     the File IO thread, which `OracleRunner10` does wait for. Only 44 of 289 chunks reached disk. Both now call
     `FILE_IO.flush()`. This is the same root cause as round 4's `makesave` fix.
   - **TileTicks order:** this order follows `scheduledTickSet`'s HashMap tree bins, and those break equal-hash
     ties with `System.identityHashCode`. `Mode_terrainsave10` now records the identity hashes in a
     `<out>.tickids` sidecar file. `terrain-makesave` replays them through `World.tick_identity_replay10`, the
     same mechanism as round 4. `verify_terrain3_10.sh` passes the sidecar file.

## Section table

| section | EXACT now / still differs | root cause + fix |
|---|---|---|
| GenLayer: 0 -37 -29 63 41 | EXACT now | already fixed by the merged rounds (Release layer stack) |
| GenLayer: 3257840388504953787 -256 -512 79 63 | EXACT now | same |
| GenLayer: -4172144997902289642 300 -100 43 29 | EXACT now | same |
| GenLayer: 9223372036854775807 -1 -1 17 13 | EXACT now | same |
| GenLayer: -9223372036854775808 12550784 -12550784 31 17 | EXACT now | same |
| biomes seed 0 | EXACT now | fix 1: 1.0 func_35556_a scans stale IntCache padding and can throw; fixture records the throw, Rust `find_position10` models it |
| biomes seed 3257840388504953787 | EXACT now | fix 1 |
| biomes seed -4172144997902289642 | EXACT now | fix 1 |
| biomes seed 9223372036854775807 | EXACT now | fix 1 |
| biomes seed -9223372036854775808 | EXACT now | fix 1 |
| utilities seed 0 | EXACT now | already fixed by merged rounds |
| utilities seed 3257840388504953787 | EXACT now | same |
| utilities seed -4172144997902289642 | EXACT now | same |
| utilities seed 9223372036854775807 | EXACT now | same |
| utilities seed -9223372036854775808 | EXACT now | same |
| Sky: 0 -4 -4 8 8 | EXACT now | already fixed (End generator) |
| Sky: 3257840388504953787 -3 -3 6 6 | EXACT now | same |
| Sky: -4172144997902289642 -2 -2 4 4 | EXACT now | same |
| Sky: 42 784420 0 4 2 | EXACT now | same |
| Sky: 42 -3 -784431 2 4 | EXACT now | same |
| Sky: 9223372036854775807 -1 -1 3 3 | EXACT now | same |
| BiomeDecorator 9 fixture 0 | EXACT now | fix 2: biome 9 uses BiomeEndDecorator (ores, spike, dragon) with the caller's Random |
| BiomeDecorator 9 fixture 1 | EXACT now | fix 2 |
| structures layout/map/blocks seed 0, 42, 3257840388504953787, -4172144997902289642, 9223372036854775807, -9223372036854775808 (18 sections) | EXACT now | already fixed by merged rounds (`verify_terrain2_10.sh`: ROUND 2 TERRAIN CASES EXACT) |
| terrain round2 sky/colors seed 0, 42, 3257840388504953787, -4172144997902289642, 9223372036854775807, -9223372036854775808 (12 sections) | EXACT now | same |
| terrain round2 locations seed -9223372036854775808 | EXACT now | same |
| stronghold layout/map/blocks/wet/grid, all 6 seeds (30 sections) | EXACT now | already fixed by merged rounds (`verify_terrain3_10.sh`) |
| Rust save / Java load seed -9223372036854775808 | EXACT now | fix 3: terrain-makesave waits for the File IO thread (all six seeds were affected on current code); TileTicks order replays recorded identity hashes |
| new world: seed 0, world rand 1 | EXACT now | merged rounds (round 4 new-world File IO flush) |
| new world: seed 3257840388504953787, world rand 7 | EXACT now | same |
| new world: seed -4172144997902289642, world rand 99 | EXACT now | same |
| new world: seed 123456789, world rand 2024 | EXACT now | same |
| new world: seed 404, world rand 5 | EXACT now | same |
| new world: seed -1, world rand -1 | EXACT now | same |
| new world, Fast graphics: seed 25 | EXACT now | same |
| new world, Fancy graphics: seed 25 | EXACT now | same |
| grid 24x24, light after each chunk | EXACT now | same |
| grid 16x16, light only at the end | EXACT now | same |
| grid 16x16, no light processing | EXACT now | same |
| ticks: 2 3 -77 3000 2000 thunder 0 0 | EXACT now | same |
| ticks: 1 7 999 100 2000 rain 40 -30 | EXACT now | same |
| ticks: -1 5 4242 0 2000 rain -100 60 | EXACT now | same |
| ticks: forest fire, seed 6 | EXACT now | same |
| ticks: saplings growing, seed 6 | EXACT now | same |
| teleporter: 3 1 0 100.5 70 -40.5 7 | EXACT now | same |
| teleporter: 3 1 0 1000.5 70 1000.5 1 | EXACT now | same |
| respawn: 3 1 plain | EXACT now | same |
| respawn: 3 1 bed | EXACT now | same |
| respawn: 404 5 nobed | EXACT now | re-checked after this round's changes |
| save format: Rust saves, Java loads (coordinator addition) | EXACT now | round 4 new-world `saveWorldIndirectly` flush; re-checked after this round's changes |
| independently populated region NBT, all 6 seeds (verify_terrain3_10, not in the list) | EXACT now | fix 3 (flush + tick identity replay) |

## Other checks

Final runs after all changes: `verify_terrain10.sh` TERRAIN CASES EXACT (157 sections),
`verify_terrain3_10.sh` ROUND 3 TERRAIN CASES EXACT (96), `verify_biomes10.sh` 5/5 EXACT, `verify_r10world3.sh`
R10 WORLD ROUND 3 EXACT (258), `verify_r10world4.sh` R10 WORLD ROUND 4 EXACT (102), both verify10 save-format
sections EXACT, and `respawn: 404 5 nobed` EXACT. `cargo test -p worldgen10` passes and the oracle `raw` command
compiles. I edited these fixtures:
`tools/oracle10/src/net/minecraft/src/Mode_biomes10.java` (records the 1.0 exception) and `Mode_terrainsave10.java`
(writes the identity sidecar file). Both still drive the original 1.0 methods.
