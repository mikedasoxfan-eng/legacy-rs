#!/usr/bin/env bash
# Original bytecode vs Rust: physics, riding, inventory, NBT and renderer inputs.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r11" "$root/tools/oracle11/out"
source "$root/tools/r11/verify_common10.sh"
# One run at a time per tree (two runs would overwrite each other's oracle output).
lock="$root/target/r11/verify11.lock"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
  mkdir -p "$root/target/r11"
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock); remove it if stale" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY11_LOCKED=1
fi
beta() { r10_client "$root/target/release/beta" --mc 1.1 "$@"; }
cd "$root"
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen11 -p client || exit 1; fi
oracle=tools/oracle11/oracle.sh
mkdir -p tools/oracle11/out  # a fresh checkout has no output folder yet
fail=0
for scene in rails train boats riding nbt longrail ridingnbt placing; do
  file="vehicles_${scene}.bin"
  echo "== vehicles10: $scene"
  if r10_oracle "$oracle" vehicles "$scene" 240 "$file" >"tools/oracle11/out/$file.log" 2>&1; then
    r10_compare target/release/compare11 vehicles "$scene" 240 "tools/oracle11/out/$file" || { echo "DIFFERS: vehicles $scene"; fail=1; }
  else
    cat "tools/oracle11/out/$file.log"; echo "ORACLE FAILED: vehicles $scene"; fail=1
  fi
done
echo "== vehicles10: inventory"
if r10_oracle "$oracle" vehicleinventory vehicleinventory.bin >tools/oracle11/out/vehicleinventory.bin.log 2>&1; then
  r10_compare target/release/compare11 vehicleinventory tools/oracle11/out/vehicleinventory.bin || { echo "DIFFERS: vehicles inventory"; fail=1; }
else
  cat tools/oracle11/out/vehicleinventory.bin.log; echo "ORACLE FAILED: vehicle inventory"; fail=1
fi
echo "== vehicles10: renderers"
r10_oracle "$oracle" renderintegration181 vehiclerenderers vehicles-renderers.bin && r10_client target/release/beta --mc 1.1 --vehiclerenderercheck tools/oracle11/out/vehicles-renderers.bin || { echo "DIFFERS: vehicles renderers"; fail=1; }
[ $fail -eq 0 ] && echo "VEHICLES ALL CASES EXACT" || echo "VEHICLE CASES DIFFER"
exit $fail
