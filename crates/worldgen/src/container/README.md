# Beta 1.7.3 containers

`Container` owns crafting/result and tile inventory views; `InventoryPlayer`
owns main, armor and cursor stacks. `World::container_command` handles window
messages and synchronizes the views with real player and tile inventories.
`World::container_snapshot` publishes current contents, furnace progress and
player-preview animation. See the client integration contract in
`crates/client/src/gui/CONTAINERS.md`.

The recipe table is extracted from the original bytecode, including the exact
sorted order and stable registration ties. Regenerate it and the GUI item-name
and rendering tables with `python tools/oracle/build_container_recipes.py`.
There are 151 recipes. `CraftingManager` preserves shaped offsets and mirrors,
shapeless matching and damage wildcards; smelting uses the original ID lookup.

The port retains Beta behavior: dispenser shift-click returns a copy without
transferring; furnace shift-click uses the inventory rows/hotbar; merge ignores
slot validity in its empty-slot pass; shift crafting recurses only for the
original smaller-source condition; container ingredients replace the complete
input stack; lava buckets are consumed without returning a bucket. Opening,
clicking, closing and invalidating a window produce stat/achievement hooks or
actual dropped entities as appropriate. Map crafting assigns the original
signed-short ID and emits `MapCreated`; map data remains an app integration.

Furnace and dispenser tiles load/save their original NBT compounds. Furnace
ticks retain metadata and contents across lit/unlit block swaps. Dispensers
select slots with their own reservoir-sampling RNG, produce the original
projectile/item entities and auxiliary effects, and scatter contents on removal.

## Verification

`bash tools/verify_containers.sh` runs original-bytecode replays and Rust comparisons:

- Every recipe is asserted to produce its registered output before pickup.
  Matrices include mirrors, offsets and wildcard damage, with normal and shift
  pickups. Random operations cover all six container layouts and mouse variants.
- 65,706 records compare returned stacks, every slot, cursor, armor, main
  inventory, crafting-stat counts and achievement hooks after each operation.
- 1,000 machine ticks compare every entity and furnace inventory/counter each
  frame, plus a complete-world fingerprint each frame and full final world bytes.
  Cases cover all smelting outputs, each fuel category, blocked output, empty
  dispensing, four directions, projectiles, item motion, fuel exhaustion and
  removal of filled machines.

`cargo test -p worldgen --test containers` checks authoritative window messages,
double-chest order, stale windows, close drops, distance invalidation, signed
machine NBT/chunk persistence and the furnace progress-read side effect.

`bash tools/verify_gui.sh` retains the existing screen suite and strictly compares
the container captures. The new scenes include all backgrounds, counts,
durability, cursor/highlight/tooltips, furnace progress, both chest sizes,
right/shift clicks, all armor texture families, held-item transforms and live
player animation. Strict comparison accepts zero differing RGB pixels.

Finished stage snapshots and `target/commit-containers.sh` preserve only the
named stage files. Run the script when Git metadata is writable; it refuses to
overwrite newer uncaptured edits and preserves unrelated staged files.
