#!/usr/bin/env bash
# Release milk uses drinking and has no ray; check all bucket branches again.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
source "$root/tools/r11/verify_common10.sh"
fail=0
for kind in buckethit7 bucketray7; do
 echo "== entities7: $kind"
 if r10_oracle tools/oracle11/oracle.sh itembucket7_181 "$kind" 0 "bucketfix10-$kind.bin"; then
  r10_compare "$root/target/release/entity_compare11" "$kind" 0 "$root/tools/oracle11/out/bucketfix10-$kind.bin" || fail=1
 else fail=1; fi
done
exit "$fail"
