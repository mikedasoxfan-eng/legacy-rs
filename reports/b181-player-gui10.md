# b1.8.1 player-gui, round 10

The area remains **NOT DONE**. This round completes one previously uncredited class, **GuiMainMenu**, and removes the remaining strict feature title failures. RenderEngine and StatFileWriter have new fixes and original-bytecode checks but remain explicitly NOT DONE. Minecraft lifecycle and handed-over GUI/world-rendering classes retain their assigned ownership.

The complete source audit and native-driver scope are in [b181-player-gui10-audit.md](b181-player-gui10-audit.md). No tolerance, captured-image substitution or pixel correction was added. The native title cube uses the installed OpenGL driver, with the existing wgpu framebuffer/blur/GUI pipeline; other renderers remain unchanged.

## New EXACT verification sections

The final delimited player-gui block in tools/verify181.sh invokes tools/verify_player_gui10_181.sh. Eight title animation scenes are appended at the end of tools/oracle181/gui/scenes.txt.

| New check | Original evidence |
| --- | --- |
| mainmenu10_181 / --mainmenu10check | 1,000 constructor strings/RNG; 10,000 layouts/keys; 40,000 updates; 60,000 actions; 3,328,664 original bytes EXACT |
| renderengine10_181 / --renderengine10check | 2,000 actual original single-prefix/filter/reset/cache cases EXACT |
| statfile10_181 / --statfile10check | 3,050 original schema/grammar/checksum/hash-order cases and 1,000 supplied-order serialization outputs EXACT |
| player-gui10-scenes.txt | All eight animated title RGB and alpha captures EXACT |
| feature-scenes.txt | All 15 feature RGB and alpha captures EXACT, including pack_title and anaglyph_pack_title |
| optional native panorama probe | Cube and all eight blur stages at time zero: zero differing RGB pixels; all 6,144 observed float matrix elements EXACT |

The focused suite compares 23 RGB captures with zero tolerance and 9,428,160 alpha bytes.

## Verification results

- Release build of worldgen181 and client passes.
- tools/verify_gui181.sh passes after the appended title block explicitly resets the sessionless fixture input. Its main combined sequence has 195 of 203 captures strict RGB EXACT; the same eight inherited achievement/stat icon captures retain one-level differences. All title captures are EXACT. All 35 containers, nine signs and 15 feature captures are strict RGB EXACT; the 33 HUD brightness hints match. No comparator tolerance was changed.
- tools/verify_player_gui7_181.sh --built, tools/verify_player_gui8_181.sh --built and tools/verify_player_gui9_181.sh --built pass, including retained original callback, RGB, alpha, framebuffer, texture conversion and animation checks.
- tools/verify_player_gui181.sh passes, including all retained complete-player replays, statistics synchronization, GUI utility/registry/controller/recipe checks and GUI captures.
- tools/verify_player_gui10_181.sh --built passes its three new original-bytecode checks and all 23 strict RGB/alpha captures.
- git diff --check and shell syntax checks pass. The complete approximately one-hour tools/verify181.sh run was not run here; it remains with the integrator.

Evidence logs are target/player-gui10-final-build.log, target/player-gui10-verify_gui181-final.log, target/player-gui10-verify-final.log and target/player-gui10-verify_player_gui{7_181,8_181,9_181,181}.log. target/player-gui10-evidence.json records the strict combined-capture counts and optional stage/matrix comparison. The initial retained batch's aggregate exit was 1 solely because its first GUI run exposed the new appended-title session input mismatch; that run was corrected and rerun successfully.

## Per-class status

Statuses outside this round's three final classes carry forward the round-nine evidence and ownership; they are not counted as newly completed here.



| Class | Status and evidence / remaining work |
| --- | --- |
| EntityPlayer | DONE, prior integrator credit retained. Round 8 fixes source-without-attacker death motion and live self-shooter attribution/position/self-kill callbacks. playerdamage8_181 matches 2,400 original calls, motion/health/flags/RNG/statistics/events (399,208 bytes EXACT); all six affected complete player replays are EXACT. No new class count is added for a previously credited class. |
| EntityPlayerSP | DONE for the vanilla singleplayer b181 delta: complete changed-method audit in b181-player-gui6-sp-audit.md. New playersp181 matches 10,000 action/sprint/FOV boundaries and 650 rotated/death/sprint-expiration ticks (1,185,418 bytes); all 23 retained full-player replays and health/XP callback checks remain EXACT. Snapshot arm fields now follow the actual action callback. |
| FoodStats | DONE: Mode_playercore / playercore181: 72,000 boundary ticks and 10,000 randomized operations, including NBT. |
| PlayerControllerTest | DONE for the vanilla b181 delta: b181-player-gui6-controller-audit.md maps every changed method. controllermode181 matches 10,000 supplied-player initialization/capability/update/flag cases (6,150,000 bytes), including inherited no-op update and no creative random-music request. Retained original controller, creative click/held input, placement/count-restoration and creative container checks remain EXACT. |
| PlayerController | DONE for the vanilla b181 singleplayer class delta: full source-diff audit in b181-player-gui7-controller-audit.md. controllerlife181 factory/flip/stop/air checks plus retained controllermode181 flags/no-ops, controller/input/use/combat and container/close replays are EXACT. Complete Minecraft lifecycle replay remains a separate NOT DONE class. |
| PlayerControllerSP | DONE for the vanilla b181 class delta: b181-player-gui7-controller-audit.md maps every change. controllerlife181 proves 1,000 original factory/flip cases, 20,000 partial/reset boundaries and 256 stop-use cases (1,232,729 bytes), including the original direct-air failure. Existing controller/mining/input/music/flag checks remain EXACT. Production spawn and respawn now use the mode-aware flip callback. |
| MovementInputFromOptions | DONE: Mode_playercore: all 64 movement-key combinations, including sneak float arithmetic. |
| WorldSettings | DONE: Mode_playercore: 30 seed/mode/features combinations. |
| InventoryPlayer | DONE for the vanilla b181 class delta: complete diff audit in b181-player-gui7-inventory-audit.md. inventorybounds181 exhausts all 65,536 signed-short item IDs, array failures before slot filtering, partially loaded arrays, callbacks, cursor/current/changed retention and membership (43,843,584 bytes EXACT). Existing playerstate181 canonical NBT and 10,000 inventory operations remain the evidence for the unchanged methods. Arbitrary noncompound NBTTagList elements and mod registration are not claimed. |
| PlayerCapabilities | DONE: Mode_playercore: defaults, creative enable, flying, creative disable. |
| MovementInput | DONE: Mode_playercore: 64 movement-key combinations. |
| Minecraft | NOT DONE in this handoff; owned by the separate b181-minecraft1 worker. Prior integration evidence is retained; no app/main-loop file is edited in round 9. |
| GuiIngame | NOT DONE in this worker handoff; owned by integrator and not edited. Retained isolated HUD captures remain covered; full combined vignette/state and debug/player-list integration belongs to the integrator. |
| GuiContainerCreative | DONE: Mode_creative click policy EXACT; first, scrolled and cursor screenshots EXACT. |
| GuiMainMenu | DONE for the vanilla b181 singleplayer delta and canonical Windows captures. Full source audit: b181-player-gui10-audit.md. mainmenu10_181 proves 1,000 original constructor splashes/RNG, 10,000 layouts/keys, 40,000 update boundaries and 60,000 singleplayer actions (3,328,664 bytes EXACT). All eight new animated titles and both feature titles are strict RGB/alpha EXACT. Native cube and eight blur stage observations are RGB EXACT. Arbitrary undefined small-window copy bytes and unsupported GL/MSAA fallback paths are not newly credited. |
| PlayerList | DONE: Mode_playerlist181 / playerlist181, 3,000 operations and complete bucket snapshots EXACT, including the vanilla duplicate-key insertion bug, null values, collisions, resize order, counts and modification counter. |
| Container | DONE: containerreplay: 66,146 fixture cases EXACT, including shift recursion, transfer returns, slots/cursor and stats. |
| GuiCreateWorld | DONE, integrator credited: all layouts EXACT, seed/settings checks and live callbacks play-tested. Retained worldseed181 and worldcallbacks181 production screen/action traces remain EXACT. Clock-random seeds retain the original policy rather than byte-identical timestamps. |
| GuiAchievements | NOT DONE under this round's strict all-EXACT counting rule. Prior integrator credit under the b173 known-driver-difference policy is historical only; three full-suite 3D icon captures retain one-level RGB differences. Other prior callbacks/captures remain covered. Integrator-owned and not edited. |
| ContainerCreative | DONE: Mode_creative / creative181: catalog, scroll positions, 10,000 operations and complete slot/cursor snapshots EXACT. |
| KeyBinding | DONE: Mode_keyregistry181 / --keyregistrycheck: 20,000 events, four simultaneous GameSettings and eight standalone bindings, global release/rebuild, duplicate/stale lookup, wrapping press overflow and consumption EXACT. Strong global registry retains old bindings and preserves original field registration order. |
| GuiStats | NOT DONE under this round's strict all-EXACT counting rule. Prior integrator credit under the b173 known-driver-difference policy is historical only; five full-suite block-icon captures retain 18 one-level RGB differences each. Exact general/item formatter/comparator evidence remains retained. Not edited. |
| GuiContainer | DONE for the vanilla b181 delta: complete audit in b181-player-gui6-container-audit.md. All 35 inherited container captures and 16 controlled nonzero-lightmap captures pixel EXACT. Original GuiContainer double-close callbacks match 384 cases/six kinds (45,600 bytes); world-side chest hooks match 1,200 ticks/five layouts (783,025 bytes). Retained vanilla click/creative/container replays remain EXACT. |
| StatsSyncher | DONE for the original receive/send/tick/readiness/file-rotation protocol: Mode_statsync181 and --statsynccheck compare 2,124 snapshots, six recovery/checksum/overflow scenarios, pending receive/acknowledgement maps, and failed synchronous/background writes. Barriers only stabilize observations; arbitrary JVM thread scheduling is not claimed. StatFileWriter JSON ordering limitations are below. |
| StatList | DONE: Mode_options181 / --playerguicheck, complete general/block/item registries and mine/craft/use/break alias arrays/order EXACT against original bytecode. |
| GuiAchievement | DONE: original source diff audited (renames and removal of the old unlicensed-copy text). Thirteen new original-client pixel captures EXACT: start/slide/full/exit/end/expiry, no revival, zero clock, new snipe achievement, information and late information. Removed incorrect text shadows; matched depth mask and retained rescale-normal state. |
| PlayerListEntry | DONE: retained 3,000 complete bucket snapshots plus 10,000 entry equality/hash/string cases EXACT, with signed long keys, equal/unequal/null values and differing stored hash/link fields. |
| ContainerDispenser | DONE: containerreplay: dispenser transfer rules and click results EXACT. |
| RenderEngine | NOT DONE. This round fixes the literal single-prefix and new %blurclamp% policy; renderengine10_181 matches 2,000 actual original resource/filter/reset/cache cases. Title RGB precision is now solved by the local GUI native cube adapter. Complete contents caching, mutable-flag failure retention, dynamic-image map refresh/deletion and image-buffer limit exceptions remain open. Retained texture conversion/animations/framebuffer evidence is carried forward. |
| GameSettings | DONE: retained options/defaults/load-save/10,000 rebinding event check EXACT; original global binding construction order and cross-instance identity additionally checked by Mode_keyregistry181. |
| GuiControls | DONE: All three controls scenes EXACT, including mouse rebinding and duplicate bindings; Mode_options181 event trace. |
| ContainerFurnace | DONE: containerreplay: furnace transfer rules and click results EXACT. |
| ContainerWorkbench | DONE: containerreplay: crafting/transfer rules and click results EXACT. |
| ContainerChest | DONE for vanilla single/double chest container behavior: retained 66,146 container replay records; Mode_chesthooks181 / chesthooks181 matches 1,200 ticks across five layouts, open/replacement/close, slots/limits, users, lid interpolation, sounds and world RNG (783,025 bytes). Uses existing world-blocks integration, with no tile-side edits. |
| StatBase | DONE for the b181 vanilla class delta (method/field renames): both decompilations audited. Retained Mode_options181 / --playerguicheck proves the complete vanilla registries, IDs/names/types and 15,072 formatter outputs EXACT; statistics synchronization exercises original stat objects. Runtime mod stat registration is outside the vanilla catalog. |
| GuiParticle | NOT DONE in this handoff; integrator-owned under the final round-nine scope. Original GUI particle callback/interpolation coverage remains open. Not edited. |
| GuiSelectWorld | DONE for the complete vanilla b181 singleplayer class delta. select8_181 compares 2,048 original label/init/selection/controller/factory-yaw/latch/failure cases plus 96 delete callbacks. All nonzero saved GameTypes select creative; worker capability and spawn/respawn yaw calls share the policy. Exact select/rename/delete pixels. Complete saved-world application lifecycle remains under Minecraft. |
| GuiSlotStats | DONE for the complete vanilla b181 delta (GuiStats accessor/draw renames), audited in b181-player-gui8-audit.md. stats8_181 proves original text/alignment/color, header hit ranges and sort cycles, click sounds, every header vertex, and 59,136 tooltip boundaries including gradient/shadow arguments. Shared GuiSlot proof and retained comparator checks remain EXACT. |
| InventoryLargeChest | NOT DONE: actual BlockChest activation checks all four double-chest directions, slot order/limits and delegated open/close exactly in Mode_chesthooks181. Arbitrary constructor null/alias cases and full interface methods remain unaudited. |
| GuiRenameWorld | DONE for the complete vanilla b181 delta. rename8_181 compares 360 original init/update/focus/key/action/save cases and missing-save failure/partial controls (47,165 bytes EXACT). Production uses actual fixture NBT and preserves missing-WorldInfo failure. Cursor, focus, empty/spaces/Enter, saved name, list return and key-repeat captures EXACT. |
| SorterStatsItem | DONE: original class diff audited; Mode_statsorder181 / --statsordercheck calls actual original comparator. 20,000 combined item/block comparisons EXACT across every column/direction, missing stats, ties and signed wrapping overflow. Production sorting uses this same comparator. |
| GuiInventory | DONE for the complete vanilla b181 singleplayer delta. b181-player-gui9-audit.md maps every changed method/field. inventorygui181 compares 10,000 actual original init/constructor cases, 30,000 updates and 50,000 buttons (940,000 bytes EXACT). All 30 new death/light/armor/held-tool/mouse captures, all 18 retained brightness/creative-transition captures and all 35 inherited container captures are pixel EXACT. Material clamp, rescale-normal retention and object-space held-item MVP now follow original GL. Combined-sequence fixture input differences are documented below; no entity renderer/model edits. |
| SorterStatsBlock | DONE: original class diff audited; Mode_statsorder181 / --statsordercheck proves all columns/directions, missing stats, ties and Java wrapping subtraction/multiply in 20,000 combined item/block comparisons. Block icon rendering differences remain under GuiStats. |
| ContainerPlayer | DONE: containerreplay: player crafting/transfer rules and click results EXACT. |
| GuiSlot | DONE for the complete vanilla b181 delta. b181-player-gui8-audit.md maps the renamed slot height and new protected mouse fields. guislot8_181 compares 10,000 original scroll/drag/header/row/overlay callbacks and state (2,630,232 bytes EXACT). New list captures and retained GUI captures are pixel EXACT. |
| GuiSlotStatsBlock | DONE for the complete vanilla b181 delta (stat-array/accessor/draw renames). stats8_181 calls actual original constructors and rows. Combined item/block coverage: 256 complete catalogs/row scenarios, 5,376 header/sort/vertex cases and 59,136 tooltip boundaries (9,756,693 bytes EXACT). Five new flat-block captures pixel EXACT. Inherited 3D icon driver differences are not new Done evidence. |
| GuiVideoSettings | DONE: b181_video_gamma pixels EXACT; gamma option labels checked by Mode_options181. |
| GuiSlotStatsItem | DONE for the complete vanilla b181 delta (stat-array/accessor/draw renames). The same stats8_181 original constructor/row/header/tooltip trace proves membership, missing-stat dashes, formatting, order, pressed icons, labels and all three sort cycles. Nine new flat-item/header/tooltip captures pixel EXACT. |
| InventoryCrafting | DONE for the b181 delta (no-op open/close and row/column naming): source diff audited. Mode_inventoryinterfaces181 checks 800 original matrix operations including event-handler recipe/result recomputation; 2,400 combined inventory/Slot operations EXACT. Retained complete recipe/container replay also EXACT. |
| GuiTexturePackSlot | NOT DONE in this handoff; integrator-owned under the final round-nine scope. Complete exception fallback transaction still needs proof. Ordinary feature captures and alpha remain covered. |
| GuiSavingLevelString | OUT OF SCOPE: multiplayer-only per the round-eight task. No singleplayer Done credit or implementation change claimed. |
| GuiTexturePacks | NOT DONE in this handoff; integrator-owned under the final round-nine scope. Full callback/timer/scan/refresh ordering remains unproven. Not edited. |
| StatFileWriter | NOT DONE. statfile10_181 proves 3,050 original schema/grammar/checksum/hash-order cases and 1,000 byte-identical serializers with supplied map iteration order. Production sorted-ID file ordering still differs from JVM identity-hash map iteration; complete constructor failures and arbitrary malformed/charset boundaries remain open. Existing StatsSyncher and achievement/state evidence is retained. |
| CraftingManager | DONE: b181 recipe table regenerated from original; all 161 recipes exercised in containerreplay, EXACT. |
| ThreadStatSyncherReceive | DONE: actual original thread executes both sent-file read and pending-map write branches; normalized disk/state snapshots, tick consumption and busy clearing match in Mode_statsync181. Uses the shared Syncher worker protocol. |
| GuiYesNo | DONE for the complete vanilla b181 delta (label field renames). yesno8_181 compares 4,096 original init/action/Escape cases with arbitrary recording parents, signed world/button IDs, repeated init and callback-directed redisplay (428,032 bytes EXACT). New hover/cancel/delete pixels EXACT. |
| InventoryBasic | DONE for the b181 delta (no-op open/close additions): source diff audited. Mode_inventoryinterfaces181 / inventoryinterfaces181 calls original IInventory and base Slot for 800 basic-inventory operations; combined 2,400 operations / 295,200 bytes EXACT, including put/clamp, get, signed/zero decrement, slot policy and callback no-op state. |
| InventoryCraftResult | DONE for the b181 delta (no-op open/close additions): source diff audited. Mode_inventoryinterfaces181 checks 800 original result-inventory operations through a base Slot; signed/zero counts and decrement behavior EXACT within the 295,200-byte fixture. Retained SlotCrafting/container replay EXACT. |
| ScaledResolution | DONE: original class diff is field renaming. Mode_guiutility181: 10,000 randomized dimensions/scales compare integer dimensions, factor and double dimensions byte for byte, including zero and sub-minimum displays. |
| Achievement | DONE for the b181 vanilla class delta (method/field renames): both decompilations audited. Retained Mode_options181 proves all 17 achievement IDs, coordinates, icons, parents, special flags, translated descriptions and grid bounds EXACT; Mode_statsync181 additionally proves achievement presence/parent policy. Runtime mod achievement construction is outside the vanilla catalog. |
| FontRenderer | DONE for the b181 class delta (method renames): Mode_guiutility181 compares 376 character/string/wrapping outputs, including all 256 characters, formatting, whitespace, newlines and unbroken words. Existing GUI text pixels and new multiline toast hints are EXACT. General renderer precision differences in other classes remain separately listed. |
| GuiSlotStatsGeneral | DONE for the b181 class delta: b181-player-gui7-general-stats-audit.md. generalstats181 captures actual original virtual drawString arguments for all general stats over 1,024 signed-value cases and compares list/no-op/selection/height policy, text, alignment and colors; 30,000 additional distance-rounding boundary calls also match (1,740,218 bytes EXACT). Existing isolated general-stat pixels remain covered. |
| IInventory | DONE for the b181 interface delta (open/close additions): source diff audited. Mode_inventoryinterfaces181 invokes original callbacks for basic/crafting/result, proving their no-op state over 2,400 operations; retained Mode_chesthooks181 proves single/double chest open/close through actual ContainerChest. InventoryPlayer callbacks are source-audited no-ops. Arbitrary InventoryLargeChest constructor behavior is separately NOT DONE. |
| ThreadStatSyncherSend | DONE: actual original thread writes periodic snapshots and clears busy after success and an unwritable .tmp-directory failure; Mode_statsync181 trace EXACT. |
| GuiScreen | NOT DONE: mouse wheel forwarding added for creative inventory; remaining class changes not fully audited. |
| RecipesCrafting | DONE: Original b181 recipe data regenerated; full recipe fixture replay EXACT. |
| AchievementList | DONE: Mode_options181 / --playerguicheck, all 17 achievement IDs, grid coordinates, icons, parents, special flags, translated text and grid bounds EXACT. |
| EnumOptions | DONE: Mode_options181 / --playerguicheck: all 15 option ordinals, metadata and labels. |
| FurnaceRecipes | DONE for the b181 delta and vanilla catalog: original class diff adds beef/chicken recipes; Mode_recipeaudit181 queries all 2,258 item IDs and the full recipe/smelting binary equals the port's recipes.bin. Container replay remains EXACT. Runtime mod registration is outside the vanilla catalog. |
| GuiChat | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. Integrator owns this class; not edited here. |
| GuiOptions | DONE: b181_options and b181_options_fov pixels EXACT. |
| GuiWorldSlot | NOT DONE, remains integrator-owned. The existing list adapter now consumes the screen-owned game-mode labels and preserves the original invalid-array-index failure. This minimal bridge serves the GuiSelectWorld delta; no separate GuiWorldSlot Done credit is claimed. |
| ICrafting | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| LoadingScreenRenderer | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| RecipesFood | DONE: Original b181 recipe data regenerated; full recipe fixture replay EXACT. |
| StatTypeDistance | DONE, strengthened in round seven: the inherited decimal double-rounding bug is corrected. generalstats181 compares 30,000 original calls around 10,000 random decimal-half boundaries, plus every general-stat row over 1,024 cases; retained 15,072 formatter outputs remain EXACT. |
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

RenderEngine still needs complete API/cache/mutable-flag/failure and dynamic-image map proof. StatFileWriter still needs production JVM identity-hash ordering and complete constructor/malformed/charset boundaries. These classes remain uncredited despite the exact new subchecks.

Minecraft runTick/lifecycle belongs to b181-minecraft1. The integrator retains GuiIngame and the other handed-over GUI/world renderer classes. The native adapter is confined to the title cube; it does not edit entity models/renderers, particles, saves, world/chunk rendering, lightmaps, fog, FOV or terrain. Only a minimal initial-capacity API was added to the existing b181 JavaHashSet for GUI JSON map copies; its default world-set behavior is unchanged.

No b173 crate/oracle/verifier or shared Oracle.java/world fixture/listener was edited. The approximately one-hour full verify181.sh run remains with the integrator as requested. Git metadata was not written. target/commit-b181-player-gui10.sh stages explicit task files and uses the requested author/email and both co-author trailers.
