# Release 1.0 rendering, round 4

All 16 classes of this round are Done: each has its complete 1.0 delta ported and an EXACT
original-bytecode check in `tools/verify_r10render4.sh` (28 sections, all EXACT, registered in
`tools/verify10.sh` before the final `pflush` and in `verify10.sh render`). The round started
from the cut-off worker's WIP commit (End portal port, glint draft, ActiveRenderInfo draft, GL
stub hooks); all of it was reviewed, kept where correct and finished. b1.8.1 (renderfx and world
round 4) is merged in; the merge was clean.

## Harness

- `GlLog10` (tools/oracle10/src): opt-in log of GL calls the shared recorder does not keep
  (texgen, texture-matrix stack and the texture matrix of every draw, lights, fog, colour, depth,
  GL13 texture units). The headless `GL11`/`GL13` stubs feed it; the End portal fixture moved to it.
  Texture-matrix calls no longer touch the modelview recorder. gui2/gui3 and render2/render3
  re-run EXACT after the stub change.
- `RendererOracle.manager`, the weather, RenderManager and TileEntityRenderer fixtures now call
  `OpenGlHelper.initializeTextures()` as `Minecraft.startGame` does; without it 1.0's
  `setActiveTexture(0)` left the recorder on a lightmap unit and dropped texture binds and lightmap
  coordinates. The glint texture (`%blur%/misc/glint.png`) has a fixed id (76) in the fixtures.
- New fixtures: `Mode_render_round4_10` (gui, activeinfo, fog, lightmap, glhelper, renderhelper,
  shadow, tessellator, smooth, auxsfx), `Mode_render_biped4_10`, `Mode_render_child4_10`,
  `Mode_render_worldrenderer4_10`; FirstPersonOracle kinds 12 (RenderPlayer 1.0), 13
  (ItemRenderer.renderItem with entity and pass), 14 (getMouseOver creative/fireballs), third-person
  front view and nausea in kind 0. `System.currentTimeMillis` glint offsets are read back from the
  recorded texture matrices / glint UVs and replayed into the Rust renderer.

## Class table

| Class | Status | Port and checks |
| --- | --- | --- |
| RenderGlobal | Done | End sky, sunrise/sun/moon-phase transforms and horizon (round 3 port, re-checked), clouds without the time term, `func_40193_b` particle factory with the Particles setting (Decreased re-rolls on World.rand, Minimal skips), new particle names, playAuxSFX 1007-1009 ghast sounds, 2002 potion splash (iconcrack, 100 tinted spells, glass sound), 2003 ender eye, 2004 spawner flames (World.rand and Math.random replayed; potion ring uses Java Math.sin/cos), `thirdPersonView != 0` entity pass. Unreachable in game: the `updateRenderers` sort branch (`var3 = false`), occlusion/display-list allocation sizes (GL resources only), `TileEntityRenderer.func_40742_a` (empty). Checks: `RenderGlobal_dimension_sky_end_moon_horizon`, `RenderGlobal_sky_gl_colors_sunrise`, `RenderGlobal_clouds_fancy_fast_anaglyph`, `RenderGlobal_playAuxSFX_spawnParticle_setting`, `RenderGlobal_particle_factory_names`, `FirstPerson_camera_fov_hand_glint_player_poses_targeting_fog_maps` (selection box and block breaking) |
| EntityRenderer | Done | updateFogColor/setupFog 1.0 (ActiveRenderInfo eye block, blindness colour/distance, water breathing density, `hasNoSky` void fog, cloud fog, sunrise fog along x), End lightmap (`worldType == 1`), smooth camera (per-tick MouseFilter, per-frame partial delta), third-person front view (orientCamera and probes), nausea wobble 7 deg/tick, FOV water test via ActiveRenderInfo, getMouseOver extendedReach 6 and fireball collision border 1.0 (worldgen10 picking.rs), renderRainSnow by temperature (>= 0.15 rain, z-major biome/temperature order) and addRainParticles with the Particles setting and cache temperature > 0.2; production fog/FOV wired to the new state. Checks: `EntityRenderer_fog_blindness_breathing_nosky_cloud`, `EntityRenderer_lightmap_end`, `EntityRenderer_lightmap_overworld_nether`, `EntityRenderer_smooth_camera`, `EntityRenderer_rain_snow_temperature_particles`, `FirstPerson_camera_fov_hand_glint_player_poses_targeting_fog_maps`, `ActiveRenderInfo_unproject_view_vectors_eye_block` |
| RenderItem | Done | Dropped potions (tinted 141 + bottle, shared copy Random), GUI potion passes, `field_40268_b` zLevel for icons/blocks/glint (container uses it instead of a translate), untinted path, GUI glint (GL_GREATER, DST_COLOR blend overridden to SRC_COLOR/ONE, time offsets, cull face), fireball icon 46 in the inherited check, mushroom-block and nether-wart item textures. Checks: `RenderItem_gui_potion_zlevel_untinted_glint`, `RenderItem_dropped_gui_overlay_renderers`, `RenderItem_pickup_renderers` |
| ItemRenderer | Done | `renderItem(entity, stack, pass)`: `getItemIcon` per holder (player cast rod, potion pass 0 liquid 141, bow pull frames; mobs use the plain icon), glint over the extruded sprite with its texture matrices, potion second pass, item brightness 1.0, drink action, `isEntityInsideOpaqueBlock` 0.8 width (overlays). Checks: `FirstPerson_camera_fov_hand_glint_player_poses_targeting_fog_maps` (kinds 1, 10, 13), `RenderBiped_held_potion_passes_glint` |
| RenderLiving | Done | Child flag on main/pass models, tripled baby limb swing, `shouldRenderPass` ints with the pass-15 armour glint (two texture-matrix passes, depth EQUAL, no depth write), `inheritRenderPass >= 0` overlays, `func_40270_a`, OpenGlHelper units. Checks: `RenderLiving_baby_animals_child_models_limbs`, `RenderLiving_mob_renderers_passes_overlays`, `RenderLiving_ghast_pigzombie_giant`, `FirstPerson_camera_fov_hand_glint_player_poses_targeting_fog_maps` (kind 12) |
| RenderEndPortal | Done | 16 seeded layers, tunnel/particlefield textures and blends, ActiveRenderInfo parallax translation, object/eye-linear texgen planes, texture-matrix stack, state restore; production draws with projective UVs from the frame's ActiveRenderInfo. Checks: `RenderEndPortal_layers_texgen_parallax` |
| RenderPlayer | Done | Armour returns 1/15 (glint), held pose 1/3 while blocking, aimed bow, 1.0 sword-block rotation, bow transform, potion colour passes, holder-aware item icons. Checks: `FirstPerson_camera_fov_hand_glint_player_poses_targeting_fog_maps` (kinds 3, 11, 12), `RenderPlayer_player_living_round4`, `RenderPlayer_player_living_round6` |
| ActiveRenderInfo | Done | LWJGL gluUnProject (row-pivot comparison bug included, failure keeps the old point), view vectors with the front-view sign, eye point from prevPos, fluid-surface eye block; updated after each camera transform in production (FOV, fog, portals). Checks: `ActiveRenderInfo_unproject_view_vectors_eye_block` |
| OpenGlHelper | Done | Units 0x84C0/0x84C1, ARB selection only without OpenGL 1.3, GL13 calls for active/client unit and lightmap coordinates (`render/opengl_helper.rs`). Unreachable on a modern driver: the ARB entry points themselves. Checks: `OpenGlHelper_OpenGlCapsChecker_units_capabilities` |
| RenderHelper | Done | Standard item lights (float-rounded Vec3D.normalize positions), `func_41089_c` GUI rotation, disable; GUI and entity shading use the shared positions (`render/render_helper.rs`). Checks: `RenderHelper_item_and_gui_lighting` |
| RenderBiped | Done | Potion second `renderItem` pass for mobs, glint on held items. `field_40296_d` is stored and never read (unobservable). Checks: `RenderBiped_held_potion_passes_glint` |
| Render | Done | Shadow radius halved for baby animals (`EntityAnimal.func_40127_l`). Checks: `Render_shadow_baby_animals`, `Render_shadow_fire` |
| Tessellator | Done | `draw()` byte count, packed vertex words, lightmap client-unit calls through OpenGlHelper. Unreachable: the VBO path (`tryVBO` is false). Checks: `Tessellator_draw_bytes_brightness_units` |
| WorldRenderer | Done | `updateRenderer` per section: y/z/x scan, pass loop with early break, list creation, skipRenderPass, special tile-entity list, isChunkLit, section-relative origin (`render/world_renderer.rs`); `bytesDrawn` is write-only. Checks: `WorldRenderer_sections_passes_skip_tiles` |
| OpenGlCapsChecker | Done | Static `checkARBOcclusion`. Checks: `OpenGlHelper_OpenGlCapsChecker_units_capabilities` |
| MapItemRenderer | Done | The added `glBlendFunc(ONE, ONE_MINUS_SRC_ALPHA)` before the map quad. Checks: `FirstPerson_camera_fov_hand_glint_player_poses_targeting_fog_maps` (kinds 1 and 9) |

## Inherited render sections (release gate)

Re-run on this branch after the b1.8.1 merge with the final binary (same commands as verify10): 49 of 49 EXACT.

| Section | Result | Fix |
| --- | --- | --- |
| chunk meshes: seed 25/1/-1/0, all four graphics/AO modes (16) | EXACT | 1.0 BlockTallGrass tints with its own column's biome colour (b1.8.1's +-16 jitter removed); 1.0 BlockFluid tints water with the biome water colour (swamp 0xE0FF70) |
| block showcase (8) | EXACT | - |
| mob models: boxes, UVs, successive poses and wolf living animations / entities10: models10 | EXACT | check reads every ModelBox; 1.0 field names (silverfishBodyParts/Wings, chestLid/Knob/Below) |
| remaining entity renderers and GUI item geometry | EXACT | RenderItem potion passes; fireball icon 46 in the check |
| ghast, pig zombie and giant full renderer draws / entities4: livingrenderers | EXACT | texture units initialised in the fixture |
| entities4: generaliving4_181 | EXACT | fixture set `field_40139_a` (slime target) instead of the rendered `field_768_a` |
| entities4/6: playerliving4_181, playerliving6_181 | EXACT | - |
| first-person view, player and block targeting | EXACT | see EntityRenderer/ItemRenderer rows |
| ambient weather vertices | EXACT | 1.0 renderRainSnow/addRainParticles; texture units in the fixture |
| entities9: RenderManager registry, camera and dispatch | EXACT | 1.0 front-view `cacheActiveRenderInfo` (options read, +180); fixture lists the 1.0 renderers |
| entities9: TileEntityRenderer class cache, active context and dispatch | EXACT | fixture hashes the book/End portal classes; their renderers keep the empty onWorldChange |

## Changes outside the area (smallest needed)

- `crates/worldgen10/src/entity/player/picking.rs`: getMouseOver reach 6 for creative, fireball border 1.0.
- `crates/client10/src/gui/screens/container.rs`: item zLevel passed to RenderItem, glint draw state; `crates/client10/src/gui/mod.rs`: `%blur%/misc/glint.png` added to the GUI texture list.
- Fixtures owned by other workers, harness-only: `Mode_generaliving4_10` (slime field), `Mode_rendermanager9_10`, `Mode_tilemanager9_10` (texture units, 1.0 classes), `Oracle.weatherMesh` (texture units, z-major temperatures, Particles argument).

## Validation

- `bash tools/verify_r10render4.sh --built`: 28 sections, R10 RENDER ROUND 4 EXACT (WorldRenderer: 4 x 200 sections, about 1.2M vertices).
- `verify_r10render2.sh`, `verify_r10render3.sh`, `verify_r10gui2.sh`, `verify_r10gui3.sh`: EXACT after the GL stub change.
- `cargo build --release` (workspace), `cargo test -p worldgen10`, `cargo test -p client10`: pass.
- `bash tools/oracle10/oracle.sh raw 0 0 0 0 0 t.bin`: compiles and runs.
- Seed-42 no-save 60-tick smoke screenshot `target/r10-smoke.png`: forest, hand, HUD and tutorial render normally.
