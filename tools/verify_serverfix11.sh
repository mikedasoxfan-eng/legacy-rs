#!/usr/bin/env bash
# Recheck the release server after initializing its new entity air watcher.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
source "$root/tools/r11/verify_common10.sh"
lock="$root/target/r11/verify11.lock"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
 mkdir "$lock" || exit 3
 trap 'rmdir "$lock" 2>/dev/null' EXIT
fi
fail=0
out="$root/tools/oracle11/out"
save="$root/target/r11/serverfix"
rm -rf "$save/java" "$save/rust"  # a leftover save would be loaded instead of generated
mkdir -p "$save"
echo "== server mode: 900-tick walk (real World.tick), unload/reload through region files"
if r10_oracle "$root/tools/oracle11/server_oracle.sh" serverwalk 0 1 10 "$save/java" 900 harness-fix10-server.bin; then
 r10_compare "$root/target/release/compare11" serverwalk 0 1 10 "$save/rust" 900 "$out/harness-fix10-server.bin" || fail=1
 if r10_oracle "$root/tools/oracle11/oracle.sh" readsave "$save/java" harness-fix10-server-saved.bin; then
  r10_compare "$root/target/release/compare11" savecheck "$save/rust" "$out/harness-fix10-server-saved.bin" || fail=1
 else fail=1; fi
 python "$root/tools/oracle11/compare_regions.py" "$save/java" "$save/rust" || fail=1
else fail=1; fi
exit "$fail"
