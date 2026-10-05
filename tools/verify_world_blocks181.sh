#!/usr/bin/env bash
# Focused world-blocks checks against original Beta 1.8.1 bytecode.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p "$root/target"
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
if [ "${1:-}" != --built ]; then
  cargo build --release -q --manifest-path "$root/Cargo.toml" -p worldgen181 || exit 1
fi
oracle="$root/tools/oracle181/oracle.sh"
cmp="$root/target/release/compare181"
out="$root/tools/oracle181/out"
fail=0
for suite in plants geometry drops saves lights chests helpers fluids teleporter items actions; do
  echo "== world-blocks b1.8.1: $suite"
  if "$oracle" worldblocks181 "$suite" "wb181-$suite.bin" && "$cmp" worldblocks181 "$suite" "$out/wb181-$suite.bin"; then :; else fail=1; fi
done
echo "== world-blocks b1.8.1: registry"
if "$oracle" wbregistry wb181-registry.bin && "$cmp" worldblocks181 registry "$out/wb181-registry.bin"; then :; else fail=1; fi
echo "== world-blocks b1.8.1: definitions"
if "$oracle" blocktable wb181-table.rs && "$oracle" collisiontable wb181-geometry.rs \
   && python "$root/tools/oracle181/check_worldblocks181_tables.py" "$out/wb181-table.rs" "$out/wb181-geometry.rs"; then :; else fail=1; fi
echo "== world-blocks b1.8.1: region cross-read"
if "$cmp" regions181 write "$out/wb181-rust.mcr" \
   && "$oracle" wbregions "$(cygpath -w "$out/wb181-rust.mcr")" wb181-regions.bin \
   && "$cmp" regions181 check "$out/wb181-regions.bin" "$out/wb181-rust.mcr"; then :; else fail=1; fi
# Round two: separate original-bytecode modes keep the shared oracle untouched.
for suite in mushrooms chestlife math storage silverfish visual colors spawn relocation; do
  echo "== world-blocks b1.8.1: $suite"
  if "$oracle" "wb$suite" "wb181-$suite.bin" && "$cmp" worldblocks181 "$suite" "$out/wb181-$suite.bin"; then :; else fail=1; fi
done
# Round three: cache, coordinate rendering, save lifecycle and tick replays.
for suite in cache textures weather arrays savelife sides nextticks frost fileclasses; do
  echo "== world-blocks b1.8.1: $suite"
  if "$oracle" "wb$suite" "wb181-$suite.bin" && "$cmp" worldblocks181 "$suite" "$out/wb181-$suite.bin"; then :; else fail=1; fi
done
# Round four: placement/harvest/fire callbacks and process-wide save compatibility.
for suite in placement4 fire4 harvest4 regioncache4 oldformat4 sounds4 emptycache4 plates4 shears4 placepred4 growth4 doors4 furnace4 dispenser4 activation4 record4; do
  echo "== world-blocks b1.8.1: $suite"
  if "$oracle" "wb$suite" "wb181-$suite.bin" && "$cmp" worldblocks181 "$suite" "$out/wb181-$suite.bin"; then :; else fail=1; fi
done
# Round five: legacy saves, piston transitions, tile removal, hash and fluid behavior.
for suite in legacy5 piston5 removal5 mchash5 chestopen5 flow5; do
  echo "== world-blocks b1.8.1: $suite"
  if "$oracle" "wb$suite" "wb181-$suite.bin" && "$cmp" worldblocks181 "$suite" "$out/wb181-$suite.bin"; then :; else fail=1; fi
done
# Round six: complete drop surface, slab merges, powered doors and conversion lifecycle.
for suite in slabs6 trapdoors6 doors6 attached6 drops6 conversion6 push6 unload6 music6 materials6 dispenser6 base6 piston6; do
  echo "== world-blocks b1.8.1: $suite"
  if "$oracle" "wb$suite" "wb181-$suite.bin" && "$cmp" worldblocks181 "$suite" "$out/wb181-$suite.bin"; then :; else fail=1; fi
done
# Round seven: furnace/container callbacks, machine replay and remaining simple blocks.
for suite in furnace7 inventory7 container7 machines7 ore7 thin7; do
  echo "== world-blocks b1.8.1: $suite"
  if "$oracle" "wb$suite" "wb181-$suite.bin" && "$cmp" worldblocks181 "$suite" "$out/wb181-$suite.bin"; then :; else fail=1; fi
done
# Integrator: BlockCake through right and left click (onBlockClicked eats too).
echo "== world-blocks b1.8.1: cake181"
if "$oracle" wbcake181 wb181-cake181.bin && "$cmp" worldblocks181 cake181 "$out/wb181-cake181.bin"; then :; else fail=1; fi
[ $fail -eq 0 ] && echo "WORLD-BLOCKS FOCUSED CASES EXACT" || echo "WORLD-BLOCKS FOCUSED CASES DIFFER"
exit $fail
