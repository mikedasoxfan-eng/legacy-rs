# Release 1.0 blocks area, round 1

Source: 1.0 MCP Java at `C:/Users/logiu/Desktop/Projects/minecraft/beta-rs/tools/retromcp-r10/minecraft/src/net/minecraft/src`, compared with linked Beta 1.8.1 source. No commits or oracle10/verify10 implementations. Protected version crates and oracle/verify tools are unchanged.

Ported means the delta is represented, including retained equivalent behavior for signature/rename-only changes. It does **not** mean bytecode-verified. Partial names a remaining integration/fidelity gap. Modules combine Java classes; inherited `181` filenames here are inside worldgen10 only.

Validation: whole-workspace `cargo build --release` passed (existing warnings); `cargo test -p worldgen10` passed: 36 tests, 15 library and 21 integration, plus all binary/doc targets. The requested seed 42 / 60 tick smoke command exited 0. Visually inspected `target/r10-smoke.png`: terrain, tree, sky, hand, HUD and inventory achievement appear normally. The inherited client version label still says Beta 1.8.1; client branding is outside this area. This generic smoke does not validate new block renderers or screens.

Primary modules: `potion_helper.rs`, `enchantment.rs`, `stack_tags.rs`, `item10.rs`, `container/brewing.rs`, `container/enchanting.rs`, `world/blocks10.rs` plus live dispatch/properties/persistence. `tests/blocks10.rs` checks live windows, saved tags, recipes/repair and all four portal rings with missing-eye rejection.

## Remaining integration and fidelity

- Entity area: live potion/pearl/eye/snowman constructors and heading/target APIs; End travel; combat/respiration/efficiency/aqua affinity/unbreaking consumers and break animation. Supplied item/block callbacks exist, but their live connections remain incomplete. Pumpkin ordinary onAdded and portal ordinary collision dispatch remain inherited until dependencies exist.
- World area: End provider/eternal bedrock fire, biome grass/foliage/water RGB, adjustable height, notifying tree/mushroom generators and Stronghold lookup. Color callback APIs exist; inherited live climate callers still need conversion.
- Client area: enchanting buttons/name-seed/book, brewing progress, new block render types, enchantment/drip/spawner particles, tooltip/rarity/glint and neighbor brightness. Only generic title/background matches and item properties are bridged here.
- Exactness: static RNG ownership, malformed/null NBT exceptions and mutable compound aliases need oracle work. Tags use the existing ItemStack identity with a process-wide sidecar. Reads clone compounds; copy/split deep-copy onto new identities. Arena entries persist for process lifetime. Mutable NBT reference identity is not proven by these tests.

## All 136 blocks-area classes

| ClassName | Status | What changed / remains | Next original-bytecode checks |
|---|---|---|---|
| PotionHelper | Ported | UTF-16 expression/recipe parsers, ingredient flags, effects, splash duration, prefixes and cached RGB. | All metadata 0..32767/both flags; malformed expressions, Java masked shifts, effect order/duration/amplifier and float color bits. |
| EnchantmentHelper | Partial | Level/max scans, ordered modifiers, costs, weighted compatible lists and Java HashMap buckets; silk/fortune harvest wired. Remains: live combat/respiration/efficiency/aqua affinity/unbreaking consumers and shared modifier RNG ownership. | Seeds and post-call RNG; duplicate/invalid NBT IDs, zero/negative modifiers, all item/material candidates, compatibility removal and oracle JVM HashMap iteration. |
| TileEntityBrewingStand | Ported | Four slots, whole-slot decrement, 600-tick brewing/abort, bottle metadata, ingredient/container consumption, NBT and live container/storage integration. | Every ingredient/potion, null cache identity, swapping ingredients, negative counts, reload midway, signed slots, completion dirty/metadata order and MP ticks. |
| ItemPotion | Partial | Effect cache, drinking/bottles, icons/color/glint, instant test, names/tooltips and splash count/sound/spawn callback. Remains: live EntityPotion and client tooltip/glint consumers. | Cache null/list identity, all metadata, creative/MP counts, full-inventory bottle handling, splash RNG/event order and translation/trim. |
| ItemEnderEye | Partial | Live frame insertion, smoke and complete portal ring scan/fill; supplied Stronghold lookup/spawn callbacks implement right-click. Remains: live world lookup and EntityEnderEye connection. | Four directions, gaps/missing eyes, wrong frame facing, shifted rows, MP/creative, negative truncating coordinates and spawn/sound/aux order. |
| Item | Partial | Recipe string access, display translation callbacks, tooltip hook, rarity/glint/enchantability and ray helper; new IDs/records in live registry overlay. Remains: generic mutable registration identity and client consumers. | Every static registration/default, ingredient string, null names, double translation/trim, ray flags and virtual callback exceptions. |
| ItemStack | Partial | Identity tag sidecar, deep copy/split, equality, inventory/tile/storage preservation, setter, enchantment NBT, tooltip/rarity/glint and unbreaking/break-animation callbacks. Remains: live damage context, mutable compound aliases and malformed-tag exceptions. | Copies/splits/aliases, nested/reordered/NaN compounds, malformed NBT, duplicate IDs, byte level narrowing and skip/break callback/stat/RNG order. |
| BlockDragonEgg | Ported | Scheduling/falling, activation/click teleport with metadata and 128-particle sequence. | Instant/entity falling, chunk gate, MP, 1000 occupied attempts, float/double coordinates and exact RNG continuation. |
| BlockBrewingStand | Ported | Properties, rod/base collision, factory, activation, smoke, bottle masks and source untagged removal scatter. | Bounds mutation, scatter quantities/Gaussians/static RNG, masks, factory lifecycle and server/client activation. |
| Block | Partial | 110..122 registrations, dragon material, obsidian hardness 50, snow opacity 0, glowstone glass, neighbor-brightness query, fortune and silk dispatch. Remains: world/client brightness consumer and mutable class registration APIs. | Complete 256 property entries, registration order, all fortune drop RNG, chance comparison, silk subtype/null creators and brightness flags. |
| BlockCauldron | Ported | Properties, five collision boxes, level textures, bucket fill and bottle extraction/count quirks. | Metadata 0..15, MP, creative bucket/bottle, inventory-full drop coordinates and write/count order. |
| TileEntityEnchantmentTable | Partial | Rotation/open/page animation, factory/mapping and live ticks with nearest-player queries. Remains: process-wide static animation RNG ownership and renderer. | Radius/ties, pi wrapping and float bits, opening/closing, target-page retry draws, transient NBT reload state. |
| Enchantment | Ported | 17 registered IDs, weights/types/names/ranges/compatibility, translation and weighted wrappers. | All registered/null positions, duplicate/invalid registration, base defaults, levels, translation and array bounds. |
| BlockFence | Ported | Dynamic wood/nether fence same-ID/gate/opaque-normal non-pumpkin connectivity and bounds. | Neighbor masks, mixed fence IDs/gates/pumpkins/glass/containers, height 1/1.5 and large-coordinate float rounding. |
| BlockEndPortal | Partial | Properties/tile, state bounds, no collision pieces, smoke, dimension placement guard and dimension-1 collision callback. Remains: connect entity End travel and client renderer. | ALLOW flag/dimensions, factory/onAdded order, bottom side rule, MP/player/mount/passenger guards. |
| BlockNetherStalk | Ported | Soul sand placement/stay, Hell growth, stage textures and mature fortune drops ignoring chance. | Light/sky/soil, biome/stage, nextInt(15) order, custom quantity/fortune/chance and update-before-flower check. |
| BlockEnchantmentTable | Ported | Properties, 3/4 height, tile/container activation and bookshelf display loops. | Factory identity, perimeter z-skip, midpoint air, both shelf heights and particle RNG/velocity bits. |
| BlockMycelium | Ported | Properties, snow texture, light conversion/four spread trials and townaura. | Lighting/opacity boundaries, target reads/offset RNG, snowy sides and display float casts. |
| EnchantmentProtection | Ported | Five variants, feet type, source integer modifiers/ranges and protection compatibility. | Source flags/identity, creative harm, negative/overflow levels, interval endpoints and both compatibility directions. |
| BlockPistonBase | Ported | Shared ignore-updates static, container push rejection, MP guard, retract tile-before-clear and failed-extension metadata restore. | Sticky/normal across worlds, reentrancy/guard transitions, mobility/containers, failure paths and write/event order. |
| BlockEndPortalFrame | Ported | 13/16 base, optional eye collision, facing placement, properties and textures. | All metadata/yaws, eye reset, box order/bounds and undroppable quantity. |
| ItemGlassBottle | Ported | Water ray/edit checks and potion replacement/inventory/drop behavior. | Hit/miss and flowing/source water, permission, MP/creative, zero/many counts and full inventory RNG. |
| ItemLilyPad | Ported | Source-water/air placement, edit checks, creative decrement and color/metadata. | Water IDs/metadata, air, failed write, permissions, MP/creative and ray flags. |
| BlockLilyPad | Ported | 1/64 bounds, still-water placement, water-material source stay and tint/render properties. | y boundaries, moving/still water with metadata, flower placement versus stay and drops/color. |
| ItemColored | Partial | Tall grass subtype names and renderColor, metadata placement; vine/lily helpers. Remains: generic configurable name-array setter/captured Block object API. | Null/custom name arrays, signed damage bounds, subtype flag, side-0 texture and full renderColor callback damage. |
| BlockMushroom | Partial | 1/25 spread, mycelium stay regardless of light, removed pre-generation soil rejection. Remains: world-owned giant mushroom/notifying generator. | Light/mycelium/opaque soil, spread RNG, failed generator restoration and invalid-soil invocation. |
| EnchantmentDamage | Ported | Sharpness/smite/bane ranges, weights, creature attribute damage and exclusion. | All attributes, levels/overflow, power endpoints, mutual and non-damage compatibility. |
| TileEntityChest | Ported | Open/close sounds and half-open closing threshold; tagged stack paths preserved. | Double adjacency, users, lid float thresholds, sound positions/pitch RNG and tagged decrement/NBT. |
| ItemBucket | Ported | Shared ray no-box flags, creative water/lava fill returns original bucket after fluid removal; milk separate. | Empty/full ray, fluids/Nether, permissions, creative stack identity and replacement order. |
| EnumArmorMaterial | Ported | Exact five material durability/reduction/enchantability tables. | Every slot/material, enum versus registration order and invalid slots. |
| EnumEnchantmentType | Ported | All/armor subslots/weapon/digger tests; hoes/bows excluded. | All item IDs, slot types, subclass identity and null item behavior. |
| BlockTorch | Ported | Opacity/normal support with missing-chunk fallback, both fences below and reentrant drop guard in live paths. | Every orientation/support, transparency/normal flag, absent chunks, nether fence and already-removed block. |
| ItemBucketMilk | Ported | 32-tick drink, unconditional decrement, server effect clear and empty bucket return. | MP/creative, counts, effect clear callback, use action/duration and stack identity. |
| BlockFluid | Partial | Water biome RGB callback and terminal drip RNG/material checks. Remains: live biome color provider and particle types. | Water/lava metadata, earlier display draws, below normal cube/movement material, float positions and RGB delegation. |
| EnchantmentLootBonus | Ported | Looting/fortune ranges/types/max level and silk exclusion. | Both IDs, bounds, signed levels and asymmetric silk/looting compatibility. |
| ItemSlab | Ported | Top-face matching slab merge with count/sound behavior; ItemBlock fallback. | All sides/metadata, edit/count rejection, failed write, merge without automatic onAdded stacking. |
| ItemBoat | Ported | Expanded-collidable eye containment rejects placement; server constructor/spawn and creative count guard. | Ray/snow offset, 0.1f32 collision border, boundary containment, MP/creative and constructor order. |
| EnchantmentNameParts | Ported | Exact word list and seeded 3/4-word draws. | Repeated reseed, count/index RNG and exact generated strings. |
| BlockStep | Ported | Double slab opaque lookup, removed onAdded merging and single-slab silk output with metadata. | Single/double bounds/opacity, all texture metadata, stacked placement, quantity and silk. |
| EnchantmentUntouching | Ported | Silk type/range and fortune-only exclusion. | Power endpoints, one level, digger eligibility and compatibility both directions. |
| ItemEnderPearl | Partial | Limit 16, creative no-op guard and throw count/pitch/sound/server constructor callback. Remains: live projectile connection. | Creative RNG unchanged, MP/server count/sound/spawn order, zero/negative counts and entity-owned heading draws. |
| BlockRedstoneWire | Ported | Power-provider side != -1; both repeater-facing/remap connections and equivalent active-repeater helper. | Side -1/0..5, all repeater states/directions, stepped wires and directional output/update recursion. |
| BlockGrass | Partial | Four spread trials after light gate and biome RGB callback. Remains: live biome provider instead of inherited climate caller. | Light/opacity boundaries, successful/failed trial RNG, biome callback coordinates and tint integration. |
| BlockVine | Partial | 1/4 server growth, up/down branch structure, shears damage 0 and biome foliage callback. Remains: live biome provider. | Attachment masks, density/neighbor direction, blocked-up fallthrough, world/supplied RNG and shears metadata. |
| EnchantmentDigging | Ported | Efficiency ranges, digger type and five levels. | Tools/hoes/bows, all power boundaries and superclass max calculation. |
| EnchantmentDurability | Ported | Unbreaking weight/type/ranges and three levels; consumer gap under ItemStack. | Item eligibility, endpoints and whole-positive-amount skip RNG. |
| EnchantmentFireAspect | Ported | Weapon type, two levels and source power ranges. | Weapon eligibility, min/max including superclass maximum and compatibility. |
| EnchantmentKnockback | Ported | Weapon type, two levels and source power ranges. | Materials, signed/overflow levels, compatibility and first-match helper. |
| EnchantmentOxygen | Ported | Head armor, three levels, 10*level interval. | Every armor slot, endpoints and respiration maximum/duplicate lookup. |
| EnchantmentWaterWorker | Ported | Head armor, one level and power interval 1..41. | Slot eligibility, endpoints, aqua affinity max>0 and negative levels. |
| ItemMetadata | Ported | Existing captured-Block ItemLog helper is behavior-equivalent to renamed ItemMetadata. | Texture Block versus placement Block, null, side 2, signed metadata and callback exceptions. |
| BlockTallGrass | Partial | Removed coordinate jitter, biome callback and fortune quantity 1+nextInt(2*f+1). Remains: live biome provider. | Subtype 0 white, other callbacks, fortune 0 draw, negative/overflow bounds, chance/ID order and shears. |
| BlockTrapDoor | Ported | Opaque-material/normal support or glowstone; positive changed ID power/provider gate. | Four supports/orientations, glowstone, changed=0 versus positive, powered query order and metadata. |
| ItemMap | Partial | Guarded bottom terrain do/while and liquid-depth gate with second read after decrement. Remains: world height/dimension and MapData owner changes. | Air/map-air blocks at y=0, water depth, dirty columns/counter and slope/shade bits/dimensions. |
| BlockPumpkin | Partial | Exact server removal/spawn plus 120 snowshovel particles exposed with constructor callback. Remains: ordinary onAdded connection to EntitySnowman. | Two snow blocks, server removal/spawn coordinates, client particles without removal and 360 double draws. |
| BlockOre | Ported | Coal/diamond/lapis fortune multiplier and base quantity/metadata. | Iron/gold no bonus, f=0/positive, negative bonus clamp, lapis draw order, chance and silk. |
| EnchantmentModifierDamage | Ported | Ordered wrapping protection accumulation. | Duplicate/invalid IDs, source flags, negative/overflow total and cap/RNG sequence. |
| EnchantmentModifierLiving | Ported | Ordered wrapping damage accumulation by creature attribute. | Attributes, duplicates, signed levels and zero/positive final modifier RNG. |
| EnumRarity | Ported | Four exact color numbers and names. | Every enum and item override including enchanted/gold apple/record. |
| ItemArmor | Ported | Material durability/reduction/enchantability and client properties. | 20 item IDs, slots, render index, max stack/damage and constructor bounds. |
| BlockLeaves | Partial | Biome foliage callback, fixed pine/birch colors and subtype-based silk. Remains: live biome RGB provider. | Metadata 0..15, fixed versus delegated colors, silk metadata and decay RNG. |
| BlockSilverfish | Ported | Silk returns hidden stone/cobble/stone brick metadata 0. | All/invalid metadata, silk versus normal spawning and MP/destruction callbacks. |
| EnumToolMaterial | Ported | Enchantability 15/5/14/10/22 added to inherited materials. | All material durability/efficiency/harvest/damage and enum order. |
| ItemAppleGold | Ported | Always glint/Epic, retained food/regeneration settings. | NBT-independent overrides, full hunger, effect probability/duration and burp RNG. |
| BlockChest | Ported | Non-normal render and tag deep copy after constructor/Gaussian motion. | Tags/counts, stackable-tag setter rejection, Gaussian order, double inventory and properties. |
| BlockStairs | Ported | Drops own stair ID instead of model; new nether stairs/model. | Every ID/metadata, quantity/chance/fortune, model callbacks and collision restoration. |
| EnchantmentData | Ported | Weighted ID/level wrapper. | Registry weight, item identity, levels and weighted iteration/draw order. |
| BlockMelon | Ported | Fortune bonus added and capped at 9. | Base/bonus draws including f=0, invalid bounds, cap, chance and offsets. |
| BlockRedstoneRepeater | Ported | Powered player destruction notifies above. | Both states, direction/delay, destruction versus removal, notifications in order. |
| BlockDispenser | Partial | Server-only tick added; inherited projectiles retained. Remains: EntityPotion splash constructor/heading branch speed 1.375/spread 3. | MP no-op, splash distinction, inventory RNG, constructor-heading-spawn-aux1002/final2000 sequence. |
| BlockFlowing | Ported | Portal material blocks flow; falling lava into water becomes stone, fizzes and returns. | All metadata/targets, downward only, neighboring hardening/fizz RNG and no horizontal continuation. |
| BlockRedstoneOre | Ported | Quantity 4+nextInt(2)+nextInt(f+1). | Lit/unlit, f=0 draw, invalid bounds, silk, chance/particles. |
| TileEntityMobSpawner | Ported | Server particle RNG loop replaced by aux2004; spawn algorithm retained. | Spawn success/failure, event position/data, removed float draws, rotation/delay and client aux rendering. |
| BlockBed | Ported | NOT_SAFE chat and zero-fortune bridge; enum adds NotSafe for callback. | All sleep results, occupancy, head/foot, MP and NOT_SAFE message without marking occupied. |
| BlockCrops | Ported | 25/growth+1 gate; base fortune 0 and seed trials 3+fortune. | Neighborhood/light/stage, trial counts, seed float positions and RNG/negative fortune. |
| BlockPane | Ported | Iron-bar drop flag true; glass-pane false yields zero ID. | Both flags/IDs, fortune/chance RNG despite zero ID and connection geometry. |
| ItemRecord | Ported | Nine new records, Rare rarity and C418 tooltips. | 11 IDs/icons/names, insertion/ejection, NBT/glint and persistence. |
| ItemSeeds | Ported | Soil parameter: farmland for old crops/stems, soul sand for wart. | All seed IDs/soils, side 1, air/edit, count and notification order. |
| ItemSword | Ported | Damage 4+materialDamage and enchantability. | Five materials, damage/durability, blocking action/duration and hit/block costs. |
| BlockDoor | Ported | All positive changed IDs propagate power regardless of provider; fortune 0. | Head/foot recursion, changed 0/nonprovider, checks, metadata and drops in order. |
| BlockSapling | Partial | Growth gate 1/7. Remains: world-owned notifying tree-generator constructor flags. | Light/metadata/species, RNG through tree selection and failed generation restoration. |
| BlockBookshelf | Ported | Three books instead of planks. | Quantity/ID/chance, fortune ignored and silk returns shelf. |
| BlockFarmland | Ported | Removed 1/5 RNG gate; hydration checked every tick. | Water scan/rain/moisture/dry crop, conversions and no removed draw. |
| BlockFire | Partial | Portal attempts only provider type<=0 and zero-fortune bridge. Remains: eternal End bedrock fire with new provider. | Dimensions, netherrack/End bedrock, portal-attempt order and rain/fire RNG. |
| BlockGlowStone | Ported | Glass material; fortune quantity clamps 1..4. | Base/bonus draws f=0/positive, harvesting, chance and glass material/sound. |
| BlockIce | Ported | Null silk creator and zero-fortune melt bridge. | Null creator, overridden harvest/water conversion/Nether, light threshold and no silk item. |
| BlockStem | Ported | 25/growth+1 gate and literal zero fortune for extra seeds. | Two fruits/stages, neighborhoods, fruit direction RNG and three seed trials. |
| ItemAxe | Ported | Expanded effective blocks and wood-material speed override. | Materials/wood IDs/slabs/pumpkins, null block, fallback and enchantability. |
| ItemBlock | Partial | Context.height() top-solid-layer test defaults to 128, preserving callbacks/sound capture. Remains: adjustable live world height field. | Custom heights, non-solid top, snow/vine, failed writes, registry replacement callback order and signed metadata. |
| ItemBow | Ported | Durability 384, creative draw/fire without arrows, consumes arrow and source damage/sound order, server spawn guard. | Arrow/no arrow/creative, charge bounds, constructor before damage, MP and breaking bow. |
| ItemDye | Ported | Name/icon damage clamped 0..15. | Negative/large metadata, virtual name, bonemeal and sheep mapping. |
| ItemFood | Ported | Spider eye food/poison and server effect RNG after burp draw. | All foods/saturation, hunger/creative, effect probabilities/durations/counts and RNG order. |
| ItemPickaxe | Ported | Rails/redstone effective list and rock/iron speed override. | Materials, rails/lit ore/new IDs, harvest levels/null and fallback. |
| ItemTool | Ported | Efficiency exposed via material helpers and enchantability added. | All tool/material/effective block combinations and durability callbacks. |
| BlockLever | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Support removal, orientation and powered notifications. Compare result, notifications and post-call RNG. |
| BlockPistonMoving | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Stored block drops force fortune 0, missing tile, MP and Facing collision offsets. Compare result, notifications and post-call RNG. |
| BlockSnow | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Unsupported/bright melt, snowball harvest, opacity 0 and drop RNG. Compare result, notifications and post-call RNG. |
| BlockTNT | Ported | Null silk creator in harvest; ordinary drops empty. | Null silk creator, priming/zero drops and metadata. Compare result, notifications and post-call RNG. |
| TileEntity | Ported | Added Cauldron, EnchantTable and Airportal factory/mapping/storage cases. | All mappings/factories, coordinate NBT and lifecycle. Compare result, notifications and post-call RNG. |
| BlockButton | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Unsupported placement/removal, pressed notifications and tick delay. Compare result, notifications and post-call RNG. |
| BlockCake | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Hunger/bites, support destruction and quantity/ID 0. Compare result, notifications and post-call RNG. |
| BlockGravel | Ported | Flint uses nextInt(10-3*fortune), without clamping; falling retained. | nextInt(10-3*fortune), invalid bounds, falling and drop RNG. Compare result, notifications and post-call RNG. |
| BlockJukeBox | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Record insertion/ejection, persistence and zero-fortune drop. Compare result, notifications and post-call RNG. |
| BlockMushroomCap | Ported | Metadata 14 all cap textures, 15 all stem. | Both types, six sides, all/invalid metadata and drop/silk. |
| BlockReed | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Support/water, age growth and item 338. Compare result, notifications and post-call RNG. |
| BlockSand | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Server guard, falling chunk gate and zero-fortune drops. Compare result, notifications and post-call RNG. |
| BlockSign | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Orientation/support, tile lifecycle and item 323. Compare result, notifications and post-call RNG. |
| BlockSnowBlock | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Four snowballs, chance and ignored fortune. Compare result, notifications and post-call RNG. |
| ItemBed | Ported | Second half set only if first survives notification. | Reentrant first-half removal, permissions/supports, MP and write/decrement order. |
| ItemEgg | Ported | Creative keeps count; server-only constructor. | MP/creative, zero counts, pitch-before-constructor and heading RNG. |
| ItemSnowball | Ported | Creative keeps count; server-only constructor. | MP/creative, zero counts, pitch-before-constructor and heading RNG. |
| TileEntityEndPortal | Ported | Empty subclass mapping Airportal, factory and storage. | Base coordinates/NBT, transient lifecycle and malformed mapping. |
| BlockCactus | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Support/liquid/growth, removal and ignored fortune. Compare result, notifications and post-call RNG. |
| BlockClay | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Four clay balls, chance RNG and ignored fortune. Compare result, notifications and post-call RNG. |
| BlockDeadBush | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Empty ID/quantity and unsupported sand. Compare result, notifications and post-call RNG. |
| BlockFlower | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Stay/light/soil and zero-fortune drops. Compare result, notifications and post-call RNG. |
| BlockFurnace | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | DropID 61 and inventory retained during lit-state swaps. Compare result, notifications and post-call RNG. |
| BlockLadder | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Support orientations/removal and ignored fortune. Compare result, notifications and post-call RNG. |
| BlockLog | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Metadata drops, leaves notifications and ignored fortune. Compare result, notifications and post-call RNG. |
| BlockMobSpawner | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Zero ID/quantity and tile removal. Compare result, notifications and post-call RNG. |
| BlockNote | Ported | Any positive changed ID triggers indirect power check. | changed 0/nonprovider/provider, power edges/query order and stored note/events. |
| BlockObsidian | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Self drop regardless of fortune and registration hardness 50. Compare result, notifications and post-call RNG. |
| BlockPistonExtension | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Extension removal drops base fortune 0, Facing offsets and metadata. Compare result, notifications and post-call RNG. |
| BlockPortal | Ported | Ambient sound volume 0.5. | Gate/pitch draws, coordinates, particles and volume bits. |
| BlockPressurePlate | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Support removal, power notifications and zero fortune. Compare result, notifications and post-call RNG. |
| BlockRail | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Slope/support/power, zero fortune and RNG. Compare result, notifications and post-call RNG. |
| BlockRedstoneTorch | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Inherited torch support, zero-fortune removal and burnout. Compare result, notifications and post-call RNG. |
| BlockStone | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | Cobblestone normal drop, silk stone and ignored fortune. Compare result, notifications and post-call RNG. |
| BlockWeb | Ported | Drop signatures/calls use shared fortune dispatch; this class ignores fortune or supplies literal 0. Existing algorithm retained. | String normal drop, silk eligibility and ignored fortune. Compare result, notifications and post-call RNG. |
| ItemSpade | Ported | Soul sand/mycelium added to effective list. | All shovel materials and IDs 88/110, fallback and enchantability. |
| Material | Ported | Separate non-push dragon egg material. | Material identities/flags/map colors and mobility. |
| MaterialWeb | Ported | Only source method/class-reference renames; behavior retained. | Direct MaterialWeb callbacks, return values, invalid/null parameters and observable order. |
| StepSoundSand | Ported | Only source method/class-reference renames; behavior retained. | Direct StepSoundSand callbacks, return values, invalid/null parameters and observable order. |
| StepSoundStone | Ported | Only source method/class-reference renames; behavior retained. | Direct StepSoundStone callbacks, return values, invalid/null parameters and observable order. |
| TileEntityFurnace | Ported | Blaze rod fuel 2400, tag-preserving inventory NBT and method renames. | All fuel IDs, blaze rod burn/cook timing, tagged/signed NBT and useability identity/distance. |
| TileEntityPiston | Ported | Only source method/class-reference renames; behavior retained. | Direct TileEntityPiston callbacks, return values, invalid/null parameters and observable order. |

## Explicitly owned simulation classes listed in the client area

| ClassName | Status | What changed / remains | Next original-bytecode checks |
|---|---|---|---|
| ContainerEnchantment | Partial | Live shelves/slot/cost/progress/XP/tag-aware shift/close; enchant_container10 button API. Remains: GUI command/buttons/name-seed snapshot and recompute-count oracle audit. | Shelf/air masks, seeds, failed/MP/level clicks, tagged count1/many shift, cursor-before-item close and validity. |
| ContainerBrewingStand | Ported | 40 slots, source transfer ranges, tile/progress roundtrip and validity. Client graphics remain with client area. | All slots/ranges, reverse merges, main/hotbar, no auto ingredient transfer, whole-slot decrement, progress and tile/distance. |
| SlotBrewingStandPotion | Ported | Potion/bottle validity, limit 1 and achievement. | Null/items, damage0/>0, right/shift pickup and achievement order. |
| SlotBrewingStandIngredient | Ported | Accepts registered recipe-string ingredients. | Null/all ingredient IDs, custom recipe mutation when registry connected and limit 64. |
| SlotEnchantmentTable | Ported | Slot store triggers costs/name recomputation. | Set/changed callback counts, MP/server seeded draws and invalid reset. |
| SlotEnchantment | Ported | Limit1, accepts all items. | Empty/stackable/tagged/broken, manual/shift and callbacks. |
| CraftingManager | Ported | New recipes/repair, wildcard block metadata and stable equal-size source order. | Entire ordered recipes, layouts/mirrors/offsets, repair overflow/tag loss and output identity. |
| RecipesFood | Ported | Shapeless soup, pumpkin seeds, fermented eye, glistering melon, blaze powder/magma cream. | Permutations, output counts/metadata, removed shaped soup and RecipesDyes ordering. |
| RecipesIngots | Ported | Nine-nugget ingot and nine nuggets from ingot. | Layouts/counts/metadata and source order. |
| FurnaceRecipes | Ported | Coal/redstone/lapis ore smelting. | Inputs16/73/21, outputs/counts/damage, inherited and invalid inputs. |

## Small cross-area edits and inherited test corrections

- Entity/player inventory conversions preserve tags/identity and compare tags during partial insertion. Player/controller files connect owned item/block callbacks, food effects/windows and properties. `entity/player.rs` adds only NotSafe for BlockBed; hostile-bed checks remain entity-owned.
- World tile/storage/tick dispatch supports new bounds/variants/persistence. `biome.rs::temperatures` fills the requested prefix while retaining a longer output buffer, matching WorldChunkManagerHell.getTemperatures; an inherited test exposed incorrect buffer replacement.
- `maps.rs` contains MapData too; only ItemMap terrain-scan logic was changed.
- `bin/compare.rs`, `entity/verify.rs`, `world/verify181/round7.rs`: minimal tile/identity exhaustiveness/constructor bridges. Their inherited byte streams do not verify new features and are not oracle10 additions.
- Client: four title/background match arms in `gui/screens/container.rs`; one item-property overlay in `render/entity/item_table.rs`; three literals replaced by ItemStack constructors in `render/entity/object_check.rs`.
- Tests: `nether_dimension.rs` compares boxed height-map slices, tests source 0.0 overworld floor/missing non-null NBT arrays/public fixed biome results; `effects.rs` reads parsed jukebox tile; `entities.rs`/`player.rs` compare canonical compound data rather than insertion order. No tests disabled.

ItemExpBottle is absent from supplied Release 1.0 source and not in this round.

## Commands and artifacts

- `cargo test -p worldgen10`: passed, `target/r10-blocks-tests.log`.
- `cargo build --release`: passed, `target/r10-blocks-build.log`.
- `target/release/beta --mc 1.0 --seed 42 --no-save --shot target/r10-smoke.png --shot-ticks 60`: passed and visually inspected, `target/r10-blocks-smoke.log`.
- `git diff --check`: passed after final report/script generation; protected-path scope audit passed.
- `target/commit-r10-blocks.sh`: explicit task file list with requested author/co-author metadata, to run from repository root after review; not executed.
