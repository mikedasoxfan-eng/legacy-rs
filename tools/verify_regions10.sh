#!/usr/bin/env bash
# Independently save both versions and compare the first serialized NBT field.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
source "$root/tools/r10/verify_common10.sh"
lock="$root/target/r10/verify10.lock"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
 mkdir "$lock" || exit 3
 trap 'rmdir "$lock" 2>/dev/null' EXIT
fi
save_root=$(mktemp -d "$root/target/r10/regions10.XXXXXX") || exit 3
fail=0
for seed in 0 42 3257840388504953787 -4172144997902289642 9223372036854775807 -9223372036854775808; do
 echo "== independently populated region NBT seed $seed (pinned entity/item RNGs)"
 if r10_oracle "$root/tools/oracle10/oracle.sh" terrainsave181 "$seed" 1 "$(cygpath -m "$save_root/java-$seed")" "harness-fix10-regions-$seed.bin" &&
    r10_compare "$root/target/release/compare10" terrain-makesave "$seed" 1 "$save_root/rust-$seed"; then
  python "$root/tools/oracle10/compare_regions.py" "$save_root/java-$seed" "$save_root/rust-$seed" || fail=1
 else fail=1; fi
done
exit "$fail"
