#!/usr/bin/env bash
# Round 11 complete ItemBlock/ItemStack checks against original Beta 1.8.1 bytecode.
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
cmp="$root/target/release/entity_compare10.exe"
fail=0
echo "== entities11: blockfull11 (all registered block items)"
if r10_oracle "$oracle" itemblock6_181 blockfull11 0 blockfull11.bin && r10_compare "$cmp" blockfull11 0 "$out/blockfull11.bin"; then :; else fail=1; fi
for kind in itempiston11 itemstack11 stackdata11 stacknbt11; do
  echo "== entities11: $kind"
  if r10_oracle "$oracle" "${kind}_181" "$kind" 0 "$kind.bin" && r10_compare "$cmp" "$kind" 0 "$out/$kind.bin"; then :; else fail=1; fi
done
[ "$fail" -eq 0 ] && echo "ENTITIES11 ALL FOCUSED CASES EXACT" || echo "ENTITIES11 SOME CASES DIFFER"
exit "$fail"
