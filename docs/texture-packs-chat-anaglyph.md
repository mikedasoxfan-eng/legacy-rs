# Texture packs, chat and anaglyph fidelity

The client reads resources from a selected ZIP before the user's Beta 1.7.3 jar. Packs
are scanned in filesystem order under `texturepacks/` next to `options.txt`; `skin`
is persisted by the original selection flow. `pack.txt` uses Java 8's Windows-1252
encoding, its first two lines are truncated to 34 UTF-16 units, and malformed ZIPs
remain listed with the unknown thumbnail. Cache identity is filename, length and
last-modified milliseconds. A missing saved pack falls back to Default without
rewriting the saved name. Replacing a selected archive creates an unopened cache
object, as in vanilla; switch away and back to open it.

Selection refreshes GUI images, the terrain/item atlases, all entity/armor/environment
images, raw grass/foliage colormaps, thumbnails and procedural animations. The worker
receives new colorizers and rebuilds its meshes. The same device and texture identifiers
remain usable by open screens. Thumbnail GPU slots are reused when the list changes.
The folder button opens the scanned directory in Explorer.

Vanilla FontRenderer reads `font/default.png` directly from the game jar. Packs therefore
cannot replace its glyph bitmap or widths. Anaglyph texture refresh converts that allocated
bitmap, while its 32 color display lists retain the palette constructed at startup.
Compass, watch and dial source bitmaps also come directly from the jar. Animation instances
persist across world and pack changes, tick in registration order, and share Java's
Math.random stream. Flow/fire overflow and all float/double casts follow the bytecode.

GuiChat accepts the original allowed characters, caps the message at 100 UTF-16 units,
handles backspace, and blinks its cursor every six screen ticks. Keyboard autorepeat
is enabled for Chat, Create World and Rename World, and disabled for the other screens,
as checked against the original LWJGL state. Escape and Enter close it;
EntityPlayerSP.sendChatMessage is empty, so Enter adds no local message. The screen inherits
vanilla's pause behavior. **T in singleplayer is an explicit requested extension:** the original
Minecraft.runTick gates the chat key on multiplayer. F11, F3+S sound reload, and F2's local-date
screenshot names, collision suffixes and chat feedback are implemented as well.

Stereo renders cyan eye 0 and red eye 1 into one shared color attachment, clearing depth
for each eye and for its hand. Pipeline variants implement the actual write masks, including
masked fog clears, alpha preservation before the Fancy water/cloud passes, and those passes'
alpha writes. GUI/HUD draw once after both eyes. Camera projection/modelview operations
are generated separately for each eye. This preserves destination-alpha blending, which
cannot in general be reproduced by combining two completed eye images.

Anaglyph branches were audited in EntityRenderer, RenderEngine, RenderGlobal, FontRenderer,
RenderBlocks and MapItemRenderer. RenderBlocks converts standard-block, crossed-plant and
cactus tint multipliers; fluids retain raw tint. Sky/cloud/fog conversions use float arithmetic,
texture/font/animation conversions use integer arithmetic. Only the sunrise fan center is
converted; its zero-alpha rim retains raw RGB. The original occlusion-query branch is disabled in anaglyph mode, including its
unreachable mask restoration. Particle, item, other entity and selection-outline
renderers have no additional anaglyph RGB branch: they use converted textures and masked
stereo draws. The map palette already follows the original conversion.

## Verification

- `bash tools/verify.sh`: full existing suite plus block showcases in all four graphics/AO
  combinations with anaglyph off/on, Fast/Fancy cloud vertices with both settings, and
  512 actual RenderGlobal sky/void/sunrise draws (38,128 float/vertex comparisons).
- Existing first-person oracle: 18,396 cases and 8,116,436 raw-bit comparisons, including
  both-eye projection/modelview matrices, fog and map colors.
- `bash tools/verify_gui.sh`: 15 additional strictly pixel-identical scenes for pack listing,
  selection, title/HUD, typing/100-character limit/backspace/blink, converted GUI textures,
  startup font palette and its retention after refresh.
- The original RenderEngine's actual GL uploads are read back for all 16,777,216 RGB values,
  including alpha. Original TextureFX bytecode supplies 1,000 compass/watch frames covering
  the Overworld, Nether, missing world/player, plus 400 frames of all seven terrain animations.
- Eighty actual GPU/OpenGL pixel comparisons cover every color mask, masked clear and
  destination-alpha blend. Pack lifecycle/resource fallback and screenshot suffix/opaque-pixel
  tests run with `cargo test -p client`.
- A scripted running client was inspected with a selected generated pack, singleplayer chat,
  and stereo switched off and back on through Video Settings.

These checks establish exact captured inputs, animation pixels, new GUI pixels and mask
behavior. They do not establish identical complete world-frame rasterization between OpenGL
and wgpu. The eight pre-existing GUI item-lighting scenes still use the documented one-color-
level GPU rounding tolerance; all 15 new feature scenes use strict comparison.
