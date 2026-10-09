#!/usr/bin/env bash
# Round-seven checks against the original 1.0 jar; no shared fixtures are changed.
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
for mode in controllerlife181 inventorybounds181; do
  echo "== player-gui round 7: $mode"
  bash tools/oracle11/oracle.sh "$mode" "$mode-java.bin" && r10_compare "target/release/${mode/181/11}" "target/r11/$mode-rust.bin" "tools/oracle11/out/$mode-java.bin" || fail=1
done
echo "== player-gui round 7: original inventory-screen callbacks"
bash tools/oracle11/oracle.sh inventorygui181 inventorygui181-java.bin && r10_client target/release/beta --mc 1.1 --inventoryguicheck tools/oracle11/out/inventorygui181-java.bin || fail=1
echo "== player-gui round 7: original general-stat drawing arguments"
bash tools/oracle11/oracle.sh generalstats181 generalstats181-java.bin && r10_client target/release/beta --mc 1.1 --generalstatscheck tools/oracle11/out/generalstats181-java.bin || fail=1
if [ "${2:-}" != --no-gui ]; then
  scenes=tools/oracle11/gui/player-gui7-scenes.txt
  java=tools/oracle11/out/player-gui7-java
  rust=target/r11/player-gui7-rust
  python - "$scenes" "$java" "$rust" <<'EOF'
from pathlib import Path
import sys
for line in Path(sys.argv[1]).read_text().splitlines():
    line=line.split('#',1)[0].strip()
    if line.startswith('shot '):
        for directory in sys.argv[2:]:
            (Path(directory)/(line.split(maxsplit=1)[1]+'.png')).unlink(missing_ok=True)
EOF
  echo "== player-gui round 7: entity brightness and inventory redirects (zero tolerance)"
  bash tools/oracle11/guishot.sh "$scenes" "$java" || fail=1
  GUI_MSAA=1 r10_client target/release/beta --mc 1.1 --guishot "$scenes" "$rust" || fail=1
  python tools/oracle11/compare_gui_strict.py "$scenes" "$java" "$rust" target/r11/player-gui7-diff || fail=1
fi
[ $fail -eq 0 ] && echo "PLAYER-GUI ROUND 7 EXACT" || echo "PLAYER-GUI ROUND 7 DIFFERS"
exit $fail
