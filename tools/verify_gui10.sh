#!/usr/bin/env bash
# Pixel-compare the GUI against the original client: every scene in tools/oracle10/gui/scenes.txt
# is captured from the real 1.0 (a game window opens for a few seconds; it draws into a
# single-sample framebuffer so driver-forced anti-aliasing can't interfere) and from
# `beta --guishot`, then diffed. Differences are written to shots10/gui_diff.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
# One run at a time per tree (two runs would overwrite each other's oracle output).
lock="$root/target/r10/verify10.lock"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  mkdir -p "$root/target/r10"
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock); remove it if stale" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
beta() { r10_client "$root/target/release/beta" --mc 1.0 "$@"; }
python "$root/tools/oracle10/gui/make_saves.py" "$root/tools/oracle10/out/guisaves" || exit 1
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
java_out="$root/tools/oracle10/out/gui_java"
rust_out="$root/shots10/gui"
clear_captures "$root/tools/oracle10/gui/scenes.txt" "$java_out" "$rust_out"
echo "== gui10: oracle capture and HUD brightness hints"
bash "$root/tools/oracle10/guishot.sh" "$root/tools/oracle10/gui/scenes.txt" "$java_out" > "$root/target/r10/gui-java.log" 2>&1 || fail=1
grep "^java: GL\|rror:" "$root/target/r10/gui-java.log" || true
# The hud scenes' brightness= values tell the port what light level the original computes for
# the player (Java prints it); a stale value would make the vignette differ for the wrong reason.
python - "$root/tools/oracle10/gui/scenes.txt" "$root/target/r10/gui-java.log" <<'EOF2' || fail=1
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
# The original's run renames and deletes fixture worlds; the port starts from the same fixtures.
python "$root/tools/oracle10/gui/make_saves.py" "$root/tools/oracle10/out/guisaves" >/dev/null || exit 1
GUI_MSAA=1 r10_client "$root/target/release/beta" --mc 1.0 --guishot "$root/tools/oracle10/gui/scenes.txt" "$rust_out" >/dev/null || exit 1
# Same rule as Beta 1.7.3 and 1.8.1: a screen whose only differences are a few pixels one colour level off
# (GPU lighting rounding on lit 3D block icons) is reported apart as "within one level"; anything else fails.
echo "== gui10: main screens"
"$root/target/release/beta" --mc 1.0 --guidiff "$java_out" "$rust_out" "$root/shots10/gui_diff" || fail=1
scenes="$root/tools/oracle10/gui/container-scenes.txt"
java_containers="$root/tools/oracle10/out/containers_gui_java"
rust_containers="$root/shots10/containers"
clear_captures "$scenes" "$java_containers" "$rust_containers"
bash "$root/tools/oracle10/guishot.sh" "$scenes" "$java_containers" > "$root/target/r10/containers-gui-java.log" 2>&1 || fail=1
GUI_MSAA=1 r10_client "$root/target/release/beta" --mc 1.0 --guishot "$scenes" "$rust_containers" >/dev/null || fail=1
python "$root/tools/oracle10/compare_gui_strict.py" "$scenes" "$java_containers" "$rust_containers" "$root/shots10/containers_diff" || fail=1
sign_scenes="$root/tools/oracle10/gui/sign-scenes.txt"
python - "$sign_scenes" "$root/tools/oracle10/out/sign_gui_java" "$root/shots10/signs" <<'EOF'
from pathlib import Path
import sys
for line in Path(sys.argv[1]).read_text().splitlines():
    line=line.split('#',1)[0].strip()
    if line.startswith('shot '):
        name=line.split(maxsplit=1)[1]
        for directory in sys.argv[2:]:
            (Path(directory)/(name+'.png')).unlink(missing_ok=True)
EOF
bash "$root/tools/oracle10/guishot.sh" "$sign_scenes" "$root/tools/oracle10/out/sign_gui_java" > "$root/target/r10/sign-gui-java.log" 2>&1 || fail=1
GUI_MSAA=1 r10_client "$root/target/release/beta" --mc 1.0 --guishot "$sign_scenes" "$root/shots10/signs" >/dev/null || fail=1
python "$root/tools/oracle10/compare_gui_strict.py" "$sign_scenes" "$root/tools/oracle10/out/sign_gui_java" "$root/shots10/signs" "$root/shots10/signs_diff" || fail=1
python "$root/tools/oracle10/gui/make_pack.py" "$root/tools/oracle10/out/packmc" || exit 1
scenes="$root/tools/oracle10/gui/feature-scenes.txt"
java_features="$root/tools/oracle10/out/features_gui_java"
rust_features="$root/shots10/features"
clear_captures "$scenes" "$java_features" "$rust_features"
bash "$root/tools/oracle10/guishot.sh" "$scenes" "$java_features" > "$root/target/r10/features-gui-java.log" 2>&1 || fail=1
GUI_MSAA=1 r10_client "$root/target/release/beta" --mc 1.0 --guishot "$scenes" "$rust_features" > "$root/target/r10/features-gui-rust.log" 2>&1 || fail=1
python "$root/tools/oracle10/compare_gui_strict.py" "$scenes" "$java_features" "$rust_features" "$root/shots10/features_diff" || fail=1
r10_client echo "== gui10: texture animation bytes"
"$root/target/release/beta" --mc 1.0 --texturecheck "$java_features/texture-fx.bin" || fail=1
[ $fail -eq 0 ] && echo "ALL GUI CASES MATCH" || echo "SOME GUI CASES DIFFER"
exit $fail
