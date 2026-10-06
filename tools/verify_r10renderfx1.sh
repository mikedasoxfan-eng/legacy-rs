#!/usr/bin/env bash
# Minecraft 1.0 renderfx round 1, against the original 1.0 bytecode (Mojang Java 8u51):
# every EntityFX class (construct, tick, packed/scalar light, Tessellator words),
# EffectRenderer (ActiveRenderInfo billboard axes, factory, lifecycle, effects),
# EntityCrit2FX's named constructor and the simple entity renderers.
# Usage: tools/verify_r10renderfx1.sh [--built]   exit code 0 only if every section is EXACT
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
out="$root/target/oracle10-renderfx1"
mkdir -p "$out" "$root/tools/oracle10/out"
if [ "${1:-}" != --built ]; then cargo build --release -q --manifest-path "$root/Cargo.toml" || exit 1; fi
oracle="$root/tools/oracle10/oracle.sh"
fail=0
# check LABEL FILE "ORACLE ARGS" RUST-FLAG: the dump path is appended to the oracle arguments.
check() {
  local label="$1" file="$out/$2" flag="$4"
  local -a oargs
  read -r -a oargs <<<"$3"
  echo "== $label"
  if "$oracle" "${oargs[@]}" "$file" >"$file.oracle.log" 2>&1 &&
     "$root/target/release/beta" --mc 1.0 "$flag" "$file" >"$file.rust.log" 2>&1; then
    tail -n 1 "$file.rust.log"
    echo "EXACT"
  else
    tail -n 15 "$file.oracle.log" "$file.rust.log"
    echo "DIFFERS: $label"
    fail=1
  fi
  [ "${VERIFY10_KEEP:-}" = 1 ] || rm -f "$file"
}

# ---- particles: Mode_render_particlesfx10 <kind>, 32 seeded variants x 340 ticks x 4 partials
particles=(EntityFX EntitySmokeFX EntitySmokeFX_large EntityFlameFX EntityExplodeFX EntitySplashFX
  EntityRainFX EntityBubbleFX EntityPortalFX EntityNoteFX EntityReddustFX EntityHeartFX EntityLavaFX
  EntityBreakingFX_snowball EntityBreakingFX_slimeball EntitySnowShovelFX EntityDiggingFX
  EntityFootStepFX EntityCritFX EntityCritFX_magic EntityCloudFX EntityAuraFX EntitySuspendFX
  EntityLargeExplodeFX EntityDropParticleFX_water EntityDropParticleFX_lava
  EntityEnchantmentTableParticleFX EntitySpellParticleFX EntitySpellParticleFX_instant
  EntitySpellParticleFX_mob EntityBreakingFX_item EntityBreakingFX_item_velocity)
for kind in "${!particles[@]}"; do
  check "particlefx_${particles[$kind]}" "particles_$kind.bin" "render_particlesfx10 $kind" --particlecheck
done

# ---- EffectRenderer and EntityCrit2FX
check effectrenderer_dispatch_activerenderinfo effectdispatch.bin "effectdispatch8_181" --effectdispatch8check
check effectrenderer_effects effects.bin "particleeffects" --particlefxcheck
check effectrenderer_factory factory.bin "particlefactory6_181" --particlefactorycheck
check effectrenderer_lifecycle lifecycle.bin "effectlifecycle4_181" --effectlifecyclecheck
check entitycrit2fx_named_emitters crit2.bin "render_critfx10" --particleemittercheck

# ---- simple entity renderers: Mode_render_simple10 <kind>, Draw10 records + lightmap per draw
renderers=(RenderSheep RenderPig RenderCreeper RenderSpider RenderSpider_cave RenderSlime
  RenderEnderman RenderSilverfish RenderXPOrb RenderPainting RenderFallingSand)
for kind in "${!renderers[@]}"; do
  check "renderfx_${renderers[$kind]}" "simple_$kind.bin" "render_simple10 $kind" --renderfx-simple-check
done
check vehicles_renderers vehicles.bin "renderintegration181 vehiclerenderers" --vehiclerenderercheck
check vehicles_renders_legacy vehiclerenders.bin "vehiclerenders181" --vehiclerenderercheck
check xporb_renderer_legacy xp.bin "xprender181" --renderercheck
check painting_renderer_legacy paintings.bin "paintingrenderers" --paintingrenderercheck
check fish_renderer fish.bin "fishrenderers" --fishrenderercheck

if [ "$fail" = 0 ]; then echo "R10 RENDERFX ROUND 1 EXACT"; else echo "R10 RENDERFX ROUND 1 DIFFER"; fi
exit "$fail"
