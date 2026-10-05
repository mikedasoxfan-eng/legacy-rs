# b1.8.1 player-gui staged port report

Round 2 supersedes this stage's outstanding-work list and per-class status: see [b181-player-gui2.md](b181-player-gui2.md). This document preserves the round-one evidence.

This is a staged port within the task's approximately 75-minute worker budget. It does not claim that the entire player-gui area is finished. DONE below means the implemented class changes have an original-bytecode check ending EXACT. NOT DONE includes partially implemented classes and classes not handled in this stage.

## Verified results

- `cargo build --release -p worldgen181 -p client` succeeds.
- `bash tools/verify_player_gui181.sh` exits 0. Its playercore comparison is EXACT (2,539,266 bytes), creative comparison is EXACT (9,850,684 bytes), options/key-binding comparison is EXACT (10,000 events), and all 23 added GUI screenshots are pixel EXACT.
- `containerreplay`: 66,146 cases EXACT, covering 161 b181 recipes, container shift/click return values, complete contents/cursor, statistics and achievements emitted by those operations.
- Existing inventory oracle: 10,000 operations EXACT.
- No full `verify181.sh` run was attempted; the integrator owns that approximately one-hour run. No existing whole-player simulation is claimed EXACT: the attempted movement harness fails in Java world serialization (`NoSuchFieldException: lightingToUpdate`). That shared harness must be migrated before using it to diagnose player ticks.

The new reachable, delimited block at the end of `tools/verify181.sh` invokes `tools/verify_player_gui181.sh`. The 23 scenes are appended in a delimited block in `tools/oracle181/gui/scenes.txt` and also available independently in `player-gui-scenes.txt` and `player-gui-container-scenes.txt`. Existing `verify_containers181.sh` was corrected to use `container_compare181`; its replay portion was made EXACT. A fresh full container verifier run again passed replay, then stopped in the Java tile harness with `NoSuchFieldException: lightingToUpdate` before any tile comparison. See `target/player-gui-containers-final.log`. `verify_gui181.sh` now launches the b181 Rust client explicitly. This does not certify all its existing scenes.

Six additional empty inventory/chest/large-chest/crafting/furnace/dispenser screenshots are pixel EXACT (`target/player-gui-extra-diff`); these are now part of the 23 focused scenes. The two fixture groups run in fresh Java processes. A combined run that restarts the world after the controls scenes exposes a 364-pixel inventory label color difference (original black, Rust gray). This transition remains unresolved and must be checked during renderer integration; the isolated inventory fixture is EXACT. All other combined-run scenes were EXACT.

Final logs: `target/player-gui-final.log` (complete focused script), `target/player-gui-core-final.log` (checks after the final creative fire/miss/reset corrections). Original/Rust captures: `tools/oracle181/out/player-gui-java` and `target/player-gui-rust`; pixel diagnostics: `target/player-gui-diff`. Per-class source diffs are in `target/player-gui-class-diffs` (86 classes; newly named classes have an empty old side).

## Per-class status

| Class | Status and evidence / remaining work |
| --- | --- |
| EntityPlayer | NOT DONE: food/XP arithmetic and FOV checks EXACT; eating, exhaustion, blocking, sprint/fly, NBT additions staged. Full damage sources, potions, XP-orb pickup and whole-player tick verification remain. |
| EntityPlayerSP | NOT DONE: movement and FOV subchecks EXACT; double-tap sprint/flight and FOV smoothing staged. Whole-player simulation and potion behavior remain unverified. |
| FoodStats | DONE: Mode_playercore / playercore181: 72,000 boundary ticks and 10,000 randomized operations, including NBT. |
| PlayerControllerTest | NOT DONE: capability transitions EXACT; instant break, five-tick cooldown and creative placement preservation staged. Starting hotbar population and complete controller oracle remain. |
| PlayerController | NOT DONE: creative-aware local controller branch staged; reach and all controller API differences need full audit/oracle. |
| PlayerControllerSP | NOT DONE: inherited mining behavior retained; b181 registry refreshed. Complete b181 controller oracle remains. |
| MovementInputFromOptions | DONE: Mode_playercore: all 64 movement-key combinations, including sneak float arithmetic. |
| WorldSettings | DONE: Mode_playercore: 30 seed/mode/features combinations. |
| InventoryPlayer | NOT DONE: 10,000 inventory operations EXACT; new item-presence behavior used by bow. Full changed-method/NBT audit remains. |
| PlayerCapabilities | DONE: Mode_playercore: defaults, creative enable, flying, creative disable. |
| MovementInput | DONE: Mode_playercore: 64 movement-key combinations. |
| Minecraft | NOT DONE: b181 launch, settings actions, creative inventory and world settings flow staged. Key queues are not yet wired through the full main loop; attack/use rebinding, creative reach, miss timing and debug handling need integration. |
| GuiIngame | NOT DONE: all four new hunger/XP/armor/creative HUD scenes EXACT. Player-list rendering, potion-linked state, full debug/overlay behavior and underwater/portal fixture coverage remain. |
| GuiContainerCreative | DONE: Mode_creative click policy EXACT; first, scrolled and cursor screenshots EXACT. |
| GuiMainMenu | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| PlayerList | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| Container | DONE: containerreplay: 66,146 fixture cases EXACT, including shift recursion, transfer returns, slots/cursor and stats. |
| GuiCreateWorld | NOT DONE: all four new layouts and toggle states EXACT; mode/features persisted. Actual world creation/seed edge cases require oracle checks and initial generation must receive map-features before preload. |
| GuiAchievements | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| ContainerCreative | DONE: Mode_creative / creative181: catalog, scroll positions, 10,000 operations and complete slot/cursor snapshots EXACT. |
| KeyBinding | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiStats | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiContainer | NOT DONE: creative branch and all three creative screenshots EXACT; wheel/drag support added. Six added noncreative empty-screen scenes are EXACT. Existing filled-slot/hover/click GUI fixtures and all changed input behavior still require coverage. |
| StatsSyncher | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| StatList | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiAchievement | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| PlayerListEntry | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| ContainerDispenser | DONE: containerreplay: dispenser transfer rules and click results EXACT. |
| RenderEngine | NOT DONE: b181 GUI item properties/textures staged locally; gamma/lightmap application and general texture-loading changes belong to further integration. |
| GameSettings | DONE: Mode_options181 labels/default bindings and 10,000 rebinding/queue events EXACT; fov/gamma load/save additions ported. |
| GuiControls | DONE: All three controls scenes EXACT, including mouse rebinding and duplicate bindings; Mode_options181 event trace. |
| ContainerFurnace | DONE: containerreplay: furnace transfer rules and click results EXACT. |
| ContainerWorkbench | DONE: containerreplay: crafting/transfer rules and click results EXACT. |
| ContainerChest | NOT DONE: chest transfer/click rules EXACT in containerreplay. Chest open/close hooks need world-blocks integration. |
| StatBase | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiParticle | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiSelectWorld | NOT DONE: loading game mode/features connected; remaining b181 screen differences not pixel-checked. |
| GuiSlotStats | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryLargeChest | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiRenameWorld | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| SorterStatsItem | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiInventory | NOT DONE: creative inventory selection added. Survival inventory pixels are EXACT in b181_inventory. Remaining creative redirect/update and input behavior require complete audit. |
| SorterStatsBlock | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| ContainerPlayer | DONE: containerreplay: player crafting/transfer rules and click results EXACT. |
| GuiSlot | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiSlotStatsBlock | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiVideoSettings | DONE: b181_video_gamma pixels EXACT; gamma option labels checked by Mode_options181. |
| GuiSlotStatsItem | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryCrafting | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiTexturePackSlot | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiSavingLevelString | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiTexturePacks | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| StatFileWriter | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| CraftingManager | DONE: b181 recipe table regenerated from original; all 161 recipes exercised in containerreplay, EXACT. |
| ThreadStatSyncherReceive | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiYesNo | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryBasic | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryCraftResult | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| ScaledResolution | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| Achievement | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| FontRenderer | NOT DONE: controls color formatting covered by pixels; remaining b181 changes not separately audited. |
| GuiSlotStatsGeneral | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| IInventory | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| ThreadStatSyncherSend | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiScreen | NOT DONE: mouse wheel forwarding added for creative inventory; remaining class changes not fully audited. |
| RecipesCrafting | DONE: Original b181 recipe data regenerated; full recipe fixture replay EXACT. |
| AchievementList | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| EnumOptions | DONE: Mode_options181 / --playerguicheck: all 15 option ordinals, metadata and labels. |
| FurnaceRecipes | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiChat | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiOptions | DONE: b181_options and b181_options_fov pixels EXACT. |
| GuiWorldSlot | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| ICrafting | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| LoadingScreenRenderer | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| RecipesFood | DONE: Original b181 recipe data regenerated; full recipe fixture replay EXACT. |
| StatTypeDistance | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| StatTypeSimple | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| StatTypeTime | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiButton | NOT DONE: placeholder formatting used by mouse-button labels fixed; general b181 class audit/check remains. |
| GuiChest | DONE: renamed inventory interface is represented by the existing container model; b181_chest and b181_largechest pixels EXACT. |
| GuiCrafting | DONE: renamed inventory interface is represented by the existing container model; b181_crafting pixels EXACT and crafting replay EXACT. |
| IStatType | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| RecipesDyes | DONE: Original b181 recipe data regenerated; full recipe fixture replay EXACT. |
| ShapedRecipes | DONE: Full shaped recipe fixture replay against original b181 EXACT. |
| ShapelessRecipes | DONE: Full shapeless recipe fixture replay against original b181 EXACT. |
| Slot | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |

## Integration and remaining work

The highest-priority next step is an original-bytecode whole-player tick oracle that serializes the new food, XP, use-item, sprint/flight and capabilities fields without depending on the outdated world serializer. FoodStats' starvation action is checked through an isolated original player callback; complete hurt immunity, armor bypass and exhaustion need DamageSource integration. The local starvation adapter bypasses armor and uses zero damage exhaustion but does not replace typed damage sources.

Eating now has 32-tick duration, slot cancellation, particles, hunger/saturation and bowl conversion. Potion effects from golden apples, raw chicken and rotten flesh are missing. Item use compares copied stack values/slot rather than Java object identity. Bow release uses b181 charge, initial speed, threshold, sound and arrow consumption, but full-charge critical arrow state/damage/particles need the entities worker's hook. XP maths and NBT are implemented; XP-orb pickup is not.

Sprint and flight mechanics are staged, but their complete tick sequences are not claimed EXACT. The input data model reproduces b181 key queues, yet the application's existing physical input handling still needs migration to that model, particularly attack/use/pick/player-list mouse or keyboard rebinding. The original global static KeyBinding registry across multiple GameSettings instances is not reproduced. Creative fire extinguishing now also removes the clicked block as the original does; creative reset is a no-op and missing-hit delay is suppressed, but these last controller corrections have no dedicated original-controller trace yet.

New create-world controls and saved mode/features work through local WorldSettings/LevelInfo additions. Terrain generation currently opens/preloads the world before applying the requested map-features flag; pass settings to initial construction when terrain changes are merged. Brightness settings are stored and displayed; world lightmap brightness must be applied by the renderer integrator.

The title panorama, PlayerList/PlayerListEntry, new achievements and stat synchronizer changes remain unported. Container chest inventory open/close hooks require the new chest block/entity integration. Existing achievement/stats/title and filled/interactive noncreative container GUI scenes still need their b181 checks; six new empty-screen cases pass. No general world-renderer or new entity renderer port is claimed here.

## Minimal dependencies outside the area

- `worldgen181/src/entity/living.rs`: player-specific jump exhaustion/sprint impulse and flying/ground/air movement speeds.
- `worldgen181/src/entity/projectile.rs`: speed-parameterized arrow constructor; the old constructor delegates with its old speed.
- `worldgen181/src/storage.rs`: game mode/map-features LevelInfo persistence.
- `client181/src/render/camera.rs` and `render/player_view.rs`: player's b181 FOV factor, smoothing and FOV option input.
- `client181/src/render/entity/living.rs`: b181 signed-byte normal packing correction needed by GUI icons.
- `client181/src/render/entity/inventory.rs`, `fixed.rs`, `item_table.rs`, and new `gui/screens/block_items.rs`/`chest_item.rs`: local b181 GUI item properties, closed chest icon, fence-gate icon geometry and icon culling/lighting. The world renderer's block property table was left alone.
- `tools/oracle181/src/net/minecraft/src/ContainerOracle.java` and `gui/GuiShot.java`: attach a world to the original chest fixtures for b181 open hooks. New checks use separate Mode_ classes and do not change Oracle dispatch.

The forbidden b173 crates, oracle and verification scripts were not edited. No git write was executed by this worker. `target/commit-b181-player-gui.sh` stages an explicit file list and commits with the identity/trailers from the task. Automatic worker snapshots exist in this branch; the script includes this worker's files across those snapshots as well as the final unstaged files.
