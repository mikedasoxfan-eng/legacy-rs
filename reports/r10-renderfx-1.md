# Minecraft 1.0 port: area "renderfx", round 1

Suite: `tools/verify_r10renderfx1.sh` (registered in `tools/verify10.sh` before the final `pflush`).
Result: 53 sections, all EXACT (`R10 RENDERFX ROUND 1 EXACT`). `bash tools/verify_vehicles10.sh --built`
is fully EXACT (`VEHICLES ALL CASES EXACT`), and the decorations painting renderer section is EXACT too.

## What the checks drive

- `particlefx_*` (Mode_render_particlesfx10 `<kind>`): every EntityFX class constructed through its original
  1.0 constructor (32 seeded variants per kind, 340 ticks, 4 partial ticks): full entity state, both RNGs,
  `getEntityBrightnessForRender`, `getEntityBrightness`, every Tessellator word including the per-vertex
  brightness (-1 when the Tessellator has none, as for EntityFootStepFX). The fixture world now includes the
  1.0 collision shapes (fences, nether brick fence, fence gates, panes, the new stairs, brewing stand, cauldron,
  end portal frame, lily pad, enchantment table, dragon egg), rotated per kind under the particles' paths.
- `effectrenderer_dispatch_activerenderinfo`: EffectRenderer.renderParticles/func_1187_b dispatch with the
  1.0 billboard axes read from ActiveRenderInfo's statics (normal and mirrored front view).
  `effectrenderer_effects`, `effectrenderer_factory`, `effectrenderer_lifecycle`: block hit/destroy effects,
  the named factory and the layer lifecycle on 1.0 bytecode.
- `entitycrit2fx_named_emitters` (Mode_render_critfx10): EntityCrit2FX through the 2-argument constructor and
  the new 3-argument constructor with "magicCrit" and "crit".
- `renderfx_*` (Mode_render_simple10 `<kind>`): original renderer bytecode recorded as Draw10 records plus the
  unit-1 lightmap coordinates in force at every draw (random states, 64 cases per mob); painting tiles with
  random packed light; falling sand/gravel/dragon egg against a lit world (raw Tessellator words incl.
  brightness), AO on and off.
- `vehicles_renderers` (now also records the entity ID), `vehicles_renders_legacy`, `xporb_renderer_legacy`,
  `painting_renderer_legacy`, `fish_renderer` (thirdPersonView 0, 1 and 2).

## Class table

| Class | Status | Notes |
|---|---|---|
| EntityDropParticleFX | Done | Water/lava drip: hang timer, texture 113/112/114, splash, liquid/solid removal, 257 packed / 1.0 scalar for lava; checks: `particlefx_EntityDropParticleFX_water`, `particlefx_EntityDropParticleFX_lava` |
| EntityEnchantmentTableParticleFX | Done | Quartic path, colours, Math.random texture, noClip, packed/scalar light overrides; checks: `particlefx_EntityEnchantmentTableParticleFX` |
| EntitySpellParticleFX | Done | Normal, instant (func_40110_b) and mob spell (func_40097_b colours) frames, motion, damping; checks: `particlefx_EntitySpellParticleFX`, `particlefx_EntitySpellParticleFX_instant`, `particlefx_EntitySpellParticleFX_mob` |
| EntityBreakingFX | Done | Item icon constructor (snowball, slime ball, any item incl. potions/eye) and the velocity constructor, layer 2 UVs; checks: `particlefx_EntityBreakingFX_snowball`, `particlefx_EntityBreakingFX_slimeball`, `particlefx_EntityBreakingFX_item`, `particlefx_EntityBreakingFX_item_velocity` |
| RenderMinecart | Done | Ported the per-entity-ID glTranslatef jitter (the missing 7th matrix op) and glColor4f(1) after the chest/furnace block; the dump now carries entityId; checks: `vehicles_renderers`, `vehicles_renders_legacy` |
| EntityFX | Done | Accessors (func_40097_b, func_40099_c/func_40100_q, colour getters) and multiplyVelocity rename; base constructor/tick/render/light; 1.0 collision boxes for particle movement (fences, gates, panes, new stairs, brewing stand, cauldron, end portal frame, lily pad, enchantment table, stems/vines/nether wart); checks: `particlefx_EntityFX`, `effectrenderer_effects` |
| RenderSheep | Done | int pass callback; fleece tint; fixed ModelSheep1/ModelSheep2 (they had ModelCow's udder and head offset); checks: `renderfx_RenderSheep` |
| RenderPig | Done | int saddle pass; checks: `renderfx_RenderPig` |
| RenderXPOrb | Done | Constant 255 colour scale, glColor4f(1), orb's own packed lightmap; checks: `renderfx_RenderXPOrb`, `xporb_renderer_legacy` |
| RenderFallingSand | Done | renderBlockFallingSand: fixed face shades and getMixedBrightnessForBlock packed light (per-vertex in game); dragon egg branch renders RenderBlocks type 27 against the world (render/falling.rs, carried in the entity snapshot); checks: `renderfx_RenderFallingSand` |
| EntityCrit2FX | Done | Named constructor ("magicCrit"); the client now handles PlayerEvent::MagicCriticalHit (EntityPlayerSP.func_40183_c); checks: `entitycrit2fx_named_emitters` |
| RenderPainting | Done | Swapped top/bottom normals, glColor3f(1) and per-tile lightmap from getLightBrightnessForSkyBlocks (worker snapshot carries packed tile lights); checks: `renderfx_RenderPainting`, `painting_renderer_legacy` |
| RenderCreeper | Done | int pass callbacks incl. powered armour; checks: `renderfx_RenderCreeper` |
| RenderSpider | Done | Eyes pass with lightmap 61680 and colour 1, cave spider scale; checks: `renderfx_RenderSpider`, `renderfx_RenderSpider_cave` |
| EffectRenderer | Done | renderParticles reads ActiveRenderInfo (new `particles::active_render_rotation`, mirrored for thirdPersonView 2), setBrightness per particle; checks: `effectrenderer_dispatch_activerenderinfo`, `effectrenderer_effects`, `effectrenderer_factory`, `effectrenderer_lifecycle` |
| RenderSlime | Done | int pass; inherited pass 1 in the hurt overlay disables blend and resets glColor to 1; checks: `renderfx_RenderSlime` |
| RenderEnderman | Done | Carried block lightmap = entity packed light, block tinted (useInventoryTint), eyes 61680, scream jitter from the renderer Random; checks: `renderfx_RenderEnderman` |
| RenderSilverfish | Done | Renames/int pass only; checks: `renderfx_RenderSilverfish` |
| RenderBoat | Done | Accessor renames only; checks: `vehicles_renderers`, `vehicles_renders_legacy` |
| EntityDiggingFX | Done | func_40099_c/func_40100_q; checks: `particlefx_EntityDiggingFX`, `effectrenderer_effects` |
| RenderFish | Done | thirdPersonView int (> 0, so front view draws the third-person line); checks: `fish_renderer` |
| EntitySuspendFX | Done | checks: `particlefx_EntitySuspendFX` |
| EntitySplashFX | Done | checks: `particlefx_EntitySplashFX` |
| EntitySnowShovelFX | Done | checks: `particlefx_EntitySnowShovelFX` |
| EntitySmokeFX | Done | checks: `particlefx_EntitySmokeFX`, `particlefx_EntitySmokeFX_large` |
| EntityReddustFX | Done | checks: `particlefx_EntityReddustFX` |
| EntityRainFX | Done | checks: `particlefx_EntityRainFX` |
| EntityPortalFX | Done | Packed and scalar light overrides; checks: `particlefx_EntityPortalFX` |
| EntityNoteFX | Done | checks: `particlefx_EntityNoteFX` |
| EntityLavaFX | Done | Packed and scalar (1.0) light overrides; checks: `particlefx_EntityLavaFX` |
| EntityHeartFX | Done | checks: `particlefx_EntityHeartFX` |
| EntityFlameFX | Done | Packed and scalar light overrides; checks: `particlefx_EntityFlameFX` |
| EntityExplodeFX | Done | checks: `particlefx_EntityExplodeFX` |
| EntityCritFX | Done | checks: `particlefx_EntityCritFX`, `particlefx_EntityCritFX_magic` |
| EntityCloudFX | Done | checks: `particlefx_EntityCloudFX` |
| EntityBubbleFX | Done | checks: `particlefx_EntityBubbleFX` |
| EntityAuraFX | Done | checks: `particlefx_EntityAuraFX` |

Unreachable in game / not required: ActiveRenderInfo's own unprojection and GL readback (render worker's
class; the fixture sets its five statics from the formula and the app passes its camera angles).

## Edits outside the area (minimal, for the integrator)

- `tools/oracle10/src/.../RendererOracle.java`: Draw10 gains `lightmapU/lightmapV` (additive, no format change
  for existing fixtures); the vehicle renderer dump is version 2 with `entityId` (Rust reader accepts 1 and 2).
- `tools/oracle10/src/.../Mode_effectdispatch8_10.java`: sets ActiveRenderInfo's statics before renderParticles
  (1.0 reads them; before this the check compared against zeros and failed at byte 30).
- `tools/oracle10/src/.../DecorationOracle.java` fishRenderers: thirdPersonView 0..2 (180 cases).
- `crates/client10/src/render/entity/models.rs` (render worker): removed ModelCow's udder box and head offset
  that had been copied into ModelSheep1/ModelSheep2.
- `crates/client10/src/render/entity/raw_living181.rs`: enderman carried block tinted; slime overlay state.
- `crates/client10/src/render/entity/fixed.rs`: `lightmap`/`draw_lightmaps` tracking; `effects.rs` Snapshot
  gains `tile_lights` and `block_mesh`; `worker.rs` fills them; `pipeline.rs` uses them (painting tiles,
  falling block per-vertex light, dragon egg mesh); `inventory.rs` `falling()` 1.0 shading; `objects.rs`
  XP orb colour; `app.rs` particle rotation via `active_render_rotation` and MagicCriticalHit.
