# b1.8.1 player-gui, round 6

The complete area remains **NOT DONE**. This round completes three vanilla class deltas: **EntityPlayerSP, PlayerControllerTest and GuiContainer**. It also records the integrator's credited EntityPlayer, GuiCreateWorld, GuiAchievements and GuiStats, without modifying those implementations. All 86 previous class rows are retained below.

## Changes

- First-person arm snapshots now copy the real SP action-state smoothing fields at the original callback, including their freeze on death. No renderer/FOV implementation was rewritten.
- Creative controller ticks preserve inherited no-op behavior: they no longer advance survival damage interpolation or request survival random music.
- GUI GL records lightmap unit-1 state per draw and applies its RGB after unit 0, using the existing lightmap sampler. Container coordinates are set at the original point. The GUI figure adapter preserves the original hurt-overlay disable and unconditional re-enable; live GUI drawing receives the existing world lightmap/brightness.
- GUI close commands preserve the original extra container callback before controller close. This retains duplicate workbench drops and double chest-use decrements. Ordinary controller close remains unchanged.
- Added four independent original-bytecode modes, Rust checkers and sixteen controlled lightmap captures. Shared Oracle.java, world fixtures and listeners are unchanged.

## Verification

The existing delimited player-gui block at the end of tools/verify181.sh invokes tools/verify_player_gui181.sh. Its new sections and results are:

| Section | Original-bytecode result |
| --- | --- |
| controllermode181 | EXACT: 10,000 initialization/capability/update/flag cases; 6,150,000 bytes |
| playersp181 | EXACT: 10,000 action/FOV boundaries and 650 actual onUpdate ticks; 1,185,418 bytes |
| guiclose181 / --guiclosecheck | EXACT: 384 original GuiContainer callbacks, six kinds, cursor/grid/drop traces; 45,600 bytes |
| guichest181 | EXACT: 1,200 ticks/five actual chest layouts, GUI close, users/lids/slots/sounds/RNG/events; 783,025 bytes |
| container-scenes.txt | All 35 inherited container captures pixel EXACT, zero tolerance |
| player-gui6-scenes.txt | All 16 controlled nonzero unit-1 texture captures pixel EXACT, zero tolerance |

The final focused verifier exits 0 (`target/player-gui6-final-focused.log`): all retained behavior checks, all 23 whole-player scenarios, statistics synchronization, the settings-before-first-save regression, and all 115 captures (64 retained, 35 inherited containers, 16 new lightmaps) are EXACT with zero pixel tolerance. The additional 10,000 inventory operations and 66,146 container records / 161 recipes also pass (`REGRESSIONS_EXIT=0`, target/player-gui6-regressions.log). Prior standalone container/lightmap comparison artifacts are target/player-gui6-containers-diff and target/player-gui6-lightmap-diff. Audit reports map every changed class method to its production port and original evidence.

The full GUI verifier was also run and exits 1 (`FULL_GUI_EXIT=1`, target/player-gui6-full-gui.log). Its separate inherited container group now matches all 35 scenes, and all nine sign captures, texture animation and GPU color-mask checks remain EXACT. The main sequence still differs: title 197 pixels/max 1; achievement icons 85/44 and block-stat icons 18 within one level; hud_full/b181_hud_full 116,534/max 2 and b181_hud_potions 116,109/max 2; creative inventory close 92,485/max 1. Combined confirmation and round-five HUD scenes also differ (including max 255); their isolated focused captures pass. These larger combined differences need a baseline/state investigation; they are not declared unchanged from round five. Feature scenes retain pack_title 197/max 1 and anaglyph_pack_title 296,833/max 67.

In the appended full-sequence lightmap block, twelve captures are EXACT; four hurt captures differ by 4,726 pixels/max 22, confined to the player figure rectangle (324,110)-(380,228). Labels, icons and backgrounds remain exact. Source audit identifies an existing GUI figure gap: the original hurt overlay uses Entity.getEntityBrightness, while the existing adapter uses 1.0. Its inherited world/player position depends on the preceding sequence. The sixteen focused captures are exact, but do not resolve this general GuiInventory preview brightness gap. No unrelated entity renderer/model was changed to hide it.

The integrator's playernbt181 regression was rerun and remains EXACT (8,212 bytes; target/player-gui6-nbt.log). The final release build, Bash syntax checks and git diff --check pass. The approximately one-hour full verify181.sh is left to the integrator as requested. No title approximation or tolerance change was retained. Original GL matrix/rotation diagnostic probes identified driver float differences, but the title's 197 pixels/max 1 remain unresolved. No integrator-owned HUD/debug/menu class was changed.

## Per-class status

| Class | Status and evidence / remaining work |
| --- | --- |
| EntityPlayer | DONE, integrator credited: playernbt181 insertion-order save tags and critical-hit particle callback. Retained playercore/tick/combat/input checks remain exact. This round does not modify persistence.rs or the critical hook. |
| EntityPlayerSP | DONE for the vanilla singleplayer b181 delta: complete changed-method audit in b181-player-gui6-sp-audit.md. New playersp181 matches 10,000 action/sprint/FOV boundaries and 650 rotated/death/sprint-expiration ticks (1,185,418 bytes); all 23 retained full-player replays and health/XP callback checks remain EXACT. Snapshot arm fields now follow the actual action callback. |
| FoodStats | DONE: Mode_playercore / playercore181: 72,000 boundary ticks and 10,000 randomized operations, including NBT. |
| PlayerControllerTest | DONE for the vanilla b181 delta: b181-player-gui6-controller-audit.md maps every changed method. controllermode181 matches 10,000 supplied-player initialization/capability/update/flag cases (6,150,000 bytes), including inherited no-op update and no creative random-music request. Retained original controller, creative click/held input, placement/count-restoration and creative container checks remain EXACT. |
| PlayerController | NOT DONE: original flags, picking/reach, placement/use and zero-sized equipped cleanup checked in controller181/controllerinput181. Full create-player/lifecycle/world callbacks and every base method are not proven. |
| PlayerControllerSP | NOT DONE: retained original controller/input/mining checks EXACT; controllermode181 adds original survival update interpolation/music/flag checks. Full create-player/lifecycle API audit remains. |
| MovementInputFromOptions | DONE: Mode_playercore: all 64 movement-key combinations, including sneak float arithmetic. |
| WorldSettings | DONE: Mode_playercore: 30 seed/mode/features combinations. |
| InventoryPlayer | NOT DONE for the complete class audit; the b181 delta is checked for the vanilla item domain: source diff adds membership, nullable ItemStack NBT factory and no-op open/close callbacks; other delta is a method rename. Mode_playerstate181 / playerstate181: 4,516 original NBT cases over IDs 0..2257, all byte slots, signed count/damage, duplicate and invalid IDs, canonical serialized inventory and retained cursor/current/changed fields, plus 108 membership/pick cases EXACT. Retained 10,000 inventory operations and 192 actual middle-click cases EXACT. Canonical inventory NBT is tested and the integrator whole-player NBT regression remains EXACT. Negative/out-of-array item ID exception semantics and remaining player-dependent inventory methods are still unproven. |
| PlayerCapabilities | DONE: Mode_playercore: defaults, creative enable, flying, creative disable. |
| MovementInput | DONE: Mode_playercore: 64 movement-key combinations. |
| Minecraft | NOT DONE: retained original picking/click/queued input checks EXACT; creative controller update policy and live GUI lightmap handoff corrected using existing APIs. New checks do not execute complete original runTick. Full application lifecycle/debug replay and general caller render-state integration remain. |
| GuiIngame | NOT DONE in this worker handoff; owned by integrator and not edited. Retained isolated HUD captures remain covered; full combined vignette/state and debug/player-list integration belongs to the integrator. |
| GuiContainerCreative | DONE: Mode_creative click policy EXACT; first, scrolled and cursor screenshots EXACT. |
| GuiMainMenu | NOT DONE: original panorama cube jitter, eight blur passes, UV crop, gradients, version/copyright and animation are implemented. Fresh title scenes differ by 197 pixels with maximum channel difference 1; 37/437-tick scenes also have one-shade differences. Title after a loaded anaglyph world has a larger inherited renderer-state difference. |
| PlayerList | DONE: Mode_playerlist181 / playerlist181, 3,000 operations and complete bucket snapshots EXACT, including the vanilla duplicate-key insertion bug, null values, collisions, resize order, counts and modification counter. |
| Container | DONE: containerreplay: 66,146 fixture cases EXACT, including shift recursion, transfer returns, slots/cursor and stats. |
| GuiCreateWorld | DONE, integrator credited: all layouts EXACT, seed/settings checks and live callbacks play-tested. Retained worldseed181 and worldcallbacks181 production screen/action traces remain EXACT. Clock-random seeds retain the original policy rather than byte-identical timestamps. |
| GuiAchievements | DONE, integrator credited under the b173 known-difference policy for one-level driver rounding in 3D icons. Configured inventory close binding and retained captures/callback checks preserved. Driver differences are not reported as pixel EXACT. |
| ContainerCreative | DONE: Mode_creative / creative181: catalog, scroll positions, 10,000 operations and complete slot/cursor snapshots EXACT. |
| KeyBinding | DONE: Mode_keyregistry181 / --keyregistrycheck: 20,000 events, four simultaneous GameSettings and eight standalone bindings, global release/rebuild, duplicate/stale lookup, wrapping press overflow and consumption EXACT. Strong global registry retains old bindings and preserves original field registration order. |
| GuiStats | DONE, integrator credited under the b173 known-difference policy for one-level driver rounding in 3D icons. General/item screen, formatters and original overflow-aware comparators remain EXACT. Driver differences are not reported as pixel EXACT. |
| GuiContainer | DONE for the vanilla b181 delta: complete audit in b181-player-gui6-container-audit.md. All 35 inherited container captures and 16 controlled nonzero-lightmap captures pixel EXACT. Original GuiContainer double-close callbacks match 384 cases/six kinds (45,600 bytes); world-side chest hooks match 1,200 ticks/five layouts (783,025 bytes). Retained vanilla click/creative/container replays remain EXACT. |
| StatsSyncher | DONE for the original receive/send/tick/readiness/file-rotation protocol: Mode_statsync181 and --statsynccheck compare 2,124 snapshots, six recovery/checksum/overflow scenarios, pending receive/acknowledgement maps, and failed synchronous/background writes. Barriers only stabilize observations; arbitrary JVM thread scheduling is not claimed. StatFileWriter JSON ordering limitations are below. |
| StatList | DONE: Mode_options181 / --playerguicheck, complete general/block/item registries and mine/craft/use/break alias arrays/order EXACT against original bytecode. |
| GuiAchievement | DONE: original source diff audited (renames and removal of the old unlicensed-copy text). Thirteen new original-client pixel captures EXACT: start/slide/full/exit/end/expiry, no revival, zero clock, new snipe achievement, information and late information. Removed incorrect text shadows; matched depth mask and retained rescale-normal state. |
| PlayerListEntry | DONE: retained 3,000 complete bucket snapshots plus 10,000 entry equality/hash/string cases EXACT, with signed long keys, equal/unequal/null values and differing stored hash/link fields. |
| ContainerDispenser | DONE: containerreplay: dispenser transfer rules and click results EXACT. |
| RenderEngine | NOT DONE: GUI GL tracks unit-1 enable, binding, texels and coordinates per draw using the existing integrator lightmap sampler. All inherited container and controlled-lightmap pixels EXACT. General post-world menu/fog/last-coordinate handoff when no first-person pass runs remains open; title 197 one-shade pixels remains unresolved. |
| GameSettings | DONE: retained options/defaults/load-save/10,000 rebinding event check EXACT; original global binding construction order and cross-instance identity additionally checked by Mode_keyregistry181. |
| GuiControls | DONE: All three controls scenes EXACT, including mouse rebinding and duplicate bindings; Mode_options181 event trace. |
| ContainerFurnace | DONE: containerreplay: furnace transfer rules and click results EXACT. |
| ContainerWorkbench | DONE: containerreplay: crafting/transfer rules and click results EXACT. |
| ContainerChest | DONE for vanilla single/double chest container behavior: retained 66,146 container replay records; Mode_chesthooks181 / chesthooks181 matches 1,200 ticks across five layouts, open/replacement/close, slots/limits, users, lid interpolation, sounds and world RNG (783,025 bytes). Uses existing world-blocks integration, with no tile-side edits. |
| StatBase | DONE for the b181 vanilla class delta (method/field renames): both decompilations audited. Retained Mode_options181 / --playerguicheck proves the complete vanilla registries, IDs/names/types and 15,072 formatter outputs EXACT; statistics synchronization exercises original stat objects. Runtime mod stat registration is outside the vanilla catalog. |
| GuiParticle | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. Integrator owns this class; not edited here. |
| GuiSelectWorld | NOT DONE: b181 saved GameType labels added; all four select-world screenshots pixel EXACT in the full GUI run. Full callbacks/method audit still remains. |
| GuiSlotStats | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryLargeChest | NOT DONE: actual BlockChest activation checks all four double-chest directions, slot order/limits and delegated open/close exactly in Mode_chesthooks181. Arbitrary constructor null/alias cases and full interface methods remain unaudited. |
| GuiRenameWorld | NOT DONE: source delta is renames; new whitespace-only name/disabled Enter capture EXACT. Complete original rename/save callbacks and failure behavior remain. Integrator owns this class; not edited here. |
| SorterStatsItem | DONE: original class diff audited; Mode_statsorder181 / --statsordercheck calls actual original comparator. 20,000 combined item/block comparisons EXACT across every column/direction, missing stats, ties and signed wrapping overflow. Production sorting uses this same comparator. |
| GuiInventory | NOT DONE: all 35 inherited container captures and focused lightmap captures now EXACT after unit-1 state port. Full-sequence hurt preview still uses fixed brightness instead of the original entity brightness (4,726 pixels/max 22, figure only). Creative initialization/update redirect callbacks still lack a complete original-bytecode replay. |
| SorterStatsBlock | DONE: original class diff audited; Mode_statsorder181 / --statsordercheck proves all columns/directions, missing stats, ties and Java wrapping subtraction/multiply in 20,000 combined item/block comparisons. Block icon rendering differences remain under GuiStats. |
| ContainerPlayer | DONE: containerreplay: player crafting/transfer rules and click results EXACT. |
| GuiSlot | NOT DONE: source audit identifies the new protected mouse-coordinate fields assigned by drawScreen; current Rust does not retain them. Vanilla single-player lists do not consume them. Full original list/input callback replay remains. Integrator owns this class; not edited here. |
| GuiSlotStatsBlock | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| GuiVideoSettings | DONE: b181_video_gamma pixels EXACT; gamma option labels checked by Mode_options181. |
| GuiSlotStatsItem | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. |
| InventoryCrafting | DONE for the b181 delta (no-op open/close and row/column naming): source diff audited. Mode_inventoryinterfaces181 checks 800 original matrix operations including event-handler recipe/result recomputation; 2,400 combined inventory/Slot operations EXACT. Retained complete recipe/container replay also EXACT. |
| GuiTexturePackSlot | NOT DONE: source audit finds the b181 exception fallback to the default texture pack during selection/refresh. Current Rust refresh does not implement the complete Java fallback transaction. Original-bytecode fallback test and port remain. Integrator owns this class; not edited here. |
| GuiSavingLevelString | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. Integrator owns this class; not edited here. |
| GuiTexturePacks | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. Integrator owns this class; not edited here. |
| StatFileWriter | NOT DONE: totals, unsent maps, wrapping i32 arithmetic, achievement presence/parents, sync/tick/readiness and valid/checksum-invalid/invalid-syntax file recovery match 2,124 original snapshots. Disk maps are normalized by stat id; raw JSON key order follows JVM identity hashes and is not byte-identical. Arbitrary malformed schema and all serialization entry points still need audit. |
| CraftingManager | DONE: b181 recipe table regenerated from original; all 161 recipes exercised in containerreplay, EXACT. |
| ThreadStatSyncherReceive | DONE: actual original thread executes both sent-file read and pending-map write branches; normalized disk/state snapshots, tick consumption and busy clearing match in Mode_statsync181. Uses the shared Syncher worker protocol. |
| GuiYesNo | NOT DONE: source delta is field renames; new original-client confirmation and cancel captures EXACT, including returning to Select World. Full callback protocol with arbitrary parent/IDs remains unproven. Integrator owns this class; not edited here. |
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
| GuiChat | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. Integrator owns this class; not edited here. |
| GuiOptions | DONE: b181_options and b181_options_fov pixels EXACT. |
| GuiWorldSlot | NOT DONE: b181 changes have not been ported and separately proven EXACT in this stage. Integrator owns this class; not edited here. |
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

The largest remaining work is the complete original Minecraft runTick/lifecycle replay, base/survival controller lifecycle methods, title panorama driver precision, general post-world GUI/fog/lightmap handoff, GuiInventory creative redirects and arbitrary stat-file schema/serialization. Other uncompleted interfaces and screens are explicitly listed above. The integrator owns GuiIngame plus GuiYesNo, GuiRenameWorld, GuiChat, GuiWorldSlot, GuiSavingLevelString, GuiSlot, GuiTexturePacks, GuiTexturePackSlot and GuiParticle.

No b173 crate/oracle/verifier was edited. No world renderer/lightmap algorithm, entity renderer/model/particle implementation, chest tile, save or terrain algorithm was rewritten. Changes use the existing owner APIs. Git metadata was not written; target/commit-b181-player-gui6.sh stages explicit paths with the requested author and both trailers.
