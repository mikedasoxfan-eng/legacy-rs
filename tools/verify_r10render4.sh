#!/usr/bin/env bash
# Release 1.0 rendering round 4: the big renderers against the original 1.0 bytecode on the
# shared oracle10 harness (headless GL recorder plus GlLog10 for texgen, texture matrices,
# lights, fog and GL13 texture units). One "== section" per check; exit 1 on any difference.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
out="$root/target/oracle10-render4"
mkdir -p "$out"
if [ "${R10_RENDER_SKIP_BUILD:-0}" != 1 ] && [ "${1:-}" != --built ]; then
  cargo build --release --manifest-path "$root/Cargo.toml" || exit 1
fi
oracle="$root/tools/oracle10/oracle.sh"
W="$out"
if command -v cygpath >/dev/null 2>&1; then W="$(cygpath -w "$out")"; fi
beta() { "$root/target/release/beta" --mc 1.0 "$@"; }
fail=0
result() { if [ "$1" = 0 ]; then echo "EXACT"; else echo "DIFFERS"; fail=1; fi; }
# section label oracle-mode... -- rust-flag...   (the oracle output path is appended to the mode
# arguments and passed as the last Rust argument)
check() {
  local label="$1"; shift
  local oargs=() rargs=()
  while [ "$1" != "--" ]; do oargs+=("$1"); shift; done; shift
  rargs=("$@")
  local bin="$label.bin"
  echo "== $label"
  if "$oracle" "${oargs[@]}" "$W/$bin" >"$out/$label.oracle.log" 2>&1 &&
     beta "${rargs[@]}" "$out/$bin" >"$out/$label.rust.log" 2>&1; then
    tail -1 "$out/$label.rust.log"; result 0
  else
    tail -n 12 "$out/$label.oracle.log" "$out/$label.rust.log"; result 1
  fi
}
round4() { check "$1" render_round4_10 "$2" -- --render-round4-check "$2"; }

check RenderEndPortal_layers_texgen_parallax render_portal10 -- --render-portal10-check
round4 ActiveRenderInfo_unproject_view_vectors_eye_block activeinfo
check FirstPerson_camera_fov_hand_glint_player_poses_targeting_fog_maps firstperson -- --firstpersoncheck
check RenderBiped_held_potion_passes_glint render_biped4_10 -- --render-biped4-check
check RenderLiving_baby_animals_child_models_limbs render_child4_10 -- --render-child4-check
check RenderLiving_mob_renderers_passes_overlays generaliving4_181 -- --newlivingcheck
check RenderLiving_ghast_pigzombie_giant livingrenderers -- --livingrenderercheck
check RenderPlayer_player_living_round4 playerliving4_181 -- --playerlivingcheck
check RenderPlayer_player_living_round6 playerliving6_181 -- --playerlivingcheck
round4 RenderItem_gui_potion_zlevel_untinted_glint gui
check RenderItem_dropped_gui_overlay_renderers renderintegration181 renderers -- --renderercheck
check RenderItem_pickup_renderers renderintegration181 pickuprenderers -- --renderercheck
round4 EntityRenderer_fog_blindness_breathing_nosky_cloud fog
round4 EntityRenderer_lightmap_end lightmap
round4 EntityRenderer_smooth_camera smooth
check EntityRenderer_lightmap_overworld_nether lightmap -- --lightmapcheck
echo "== EntityRenderer_rain_snow_temperature_particles"
weather_ok=0
for args in "2 0.5 70 0.5 100 0.75 0.8" "11 0.5 80 0.5 2147483647 0.25 1" "3257840388504953787 -39.5 75 65.5 23 0.5 1" "1 0.5 70 0.5 0 0 1" "42 -20.25 90 -31.75 -2147483648 1.0 0.01"; do
  for graphics in fancy fast; do
    for particles in 0 1 2; do
      if "$oracle" weathermesh $args "$W/weather.bin" natural 0 "$graphics" "$particles" >"$out/weather.oracle.log" 2>&1 &&
         beta --weathercheck $args "$out/weather.bin" natural 0 "$graphics" "$particles" >"$out/weather.rust.log" 2>&1; then :; else
        echo "weather differs: $args $graphics particles $particles"; tail -3 "$out/weather.rust.log"; weather_ok=1; fi
    done
  done
done
for scene in lava roof; do
  for graphics in fancy fast; do
    if "$oracle" weathermesh 42 0.5 80 0.5 123 0.5 1 "$W/weather.bin" "$scene" 5 "$graphics" >"$out/weather.oracle.log" 2>&1 &&
       beta --weathercheck 42 0.5 80 0.5 123 0.5 1 "$out/weather.bin" "$scene" 5 "$graphics" >"$out/weather.rust.log" 2>&1; then :; else
      echo "weather differs: $scene $graphics"; weather_ok=1; fi
  done
done
[ "$weather_ok" = 0 ] && echo "weather exact: 5 scenes x 2 graphics x 3 particle settings, lava and roof"
result "$weather_ok"
round4 OpenGlHelper_OpenGlCapsChecker_units_capabilities glhelper
round4 RenderHelper_item_and_gui_lighting renderhelper
round4 Render_shadow_baby_animals shadow
check Render_shadow_fire rendereffects -- --effectcheck
round4 Tessellator_draw_bytes_brightness_units tessellator
echo "== WorldRenderer_sections_passes_skip_tiles"
wr_ok=0
for spec in "1 1 fancy on" "1 1 fast off" "25 1 fancy off" "25 1 fast on"; do
  if "$oracle" render_worldrenderer4_10 $spec 2 "$W/worldrenderer.bin" >"$out/worldrenderer.oracle.log" 2>&1 &&
     beta --worldrenderer4-check $spec "$out/worldrenderer.bin" >"$out/worldrenderer.rust.log" 2>&1; then
    tail -1 "$out/worldrenderer.rust.log"
  else echo "WorldRenderer differs: $spec"; tail -5 "$out/worldrenderer.rust.log"; wr_ok=1; fi
done
result "$wr_ok"
round4 RenderGlobal_playAuxSFX_spawnParticle_setting auxsfx
check RenderGlobal_particle_factory_names particlefactory6_181 -- --particlefactorycheck
echo "== RenderGlobal_clouds_fancy_fast_anaglyph"
cloud_ok=0
for args in "1 0.5 70 0.5 100 0.75 6000 0 0" "42 -20.25 110 -31.75 123 0.5 13000 0.75 0.4" \
  "2 30000.25 120 -50000.75 2147483647 0.25 23500 1 1" "11 2048 107 0 -2147483648 1.0 18000 0.1 0.9"; do
  for graphics in fancy fast; do
    for stereo in normal anaglyph; do
      if "$oracle" cloudmesh $args "$graphics" "$W/cloud.bin" "$stereo" >"$out/cloud.oracle.log" 2>&1 &&
         beta --cloudcheck $args "$graphics" "$out/cloud.bin" "$stereo" >"$out/cloud.rust.log" 2>&1; then :; else
        echo "clouds differ: $args $graphics $stereo"; cloud_ok=1; fi
    done
  done
done
[ "$cloud_ok" = 0 ] && echo "clouds exact: 4 cameras x fancy/fast x normal/anaglyph"
result "$cloud_ok"
check RenderGlobal_sky_gl_colors_sunrise skydraw -- --skydrawcheck
check RenderGlobal_dimension_sky_end_moon_horizon render_global10 -- --render-global10-check
if [ "$fail" = 0 ]; then echo "R10 RENDER ROUND 4 EXACT"; else echo "R10 RENDER ROUND 4 DIFFER"; fi
exit "$fail"
