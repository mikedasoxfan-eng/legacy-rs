#!/usr/bin/env bash
# Round 10 complete-class checks against original Beta 1.8.1 bytecode.
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
cmp="$root/target/release/entity_compare11.exe"
fail=0
for kind in itembase10 itemblock10 entitylist10; do
  echo "== entities10: $kind"
  if r10_oracle "$oracle" "${kind}_181" "$kind" 0 "$kind.bin" && r10_compare "$cmp" "$kind" 0 "$out/$kind.bin"; then :; else fail=1; fi
done
[ "$fail" -eq 0 ] && echo "ENTITIES10 ALL FOCUSED CASES EXACT" || echo "ENTITIES10 SOME CASES DIFFER"
exit "$fail"
