#!/usr/bin/env bash
# Original-bytecode stronghold and provider checks for terrain round 3.
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
if [ "${1:-}" != --built ]; then cargo build --release --manifest-path "$root/Cargo.toml" -p worldgen10 --bin terrain_compare10 --bin compare10 || exit 1; fi
oracle="$root/tools/oracle10/oracle.sh"
cmp="$root/target/release/terrain_compare10"
out="$root/tools/oracle10/out/terrain-round3.bin"
fail=0
save_root=$(mktemp -d "$root/target/r10/terrain3-save.XXXXXX") || exit 1
for seed in 0 42 3257840388504953787 -4172144997902289642 9223372036854775807 -9223372036854775808; do
  for mode in layout map blocks wet pieces grid; do
    echo "== stronghold $mode seed $seed"
    r10_oracle "$oracle" strongholds181 "$mode" "$seed" terrain-round3.bin >/dev/null && r10_compare "$cmp" structures "stronghold-$mode" "$seed" "$out" || fail=1
  done
  for mode in longset scan saves; do
    echo "== chunk provider $mode seed $seed"
    r10_oracle "$oracle" provider181 "$mode" "$seed" terrain-round3.bin >/dev/null && r10_compare "$cmp" structures "provider-$mode" "$seed" "$out" || fail=1
  done
  for mode in cache null failure; do
    echo "== alternate chunk cache $mode seed $seed"
    r10_oracle "$oracle" chunkcache181 "$mode" "$seed" terrain-round3.bin >/dev/null 2>&1 && r10_compare "$cmp" structures "chunkcache-$mode" "$seed" "$out" || fail=1
  done
  echo "== NBT compound serialization seed $seed"
  r10_oracle "$oracle" terrainnbt181 "$seed" terrain-round3.bin >/dev/null && r10_compare "$cmp" structures terrain-nbt "$seed" "$out" || fail=1
  java_save="$save_root/java-$seed"
  rust_save="$save_root/rust-$seed"
  world_cmp="$root/target/release/compare10"
  echo "== independently populated region NBT seed $seed (pinned entity/item RNGs)"
  if r10_oracle "$oracle" terrainsave181 "$seed" 1 "$(cygpath -w "$java_save")" terrain-round3-save.bin >/dev/null \
     && "$world_cmp" terrain-makesave "$seed" 1 "$rust_save" \
     && python "$root/tools/oracle10/compare_regions.py" "$java_save" "$rust_save"; then
    echo "INDEPENDENT SAVES EXACT"
  else fail=1; fi
  echo "== Java save / Rust load seed $seed"
  "$world_cmp" savecheck "$java_save" "$root/tools/oracle10/out/terrain-round3-save.bin" || fail=1
  echo "== Rust save / Java load seed $seed"
  r10_oracle "$oracle" readsave "$(cygpath -w "$rust_save")" terrain-round3-read.bin >/dev/null \
    && "$world_cmp" preload-worldonly "$seed" 1 "$root/tools/oracle10/out/terrain-round3-read.bin" || fail=1
done
[ "$fail" -eq 0 ] && echo "ROUND 3 TERRAIN CASES EXACT" || echo "ROUND 3 TERRAIN CASES DIFFER"
exit "$fail"
