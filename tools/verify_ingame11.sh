#!/usr/bin/env bash
# GuiIngame (1.0) pixel scenes: the portal overlay and its nausea exception.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r11" "$root/tools/oracle11/out"
source "$root/tools/r11/verify_common10.sh"
python "$root/tools/oracle11/gui/make_saves.py" "$root/tools/oracle11/out/guisaves" >/dev/null || exit 1
fail=0
for f in ingame10 ingame10-nausea; do
  java="$root/tools/oracle11/out/$f-java"; rust="$root/shots11/$f"
  rm -rf "$java" "$rust"; mkdir -p "$java" "$rust"
  bash "$root/tools/oracle11/guishot.sh" "$root/tools/oracle11/gui/$f-scenes.txt" "$java" > "$root/target/r11/$f-java.log" 2>&1 || fail=1
  GUI_MSAA=1 r10_client "$root/target/release/beta" --mc 1.1 --guishot "$root/tools/oracle11/gui/$f-scenes.txt" "$rust" >/dev/null || fail=1
  python "$root/tools/oracle11/compare_gui_strict.py" "$root/tools/oracle11/gui/$f-scenes.txt" "$java" "$rust" "$root/shots11/${f}_diff" || fail=1
done
[ $fail -eq 0 ] && echo "INGAME GUI CASES EXACT" || echo "INGAME GUI CASES DIFFER"
exit $fail
