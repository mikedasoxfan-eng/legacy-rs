#!/usr/bin/env bash
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
export JAVA_TOOL_OPTIONS="${JAVA_TOOL_OPTIONS:-} -Xmx512m"
fail=0
python tools/oracle10/container_cases.py || exit 1
echo "== containers10: clicks and recipes"
if tools/oracle10/oracle.sh containerreplay "$(cygpath -w "$root/target/r10/container-cases.txt")" container-replay.bin; then
r10_compare "$root/target/release/container_compare10" target/r10/container-cases.txt tools/oracle10/out/container-replay.bin || fail=1
else echo "ORACLE FAILED: container replay"; fail=1; fi
echo "== containers10: furnace and dispenser tiles"
if tools/oracle10/oracle.sh containertiles 1000 container-tiles.bin; then
r10_compare "$root/target/release/container_compare10" tiles 1000 tools/oracle10/out/container-tiles.bin || fail=1
else echo "ORACLE FAILED: container tiles"; fail=1; fi
exit "$fail"
