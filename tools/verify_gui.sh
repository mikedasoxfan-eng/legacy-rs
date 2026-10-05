#!/usr/bin/env bash
# Pixel-compare the GUI against the original client: every scene in tools/oracle/gui/scenes.txt
# is captured from the real b1.7.3 (a game window opens for a few seconds; it draws into a
# single-sample framebuffer so driver-forced anti-aliasing can't interfere) and from
# `beta --guishot`, then diffed. Differences are written to shots/gui_diff.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
python "$root/tools/oracle/gui/make_saves.py" "$root/tools/oracle/out/guisaves" || exit 1
cargo build --release -q --manifest-path "$root/Cargo.toml" -p client || exit 1
fail=0
clear_captures() {
# Remove precisely the requested captures so an oracle failure cannot reuse old images.
python - "$@" <<'EOF'
from pathlib import Path
import sys
for line in Path(sys.argv[1]).read_text(encoding='utf-8').splitlines():
    line=line.split('#',1)[0].strip()
    if line.startswith('shot '):
        name=line.split(maxsplit=1)[1]
        for directory in sys.argv[2:]:
            (Path(directory)/(name+'.png')).unlink(missing_ok=True)
EOF
}
java_out="$root/tools/oracle/out/gui_java"
rust_out="$root/shots/gui"
clear_captures "$root/tools/oracle/gui/scenes.txt" "$java_out" "$rust_out"
bash "$root/tools/oracle/guishot.sh" "$root/tools/oracle/gui/scenes.txt" "$java_out" > "$root/target/gui-java.log" 2>&1 || fail=1
grep "^java: GL\|rror:" "$root/target/gui-java.log" || true
GUI_MSAA=1 "$root/target/release/beta" --guishot "$root/tools/oracle/gui/scenes.txt" "$rust_out" >/dev/null || exit 1
"$root/target/release/beta" --guidiff "$java_out" "$rust_out" "$root/shots/gui_diff" || fail=1
scenes="$root/tools/oracle/gui/container-scenes.txt"
java_containers="$root/tools/oracle/out/containers_gui_java"
rust_containers="$root/shots/containers"
clear_captures "$scenes" "$java_containers" "$rust_containers"
bash "$root/tools/oracle/guishot.sh" "$scenes" "$java_containers" > "$root/target/containers-gui-java.log" 2>&1 || fail=1
GUI_MSAA=1 "$root/target/release/beta" --guishot "$scenes" "$rust_containers" >/dev/null || fail=1
python "$root/tools/oracle/compare_gui_strict.py" "$scenes" "$java_containers" "$rust_containers" "$root/shots/containers_diff" || fail=1
python "$root/tools/oracle/gui/make_pack.py" "$root/tools/oracle/out/packmc" || exit 1
scenes="$root/tools/oracle/gui/feature-scenes.txt"
java_features="$root/tools/oracle/out/features_gui_java"
rust_features="$root/shots/features"
clear_captures "$scenes" "$java_features" "$rust_features"
bash "$root/tools/oracle/guishot.sh" "$scenes" "$java_features" > "$root/target/features-gui-java.log" 2>&1 || fail=1
GUI_MSAA=1 "$root/target/release/beta" --guishot "$scenes" "$rust_features" > "$root/target/features-gui-rust.log" 2>&1 || fail=1
python "$root/tools/oracle/compare_gui_strict.py" "$scenes" "$java_features" "$rust_features" "$root/shots/features_diff" || fail=1
"$root/target/release/beta" --texturecheck "$java_features/texture-fx.bin" || fail=1
[ $fail -eq 0 ] && echo "ALL GUI CASES MATCH" || echo "SOME GUI CASES DIFFER"
exit $fail
