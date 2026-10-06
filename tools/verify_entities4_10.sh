#!/usr/bin/env bash
# Round 4 networking, impacts, rendering and particle lifecycle against original Mojang 8u51 bytecode.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
cd "$root"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  mkdir -p target/r10
  if ! mkdir target/r10/verify10.lock 2>/dev/null; then echo "another 1.0 verify is running" >&2; exit 3; fi
  trap 'rmdir target/r10/verify10.lock 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -p worldgen10 -p client -j 2 || exit 1; fi
oracle="$root/tools/oracle10/oracle.sh"
out="$root/tools/oracle10/out"
cmp="$root/target/release/entity_compare10.exe"
beta="$root/target/release/beta.exe"
fail=0
for kind in creeperdeath4 sun4 network4 lightning4 retaliation4 impact4 flying4 explosionblocks4 creeper4; do
  echo "== entities4: $kind"
  if r10_oracle "$oracle" entityround4_181 "$kind" 0 "${kind}.bin" && r10_compare "$cmp" "$kind" 0 "$out/${kind}.bin"; then :; else fail=1; fi
done
for spec in "generaliving4_181 --newlivingcheck" "playerliving4_181 --playerlivingcheck" "effectlifecycle4_181 --effectlifecyclecheck" "livingrenderers --livingrenderercheck" "rendereffects --effectcheck" "particleeffects --particlefxcheck"; do
  read -r mode option <<< "$spec"
  echo "== entities4: $mode"
  if r10_oracle "$oracle" "$mode" "${mode}4.bin" && r10_client "$beta" --mc 1.0 "$option" "$out/${mode}4.bin"; then :; else fail=1; fi
done
for mode in paintings fishing; do
  echo "== entities4: $mode"
  if r10_oracle "$oracle" "$mode" "${mode}4.bin" && r10_compare "$root/target/release/compare10.exe" "$mode" "$out/${mode}4.bin"; then :; else fail=1; fi
done
# Includes original RenderLightningBolt vertices and deterministic renderer RNG.
if r10_oracle "$oracle" weathermesh 2 0.5 70 0.5 100 0.75 0.8 weather4.bin natural 0 fancy && r10_client "$beta" --mc 1.0 --weathercheck 2 0.5 70 0.5 100 0.75 0.8 "$out/weather4.bin" natural 0 fancy; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES4 ALL FOCUSED CASES EXACT" || echo "ENTITIES4 SOME CASES DIFFER"
exit "$fail"
