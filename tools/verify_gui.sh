#!/usr/bin/env bash
# Pixel-compare the GUI against the original client: every scene in tools/oracle/gui/scenes.txt
# is captured from the real b1.7.3 (a game window opens for a few seconds; it draws into a
# single-sample framebuffer so driver-forced anti-aliasing can't interfere) and from
# `beta --guishot`, then diffed. Differences are written to shots/gui_diff.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cargo build --release -q --manifest-path "$root/Cargo.toml" -p client || exit 1
java_out="$root/tools/oracle/out/gui_java"
rust_out="$root/shots/gui"
bash "$root/tools/oracle/guishot.sh" "$root/tools/oracle/gui/scenes.txt" "$java_out" 2>&1 | grep "^java: GL\|rror:" || true
GUI_MSAA=1 "$root/target/release/beta" --guishot "$root/tools/oracle/gui/scenes.txt" "$rust_out" >/dev/null || exit 1
"$root/target/release/beta" --guidiff "$java_out" "$rust_out" "$root/shots/gui_diff"
