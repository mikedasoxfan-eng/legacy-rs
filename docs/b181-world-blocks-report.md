# Beta 1.8.1 world-blocks handoff

This remains a verified **partial port**. Round two adds live item use, hunger/saturation, huge mushrooms, silverfish harvest spawning, chest backing-array/cache lifecycle, block visuals/creative variants, spawn search, save helpers and original-bytecode proofs. Classes below are Done only for audited changed behavior with current EXACT replays. Full world ticking, terrain integration and save lifecycle remain incomplete.

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

## Round-two integration and remaining work

Live input now starts and releases bow/sword/food use, suppresses mining/repeated clicks during use, and slows movement input while using. Food state saves with the player. Hunger exhaustion is connected to walking/swimming/diving, blockable damage, and food potion ticking. The minimal DamageSource adapter accepts armor-bypass and exhaustion fields; starvation uses it. The entities owner must map all existing environmental callers, finish jump/sprint/attack/mining exhaustion, player capabilities, full potion behavior and critical-arrow hit/flight replay. No full player simulation EXACT claim is made.

Huge mushrooms and silverfish player-harvest spawning are integrated. `block::visual181` exports original block render types, side/metadata textures, color APIs and creative item stacks. The integrator must call these APIs from RenderBlocks/lightmap/creative UI; coordinate-sensitive textures are still outstanding. Climate floats come from the terrain owner's WorldChunkManager.

`world::spawn181` ports the original biome search, including the replacement-count reservoir bug, and bounded seed-local spawn walk. The terrain owner must pass the 129x129 GenLayer biome rectangle to `find_initial_spawn181`; the old live entry uses the no-biome fallback until that integration is available. Overworld spawn now tests grass. `set_spawn_location` has the original 10000-attempt cap. Full World/Chunk/ChunkCache ticking is incomplete, including x-XOR-z hash collision iteration, weather biome integration and map-feature/game-type propagation. Source correction: this jar's snow path uses `getEnableSnow()`, not a temperature-0.15 predicate. Taiga in this original version does not enable snow. Existing legacy biome weather selection must be replaced with the original biome flags; freezing additionally requires a horizontal nonwater neighbor.

Chest backing arrays start with 36 slots and load as 27; containers expose only 27. Signed tile inventory counts, split/clamp behavior and cache/invalidation paths are ported. Save helpers cover uncompressed/safe-replace operations. Region relocation preserves the original bug: corrected coordinates are written to the outer root and the unchanged Level coordinates are loaded again. Region cache disposal, malformed-array semantics/repair, conversion, old save loader and save-handler compatibility remain incomplete.

No Beta 1.7.3 source or verifier was edited. Adjacent changes are limited to player/item state and inventory identity/count plumbing, charged-arrow construction/critical flags/gravity, a silverfish constructor/destroy hook, and signed-count fixture conversions in the Beta 1.8.1 renderer check. No entity AI, terrain generator, GUI or world renderer was rewritten.

Git remains unmodified. `target/commit-b181-world-blocks2.sh` stages the explicit changed files and commits with the requested author and both trailers. Run it from a writable Git environment.

## Round-two verification

Final focused validation: all 14 round-one suites and all 13 round-two suites are EXACT. Release build succeeds with existing dead-code warnings.

`tools/verify181.sh` already ends with the delimited world-blocks block calling `tools/verify_world_blocks181.sh --built`; the focused script now includes the new modes below. Every Java probe calls the original Beta 1.8.1 bytecode. The visual table is regenerated independently by the oracle on each replay and compared to output through the Rust texture/color/creative-list APIs.

| Added focused section | Result / bytes | Coverage |
|---|---|---|
| use | EXACT / 467,956 | Food completion, effects/events/RNG, hunger/saturation, bow charge/release/constructor |
| useticks | EXACT / 57,832 | 40-tick use/cancel/alias/mutation/release lifecycle |
| mushrooms | EXACT / 973,792 | 320 bonemeal configurations and 96 direct generator replays |
| chestlife | EXACT / 108,062 | 400 inventory operations, backing-array/NBT and metadata/cache lifecycle |
| math | EXACT / 1,060,373 | Complete public helper surface, random bit patterns and integer extrema |
| storage | EXACT / 63,957 | Uncompressed and safe-replace save helper lifecycle/failures |
| silverfish | EXACT / 65,830 | 64 constructor/event RNG replays plus conversion predicates |
| visual | EXACT / 246,524 | 256 IDs, 16 metadata states, six sides, both leaf graphics settings; creative list |
| colors | EXACT / 253,184 | Synthetic float climate/colormaps, extreme coordinates, all stem direction masks |
| spawn | EXACT / 10,986 | Original biome search/RNG, walk limit and all three provider predicates |
| blocking | EXACT / 30,176 | Sword/blocking, armor durability/remainder, bypass and food exhaustion |
| relocation | EXACT / 657,482 | Original region-load relocation, missing Level/Blocks and invalid IDs |
| foodstats | EXACT / 409,600 | 4,096 persistence cases and 16,384 arithmetic operations, arbitrary float bits/NaNs and signed integer overflow |

The final release build and all focused replays are recorded in `target/world-blocks2-build.log` and `target/world-blocks2-verify.log`. The full approximately one-hour `verify181.sh` run is left to the integrator. Inherited classes without complete current coverage stay Not done below.

## Per-class status

“Not done” includes partial ports and inherited implementations whose complete Beta 1.8.1 coverage has not been established.

| Class | Status | Proof / remaining scope |
|---|---|---|
| World | Not done | Round-one lights/helpers plus spawn EXACT for biome reservoir search, separate seed RNG, 1000-attempt cap and provider predicates. Live entry needs terrain GenLayer grid; weather, active-chunk hash collisions and complete ticking remain. |
| Chunk | Not done | lights/helpers/chests EXACT for changed relight/occlusion/cache paths and lazy tile creation. Full chunk/save lifecycle coverage incomplete. |
| ChunkCache | Not done | World packed-light APIs exist; independent ChunkCache implementation and coverage missing. |
| CompressedStreamTools | Done | saves + storage EXACT: gzip semantics, uncompressed reads/writes, safe replacement, stale temporary files, missing file, invalid root, failed target deletion and ignored rename result. |
| Particle | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| WorldInfo | Done | saves: game type, missing/default map features, clone, persistence, player dimension and existing level fields. |
| MCHash | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| Teleporter | Done | teleporter: six flat-world replays, search, construction, high arrival, ocean/platform fallback, portal axes, positions and RNG. |
| ChunkFile | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ChunkLoader | Not done | relocation EXACT for valid arrays, wrong Level coordinates (vanilla outer-root relocation bug), missing Level/Blocks and invalid block IDs. Malformed/missing array lifecycle and old chunk-directory loader still require work. |
| MathHelper | Done | math + helpers EXACT: all public helpers, 20,000 arbitrary float/double bit patterns, integer extrema, negative zero/NaNs, inclusive Random states; prior 65,536-entry sine table remains exact. |
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
| WorldProvider | Not done | spawn EXACT for grass predicate; brightness tables retained. Terrain provider construction and map-feature propagation remain. |
| IWorldAccess | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| SaveHandler | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ChunkFilePattern | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ChunkFolderPattern | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| EmptyChunk | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| IChunkProvider | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ISaveFormat | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ISaveHandler | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| NextTickListEntry | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| WorldProviderHell | Not done | spawn EXACT for nonair, nonbedrock, opaque spawn predicate; lights brightness EXACT. Full provider generation remains with terrain owner. |
| WorldProviderSky | Not done | spawn EXACT for material movement-blocking predicate. Live Sky dimension/provider integration remains. |
| BlockVine | Not done | plants/geometry/drops/items retained; visual/colors EXACT for render type, textures and biome foliage tint. Full inherited class audit/render hookup remains. |
| BlockChest | Not done | chests/chestlife EXACT for inventory, metadata, adjacency and cache paths; visual texture defaults EXACT. Coordinate-sensitive double-chest texture selection and complete activation replay remain. |
| BlockStem | Done | plants/geometry/drops/items + visual/colors EXACT: growth, fruit, fertilizer, seed drops, bounds, all metadata colors/textures and neighboring-fruit direction priority. |
| TileEntityChest | Done | chests + chestlife EXACT: object identity, 36 initial slots/27 loaded slots, signed counts, clamping/splitting including negative amounts, NBT inventory, cached metadata/block, reset/invalidate/validate, adjacency, lids, viewers, sounds/events. Renderer consumes exported state. |
| ItemDoor | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemDye | Not done | items + mushrooms EXACT for crops/stems and both huge mushrooms, including failed growth/metadata restoration. Edit-permission and full inherited dye surface remain. |
| BlockFluid | Not done | fluids EXACT for random-display RNG/events, packed light and top-face solidity. Renderer hookup/full fluid suite pending. |
| BlockPane | Not done | geometry/drops EXACT for neighbor connections, collision/selection and drops. Renderer integration pending. |
| BlockDispenser | Not done | null tile guard added; original algorithm otherwise retained. Full dispenser replay not run. |
| Block | Not done | definitions/registry/geometry/drops cover defaults and new block registration. Base render/light/item-use hooks not fully ported. |
| BlockPistonBase | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockMushroom | Done | plants + mushrooms + visual EXACT: density/spread, placement, bonemeal, huge generator checks/caps/stems, failed restoration, metadata and original RNG state. |
| FoodStats | Done | use/useticks/foodstats EXACT: NBT/defaults, signed overflow and float/NaN arithmetic, exhaustion/saturation depletion, 80-tick healing/starvation and difficulty thresholds. |
| ItemFood | Done | registry/actions/use/useticks EXACT: all 15 foods, hunger/saturation admission, 32-tick use, effects/RNG/sounds/particles, potion IDs/durations/chances and completion consumption. General player potion engine remains with entities owner. |
| BlockFenceGate | Not done | geometry/items EXACT for collision, placement/facing, opening and closing. Render type 21 integration pending. |
| ItemBow | Done | registry/actions/use/useticks EXACT: hold, release -20..80 elapsed ticks, threshold, speed, full-charge critical flag, arrow construction floats/RNG, sound and arrow consumption. Projectile simulation beyond construction needs entities coverage. |
| BlockFurnace | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| Item | Not done | registry/actions EXACT for definitions, use action and duration. Use lifecycle and edit permission support incomplete. |
| TileEntityPiston | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockSilverfish | Done | definitions/geometry/drops + silverfish + visual EXACT: metadata textures, conversion predicates, player-destroy spawn constructor/position/RNG and all 20 explosion particles. Silverfish AI/model/sounds belong to entities owner. |
| ItemBlock | Not done | placement supports all new IDs, replacing vines, metadata and surviving-ID guard. Edit permissions and complete placement replay missing. |
| ItemStack | Not done | nullable NBT proof retained; use identity tokens now cancel identical replacements while permitting same-object hotbar aliases (useticks EXACT). Copy-based inventory still lacks fully shared mutation of aliased Java stacks; full item delegation/load audit remains. |
| TileEntity | Not done | chestlife proves metadata/block caches, inventory refresh, resets and invalidate/validate. Full factory/persistence surface for every tile remains. |
| BlockTallGrass | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockJukeBox | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockMushroomCap | Done | definitions/geometry/drops + visual/mushrooms EXACT: every metadata/side texture, drops and huge cap generation. |
| BlockMelon | Done | definitions/geometry/drops + visual EXACT: render defaults, top/side textures, quantity RNG and metadata drops. |
| BlockPistonMoving | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemVine | Not done | side placement and shearing paths ported; no full placement oracle suite. |
| BlockGrass | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockStoneBrick | Done | definitions/geometry/drops + visual EXACT: all metadata/side textures, subtype drops and original creative variants. |
| ItemReed | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemHoe | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockRedstoneWire | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemFlintAndSteel | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| BlockRedstoneRepeater | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemBucket | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemPainting | Not done | Inherited implementation not fully audited/ported or covered by a current class-level oracle proof. |
| ItemSword | Done | registry/actions/use/useticks/blocking EXACT: properties, use action/duration, hold/cancellation, blocking and armor damage/remainder/exhaustion with bypass flag. Existing environmental attack callers still need DamageSource mapping. |
| BlockContainer | Not done | direct lazy tile factory path ported; full container class coverage pending. |
| BlockLeaves | Not done | plants/items retained; visual/colors EXACT for fancy/fast texture selection, pine/birch flags and float-climate foliage lookup. Full decay/render integration remains. |
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
| ItemSoup | Done | actions/use/useticks EXACT: 32-tick completion, hunger/saturation and bowl replacement. |
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
