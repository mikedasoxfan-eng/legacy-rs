#!/usr/bin/env bash
# Recheck all fixture groups whose constants changed in the initial copy.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
lock="$root/target/r10/verify10.lock"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
 mkdir "$lock" || exit 3
 trap 'rmdir "$lock" 2>/dev/null' EXIT
 export VERIFY10_LOCKED=1
fi
fail=0
bash tools/verify_decorations10.sh || fail=1
bash tools/verify_entities4_10.sh --built || fail=1
bash tools/verify_entities5_10.sh --built || fail=1
bash tools/verify_entities6_10.sh --built || fail=1
bash tools/verify_drawtrace10.sh || fail=1
bash tools/verify_player_gui10.sh --no-gui || fail=1
bash tools/verify_player_gui7_10.sh --built --no-gui || fail=1
bash tools/verify_player_gui8_10.sh --built --no-gui || fail=1
bash tools/verify_player_gui10_10.sh --built --no-gui || fail=1
bash tools/verify_player_gui11_10.sh --built --no-gui || fail=1
bash tools/verify_minecraft10.sh --built || fail=1
exit "$fail"
