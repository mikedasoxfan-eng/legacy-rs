# Release 1.0 rendering, round 2

29 of the 83 assigned classes are Done under this round's complete-port plus EXACT-original-bytecode criteria. 54 remain Not done. This is a partial area handoff, not a claim that the entire rendering task is complete.

The release model foundation, all seven new models, seven changed models, new mob/object/book renderers, dragon passes/effects/boss bar and both registration deltas are ported. Simulation remains in worldgen10; no worldgen crate or Beta crate/oracle/suite was edited. ModelZombie is additionally checked as the inherited skeleton dependency.

## Exact checks

`tools/verify_r10render2.sh` passed all 45 sections with 10,420,211 raw-bit comparisons and zero mismatches, plus exact counts, flags, names, ordering, texture identities and EOF assertions. It has `==` section lines, no `set -e`, creates output with `mkdir -p`, and accumulates failures. Use `R10_JAR` for the remapped original jar and `R10_LWJGL` for its LWJGL metadata dependency if the usual paths are absent. `R10_RENDER_SKIP_BUILD=1` skips an already completed release build.

Every checked model/renderer/manager class is loaded from the original 1.0 remapped jar. No checked Minecraft class is recompiled. Only the new Mode_ harnesses and graphics endpoints are compiled. GL operations and raw float vertex/UV/normal/color/material/depth/blend/cull/cutoff arguments are compared before the GPU. The endpoint recorder does not verify native OpenGL, original Tessellator or light-configuration implementation. Boss checks supply font metrics as an external fixture and invoke the original private GuiIngame boss callback; font rendering itself is outside this area. SnowMan's pumpkin ItemRenderer endpoint forwards to original RenderBlocks and checks its item/pass arguments. These boundaries are explicit in the harness sources.

Reference source and bytecode are read from `../beta-rs/tools/retromcp-r10` because they are absent from this checkout. Remapped jar SHA-256: `679a5f73da7ef7073ad5e3c0ccb4e45f6c26602aa72e1ad08cebcc389af11b8a`. The source decompiler reordered ModelBook initializer registration; the implementation follows the original jar. RenderDragon's first version replacement retains its constructor pass model, including that model's zero animation partial; the full original callback check covers this behavior.

The shared `tools/verify10.sh` and generic oracle dispatcher are absent. `r10-common.md` says these are being built by another worker and must not be created here. Only task-specific Mode_ files/endpoints and the requested standalone suite were added. Integrator must insert `"$root/tools/verify_r10render2.sh" || fail=1` before the final `pflush` of `tools/verify10.sh` when it arrives. This required aggregator append remains outstanding.

## Validation and artifacts

- Whole workspace `cargo build --release`: passed (`target/build-render-final.log`).
- `cargo test -p worldgen10`: 63 passed, zero failures (`target/test-worldgen10-render.log`).
- `cargo test -p client10`: 25 passed, zero failures (`target/test-client10-render.log`).
- Standalone original-jar suite: EXACT (`target/verify-r10render2.log`).
- Required seed-42, no-save, 60-tick smoke: passed; inspected `target/r10-smoke.png`. It shows the forest, first-person hand and HUD without rendering corruption. Inherited GUI version text still says Beta 1.8.1 and belongs to the GUI worker.
- New model/attachment showcase: inspected `target/r10-models.png`, generated with `--release-modelshot`; book, mooshroom, villager, snow golem, blaze, magma cube and End crystal render with release textures. Raw dragon drawing is covered independently by the exact oracle.
- Smoke used `BETA_JAR=../beta-rs/tools/retromcp-r10/jars/minecraft.jar` and `--options-dir target/r10-smoke-options` to keep all generated files in this worktree.
- `git diff --check`: passed. No commit was made. `target/commit-r10-render.sh` stages the explicit changed/new files and contains the requested author/trailers.

## Shared implementation and boundaries

ModelRenderer appends ModelBox entries and freezes the first compilation scale, supports named texture offsets and live child links, and implements the original three transform orders/visibility branches. Existing models now use multi-cube heads/body attachments. The GPU receives per-draw front/back culling and alpha cutoff, a depth-equal opaque pass, SrcAlpha/One ray blending and fan/strip triangulation. The light vertex buffer carries cutoff beside the existing packed light.

Worker/app changes transport existing enchanting-tile animation state and linked crystal state, and include release projectiles in their original extended draw range. Dragon rendering reads worldgen10 history/health; book rendering reads worldgen10 tile fields. No simulation algorithms are duplicated.

Minimal GUI-boundary changes: one hook in `gui/ingame.rs` invokes the task-requested boss bar at the original HUD position; one Batch initializer in `gui/screens/inventory_equipped.rs` supplies default cull/cutoff fields. The boss drawing implementation lives in the rendering module. Other Batch/Draw initializer adaptations are mechanical defaults in existing rendering/check files. GuiIngame and inventory screens are not claimed Done.

## Class table

| Class | Status | Port, exact check, or remaining work |
| --- | --- | --- |
| RenderBlocks | Not done | Partial: retained round-1 release block types; fixed crossed-plane inventory geometry needed by mooshrooms. Mushroom and pumpkin inventory endpoints are checked, but the full 1.0 block delta and complete original mesh/AO/metadata matrix remain unverified. |
| RenderGlobal | Not done | Partial: retained round-1 End sky, moon phase and particle dispatch work. Complete dimension/sky/cloud state, update ordering, visibility, world-RNG consumption and auxiliary-effect bytecode checks remain. |
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
| Tessellator | Not done | Partial backend support: added triangle fan/strip conversion, per-draw culling and alpha cutoff. The oracle replaces Tessellator with a recorder and does not verify the original Tessellator implementation; its full delta/packing/upload check remains. |
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
