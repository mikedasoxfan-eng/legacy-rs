# Fix: server-mode 900-tick walk (pending ticks / TileTicks)

Section: `tools/verify10.sh` `== server mode: 900-tick walk (real World.tick), unload/reload through region files`
(also `tools/verify_serverfix10.sh`).

## Symptoms

Two different failures were mixed in that section:

1. **`serialized chunk NBT: DIFFERS`, every run** (also in r10-full-4 and r10-full-5, which people remembered as
   passing: their in-memory and on-disk lines passed, this line did not):
   `chunk (5, 1).NBT byte 67956, root.Level.TileTicks[450].x: Java 82, Rust 81`, 1,573 of 4,621 chunks differing.
2. **In memory / on disk DIFFERS, only sometimes** (r10-full-3, -6, -7): Rust-only or Java-only gravel ticks at
   (1880..1887, 16..20, 680..684) t=608, water at (2010/2011, 61, 798/799) t=656, lava at (1884, 19..20, 817..818)
   t=685, stone vs gravel/ore blocks. In -6 Java had the gravel ticks, in -7 Rust had them; the Java tick count
   itself changed between runs (1467526, 1467500, 1467523).

No commit between the passing and failing runs caused either; neither is a port bug in game logic.

## Cause 1: unrecorded NextTickListEntry identity hashes (deterministic NBT difference)

`World.func_41081_a` (chunk save) writes TileTicks in `scheduledTickSet` iteration order (a Java 8 `HashSet`).
`NextTickListEntry.hashCode` is `(x*1024*1024 + z*1024 + y)*256 + blockID`, so x and x+16 overflow to the
same hash: bins treeify and `HashMap.TreeNode.tieBreakOrder` orders equal hashes by `System.identityHashCode`.
The port models this exactly (`JavaHashSet`, world round 5's `tick_identity_replay10`), but the server walk
never gave it the identities: `ServerOracle` did not record them and the Rust walk used the insertion-order
stand-in. The terrain-save checks already recorded them (`Mode_terrainsave10`), the server walk did not.

## Cause 2: the original ThreadedChunkLoader race (intermittent in-memory difference)

`ThreadedChunkLoader.func_40324_c` (File IO thread) removes the head entry from the pending list and set inside
its lock, but serializes and writes it to the region file outside the lock. A main-thread
`ThreadedChunkLoader.loadChunk` of that chunk inside the window finds neither the pending entry nor region data,
returns null, and `ChunkProviderServer` regenerates the chunk (losing its neighbours' ore/gravel spill-over and
re-populating it). The port reproduces this race faithfully (a real File IO thread), so either side could hit
it. The walk turns round at t=600 and reloads the chunks it just unloaded, which are then at the head of the IO
queue; chunk (117, 42) is the one that showed up in -6 and -7.

Proof: making that one write slow in Rust (temporary 30 s sleep after the pop of (117, 42), gate off) gives
`1 chunk loads during that chunk's File IO write, 1 of them found no data` and exactly the r10-full-7
signature: Rust-only gravel ticks at (1880..1887, 16..20, 680..684) t=608 and Java-only water ticks at
(2010/2011, 61, 798/799) t=656 and lava ticks at (1884, 19..20, 817..818) t=685.

Separately, the Java oracle's history changes from run to run (five runs, five different end `rand` values,
different activeChunkSet identity streams). ChunkCoordIntPair's `x ^ z` hash treeifies HashSets, and how many
identity hashes the main thread draws depends on File IO timing. That was already harmless because the
chunk identities are recorded and replayed. With the tick identities also replayed, Rust matched all of
these runs.

## Fix

The port's game logic (and the game's default behaviour) is unchanged. The check now pins both VM/thread inputs,
which the Done standard puts outside required behaviour ("thread scheduling internals"). Their effect on bytes
written is still checked through replay:

- `tools/oracle10/server/.../ServerOracle.java`: records `System.identityHashCode` of every NextTickListEntry
  added to `scheduledTickSet` (`<out>.tickids`, add order, like `Mode_terrainsave10`). Uses a ThreadedChunkLoader
  subclass (`AtomicIOChunkLoader`, via `SaveOldDir.getChunkLoader`) that makes each `func_40324_c` write atomic
  with respect to `loadChunk` (the original classes, with the one interleaving the replay uses;
  `-Doracle.atomicio=0` turns it off), and reports loads that overlapped a write of the same chunk.
- `crates/worldgen10/src/threaded_io10.rs`: `ThreadedChunkLoader::load_root` (ThreadedChunkLoader.loadChunk's
  pending-then-region read, moved from storage.rs) and `write_next` (func_40324_c) track the in-flight entry.
  `ATOMIC_IO_FOR_REPLAY` (default **off**, so the game keeps the original race) gives replays the same gate;
  the counters `LOADS_DURING_WRITE` and `LOADS_MISSED_DURING_WRITE` record overlapping loads.
- `crates/worldgen10/src/bin/compare.rs` `serverwalk`: replays `<java.bin>.tickids` into
  `tick_identity_replay10`, enables the gate, asserts every recorded identity was used, prints the counters.
- `tools/verify10.sh` cleans up `v_*.bin.tickids`; `tools/verify_serverfix10.sh` deletes its two save folders
  first (it otherwise loaded a leftover save).

## Evidence

| Java run | gate | loads during write | Rust with replay: memory / NBT | Rust without tick replay: memory / NBT |
|---|---|---|---|---|
| j0 | off | 0 | EXACT / 4621 of 4621 identical | EXACT / TileTicks[450] differs |
| g1, g2 (byte-identical to j1) | on | 0 | EXACT / 4621 of 4621 identical (Rust run twice, identical saves) | EXACT / TileTicks[450] differs |
| u1, u2, u3 (all different) | off | 0 | EXACT / 4621 of 4621 identical | EXACT / TileTicks[450] differs |
| j1 | off | 1 (data found) | equal to g1 | |

Final run of the section's own commands (fresh oracle, gate on): `in memory: EXACT MATCH`,
`on disk: EXACT MATCH`, `serialized chunk NBT: EXACT MATCH` (4621/4621 identical data, identical sector layout).

Regression suites after the change, all `--built`, all exit 0 / EXACT: verify_r10blocks4 (BLOCKS4 ALL CASES EXACT),
verify_r10blocks3 (BLOCKS3 ALL CASES EXACT), verify_r10world4 (R10 WORLD ROUND 4 EXACT), verify_r10world3
(R10 WORLD ROUND 3 EXACT), verify_terrain3_10 (ROUND 3 TERRAIN CASES EXACT), verify_serverfix10 (memory, disk and
now also NBT: 4621/4621 identical), verify_world_blocks10 (WORLD-BLOCKS FOCUSED CASES EXACT).
Real-client play tests: `journey_nether_round_trip` (4 positions) and `mobs_breeding` pass, no crash.
`cargo build --release` is clean; `cargo test --release -p worldgen10` passes (31 test binaries ok).
