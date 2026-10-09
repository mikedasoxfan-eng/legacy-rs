#!/usr/bin/env bash
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r11" "$root/tools/oracle11/out"
source "$root/tools/r11/verify_common10.sh"
# One run at a time per tree (two runs would overwrite each other's oracle output).
lock="$root/target/r11/verify11.lock"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
  mkdir -p "$root/target/r11"
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock); remove it if stale" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY11_LOCKED=1
fi
beta() { r10_client "$root/target/release/beta" --mc 1.1 "$@"; }
cd "$root"
export JAVA_TOOL_OPTIONS="${JAVA_TOOL_OPTIONS:-} -Xmx512m"
fail=0
python tools/oracle11/container_cases.py || exit 1
echo "== containers10: clicks and recipes"
if tools/oracle11/oracle.sh containerreplay "$(cygpath -w "$root/target/r11/container-cases.txt")" container-replay.bin; then
r10_compare "$root/target/release/container_compare11" target/r11/container-cases.txt tools/oracle11/out/container-replay.bin || fail=1
else echo "ORACLE FAILED: container replay"; fail=1; fi
echo "== containers10: furnace and dispenser tiles"
if tools/oracle11/oracle.sh containertiles 1000 container-tiles.bin; then
r10_compare "$root/target/release/container_compare11" tiles 1000 tools/oracle11/out/container-tiles.bin || fail=1
else echo "ORACLE FAILED: container tiles"; fail=1; fi
exit "$fail"
