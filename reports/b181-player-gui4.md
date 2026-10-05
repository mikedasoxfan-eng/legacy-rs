# b1.8.1 player-gui, round 4

This round advances the merged round-three work within the task's approximately one-hour budget. The entire player/gui area is **not finished**. Previously DONE classes retain their evidence; new class completions are KeyBinding, ContainerChest, SorterStatsItem and SorterStatsBlock. The table below keeps every class visible, including unfinished audits.

## Changes

- Added original-bytecode controller replay: 2,400 controller transitions, 800 actual Minecraft held-button callbacks, 192 actual middle-click cases and 136 original getMouseOver cases. The complete fixture is 2,737,721 bytes EXACT. Fixed vines step sound, shears use statistics/tallgrass harvesting and default harvest hunger exhaustion. Creative removal calls the existing silverfish hook locally.
- Corrected creative block reach (5) and entity reach (32), retaining survival block reach (4) and entity reach (3). Moved pick actions into the tick action queues and retained press counts while using an item. The 800 held-input replay frames now also check original KeyBinding queued picks, item-use release, selected slots and pending counts. Its dispatcher follows the original runTick source branch while invoking original key/controller/click handlers; the complete original runTick is not executed. Held mining continues during item use and triggers the vanilla swing.
- Implemented the original global KeyBinding registry with shared binding identity, strong retention of old bindings, stale mappings until explicit rebuild, construction order and wrapping press counters. The original registry matches 20,000 events across four GameSettings and eight standalone bindings, plus overflow boundaries.
- Verified already-merged chest open/close integration through the real original BlockChest/ContainerChest path. No chest/world owner implementation was changed. Five single/double layouts, replacement/close, slot limits, lid/sound/RNG state match 1,200 ticks (783,025 bytes).
- Fixed statistics comparator signed overflow and extracted the comparator used by production sorting. Original item/block comparators match 20,000 randomized comparisons, including missing entries and ID ties.
- Matched Java Long.parseLong and UTF-16 String.hashCode seed policy, including signs, all Java 8 BMP decimal digits, overflow and zero/empty random policy. The original GUI callback is executed for 10,096 records. Rust checks its production parser and seed/settings policy; it does not execute the live GUI callback. Random seed values and JVM seed uniquifier/clock sequencing are not claimed exact.
- Fixed achievements closing on the configured inventory binding. Added explicit screen-state assertions and four new captures in two isolated groups for achievements/creative close and signed seed layouts. The original screenshot wrapper now clears its captured inner screen when vanilla closes it, so screen-state assertions observe the actual callback.
- Ported the source's F3+F render-distance cycling (Shift reverses it) and removed obsolete F8 smooth-camera toggling. Full original application/debug event replay remains open.
- Restored the haste/fatigue/nausea potion setup missing from the inherited Mode_playertick181 Java fixture; these scenario inputs already existed on the Rust side. The original bytecode is still responsible for potion behavior.

## Verification

Final verification results are recorded below after the last run. The focused verifier is still invoked from the existing delimited player-gui block at the end of tools/verify181.sh; new modes are new Mode_ classes. Existing scenes are retained and round-four scenes are appended at the end of scenes.txt.

The final focused verifier exits 0 and matches the new controller/input/pick/reach, chest lifecycle, global key registry, comparator and seed-policy fixtures; all 23 whole-player scenarios; 2,124 stat-sync snapshots; the settings-before-preload/save regression; 3,805,894 combat bytes; scaling/buttons/font utilities; PlayerList and entry checks; playercore (2,539,266 bytes); creative (9,850,684 bytes); options/stat registries/formatters; and the complete recipe/smelting catalog. All 47 captures in six isolated groups are pixel EXACT (43 retained, four added). The successful final launch is recorded as BASH_EXIT=0 in target/player-gui4-final-focused.log. The traced --no-gui verifier reaches `exit 0` in target/player-gui4-final-nogui.log.

Release build, git diff --check, verifier Bash syntax and commit-script Bash syntax pass. All 31 changed/new source/report paths are inside the owned b181 crates/oracle, the existing focused verifier, or this report. No git staging or commit was run. The report retains all 86 class rows from round three.

Existing inventory/container regressions: 10,000 player inventory operations and 66,146 container click records / 161 recipes are EXACT. The existing player dig (1,000 ticks), digextras (1,600), place (360), use (300), and pickentities (200) sections match every frame and final world. These checks are retained in target/player-gui4-regressions.log.

The broader regression script exits 1 and is **not** a passing suite: containertiles differs at byte 1,057 (Rust 15,723,484 / Java 15,639,484 bytes), and the old api/pipeline Java drivers abort with NoSuchFieldException: mouseTicksRan. The tile replay exercises no player/controller/input and its implementation/fixture are unchanged by this round; its mismatch needs the world/entity fixture owners. The two obsolete main-loop drivers are retained unchanged; the new original-method controller/input replay provides focused coverage but does not replace a complete runTick/pipeline test.

A mixed close-screen diagnostic also differs on the creative HUD by 118,650 pixels/max 2 (target/player-gui4-new-gui.log). Both close-screen flows match in isolated fixtures; that result is not a claim that mixed inherited render state is fixed. The final focused verifier runs isolated groups, matching the existing focused HUD strategy. Strict comparisons use zero tolerance; no renderer-state reset was added to production code.

The full one-hour verify181.sh and diagnostic combined GUI suite were not rerun in this round. Round-three full GUI failures are retained as unresolved: title 197 pixels/max 1; achievements 85 (tooltip 44)/max 1; block stats 18/max 1; combined HUD 116,109 or 116,534/max 2; combined inventory label 364; equipped/filled models 1,996?5,009/max up to 255; post-anaglyph title 296,833/max 67. Focused scenes use their own fixture state; their exact result does not resolve those combined renderer failures.

## Per-class status

| Class | Status and evidence / remaining work |
| --- | --- |
| EntityPlayer | NOT DONE: retained playercore/tick/combat/inventory checks; Mode_controller181 additionally checks shears harvesting/use statistics, mining exhaustion and placement/use state. Complete changed-method interaction/NBT audit and critical visual callback remain. |
| EntityPlayerSP | NOT DONE: all 23 whole-player scenarios EXACT, including haste/fatigue/nausea. Controller/held input and picking replay are now EXACT; full application input replay and remaining methods are open. |
| FoodStats | DONE: Mode_playercore / playercore181: 72,000 boundary ticks and 10,000 randomized operations, including NBT. |
| PlayerControllerTest | NOT DONE: Mode_controller181 checks creative removal/fire/cooldown, placement count/damage restoration, sword use/stop, flags and reach; existing creativecontroller checks initialization/capabilities. Complete inherited API and lifecycle audit remains. |
| PlayerController | NOT DONE: original base/creative flags and actual Minecraft picking/reach plus placement/use are checked in Mode_controller181. Complete create-player/lifecycle/world callbacks and every base method are not proven. |
| PlayerControllerSP | NOT DONE: 2,400 survival/creative controller transitions and 800 held-button callbacks EXACT; block/metadata, inventory, mining state, exhaustion, statistics, RNG, events and spawned entities included. Complete lifecycle/music/interpolation API audit remains. |
| MovementInputFromOptions | DONE: Mode_playercore: all 64 movement-key combinations, including sneak float arithmetic. |
| WorldSettings | DONE: Mode_playercore: 30 seed/mode/features combinations. |
| InventoryPlayer | NOT DONE: retained 10,000 operations; setCurrentItem is now checked through 192 actual Minecraft middle-click cases, including first-match outside the hotbar and entity-hit zero coordinates. Complete changed-method/NBT audit remains. |
| PlayerCapabilities | DONE: Mode_playercore: defaults, creative enable, flying, creative disable. |
| MovementInput | DONE: Mode_playercore: 64 movement-key combinations. |
| Minecraft | NOT DONE: physical middle click (192 cases), creative/survival picking rays (136 cases), and actual held-button callbacks (800 transitions) EXACT. Pick counts now queue while using an item; held mining still runs and swings. F3+F / Shift+F3+F render distance source delta ported. Complete original runTick/application/debug replay is still absent. |
| GuiIngame | NOT DONE: original four HUD scenes plus seven new poison/hunger/regen/air/creative scenes are pixel EXACT. Player-list overlay, debug and complete portal/overlay coverage remain. |
| GuiContainerCreative | DONE: Mode_creative click policy EXACT; first, scrolled and cursor screenshots EXACT. |
| GuiMainMenu | NOT DONE: original panorama cube jitter, eight blur passes, UV crop, gradients, version/copyright and animation are implemented. Fresh title scenes differ by 197 pixels with maximum channel difference 1; 37/437-tick scenes also have one-shade differences. Title after a loaded anaglyph world has a larger inherited renderer-state difference. |
| PlayerList | DONE: Mode_playerlist181 / playerlist181, 3,000 operations and complete bucket snapshots EXACT, including the vanilla duplicate-key insertion bug, null values, collisions, resize order, counts and modification counter. |
| Container | DONE: containerreplay: 66,146 fixture cases EXACT, including shift recursion, transfer returns, slots/cursor and stats. |
| GuiCreateWorld | NOT DONE: Java 8 decimal digits exhaustive over 65,536 UTF-16 code units; 10,096 signed/overflow/hash/random-policy seed and mode/features records EXACT. Java executes original actionPerformed; Rust checks the production parser and policy, not live GUI callbacks. Clock-random seeds are classified, not claimed byte-identical. Existing four layouts plus two signed-seed layouts and world-settings integration pass. Folder/callback audit remains. |
| GuiAchievements | NOT DONE: fixed close key to configured inventory binding instead of sneak; new scene asserts sneak keeps it open and inventory closes it. Existing achievement rendering precision failures remain (85/44 one-shade pixels in round-three full suite). |
| ContainerCreative | DONE: Mode_creative / creative181: catalog, scroll positions, 10,000 operations and complete slot/cursor snapshots EXACT. |
| KeyBinding | DONE: Mode_keyregistry181 / --keyregistrycheck: 20,000 events, four simultaneous GameSettings and eight standalone bindings, global release/rebuild, duplicate/stale lookup, wrapping press overflow and consumption EXACT. Strong global registry retains old bindings and preserves original field registration order. |
| GuiStats | NOT DONE: retained exact general/item screen and formatter checks; item/block comparators now EXACT including overflow. Five block-stat icon scenes still had 18 one-shade pixels in round-three full GUI run. |
| GuiContainer | NOT DONE: creative branch and all three creative screenshots EXACT; wheel/drag support added. Six added noncreative empty-screen scenes are EXACT. Existing filled-slot/hover/click GUI fixtures and all changed input behavior still require coverage. |
| StatsSyncher | DONE for the original receive/send/tick/readiness/file-rotation protocol: Mode_statsync181 and --statsynccheck compare 2,124 snapshots, six recovery/checksum/overflow scenarios, pending receive/acknowledgement maps, and failed synchronous/background writes. Barriers only stabilize observations; arbitrary JVM thread scheduling is not claimed. StatFileWriter JSON ordering limitations are below. |
| StatList | DONE: Mode_options181 / --playerguicheck, complete general/block/item registries and mine/craft/use/break alias arrays/order EXACT against original bytecode. |
| GuiAchievement | DONE: original source diff audited (renames and removal of the old unlicensed-copy text). Thirteen new original-client pixel captures EXACT: start/slide/full/exit/end/expiry, no revival, zero clock, new snipe achievement, information and late information. Removed incorrect text shadows; matched depth mask and retained rescale-normal state. |
| PlayerListEntry | DONE: retained 3,000 complete bucket snapshots plus 10,000 entry equality/hash/string cases EXACT, with signed long keys, equal/unequal/null values and differing stored hash/link fields. |
| ContainerDispenser | DONE: containerreplay: dispenser transfer rules and click results EXACT. |
| RenderEngine | NOT DONE: dynamic panorama textures, framebuffer copies and color write masks added locally; all existing texture animation and GPU mask checks remain EXACT. Title one-shade precision and inherited second-texture/fog state after a live world need renderer integration. |
| GameSettings | DONE: retained options/defaults/load-save/10,000 rebinding event check EXACT; original global binding construction order and cross-instance identity additionally checked by Mode_keyregistry181. |
| GuiControls | DONE: All three controls scenes EXACT, including mouse rebinding and duplicate bindings; Mode_options181 event trace. |
| ContainerFurnace | DONE: containerreplay: furnace transfer rules and click results EXACT. |
| ContainerWorkbench | DONE: containerreplay: crafting/transfer rules and click results EXACT. |
| ContainerChest | DONE for vanilla single/double chest container behavior: retained 66,146 container replay records; Mode_chesthooks181 / chesthooks181 matches 1,200 ticks across five layouts, open/replacement/close, slots/limits, users, lid interpolation, sounds and world RNG (783,025 bytes). Uses existing world-blocks integration, with no tile-side edits. |
| StatBase | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiParticle | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiSelectWorld | NOT DONE: b181 saved GameType labels added; all four select-world screenshots pixel EXACT in the full GUI run. Full callbacks/method audit still remains. |
| GuiSlotStats | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryLargeChest | NOT DONE: actual BlockChest activation checks all four double-chest directions, slot order/limits and delegated open/close exactly in Mode_chesthooks181. Arbitrary constructor null/alias cases and full interface methods remain unaudited. |
| GuiRenameWorld | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| SorterStatsItem | DONE: original class diff audited; Mode_statsorder181 / --statsordercheck calls actual original comparator. 20,000 combined item/block comparisons EXACT across every column/direction, missing stats, ties and signed wrapping overflow. Production sorting uses this same comparator. |
| GuiInventory | NOT DONE: creative inventory selection added. Survival inventory pixels are EXACT in b181_inventory. Remaining creative redirect/update and input behavior require complete audit. |
| SorterStatsBlock | DONE: original class diff audited; Mode_statsorder181 / --statsordercheck proves all columns/directions, missing stats, ties and Java wrapping subtraction/multiply in 20,000 combined item/block comparisons. Block icon rendering differences remain under GuiStats. |
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

The NOT DONE rows are not completion claims. The largest remaining work is full EntityPlayer/EntityPlayerSP/controller/main-loop replay, GUI rendering precision and inherited GL state integration, arbitrary stat-file schema/serialization, full create/select/rename callbacks, and the smaller interface/screen audits retained above. The comparator check proves sorting, not block icon rendering. The seed check proves parsing/settings policy, not live Rust callback counts or clock-random values.

The original class deltas were reviewed using the two decompilations and a scratch normalized diff in target/player-gui4-class-audit.diff. The identifier map contains ambiguous mappings; source review, rather than normalized diff alone, determines behavior. No untested class was promoted merely because its normalized diff looked like renaming.

No b173 crate, oracle or verification script was edited. No entity renderer/model/particle, world renderer/lightmap, chest tile implementation, save or terrain algorithm was rewritten. Local player/controller changes use the existing world and entity-owner APIs. Git metadata was not written. target/commit-b181-player-gui4.sh contains explicit staging paths, the requested author and both coauthor trailers for the integrator to run.
