#!/usr/bin/env bash
# Original bytecode vs Rust: physics, riding, inventory, NBT and renderer inputs.
set -euo pipefail
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
cd "$root"
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen181 -p client; fi
oracle=tools/oracle181/oracle.sh
fail=0
for scene in rails train boats riding nbt longrail ridingnbt placing; do
  echo "== vehicles: $scene, 240 ticks, canonical NBT and final world"
  file="vehicles_${scene}.bin"
  if "$oracle" vehicleintegration181 "$scene" 240 "$file" >"tools/oracle181/out/$file.log" 2>&1; then
    target/release/compare181 vehicles "$scene" 240 "tools/oracle181/out/$file" || fail=1
  else
    cat "tools/oracle181/out/$file.log"; echo "ORACLE FAILED: vehicles $scene"; fail=1
  fi
done
echo "== vehicles: inventory and canonical NBT, 10000 operations"
if "$oracle" vehicleinventory vehicleinventory.bin >tools/oracle181/out/vehicleinventory.bin.log 2>&1; then
  target/release/compare181 vehicleinventory tools/oracle181/out/vehicleinventory.bin || fail=1
else
  cat tools/oracle181/out/vehicleinventory.bin.log; echo "ORACLE FAILED: vehicle inventory"; fail=1
fi
echo "== vehicles: original renderer inputs"
"$oracle" renderintegration181 vehiclerenderers vehicles-renderers.bin && target/release/beta --mc b1.8.1 --vehiclerenderercheck tools/oracle181/out/vehicles-renderers.bin || fail=1
[ "$fail" -eq 0 ] && echo "VEHICLES ALL CASES EXACT" || echo "VEHICLE CASES DIFFER"
exit "$fail"
