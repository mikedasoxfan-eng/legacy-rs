# GuiInventory b1.8.1 delta and round-nine framebuffer audit

Both complete GuiInventory decompilations were compared with `git diff --no-index`: tools/retromcp/minecraft/src/net/minecraft/src/GuiInventory.java and tools/retromcp-b181/minecraft/src/net/minecraft/src/GuiInventory.java. The class has these four changes; its other methods are unchanged.

| Original change | Production behavior and original-client evidence |
| --- | --- |
| Constructor field `field_948_f` becomes `allowUserInput` | Existing screen policy allows input and retains the constructor's open-inventory achievement. inventorygui181 records actual original constructor/init state over 10,000 cases. |
| New updateScreen creative redirect | The existing inventory screen queries the controller mode on each update and displays creative inventory. inventorygui181 compares 30,000 updates, including mode changes, repeated callbacks and death retention. Round-seven transition captures remain pixel EXACT. |
| initGui creative redirect after clearing controls | Existing production initialization preserves ordering and an empty control list. The 10,000 original init cases and both transition captures are EXACT. The retained 50,000 button callbacks prove unchanged button behavior. |
| Remove both entityBrightness writes around the preview render | Production consumes the owner's supplied PlayerPreview brightness rather than overriding the entity. All 16 round-seven brightness/hurt captures remain pixel EXACT. Round-nine adds 18 death/light cases, four armored deaths with a held tool and eight successive mouse-position/death captures; all 30 are pixel EXACT. |

The previously uncredited death differences were in the GUI adapter's fixed-function lighting and item matrix submission. Lighting must multiply the material before the final color clamp. A sprite item disables GL_RESCALE_NORMAL before the later hurt/death overlay; the block item branch retains it. Held sprite vertices now remain in object space through the GPU's combined MVP, as in the original Tessellator; the adapter uses the existing Gl degree rotations. These edits are limited to gui/screens/inventory_model.rs and inventory_equipped.rs. Existing entity models, renderer and particle implementations are unchanged.

The original callback fixture uses the actual original GuiInventory methods, including allow-user-input, achievement, controls, redirects and pause/death behavior. Its existing 940,000-byte record is regenerated, not replaced by a Rust-only expectation. The new pixel scenes render the original client and production Rust GUI at 854 by 480, using GUI_MSAA=1 and a zero-tolerance comparator. This completes the vanilla singleplayer GuiInventory delta; arbitrary world renderer state handoff remains a separate RenderEngine/Minecraft concern.

## Panorama/framebuffer behavior

Original GuiMainMenu's `func_35355_b` and blur methods draw into the bottom-left 256-square viewport of the existing framebuffer, then copy that framebuffer into the blur texture. They do not allocate a fresh cleared framebuffer for each pass. The first cube sample inherits the color mask and depth-test state; subsequent cube/blur samples suppress alpha writes. The final crop restores all four color channels and the source depth/alpha state.

Production now follows that sequence on the actual target. It retains framebuffer alpha, copies the bottom-left region, preserves depth between passes and obeys GL_COLOR_WRITEMASK for clears. GL_DST_ALPHA is implemented through the same production pipeline. RGBA and BGRA targets are both checked through --guiframebuffercheck against the 80 original GPU pixels already recorded in texture-fx.bin; the framebuffer copy is independently required to preserve each pixel. No palette correction, tolerance or source-image replacement was added.

An optional GUI_CAPTURE_ALPHA observation writes framebuffer alpha sidecars in both capture harnesses. Java only reads the existing glReadPixels buffer; it does not alter source GL state or original rendering. All 98 captures in the round-nine focused suite compare their alpha bytes exactly (40,172,160 bytes). Feature RGB is deliberately still checked by verify_gui181.sh: the focused alpha check does not claim its title RGB is EXACT. The remaining plain-title and anaglyph-title one-level differences keep GuiMainMenu and RenderEngine NOT DONE.

The feature scene now supplies the existing generated saves directory, matching its final `open rename World1` instruction. Without that input Rust correctly fails on missing WorldInfo. Original save/world implementations were not changed.

The live app's world-to-GUI mask bridge is not edited: app.rs/Minecraft has a separate worker. That owner must supply the inherited Gl.color_mask before begin_frame. The capture fixture supplies the original observed mask and frame timing; this proves production renderer behavior without claiming the whole live-world handoff complete.

The framebuffer check also runs the original uniform-color cases at 160 by 200, in both framebuffer orientations, comparing the defined copied pixel at the same coordinate. The viewport crops clip coordinates to preserve the 256-pixel mapping within a smaller attachment; out-of-window copy texels remain WebGPU-initialized zero (OpenGL leaves those pixels undefined). Actual panorama passes are smoke-tested at 256 by 256 and 160 by 200 in RGBA/BGRA. This detects invalid viewport/copy requests without claiming original small-window panorama pixel equivalence.
