# GuiContainer b1.8.1 delta audit

Both original sources were diffed in `target/player-gui6-container.diff`.

| Changed part | Port and original-client evidence |
| --- | --- |
| Item renderer becomes protected | Existing shared `gui_item` implementation remains available to base and creative inventory screens. |
| `drawScreen` assigns unit-1 coordinates (240,240) | `screens/container.rs` assigns packed coordinates after the background/player figure. `gl.rs` retains enable/binding/coordinates, snapshots the existing lightmap sampler at each draw and passes the sample separately to the fragment shader; unit 1 modulates after the main texture. The original RenderLiving disable/enable sequence is represented in the GUI figure adapter, without modifying entity renderer/model code. |
| Slot/list and inventory method renames | Existing vanilla slot indices and container model are unchanged. `containerreplay` covers 66,146 cases and all 161 recipes. |
| New `func_35309_a` click dispatch | Base clicks retain vanilla slot-number/index identity; creative clicks use the existing creative container policy. All 35 inherited container captures, including actual right/shift/crafting-result clicks, are EXACT. Existing three creative captures and click replay remain exact. Runtime-mod slots with reassigned slot numbers are outside the vanilla catalog. |
| Extra `onCraftGuiClosed` call before the controller's close | New `CloseFromGui` command preserves both callbacks; direct controller/session `Close` remains a single close. `guiclose181` runs actual original GuiContainer callbacks over 384 cases/six kinds, checking cursor drops, grid contents, duplicate workbench drops and chest callback counts; 45,600 bytes EXACT. `guichest181` runs actual original GUI close through five real chest layouts, checking use counts/lids/slots/sounds/RNG/events; 1,200 ticks / 783,025 bytes EXACT. Replacement UI callbacks suppress the redundant old-window close as before. |

All 35 inherited container scenes are now zero-tolerance EXACT, both focused and in the full GUI verifier's container group. Sixteen extra focused captures use four nonzero controlled unit-1 textures on empty/filled/hurt inventory and chest screens; all are zero-tolerance EXACT. They prove coordinates, color modulation order, hurt-overlay disable and next-frame reset instead of relying on a black texture alone. In the full main sequence the four hurt captures retain a figure-only brightness difference (4,726 pixels/max 22); their labels, icons and backgrounds remain exact. The existing GuiInventory adapter's fixed hurt-overlay brightness is separately NOT DONE and is not a GuiContainer source delta.

The live caller uses the existing world lightmap and packed brightness adapter. General post-world menu/fog state and last unit-1 coordinates when no first-person pass runs remain a RenderEngine/Minecraft integration gap; these class rows are not credited by the container delta check.
