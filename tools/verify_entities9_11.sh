#!/usr/bin/env bash
# Round 9 complete-class checks against original Beta 1.8.1 bytecode.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r11" "$root/tools/oracle11/out"
source "$root/tools/r11/verify_common10.sh"
cd "$root"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
  mkdir -p target/r11
  if ! mkdir target/r11/verify11.lock 2>/dev/null; then echo "another 1.0 verify is running" >&2; exit 3; fi
  trap 'rmdir target/r11/verify11.lock 2>/dev/null' EXIT
  export VERIFY11_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -p worldgen11 -p client || exit 1; fi
oracle="$root/tools/oracle11/oracle.sh"
out="$root/tools/oracle11/out"
beta="$root/target/release/beta.exe"
fail=0
echo "== entities9: RenderManager registry, camera and dispatch"
if r10_oracle "$oracle" rendermanager9_181 rendermanager9.bin && r10_client "$beta" --mc 1.1 --rendermanager9check "$out/rendermanager9.bin"; then :; else fail=1; fi
echo "== entities9: actual shadow and thrown-item renderers"
if r10_oracle "$oracle" managervisual9_181 managervisual9.bin && r10_client "$beta" --mc 1.1 --managervisual9check "$out/managervisual9.bin"; then :; else fail=1; fi
echo "== entities9: TileEntitySpecialRenderer context and resources"
if r10_oracle "$oracle" tilespecial9_181 tilespecial9.bin && r10_client "$beta" --mc 1.1 --tilespecial9check "$out/tilespecial9.bin"; then :; else fail=1; fi
echo "== entities9: TileEntityRenderer class cache, active context and dispatch"
if r10_oracle "$oracle" tilemanager9_181 tilemanager9.bin && r10_client "$beta" --mc 1.1 --tilemanager9check "$out/tilemanager9.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES9 ALL FOCUSED CASES EXACT" || echo "ENTITIES9 SOME CASES DIFFER"
exit "$fail"
