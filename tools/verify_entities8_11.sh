#!/usr/bin/env bash
# Round 8 complete-class checks against original Beta 1.8.1 bytecode.
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
cmp="$root/target/release/entity_compare11.exe"
fail=0
echo "== entities8: EffectRenderer dispatch"
if r10_oracle "$oracle" effectdispatch8_181 effectdispatch8.bin && r10_client "$beta" --mc 1.1 --effectdispatch8check "$out/effectdispatch8.bin"; then :; else fail=1; fi
echo "== entities8: ItemMap lifecycle"
if r10_oracle "$oracle" itemmap8_181 itemmap8 0 itemmap8.bin && r10_compare "$cmp" itemmap8 0 "$out/itemmap8.bin"; then :; else fail=1; fi
echo "== entities8: Wolf despawning"
if r10_oracle "$oracle" wolfdespawn8_181 wolfdespawn8 0 wolfdespawn8.bin && r10_compare "$cmp" wolfdespawn8 0 "$out/wolfdespawn8.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES8 ALL FOCUSED CASES EXACT" || echo "ENTITIES8 SOME CASES DIFFER"
exit "$fail"
