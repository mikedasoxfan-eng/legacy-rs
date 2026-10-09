#!/usr/bin/env bash
# Run from the repository root. ORACLE10_BASE may point to the already-built base harness.
# Its original jar and headless plumbing are read-only dependencies while the base is unmerged.
mkdir -p target/r11gui2
status=0
for check in font enchantment brewing registry inventory gradient backgrounds settings statformats hardcore slot disconnected win menustate menudraw video itemnames boss gameover worldslot; do
    echo "== r10gui2 $check original bytecode"
    python tools/r11/gui2_oracle.py "gui2${check}10" "target/r11gui2/$check.bin" || { status=1; continue; }
    target/release/beta --mc 1.1 --r10guicheck "$check" "target/r11gui2/$check.bin" || status=1
done
exit "$status"
