#!/usr/bin/env bash
# End-to-end play tests of the real 1.0 client (tools/playtest11/*.txt): scripted play through new
# 1.0 features in the actual game, failing on crashes, hangs or a wrong position.
root="$(cd "$(dirname "$0")/.." && pwd)"
[ "${1:-}" = "--built" ] || cargo build --release -q --manifest-path "$root/Cargo.toml" -p client || exit 1
python "$root/tools/playtest11/run.py"
