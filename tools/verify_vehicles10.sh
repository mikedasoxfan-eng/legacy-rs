#!/usr/bin/env bash
# Original bytecode vs Rust: physics, riding, inventory, NBT and renderer inputs.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
# One run at a time per tree (two runs would overwrite each other's oracle output).
lock="$root/target/r10/verify10.lock"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  mkdir -p "$root/target/r10"
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock); remove it if stale" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
beta() { r10_client "$root/target/release/beta" --mc 1.0 "$@"; }
cd "$root"
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen10 -p client || exit 1; fi
oracle=tools/oracle10/oracle.sh
mkdir -p tools/oracle10/out  # a fresh checkout has no output folder yet
fail=0
for scene in rails train boats riding nbt longrail ridingnbt placing; do
  file="vehicles_${scene}.bin"
  echo "== vehicles10: $scene"
  if r10_oracle "$oracle" vehicles "$scene" 240 "$file" >"tools/oracle10/out/$file.log" 2>&1; then
    r10_compare target/release/compare10 vehicles "$scene" 240 "tools/oracle10/out/$file" || { echo "DIFFERS: vehicles $scene"; fail=1; }
  else
    cat "tools/oracle10/out/$file.log"; echo "ORACLE FAILED: vehicles $scene"; fail=1
  fi
done
echo "== vehicles10: inventory"
if r10_oracle "$oracle" vehicleinventory vehicleinventory.bin >tools/oracle10/out/vehicleinventory.bin.log 2>&1; then
  r10_compare target/release/compare10 vehicleinventory tools/oracle10/out/vehicleinventory.bin || { echo "DIFFERS: vehicles inventory"; fail=1; }
else
  cat tools/oracle10/out/vehicleinventory.bin.log; echo "ORACLE FAILED: vehicle inventory"; fail=1
fi
echo "== vehicles10: renderers"
r10_oracle "$oracle" renderintegration181 vehiclerenderers vehicles-renderers.bin && r10_client target/release/beta --mc 1.0 --vehiclerenderercheck tools/oracle10/out/vehicles-renderers.bin || { echo "DIFFERS: vehicles renderers"; fail=1; }
[ $fail -eq 0 ] && echo "VEHICLES ALL CASES EXACT" || echo "VEHICLE CASES DIFFER"
exit $fail
