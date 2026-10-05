#!/usr/bin/env bash
# Round eleven: complete remaining StatFileWriter and RenderEngine boundaries.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p target
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen181 -p client || exit 1; fi
fail=0
for spec in "statfile10_181 statfile10check" "statfile11_181 statfile11check" "renderengine10_181 renderengine10check" "renderengine11_181 renderengine11check"; do
  read -r mode check <<<"$spec"
  echo "== player-gui round 11: $mode"
  bash tools/oracle181/oracle.sh "$mode" "$mode-java.bin" && target/release/beta --mc b1.8.1 "--$check" "tools/oracle181/out/$mode-java.bin" || fail=1
done
[ $fail -eq 0 ] && echo "PLAYER-GUI ROUND 11 EXACT" || echo "PLAYER-GUI ROUND 11 DIFFERS"
exit $fail
