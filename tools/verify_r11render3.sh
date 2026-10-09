#!/usr/bin/env bash
# Release rendering, original-bytecode comparisons on the shared oracle11 harness.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
out="$root/target/oracle11-render3"
mkdir -p "$out"
if [ "${1:-}" != --built ]; then cargo build --release --manifest-path "$root/Cargo.toml" || exit 1; fi
oracle="$root/tools/oracle11/oracle.sh"
beta() { "$root/target/release/beta" --mc 1.1 "$@"; }
layout="$out/blocks.txt"
python "$root/tools/r11/render_showcase3.py" "$layout" || exit 1
java_layout="$layout"
if command -v cygpath >/dev/null 2>&1; then java_layout="$(cygpath -w "$layout")"; fi
fail=0
for graphics in fancy fast; do
  for ao in on off; do
    for stereo in normal anaglyph; do
      label="RenderBlocks_world_${graphics}_${ao}_${stereo}"
      echo "== $label"
      if "$oracle" render_blocks10 "$java_layout" "$out/$label.bin" "$graphics" "$ao" "$stereo" >"$out/$label.oracle.log" 2>&1 &&
         beta --meshcheck 0 0 "$out/$label.bin" "$graphics" "$ao" "showcase=$layout" "$stereo" >"$out/$label.rust.log" 2>&1; then
        tail -1 "$out/$label.rust.log"
      else
        tail -n 25 "$out/$label.oracle.log" "$out/$label.rust.log"
        fail=1
      fi
    done
  done
done
echo "== RenderBlocks_inventory_lily_pad_dragon_egg"
if "$oracle" render_blockinventory10 "$out/inventory.bin" && beta --render-blockinventory10-check "$out/inventory.bin"; then :; else fail=1; fi
echo "== RenderGlobal_dimension_sky_phases_transforms_horizon"
if "$oracle" render_global10 "$out/global.bin" && beta --render-global10-check "$out/global.bin"; then :; else fail=1; fi
if [ "$fail" = 0 ]; then echo "R10 RENDER ROUND 3 EXACT"; else echo "R10 RENDER ROUND 3 DIFFER"; fi
exit "$fail"
