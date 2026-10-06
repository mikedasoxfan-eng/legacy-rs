# Minecraft 1.0 GUI, round 2

23 of the 45 assigned classes meet this round's Done rule. The other 22 are listed explicitly below with remaining work. No commit was made.

Read the task, r10-common.md and reports/r10-client-1.md. The original release and Beta source references were read from ../beta-rs/tools/retromcp-r10 and retromcp-b181 because this checkout has no local MCP sources. No AGENTS.md was found. No Beta crates, Beta oracles or Beta verification scripts were edited.

The checkout still has no base tools/verify10.sh or oracle10 harness. Only new Mode_gui2*10.java fixture extensions and Gui2Fixture10.java were added under tools/oracle10; no base harness was created or copied. tools/r10/gui2_oracle.py reads the separately built ../wt181-r10harness/tools/oracle10 without modifying it, or accepts ORACLE10_BASE. It prefers a merged local built harness when available, requires Mojang Java 8u51, and puts the original deobfuscated jar ahead of base build classes so recompiled game classes cannot shadow the original. Instrumented GL stubs and all outputs live under ignored target/.

Run bash tools/verify_r10gui2.sh --built. The suite creates folders with mkdir -p, emits one == section per check, collects failures and has no set -e. tools/r10/register_gui2_suite.py inserts its call before the final pflush once the separately maintained tools/verify10.sh merges. Running it here reports Deferred; creating that base file would violate r10-common.md. The master-suite append remains an integration step.

Validation:

- cargo build --release: PASS, exit 0 (target/build-r10gui2-final.log).
- cargo test -p worldgen10: PASS, exit 0 (target/test-worldgen10.log); all unit/integration/doc suites passed after the simulation bridges.
- bash tools/verify_r10gui2.sh --built: PASS, exit 0; all 20 sections EXACT (target/verify-r10gui2.log). Each mode compares complete little-endian output bytes, including raw float bits where applicable.
- beta.exe --mc 1.0 --seed 42 --no-save --shot target/r10-smoke.png --shot-ticks 60: PASS, exit 0. Screenshot inspected: ordinary terrain, player hand, hearts, food, hotbar and inventory achievement render, with no old Beta label. This smoke does not exercise the absent dragon/book renderers.
- tools/r10/gui2-scenes.txt provides additional GPU review scenes for title, hardcore death, brewing, enchanting, potion-effect inventory and the poem. PASS, exit 0 (target/gui2-scenes.log). All six PNGs in target/r10gui2-scenes were inspected. The enchanting scene has offers/items but no book, matching the documented renderer dependency. These scenes are presentation-only and create no saves; they are not original pixel-parity checks.

The only worldgen10 edits are container/mod.rs (enchantment name seed in ContainerSnapshot and Enchant request) and container/session.rs (route the active matching window/offer to the existing world-owned enchant operation and copy its item/cost result). Player hardcore/effects/level data is bridged through worker.rs and app.rs; simulation classes and recipes are not duplicated. App remembers the real save directory for hardcore deletion, closes/joins the worker, then deletes and returns to the title. F5 stores the release's three states; front camera geometry remains renderer-owned.

Priority remaining GUI integration is ModelBook rendering, RenderDragon handoff, complete GuiContainer item depth/tooltips and the End return adapter. GuiWinGame's temporary finish action is ordinary respawn and must be replaced with Minecraft.respawn(false, 0, true) before connecting PlayerEvent::WinGame. There is no claim of correct End completion/network behavior.

| Class | Done / Not done | Changes, checks, or remaining work |
|---|---|---|
| GuiEnchantment | Not done | Ported background/offers, alternate font, click dispatch, shared world-owned offer seed and item identity-sensitive animation. `enchantment` is exact for 10,000 ticks, RNG and 1,000 name triples. ModelBook geometry/GL setup belongs to render and is absent; full drawing, pose and offer/click bytecode coverage remain. |
| GuiWinGame | Not done | Added original jar poem/credits, substitutions, blank lines, wrapping, scrolling, centered credits, last-line freeze, seeded obfuscation and fade. `win` checks original parsing, heights, resize and 1,000 backgrounds. Full draw/update/finish coverage remains. PlayerEvent::WinGame is deliberately unconnected: the temporary finish bridge uses ordinary respawn, while the correct End return must preserve inventory and transfer to dimension 0. |
| ContainerEnchantment | Not done | Simulation is blocks-owned and already available. Added only snapshot name-seed and Enchant command/session bridges; no duplicate container. Complete original constructor/slots, costs, item mutation, XP, callbacks and transfer/close fixtures remain with the owner. |
| GuiContainer | Not done | Cached container origin, release foreground/background callbacks, tooltips with rarity/potions/enchantments/records, z=300 tooltip, 120-degree GUI item lighting and creative scrollbar ordering ported. `backgrounds`, `gradient`, `inventory`, `itemnames` check subsets. RenderItem slot z=100/cursor z=200 and complete tooltip/depth/item rendering bytecode coverage remain. |
| RenderEngine | Not done | Inherited cache/texture-pack implementation retained. The 1.0 updateDynamicTextures delta (binding reuse, removal of obsolete mipmap/secondary texture-ID passes) and an ordered original GL upload fixture remain; actual GPU upload code is in the render boundary. |
| ContainerBrewingStand | Not done | Uses the existing blocks-owned four-slot brewing container and snapshot progress. No simulation duplicated. Original constructor/slot rules/progress synchronization/transfer fixtures remain with blocks. |
| GuiIngame | Not done | 1.0 identity, hardcore heart UV, XP level text and one-frame boss handoff added. `boss` is exact for 1,000 watcher-health cases including negative/over-max health and consumption. RenderDragon must call GuiIngame::dragon_rendered only for a rendered dragon; renderer is absent here. Full release HUD draw/RNG/heart/XP original fixture remains. |
| FontRenderer | Done | `font`: all reviewed release deltas, mutable line height, recursive newline shadow behavior, formatting carry across wrapping and obfuscated glyph/color list generation. Four line heights, 16 strings, seven widths, both shadow states, 2,000 glyph/RNG traces; original resources and raw ordered command/RNG equality. |
| GuiBrewingStand | Done | `brewing`, `backgrounds`: shared container screen, alchemy texture, 176x166 origin, inventory/title labels, exact float-to-int brew progress and bubble table. Timers -10..1200 and randomized dimensions, plus background/foreground callback coverage. |
| CraftingManager | Not done | Recipe registration and matching are simulation/blocks-owned. GUI uses existing world-owned output. Complete release recipe registration order and matching oracle coverage remain with that owner. |
| GuiInventory | Done | `inventory`: cached init origin, creative redirect ordering and release potion-effect panels/icons/translations/amplifier/duration/spacing. 1,000 sizes/effect sets (0..19 effects), including negative durations/amplifiers. Actual player effects are supplied by the worker/App bridge in Java hash-bucket order. |
| GuiDisconnected | Done | `disconnected`: translated constructor with/null formatting arguments, button layout, centered labels, suppressed keys and main-menu action. 1,000 original layouts and callback cases. |
| GameSettings | Done | `settings`, `registry`: third-person integer default (F5 cycles 0/1/2 in the client input bridge), 17-option metadata/Boolean mapping, clouds eligibility, particle/cloud cycles/labels/persistence. 1,000 overflow-sensitive option cases and 32 real original load/saveOptions files. Front camera orientation is EntityRenderer-owned. |
| GuiGameOver | Done | `gameover`: hardcore one-button delete layout/title/info, normal respawn/title menu and session-null disabling, score color, suppressed keys, pause and action dispatch. 1,000 original cases. Delete bridge closes/joins the world before deleting its actual save folder and returning to the menu; physical filesystem deletion is not performed by the fixture. |
| GuiMainMenu | Done | `menustate`, `menudraw`: 1.0 labels, constructor random draw after splash selection, unchanged panorama counter update semantics, release gradients and rare Minceraft logo branch. Original 1,000 constructors/RNG, 10,000 layouts/keys, 40,000 updates, 60,000 actions; actual original decoration callbacks including branch/NaN boundaries. Panorama implementation is inherited from prior GUI rounds. |
| ContainerCreative | Not done | Simulation inventory/slot/transfer delta and original callback checks remain blocks-owned. GUI continues to use the existing container. |
| SlotBrewingStandPotion | Not done | Existing blocks-owned potion slot used. Original acceptance/limit/onPickup achievement fixture remains with blocks. |
| GuiVideoSettings | Done | `video`, `registry`: release cloud/particle controls, 32-bit far-distance warning and relayout only when GUI scale changes. 300 sizes/settings/bitness cases, with four original actions and labels/layout after each. |
| GuiCreateWorld | Done | `hardcore`: survival/hardcore/creative cycle, difficulty/hardcore labels and world settings, preserving name/seed on resize. 300 original typed/resize/launch cases, including actual WorldSettings passed to startWorld. |
| GuiContainerCreative | Not done | Moved scrollbar drawing into the background phase and retained shared cached container origin. Complete release item palette/scroll/constructor/drawing/click original coverage remains. |
| SlotEnchantmentTable | Not done | Existing blocks-owned slot used. Original callback to owner/cost recalculation fixture remains with blocks. |
| GuiSlot | Done | `slot`: event-queue wheel draining only while mouse button is up, sign reduction, integer half-slot step and scroll clamp. 1,000 lists over eight original frames, including held-button wheel batches, odd slot heights and slot hit testing. |
| SlotBrewingStandIngredient | Not done | Existing blocks-owned ingredient slot used. Original ingredient predicate and stack-limit fixture remains with blocks. |
| AchievementList | Done | `registry`: all 27 release achievements, IDs, coordinates, parents, icons, special flags, labels/descriptions and bounds compared with the original registry. |
| SlotEnchantment | Not done | Existing blocks-owned single-item slot used. Original item acceptance and limit fixture remains with blocks. |
| RecipesFood | Not done | Blocks-owned recipe additions already supplied by simulation; ordered original recipe fixture remains with that owner. |
| GuiWorldSlot | Done | `worldslot`: removed size-in-MB suffix and added hardcore override/color. 1,000 actual original drawSlot calls cover blank names, game types, conversion/hardcore precedence, signed sizes and label coordinates; date formatter uses the Rust UI pattern and the same local timezone. |
| EnumOptionsMappingHelper | Done | `settings`, `registry`: release option ordinal/Boolean switch behavior covered over all 17 options, including clouds and particles. |
| GuiOptions | Done | `hardcore`: disables/relabels difficulty only for an open hardcore world. 1,000 original layouts over hardcore/non-hardcore and open/closed world combinations. |
| SlotCrafting | Not done | Blocks-owned crafting output achievement additions and original pickup fixture remain with blocks. |
| GuiStats | Not done | Ported changed GUI item light rotation to 120 degrees. Complete original RenderHelper/GL state and item drawing coverage remains. |
| GuiAchievements | Not done | Ported changed GUI item light rotation to 120 degrees; uses the updated release registry. Complete original RenderHelper/GL state and achievement drawing coverage remains. |
| GuiAchievement | Not done | Ported toast GUI item light rotation to 120 degrees. Complete original RenderHelper/GL state and toast drawing coverage remains. |
| Gui | Done | `gradient`: release zLevel-aware gradient geometry, ordered positions and packed color. 1,000 random bounds/colors/z values against the original Tessellator, four vertices each. |
| FurnaceRecipes | Not done | Blocks-owned release additions; original input/output/XP registry fixture remains with blocks. |
| EnumOptions | Done | `registry`, `video`, `settings`: all 17 original ordinals, keys, labels, float/Boolean flags and release cloud/particle controls. |
| Container | Not done | Uses existing simulation container and only adds the minimal Enchant request bridge. Complete release delta and original constructor/callback/slot fixtures remain blocks-owned. |
| RecipesIngots | Not done | Blocks-owned release nugget/ingot additions; ordered original recipe fixture remains with blocks. |
| StatTypeTime | Done | `statformats`: 5,000 signed integer values, including min/max/zero, exactly match original formatted text and release two-digit decimal precision. |
| StatTypeSimple | Done | `statformats`: the same 5,000 original signed-value comparisons cover the complete formatting delta. |
| StatTypeDistance | Done | `statformats`: the same 5,000 original signed-value comparisons cover thresholds, negative values and release decimal precision. |
| GuiFurnace | Done | `backgrounds`: release background callback signature and retained burn/cook rectangles and foreground labels; actual original tile scaling sampled over randomized timers/dimensions. |
| GuiDispenser | Done | `backgrounds`: release background callback signature and retained dimensions/origin/texture rectangles/foreground labels against original methods. |
| GuiCrafting | Done | `backgrounds`: release background callback signature and retained dimensions/origin/texture rectangles/foreground labels against original methods. |
| GuiChest | Done | `backgrounds`: release background callback signature and retained chest/inventory rectangles/foreground labels over 1..9 rows and randomized dimensions. |

| Suite check | Exact output bytes |
|---|---:|
| font | 212,932 |
| enchantment | 422,686 |
| brewing | 72,500 |
| registry | 4,625 |
| inventory | 1,114,268 |
| gradient | 112,000 |
| backgrounds | 246,068 |
| settings | 117,234 |
| statformats | 411,556 |
| hardcore | 603,832 |
| slot | 96,000 |
| disconnected | 151,330 |
| win | 166,207 |
| menustate | 3,149,362 |
| menudraw | 1,776 |
| video | 965,600 |
| itemnames | 159,374 |
| boss | 116,672 |
| gameover | 234,420 |
| worldslot | 150,720 |

Commit nothing was respected. target/commit-r10-gui.sh stages 59 explicit touched/new files and uses the required author and both co-author trailers; it was syntax-checked with bash -n and was not executed. git diff --check passed. The staging list excludes target artifacts and all protected Beta areas.
