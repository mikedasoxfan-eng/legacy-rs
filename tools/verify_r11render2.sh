#!/usr/bin/env bash
# Exact Release 1.0 model/renderer checks against the original remapped jar.
# No model or renderer source is compiled: only Mode_ harnesses and GL endpoints.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
out="$root/target/oracle11-render2"
mkdir -p "$out"
if [ "${R10_RENDER_SKIP_BUILD:-0}" != 1 ] && [ "${1:-}" != --built ]; then
  cargo build --release --manifest-path "$root/Cargo.toml" || exit 1
fi
oracle="$root/tools/oracle11/oracle.sh"
fail=0
beta() { "$root/target/release/beta" --mc 1.1 "$@"; }
run() {
  local label="$1" mode="$2" flag="$3"; shift 3
  local path="$out/$label.bin"
  echo "== $label"
  if "$oracle" "$mode" "$@" "$path" && beta "$flag" "$path"; then :; else fail=1; fi
}
run ModelBase_ModelBox_ModelRenderer render_geometry10 --render-geometry10-check
names=(ModelBiped ModelZombie ModelSkeleton ModelQuadruped ModelPig ModelCow ModelChicken ModelWolf ModelDragon ModelVillager ModelSnowMan ModelBlaze ModelMagmaCube ModelBook ModelEnderCrystal)
for kind in "${!names[@]}"; do
  run "${names[$kind]}_constructor_pose" render_models10 --render-models10-check "$kind"
  run "${names[$kind]}_draw" render_modeldraw10 --render-modeldraw10-check "$kind"
done
run RenderEnchantmentTable render_tiles10 --render-tiles10-check 0
run RenderEnderCrystal render_tiles10 --render-tiles10-check 1
run RenderSnowball render_effects10 --render-effects10-check 0
run RenderFireball render_effects10 --render-effects10-check 2
run RenderDragon_death_rays render_effects10 --render-effects10-check 1
run RenderBlaze render_living10 --render-living10-check 0
run RenderMagmaCube render_living10 --render-living10-check 1
run RenderVillager render_living10 --render-living10-check 2
run RenderMooshroom render_living10 --render-living10-check 3
run RenderSnowMan render_living10 --render-living10-check 4
run RenderDragon_crystal_beam render_dragonbeam10 --render-dragonbeam10-check
run RenderDragon_passes render_dragon10 --render-dragon10-check
run RenderDragon_boss_overlay render_boss10 --render-boss10-check
run RenderManager_TileEntityRenderer render_registry10 --render-registry10-check
if [ "$fail" = 0 ]; then echo "R10 RENDER ROUND 2 EXACT"; else echo "R10 RENDER ROUND 2 DIFFER"; fi
exit "$fail"
