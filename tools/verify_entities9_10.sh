#!/usr/bin/env bash
# Round 9 complete-class checks against original Beta 1.8.1 bytecode.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
cd "$root"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  mkdir -p target/r10
  if ! mkdir target/r10/verify10.lock 2>/dev/null; then echo "another 1.0 verify is running" >&2; exit 3; fi
  trap 'rmdir target/r10/verify10.lock 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -p worldgen10 -p client || exit 1; fi
oracle="$root/tools/oracle10/oracle.sh"
out="$root/tools/oracle10/out"
beta="$root/target/release/beta.exe"
fail=0
echo "== entities9: RenderManager registry, camera and dispatch"
if r10_oracle "$oracle" rendermanager9_181 rendermanager9.bin && r10_client "$beta" --mc 1.0 --rendermanager9check "$out/rendermanager9.bin"; then :; else fail=1; fi
echo "== entities9: actual shadow and thrown-item renderers"
if r10_oracle "$oracle" managervisual9_181 managervisual9.bin && r10_client "$beta" --mc 1.0 --managervisual9check "$out/managervisual9.bin"; then :; else fail=1; fi
echo "== entities9: TileEntitySpecialRenderer context and resources"
if r10_oracle "$oracle" tilespecial9_181 tilespecial9.bin && r10_client "$beta" --mc 1.0 --tilespecial9check "$out/tilespecial9.bin"; then :; else fail=1; fi
echo "== entities9: TileEntityRenderer class cache, active context and dispatch"
if r10_oracle "$oracle" tilemanager9_181 tilemanager9.bin && r10_client "$beta" --mc 1.0 --tilemanager9check "$out/tilemanager9.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES9 ALL FOCUSED CASES EXACT" || echo "ENTITIES9 SOME CASES DIFFER"
exit "$fail"
