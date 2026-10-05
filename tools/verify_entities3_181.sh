#!/usr/bin/env bash
# Round 3 branch matrices against original Mojang 8u51 bytecode.
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
for kind in slimebranches skeletonbranches wolfbranches vehiclebranches fireballbranches spawnorder spawnbranches factorybranches; do
  echo "== entities3: $kind"
  if "$oracle" entitybranches181 "$kind" 0 "${kind}181.bin" && "$cmp" "$kind" 0 "$out/${kind}181.bin"; then :; else fail=1; fi
done
if "$oracle" modelregistry3_181 modelregistry3_181.bin && "$beta" --mc b1.8.1 --modelregistrycheck "$out/modelregistry3_181.bin"; then :; else fail=1; fi
if "$oracle" generaliving181 generaliving181.bin && "$beta" --mc b1.8.1 --newlivingcheck "$out/generaliving181.bin"; then :; else fail=1; fi
if "$oracle" signrenderers sign-renderers.bin && "$beta" --mc b1.8.1 --signrenderercheck "$out/sign-renderers.bin"; then :; else fail=1; fi
if "$oracle" fishrenderers fish-renderers.bin && "$beta" --mc b1.8.1 --fishrenderercheck "$out/fish-renderers.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES3 ALL FOCUSED CASES EXACT" || echo "ENTITIES3 SOME CASES DIFFER"
exit "$fail"
