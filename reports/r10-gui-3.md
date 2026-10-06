# Minecraft 1.0 GUI, round 3

The 22 classes assigned in b181-r10gui.md now have their complete GUI/container/recipe release delta and original-bytecode comparisons. No commit was made. Protected Beta crates, oracles and verification scripts were not changed.

The merged tools/oracle10 is the base harness. GUI fixtures live outside src in tools/oracle10/gui3 because they require instrumented headless GL. tools/r10/gui3_oracle.py compiles fixture and instrumentation classes into target/oracle10-gui3, requires Mojang Java 8u51, and places the original release jar before the base build classes. It never compiles decompiled game sources. The obsolete harness-worktree fallback was removed from gui2_oracle.py. The new tools/verify_r10gui3.sh creates its output folders, emits a == section for every check, collects failures without set -e, and is registered before the final pflush in tools/verify10.sh.

Changes include the shared ModelBook in the enchanting screen, exact offer and book camera inputs, the inventory-preserving singleplayer End return, one canonical boss overlay, slot/cursor item depths, tooltip text depth, release GUI lighting, dynamic texture upload order, the release creative catalog, brewing inventory setter behavior, enchanting close drops, progress synchronization, the default container enchant hook, and the two crafting achievement additions.

| Class | Done / Not done | Exact checks and result |
|---|---|---|
| GuiEnchantment | Done | enchantmentdraw: 1,000 camera/viewports/model pose/offer/font/foreground traces; enchantclick: 1,000 button/bounds/cost/XP/item controller dispatches. Regression enchantment: 10,000 animation ticks, stack changes and RNG, plus names. Uses the existing render-owned ModelBook. |
| GuiWinGame | Done | winflow: 1,000 scroll, centered credit, last-line freeze, text-seed, completion, escape and pause cases. Regression win: real poem/credits parsing, wrapping, heights, resize and backgrounds. endcopy: 200 original copy/prepare cases, including inventory, armor, food, XP, score, dimension and health reset. PlayerEvent::WinGame opens the real screen; finish routes EndReturn through the GUI/system/app/worker to dimension 0 with keep=true. |
| ContainerEnchantment | Done | enchanting: 600 original constructors, bookshelf layouts, remote/local costs and seed/RNG, item tags, all offers, XP and progress cases. containers: transfers and repeated close callbacks. lifecycle: 2,400 real slot puts/pickups with exact RNG and offers. progress: listener ordering, repeated broadcasts, invalid IDs and costs. |
| GuiContainer | Done | containerdraw: 400 actual original drawScreen traces, random 45-slot inventories, stack counts/damage/enchantments/potions/records, hover borders, held cursor, item depths 100/200 and tooltip gradients/text. Cached origins and existing background/foreground behavior retained. |
| RenderEngine | Done | dynamic: 200 eight-effect sequences, three updates each; exact ticks, anaglyph propagation, icon binding reuse, obsolete mipmap/texture-ID removal, tile order, buffer state and all uploaded bytes. Additional renderengine10_181 and renderengine11_181 regressions passed for cache/refresh/filter/buffer behavior. |
| ContainerBrewingStand | Done | containers: 160 constructors/layouts, every transfer slot, overflow inventory setters, pickups and repeated close. progress: 1,000 changing/unchanged brew time cases, 0..4 listeners, shared cache and progress ID filtering. |
| CraftingManager | Done | recipes: ordered registry of all 174 recipes, every ingredient/output/metadata/dimension, 2x2/3x3 matching, mirrored and wildcard variants, and repair arithmetic including overflow boundaries. |
| ContainerCreative | Done | creative: exact 265-item release catalog, 81-slot layout, 500 scroll positions including clamps, default enchant rejection and production transfer fallback. Fixed missing release blocks/leaf/tall-grass variants and excluded potion items. |
| SlotBrewingStandPotion | Done | slots: every registered item acceptance, limit 1 and six potion damage pickup achievement cases. containers covers all three original potion slots and transfers. |
| GuiContainerCreative | Done | creativegui: 500 real constructors/init/draw calls, cached origins even after width/height changes, empty controls and scrollbar drawn in the background before items. creative covers the inventory palette and row scroll contents. |
| SlotEnchantmentTable | Done | lifecycle: original Slot.putStack/decrement notifications with clamp 1 and exact nextLong, name seed and offer state. enchanting also drives direct inventory changes. |
| SlotBrewingStandIngredient | Done | slots: original acceptance across every registered item and limit 64; containers verifies storage and transfer behavior. |
| SlotEnchantment | Done | slots: accepts every registered item, inherited inventory limit 1; enchanting/containers verify the original 25,47 slot layout and item behavior. |
| RecipesFood | Done | recipes: full ordered release recipe registry and matching, including release food additions. |
| SlotCrafting | Done | slots: all ten crafting achievement outputs, including enchantment table and bookcase, plus a non-achievement diamond output. Added the two missing release achievements. |
| GuiStats | Done | lighting: 100 real original drawItemSprite calls. Replaced the incorrect prior 120-degree X approximation with -30 degrees Y followed by 165 degrees X, shared with the other release GUI callers. |
| GuiAchievements | Done | lighting: 100 real original achievement drawScreen calls against the release registry and exact helper rotation sequence. |
| GuiAchievement | Done | lighting: 100 real original updateAchievementWindow calls and exact release helper rotation sequence. |
| FurnaceRecipes | Done | recipes: original smelting result lookup for IDs -2..2259, including coal/redstone/lapis additions. |
| Container | Done | creative drives the production default enchant rejection and inherited transfer behavior; containers/lifecycle exercise base slot callbacks, close and transfers. The shared enchant hook is also used by the world-thread session bridge. |
| RecipesIngots | Done | recipes: ordered gold ingot/nugget registration and all matching/output metadata in the full registry. |

The boss regression now invokes render_release::boss_overlay itself, captures its actual rectangle/text calls, and verifies consumption. GuiIngame's duplicate dragon_rendered/render_boss/take_boss_rectangles path is removed. hud also exercises that same canonical overlay while checking the rest of the survival/creative HUD.

Exact output sizes for the new modes:

| Check | Bytes |
|---|---:|
| enchanting | 600,080 |
| containers | 5,234,916 |
| creative | 345,923 |
| slots | 892 |
| recipes | 33,213 |
| enchantmentdraw | 668,412 |
| winflow | 241,934 |
| dynamic | 16,947,588 |
| containerdraw | 1,374,324 |
| lighting | 10,800 |
| hud | 1,391,702 |
| creativegui | 32,000 |
| lifecycle | 108,000 |
| endcopy | 133,904 |
| progress | 207,952 |
| enchantclick | 5,456 |

Minimal edits at other ownership boundaries:

- render/entity/gui_item.rs supplies the shared release GUI light transform and optional verifier capture; render/entity/render_release.rs exposes the canonical dragon handoff/consumption used by the HUD and checks. No alternate dragon/book renderer was created.
- world/dimension.rs adds the keep-inventory respawn argument and calls the existing EntityPlayer copy method before prepare-to-spawn. worker.rs switches the same save to the Overworld for EndReturn without creating a Nether portal.
- entity/player/registry.rs registers existing release item definitions 369..371 so the creative catalog and ingredient acceptance can include blaze rod, ghast tear and gold nugget. No other entity logic changed.

Validation:

- cargo build --release: PASS, whole workspace, exit 0; target/build-r10gui3-final.log.
- cargo test -p worldgen10: PASS, 80 unit/integration tests plus doc tests, exit 0; target/test-r10gui3-worldgen-final.log.
- bash tools/verify_r10gui3.sh --built: final result recorded in target/verify-r10gui3-final.log and target/r10gui3-final-results.json.
- bash tools/oracle10/oracle.sh raw 0 0 0 0 0 t.bin: final result recorded in target/oracle-r10gui3-raw-final.log. All Java src fixtures compile together.
- Required seed-42, no-save, 60-tick startup screenshot: target/r10-smoke.png, inspected; terrain, hand, hearts, food, hotbar and inventory hint render correctly.
- Additional off-screen enchanting and poem scenes: target/r10gui3/scenes/enchantment.png and credits.png, inspected. The book is visible and offers/items/credits render. These are visual smoke checks, not original GPU pixel comparisons; detailed ModelBook/RenderItem geometry remains render-owned.
- RenderEngine round-11 regressions: both renderengine10_181 and renderengine11_181 returned zero differences. The broader verify_player_gui11_10.sh also found one StatFileWriter malformed-input mismatch: statfile11_181 case 397 (valid stats JSON followed by trailing k), expected throw:Nonterminating, actual empty result. StatFileWriter was not changed in this task. Its malformed-input error path is outside the assigned classes and the common task's required reachable-game behavior. This broader suite returned failure, so it is not reported as passing.

The End adapter has exact finish/copy/prepare coverage and real singleplayer wiring. This suite does not independently compare complete dimension terrain loading or provide multiplayer transport, which are existing world/client boundaries. Java-only API identity, unobserved null inputs, IO/exception paths and scheduling are excluded according to r10-common.md.

The master verify10 suite was registered but not run in full; the new GUI suite and the relevant RenderEngine regressions were run. git diff --check and shell syntax checks passed. target/commit-r10-gui.sh stages explicit changed/new source, fixture, suite and report files with the required author and co-author trailers; it was not executed.
