#!/usr/bin/env bash
# Round eleven: complete remaining StatFileWriter and RenderEngine boundaries.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
cd "$root"
lock="$root/target/r10/verify10.lock"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  mkdir -p target/r10
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen10 -p client || exit 1; fi
fail=0
for spec in "statfile10_181 statfile10check" "statfile11_181 statfile11check" "renderengine10_181 renderengine10check" "renderengine11_181 renderengine11check"; do
  read -r mode check <<<"$spec"
  echo "== player-gui round 11: $mode"
  # Both sides print the original's own messages for malformed stats files (CHECKSUM MISMATCH,
  # invalid syntax); full output goes to target/r10/, the section shows the check's summary.
  if bash tools/oracle10/oracle.sh "$mode" "$mode-java.bin" >"target/r10/$mode-oracle.log" 2>&1; then
    r10_client target/release/beta --mc 1.0 "--$check" "tools/oracle10/out/$mode-java.bin" >"target/r10/$mode-check.log" 2>&1; rc=$?
    grep -v -E 'is not a valid stat|invalid syntax|CHECKSUM MISMATCH' "target/r10/$mode-check.log" | tail -n 3
    [ $rc -eq 0 ] || { echo "DIFFERS: $mode"; fail=1; }
  else
    tail -n 5 "target/r10/$mode-oracle.log"; echo "ORACLE FAILED: $mode"; fail=1
  fi
done
[ $fail -eq 0 ] && echo "PLAYER-GUI ROUND 11 EXACT" || echo "PLAYER-GUI ROUND 11 DIFFERS"
exit $fail
