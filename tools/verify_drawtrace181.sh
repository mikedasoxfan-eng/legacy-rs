#!/usr/bin/env bash
# Complete original draws; this verifies capture integrity, not renderer parity.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p "$root/target"
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
for kind in objects pickup pickupfast particles pickuplight; do
  echo "== complete 1.8.1 $kind fixture (capture consistency)"
  file="$root/tools/oracle181/out/drawtrace_${kind}.bin"
  "$root/tools/oracle181/oracle.sh" drawtrace181 "$kind" "drawtrace_${kind}.bin"
  python "$root/tools/b181/inspect_drawtrace181.py" "$file"
done
