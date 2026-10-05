#!/usr/bin/env bash
# Original bytecode vs Rust: physics, riding, inventory, NBT and renderer inputs.
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
cd "$root"
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen181 -p client || exit 1; fi
fail=0
oracle=tools/oracle181/oracle.sh
for scene in rails train boats riding nbt longrail ridingnbt placing; do
  file="vehicles_${scene}.bin"
  echo "== vehicles181: $scene"
  "$oracle" vehicles "$scene" 240 "$file" >/dev/null && target/release/compare181 vehicles "$scene" 240 "tools/oracle181/out/$file" || { echo "DIFFERS: vehicles $scene"; fail=1; }
done
echo "== vehicles181: inventory"
"$oracle" vehicleinventory vehicleinventory.bin >/dev/null && target/release/compare181 vehicleinventory tools/oracle181/out/vehicleinventory.bin || { echo "DIFFERS: vehicles inventory"; fail=1; }
echo "== vehicles181: renderers"
"$oracle" vehiclerenderers vehicles-renderers.bin >/dev/null && target/release/beta --mc b1.8.1 --vehiclerenderercheck tools/oracle181/out/vehicles-renderers.bin || { echo "DIFFERS: vehicles renderers"; fail=1; }
[ $fail -eq 0 ] && echo "VEHICLES ALL CASES EXACT"
exit $fail
