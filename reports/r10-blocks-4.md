# Release 1.0 blocks, round 4

**53 of 53 listed classes Done, 0 Not done.** `tools/verify_r10blocks4.sh` passes all 21 sections (`BLOCKS4 ALL CASES EXACT`). It is registered in `tools/verify10.sh` immediately before the final `pflush`.

I compared a full `tools/verify10.sh` run of this branch with a full run of the base commit 66f9226 in the same worktree. **No section regressed, and 31 sections that failed on the base commit now pass:**
- all 16 `chunk meshes` sections: the client tint now uses the block-owned colour dispatch
- `containers10: furnace and dispenser tiles`
- `decorations10: maps`
- `world-blocks 1.0:` bed9, cake9, door9, doors6, furnace7, machines7, piston6 and pistontile10
- `player: activate` and `player: digextras`
- three player-gui statistics/options sections, through the regenerated stats table

`world-blocks 1.0: definitions` is EXACT in both runs. The remaining full-run differences also fail on the base commit and belong to other areas: vehicle/living/first-person renderers, sky, gui10 frames, biomes/region NBT and entities3..8 sections.

## The round-3 "shared cause"

The integrator suspected one shared cause. It was four independent causes, plus one renderer range that Rust counted as a block range:

1. **furnace7 / machines7 / containertiles** (addedTileEntityList 6 vs 5).
   - Cause: 1.0 `Chunk.setBlockID*` and `Chunk.setBlockMetadata` use `Chunk.getChunkBlockTileEntity`, which never searches `World.addedTileEntityList`; only 1.0 `World.getBlockTileEntity` does. Rust used the world lookup, found a pending tile, and so skipped the original lazy `World.setBlockTileEntity` (the one with chunk-local coordinates).
   - Fix: the new `World::chunk_tile_id10` implements `Chunk.getChunkBlockTileEntity`. `refresh_changed_block_tile181` and `get_block_tile_entity` now use it.
   - `remove_block_tile_entity` now removes only the first `addedTileEntityList` occurrence, like `List.remove`.
   - `swap_furnace_block` (`BlockFurnace.updateFurnaceBlockState`) now also keeps a tile that is only pending.
   - Files: `world/tile_lifecycle.rs`, `container/tiles.rs`.
2. **piston6.** 1.0 `BlockPistonBase.ignoreUpdates` is static. Production code already used one static guard, but the round-6 fixture set a per-block field that nothing read. The fixture now sets `pistons::IGNORE_UPDATES`, and the dead `World::piston_guard` field is removed.
3. **bed9.** 1.0 `EnumStatus` has a sixth value, `NOT_SAFE`. The Rust fixture iterated only five values; production code already handled it.
4. **doors6 / door9.** In 1.0 `BlockDoor`, the upper half forwards a neighbour change to the lower half only when the neighbour provides power (`entity/player/block_hooks.rs`).
5. **cake9 / door9 ranges.** `Chunk.relightBlock` calls `World.markBlocksDirtyVertical`, which is a renderer range, not a block-owned direct range (`world/light181.rs`).

## Other production fixes found by the new checks

- `maps::MATERIAL_COLORS` now has the 30-entry 1.0 table, with the dragon-egg material at index 25. This fixes the `maps.rs:19` panic.
- `stats_table.rs` and `tools/oracle10/statlist.txt` are regenerated from the 1.0 jar (`oracle.sh statlist`). The old table lacked every new 1.0 block/item stat, so mine/use/break/craft stats for ids 110-122, 369-382 and records 2258-2266 were -1.
- `EntityPlayer.onUpdate` now finishes an item use only when `!multiplayerWorld`. The new `Entity::finish_item_use` implements `func_35208_ae`.
- ItemBow damages the in-use stack in place, so a broken bow stays as a zero-size stack.
- `Block.dropBlockAsItem_do` does nothing on remote worlds.
- 1.0 `BlockSnow.harvestBlock` drops without the remote guard, then removes the layer.
- With Silk Touch:
  - redstone ore (both ids) drops ore 73
  - ice (null `func_41049_c_`) drops nothing but still melts
- `BlockEndPortal.onBlockAdded` no longer adds a tile.
- `BlockEnchantmentTable.blockActivated` no longer looks up a tile.
- BlockBrewingStand's own `Random` is exposed as `blocks10::brewing_stand_random10()`.
- ItemBlock icons for blocks 110-122 are fixed at construction, using metadata 0. `item_properties` now covers records 2256 and 2257.
- `color_multiplier10` now includes redstone wire. The client (`render/blocks.rs`, `render/access.rs`) now uses `color_multiplier10` with the biome callbacks `biome_tint10` and the new `biome_water10`.

## Checks

All modes are new original-bytecode modes in `tools/oracle10/src`, with the shared writer `Blocks4Oracle.java`. The Rust comparators are in `crates/worldgen10/src/world/verify181/blocks4.rs` and run as `compare10 worldblocks181 b4<kind>`; set `B4DEBUG=1` to print case offsets.

- `right` (`Mode_wbb4right`): `PlayerController.sendUseItem` for 31 items: buckets, milk, boat, snowball, egg, pearl, potions including splash, bottle, lily pad, bow, foods, sword, a block and shears.
  - Terrains (8): stone, water source, flowing water, lava, air, snow, occupied lily pad, and a pig blocking the boat.
  - Also varied: 3 pitches, 2 yaws, survival / hungry with effects / creative, and local / remote.
  - Compares inventory, item in use, food, effects, blocks, entities (including potion damage), the World, itemRand and Math RNGs, and world events.
- `eat` (`Mode_wbb4eat`): `EntityPlayer.func_35208_ae` -> `onFoodEaten` for every food, milk, 13 potion damages, the bow and the sword.
  - Varies count 1/2, full/hungry, three effect sets and remote.
  - Also runs `ItemBow.onPlayerStoppedUsing` over 14 charge times, 0-2 arrows, creative, durability 0/383/384 and remote.
- `use` (`Mode_wbb4use`): `Item.onItemUse` for 53 item/damage pairs, across 6 sides and 23 configurations with varying yaw. The items are:
  - slabs, the lily pad, vine, tall grass, leaves, saplings, logs, stone brick and cloth
  - all seeds, including nether wart on soul sand, and dyes -1..16 including bonemeal
  - the bed, records into an empty or full jukebox, doors, reeds, the End portal frame and potions
- `items` (`Mode_wbb4items`): every registered item id 0..2266.
  - Covers the potion-recipe getters `func_40406_n`/`func_40405_m`, `getItemEnchantability`, rarity, glint, `func_40708_t`, use action and duration, tooltip lines (plain and enchanted) and the potion display name.
  - For every visual-subclass item it also covers icon, colour, placed metadata and names over damage -2..19.
- `harvest` (`Mode_wbb4harvest`): `Block.harvestBlock` for every block 1..122 × 7 metadata × {hand, pick, Silk Touch, Fortune III, shears, Fortune I} × local/remote. Compares drops, blocks, RNGs, events and exhaustion. Includes BlockPistonMoving's stored drops.
- `ticks` (`Mode_wbb4ticks`):
  - dragon egg: added, neighbour and tick, including instant fall, the missing-chunk gate and remote
  - random ticks: nether wart (overworld, Hell, End), mycelium and grass (bright, dark, opaque cover), vine growth, both stems, and fire (End bedrock, Nether)
  - onBlockAdded: pumpkin and jack-o'-lantern snowman, End portal flag × dimension, fire portal guard
  - activation of the enchanting table and brewing stand
  - End portal collision: rider, ridden, non-player, remote
  - removal: brewing-stand scatter with its own RNG, End portal and enchanting table tile removal
  - splash-potion dispensing and the remote dispenser guard
  - display ticks: fluid drips, portal ambience, mycelium
  - every block's mobility, map colour and material flags
- `colors` (`Mode_wbb4colors`): `Block.colorMultiplier` for 15 blocks × 16 metadata × 90 positions, on a biome-varying WorldChunkManager (swamp grass and foliage tint, swamp water, temperature that varies with y).
- `tiles` (`Mode_wbb4tiles`): `TileEntity.createAndLoadEntity` for Cauldron, EnchantTable and Airportal plus the older mappings, including malformed input, and the written NBT.
- `stack` (`Mode_wbb4stack`): `ItemStack.damageItem` with held Unbreaking I/III, on the held stack or another one (sword, armour, block, bow), amounts 0/1/2/7/-1, by a player or a zombie. Compares the stack, RNGs, break effects and stats.
- Inherited checks rerun in this suite: `piston5`, `piston6`, `push6`, `bed9`, `door9`, `doors6`, `cake9`, `furnace7`, `inventory7`, `machines7`, `containertiles` and `maps`.

## Validation

- `cargo build --release` passes for the whole workspace, and `cargo test -p worldgen10` passes.
- `beta --mc 1.0 --seed 42 --no-save --shot target/r10-smoke.png --shot-ticks 60` exits 0, and the screenshot renders normally (forest, HUD, hand).
- After the World changes I reran, all EXACT: `verify_r10blocks4.sh`, `verify_r10blocks3.sh`, `verify_r10world3.sh`, `verify_r10entities2.sh`, `verify_r10entities3.sh`, `verify_world_blocks10.sh` and `verify_containers10.sh`.
- `oracle.sh raw 0 0 0 0 0 t.bin` succeeds.

## Classes

| Class | Status | Ported / checks |
|---|---|---|
| ItemPotion | Done | drink start vs splash throw (count, sound, itemRand, EntityPotion spawn, remote), onFoodEaten effects/bottle return, icon/colour/glint/tooltip/name, splash dispensing; checks: `right`, `eat`, `items`, `ticks` |
| Item | Done | potion-recipe getters, enchantability, rarity, glint, func_40708_t, use action/duration, tooltip base, func_40402_a ray (bucket/bottle/lily) through ordinary dispatch; checks: `items`, `right` |
| ItemStack | Done | damageItem with held Unbreaking (World.rand), break effects/stat/count/reset, rarity/glint/tooltip/enchantable delegation; checks: `stack`, `items` |
| BlockDragonEgg | Done | onBlockAdded/neighbour scheduling, updateTick fall (entity vs instant, chunk gate, remote); teleport also in blocks3; checks: `ticks` |
| BlockBrewingStand | Done | activation (remote, missing tile), removal scatter with its own Random, factory, retained tile; checks: `ticks`, `tiles` |
| Block | Done | harvestBlock Silk Touch/Fortune dispatch, func_41049_c_ overrides, exhaustion, fortune drops, colorMultiplier dispatch; checks: `harvest`, `colors` |
| BlockEndPortal | Done | onBlockAdded flag/dimension (no tile), player/rider/non-player collision, removal, Airportal factory; checks: `ticks`, `tiles` |
| BlockNetherStalk | Done | Hell-biome growth (RNG drawn only in Hell), stay check, mature fortune drops; checks: `ticks`, `harvest` |
| BlockEnchantmentTable | Done | activation without tile lookup, tile removal, EnchantTable factory; checks: `ticks`, `tiles` |
| BlockMycelium | Done | dark reversion, four spread trials, townaura display tick, dirt drop; checks: `ticks`, `harvest` |
| BlockPistonBase | Done | static ignoreUpdates in all entry points, extend/retract; checks: `piston5`, `piston6`, `push6` |
| BlockEndPortalFrame | Done | yaw placement metadata through the item path (eye insertion/geometry in blocks3); checks: `use`, `harvest` |
| ItemLilyPad | Done | water-source ray placement, flowing/occupied/air rejection, creative count, icon/colour; checks: `right`, `items` |
| ItemGlassBottle | Done | water ray, decrement/replacement/inventory/drop; checks: `right` |
| ItemColored | Done | vine/tall-grass icons, colours, placed metadata, names, placement; checks: `items`, `use` |
| BlockLilyPad | Done | placement, tint, harvest/drops; checks: `right`, `colors`, `harvest` |
| ItemBucket | Done | func_40402_a ray, source pickup with creative depleteBuckets, placement, cow milking; checks: `right` |
| ItemBucketMilk | Done | right click starts use; onFoodEaten clears effects and returns a bucket; remote; checks: `right`, `eat` |
| BlockFluid | Done | biome water colour, drip display particles; checks: `colors`, `ticks` |
| ItemSlab | Done | all sides/metadata merges, count/permission rejection, merge sound; checks: `use` |
| ItemBoat | Done | entity look-box blocking, snow offset, creative count, remote; checks: `right` |
| ItemEnderPearl | Done | creative no-op, count/sound/itemRand/spawn, remote; checks: `right` |
| BlockGrass | Done | four spread trials, biome colour callback; checks: `ticks`, `colors` |
| ItemMetadata | Done | side-2 icon from its own block, placed metadata, placement; checks: `items`, `use` |
| BlockVine | Done | 1.0 growth (1/4 chance, restructured branches), biome foliage colour, shears; checks: `ticks`, `colors`, `harvest` |
| ItemMap | Done | rename-only delta plus the 1.0 MapColor table used for terrain colours; checks: `maps` |
| BlockTallGrass | Done | biome colour callback, fortune quantity, shears/seed drops; checks: `colors`, `harvest` |
| BlockPumpkin | Done | snowman: removal order, spawn position, 120 particles, remote particles only; checks: `ticks` |
| ItemAppleGold | Done | epic rarity, glint, regeneration on eating; checks: `items`, `eat` |
| BlockLeaves | Done | biome foliage colour, shears/fortune harvest; checks: `colors`, `harvest` |
| BlockDispenser | Done | splash-potion constructor/heading/aux SFX, remote neighbour guard; checks: `ticks` |
| ItemSeeds | Done | crop/stem/wart soil pairs, side, air, count; checks: `use` |
| ItemRecord | Done | insertion into empty/full jukebox, rarity rare, tooltip, icons; checks: `use`, `items` |
| BlockBed | Done | NOT_SAFE chat, fortune-0 drops; checks: `bed9` |
| BlockDoor | Done | upper half forwards only power providers; checks: `door9`, `doors6` |
| ItemFood | Done | burp sound, effect probability/duration, saturation; checks: `eat`, `right` |
| ItemDye | Done | clamped icon/name for signed and oversized damage, bonemeal on stems; checks: `use`, `items` |
| ItemBow | Done | creative/no-arrow gates, charge, damage before sound, broken bow, remote; checks: `eat`, `right` |
| BlockStem | Done | 25/rate+1 growth chance, fruit placement, seed drops; checks: `ticks`, `harvest` |
| BlockIce | Done | null silk creator (still melts), fortune-0 drop; checks: `harvest` |
| BlockFire | Done | End-bedrock eternal fire, portal attempt only in dimension 0; checks: `ticks` |
| TileEntity | Done | Cauldron/EnchantTable/Airportal mappings, load/write; checks: `tiles` |
| BlockTNT | Done | null Silk Touch creator (no drop); checks: `harvest` |
| BlockPistonMoving | Done | stored drop with fortune 0, live lifecycle; checks: `harvest`, `piston6` |
| TileEntityEndPortal | Done | Airportal mapping, transient lifecycle/removal; checks: `tiles`, `ticks` |
| ItemSnowball | Done | creative count guard, sound/itemRand/spawn, remote; checks: `right` |
| ItemEgg | Done | creative count guard, sound/itemRand/spawn, remote; checks: `right` |
| ItemBed | Done | second half only if the first half survived; checks: `use` |
| BlockCake | Done | range/notification behaviour; checks: `cake9` |
| TileEntityFurnace | Done | blaze-rod fuel, NBT rename, live replay; checks: `furnace7`, `inventory7`, `machines7`, `containertiles` |
| Material | Done | 1.0 materials including field_41056_z: mobility, map colour, flags; checks: `ticks`, `maps` |
| BlockPortal | Done | ambient sound volume 0.5 with RNG; checks: `ticks` |
| BlockFurnace | Done | fortune-0 drop dispatch, lit-state tile retention; checks: `furnace7`, `machines7` |

**Unreachable in game, not held back:** custom Item subclasses, null players/worlds, and Java object identity of PotionEffect lists.

**Cross-area edits** (minimal and commented):
- World tile lookup (`tile_lifecycle.rs`)
- light renderer range (`light181.rs`)
- EntityPlayer item-use finish and bow (`mechanics.rs`)
- regenerated `stats_table.rs`
- client tint path (`client10/src/render/blocks.rs`, `access.rs`)
