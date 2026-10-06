# Release 1.0 rendering, round 3

30 of the 83 assigned classes are Done under the complete-delta plus EXACT-original-bytecode criteria; 53 remain Not done. This round restores credit for the 29 round-2 classes and finishes RenderBlocks. RenderGlobal has a new exact sky check but remains incomplete. This is a partial area handoff.

## Shared oracle restoration

All ten parked Mode_render_* harnesses now live under tools/oracle10/src/net/minecraft/src and compile with the shared harness. They invoke the original jar's models, renderers and Tessellator. The old pending-render/headless endpoints are unused migration references. RendererOracle adds opt-in extended snapshots for primitive mode, per-vertex packed normals/colors, cull face and alpha cutoff; existing modes retain their original recording format and behavior. GL11 forwards to that shared recorder. No checked Minecraft model, renderer or Tessellator source is compiled.

The shared oracle.sh uses a javac argument file to avoid Windows command-line limits. Oracle dispatches the restored render_*10 modes. Both rendering suites are appended to verify10.sh before its final pflush. Its optional `render` selection executes those same suites after the workspace release build, so all 29 restored classes have now run EXACT through verify10 rather than only through a standalone script.

Original jar: tools/retromcp-r10/minecraft/jars/deobfuscated.jar. The oracle requires the launcher's Java 8u51. The exact draw comparisons cover original Tessellator's emitted packing as consumed by these renderers, but do not establish a complete Tessellator port or native OpenGL correctness. Light configuration and packed lightmap upload remain outside the extended model-draw format. Supplied texture identities, font metrics and world sky/time values are explicit external fixtures.

## RenderBlocks

Audited the release delta against both local Java versions. Retained the already ported lily pad, cauldron, brewing stand, End portal frame, dragon egg, crop, door, fence and fluid implementations. Regenerated the complete render-property table from original 1.0 bytecode, fixing stale chest normal-block behavior and mushroom metadata 14/15 textures. Corrected redstone-wire connectivity: direction -1 excludes ordinary power providers, and repeaters connect along both directions of their axis. Corrected lily-pad inventory tint even when that render type emits no inventory geometry.

`RenderBlocks_world_{fancy,fast}_{on,off}_{normal,anaglyph}` passes all eight variants. The fixture contains 9,043 blocks, every id 1..122 with all 16 metadata values, changed-type neighboring full/inset/transparent/matching blocks, AO diagonals, negative coordinates, and independent door hinge/power metadata. Each run checks 636 chunks and both passes, including every vertex's raw position/UV/color/light words, counts, order and EOF. Fancy produces 1,527,968 vertices per run; fast produces 1,526,864. The inventory delta is covered by `RenderBlocks_inventory_lily_pad_dragon_egg`: 128 brightness/tint/metadata cases, 88,704 comparisons, zero mismatches, including normals and final state.

## RenderGlobal sky progress

`RenderGlobal_dimension_sky_phases_transforms_horizon` passes 64 cases and 1,437,136 comparisons with zero mismatches. It invokes the original RenderGlobal constructor and renderSky, including original Tessellator/display-list geometry. Cases cover Overworld, End and Nether; sunrise, rain/star brightness, anaglyph, signed moon phases and heights below/at/above the horizon. The production GPU uses the same checked sun geometry and celestial/sunrise/End operation helpers. The diagnostic adapter reconstructs original quads/fans from production geometry to compare the original draw format.

This check does not complete RenderGlobal: cloud geometry/options/state, renderer initialization/update/deletion and visibility ordering, entity/tile dispatch, particle factory and auxiliary-effect callbacks, and world-RNG consumption still need complete delta coverage. The diagnostic adapter's black horizon box and GL state are checked against bytecode; the full GPU sky pipeline is not claimed bit-exact.

## Validation

- Whole workspace `cargo build --release`: passed, also run by verify10 (`target/build-render3c.log`, `target/verify10-render3.log`).
- `bash tools/verify10.sh render`: passed, all 55 sections EXACT (`target/verify10-render3.log`). Round 2 has 45 sections and 10,420,211 comparisons; round 3 has eight world meshes, block inventory and global sky.
- `cargo test -p worldgen10`: 80 passed, zero failures (`target/test-worldgen10-render3.log`).
- `cargo test -p client10`: 25 passed, zero failures (`target/test-client10-render3.log`).
- Required shared-oracle compile/run command `bash tools/oracle10/oracle.sh raw 0 0 0 0 0 t.bin`: passed (`target/raw-render3.log`; output is relative to tools/oracle10/out).
- Additional first-person integration comparison: **does not pass**. The oracle emits 21,124 cases; the existing Rust reader reports texture identity and other remaining renderer mismatches, then panics on unsupported case kind 20 (`target/firstperson-render3-oracle.log`, `target/firstperson-render3-rust.log`). This broad suite is not credited as EXACT. ItemRenderer, RenderItem, RenderGlobal and EntityRenderer remain Not done; the 55 rendering sections above are independently EXACT.
- Shared-recorder compatibility: passed. Recompiled the unmodified HEAD versions of Oracle, RendererOracle and GL11 into an isolated target directory and reran all 21,124 first-person bytecode cases. Their binary output matches the current recorder byte for byte (`target/recorder-compat-render3.log`). The extended recorder therefore preserves this existing recording format; it does not resolve the Rust reader's remaining first-person gaps.
- Required seed-42 no-save 60-tick smoke: passed; visually inspected `target/r10-smoke.png`. Forest, foliage, first-person hand, tutorial and HUD render without visible corruption. Used the local release asset jar through BETA_JAR and `--options-dir target/r10-smoke-options`.
- No Beta crate, oracle or verifier was edited; no worldgen10 simulation implementation was changed. No GUI implementation was edited. Concurrent staged GUI2 harness moves belong to the other worker and are excluded from the commit helper.
- `git diff --check` and shell syntax checks: passed. No commit was made. `target/commit-r10-render.sh` stages explicit rendering files and uses an explicit-path commit to exclude unrelated staged GUI2 changes, with the requested author and trailers.

## Class table

Done rows below are the reverified round-2 deltas plus RenderBlocks. Not done rows describe the remaining port or bytecode coverage; inherited behavior alone earns no credit.

| Class | Status | Port, exact check, or remaining work |
| --- | --- | --- |
| RenderBlocks | Done | Complete audited release delta: render types 23..27, crops/doors/fences/fluids, regenerated release properties, two-way repeater/wire adjacency and lily inventory tint. EXACT: RenderBlocks_world_{fancy,fast}_{on,off}_{normal,anaglyph} and RenderBlocks_inventory_lily_pad_dragon_egg. |
| RenderGlobal | Not done | EXACT: RenderGlobal_dimension_sky_phases_transforms_horizon (64 cases) covers original sky draws/state using production geometry/helpers. Clouds, initialization/update/deletion, visibility and entity/tile dispatch ordering, particle factory, auxiliary effects and world-RNG checks remain. |
| ModelDragon | Done | 256x256 named boxes, mirrored child hierarchy, history sampling/wrapping, neck/head/jaw/wing/limb/tail poses and culling. EXACT: `ModelDragon_constructor_pose` (80 cases) and `ModelDragon_draw` (32 sequential cases). |
| EntityRenderer | Not done | Partial: retained cloud-option/distance guard. Complete fog, lightmap/night vision, FOV, camera rays, transforms and frame-pacing delta and original checks remain. |
| RenderDragon | Done | EXACT: RenderDragon_passes, RenderDragon_death_rays, RenderDragon_crystal_beam, RenderDragon_boss_overlay: canonical manager renderer replacement/pass-model behavior, death alpha cutoff, eyes/fullbright, hurt passes, seeded rays, beam casts/UV/color, watched health and boss rectangles/title. Live state reads simulation history and linked crystals even outside crystal draw range. |
| RenderItem | Not done | Partial: release inventory dependencies retained. Dropped-item enchantment glint, potion passes and complete item/GUI bytecode checks remain. |
| ItemRenderer | Not done | Inherited first-person renderer retained; full 1.0 glint, potion passes, bow/food/milk gestures, equipment state and original-bytecode checks remain. Snow-golem pumpkin uses the existing block branch. |
| ModelRenderer | Done | Complete release named offsets, cube geometry/list/child hierarchy, visibility, all three draw methods and first-scale compilation caching. EXACT: `ModelBase_ModelBox_ModelRenderer`, plus all model constructor/draw sections. |
| RenderLiving | Not done | Partial: propagated child state and tripled child limb swing, removed embedded-arrow drawing, integrated new callbacks and model hierarchy. Complete armor glint/pass-15 texture matrices, all inherited passes and state/lightmap verification remain. New-renderer delegation checks cover only their exercised branches. |
| RenderEndPortal | Not done | Registered the renderer identity only. All 16 layers, seeded colors, texgen/projective sampling, texture matrices, camera offsets and state restoration are not implemented or checked. |
| EntityDropParticleFX | Not done | Round-1 water/lava implementation retained. Original hanging/dripping/landing/material-removal, RNG, texture and brightness checks remain. |
| ModelBox | Done | Complete release named offsets, cube geometry/list/child hierarchy, visibility, all three draw methods and first-scale compilation caching. EXACT: `ModelBase_ModelBox_ModelRenderer`, plus all model constructor/draw sections. |
| ModelVillager | Done | Constructor offsets/inflation, robe/nose/crossed arms and head/leg poses. EXACT: `ModelVillager_constructor_pose` (80 cases) and `ModelVillager_draw` (32 sequential cases). |
| EntityEnchantmentTableParticleFX | Not done | Round-1 implementation retained. Original quartic path, float/double casts, bounds, RNG, UVs and light/expiry checks remain. |
| RenderPlayer | Not done | Partial: removed inherited embedded-arrow draw. Active block/bow pose selection, potion held-item passes, armor glint and complete cape/name/sleep/armor bytecode matrix remain. |
| ActiveRenderInfo | Not done | Inherited camera helpers retained; new unprojection/active-view offsets and material lookup delta need a full port and original checks, including End-portal camera inputs. |
| ModelSnowMan | Done | Five parts, torso/arm/head poses and render order. EXACT: `ModelSnowMan_constructor_pose` (80 cases) and `ModelSnowMan_draw` (32 sequential cases). |
| EntitySpellParticleFX | Not done | Round-1 implementation retained. Original normal/instant frames, random motion, damping, ceiling/collision and expiry checks remain. |
| ModelBlaze | Done | Twelve rods in three sine-table orbits, head pose and version 8. EXACT: `ModelBlaze_constructor_pose` (80 cases) and `ModelBlaze_draw` (32 sequential cases). |
| ModelWolf | Done | Multi-cube head, ears/snout, child groups, sitting/shake/interested/angry poses and render ordering. EXACT: `ModelWolf_constructor_pose` (80 cases) and `ModelWolf_draw` (32 sequential cases). |
| ModelMagmaCube | Done | Eight slices, core, exceptional slice UVs, clamped squish interpolation and version 5. EXACT: `ModelMagmaCube_constructor_pose` (80 cases) and `ModelMagmaCube_draw` (32 sequential cases). |
| RenderMooshroom | Done | EXACT: RenderMooshroom: 64 full original delegations, adult/child guard and groups, three crossed mushrooms, head attachment, culling, texture and overlay state. |
| RenderEnchantmentTable | Done | EXACT: RenderEnchantmentTable: 64 original tile callbacks for wrapped rotation, hover, pages, opening, geometry and texture. Live book snapshots read worldgen10 EnchantmentTable state. |
| RenderVillager | Done | EXACT: RenderVillager: 64 full original delegations, all profession/default textures, 0.9375 scale, shadow, pose and hurt/death state. |
| ModelBook | Done | Covers, spine, pages and flippers; original initializer registration order and opening/page poses. EXACT: `ModelBook_constructor_pose` (80 cases) and `ModelBook_draw` (32 sequential cases). |
| EntityBreakingFX | Not done | Round-1 item-fragment constructors/factory retained. Original item/potion/eye icon, constructor RNG/motion and jittered UV checks remain. |
| OpenGlHelper | Not done | Existing wgpu lightmap adapter retained. Headless GL13 calls are oracle endpoints, not a port of this class. ARB/core capability branches and active/client texture-coordinate behavior remain unaudited. |
| ModelEnderCrystal | Done | Base/glass/cube geometry and nested scale/rotation/bob sequence. EXACT: `ModelEnderCrystal_constructor_pose` (80 cases) and `ModelEnderCrystal_draw` (32 sequential cases). |
| RenderSnowball | Done | EXACT: RenderSnowball: 64 original callbacks, snowball/egg/pearl/eye billboards and tinted potion liquid plus bottle overlay; material, normals, UVs and final state. |
| RenderMagmaCube | Done | EXACT: RenderMagmaCube: 64 full original delegations with size/squish, sliced model, texture, shadow and hurt state; version 5 checked by ModelMagmaCube_constructor_pose. |
| ModelBiped | Done | Integer held poses and aimed-bow order, constructor options, riding/sneak/attack and registered geometry. EXACT: `ModelBiped_constructor_pose` (80 cases) and `ModelBiped_draw` (32 sequential cases). |
| ModelCow | Done | Head horn cubes, body udder cube and child-head offsets. EXACT: `ModelCow_constructor_pose` (80 cases) and `ModelCow_draw` (32 sequential cases). |
| RenderSnowMan | Done | EXACT: RenderSnowMan: 64 full original delegations, head attachment and pumpkin arguments/geometry. ItemRenderer is an isolated pumpkin endpoint calling original RenderBlocks; ItemRenderer itself is not counted Done. |
| RenderEnderCrystal | Done | EXACT: RenderEnderCrystal: 64 sequential original doRender calls, constructor/model geometry, rotation/bob/transforms and texture. |
| RenderMinecart | Not done | Inherited vehicle renderer retained. Complete 1.0 track/model/container rendering delta and original transform/geometry/state checks remain. |
| EntityFX | Not done | Round-1 particle base retained. Full release constructor, motion, render colors/UVs, packed-light and RNG bytecode checks remain. |
| RenderBlaze | Done | EXACT: RenderBlaze: 64 full original RenderLiving delegations, model, texture, shadow, poses, corpse/hurt state; version 8 also checked by ModelBlaze_constructor_pose. |
| ModelQuadruped | Done | Child head/body groups, constructor leg heights and pose arithmetic. EXACT: `ModelQuadruped_constructor_pose` (80 cases) and `ModelQuadruped_draw` (32 sequential cases). |
| ModelChicken | Done | Child group transforms, head/body/legs/wings and pose ordering. EXACT: `ModelChicken_constructor_pose` (80 cases) and `ModelChicken_draw` (32 sequential cases). |
| RenderManager | Done | EXACT: RenderManager_TileEntityRenderer: all original entity registrations, renderer identities, shadow size/opacity and PigZombie inheritance. Added every new release entity and EntityFishHook naming. |
| ModelSkeleton | Done | Aimed-bow state before inherited zombie pose and original geometry/ordering. EXACT: `ModelSkeleton_constructor_pose` (80 cases) and `ModelSkeleton_draw` (32 sequential cases). |
| RenderSheep | Not done | Child model state is propagated by the shared renderer. Complete sheep pass callback, fleece tint/visibility and original animation/pass checks remain. |
| RenderPig | Not done | Child model state is propagated by the shared renderer. Complete saddle callback/visibility and original pass checks remain. |
| RenderHelper | Not done | Inherited lighting adapter retained. Complete 1.0 GUI-light rotation/setup and original light-vector/material/capability checks remain; the oracle ignores light configuration outside recorded draw flags. |
| ModelPig | Done | Snout as a head cube, child-head offset and quadruped groups. EXACT: `ModelPig_constructor_pose` (80 cases) and `ModelPig_draw` (32 sequential cases). |
| RenderXPOrb | Not done | Inherited orb renderer retained. Complete 1.0 packed-light/color/animation delta and original draw-state checks remain. |
| RenderFallingSand | Not done | Inherited falling-block renderer retained. Complete release block-access/brightness and original geometry/lightmap checks remain. |
| RenderBiped | Not done | Inherited held-item renderer retained. Constructor scale field and potion second pass still need complete audit/port and original callback/argument checks. |
| ModelBase | Done | Complete release named offsets, cube geometry/list/child hierarchy, visibility, all three draw methods and first-scale compilation caching. EXACT: `ModelBase_ModelBox_ModelRenderer`, plus all model constructor/draw sections. |
| EntityCrit2FX | Not done | Inherited implementation retained. Release reduced-crit constructor/color/scale/lifetime/setter delta and original checks remain. |
| Render | Not done | Inherited shadow/fire and dispatch base retained. Complete release child-shadow, fire, labels and world/lightmap bytecode checks remain. |
| RenderPainting | Not done | Inherited painting renderer retained. Full release packed-light/tile geometry delta and original draw/state checks remain. |
| RenderFireball | Done | EXACT: RenderFireball: 64 original callbacks at scale 2 and 0.5, corrected icon 46, camera transforms and draw/material/state. |
| RenderCreeper | Not done | Inherited creeper callbacks retained. Full release powered/flash/scale/pass-state delta and original checks remain. |
| RenderSpider | Not done | Inherited spider renderer retained. Full release eye-lightmap/depth/color and corpse-transform delta and original checks remain. |
| EffectRenderer | Not done | Round-1 particle factory/lifecycle changes retained. Complete 1.0 ordered layer/update/dead-removal/world-RNG original checks remain. |
| TileEntityRenderer | Done | EXACT: RenderManager_TileEntityRenderer: all six original tile registrations, including book and End portal. This proves its registration delta, not the unimplemented RenderEndPortal draw routine. |
| Tessellator | Not done | Original Tessellator now executes in the shared oracle, and emitted positions/UV/colors/normals are checked by rendering sections. GPU fan/strip conversion exists. Complete release delta, buffer packing/upload and lightmap bytecode checks for this class itself remain. |
| RenderSlime | Not done | Inherited slime renderer retained. Full release shadow/squish/normal/pass-state delta and original checks remain. |
| RenderEnderman | Not done | Shared child/model changes apply. Complete release carried-block, scream jitter, eye/hurt/lightmap callback audit and original checks remain. |
| WorldRenderer | Not done | Inherited section renderer retained. Release initialization/update/list and dirty-state delta and original checks remain. |
| RenderSilverfish | Not done | Inherited silverfish renderer retained. Full release constructor/texture/death-transform bytecode check remains. |
| RenderBoat | Not done | Inherited boat renderer retained. Full release rocking/transforms/model/lightmap bytecode check remains. |
| EntityDiggingFX | Not done | Inherited digging effect retained. Release block-side tint, packed light, motion/UV delta and original checks remain. |
| RenderFish | Not done | Registered the release EntityFishHook name. Full renderer/angler line/visibility bytecode check remains. |
| OpenGlCapsChecker | Not done | Inherited capability abstraction retained. Full release capability decision and original check remain. |
| MapItemRenderer | Not done | Inherited map renderer retained. Full release map texture/icon/state delta and original checks remain. |
| EntitySuspendFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntitySplashFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntitySnowShovelFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntitySmokeFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityReddustFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityRainFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityPortalFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityNoteFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityLavaFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityHeartFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityFlameFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityExplodeFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityCritFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityCloudFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityBubbleFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
| EntityAuraFX | Not done | Inherited particle implementation retained. Complete release constructor/tick/render/packed-light/RNG audit and original-bytecode comparison remain. |
