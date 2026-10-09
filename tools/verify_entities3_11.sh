#!/usr/bin/env bash
# Round 3 branch matrices against original Mojang 8u51 bytecode.
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
if [ "${1:-}" != --built ]; then cargo build --release -p worldgen11 -p client -j 2 || exit 1; fi
oracle="$root/tools/oracle11/oracle.sh"
out="$root/tools/oracle11/out"
cmp="$root/target/release/entity_compare11.exe"
beta="$root/target/release/beta.exe"
fail=0
for kind in slimebranches skeletonbranches wolfbranches vehiclebranches fireballbranches spawnorder spawnbranches factorybranches; do
  echo "== entities3: $kind"
  if r10_oracle "$oracle" entitybranches181 "$kind" 0 "${kind}181.bin" && r10_compare "$cmp" "$kind" 0 "$out/${kind}181.bin"; then :; else fail=1; fi
done
echo "== entities3: model registry"
if r10_oracle "$oracle" modelregistry3_181 modelregistry3_181.bin && r10_client "$beta" --mc 1.1 --modelregistrycheck "$out/modelregistry3_181.bin"; then :; else fail=1; fi
echo "== entities3: new living renderers"
if r10_oracle "$oracle" generaliving181 generaliving181.bin && r10_client "$beta" --mc 1.1 --newlivingcheck "$out/generaliving181.bin"; then :; else fail=1; fi
echo "== entities3: sign renderers"
if r10_oracle "$oracle" signrenderers sign-renderers.bin && r10_client "$beta" --mc 1.1 --signrenderercheck "$out/sign-renderers.bin"; then :; else fail=1; fi
echo "== entities3: fish renderers"
if r10_oracle "$oracle" fishrenderers fish-renderers.bin && r10_client "$beta" --mc 1.1 --fishrenderercheck "$out/fish-renderers.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES3 ALL FOCUSED CASES EXACT" || echo "ENTITIES3 SOME CASES DIFFER"
exit "$fail"
