# b1.8.1 player-gui, round 2

Continued the interrupted run's uncommitted changes in place. This is a staged handoff under the task's time budget, not a claim that the entire area is finished. DONE requires an EXACT original-bytecode check; NOT DONE includes partially implemented classes. Round-one results are retained where rerun or explicitly identified below.

## Changes and exact checks

- Added a whole-EntityPlayerSP tick oracle independent of the outdated world serializer. Twenty modes compare every tick's position/motion/living state, player inventory/stat events, hunger, XP, use-item, sprint/flight/capabilities, arm interpolation, spawned-arrow launch state, controller delay, blocks and world events. All are byte-for-byte EXACT: idle, movement, sprint, flight, creativecontroller, starve, regen, eat260/282/322/365/367, cancel, identity, bow, xp, damage, creativedamage, creativevoid and speed. Movement/sprint use 240 ticks; the other modes use 500. Longer forward traces leave the inherited Java fixture and request missing chunks.
- Ported golden-apple/raw-chicken/rotten-flesh potion effects, held-stack object identity, full-charge critical arrow flag, player damage-source routing, jump exhaustion, XP cooldown ordering and arm interpolation. XP-orb pickup matches the original tick replay. Critical arrow impact damage/particles remain the entities area's responsibility.
- Creative startup/respawn now fills only empty hotbar slots with the original nine blocks. Original-controller replay verifies occupied-slot preservation, block removal, fire extinguishing/removal, five-tick cooldown and no-op reset.
- Physical input uses the b181 binding queues for attack/use/pick/player-list, including mouse or keyboard rebindings, and passes attack/use counts through the worker. Complete application tick replay is still needed.
- Added the original PlayerList long-key table with its duplicate-key insertion bug. Complete bucket snapshots across 3,000 operations are EXACT.
- Refreshed the stat registry and added snipeSkeleton. Complete stat/achievement metadata and 15,072 formatter outputs are EXACT. Fixed Java scientific notation in the time formatter and the achievement tooltip's inventory-binding lookup.
- Added the title panorama and dynamic framebuffer texture path. It is visually close but not pixel EXACT; precision and inherited renderer state remain below.
- Added seven potion/regen/underwater-air HUD captures. Every new capture and all 23 focused round-one GUI captures are pixel EXACT. Water is installed at the original player's eye cell by an opt-in fixture; counters are pinned before captures to avoid unrelated Java updateTick drift.
- Existing playercore remains EXACT (2,539,266 bytes), creative remains EXACT (9,850,684 bytes), 10,000 key events remain EXACT, containerreplay remains EXACT (66,146 records / 161 recipes), and player inventory remains EXACT (10,000 operations).
- Release build succeeds. `git diff --check` has no whitespace errors. No full one-hour verify181 run was attempted.

The existing delimited player-gui block at the end of verify181.sh invokes the focused verifier; it now includes the new tick/list checks and HUD scenes. New GUI scenes are appended in a separate delimited block at the end of scenes.txt. Reproduce with `bash tools/verify_player_gui181.sh`; use `bash tools/verify_playertick181.sh` for only the new whole-player traces. The focused verifier exits 0 (`target/player-gui2-final-traced.log`); every byte/pixel comparison passes.

Logs: `target/player-gui2-final-traced.log`, `target/player-gui2-inventory-containers.log`, `target/player-gui2-build.log`, `target/player-gui2-full-gui-final.log`, `target/player-gui2-achievements-final.log`. Captures are in tools/oracle181/out/player-gui-java and target/player-gui-rust. The full GUI log is diagnostic and contains known failures; it must not be treated as a passing suite.

## Per-class status

| Class | Status and evidence / remaining work |
| --- | --- |
| EntityPlayer | NOT DONE: whole-player tick modes now EXACT for food, exhaustion, eating/potions/identity, XP pickup, armor/blocking, typed starvation/cactus/void damage and creative immunity. Additional interactions/NBT/method audit and full critical-arrow impact behavior remain. |
| EntityPlayerSP | NOT DONE: 20 original whole-player scenarios are EXACT, including double-tap sprint/flight, movement, arm interpolation and potion speed. Longer movement is limited by the seven-chunk Java fixture; full application/main-loop replay remains. |
| FoodStats | DONE: Mode_playercore / playercore181: 72,000 boundary ticks and 10,000 randomized operations, including NBT. |
| PlayerControllerTest | NOT DONE: creative initialization (retaining occupied slots), instant removal, fire removal, cooldown and reset behavior are EXACT in creativecontroller. Placement/count preservation and all API methods need a complete controller replay. |
| PlayerController | NOT DONE: creative-aware local controller branch staged; reach and all controller API differences need full audit/oracle. |
| PlayerControllerSP | NOT DONE: inherited mining behavior retained; b181 registry refreshed. Complete b181 controller oracle remains. |
| MovementInputFromOptions | DONE: Mode_playercore: all 64 movement-key combinations, including sneak float arithmetic. |
| WorldSettings | DONE: Mode_playercore: 30 seed/mode/features combinations. |
| InventoryPlayer | NOT DONE: existing 10,000 operations EXACT; held-stack identity cancellation is now EXACT. Complete changed-method/NBT audit remains. |
| PlayerCapabilities | DONE: Mode_playercore: defaults, creative enable, flying, creative disable. |
| MovementInput | DONE: Mode_playercore: 64 movement-key combinations. |
| Minecraft | NOT DONE: attack/use queues and mouse/keyboard rebinding now feed the worker; releasing use retains the original if/else queue ordering. Complete main-loop replay, repeated drop/pick handling, debug and all controller transitions remain. |
| GuiIngame | NOT DONE: original four HUD scenes plus seven new poison/hunger/regen/air/creative scenes are pixel EXACT. Player-list overlay, debug and complete portal/overlay coverage remain. |
| GuiContainerCreative | DONE: Mode_creative click policy EXACT; first, scrolled and cursor screenshots EXACT. |
| GuiMainMenu | NOT DONE: original panorama cube jitter, eight blur passes, UV crop, gradients, version/copyright and animation are implemented. Fresh title scenes differ by 197 pixels with maximum channel difference 1; 37/437-tick scenes also have one-shade differences. Title after a loaded anaglyph world has a larger inherited renderer-state difference. |
| PlayerList | DONE: Mode_playerlist181 / playerlist181, 3,000 operations and complete bucket snapshots EXACT, including the vanilla duplicate-key insertion bug, null values, collisions, resize order, counts and modification counter. |
| Container | DONE: containerreplay: 66,146 fixture cases EXACT, including shift recursion, transfer returns, slots/cursor and stats. |
| GuiCreateWorld | NOT DONE: all four new layouts and toggle states EXACT; mode/features persisted. Actual world creation/seed edge cases require oracle checks and initial generation must receive map-features before preload. |
| GuiAchievements | NOT DONE: 17th achievement and inventory-key tooltip lookup ported. Terrain/icon rendering differs by 85 one-shade pixels; corrected tooltip is reduced to 44 one-shade pixels in the final original capture. |
| ContainerCreative | DONE: Mode_creative / creative181: catalog, scroll positions, 10,000 operations and complete slot/cursor snapshots EXACT. |
| KeyBinding | NOT DONE: 10,000 queue/rebinding events EXACT, and physical attack/use/pick/player-list routing migrated. Original global registry shared across multiple GameSettings instances is still absent. |
| GuiStats | NOT DONE: general and eight item/scroll/sort screens pixel EXACT; all formatter outputs now EXACT. Five block screens still have 18 one-shade item-rendering differences. |
| GuiContainer | NOT DONE: creative branch and all three creative screenshots EXACT; wheel/drag support added. Six added noncreative empty-screen scenes are EXACT. Existing filled-slot/hover/click GUI fixtures and all changed input behavior still require coverage. |
| StatsSyncher | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| StatList | DONE: Mode_options181 / --playerguicheck, complete general/block/item registries and mine/craft/use/break alias arrays/order EXACT against original bytecode. |
| GuiAchievement | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| PlayerListEntry | NOT DONE: original entry key/value/hash and bucket-link order match in 3,000 table snapshots. Equality and toString are implemented but lack separate original-bytecode replay coverage. |
| ContainerDispenser | DONE: containerreplay: dispenser transfer rules and click results EXACT. |
| RenderEngine | NOT DONE: dynamic panorama textures, framebuffer copies and color write masks added locally; all existing texture animation and GPU mask checks remain EXACT. Title one-shade precision and inherited second-texture/fog state after a live world need renderer integration. |
| GameSettings | DONE: Mode_options181 labels/default bindings and 10,000 rebinding/queue events EXACT; fov/gamma load/save additions ported. |
| GuiControls | DONE: All three controls scenes EXACT, including mouse rebinding and duplicate bindings; Mode_options181 event trace. |
| ContainerFurnace | DONE: containerreplay: furnace transfer rules and click results EXACT. |
| ContainerWorkbench | DONE: containerreplay: crafting/transfer rules and click results EXACT. |
| ContainerChest | NOT DONE: chest transfer/click rules EXACT in containerreplay. Chest open/close hooks need world-blocks integration. |
| StatBase | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiParticle | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiSelectWorld | NOT DONE: b181 saved GameType labels added; all four select-world screenshots pixel EXACT in the full GUI run. Full callbacks/method audit still remains. |
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
| AchievementList | DONE: Mode_options181 / --playerguicheck, all 17 achievement IDs, grid coordinates, icons, parents, special flags, translated text and grid bounds EXACT. |
| EnumOptions | DONE: Mode_options181 / --playerguicheck: all 15 option ordinals, metadata and labels. |
| FurnaceRecipes | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiChat | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiOptions | DONE: b181_options and b181_options_fov pixels EXACT. |
| GuiWorldSlot | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| ICrafting | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| LoadingScreenRenderer | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| RecipesFood | DONE: Original b181 recipe data regenerated; full recipe fixture replay EXACT. |
| StatTypeDistance | DONE: Mode_options181 / --playerguicheck, original formatter output EXACT for 24 boundaries and 5,000 signed random integers. |
| StatTypeSimple | DONE: Mode_options181 / --playerguicheck, original formatter output EXACT for 24 boundaries and 5,000 signed random integers. |
| StatTypeTime | DONE: Mode_options181 / --playerguicheck, original formatter output EXACT for 24 boundaries and 5,000 signed random integers, including Java scientific notation for large negative seconds. |
| GuiButton | NOT DONE: placeholder formatting used by mouse-button labels fixed; general b181 class audit/check remains. |
| GuiChest | DONE: renamed inventory interface is represented by the existing container model; b181_chest and b181_largechest pixels EXACT. |
| GuiCrafting | DONE: renamed inventory interface is represented by the existing container model; b181_crafting pixels EXACT and crafting replay EXACT. |
| IStatType | DONE: all three formatter implementations checked through the original interface; 15,072 outputs EXACT. |
| RecipesDyes | DONE: Original b181 recipe data regenerated; full recipe fixture replay EXACT. |
| ShapedRecipes | DONE: Full shaped recipe fixture replay against original b181 EXACT. |
| ShapelessRecipes | DONE: Full shapeless recipe fixture replay against original b181 EXACT. |
| Slot | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |

## Remaining work and integration

The full GUI run still differs on the title (197 one-shade pixels in fresh scenes), achievement terrain/icons (85 one-shade pixels; 44 on the corrected open tooltip), and block-stat icons (18 one-shade pixels). General/item statistics and select-world scenes are EXACT. Filled survival-inventory scenes show inherited second-texture/lightmap state after the 3D player preview; a combined world-restart fixture also retains the existing 364-pixel inventory-label black/gray difference. The title after loading an anaglyph world shows a larger inherited renderer-state difference (296,833 pixels, max 67). Texture animation (16,777,216 RGB colors, 1,000 compass/watch and 400 terrain frames) and 80 GPU color-mask cases remain EXACT. Chest/crafting/dispenser/furnace interaction screenshots, all nine sign screenshots and chat/texture-pack screens pass, except the listed title cases. Some old full-suite HUD fixtures supply brightness 1.0 on the first new-world frame when Java brightness is 0.0; isolated focused fixtures use the actual original values and pass.

StatsSyncher and its receive/send threads, full stat-file synchronization, and other NOT DONE class audits remain. Complete container chest open/close hooks need the world-blocks integration. Initial map-feature settings still need to enter terrain construction before preload. Brightness/lightmap and general renderer integration remain with the integrator. The old shared player/tile world serializer still reads the removed lightingToUpdate field; the new tick oracle avoids that serializer rather than altering it.

Minimal outside-area edits are confined to worldgen181 entity/living.rs (player jump exhaustion), entity/physics.rs (flying fall immunity), entity/mod.rs (typed damage callback) and entity/verify181.rs (state serializer visibility for the new local replay). Existing entity damage/potion/XP/projectile systems are reused. No general world-renderer or entity-renderer port is claimed. GuiShot's optional water fixture is opt-in; existing scenes are unaffected.

The forbidden b173 crates, oracle and verification scripts were not edited. No git write was performed. `target/commit-b181-player-gui2.sh` stages an explicit list of this round's files and uses the task's author and coauthor trailers.
