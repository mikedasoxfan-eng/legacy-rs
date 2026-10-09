#!/usr/bin/env bash
# Preserve stderr from the original release biome search failure.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
source "$root/tools/r11/verify_common10.sh"
lock="$root/target/r11/verify11.lock"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
 mkdir "$lock" || exit 3
 trap 'rmdir "$lock" 2>/dev/null' EXIT
fi
fail=0
for seed in 0 3257840388504953787 -4172144997902289642 9223372036854775807 -9223372036854775808; do
 echo "== biomes seed $seed"
 file="harness-fix10-biomes-$seed.bin"
 if r10_oracle "$root/tools/oracle11/oracle.sh" biomes181 "$seed" "$file"; then
  r10_compare "$root/target/release/terrain_compare11" biomes "$seed" "$root/tools/oracle11/out/$file" || fail=1
 else fail=1; fi
done
exit "$fail"
