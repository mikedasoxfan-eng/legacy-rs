#!/usr/bin/env bash
# Complete original draws; this verifies capture integrity, not renderer parity.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
lock="$root/target/r10/verify10.lock"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  mkdir -p "$root/target/r10"
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
for kind in objects pickup pickupfast particles pickuplight; do
  echo "== complete 1.8.1 $kind fixture (capture consistency)"
  file="$root/tools/oracle10/out/drawtrace_${kind}.bin"
  "$root/tools/oracle10/oracle.sh" drawtrace181 "$kind" "drawtrace_${kind}.bin"
  python "$root/tools/b181/inspect_drawtrace181.py" "$file"
done
