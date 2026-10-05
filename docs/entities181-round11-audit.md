# Round 11: ItemBlock and ItemStack source audit

The complete original Beta 1.7.3 and Beta 1.8.1 classes were compared. Raw diffs are `target/entities11-ItemBlock.diff` and `target/entities11-ItemStack.diff`. Only these two classes are counted in this round. Previous Item, EntityList and SpawnerAnimals work is not reopened.

## ItemBlock

The version diff adds the stored block-ID accessor, player edit permission, vine replacement, subclass placement metadata and the post-set block-ID guard. The round-10 literal class dispatcher already implements every one of those changes. Round 11 finishes its concrete placement dependencies.

| Original method or dependency | Rust implementation | Original-bytecode coverage |
|---|---|---|
| Constructor, registration before failed texture getter, stored ID, both names | `entity/item_block.rs::BlockItem` | Retained `itemblock10` constructor/name probes |
| Complete `onItemUse`, null/error order, wrapping offsets/count, dynamic block lookup, retained sound object | `BlockItem::on_item_use`, borrowed `LiveBlockItemContext` | Retained 92,160-call `itemblock10`; new `blockfull11` |
| Every registered non-moving block item, all faces/heights, replacement and entity collision | Existing `Entity::place_block_item`, placement predicates and world callbacks | `blockfull11`: 108 registered IDs, 31,104 real placements; Rust independently derives the input registry, then computes all results |
| Fire/crop/fence placement predicates | Three local branches in existing `player/placement.rs::can_place_at`; existing fire helper reused | `blockfull11`; previously differing IDs 51/59/85 now match |
| Moving candidate collision before replacement/placement rejection; missing/wrong/invalid tile | `World::placement_collision_box181`, checked live ItemBlock adapter | `itempiston11`: 19,600 original ItemBlock/World calls |
| Missing-tile Chunk byte-array indexing without a height guard; unknown carried block; invalid direction after non-null collision | Same local checked collision adapter | `itempiston11`: -1/0/64/127/128/256/32768 heights, two local X positions, five sides, seven carried IDs, ten directions, four tile states; exact exceptions, retained tile map/invalidation, RNG and events |

The moving-piston adapter uses the existing Piston/TileEntity representation. It checks the native Chunk array access, distinguishes null/wrong/invalid tiles, and returns typed dependency errors. It does not rewrite world tile lifecycle, piston ticking, rendering or storage. The boolean live adapter retains its existing panic-on-error convention. Piston storage still belongs to world-blocks; no claim is made to audit that separate class's entire malformed-NBT space.

## ItemStack

The version diff replaces the public NBT constructor with a private zero-default constructor plus nullable static load; adds eaten/use-duration/use-action/stopped-use delegation; and renames stat arrays and the empty destroyed-use callback. Every unchanged method is also audited.

The port extends the existing `player::ItemStack` through `entity/item_stack.rs`. Its fields, Copy borrow representation, identity counter, inventory publication and controller policy remain owned by the existing player implementation. No second object/alias store is introduced. All virtual callbacks borrow the same mutable stack; returned snapshots use that representation's existing identity token.

| Original methods | Rust implementation | Original-bytecode coverage |
|---|---|---|
| Int/Block/Item constructor overloads and nullable arguments | Existing `ItemStack::new`, `from_block181`, `from_item181` | `stackdata11` |
| Private zero-default construction and static nullable load | `load_checked181`, existing `try_from_nbt181` delegates to it | `stackdata11`, retained EntityList/inventory/NBT proofs |
| `readFromNBT`: sequential assignments, signed byte/short, absent fields, wrong types, exceptions after earlier mutation; animation preserved | `read_nbt_checked181`, `read_nbt_with181`; existing `from_nbt` delegates | `stackdata11`, `stacknbt11` |
| `writeToNBT`: signed narrowing, key overwrite, unrelated-key preservation, same compound return; virtual getter/setter mutation and failure | `write_nbt181`, `write_nbt_with181`; existing `to_nbt` delegates | `stackdata11`, `stacknbt11` |
| `splitStack`, `copy`, nullable `copyItemStack` | Existing split with explicit wrapping; `copy_stack181`, `copy_nullable181` allocate through the existing constructor and reset animations | `stackdata11`: 625 signed scalar combinations; retained input identity, distinct result identity and copy animation reset |
| Static full equality, private full equality, `isStackEqual`, `isItemEqual` | `stacks_equal181`, `stack_equal181`, `item_equal181` | `stackdata11`; animation and identity excluded as in Java |
| Display/current damage getters and damage setter | Existing `damage` field (full signed int) | `stackdata11` signed mutation/getter observations |
| Item lookup, max stack/max damage, damageability/subtypes/damaged/stackability | `get_item181`, checked registered getters, literal `dispatch181` branches | `itemstack11`, `stackdata11`; 32,000-slot bounds and null errors, virtual mutation, short circuit/repeated lookup and subtype independence |
| `damageItem`: wrapping addition/decrement, threshold, stat before count/reset, non-player/null entities | `dispatch181(Damage)`; existing production damage helper retains its caller-owned stat adapter | `itemstack11`, retained combat/inventory/use checks |
| Icon, strength, right click, eaten, entity hit, destroyed block, damage vs entity, harvesting, saddle | Literal dispatch through borrowed `StackContext` | `itemstack11`: same stack alias and nullable resources, dynamic item replacement, signed arguments, true/false returns, same/new/other/null stack returns, exceptions |
| Stack name and `toString` evaluation order | `dispatch181(Name/DisplayString)` captures count before virtual name, appends current damage afterwards | `itemstack11`: null names and aliased callback mutation |
| Animation update, crafting, use duration/action/stopped use, empty destroyed-use callback | Literal dispatch (animation decrement before lookup, crafting stat before lookup/callback) | `itemstack11`: missing item/player/world/target, nullable enum, signed durations/slots, injected item/stat failures |
| `useItem`, entity-hit and destroyed-block use stats | Reload mutated stack ID after virtual callback; stat sink receives current ID | `itemstack11`; live block use now calls `ItemStack::use_item181` and the existing item/stat APIs |

`itemstack11` executes 68,640 original method calls with mutable retained aliases, null registry entries/resources, int overflow, replacement item handles and item/stat exceptions. `stacknbt11` adds 280 original virtual-compound calls, preserving mutations before throws and recording each setter value after signed narrowing. `stackdata11` exercises 200 constructors, 625 scalar/copy cases, 5,265 read/load/write cases, null compounds, 30 wrong-type tags and 192 registered scalar queries.

The new fixtures are separate Mode classes. No shared Java recorder, original Java, Beta 1.7.3 file, item registry snapshot, player identity/inventory/container/GUI system, world-block implementation, terrain or integrator render code is modified. The only existing player-facing changes are ItemStack scalar/NBT delegation, explicit wrapping, one block-use delegation and the local placement predicates/checked adapter.
