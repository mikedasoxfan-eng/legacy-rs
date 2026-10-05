#!/usr/bin/env bash
# Original bytecode vs Rust: physics, riding, inventory, NBT and renderer inputs.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen -p client; fi
oracle=tools/oracle/oracle.sh
for scene in rails train boats riding nbt longrail ridingnbt placing; do
  file="vehicles_${scene}.bin"
  "$oracle" vehicles "$scene" 240 "$file" >/dev/null
  target/release/compare vehicles "$scene" 240 "tools/oracle/out/$file"
done
"$oracle" vehicleinventory vehicleinventory.bin >/dev/null
target/release/compare vehicleinventory tools/oracle/out/vehicleinventory.bin
"$oracle" vehiclerenderers vehicles-renderers.bin >/dev/null
target/release/beta --vehiclerenderercheck tools/oracle/out/vehicles-renderers.bin
echo "VEHICLES ALL CASES EXACT"
