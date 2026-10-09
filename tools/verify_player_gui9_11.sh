#!/usr/bin/env bash
# Round-nine original-bytecode/capture checks. No RGB tolerance is used.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r11" "$root/tools/oracle11/out"
source "$root/tools/r11/verify_common10.sh"
cd "$root"
lock="$root/target/r11/verify11.lock"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
  mkdir -p target/r11
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY11_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen11 -p client || exit 1; fi
fail=0
echo "== player-gui round 9: original inventory callbacks"
bash tools/oracle11/oracle.sh inventorygui181 inventorygui181-java.bin && r10_client target/release/beta --mc 1.1 --inventoryguicheck tools/oracle11/out/inventorygui181-java.bin || fail=1
python tools/oracle11/gui/make_saves.py tools/oracle11/out/guisaves || exit 1
python tools/oracle11/gui/make_pack.py tools/oracle11/out/packmc || exit 1
for group in player-gui9-scenes player-gui7-scenes container-scenes feature-scenes; do
  scenes="tools/oracle11/gui/$group.txt"
  java="target/r11/player-gui9-$group-java"
  rust="target/r11/player-gui9-$group-rust"
  python - "$scenes" "$java" "$rust" <<'EOF'
from pathlib import Path
import sys
for line in Path(sys.argv[1]).read_text().splitlines():
    line=line.split('#',1)[0].strip()
    if line.startswith('shot '):
        for directory in sys.argv[2:]:
            for suffix in ['.png','.alpha']:
                (Path(directory)/(line.split(maxsplit=1)[1]+suffix)).unlink(missing_ok=True)
EOF
  GUI_CAPTURE_ALPHA=1 bash tools/oracle11/guishot.sh "$scenes" "$java" || { fail=1; continue; }
  GUI_CAPTURE_ALPHA=1 GUI_MSAA=1 r10_client target/release/beta --mc 1.1 --guishot "$scenes" "$rust" || { fail=1; continue; }
  if [ "$group" != feature-scenes ]; then
    python tools/oracle11/compare_gui_strict.py "$scenes" "$java" "$rust" "target/r11/player-gui9-$group-diff" || fail=1
  fi
  # Feature RGB includes the unresolved panorama driver gap: it remains in
  # verify_gui11.sh and is NOT asserted/credited EXACT by this alpha check.
  python tools/oracle11/compare_gui_alpha.py "$scenes" "$java" "$rust" || fail=1
  if [ "$group" = feature-scenes ]; then
    r10_client target/release/beta --mc 1.1 --guiframebuffercheck "$java/texture-fx.bin" || fail=1
    r10_client target/release/beta --mc 1.1 --texturecheck "$java/texture-fx.bin" || fail=1
  fi
done
[ $fail -eq 0 ] && echo "PLAYER-GUI ROUND 9 EXACT" || echo "PLAYER-GUI ROUND 9 DIFFERS"
exit $fail
