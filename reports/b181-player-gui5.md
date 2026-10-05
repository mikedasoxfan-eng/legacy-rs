# b1.8.1 player-gui, round 5

The full player/gui area remains **NOT DONE**. This round prioritizes gameplay boundaries, production Create World callbacks and inventory interfaces within the approximately one-hour task budget. All 86 previous class rows are retained below. Seven rows now have explicit b181-delta completion evidence: InventoryBasic, InventoryCrafting, InventoryCraftResult, IInventory, Slot, StatBase and Achievement. Vanilla catalog/delta scope is stated in each row; it is not an arbitrary-mod API claim.

## Changes

- Ported original InventoryPlayer membership and nullable NBT stack loading. Membership includes zero/negative stacks in every main slot and ignores armor. Bow start/release now use this shared method.
- Fixed sendUseItem's object-identity/count test and cleanup only in its changed branch. An unchanged zero-count held item remains equipped; placement and destruction clear exactly zero, retaining negative counts as vanilla does. The original click replay found these differences.
- Added EntityPlayerSP health/experience callbacks, including damage bypassing armor/blocking, exact damage/heal timers and argument order. Existing entity damage API is reused.
- Made HUD XP addition/multiplication explicitly wrapping, preserving Java behavior in debug as well as release builds.
- Added four new original-bytecode modes and Rust checkers. Create World replays call the production Rust screen methods and compare every launch/display action, rather than testing a parallel policy implementation.
- Added seven menu captures and ten HUD captures. HUD fixture fields now expose portal interpolation, nausea, sleep and position; potion reset supports explicit zero. Original production renderer state is untouched.
- Audited original inventory/interface and small-screen/statistic diffs. Texture-pack exception fallback and retained list mouse coordinates are concrete remaining deltas, recorded below.

## Verification

New sections in tools/verify_player_gui181.sh, reached by the existing delimited player-gui block at the end of tools/verify181.sh:

| Section | Original-bytecode result |
| --- | --- |
| playerstate181 | EXACT: 3,990,892 bytes; 4,516 NBT cases, 108 membership/pick cases, 6,000 health/XP callbacks, 10,000 mining interpolations |
| controllerinput181 | EXACT: 2,681,544 bytes; 2,800 survival/creative transitions through original Minecraft/controller/key methods |
| worldcallbacks181 / --worldcallbackcheck | EXACT: 336 production screen replays, 1,344 snapshots and complete launch/display traces |
| inventoryinterfaces181 | EXACT: 295,200 bytes; 2,400 original IInventory/base Slot operations |
| Round-five menu pixels | EXACT: seven captures, including blank/collision/focus names, disabled rename, confirmation/cancel |
| Round-five HUD pixels | EXACT in ten fresh processes: portal full/partial/nausea, creative pumpkin, four sleep fades, two XP overflow boundaries |

The final focused verifier exits 0 (BASH_EXIT=0 in target/player-gui5-final-focused.log). It retains all previous checks, and matches all 64 captures (47 retained, 17 new) in isolated groups. Zero pixel tolerance is used. No renderer-state reset was added to production. The ten HUD cases also passed their isolated diagnostic (target/player-gui5-hud-isolated.log, exit 0).

Existing regressions rerun this round: 10,000 player inventory operations and 66,146 container records / 161 recipes EXACT (target/player-gui5-regressions.log). The same scratch script's old player NBT simulation aborts in original Java EntityPlayerSP.onLivingUpdate with NullPointerException; its overall exit is 1, not a passing suite. Shared PlayerOracle is unchanged. New canonical inventory NBT coverage does not substitute for whole-player persistence verification.

The ten-scene combined HUD diagnostic exits 1: full portal EXACT; partial portal 116,070 pixels/max 2; nausea 116,534/max 2; pumpkin 16,534/max 1; sleep enter 116,534/max 2, full 32,328/max 1, exit 76,535/max 2, end 116,534/max 2; XP multiplication overflow 115,828/max 2 and cap overflow 117,776/max 2. These remain unresolved inherited render-state/precision failures (target/player-gui5-hud.log). Fresh-process results do not resolve them.

The full approximately one-hour verify181.sh and combined full GUI suite were not run. Previous unresolved diagnostics remain: title 197 pixels/max 1; achievements 85/tooltip 44/max 1; block stats 18/max 1; combined HUD 116,109 or 116,534/max 2; inventory label 364; equipped/filled models 1,996-5,009/max up to 255; post-anaglyph title 296,833/max 67. Round-four containertiles differed at byte 1,057, and obsolete api/pipeline drivers referenced removed mouseTicksRan. Those fixtures/implementations remain unchanged here.

The final release build, Bash syntax and git diff --check pass. No staging or commit is run. target/commit-b181-player-gui5.sh contains explicit paths, the requested author and both trailers.

## Per-class status

| Class | Status and evidence / remaining work |
| --- | --- |
| EntityPlayer | NOT DONE: retained playercore/tick/combat/inventory checks; both original controller fixtures check mining, shears, statistics, exhaustion, placement and item-use state. Round-five input catches zero/negative stack cleanup and same-object use return behavior. Complete changed-method interaction/NBT audit and critical visual callback remain. |
| EntityPlayerSP | NOT DONE: all 23 whole-player scenarios EXACT, including haste/fatigue/nausea; new Mode_playerstate181 proves 6,000 original setHealth/experience callbacks, timers, bypassed armor and XP fields/cap. Controller/held input/picking replays are EXACT. Full application input, remaining methods and complete player NBT replay remain open. |
| FoodStats | DONE: Mode_playercore / playercore181: 72,000 boundary ticks and 10,000 randomized operations, including NBT. |
| PlayerControllerTest | NOT DONE: retained removal/fire/cooldown, count/damage restoration, sword use/stop, flags/reach and creative initialization checks. New Mode_controllerinput181 also checks actual original click handlers over 1,400 creative transitions, including zero/negative counts and queued actions during use. Complete inherited API/lifecycle audit remains. |
| PlayerController | NOT DONE: original flags, picking/reach, placement/use and zero-sized equipped cleanup checked in controller181/controllerinput181. Full create-player/lifecycle/world callbacks and every base method are not proven. |
| PlayerControllerSP | NOT DONE: retained 2,400 controller transitions and 800 held-button callbacks EXACT; new 1,400 survival click/input transitions and 10,000 setPartialTime interpolations EXACT, including negative damage. Complete lifecycle/music API audit remains. |
| MovementInputFromOptions | DONE: Mode_playercore: all 64 movement-key combinations, including sneak float arithmetic. |
| WorldSettings | DONE: Mode_playercore: 30 seed/mode/features combinations. |
| InventoryPlayer | NOT DONE for the complete class audit; the b181 delta is checked for the vanilla item domain: source diff adds membership, nullable ItemStack NBT factory and no-op open/close callbacks; other delta is a method rename. Mode_playerstate181 / playerstate181: 4,516 original NBT cases over IDs 0..2257, all byte slots, signed count/damage, duplicate and invalid IDs, canonical serialized inventory and retained cursor/current/changed fields, plus 108 membership/pick cases EXACT. Retained 10,000 inventory operations and 192 actual middle-click cases EXACT. Canonical inventory NBT is tested; negative/out-of-array item ID exception semantics, remaining player-dependent inventory methods and complete player NBT are still unproven. |
| PlayerCapabilities | DONE: Mode_playercore: defaults, creative enable, flying, creative disable. |
| MovementInput | DONE: Mode_playercore: 64 movement-key combinations. |
| Minecraft | NOT DONE: retained actual middle click (192), picking rays (136) and held-button callbacks (800) EXACT. Mode_controllerinput181 adds 2,800 transitions invoking original clickMouse/sendClickBlockToController/middle-click methods and KeyBinding queues, including delayed use/release, miss cooldown and creative/survival differences. F3+F source delta retained. The fixture copies the runTick queue branch; it does not execute complete original runTick. Full application/debug replay remains absent. |
| GuiIngame | NOT DONE: retained four HUD and seven potion/air/creative captures EXACT. Ten new portal/nausea/pumpkin/sleep/XP overflow captures are EXACT in separate fresh processes; XP arithmetic explicitly wraps as Java. The same ten-capture combined replay still differs (nine scenes, maximum 1 or 2). Player-list overlay, debug, remaining overlays and inherited render-state integration remain. |
| GuiContainerCreative | DONE: Mode_creative click policy EXACT; first, scrolled and cursor screenshots EXACT. |
| GuiMainMenu | NOT DONE: original panorama cube jitter, eight blur passes, UV crop, gradients, version/copyright and animation are implemented. Fresh title scenes differ by 197 pixels with maximum channel difference 1; 37/437-tick scenes also have one-shade differences. Title after a loaded anaglyph world has a larger inherited renderer-state difference. |
| PlayerList | DONE: Mode_playerlist181 / playerlist181, 3,000 operations and complete bucket snapshots EXACT, including the vanilla duplicate-key insertion bug, null values, collisions, resize order, counts and modification counter. |
| Container | DONE: containerreplay: 66,146 fixture cases EXACT, including shift recursion, transfer returns, slots/cursor and stats. |
| GuiCreateWorld | NOT DONE: retained 65,536 UTF-16 decimal-code-unit and 10,096 seed/settings records EXACT. New Mode_worldcallbacks181 / --worldcallbackcheck runs actual production Rust screen initialization and callbacks: 336 runs, 1,344 snapshots, full Display/StartWorld traces EXACT for 14 name cases, six nonzero signed/hash seeds, collisions, mode/features, advanced view/focus, repeated create and cancel. Java executes original key/action callbacks after fixture initialization. Four original layouts, two signed-seed layouts and four new boundary layouts EXACT. Clock-random seeds, arbitrary filesystem failures and complete lifecycle are not proven. |
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
| StatBase | DONE for the b181 vanilla class delta (method/field renames): both decompilations audited. Retained Mode_options181 / --playerguicheck proves the complete vanilla registries, IDs/names/types and 15,072 formatter outputs EXACT; statistics synchronization exercises original stat objects. Runtime mod stat registration is outside the vanilla catalog. |
| GuiParticle | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiSelectWorld | NOT DONE: b181 saved GameType labels added; all four select-world screenshots pixel EXACT in the full GUI run. Full callbacks/method audit still remains. |
| GuiSlotStats | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryLargeChest | NOT DONE: actual BlockChest activation checks all four double-chest directions, slot order/limits and delegated open/close exactly in Mode_chesthooks181. Arbitrary constructor null/alias cases and full interface methods remain unaudited. |
| GuiRenameWorld | NOT DONE: source delta is renames; new whitespace-only name/disabled Enter capture EXACT. Complete original rename/save callbacks and failure behavior remain. |
| SorterStatsItem | DONE: original class diff audited; Mode_statsorder181 / --statsordercheck calls actual original comparator. 20,000 combined item/block comparisons EXACT across every column/direction, missing stats, ties and signed wrapping overflow. Production sorting uses this same comparator. |
| GuiInventory | NOT DONE: creative inventory selection added. Survival inventory pixels are EXACT in b181_inventory. Remaining creative redirect/update and input behavior require complete audit. |
| SorterStatsBlock | DONE: original class diff audited; Mode_statsorder181 / --statsordercheck proves all columns/directions, missing stats, ties and Java wrapping subtraction/multiply in 20,000 combined item/block comparisons. Block icon rendering differences remain under GuiStats. |
| ContainerPlayer | DONE: containerreplay: player crafting/transfer rules and click results EXACT. |
| GuiSlot | NOT DONE: source audit identifies the new protected mouse-coordinate fields assigned by drawScreen; current Rust does not retain them. Vanilla single-player lists do not consume them. Full original list/input callback replay remains. |
| GuiSlotStatsBlock | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiVideoSettings | DONE: b181_video_gamma pixels EXACT; gamma option labels checked by Mode_options181. |
| GuiSlotStatsItem | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryCrafting | DONE for the b181 delta (no-op open/close and row/column naming): source diff audited. Mode_inventoryinterfaces181 checks 800 original matrix operations including event-handler recipe/result recomputation; 2,400 combined inventory/Slot operations EXACT. Retained complete recipe/container replay also EXACT. |
| GuiTexturePackSlot | NOT DONE: source audit finds the b181 exception fallback to the default texture pack during selection/refresh. Current Rust refresh does not implement the complete Java fallback transaction. Original-bytecode fallback test and port remain. |
| GuiSavingLevelString | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiTexturePacks | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| StatFileWriter | NOT DONE: totals, unsent maps, wrapping i32 arithmetic, achievement presence/parents, sync/tick/readiness and valid/checksum-invalid/invalid-syntax file recovery match 2,124 original snapshots. Disk maps are normalized by stat id; raw JSON key order follows JVM identity hashes and is not byte-identical. Arbitrary malformed schema and all serialization entry points still need audit. |
| CraftingManager | DONE: b181 recipe table regenerated from original; all 161 recipes exercised in containerreplay, EXACT. |
| ThreadStatSyncherReceive | DONE: actual original thread executes both sent-file read and pending-map write branches; normalized disk/state snapshots, tick consumption and busy clearing match in Mode_statsync181. Uses the shared Syncher worker protocol. |
| GuiYesNo | NOT DONE: source delta is field renames; new original-client confirmation and cancel captures EXACT, including returning to Select World. Full callback protocol with arbitrary parent/IDs remains unproven. |
| InventoryBasic | DONE for the b181 delta (no-op open/close additions): source diff audited. Mode_inventoryinterfaces181 / inventoryinterfaces181 calls original IInventory and base Slot for 800 basic-inventory operations; combined 2,400 operations / 295,200 bytes EXACT, including put/clamp, get, signed/zero decrement, slot policy and callback no-op state. |
| InventoryCraftResult | DONE for the b181 delta (no-op open/close additions): source diff audited. Mode_inventoryinterfaces181 checks 800 original result-inventory operations through a base Slot; signed/zero counts and decrement behavior EXACT within the 295,200-byte fixture. Retained SlotCrafting/container replay EXACT. |
| ScaledResolution | DONE: original class diff is field renaming. Mode_guiutility181: 10,000 randomized dimensions/scales compare integer dimensions, factor and double dimensions byte for byte, including zero and sub-minimum displays. |
| Achievement | DONE for the b181 vanilla class delta (method/field renames): both decompilations audited. Retained Mode_options181 proves all 17 achievement IDs, coordinates, icons, parents, special flags, translated descriptions and grid bounds EXACT; Mode_statsync181 additionally proves achievement presence/parent policy. Runtime mod achievement construction is outside the vanilla catalog. |
| FontRenderer | DONE for the b181 class delta (method renames): Mode_guiutility181 compares 376 character/string/wrapping outputs, including all 256 characters, formatting, whitespace, newlines and unbroken words. Existing GUI text pixels and new multiline toast hints are EXACT. General renderer precision differences in other classes remain separately listed. |
| GuiSlotStatsGeneral | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| IInventory | DONE for the b181 interface delta (open/close additions): source diff audited. Mode_inventoryinterfaces181 invokes original callbacks for basic/crafting/result, proving their no-op state over 2,400 operations; retained Mode_chesthooks181 proves single/double chest open/close through actual ContainerChest. InventoryPlayer callbacks are source-audited no-ops. Arbitrary InventoryLargeChest constructor behavior is separately NOT DONE. |
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
| Slot | DONE for the b181 delta (inventory reference becomes public final): source diff audited. Mode_inventoryinterfaces181 proves 2,400 actual original base Slot operations, get/put/decrement/limit/background/item-valid/pickup behavior EXACT. Retained 66,146 container records exercise specialized slot policies. Java reference visibility is represented by the existing Rust slot/container model. |

## Remaining work and ownership

The largest remaining work is complete EntityPlayer/EntityPlayerSP/controller/Minecraft runTick and lifecycle replay, critical-hit presentation integration with the particle owner, full player NBT, player-list/debug overlays, combined GUI renderer precision/state, arbitrary stat-file schema/serialization, complete select/rename/save callbacks, texture-pack error fallback and the smaller screens/interfaces still marked NOT DONE. New click fixtures invoke original handlers but copy the runTick dispatcher branch; they do not claim full original application execution. Create World random policy remains covered by the prior seed fixture, with clock-random values not byte-identical.

Source audit scratch files: target/player-gui5-small-class-audit.diff and target/player-gui5-inventory-class-audit.diff. Review accounted for decompiler naming changes; completion evidence is explicit in each row.

No b173 crate, oracle or verification script was edited. No entity renderer/model/particle, world renderer/lightmap, chest tile implementation, save or terrain algorithm was rewritten. Changes use existing world/entity APIs. Git metadata was not written.
