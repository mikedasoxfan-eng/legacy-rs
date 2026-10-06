#!/usr/bin/env bash
# Round 7 class-complete item checks, using original Beta 1.8.1 bytecode.
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
if [ "${1:-}" != --built ]; then cargo build --release -p worldgen10 -p client -j 2 || exit 1; fi
oracle="$root/tools/oracle10/oracle.sh"
out="$root/tools/oracle10/out"
cmp="$root/target/release/entity_compare10.exe"
fail=0
for kind in beditem7 signitem7; do
  echo "== entities7: $kind"
  if r10_oracle "$oracle" itemplacement7_181 "$kind" 0 "$kind.bin" && r10_compare "$cmp" "$kind" 0 "$out/$kind.bin"; then :; else fail=1; fi
done
echo "== entities7: metadata7"
if r10_oracle "$oracle" itemmetadata7_181 metadata7 0 metadata7.bin && r10_compare "$cmp" metadata7 0 "$out/metadata7.bin"; then :; else fail=1; fi
echo "== entities7: metaplace7"
if r10_oracle "$oracle" itemmetaplace7_181 metaplace7 0 metaplace7.bin && r10_compare "$cmp" blockitem6 0 "$out/metaplace7.bin"; then :; else fail=1; fi
for kind in buckethit7 bucketray7; do
  echo "== entities7: $kind"
  if r10_oracle "$oracle" itembucket7_181 "$kind" 0 "$kind.bin" && r10_compare "$cmp" "$kind" 0 "$out/$kind.bin"; then :; else fail=1; fi
done
echo "== entities7: logconstructor7"
if r10_oracle "$oracle" itemlog7_181 logconstructor7 0 logconstructor7.bin && r10_compare "$cmp" logconstructor7 0 "$out/logconstructor7.bin"; then :; else fail=1; fi
echo "== entities7: vineconstructor7"
if r10_oracle "$oracle" itemvine7_181 vineconstructor7 0 vineconstructor7.bin && r10_compare "$cmp" vineconstructor7 0 "$out/vineconstructor7.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES7 ALL FOCUSED CASES EXACT" || echo "ENTITIES7 SOME CASES DIFFER"
exit "$fail"
