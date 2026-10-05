#!/usr/bin/env bash
# GuiIngame (b1.8.1) pixel scenes: the portal overlay and its nausea exception.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
python "$root/tools/oracle181/gui/make_saves.py" "$root/tools/oracle181/out/guisaves" >/dev/null || exit 1
fail=0
for f in ingame181 ingame181-nausea; do
  java="$root/tools/oracle181/out/$f-java"; rust="$root/shots181/$f"
  rm -rf "$java" "$rust"; mkdir -p "$java" "$rust"
  bash "$root/tools/oracle181/guishot.sh" "$root/tools/oracle181/gui/$f-scenes.txt" "$java" > "$root/target/$f-java.log" 2>&1 || fail=1
  GUI_MSAA=1 "$root/target/release/beta" --mc b1.8.1 --guishot "$root/tools/oracle181/gui/$f-scenes.txt" "$rust" >/dev/null || fail=1
  "$root/target/release/beta" --mc b1.8.1 --guidiff "$java" "$rust" "$root/shots181/${f}_diff" > "$root/target/$f-diff.log" || fail=1
  python "$root/tools/b181/gui_sections.py" "$root/target/$f-diff.log"
done
[ $fail -eq 0 ] && echo "INGAME GUI CASES EXACT" || echo "INGAME GUI CASES DIFFER"
exit $fail
