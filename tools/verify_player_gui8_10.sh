#!/usr/bin/env bash
# Round-eight original-bytecode and zero-tolerance GUI checks.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
cd "$root"
lock="$root/target/r10/verify10.lock"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  mkdir -p target/r10
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen10 -p client || exit 1; fi
fail=0
echo "== player-gui round 8: environmental death and self-owned arrows"
bash tools/oracle10/oracle.sh playerdamage8_181 playerdamage8_181-java.bin && r10_compare target/release/playerdamage8_10 target/r10/playerdamage8_181-rust.bin tools/oracle10/out/playerdamage8_181-java.bin || fail=1
for replay in "movement 1000" "damage 1000" "pickup 1000" "survival 800" "bucket 100" "combat 400"; do
  read -r kind ticks <<<"$replay"
  echo "== player-gui round 8: retained player $kind ($ticks frames)"
  java="player-gui8-$kind.bin"
  bash tools/oracle10/oracle.sh playerintegration181 "$kind" "$ticks" "$java" && r10_compare target/release/compare10 player "$kind" "$ticks" "tools/oracle10/out/$java" || fail=1
done
for spec in "guislot8_181 guislot8check" "yesno8_181 yesno8check" "rename8_181 rename8check" "select8_181 select8check" "stats8_181 stats8check"; do
  read -r mode check <<<"$spec"
  echo "== player-gui round 8: $mode"
  bash tools/oracle10/oracle.sh "$mode" "$mode-java.bin" && r10_client target/release/beta --mc 1.0 "--$check" "tools/oracle10/out/$mode-java.bin" || fail=1
done
if [ "${2:-}" != --no-gui ]; then
  for scenes in tools/oracle10/gui/player-gui8-scenes.txt tools/oracle10/gui/player-gui8-stats-scenes.txt; do
    group="$(basename "$scenes" .txt)"
    java="tools/oracle10/out/$group-java"
    rust="target/r10/$group-rust"
    python tools/oracle10/gui/make_saves.py tools/oracle10/out/guisaves || { fail=1; continue; }
    python - "$scenes" "$java" "$rust" <<'EOF'
from pathlib import Path
import sys
for line in Path(sys.argv[1]).read_text().splitlines():
    line=line.split('#',1)[0].strip()
    if line.startswith('shot '):
        for directory in sys.argv[2:]:
            (Path(directory)/(line.split(maxsplit=1)[1]+'.png')).unlink(missing_ok=True)
EOF
    echo "== player-gui round 8: $group (zero tolerance)"
    bash tools/oracle10/guishot.sh "$scenes" "$java" || fail=1
    GUI_MSAA=1 r10_client target/release/beta --mc 1.0 --guishot "$scenes" "$rust" || fail=1
    python tools/oracle10/compare_gui_strict.py "$scenes" "$java" "$rust" "target/r10/$group-diff" || fail=1
  done
fi
[ $fail -eq 0 ] && echo "PLAYER-GUI ROUND 8 EXACT" || echo "PLAYER-GUI ROUND 8 DIFFERS"
exit $fail
