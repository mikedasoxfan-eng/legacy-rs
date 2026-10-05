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
for scene in rails train boats riding nbt longrail ridingnbt placing; do
  file="vehicles_${scene}.bin"
  "$oracle" vehicles "$scene" 240 "$file" >/dev/null
  target/release/compare181 vehicles "$scene" 240 "tools/oracle181/out/$file"
done
"$oracle" vehicleinventory vehicleinventory.bin >/dev/null
target/release/compare181 vehicleinventory tools/oracle181/out/vehicleinventory.bin
"$oracle" vehiclerenderers vehicles-renderers.bin >/dev/null
target/release/beta --vehiclerenderercheck tools/oracle181/out/vehicles-renderers.bin
echo "VEHICLES ALL CASES EXACT"
