#!/usr/bin/env bash
# Round 6 singleplayer entity/item proofs against original Mojang 8u51 bytecode.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p target
  if ! mkdir target/verify181.lock 2>/dev/null; then echo "another b1.8.1 verify is running" >&2; exit 3; fi
  trap 'rmdir target/verify181.lock 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -p worldgen181 -p client -j 2 || exit 1; fi
oracle="$root/tools/oracle181/oracle.sh"
out="$root/tools/oracle181/out"
cmp="$root/target/release/entity_compare181.exe"
beta="$root/target/release/beta.exe"
fail=0
for kind in living6 livingnbt6 livingdamage6 sleep6; do
  echo "== entities6: $kind"
  if "$oracle" entityround6_181 "$kind" 0 "$kind.bin" && "$cmp" "$kind" 0 "$out/$kind.bin"; then :; else fail=1; fi
done
for kind in dyeproperties6 dyeblocks6; do
  echo "== entities6: $kind"
  if "$oracle" itemdye6_181 "$kind" 0 "$kind.bin" && "$cmp" "$kind" 0 "$out/$kind.bin"; then :; else fail=1; fi
done
for kind in doorhelper6 dooritem6; do
  echo "== entities6: $kind"
  if "$oracle" itemdoor6_181 "$kind" 0 "$kind.bin" && "$cmp" "$kind" 0 "$out/$kind.bin"; then :; else fail=1; fi
done
echo "== entities6: blockbasic6 (limited block callbacks)"
if "$oracle" itemblock6_181 blockbasic6 0 blockbasic6.bin && "$cmp" blockitem6 0 "$out/blockbasic6.bin"; then :; else fail=1; fi
echo "== entities6: spawninputs6 (real JVM identity hashes)"
if "$oracle" entityspawninputs6_181 spawninputs6.bin && "$cmp" spawninputs6 0 "$out/spawninputs6.bin"; then :; else fail=1; fi
echo "== entities6: playerliving6_181"
if "$oracle" playerliving6_181 playerliving6_181.bin && "$beta" --mc b1.8.1 --playerlivingcheck "$out/playerliving6_181.bin"; then :; else fail=1; fi
echo "== entities6: particlefactory6_181"
if "$oracle" particlefactory6_181 particlefactory6_181.bin && "$beta" --mc b1.8.1 --particlefactorycheck "$out/particlefactory6_181.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES6 ALL FOCUSED CASES EXACT" || echo "ENTITIES6 SOME CASES DIFFER"
exit "$fail"
