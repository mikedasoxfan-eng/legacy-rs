#!/usr/bin/env bash
# Round 9 complete-class checks against original Beta 1.8.1 bytecode.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p target
  if ! mkdir target/verify181.lock 2>/dev/null; then echo "another b1.8.1 verify is running" >&2; exit 3; fi
  trap 'rmdir target/verify181.lock 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -p worldgen181 -p client || exit 1; fi
oracle="$root/tools/oracle181/oracle.sh"
out="$root/tools/oracle181/out"
beta="$root/target/release/beta.exe"
fail=0
echo "== entities9: RenderManager registry, camera and dispatch"
if "$oracle" rendermanager9_181 rendermanager9.bin && "$beta" --mc b1.8.1 --rendermanager9check "$out/rendermanager9.bin"; then :; else fail=1; fi
echo "== entities9: actual shadow and thrown-item renderers"
if "$oracle" managervisual9_181 managervisual9.bin && "$beta" --mc b1.8.1 --managervisual9check "$out/managervisual9.bin"; then :; else fail=1; fi
echo "== entities9: TileEntitySpecialRenderer context and resources"
if "$oracle" tilespecial9_181 tilespecial9.bin && "$beta" --mc b1.8.1 --tilespecial9check "$out/tilespecial9.bin"; then :; else fail=1; fi
echo "== entities9: TileEntityRenderer class cache, active context and dispatch"
if "$oracle" tilemanager9_181 tilemanager9.bin && "$beta" --mc b1.8.1 --tilemanager9check "$out/tilemanager9.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES9 ALL FOCUSED CASES EXACT" || echo "ENTITIES9 SOME CASES DIFFER"
exit "$fail"
