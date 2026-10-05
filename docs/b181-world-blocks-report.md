# Beta 1.8.1 world-blocks handoff

This is a verified **partial port**, not a complete Beta 1.8.1 implementation. The task's approximately 75-minute work window was used for registration, new block mechanics, synchronous lighting, chest state, save metadata and focused original-bytecode proofs. Classes below are marked done only where the port and an original-bytecode replay cover the changed behavior; passing a narrow fixture does not make its containing class complete.

## Verification

`cargo build --release -p worldgen181 -p client` succeeds (existing dead-code warnings).

Run `bash tools/verify_world_blocks181.sh` for the focused checks, or `--built` after building. The delimited world-blocks block is appended to `tools/verify181.sh` before its final result/exit, so it executes in the full suite. The full approximately one-hour suite was not run.

| New focused check | Result / byte count | Scope |
|---|---:|---|
| plants | EXACT / 24,399,872 | 112 configurations, 128 ticks each; stems, vines, mushroom spreading, leaves, farmland and RNG |
| geometry | EXACT / 1,256,366 | IDs 97..109, 16 metadata, 16 neighbor masks, three neighbor kinds; bounds/collisions; fence/gate connection predicate |
| drops | EXACT / 370,368 | New IDs and old/new stairs, 16 metadata, 32 seeds, chance, RNG and item positions |
| saves | EXACT / 1,464 | 24 level-info cases, clone and gzip persistence |
| lights | EXACT / 10,823,568 | 80 edits, height/occlusion state, sky/block flood light, packed light, provider brightness |
| chests | EXACT / 67,512 | 3,360 facing cases plus 800 animation ticks, adjacency, negative viewers, sounds and RNG |
| helpers | EXACT / 263,588 | floor overflow/NaN, inclusive RNG state, variable-height nibbles, precipitation cache invalidation |
| fluids | EXACT / 717,113 | 51,200 display ticks, ordered particles/sounds, RNG, packed light and face rules |
| teleporter | EXACT / 1,129,872 | Six fixture worlds with creation/search/fallback and both portal axes |
| items | EXACT / 4,280 | Seeds, stem bonemeal, gates, nullable NBT, vine/leaf shearing |
| actions | EXACT / 3,107 | All registered item actions/durations and real bytecode bow shot threshold/critical flag |
| registry | EXACT / 2,936,680 | Item properties, tool strengths, harvestability, block hardness and attack damage |
| definitions | EXACT | All 256 block entries, 29 materials and geometry defaults regenerated from bytecode |
| region cross-read | EXACT / 190,406 | 36 write/overwrite operations; size deltas; original Java reads Rust, Rust reads original Java |

Existing `all Beta block collision/selection states (10000 cases)` is EXACT after the geometry/lazy-tile changes. Existing MathHelper sine-table check remains EXACT (0 / 65,536 differences). No claim is made that the old full-world save, lighting-grid, weather/tick, player-use or travel sections are EXACT. The existing terrain-based teleporter oracle itself fails on its stale `field_4232_a` reflection name; the new independent teleporter suite uses the correct field and exercises the original algorithm.

## Integration and outstanding work

- Preserve this branch's verified mechanics while finishing live item use. Bow charging, sword blocking and 32-tick food eating require player input/state, release/completion, hunger/saturation and potion-effect integration. `registry::ItemUseAction`, `use_duration`, and `bow_charge` are available; the old `use_item` path is still active.
- Complete World spawning and weather: grass-based provider spawn test, biome search/new spawn rules, game-type/map-feature propagation, temperature-based snow/ice logic, and exact active-chunk iteration. The current hash collection assumes distinct hashes; Beta 1.8.1 ChunkCoordIntPair uses x XOR z and Java identity tie-breaking can affect tree-bin order. Random ticks are not proven exact.
- Implement huge-mushroom bonemeal/generation, silverfish harvest spawning, new block render colors/textures and creative variants. The integrator owns RenderBlocks/lightmap rendering; new IDs, pane/vine bounds, gate connection API and chest animation state are available but have not been wired into its renderer.
- Chest state is keyed by tile ID to preserve Java object references. The inventory enum still uses 27 slots; vanilla starts with an inaccessible 36-slot backing array and switches to 27 after NBT load. Complete this edge case and all invalidation/metadata cache probes before declaring TileEntityChest done.
- Finish save wrappers and full lifecycle verification: uncompressed/safe-replace CompressedStreamTools helpers, RegionFileCache, ChunkLoader relocation/repair, conversion and save-handler compatibility. Cross-read region tests exclude wall-clock timestamps and compressed-byte identity; they compare original observable contents and sector size deltas.
- Audit the remaining old block/item classes below. Rename-only diffs were not automatically promoted to done without a current oracle replay.

Minimal changes in adjacent files: entity collision dispatch/table for new blocks; farmland walking hook; tile-update dispatch for chest state; container open/close dispatch for viewer counts; fluid display ticks; item inventory nullable loading. No entity AI, player GUI, terrain generator, or client renderer was rewritten. No Beta 1.7.3 source or verifier was edited.

Git was not mutated by this agent. An external worker snapshot commit `dcf530e` appeared during the run and includes the first implementation stage. `target/commit-b181-world-blocks.sh` stages the explicit owned files, including remaining edits and this report, and uses the requested author and trailers. Run it from a writable Git environment.

## Per-class status

“Not done” includes partial ports and inherited implementations whose complete Beta 1.8.1 coverage has not been established.

| Class | Status | Proof / remaining scope |
|---|---|---|
| World | Not done | lights/helpers EXACT for synchronous radius-17 propagation, lightmap packing, 30M bounds, precipitation caching. Random-tick RNG draws added. Spawn rules, hash iteration, freezing, full ticking and entity changes incomplete. |
| Chunk | Not done | lights/helpers/chests EXACT for changed relight/occlusion/cache paths and lazy tile creation. Full chunk/save lifecycle coverage incomplete. |
| ChunkCache | Not done | World packed-light APIs exist; independent ChunkCache implementation and coverage missing. |
| CompressedStreamTools | Not done | saves validates gzip NBT semantics. New uncompressed file and safe-replace helpers still missing. |
| Particle | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| WorldInfo | Done | saves: game type, missing/default map features, clone, persistence, player dimension and existing level fields. |
| MCHash | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| Teleporter | Done | teleporter: six flat-world replays, search, construction, high arrival, ocean/platform fallback, portal axes, positions and RNG. |
| ChunkFile | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ChunkLoader | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| MathHelper | Not done | helpers covers new long floor, int abs and inclusive Random helper; sin covers all 65,536 entries. Complete public helper surface not audited. |
| RegionFile | Done | regions: 36 writes/overwrites, sector reuse and size deltas, Java reads Rust and Rust reads Java. |
| RegionFileCache | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| WorldBlockPositionType | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| MapStorage | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| SaveConverterMcRegion | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| NibbleArray | Done | helpers: both constructors, heights 4..8, every nibble and underlying bytes. |
| SaveFormatComparator | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| EnumAction | Done | actions: original action ordinals for every registered item. |
| RegionFileChunkBuffer | Done | regions: original compressed-stream close writes checked against Rust compression/write path. |
| IBlockAccess | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| SaveFormatOld | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| WorldProvider | Not done | lights checks Overworld and Nether brightness tables. Grass spawn predicate, map-feature provider construction and Sky provider incomplete. |
| IWorldAccess | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| SaveHandler | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ChunkFilePattern | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ChunkFolderPattern | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| EmptyChunk | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| IChunkProvider | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ISaveFormat | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ISaveHandler | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| NextTickListEntry | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| WorldProviderHell | Not done | brightness table remains EXACT in lights; broader provider coverage not run. |
| WorldProviderSky | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockVine | Not done | plants/geometry/drops/items EXACT for spreading, support, bounds and shears. Biome color/render methods incomplete. |
| BlockChest | Not done | chests EXACT for added/placed metadata; tile lifecycle and inventory activation integrated. Chest texture/render integration incomplete. |
| BlockStem | Not done | plants/geometry/drops/items EXACT for growth, fruit, fertilizer and seeds. Render color/direction methods incomplete. |
| TileEntityChest | Not done | chests EXACT for adjacency identities, lid floats, viewer counts, periodic events and sounds. Rendering handoff and vanilla initial 36-slot versus loaded 27-slot quirk remain. |
| ItemDoor | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemDye | Not done | items EXACT for crops/stem bonemeal. Huge mushrooms and edit permissions missing. |
| BlockFluid | Not done | fluids EXACT for random-display RNG/events, packed light and top-face solidity. Renderer hookup/full fluid suite pending. |
| BlockPane | Not done | geometry/drops EXACT for neighbor connections, collision/selection and drops. Renderer integration pending. |
| BlockDispenser | Not done | null tile guard added; original algorithm otherwise retained. Full dispenser replay not run. |
| Block | Not done | definitions/registry/geometry/drops cover defaults and new block registration. Base render/light/item-use hooks not fully ported. |
| BlockPistonBase | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockMushroom | Not done | plants EXACT for density cap and four-step spread. Huge-mushroom bonemeal generation missing. |
| ItemFood | Not done | registry/actions cover food registration/use metadata. Still uses old instant healing; 32-tick eating, hunger/saturation and potion effects missing. |
| BlockFenceGate | Not done | geometry/items EXACT for collision, placement/facing, opening and closing. Render type 21 integration pending. |
| ItemBow | Not done | actions checks duration/action and actual original release threshold/critical flag. Charge helper exists; live hold/release and arrow construction integration missing. |
| BlockFurnace | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| Item | Not done | registry/actions EXACT for definitions, use action and duration. Use lifecycle and edit permission support incomplete. |
| TileEntityPiston | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockSilverfish | Not done | definitions/geometry/drops EXACT; silverfish harvest spawning missing. |
| ItemBlock | Not done | placement supports all new IDs, replacing vines, metadata and surviving-ID guard. Edit permissions and complete placement replay missing. |
| ItemStack | Not done | items EXACT for nullable NBT loader on valid/unregistered positive IDs and signed counts/damage. New use methods and all loading callers not integrated. |
| TileEntity | Not done | lazy creation changed; chest state keyed by object identity. Metadata/cache and complete lifecycle suite pending. |
| BlockTallGrass | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockJukeBox | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockMushroomCap | Not done | definitions/geometry/drops EXACT; texture sides and huge growth pending. |
| BlockMelon | Not done | definitions/geometry/drops EXACT; visual integration pending. |
| BlockPistonMoving | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemVine | Not done | side placement and shearing paths ported; no full placement oracle suite. |
| BlockGrass | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockStoneBrick | Not done | definitions/geometry/drops EXACT; placement metadata preserved. Texture/creative variants pending. |
| ItemReed | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemHoe | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockRedstoneWire | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemFlintAndSteel | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockRedstoneRepeater | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemBucket | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemPainting | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemSword | Not done | registry/actions cover properties and block action/duration; live blocking integration pending. |
| BlockContainer | Not done | direct lazy tile factory path ported; full container class coverage pending. |
| BlockLeaves | Not done | plants EXACT for player-placed persistence/decay and items for shears. New biome/render colors pending. |
| Material | Done | definitions: all 29 material defaults; registry also checks harvest flags and tool behavior. |
| BlockNote | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemRedstone | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| StepSound | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockStairs | Not done | geometry/drops EXACT including new stairs and model metadata drop quirk. Rendering integration pending. |
| ItemBed | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| MaterialWeb | Done | definitions: separate web material with movement disabled while isSolid remains true. |
| ItemSign | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockPistonExtension | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| TileEntityDispenser | Not done | nullable item loading integrated; full tile inventory replay not run. |
| TileEntityFurnace | Not done | nullable item loading integrated; full furnace replay not run. |
| ItemShears | Not done | items/registry cover vine and leaf harvest; tall grass/vines tool cost changed. Full destroy-stat/cost replay pending. |
| ItemSlab | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockFence | Done | geometry: original fence/gate connection predicate for IDs 0..109; existing 10,000-case block geometry replay. |
| ItemLog | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockDoor | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockFire | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockFarmland | Done | plants: water, metadata, crops and both new stems; stem-aware walking hook ported. |
| BlockFlower | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockRedstoneOre | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemSeeds | Not done | items EXACT for all seed IDs, sides, soil and occupied target. Player edit permissions pending. |
| ItemSoup | Not done | action/duration registered; use completion and bowl replacement not ported. |
| PistonBlockTextures | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockCake | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockCloth | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockPressurePlate | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockStep | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockTrapDoor | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockWeb | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemCloth | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemMap | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| TileEntitySign | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockBed | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockCactus | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockMobSpawner | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockReed | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockSign | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockSnowBlock | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemLeaves | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemRecord | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
