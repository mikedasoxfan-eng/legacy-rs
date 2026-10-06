# Release 1.0 blocks, round 3

**88 Done; 58 Not done, across all 146 assigned classes.** The focused round-3 suite passes all 25 sections; the wider inherited suites retain mismatches. Area completion is not claimed.

## Changes

Added `Mode_r10blocks3` plus the Rust `r10blocks3` comparator and `tools/verify_r10blocks3.sh`, hooked immediately before verify10's final pflush. The oracle invokes original release-jar methods on its pinned JVM. Fixtures compare values, raw float/double bits, field/inventory state, ordered events and RNG continuation.

Re-extracted release block/material/default geometry, visual, item-registry and ordered recipe/smelting tables. Added missing blaze rod, ghast tear and gold nugget registrations; removed duplicate manual recipe overlays and stale mushroom texture overrides. Corrected separate direct-side versus metadata texture methods. Kept material compatibility aliases and replaced a numeric piston material reference with the dragon-egg symbol.

Connected ordinary End-eye use to the merged World Stronghold locator and EntityEnderEye constructor/target/spawn; corrected its entity sound position. Fixed creative full-inventory bottle handling, lily-pad/Nether-wart height boundaries, End-portal direct/world collision distinction, direct zero drop IDs, mycelium/lily-pad step sounds and portal-frame break sound. Torch support now selects missing-chunk fallback independently of height. The ItemStack comparison probe now models the null inventory/world of its corresponding Unsafe-allocated Java probe; production Unbreaking/break animation already use initialized live dependencies.

Cross-area bridges are limited to those item-to-World/entity calls, World height/support reads, collision selection and the piston material consumer. Protected Beta crates/oracles/scripts and client code were not edited. Nothing was committed.

Unreachable in game: custom mutable registry replacement/registration, custom callback Items rewriting stack fields, detached/null entity contexts, malformed Java-only API inputs, and unobserved nested NBT/PotionEffect identity or fail-fast iteration. These do not hold classes back. Client screens/renderers and World generation/provider algorithms remain with their owners; rows identify missing in-game checks or live simulation behavior.

## Validation

- Whole-workspace `cargo build --release` passed; existing warnings remain. `target/r10-blocks3-final-build.log`.
- `cargo test -p worldgen10` passed: 80 tests, zero failures. `target/r10-blocks3-tests-final.log`.
- Requested `beta.exe --mc 1.0 --seed 42 --no-save --shot target/r10-smoke.png --shot-ticks 60` exited 0. Inspected terrain/trees/sky, hand, HUD and achievement; they render normally. Inherited client label still reads Beta 1.8.1. `target/r10-blocks3-smoke-final.log`.
- `verify_r10blocks3.sh --built`: **25/25 sections EXACT**, ending `BLOCKS3 ALL CASES EXACT`. `target/r10-blocks3-focused-final.log` (also `target/r10-blocks3-final-audit.log`).
- The 19 new modes are potion, enchantments, helper, modifiers, names, animation, potionitem, brewing, geometry, particles, dragon, cauldron, eye, eyeright, recipes, fortune, enchantcontainer, sounds and brewcontainer. They cover exhaustive potion/ingredient metadata, every enchantment, brewing completion/abort/reload, all animation fields, enchant costs/buttons/levels/tags/RNG, slot validity/transfers, cauldron and End-eye state/events, all 122 block sounds and fortune methods. The definition checker and inherited itembase10/blockfull11/itemstack11/stackdata11/stacknbt11 also pass.
- Wider world-blocks baseline: most sections EXACT, including registry, definitions, plants/growth, drops, fluids, materials, chest and tile serialization. Fixtures interrupted by a temporary oracle compile error were rerun after correction: ore7, thin7, cake181, trapdoor8, repeater8, reed8, cactus8 and textures pass. Final visual lookup also passes all 246,896 bytes after removing stale overrides: `target/r10-blocks3-visual-final.log`. Broader logs: `target/r10-blocks3-worldblocks-final.log` and `target/r10-blocks3-final-audit.log`.
- Remaining measured block failures: doors6/door9, piston6, cake9, bed9, furnace7 and machines7. World-owned spawn/relocation/oldformat4/fileclasses/legacy5 also differ. Wider container replay still fails at record 1412 in stat-hook count, and machine replay at frame 1/furnace10 slot0 presence. These are not passing proofs.
- Required `oracle.sh raw 0 0 0 0 0 blocks3-raw.bin` succeeds and compiles all Java sources together. Shell syntax and `git diff --check` pass. `target/commit-r10-blocks.sh` stages 25 explicit paths and uses the required author/trailers; it was not executed.

`blocks3:` names are sections of `tools/verify_r10blocks3.sh`; `worldblocks181:` names run in `tools/verify_world_blocks10.sh`. Done refers to the complete changed/new delta; signature/rename-only classes use family checks, as permitted by the common task. Rows marked Not done distinguish missing verification from measured live failures.

## All 136 blocks classes

| Class | Status | Exact checks / what remains |
|---|---|---|
| PotionHelper | Done | `blocks3:potion`, `blocks3:brewing`: complete parser/effect/color/prefix/ingredient delta; exhaustive metadata and masked shifts. |
| EnchantmentHelper | Done | `blocks3:helper`, `blocks3:modifiers`, `blocks3:enchantcontainer`: candidate maps, weighted rolls, compatibility, modifiers and RNG. |
| TileEntityBrewingStand | Done | `blocks3:brewing`, `blocks3:brewcontainer`: ingredient/metadata combinations, completion/abort/reload, signed slots, bottle masks and transfers. |
| ItemPotion | Not done | Potion properties/drinking/cache and brewing pass. Add exact right-click splash construction/heading/spawn, creative/remote counts and RNG/event order. |
| ItemEnderEye | Done | `blocks3:eye`, `blocks3:eyeright`: insertion/ring scan, remote/creative, ray/locator callback, targets/spawn/sounds and RNG. Ordinary use connects the merged World locator. |
| Item | Not done | itembase10, registry/actions pass. Add exact new base rarity/glint/tooltips and potion-recipe getters through ordinary dispatch. |
| ItemStack | Not done | itemstack11, stackdata11, stacknbt11 and enchantment tags pass. Add initialized-player damageItem checks for held Unbreaking on separately damaged armor/tools, skip/break/stat/count/reset and RNG. |
| BlockDragonEgg | Not done | Teleport/occupied attempts, particles/RNG, geometry/drops/sounds pass. Add scheduled/instant/entity falling, chunk gate and placement/removal order. |
| BlockBrewingStand | Not done | Geometry/display/drop and brewing tile checks pass. Add exact removal scatter/static RNG/retained tile counts, factory and activation callbacks. |
| BlockCauldron | Done | `blocks3:cauldron`, `blocks3:geometry`, `blocks3:fortune`, `blocks3:sounds`: interactions/full inventory/creative handling, geometry/textures and defaults. |
| Block | Not done | Definitions/registry/neighbor brightness/base callbacks/fortune methods pass. Add equipped Silk Touch/Fortune harvesting, chance/subtypes/null creators and drop-event order. |
| TileEntityEnchantmentTable | Done | `blocks3:animation`: every field/raw float bit, static RNG, page retries, wrapping, player-radius transitions and large-coordinate rounding. |
| Enchantment | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| BlockFence | Done | `blocks3:geometry`, `worldblocks181:geometry`, `blockfull11`: wood/nether connections, gate/normal-cube/pumpkin predicates and bounds. |
| BlockEndPortal | Not done | Geometry/display pass; direct versus world collision boxes corrected. Add tile factory/onAdded ALLOW/dimension order and player/mount/passenger collision travel. |
| BlockNetherStalk | Not done | Height/stay/texture/drop-method checks pass. Add Hell growth/stage changes and mature custom harvest/fortune/chance events/RNG. |
| BlockEnchantmentTable | Not done | Geometry/bookshelf display/RNG pass. Add factory and server/remote activation/container callbacks. |
| BlockMycelium | Not done | Geometry/display/defaults/sounds pass. Add light/opacity conversion and four spread trials with reads/writes/RNG. |
| EnchantmentProtection | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| BlockPistonBase | Not done | push6/piston5 pass; dragon-egg material consumer corrected. piston6 differs at byte 533340: Java air, Rust moving-piston ID 36; resolve retract/tile lifecycle. |
| BlockEndPortalFrame | Not done | Geometry/insertion/registered placement pass. Add focused facing/yaw placement and eye-reset check. |
| ItemLilyPad | Not done | Complete Release in-game delta still needs an exact check for: Water IDs/metadata, air, failed write, permissions, MP/creative and ray flags. |
| ItemGlassBottle | Not done | Creative full-inventory insertion corrected. Add exact water ray/edit, replacement/inventory/drop, count and RNG checks. |
| ItemColored | Not done | Default registrations work. Add exact signed-metadata icon/color/name suffix and captured Block callbacks. Custom registry replacement is unreachable in game and is not a blocker. |
| BlockLilyPad | Not done | Geometry and custom-height stay pass. Add direct source-water placement predicate, render/tint and live placement/drop callback checks. |
| EnchantmentDamage | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| BlockMushroom | Done | `worldblocks181:mushrooms`, `worldblocks181:plants`: spread RNG, light/mycelium stay and growth/restoration delta. |
| TileEntityChest | Done | `worldblocks181:chestlife`, `worldblocks181:chestopen5`: open/close sounds and lid threshold delta. |
| ItemBucket | Not done | Complete Release in-game delta still needs an exact check for: Empty/full ray, fluids/Nether, permissions, creative stack identity and replacement order. |
| EnumEnchantmentType | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| EnumArmorMaterial | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| ItemBucketMilk | Not done | Complete Release in-game delta still needs an exact check for: MP/creative, counts, effect clear callback, use action/duration and stack identity. |
| BlockTorch | Done | `blockfull11`, `worldblocks181:placepred4`: support predicate, placement faces/heights; corrected missing-chunk fallback. |
| BlockFluid | Not done | Fluid simulation passes. Add changed biome-water color callbacks across distinct 1.0 biome providers and live delegation. |
| ItemSlab | Not done | Complete Release in-game delta still needs an exact check for: All sides/metadata, edit/count rejection, failed write, merge without automatic onAdded stacking. |
| EnchantmentLootBonus | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| ItemBoat | Not done | Complete Release in-game delta still needs an exact check for: Ray/snow offset, 0.1f32 collision border, boundary containment, MP/creative and constructor order. |
| EnchantmentNameParts | Done | `blocks3:names`: seeded names, word list, repeated resets and RNG continuation. |
| ItemEnderPearl | Not done | Complete Release in-game delta still needs an exact check for: Creative RNG unchanged, MP/server count/sound/spawn order, zero/negative counts and entity-owned heading draws. |
| EnchantmentUntouching | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| BlockStep | Done | `worldblocks181:slabs6`, `worldblocks181:materials6`, `worldblocks181:textures`, definitions: subtype/merge/texture delta. |
| BlockRedstoneWire | Done | `worldblocks181:wire8`: changed-neighbor/power delta and event/RNG traces. |
| BlockGrass | Not done | Growth/plains color pass. Verify changed biome RGB callbacks across biomes and live tint delegation. |
| ItemMetadata | Not done | Complete Release in-game delta still needs an exact check for: Texture Block versus placement Block, null, side 2, signed metadata and callback exceptions. |
| EnchantmentWaterWorker | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| EnchantmentOxygen | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| EnchantmentKnockback | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| EnchantmentFireAspect | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| EnchantmentDurability | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| EnchantmentDigging | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| BlockVine | Not done | Drops/shears/plains color pass. Verify changed biome RGB across biomes and height/attachment callbacks. |
| ItemMap | Not done | Remaining World-owned MapData/provider behavior and exact item dimension/texture/update callbacks; default registry is correct. |
| BlockTrapDoor | Done | `worldblocks181:trapdoor8`, `worldblocks181:trapdoors6`: changed support/power predicates and metadata behavior. |
| BlockTallGrass | Not done | Drops/shears/plains color pass. Verify changed biome RGB across biomes and live tint delegation. |
| BlockPumpkin | Not done | Complete Release in-game delta still needs an exact check for: Two snow blocks, server removal/spawn coordinates, client particles without removal and 360 double draws. |
| ItemArmor | Done | `blocks3:enchantments`, `worldblocks181:registry`: material constructor, reduction/durability and enchantability delta. |
| EnumRarity | Done | `blocks3:enchantments`, `blocks3:helper`, `blocks3:modifiers`: all registered variants, levels/ranges, material/type tables, compatibility and modifiers. |
| EnchantmentModifierLiving | Done | `blocks3:modifiers`: living/protection accumulation and process-wide modifier RNG. |
| EnchantmentModifierDamage | Done | `blocks3:modifiers`: living/protection accumulation and process-wide modifier RNG. |
| BlockOre | Done | `blocks3:fortune`, `worldblocks181:drops6`: quantity/drop-ID fortune methods and RNG continuation. |
| ItemAppleGold | Not done | Complete Release in-game delta still needs an exact check for: NBT-independent overrides, full hunger, effect probability/duration and burp RNG. |
| EnumToolMaterial | Done | `worldblocks181:registry`, `blocks3:helper`, `blocks3:enchantments`: all item/block strength/harvest pairs and damage/durability/material/enchantability delta. |
| BlockSilverfish | Done | `worldblocks181:silverfish`, `blocks3:fortune`, definitions: infested metadata/hardness/drop delta. |
| BlockLeaves | Not done | Drops/shears/growth/plains color pass. Verify changed biome RGB across biomes and equipped fortune/silk harvesting. |
| EnchantmentData | Done | `blocks3:helper`, `blocks3:enchantments`: ID/level/weight fields and weighted-list order. |
| BlockStairs | Done | `worldblocks181:stairs8`, `blocks3:geometry`, definitions: stair model registration/geometry/default delta. |
| BlockChest | Done | `worldblocks181:chests`, `worldblocks181:chestlife`, `blockfull11`: changed placement/adjacency predicate and inventory behavior. |
| BlockRedstoneRepeater | Done | `worldblocks181:repeater8`: changed-neighbor power/transition delta. |
| BlockMelon | Done | `blocks3:fortune`, `worldblocks181:drops6`: quantity/drop-ID fortune methods and RNG continuation. |
| TileEntityMobSpawner | Done | `worldblocks181:spawner8`: renamed method/class references and unchanged spawn behavior. |
| BlockRedstoneOre | Done | `blocks3:fortune`, `worldblocks181:drops6`: quantity/drop-ID fortune methods and RNG continuation. |
| BlockFlowing | Done | `worldblocks181:fluids`, `worldblocks181:flow5`: update/removal signature and notification delta. |
| BlockDispenser | Not done | Existing callback suites pass. Add exact splash-potion constructor/heading/aux/spawn with static/world RNG. |
| ItemSword | Done | `worldblocks181:registry`, `blocks3:helper`, `blocks3:enchantments`: all item/block strength/harvest pairs and damage/durability/material/enchantability delta. |
| ItemSeeds | Not done | Complete Release in-game delta still needs an exact check for: All seed IDs/soils, side 1, air/edit, count and notification order. |
| ItemRecord | Not done | Complete Release in-game delta still needs an exact check for: 11 IDs/icons/names, insertion/ejection, NBT/glint and persistence. |
| BlockPane | Done | `blocks3:fortune`, `worldblocks181:plants`, `worldblocks181:growth4`, definitions: changed drop signatures, growth and geometry/default delta. |
| BlockCrops | Done | `blocks3:fortune`, `worldblocks181:plants`, `worldblocks181:growth4`, definitions: changed drop signatures, growth and geometry/default delta. |
| BlockBed | Not done | bed9 differs at byte 546: Java air, Rust bed ID 26. Resolve placement/neighbor removal order. |
| BlockSapling | Done | `blocks3:fortune`, `worldblocks181:plants`, `worldblocks181:growth4`, definitions: changed drop signatures, growth and geometry/default delta. |
| BlockDoor | Not done | doors6/door9 differ in metadata: Java 4, Rust 0. Resolve power/placement notification ordering. |
| ItemTool | Done | `worldblocks181:registry`, `blocks3:helper`, `blocks3:enchantments`: all item/block strength/harvest pairs and damage/durability/material/enchantability delta. |
| ItemPickaxe | Done | `worldblocks181:registry`, `blocks3:helper`, `blocks3:enchantments`: all item/block strength/harvest pairs and damage/durability/material/enchantability delta. |
| ItemFood | Not done | Complete Release in-game delta still needs an exact check for: All foods/saturation, hunger/creative, effect probabilities/durations/counts and RNG order. |
| ItemDye | Not done | Complete Release in-game delta still needs an exact check for: Negative/large metadata, virtual name, bonemeal and sheep mapping. |
| ItemBow | Not done | Complete Release in-game delta still needs an exact check for: Arrow/no arrow/creative, charge bounds, constructor before damage, MP and breaking bow. |
| ItemBlock | Done | `blockfull11`, `itembase10`, `stackdata11`: all registered block-item placements, faces/heights/replacements/collisions, metadata, writes/sounds/RNG. |
| ItemAxe | Done | `worldblocks181:registry`, `blocks3:helper`, `blocks3:enchantments`: all item/block strength/harvest pairs and damage/durability/material/enchantability delta. |
| BlockStem | Not done | Complete Release in-game delta still needs an exact check for: Two fruits/stages, neighborhoods, fruit direction RNG and three seed trials. |
| BlockIce | Not done | Complete Release in-game delta still needs an exact check for: Null creator, overridden harvest/water conversion/Nether, light threshold and no silk item. |
| BlockGlowStone | Done | `blocks3:fortune`, `worldblocks181:drops6`: quantity/drop-ID fortune methods and RNG continuation. |
| BlockFire | Not done | fire4 passes. Add End-bedrock eternal-fire and provider portal-attempt guards with notifications/RNG. |
| BlockFarmland | Done | `blocks3:fortune`, `worldblocks181:plants`, `worldblocks181:growth4`, definitions: changed drop signatures, growth and geometry/default delta. |
| BlockBookshelf | Done | `blocks3:fortune`, `worldblocks181:plants`, `worldblocks181:growth4`, definitions: changed drop signatures, growth and geometry/default delta. |
| TileEntity | Not done | tileentity10 passes but its mapping fixture omits the new IDs. Add Cauldron/EnchantTable/Airportal factory/load/write/lifecycle. |
| BlockTNT | Not done | Complete Release in-game delta still needs an exact check for: Null silk creator, priming/zero drops and metadata. Compare result, notifications and post-call RNG. |
| BlockSnow | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockPistonMoving | Not done | Piston tile NBT/extension removal pass; piston6 still differs. Verify stored drops force fortune zero and fix live lifecycle. |
| BlockLever | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| TileEntityEndPortal | Not done | Complete Release in-game delta still needs an exact check for: Base coordinates/NBT, transient lifecycle and malformed mapping. |
| ItemSnowball | Not done | Complete Release in-game delta still needs an exact check for: MP/creative, zero counts, pitch-before-constructor and heading RNG. |
| ItemEgg | Not done | Complete Release in-game delta still needs an exact check for: MP/creative, zero counts, pitch-before-constructor and heading RNG. |
| ItemBed | Not done | Complete Release in-game delta still needs an exact check for: Reentrant first-half removal, permissions/supports, MP and write/decrement order. |
| BlockSnowBlock | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockSign | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockSand | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockReed | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockMushroomCap | Done | `worldblocks181:textures`, `blocks3:fortune`: release cap/stem metadata textures and changed drop signature. |
| BlockJukeBox | Done | `blocks3:fortune`, `worldblocks181:jukebox10`: zero-fortune dispatch and persisted record behavior. |
| BlockGravel | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockCake | Not done | cake181 passes; cake9 differs at byte 35711: Java range-notification count 0, Rust 1. Fix notification behavior. |
| BlockButton | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| TileEntityPiston | Done | `worldblocks181:pistontile10`: rename-only delta and unchanged serialized/moving tile behavior. |
| TileEntityFurnace | Not done | furnacetile10/inventory7/fuel registrations pass. containertiles/machines7 differ at byte 1097: frame 1, furnace10.slot0 presence. Resolve live tile replay/lifecycle. |
| StepSoundStone | Done | `blocks3:sounds`, `worldblocks181:sounds4`: rename-only delta, names and volume/pitch bits. |
| StepSoundSand | Done | `blocks3:sounds`, `worldblocks181:sounds4`: rename-only delta, names and volume/pitch bits. |
| MaterialWeb | Done | definitions, `worldblocks181:thin7`: rename-only material behavior/flags and unchanged web behavior. |
| Material | Not done | All 30 extracted material flags match; symbolic dragon-egg mobility consumer fixed. Add exact no-push mobility/map-color checks. |
| ItemSpade | Done | `worldblocks181:registry`, `blocks3:helper`, `blocks3:enchantments`: all item/block strength/harvest pairs and damage/durability/material/enchantability delta. |
| BlockWeb | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockStone | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockRedstoneTorch | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockRail | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockPressurePlate | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockPortal | Not done | Add exact changed ambient sound volume 0.5 with coordinate, pitch, particle and RNG sequence; step/break sound checks pass. |
| BlockPistonExtension | Done | `blocks3:fortune`, `worldblocks181:extension9`: zero-fortune dispatch and extension/base removal behavior. |
| BlockObsidian | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockNote | Done | `worldblocks181:music6`, `blocks3:fortune`: changed-neighbor power predicate and drop signature. |
| BlockMobSpawner | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockLog | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockLadder | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockFurnace | Not done | furnace4 passes; furnace7 differs at byte 59713: addedTileEntityList size Java 6, Rust 5. Fix lit-state tile retention/publication order. |
| BlockFlower | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockDeadBush | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockClay | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |
| BlockCactus | Done | `blocks3:fortune`, definitions: complete changed fortune/drop-signature or constructor delta, all metadata/quantities/drop IDs/RNG. Existing plant/drop/geometry fixtures also pass. |

## Owned container and recipe simulation

| Class | Status | Exact checks / what remains |
|---|---|---|
| ContainerEnchantment | Not done | enchantcontainer passes recomputation/costs/buttons/levels/tags/progress/RNG. Add shift/close/validity and crafting-progress callback order. |
| ContainerBrewingStand | Not done | brewcontainer passes layout/validity/clicks/transfers/achievements. Add progress notifications, retained tile roundtrip and validity callbacks. |
| SlotBrewingStandPotion | Done | `blocks3:brewcontainer`, `blocks3:brewing`: registered-item validity, limits, potion achievement, left/right/shift behavior. |
| SlotBrewingStandIngredient | Done | `blocks3:brewcontainer`, `blocks3:brewing`: registered-item validity, limits, potion achievement, left/right/shift behavior. |
| SlotEnchantmentTable | Not done | enchantcontainer proves double recomputation on putStack. Add decrement/direct-change callbacks and RNG. |
| SlotEnchantment | Not done | Add focused original-bytecode accepts-all/limit and manual/shift callback checks. |
| CraftingManager | Not done | Entire ordered release table matches; duplicate overlays removed. Container replay fails at record 1412: Java 2 stat callbacks, Rust 1. Isolate matching/repair outputs and callback/tag-loss behavior. |
| RecipesFood | Done | `blocks3:recipes`: complete release recipe/smelting registrations, output counts/metadata and stable source order. |
| RecipesIngots | Done | `blocks3:recipes`: complete release recipe/smelting registrations, output counts/metadata and stable source order. |
| FurnaceRecipes | Done | `blocks3:recipes`: complete release recipe/smelting registrations, output counts/metadata and stable source order. |

ItemExpBottle is absent from the original Release 1.0 class list and was not added.
