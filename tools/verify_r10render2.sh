#!/usr/bin/env bash
# Exact Release 1.0 model/renderer checks against the original remapped jar.
# No model or renderer source is compiled: only Mode_ harnesses and GL endpoints.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
out="$root/target/oracle10-render2"
mkdir -p "$out"
jar="${R10_JAR:-$root/tools/retromcp-r10/minecraft/jars/deobfuscated.jar}"
if [ ! -f "$jar" ] && [ -z "${R10_JAR:-}" ]; then
  jar="$root/../beta-rs/tools/retromcp-r10/minecraft/jars/deobfuscated.jar"
fi
if [ ! -f "$jar" ]; then echo "Original 1.0 remapped jar missing; set R10_JAR" >&2; exit 2; fi
library="${R10_LWJGL:-$(dirname "$jar")/../../libraries/org/lwjgl/lwjgl/lwjgl/2.9.4-nightly-20150209/lwjgl-2.9.4-nightly-20150209.jar}"
if [ ! -f "$library" ]; then echo "LWJGL metadata jar missing; set R10_LWJGL (native GL is replaced by the recorder)" >&2; exit 2; fi
python "$root/tools/oracle10/headless/extract_lwjgl10.py" "$library" "$out/lwjgl-metadata" || exit 1
cp="$out:$jar:$out/lwjgl-metadata"
if command -v cygpath >/dev/null 2>&1; then cp="$(cygpath -w "$out");$(cygpath -w "$jar");$(cygpath -w "$out/lwjgl-metadata")"; fi
if [ "${R10_RENDER_SKIP_BUILD:-0}" != 1 ]; then
  cargo build --release --manifest-path "$root/Cargo.toml" || exit 1
fi
javac -cp "$cp" -d "$out" \
  "$root/tools/oracle10/headless/org/lwjgl/opengl/GL11.java" \
  "$root/tools/oracle10/headless/org/lwjgl/opengl/GL12.java" \
  "$root/tools/oracle10/headless/org/lwjgl/opengl/GL13.java" \
  "$root/tools/oracle10/headless/net/minecraft/src/Tessellator.java" \
  "$root/tools/oracle10/src/net/minecraft/src/Mode_render_models10.java" \
  "$root/tools/oracle10/src/net/minecraft/src/Mode_render_modeldraw10.java" \
  "$root/tools/oracle10/src/net/minecraft/src/Mode_render_geometry10.java" \
  "$root/tools/oracle10/src/net/minecraft/src/Mode_render_tiles10.java" \
  "$root/tools/oracle10/src/net/minecraft/src/Mode_render_effects10.java" \
  "$root/tools/oracle10/src/net/minecraft/src/Mode_render_living10.java" \
  "$root/tools/oracle10/src/net/minecraft/src/Mode_render_dragonbeam10.java" \
  "$root/tools/oracle10/src/net/minecraft/src/Mode_render_dragon10.java" \
  "$root/tools/oracle10/src/net/minecraft/src/Mode_render_boss10.java" \
  "$root/tools/oracle10/src/net/minecraft/src/Mode_render_registry10.java" || exit 1
fail=0
beta() { "$root/target/release/beta" --mc 1.0 "$@"; }
run() {
  local label="$1" mode="$2" flag="$3"; shift 3
  local path="$out/$label.bin"
  echo "== $label"
  if java -cp "$cp" "net.minecraft.src.Mode_$mode" "$@" "$path" && beta "$flag" "$path"; then :; else fail=1; fi
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
