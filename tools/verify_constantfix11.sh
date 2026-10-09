#!/usr/bin/env bash
# Recheck all fixture groups whose constants changed in the initial copy.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
lock="$root/target/r11/verify11.lock"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
 mkdir "$lock" || exit 3
 trap 'rmdir "$lock" 2>/dev/null' EXIT
 export VERIFY11_LOCKED=1
fi
fail=0
bash tools/verify_decorations11.sh || fail=1
bash tools/verify_entities4_11.sh --built || fail=1
bash tools/verify_entities5_11.sh --built || fail=1
bash tools/verify_entities6_11.sh --built || fail=1
bash tools/verify_drawtrace11.sh || fail=1
bash tools/verify_player_gui11.sh --no-gui || fail=1
bash tools/verify_player_gui7_11.sh --built --no-gui || fail=1
bash tools/verify_player_gui8_11.sh --built --no-gui || fail=1
bash tools/verify_player_gui10_11.sh --built --no-gui || fail=1
bash tools/verify_player_gui11_11.sh --built --no-gui || fail=1
bash tools/verify_minecraft11.sh --built || fail=1
exit "$fail"
