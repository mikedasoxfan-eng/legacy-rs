#!/usr/bin/env bash
# Focused player/gui checks; safe to invoke independently or from verify181.sh.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p "$root/target"
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
cd "$root"
cargo build --release -q -p worldgen181 -p client || exit 1
fail=0
echo "== player-gui b1.8.1: whole-player ticks"
bash tools/verify_playertick181.sh --built || fail=1
echo "== player-gui b1.8.1: PlayerList hash table"
bash tools/oracle181/oracle.sh playerlist181 playerlist181-java.bin && target/release/playerlist181 target/playerlist181-rust.bin tools/oracle181/out/playerlist181-java.bin || fail=1
for mode in playercore creative; do
  echo "== player-gui b1.8.1: $mode"
  bash tools/oracle181/oracle.sh "$mode" "$mode-java.bin" || { fail=1; continue; }
  "target/release/${mode}181" "target/$mode-rust.bin" || { fail=1; continue; }
  python - "$mode" "tools/oracle181/out/$mode-java.bin" "target/$mode-rust.bin" <<'EOF'
from pathlib import Path
import sys
mode,java,rust=sys.argv[1:]
a,b=Path(java).read_bytes(),Path(rust).read_bytes()
if a!=b:
    at=next((i for i,(x,y) in enumerate(zip(a,b)) if x!=y),min(len(a),len(b)))
    print(f"{mode}: DIFFERS at byte {at}, lengths {len(a)}/{len(b)}")
    sys.exit(1)
print(f"{mode}: EXACT ({len(a)} bytes)")
EOF
  [ $? -eq 0 ] || fail=1
done
echo "== player-gui b1.8.1: options and keybindings"
bash tools/oracle181/oracle.sh options181 options181-java.txt && target/release/beta --mc b1.8.1 --playerguicheck tools/oracle181/out/options181-java.txt || fail=1
if [ "${1:-}" != --no-gui ]; then
  for scenes in tools/oracle181/gui/player-gui-scenes.txt tools/oracle181/gui/player-gui-container-scenes.txt tools/oracle181/gui/player-gui2-scenes.txt; do
  echo "== player-gui b1.8.1: GUI pixels $(basename "$scenes" .txt)"
  java=tools/oracle181/out/player-gui-java
  rust=target/player-gui-rust
  python - "$scenes" "$java" "$rust" <<'EOF'
from pathlib import Path
import sys
for line in Path(sys.argv[1]).read_text().splitlines():
    line=line.split('#',1)[0].strip()
    if line.startswith('shot '):
        for directory in sys.argv[2:]:
            (Path(directory)/(line.split(maxsplit=1)[1]+'.png')).unlink(missing_ok=True)
EOF
  bash tools/oracle181/guishot.sh "$scenes" "$java" || fail=1
  GUI_MSAA=1 target/release/beta --mc b1.8.1 --guishot "$scenes" "$rust" || fail=1
  python tools/oracle181/compare_gui_strict.py "$scenes" "$java" "$rust" target/player-gui-diff || fail=1
  done
fi
exit $fail
