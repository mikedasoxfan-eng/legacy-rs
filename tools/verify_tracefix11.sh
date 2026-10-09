#!/usr/bin/env bash
# Recheck lightmap capture after original release OpenGlHelper initialization.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
echo "== complete 1.8.1 pickuplight fixture (capture consistency)"
bash tools/oracle11/oracle.sh drawtrace181 pickuplight drawtrace_pickuplight.bin
python tools/b181/inspect_drawtrace181.py tools/oracle11/out/drawtrace_pickuplight.bin
