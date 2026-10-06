# Release 1.0 world port, round 4

**51 Done; 0 Not done** of the 51 classes in this round's list. Each Done row cites `== r10world4` sections in
`tools/verify_r10world4.sh` (registered in `tools/verify10.sh` before the final `pflush`). The suite runs
original Release bytecode through `tools/oracle10` and compares bytes with the Rust port. The final run
was **R10 WORLD ROUND 4 EXACT** (102 sections). `tools/verify_r10world3.sh` was re-run afterwards and was
**R10 WORLD ROUND 3 EXACT** (258 sections). The rows apply the Done standard: gaps that no 1.0 game path can
observe are listed as "unreachable in game".

Sections that take a seed run for seeds 0, 42 and -1 (`r10world4 <mode>/0`, `/42`, `/-1`). Below, `mode/*`
stands for all three seeds. The inherited round-3 fixtures are re-run in this suite under `r10world4` names.

## Production fixes this round

- **Integrator item: "save format: Rust saves, Java loads" in `tools/verify10.sh` (was "chunk count differs: java 35 rust 289").**
  `Minecraft.changeWorld` saves a new world with `World.saveWorldIndirectly`, which is `saveWorld(true)`
  followed by `ThreadedFileIOBase.func_40566_a()` (wait for the File IO thread). `scenario.rs` only called
  `save_world(true)`, so a process that exited straight after opening a world (`compare10 makesave`) lost
  the chunks still waiting in the threaded queue. It now calls `save_world_indirectly()`. Both save-format
  sections are EXACT again (checked twice, plus again after the other changes).
- **Exact Java deflate, with no binary dependency.** The previous worker had made gzip/region bytes exact by
  loading an embedded zlib1.dll (left untracked in `assets/r10-zlib`, which this commit command would not
  have included, so other checkouts would not build). It is replaced by `crates/worldgen10/src/compression10.rs`,
  a line-by-line pure-Rust port of zlib's level-6 path (deflate.c `fill_window`/`longest_match`/`deflate_slow`,
  trees.c). Before the DLL was removed, the port was byte-compared with it on 600 inputs (40 MB). It also
  matches Java on the `gzipbytes` and `regionbytes` checks. `cargo test -p worldgen10` passes, the oracle `raw` command compiles, and the 1.0 client smoke screenshot renders normally. `libloading` and the DLL are gone, and the
  Beta crates' compression is unchanged.
- **Nether chunk `isModified`.** `ChunkProviderHell.provideChunk` returns `new Chunk(...)` without
  `generateSkylightMap`, so a freshly generated Nether chunk is not modified. The port marked it modified,
  so `saveWorld` wrote unpopulated edge chunks that Java does not write, and the region sectors drifted
  (`hellregionbytes`).
- **Tick-set iteration order.** `World.scheduledTickSet` is a `HashSet<NextTickListEntry>`. Its classfile has
  no generic `Comparable` signature, so equal-hash entries in tree bins are ordered by
  `System.identityHashCode`. These are equal hashes for x positions 16 apart with the same y/z/id, which is
  common in saved `TileTicks`. The port's `JavaHashSet` already modelled tie-breaking, and
  `World.tick_identity_replay10` now lets a fixture replay the recorded VM identity hashes. Without a replay,
  insertion order stands in, because the real values are VM internals.
- `SaveFormatOld.deleteFiles` prints `Deleting <dir>` for nested directories, as 1.0 added.

## Checks added (tools/verify_r10world4.sh)

New: `geometry`, `intcache`, `misc`, `savefolders`, `notify`, `endteleport`, and per seed `nbt`, `gzipbytes`
(16 random trees plus 9 large 1–120 KB payloads crossing the window/block limits, byte-equal GZIP output),
`colors`, `nearest`, `waterlily`, `population`, `hellpopulation`, `ticks`, `hellticks` (TreeSet and HashSet
order plus per-chunk lists), `fortressspawn` (finds the nearest fortress, generates and populates 7x7
Nether chunks around it, queries 3,000+ points through `ChunkProvider.func_40377_a`, and dumps the populated
blocks), `providerapi` (now also with the `ChunkProviderLoadOrGenerate` wrapper), `worldinfo`, `regionbytes`
and `hellregionbytes` (all region files byte-for-byte after the first save and after three rounds of
grow/shrink rewrites, with only the wall-clock timestamp table cleared), and `saveformat`. Inherited:
`threaded`, `worldheight`, `endprovider`, `endterrain/*`, `endpopulation/*`, `biomes/*`, `biomecache/*`,
`fortress/map/*`, `loadorgenerate/{cache,null,failure}/*`, `provider/{longset,scan,saves}/*`,
`world/{arrays,cache,weather,nextticks,frost,savelife,saves,storage,relocation,regioncache4,oldformat4,emptycache4,legacy5,lights}`,
`teleporter`. The WIP's `known-gap` group (the `sharedsave` diagnostic) was removed (see ThreadedChunkLoader).
The comparator accepts `WORLD4_RUST_OUT=<file>` to dump the Rust bytes.

## Class table

| Class | Status | Notes |
|---|---|---|
| World | Done | Release fields/queries, population, ticks and saving are exact. This round: new-world save waits for File IO (saveWorldIndirectly), tick-set identity replay, Nether isModified. Unreachable in game: mutated height/shift fields (always 7/128 in 1.0; Packet height fields are never applied), alternate-constructor object sharing (the old World is discarded), exception paths. Also fixes verify10 `save format: Rust saves, Java loads`. Checks: `r10world4 worldheight`, `r10world4 population/0`, `r10world4 population/42`, `r10world4 population/-1`, `r10world4 hellpopulation/0`, `r10world4 ticks/0`, `r10world4 ticks/42`, `r10world4 ticks/-1`, `r10world4 hellticks/0`, `r10world4 hellticks/42`, `r10world4 hellticks/-1`, `r10world4 providerapi/0`, `r10world4 regionbytes/0`, `r10world4 regionbytes/42`, `r10world4 regionbytes/-1`, `r10world4 world/weather`, `r10world4 world/nextticks`, `r10world4 world/frost`, `r10world4 world/lights`, `r10world4 world/saves` |
| ChunkProviderEnd | Done | Raw terrain, density, height/sky arrays, population, entities and provider API are exact. Unreachable in game: mutated World height fields. Checks: `r10world4 endterrain/0`, `r10world4 endterrain/42`, `r10world4 endterrain/-1`, `r10world4 endpopulation/0`, `r10world4 endpopulation/42`, `r10world4 endpopulation/-1`, `r10world4 endprovider`, `r10world4 endteleport`, `r10world4 providerapi/42` |
| Chunk | Done | Constructors/loads, light, height, populated flag and isModified (Nether fix) are exact. Unreachable in game: mutated height fields and malformed setter inputs; backing-array aliases are only seen through IO-thread timing (see ThreadedChunkLoader). Checks: `r10world4 world/arrays`, `r10world4 world/cache`, `r10world4 world/lights`, `r10world4 population/0`, `r10world4 hellpopulation/42`, `r10world4 regionbytes/42`, `r10world4 hellregionbytes/0`, `r10world4 hellregionbytes/42`, `r10world4 hellregionbytes/-1` |
| ChunkProviderGenerate | Done | Full executed population of 25 chunks (blocks, data, both light arrays, height maps, populated flags, entities, all RNG states), spawn lists, nearest-stronghold dispatch and saved output. Checks: `r10world4 population/0`, `r10world4 population/42`, `r10world4 population/-1`, `r10world4 providerapi/0`, `r10world4 providerapi/42`, `r10world4 providerapi/-1`, `r10world4 regionbytes/0`, `r10world4 regionbytes/42`, `r10world4 regionbytes/-1` |
| ThreadedChunkLoader | Done | FIFO pending replacement, pending reads, region writes in order, folders, and exact zlib payload bytes. Unreachable in game: Java's pending NBT shares the live chunk arrays, so a write can reflect later edits, but only depending on File IO thread timing. The port snapshots at save time, which is the schedule where the thread writes immediately; a re-save of a modified chunk always replaces it. Checks: `r10world4 threaded`, `r10world4 regionbytes/0`, `r10world4 regionbytes/42`, `r10world4 regionbytes/-1`, `r10world4 hellregionbytes/42`, `r10world4 savefolders` |
| MapGenStructure | Done | Nearest-query candidate/generated fallback, overflow coordinates, containment (func_40483_a) through generated fortress starts, placements. Checks: `r10world4 nearest/0`, `r10world4 nearest/42`, `r10world4 nearest/-1`, `r10world4 fortressspawn/0`, `r10world4 fortressspawn/42`, `r10world4 fortressspawn/-1`, `r10world4 fortress/map/42` |
| ChunkCache | Done | Release height fields and query/light behaviour. Unreachable in game: provider eviction during a path search (the cache is built and used within one call). Checks: `r10world4 worldheight`, `r10world4 world/cache`, `r10world4 world/emptycache4`, `r10world4 world/lights` |
| ChunkProviderHell | Done | Raw terrain, caves, fortress population, spawn-list dispatch (fortress list vs biome list), isModified and saved bytes. Checks: `r10world4 hellpopulation/0`, `r10world4 hellpopulation/42`, `r10world4 hellpopulation/-1`, `r10world4 fortressspawn/0`, `r10world4 fortressspawn/42`, `r10world4 fortressspawn/-1`, `r10world4 hellregionbytes/0`, `r10world4 hellregionbytes/42`, `r10world4 hellregionbytes/-1`, `r10world4 providerapi/0` |
| ThreadedFileIOBase | Done | Identity de-duplication, queued/completed counters, flush barrier, and the new-world flush (verify10 save format). Unreachable in game: thread priority, interrupts, scheduler races. Checks: `r10world4 threaded`, `r10world4 savefolders`, `r10world4 regionbytes/0`, `r10world4 hellregionbytes/-1` |
| BiomeGenBase | Done | Release grass/foliage colour getters (cached float climate widened to double), singleton attributes and lists. Checks: `r10world4 colors/0`, `r10world4 colors/42`, `r10world4 colors/-1`, `r10world4 biomes/0`, `r10world4 biomes/42`, `r10world4 biomes/-1` |
| NBTBase | Done | Factory, named write/read, copy, all types in random trees. Unreachable in game: null payloads, unpaired UTF-16 (typed text is filtered by ChatAllowedCharacters), one tag object inserted under two names, unknown-type exceptions. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1`, `r10world4 gzipbytes/0` |
| NBTTagCompound | Done | Key order, copy, nested write/read, level.dat round trips. Unreachable in game: same as NBTBase. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1`, `r10world4 worldinfo/42`, `r10world4 gzipbytes/42` |
| Teleporter | Done | End platform placement (floor/negative coordinates, yaw/pitch/motion, RNG) and normal portals. Checks: `r10world4 endteleport`, `r10world4 teleporter` |
| NBTTagList | Done | Element type byte (including empty lists), nested lists, copy. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1` |
| MapGenNetherBridge | Done | Candidate rule, start graphs, clipped population through the retained map, spawn-list containment. Checks: `r10world4 fortressspawn/0`, `r10world4 fortressspawn/42`, `r10world4 fortressspawn/-1`, `r10world4 fortress/map/0`, `r10world4 hellpopulation/42` |
| CompressedStreamTools | Done | GZIP write bytes equal Java's (pure-Rust zlib port), read/write round trips, root handling. Unreachable in game: stream close/IO failures. Checks: `r10world4 gzipbytes/0`, `r10world4 gzipbytes/42`, `r10world4 gzipbytes/-1`, `r10world4 nbt/42`, `r10world4 worldinfo/0` |
| BiomeGenSwamp | Done | Swamp grass/foliage tint ((c & 0xfefefe) + 0x4e0e4e) / 2 through the original getters, water colour, trees. Checks: `r10world4 colors/0`, `r10world4 colors/42`, `r10world4 colors/-1`, `r10world4 biomes/42` |
| NBTTagByteArray | Done | Lengths 0..64 plus large arrays, copy, round trip. Unreachable in game: Java identity equality (1.0 item tags hold no byte arrays). Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 gzipbytes/-1` |
| MapGenStronghold | Done | Nearest portal-room query before/after generation and with overflow coordinates. Checks: `r10world4 nearest/0`, `r10world4 nearest/42`, `r10world4 nearest/-1`, `r10world4 providerapi/42` |
| MapGenCavesHell | Done | Executed through complete Nether chunk generation (lava exclusion, branching, RNG). Checks: `r10world4 hellpopulation/0`, `r10world4 hellpopulation/42`, `r10world4 hellpopulation/-1`, `r10world4 fortressspawn/42` |
| NBTTagString | Done | Modified UTF-8 including NUL, Latin-1 and supplementary characters. Unreachable in game: null and unpaired surrogates. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1` |
| NBTTagShort | Done | Random signed values, names, copy. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1` |
| NBTTagLong | Done | Random 64-bit values. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1` |
| NBTTagInt | Done | Random 32-bit values. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1` |
| NBTTagFloat | Done | Random bit patterns including NaNs/subnormals. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1` |
| NBTTagDouble | Done | Random bit patterns. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1` |
| NBTTagByte | Done | Random signed bytes. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1` |
| SaveHandler | Done | Folder selection (DIM-1/DIM1), data/players dirs, Unicode save name (func_40530_d), level save/load. Checks: `r10world4 savefolders`, `r10world4 worldinfo/0`, `r10world4 world/saves`, `r10world4 world/savelife`, `r10world4 world/storage` |
| WorldGenerator | Done | func_41060_a with notify false/true, repeated and stale metadata, RNG. Checks: `r10world4 notify`, `r10world4 population/42` |
| MapGenWaterlily | Done | Still/moving water, metadata, rejections, top/bottom heights. Checks: `r10world4 waterlily/0`, `r10world4 waterlily/42`, `r10world4 waterlily/-1` |
| WorldInfo | Done | hardcore/MapFeatures missing/true/false, copy constructor, dimension, saved key order. Checks: `r10world4 worldinfo/0`, `r10world4 worldinfo/42`, `r10world4 worldinfo/-1`, `r10world4 saveformat/42` |
| WorldChunkManagerHell | Done | Fixed Hell/End biome, climate and buffers. Unreachable in game: returned-array identity. Checks: `r10world4 biomes/0`, `r10world4 biomes/42`, `r10world4 hellpopulation/-1`, `r10world4 fortressspawn/0` |
| WorldChunkManager | Done | Release layers, caches, temperatures. Unreachable in game: cache array identity and wall-clock cache expiry. Checks: `r10world4 biomes/0`, `r10world4 biomes/42`, `r10world4 biomes/-1`, `r10world4 biomecache/42`, `r10world4 colors/42` |
| NBTTagEnd | Done | Terminator write and read. Checks: `r10world4 nbt/0`, `r10world4 nbt/42`, `r10world4 nbt/-1` |
| ChunkProvider | Done | func_40377_a/func_40376_a delegation, cache/save behaviour. Checks: `r10world4 providerapi/0`, `r10world4 providerapi/42`, `r10world4 providerapi/-1`, `r10world4 provider/longset/42`, `r10world4 provider/scan/42`, `r10world4 provider/saves/42` |
| WorldProvider | Done | Providers -1/0/1 and unknown (null), flags, cloud height, moon phase with extreme times. Checks: `r10world4 providerapi/0`, `r10world4 providerapi/42`, `r10world4 providerapi/-1`, `r10world4 endprovider` |
| ThreadedChunkLoaderPending | Done | Coordinate/payload replacement order. Unreachable in game: shared root identity (see ThreadedChunkLoader). Checks: `r10world4 threaded`, `r10world4 regionbytes/42` |
| TextureOffset | Done | Signed coordinates. Checks: `r10world4 geometry` |
| SaveOldDir | Done | Threaded loaders for all dimensions, DIM-1/DIM1 folders. Checks: `r10world4 savefolders`, `r10world4 regionbytes/0`, `r10world4 hellregionbytes/0` |
| RegionFile | Done | Sector allocation, reuse and growth across rewrites, header and length/type bytes, zlib payloads. Unreachable in game: wall-clock timestamps (cleared in the check) and IO errors. Checks: `r10world4 regionbytes/0`, `r10world4 regionbytes/42`, `r10world4 regionbytes/-1`, `r10world4 hellregionbytes/0`, `r10world4 hellregionbytes/42`, `r10world4 hellregionbytes/-1`, `r10world4 world/regioncache4`, `r10world4 world/relocation` |
| ChunkProviderLoadOrGenerate | Done | New delegations (the class is never constructed by 1.0 game code). Checks: `r10world4 providerapi/0`, `r10world4 providerapi/42`, `r10world4 providerapi/-1`, `r10world4 loadorgenerate/cache/42`, `r10world4 loadorgenerate/null/42` |
| SaveFormatOld | Done | Save list with hardcore flag and sort order, rename, delete (plus the new `Deleting` log), loaders. Checks: `r10world4 saveformat/0`, `r10world4 saveformat/42`, `r10world4 saveformat/-1`, `r10world4 world/oldformat4` |
| IntCache | Done | Pool reset/resize/stale tails from the original static pool. Checks: `r10world4 intcache` |
| IEnchantmentModifier | Done | Null stacks, armour-array order, duplicate enchantments, singleton identity. Checks: `r10world4 misc` |
| TexturedQuad | Done | UV float bits for three texture sizes, flipFace, invertNormal draw normals. Checks: `r10world4 geometry` |
| ISaveHandler | Done | func_40530_d and dimension loaders. Checks: `r10world4 savefolders`, `r10world4 world/storage` |
| IChunkProvider | Done | Spawn list and nearest-structure contract (null for other dimensions/names). Checks: `r10world4 providerapi/0`, `r10world4 providerapi/42`, `r10world4 providerapi/-1`, `r10world4 fortressspawn/42` |
| Empty3 | Done | No fields, methods or constructors. Checks: `r10world4 misc` |
| EnumAction | Done | Ordinals, names and item use actions. Checks: `r10world4 misc` |
| BiomeCacheBlock | Done | Values and signed indexing. Unreachable in game: array identity. Checks: `r10world4 biomecache/0`, `r10world4 biomecache/42`, `r10world4 biomecache/-1` |
| BiomeCache | Done | Values, key order, cleanup. Unreachable in game: returned-array identity. Checks: `r10world4 biomecache/0`, `r10world4 biomecache/42`, `r10world4 biomecache/-1` |

## Notes for the integrator

- `assets/r10-zlib` (the untracked DLL from the WIP) was deleted. Nothing references it now.
- The Nether isModified fix changes which Nether chunks `saveWorld` writes. All round-3/4 world checks and
  both save-format sections pass. Nether server/portal suites outside this area were not re-run.
- The `ticks`/`regionbytes` oracle outputs start with recorded identity hashes. The Rust side reads them
  back from the expected file (`world4_verify::dump_expected`).
