# b1.8.1 player-gui, round 3

This round completes the stages below within the task's approximately one-hour budget. It does not claim the entire player/gui area is finished. The per-class table retains the round-two status for classes without new evidence and updates the classes checked here.

## Changes and checks

- Added real asynchronous stat receive/send workers, shared busy/mailbox state, periodic readiness/cooldowns, synchronous shutdown writes and literal wrapping stat arithmetic. The original workers and the port match 2,124 observed snapshots, including six file recovery/checksum cases, both pending maps, and write failures. Parsed disk maps are compared; JVM identity-key JSON iteration order is deliberately not described as raw-byte EXACT.
- Added original melee/mining/drop replay: 600 attacks (critical, sprint, immune/refused hits, creative, weapon durability and saddle), 2,304 haste/fatigue mining cases (tools, water/off-ground and amplifier boundaries), and 100 queued drop batches. Player/target motion, health, item state, statistics, RNG, spawned entities and world events compare byte for byte.
- Added haste, fatigue and nausea whole-player scenarios; all 23 original tick scenarios remain EXACT. Corrected the fixture's obsolete removal of the only event recorder after the entities integration.
- Preserved drop press counts across app/worker frames instead of reducing them to a boolean. Legacy single-drop inputs still work.
- Passed new-world settings into World::open_with_settings before spawn search/preload. A generation/save/reopen regression checks 289 preload progress callbacks, disabled initial structure generation, seed/mode/features in the first save, and saved-setting precedence on reopen. This is a Rust integration regression, not a new terrain-bytecode claim.
- Added 10,000 scaling cases, 10,000 button cases and 376 font outputs, all byte EXACT. Extended PlayerListEntry with 10,000 original equality/hash/string cases.
- Added thirteen achievement-toast pixel captures, all EXACT. Fixed earned-achievement text shadows and GL depth-mask/rescale state. Scene files are appended in the round-three block at the end of scenes.txt and are part of the focused verifier.
- Refreshed the original recipe-catalog audit with client-order registry initialization. The complete binary catalog matches the compiled Rust data: 161 recipes and 2,258 smelting/item metadata rows, including the new beef and chicken recipes.

## Verification

The delimited player-gui section at the end of tools/verify181.sh continues to invoke tools/verify_player_gui181.sh. The final focused verifier exits 0: whole-player (23 modes), stat synchronization (2,124 snapshots), world-settings regression, combat/mining/drop (3,805,894 bytes), PlayerList, playercore (2,539,266 bytes), creative (9,850,684 bytes), options/keybinding/stat-registry, and all 43 captures in four GUI scene groups pass. The full one-hour verify181.sh run was not attempted.

The existing inventory and container checks were rerun: 10,000 player inventory operations and 66,146 container records / 161 recipes remain EXACT (target/player-gui3-inventory-containers.log). The complete recipe/smelting data check is now included in the focused verifier. Release build passes; git diff --check and Bash syntax checks pass.

Final logs: target/player-gui3-final-build.log, target/player-gui3-final-focused.log, target/player-gui3-settings-test.log, target/player-gui3-full-gui.log. Stats input/output traces are in the fresh target/statsync181.* directory printed by the verifier. Combat fixtures are tools/oracle181/out/playercombat181-java.bin and target/playercombat181-rust.bin. GUI captures/diffs are retained in the corresponding out and target/shots directories. The full GUI pass is diagnostic; its known failures must not be reported as a passing suite.

## Per-class status

| Class | Status and evidence / remaining work |
| --- | --- |
| EntityPlayer | NOT DONE: retained tick/inventory checks; Mode_playercombat181 now checks 600 melee attacks, 2,304 potion-mining cases and 100 queued-drop batches. Fixed critical-hit conditions/damage, typed player damage, sprint knockback, refused-hit exhaustion and saddle use statistics. Full interaction/NBT audit and critical visual callback remain. |
| EntityPlayerSP | NOT DONE: 23 whole-player tick scenarios EXACT, adding haste/fatigue swing timing and nausea portal-factor behavior. Complete application replay and remaining input/controller transitions are still open. |
| FoodStats | DONE: Mode_playercore / playercore181: 72,000 boundary ticks and 10,000 randomized operations, including NBT. |
| PlayerControllerTest | NOT DONE: creative initialization (retaining occupied slots), instant removal, fire removal, cooldown and reset behavior are EXACT in creativecontroller. Placement/count preservation and all API methods need a complete controller replay. |
| PlayerController | NOT DONE: creative-aware local controller branch staged; reach and all controller API differences need full audit/oracle. |
| PlayerControllerSP | NOT DONE: inherited mining behavior retained; b181 registry refreshed. Complete b181 controller oracle remains. |
| MovementInputFromOptions | DONE: Mode_playercore: all 64 movement-key combinations, including sneak float arithmetic. |
| WorldSettings | DONE: Mode_playercore: 30 seed/mode/features combinations. |
| InventoryPlayer | NOT DONE: existing 10,000 operations EXACT; held-stack identity cancellation is now EXACT. Complete changed-method/NBT audit remains. |
| PlayerCapabilities | DONE: Mode_playercore: defaults, creative enable, flying, creative disable. |
| MovementInput | DONE: Mode_playercore: 64 movement-key combinations. |
| Minecraft | NOT DONE: drop press counts now survive app/worker batching and are consumed in order before item-use processing; 100 original repeated-drop batches check inventory, statistics, RNG and spawned entities. Attack/use checks retained. Pick while using, complete application/debug replay and controller transitions remain. |
| GuiIngame | NOT DONE: original four HUD scenes plus seven new poison/hunger/regen/air/creative scenes are pixel EXACT. Player-list overlay, debug and complete portal/overlay coverage remain. |
| GuiContainerCreative | DONE: Mode_creative click policy EXACT; first, scrolled and cursor screenshots EXACT. |
| GuiMainMenu | NOT DONE: original panorama cube jitter, eight blur passes, UV crop, gradients, version/copyright and animation are implemented. Fresh title scenes differ by 197 pixels with maximum channel difference 1; 37/437-tick scenes also have one-shade differences. Title after a loaded anaglyph world has a larger inherited renderer-state difference. |
| PlayerList | DONE: Mode_playerlist181 / playerlist181, 3,000 operations and complete bucket snapshots EXACT, including the vanilla duplicate-key insertion bug, null values, collisions, resize order, counts and modification counter. |
| Container | DONE: containerreplay: 66,146 fixture cases EXACT, including shift recursion, transfer returns, slots/cursor and stats. |
| GuiCreateWorld | NOT DONE: four layouts/toggles remain pixel EXACT. New-world settings now precede spawn search/preload and the initial save. A real generation/save/reopen regression passes, including saved-setting precedence. Seed parsing/random-seed and full callback audit remain. |
| GuiAchievements | NOT DONE: 17th achievement and inventory-key tooltip lookup ported. Terrain/icon rendering differs by 85 one-shade pixels; corrected tooltip is reduced to 44 one-shade pixels in the final original capture. |
| ContainerCreative | DONE: Mode_creative / creative181: catalog, scroll positions, 10,000 operations and complete slot/cursor snapshots EXACT. |
| KeyBinding | NOT DONE: 10,000 queue/rebinding events EXACT, and physical attack/use/pick/player-list routing migrated. Original global registry shared across multiple GameSettings instances is still absent. |
| GuiStats | NOT DONE: general and eight item/scroll/sort screens pixel EXACT; all formatter outputs now EXACT. Five block screens still have 18 one-shade item-rendering differences. |
| GuiContainer | NOT DONE: creative branch and all three creative screenshots EXACT; wheel/drag support added. Six added noncreative empty-screen scenes are EXACT. Existing filled-slot/hover/click GUI fixtures and all changed input behavior still require coverage. |
| StatsSyncher | DONE for the original receive/send/tick/readiness/file-rotation protocol: Mode_statsync181 and --statsynccheck compare 2,124 snapshots, six recovery/checksum/overflow scenarios, pending receive/acknowledgement maps, and failed synchronous/background writes. Barriers only stabilize observations; arbitrary JVM thread scheduling is not claimed. StatFileWriter JSON ordering limitations are below. |
| StatList | DONE: Mode_options181 / --playerguicheck, complete general/block/item registries and mine/craft/use/break alias arrays/order EXACT against original bytecode. |
| GuiAchievement | DONE: original source diff audited (renames and removal of the old unlicensed-copy text). Thirteen new original-client pixel captures EXACT: start/slide/full/exit/end/expiry, no revival, zero clock, new snipe achievement, information and late information. Removed incorrect text shadows; matched depth mask and retained rescale-normal state. |
| PlayerListEntry | DONE: retained 3,000 complete bucket snapshots plus 10,000 entry equality/hash/string cases EXACT, with signed long keys, equal/unequal/null values and differing stored hash/link fields. |
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
| StatFileWriter | NOT DONE: totals, unsent maps, wrapping i32 arithmetic, achievement presence/parents, sync/tick/readiness and valid/checksum-invalid/invalid-syntax file recovery match 2,124 original snapshots. Disk maps are normalized by stat id; raw JSON key order follows JVM identity hashes and is not byte-identical. Arbitrary malformed schema and all serialization entry points still need audit. |
| CraftingManager | DONE: b181 recipe table regenerated from original; all 161 recipes exercised in containerreplay, EXACT. |
| ThreadStatSyncherReceive | DONE: actual original thread executes both sent-file read and pending-map write branches; normalized disk/state snapshots, tick consumption and busy clearing match in Mode_statsync181. Uses the shared Syncher worker protocol. |
| GuiYesNo | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryBasic | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryCraftResult | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| ScaledResolution | DONE: original class diff is field renaming. Mode_guiutility181: 10,000 randomized dimensions/scales compare integer dimensions, factor and double dimensions byte for byte, including zero and sub-minimum displays. |
| Achievement | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| FontRenderer | DONE for the b181 class delta (method renames): Mode_guiutility181 compares 376 character/string/wrapping outputs, including all 256 characters, formatting, whitespace, newlines and unbroken words. Existing GUI text pixels and new multiline toast hints are EXACT. General renderer precision differences in other classes remain separately listed. |
| GuiSlotStatsGeneral | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| IInventory | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| ThreadStatSyncherSend | DONE: actual original thread writes periodic snapshots and clears busy after success and an unwritable .tmp-directory failure; Mode_statsync181 trace EXACT. |
| GuiScreen | NOT DONE: mouse wheel forwarding added for creative inventory; remaining class changes not fully audited. |
| RecipesCrafting | DONE: Original b181 recipe data regenerated; full recipe fixture replay EXACT. |
| AchievementList | DONE: Mode_options181 / --playerguicheck, all 17 achievement IDs, grid coordinates, icons, parents, special flags, translated text and grid bounds EXACT. |
| EnumOptions | DONE: Mode_options181 / --playerguicheck: all 15 option ordinals, metadata and labels. |
| FurnaceRecipes | DONE for the b181 delta and vanilla catalog: original class diff adds beef/chicken recipes; Mode_recipeaudit181 queries all 2,258 item IDs and the full recipe/smelting binary equals the port's recipes.bin. Container replay remains EXACT. Runtime mod registration is outside the vanilla catalog. |
| GuiChat | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiOptions | DONE: b181_options and b181_options_fov pixels EXACT. |
| GuiWorldSlot | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| ICrafting | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| LoadingScreenRenderer | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| RecipesFood | DONE: Original b181 recipe data regenerated; full recipe fixture replay EXACT. |
| StatTypeDistance | DONE: Mode_options181 / --playerguicheck, original formatter output EXACT for 24 boundaries and 5,000 signed random integers. |
| StatTypeSimple | DONE: Mode_options181 / --playerguicheck, original formatter output EXACT for 24 boundaries and 5,000 signed random integers. |
| StatTypeTime | DONE: Mode_options181 / --playerguicheck, original formatter output EXACT for 24 boundaries and 5,000 signed random integers, including Java scientific notation for large negative seconds. |
| GuiButton | DONE: original class diff audited. Mode_guiutility181: 10,000 visible/enabled/bounds cases compare press and hover state; existing options/controls/container pixels and all new toast captures remain EXACT. |
| GuiChest | DONE: renamed inventory interface is represented by the existing container model; b181_chest and b181_largechest pixels EXACT. |
| GuiCrafting | DONE: renamed inventory interface is represented by the existing container model; b181_crafting pixels EXACT and crafting replay EXACT. |
| IStatType | DONE: all three formatter implementations checked through the original interface; 15,072 outputs EXACT. |
| RecipesDyes | DONE: Original b181 recipe data regenerated; full recipe fixture replay EXACT. |
| ShapedRecipes | DONE: Full shaped recipe fixture replay against original b181 EXACT. |
| ShapelessRecipes | DONE: Full shapeless recipe fixture replay against original b181 EXACT. |
| Slot | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |

## Remaining work and ownership

The diagnostic full GUI run still differs on fresh title scenes (197 pixels, maximum 1), achievements (85 pixels; open tooltip 44, maximum 1), and five block-stat scenes (18 pixels, maximum 1). The three combined old/focused full-HUD fixtures differ by 116,109 or 116,534 pixels, maximum 2; their isolated focused versions pass. Combined inventory has the existing 364-pixel label difference. Filled/turned/equipped inventory captures differ by 1,996–5,009 pixels with maximum up to 255. The title after an anaglyph world differs by 296,833 pixels, maximum 67. All thirteen new toasts also pass within the combined full GUI run. Chest/crafting/dispenser/furnace interaction and all nine sign captures pass. Chat, general/item statistics and select-world captures pass. The 16,777,216-color, 1,000 compass/watch-frame, 400 terrain-frame and 80 GPU color-mask checks pass. This is a diagnostic failing suite, not an EXACT result for the renderer.

Complete chest open/close hooks need world-blocks integration. EntityPlayer/EntityPlayerSP/controller, physical pick/main-loop/debug, global KeyBinding registry, arbitrary stat-file JSON schema/formatting and the other NOT DONE class audits remain.

Outside-area changes in this round are confined to the existing worldgen181/scenario.rs constructor entry point so the player/gui worker can supply settings at the correct time; no terrain algorithms, entity classes/renderers, particles, world renderer or lightmap were rewritten. Melee sets the existing target.velocity_changed field locally rather than changing the entities owner's generic add_velocity implementation. The dropped-entity replay reuses the entities owner's serializer through a helper in the owned player verification module.

The forbidden b173 crates, oracle and verification scripts are unchanged. No git write was performed by this worker. A worktree snapshot commit appeared during the session; the final explicit staging script includes this round's full file set relative to the merged round-two commit, plus later files. target/commit-b181-player-gui3.sh uses the requested author and both coauthor trailers.
