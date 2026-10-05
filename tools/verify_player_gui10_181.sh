#!/usr/bin/env bash
# Round ten: original title callbacks, texture prefix policy and stat-file schema.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p target
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen181 -p client || exit 1; fi
fail=0
for spec in "mainmenu10_181 mainmenu10check" "renderengine10_181 renderengine10check" "statfile10_181 statfile10check"; do
  read -r mode check <<<"$spec"
  echo "== player-gui round 10: $mode"
  # The original prints its own parse errors for the malformed inputs; keep them out of the section.
  # Both sides print the original's own messages for malformed stats files (CHECKSUM MISMATCH,
  # invalid syntax); full output goes to target/, the section shows the check's summary.
  if bash tools/oracle181/oracle.sh "$mode" "$mode-java.bin" >"target/$mode-oracle.log" 2>&1; then
    target/release/beta --mc b1.8.1 "--$check" "tools/oracle181/out/$mode-java.bin" >"target/$mode-check.log" 2>&1; rc=$?
    grep -v -E 'is not a valid stat|invalid syntax|CHECKSUM MISMATCH' "target/$mode-check.log" | tail -n 3
    [ $rc -eq 0 ] || { echo "DIFFERS: $mode"; fail=1; }
  else
    tail -n 5 "target/$mode-oracle.log"; echo "ORACLE FAILED: $mode"; fail=1
  fi
done
if [ "${2:-}" != --no-gui ]; then
  for scenes in tools/oracle181/gui/player-gui10-scenes.txt tools/oracle181/gui/feature-scenes.txt; do
    group="$(basename "$scenes" .txt)"
    java="tools/oracle181/out/player-gui10-$group-java"
    rust="target/player-gui10-$group-rust"
    python tools/oracle181/gui/make_saves.py tools/oracle181/out/guisaves || { fail=1; continue; }
    python tools/oracle181/gui/make_pack.py tools/oracle181/out/packmc || { fail=1; continue; }
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
    GUI_CAPTURE_ALPHA=1 bash tools/oracle181/guishot.sh "$scenes" "$java" || fail=1
    GUI_CAPTURE_ALPHA=1 GUI_MSAA=1 target/release/beta --mc b1.8.1 --guishot "$scenes" "$rust" || fail=1
    python tools/oracle181/compare_gui_strict.py "$scenes" "$java" "$rust" "target/player-gui10-$group-diff" || fail=1
    python - "$scenes" "$java" "$rust" <<'EOF' || fail=1
from pathlib import Path
import sys
scenes,java,rust=map(Path,sys.argv[1:]);total=0
for line in scenes.read_text().splitlines():
    line=line.split('#',1)[0].strip()
    if not line.startswith('shot '): continue
    name=line.split(maxsplit=1)[1]+'.alpha'
    a=(java/name).read_bytes();b=(rust/name).read_bytes()
    assert a==b, f'ALPHA DIFFERS: {name}'
    total+=len(a)
print(f'FRAMEBUFFER ALPHA EXACT: {total} original bytes')
EOF
  done
fi
[ $fail -eq 0 ] && echo "PLAYER-GUI ROUND 10 EXACT" || echo "PLAYER-GUI ROUND 10 DIFFERS"
exit $fail
