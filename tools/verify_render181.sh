#!/usr/bin/env bash
# Compare the Rust generator against the Java oracle (Mojang's original b1.8.1 code
# running on the launcher's Java 8u51) across a fixed set of seeds and scenarios.
# Usage: tools/verify181.sh [quick]     exit code 0 only if every case is an exact match
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
# One run at a time per tree (two runs would overwrite each other's oracle output).
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p "$root/target"
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock); remove it if stale" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
beta() { "$root/target/release/beta" --mc b1.8.1 "$@"; }
oracle="$root/tools/oracle181/oracle.sh"
out="$root/tools/oracle181/out"
cargo build --release -q --manifest-path "$root/Cargo.toml" -p worldgen181 -p client || exit 1
cmp="$root/target/release/compare181"
quick="${1:-}"

fail=0
run() { # label, oracle args..., then "--", compare args (oracle output file appended)
  local label="$1"; shift
  local oargs=() cargs=()
  while [ "$1" != "--" ]; do oargs+=("$1"); shift; done; shift
  cargs=("$@")
  local f="case_$(echo "${oargs[*]}" | tr ' ' '_').bin"
  echo "== $label"
  if "$oracle" "${oargs[@]:0:1}" "${oargs[@]:1}" "$f" >/dev/null && "$cmp" "${cargs[@]}" "$out/$f"; then :; else fail=1; fi
  rm -f "$out/$f" "$out/$f.txt" "$out/$f.events"
}
run_extra() { # like run, but the oracle takes an argument after the output file
  local label="$1" extra="$2"; shift 2
  local oargs=() cargs=()
  while [ "$1" != "--" ]; do oargs+=("$1"); shift; done; shift
  cargs=("$@")
  local f="case_$(echo "${oargs[*]}_$extra" | tr ' :,' '_xc' | cut -c1-80).bin"
  echo "== $label"
  if "$oracle" "${oargs[@]}" "$f" "$extra" >/dev/null && "$cmp" "${cargs[@]}" "$out/$f" "$extra"; then :; else fail=1; fi
  rm -f "$out/$f" "$out/$f.txt" "$out/$f.events"
}
stats_rows() { grep -E '^(-?[0-9]+ -?[0-9]+|null)$' || true; }

# ---- rendering: RenderBlocks vertex output (smooth lighting, Fancy) over 11x11 chunks
mesh_cases=("25 1" "1 1")   # snowy taiga; desert with cactus
[ "$quick" != quick ] && mesh_cases+=("-1 -1" "0 1")
for c in "${mesh_cases[@]}"; do
  read -r seed wr <<<"$c"
  for mode in "fancy on" "fast on" "fancy off" "fast off"; do
    read -r graphics ao <<<"$mode"
    echo "== chunk meshes: seed $seed, world rand $wr, $graphics, AO $ao"
    if "$oracle" rendermesh "$seed" "$wr" mesh_case.bin auto 5 "$graphics" "$ao" >"$out/mesh_case.log" 2>&1 &&
       beta --meshcheck "$seed" "$wr" "$out/mesh_case.bin" "$graphics" "$ao" >"$out/mesh_compare.log" 2>&1; then
      tail -1 "$out/mesh_compare.log"
    else
      cat "$out/mesh_case.log" "$out/mesh_compare.log"
      echo "MESHES DIFFER"; fail=1
    fi
    rm -f "$out/mesh_case.bin" "$out/mesh_case.log" "$out/mesh_compare.log"
  done
done

# ---- every RenderBlocks render type: each block id x metadata plus neighbour arrangements
# (tools/oracle181/make_showcase.py), in all four graphics / smooth-lighting combinations
showcase="$root/tools/oracle181/showcase.txt"
for m in "fancy on" "fancy off" "fast on" "fast off"; do
  [ "$quick" = quick ] && [ "$m" != "fancy on" ] && continue
  for stereo in normal anaglyph; do
  set -- $m
  echo "== block showcase: $m, $stereo"
  if "$oracle" rendershowcase "$(cygpath -w "$showcase")" showcase_mesh.bin "$1" "$2" "$stereo" >/dev/null 2>&1     && beta --meshcheck 0 0 "$out/showcase_mesh.bin" "$1" "$2" "showcase=$showcase" "$stereo" | tail -1 | grep -q "MESHES EXACT"; then
    echo "MESHES EXACT"; else echo "MESHES DIFFER"; fail=1; fi
  rm -f "$out/showcase_mesh.bin"
  done
done
if [ "$quick" != quick ]; then
  echo "== Nether terrain meshes (fire, glowstone, the Nether light table)"
  if "$oracle" rendermeshnether 5 1 mesh_nether.bin auto 2 fancy on >/dev/null 2>&1     && beta --meshcheck 5 1 "$out/mesh_nether.bin" fancy on nether | tail -1 | grep -q "MESHES EXACT"; then
    echo "MESHES EXACT"; else echo "MESHES DIFFER"; fail=1; fi
  rm -f "$out/mesh_nether.bin"
fi

# ---- first-person camera, hand, overlays, player armor and targeting
# Original bytecode GL inputs, projection matrices and Tessellator vertex bits.
echo "== first-person view, player and block targeting"
if "$oracle" firstperson firstperson_case.bin && beta --firstpersoncheck "$out/firstperson_case.bin"; then :; else fail=1; fi
rm -f "$out/firstperson_case.bin"

# ---- sky, fog and cloud colors, sun angle, night darkness
echo "== renderer window sizes and Java negative-coordinate wrapping"
for distance in 0 1 2 3; do
  for position in 0 -1 8 -8 208 -209 30000000 -30000000; do
    j=$("$oracle" videowindow "$distance" "$position"); r=$(beta --videowindow "$distance" "$position")
    [ "$j" = "$r" ] || { echo "window differs: $distance $position"; echo "java $j"; echo "rust $r"; fail=1; }
  done
done
echo "== sky colors"
for args in "3257840388504953787 -39 65 6000 0.0" "0 1000 -2000 13000 0.5" "123 -500 77 23500 0.25" "42 99999 -12345 18000 0.9" "1 0 0 12500 0.75" \
  "1 0 0 6000 0.5 1 1 2" "0 1000 -2000 13000 0.5 0.75 0.4 1" \
  "123 -500 77 23500 0.25 0.33 0.9 0" "42 99999 -12345 18000 0.9 1 1 1" \
  "2 -1 -17 12500 0.0 0.1 1 2 0.9 0.3" "11 0 0 0 1.0 1 0.7 1 0.2 0.9" \
  "42 0 0 6000 0.25 1 1 2 0.9 0.7 0.0 0.1"; do
  for distance in 0 1 2 3; do
    read -r -a skyargs <<<"$args"
    defaults=(0 0 0 0 0 1 1)
    for ((i=${#skyargs[@]}; i<12; i++)); do
      # Previous rain/thunder default to current values, like the oracle.
      if [ "$i" = 8 ]; then skyargs+=("${skyargs[5]}")
      elif [ "$i" = 9 ]; then skyargs+=("${skyargs[6]}")
      else skyargs+=("${defaults[i-5]}"); fi
    done
    j=$("$oracle" skycolor "${skyargs[@]}" "$distance"); r=$(beta --skycolor "${skyargs[@]}" "$distance")
  [ "$j" = "$r" ] || { echo "sky differs for $args"; echo " java $j"; echo " rust $r"; fail=1; }
  done
done
# The Nether: WorldProviderHell's fog color, celestial angle 0.5, fog from the eye.
for args in "5 10 -20 6000 0.5" "5 300 40 18000 0.25 1 0 0 1 0" "9 -7 7 100 0.75 0.4 0.6 0 0.4 0.6 0.3 0.8"; do
  for distance in 0 2 3; do
    read -r -a skyargs <<<"$args"
    defaults=(0 0 0 0 0 1 1)
    for ((i=${#skyargs[@]}; i<12; i++)); do
      if [ "$i" = 8 ]; then skyargs+=("${skyargs[5]}")
      elif [ "$i" = 9 ]; then skyargs+=("${skyargs[6]}")
      else skyargs+=("${defaults[i-5]}"); fi
    done
    j=$("$oracle" skycolornether "${skyargs[@]}" "$distance"); r=$(beta --skycolor-nether "${skyargs[@]}" "$distance")
  [ "$j" = "$r" ] || { echo "nether sky differs for $args"; echo " java $j"; echo " rust $r"; fail=1; }
  done
done
[ $fail -eq 0 ] && echo "sky colors exact"

# RenderGlobal cloud geometry, camera wrapping, above/below and within the cloud layer.
echo "== Fast and Fancy clouds"
for args in "1 0.5 70 0.5 100 0.75 6000 0 0" "42 -20.25 110 -31.75 123 0.5 13000 0.75 0.4" \
  "2 30000.25 120 -50000.75 2147483647 0.25 23500 1 1" "11 2048 107 0 -2147483648 1.0 18000 0.1 0.9"; do
  for graphics in fancy fast; do
    for stereo in normal anaglyph; do
      if "$oracle" cloudmesh $args "$graphics" cloud_case.bin "$stereo" && beta --cloudcheck $args "$graphics" "$out/cloud_case.bin" "$stereo"; then :; else fail=1; fi
    done
  done
done
rm -f "$out/cloud_case.bin"

echo "== sky GL colors and sunrise vertex colors, anaglyph on/off"
if "$oracle" skydraw sky_draw_case.bin && beta --skydrawcheck "$out/sky_draw_case.bin"; then :; else fail=1; fi
rm -f "$out/sky_draw_case.bin"

# --- render-world (b1.8.1) ---
echo "== lightmap texels (EntityRenderer.func_35808_e): day, night, rain, thunder, flashes, flicker, brightness"
if "$oracle" lightmap lightmap_case.bin >/dev/null 2>&1 && beta --lightmapcheck "$out/lightmap_case.bin" | tail -1 | grep -q " 0 differences"; then echo "LIGHTMAP EXACT"; else echo "lightmap DIFFERS"; fail=1; fi
# --- end render-world ---
# --- world-blocks (b1.8.1) ---
bash "$root/tools/verify_world_blocks181.sh" --built || fail=1
# --- end world-blocks ---
# --- terrain (b1.8.1) ---
bash "$root/tools/verify_terrain181.sh" --built || fail=1
# --- end terrain ---
[ $fail -eq 0 ] && echo "ALL CASES EXACT" || echo "SOME CASES DIFFER"
exit $fail

[ $fail -eq 0 ] && echo "ALL CASES EXACT" || echo "SOME CASES DIFFER"
exit $fail
