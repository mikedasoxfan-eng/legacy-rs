#!/usr/bin/env bash
# Original-bytecode checks for the second terrain stage. Uses the shared lock.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  if ! mkdir "$root/target/r10/verify10.lock" 2>/dev/null; then echo "another 1.0 verify is running" >&2; exit 3; fi
  trap 'rmdir "$root/target/r10/verify10.lock" 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release --manifest-path "$root/Cargo.toml" -p worldgen10 --bin terrain_compare10 || exit 1; fi
oracle="$root/tools/oracle10/oracle.sh"
cmp="$root/target/release/terrain_compare10"
out="$root/tools/oracle10/out/terrain-round2.bin"
fail=0
for seed in 0 42 3257840388504953787 -4172144997902289642 9223372036854775807 -9223372036854775808; do
  echo "== StructureComponent helpers seed $seed"
  r10_oracle "$oracle" component181 "$seed" terrain-round2.bin >/dev/null && r10_compare "$cmp" component "$seed" "$out" || fail=1
  for mode in layout map blocks village-layout village-map village-blocks; do
    echo "== structures $mode seed $seed"
    r10_oracle "$oracle" structures181 "$mode" "$seed" terrain-round2.bin >/dev/null && r10_compare "$cmp" structures "$mode" "$seed" "$out" || fail=1
  done
  for mode in sky skygrid dungeon lifetime colors locations; do
    echo "== terrain round2 $mode seed $seed"
    r10_oracle "$oracle" terrainround2 "$mode" "$seed" terrain-round2.bin >/dev/null && r10_compare "$cmp" round2 "$mode" "$seed" "$out" || fail=1
  done
done
[ "$fail" -eq 0 ] && echo "ROUND 2 TERRAIN CASES EXACT" || echo "ROUND 2 TERRAIN CASES DIFFER"
exit "$fail"
