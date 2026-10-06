#!/usr/bin/env bash
# Name the first changed alpha pixel in the retained original GL captures.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
fail=0
for group in player-gui9-scenes player-gui7-scenes container-scenes feature-scenes; do
 python tools/oracle10/compare_gui_alpha.py "tools/oracle10/gui/$group.txt" "target/r10/player-gui9-$group-java" "target/r10/player-gui9-$group-rust" || fail=1
done
exit "$fail"
