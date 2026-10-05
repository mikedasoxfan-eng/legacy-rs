#!/usr/bin/env bash
# Pixel-compare the GUI against the original client: every scene in tools/oracle181/gui/scenes.txt
# is captured from the real b1.8.1 (a game window opens for a few seconds; it draws into a
# single-sample framebuffer so driver-forced anti-aliasing can't interfere) and from
# `beta --guishot`, then diffed. Differences are written to shots/gui_diff.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
# One run at a time per tree (two runs would overwrite each other's oracle output).
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p "$root/target"
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock); remove it if stale" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
beta() { "$root/target/release/beta" --mc b1.8.1 "$@"; }
python "$root/tools/oracle181/gui/make_saves.py" "$root/tools/oracle181/out/guisaves" || exit 1
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
java_out="$root/tools/oracle181/out/gui_java"
rust_out="$root/shots/gui"
clear_captures "$root/tools/oracle181/gui/scenes.txt" "$java_out" "$rust_out"
bash "$root/tools/oracle181/guishot.sh" "$root/tools/oracle181/gui/scenes.txt" "$java_out" > "$root/target/gui-java.log" 2>&1 || fail=1
grep "^java: GL\|rror:" "$root/target/gui-java.log" || true
# The hud scenes' brightness= values tell the port what light level the original computes for
# the player (Java prints it); a stale value would make the vignette differ for the wrong reason.
python - "$root/tools/oracle181/gui/scenes.txt" "$root/target/gui-java.log" <<'EOF2' || fail=1
import re, sys
hints = [float(m.group(1)) for l in open(sys.argv[1], encoding='utf-8')
         if l.startswith('hud ') for m in [re.search(r'brightness=([0-9.]+)', l)] if m]
java = [float(l.split()[-1]) for l in open(sys.argv[2], encoding='utf-8', errors='replace') if 'player brightness' in l]
bad = [i for i, (h, j) in enumerate(zip(hints, java)) if abs(h - j) > 1e-6]
if len(hints) != len(java) or bad:
    print(f"HUD BRIGHTNESS HINTS DIFFER from the original's light levels: {len(hints)} hints, {len(java)} values, mismatches at {bad}")
    sys.exit(1)
print(f"HUD BRIGHTNESS HINTS EXACT ({len(hints)} scenes)")
EOF2
GUI_MSAA=1 "$root/target/release/beta" --mc b1.8.1 --guishot "$root/tools/oracle181/gui/scenes.txt" "$rust_out" >/dev/null || exit 1
"$root/target/release/beta" --mc b1.8.1 --guidiff "$java_out" "$rust_out" "$root/shots/gui_diff" || fail=1
scenes="$root/tools/oracle181/gui/container-scenes.txt"
java_containers="$root/tools/oracle181/out/containers_gui_java"
rust_containers="$root/shots/containers"
clear_captures "$scenes" "$java_containers" "$rust_containers"
bash "$root/tools/oracle181/guishot.sh" "$scenes" "$java_containers" > "$root/target/containers-gui-java.log" 2>&1 || fail=1
GUI_MSAA=1 "$root/target/release/beta" --mc b1.8.1 --guishot "$scenes" "$rust_containers" >/dev/null || fail=1
python "$root/tools/oracle181/compare_gui_strict.py" "$scenes" "$java_containers" "$rust_containers" "$root/shots/containers_diff" || fail=1
sign_scenes="$root/tools/oracle181/gui/sign-scenes.txt"
python - "$sign_scenes" "$root/tools/oracle181/out/sign_gui_java" "$root/shots/signs" <<'EOF'
from pathlib import Path
import sys
for line in Path(sys.argv[1]).read_text().splitlines():
    line=line.split('#',1)[0].strip()
    if line.startswith('shot '):
        name=line.split(maxsplit=1)[1]
        for directory in sys.argv[2:]:
            (Path(directory)/(name+'.png')).unlink(missing_ok=True)
EOF
bash "$root/tools/oracle181/guishot.sh" "$sign_scenes" "$root/tools/oracle181/out/sign_gui_java" > "$root/target/sign-gui-java.log" 2>&1 || fail=1
GUI_MSAA=1 "$root/target/release/beta" --mc b1.8.1 --guishot "$sign_scenes" "$root/shots/signs" >/dev/null || fail=1
python "$root/tools/oracle181/compare_gui_strict.py" "$sign_scenes" "$root/tools/oracle181/out/sign_gui_java" "$root/shots/signs" "$root/shots/signs_diff" || fail=1
python "$root/tools/oracle181/gui/make_pack.py" "$root/tools/oracle181/out/packmc" || exit 1
scenes="$root/tools/oracle181/gui/feature-scenes.txt"
java_features="$root/tools/oracle181/out/features_gui_java"
rust_features="$root/shots/features"
clear_captures "$scenes" "$java_features" "$rust_features"
bash "$root/tools/oracle181/guishot.sh" "$scenes" "$java_features" > "$root/target/features-gui-java.log" 2>&1 || fail=1
GUI_MSAA=1 "$root/target/release/beta" --mc b1.8.1 --guishot "$scenes" "$rust_features" > "$root/target/features-gui-rust.log" 2>&1 || fail=1
python "$root/tools/oracle181/compare_gui_strict.py" "$scenes" "$java_features" "$rust_features" "$root/shots/features_diff" || fail=1
"$root/target/release/beta" --mc b1.8.1 --texturecheck "$java_features/texture-fx.bin" || fail=1
[ $fail -eq 0 ] && echo "ALL GUI CASES MATCH" || echo "SOME GUI CASES DIFFER"
exit $fail
