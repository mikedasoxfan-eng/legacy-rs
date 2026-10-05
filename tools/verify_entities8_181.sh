#!/usr/bin/env bash
# Round 8 complete-class checks against original Beta 1.8.1 bytecode.
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
cmp="$root/target/release/entity_compare181.exe"
fail=0
echo "== entities8: EffectRenderer dispatch"
if "$oracle" effectdispatch8_181 effectdispatch8.bin && "$beta" --mc b1.8.1 --effectdispatch8check "$out/effectdispatch8.bin"; then :; else fail=1; fi
echo "== entities8: ItemMap lifecycle"
if "$oracle" itemmap8_181 itemmap8 0 itemmap8.bin && "$cmp" itemmap8 0 "$out/itemmap8.bin"; then :; else fail=1; fi
echo "== entities8: Wolf despawning"
if "$oracle" wolfdespawn8_181 wolfdespawn8 0 wolfdespawn8.bin && "$cmp" wolfdespawn8 0 "$out/wolfdespawn8.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES8 ALL FOCUSED CASES EXACT" || echo "ENTITIES8 SOME CASES DIFFER"
exit "$fail"
