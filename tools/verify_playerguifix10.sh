#!/usr/bin/env bash
# Recheck the headless player/GUI modes after preserving their dump basenames.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
bash "$root/tools/verify_player_gui10.sh" --no-gui
