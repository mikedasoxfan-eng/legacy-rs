#!/usr/bin/env bash
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p target/r11 tools/oracle11/out
source tools/r11/verify_common10.sh
if [ -z "${VERIFY11_LOCKED:-}" ]; then
  if ! mkdir target/r11/verify11.lock 2>/dev/null; then echo "another 1.0 verify is running" >&2; exit 3; fi
  trap 'rmdir target/r11/verify11.lock 2>/dev/null' EXIT
  export VERIFY11_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -p worldgen11 || exit 1; fi
oracle="$root/tools/oracle11/oracle.sh"
fail=0
for kind in potion enchantments helper modifiers names animation potionitem brewing geometry particles dragon cauldron eye eyeright recipes fortune enchantcontainer sounds brewcontainer; do
  echo "== blocks3: $kind"
  if r10_oracle "$oracle" r10blocks3 "$kind" "blocks3-$kind.bin" && "$root/target/release/r10blocks3_11.exe" "$kind" "$root/tools/oracle11/out/blocks3-$kind.bin"; then :; else fail=1; fi
done
echo "== blocks3: release block and geometry definitions"
if r10_oracle "$oracle" blocktable blocks3-table.rs && r10_oracle "$oracle" collisiontable blocks3-geometry.rs && python tools/oracle11/check_worldblocks10_tables.py tools/oracle11/out/blocks3-table.rs tools/oracle11/out/blocks3-geometry.rs; then :; else fail=1; fi
for kind in itembase10 blockfull11 itemstack11 stackdata11 stacknbt11; do
  echo "== blocks3: $kind"
  mode="${kind}_10"
  [ "$kind" = blockfull11 ] && mode=itemblock6_10
  if r10_oracle "$oracle" "$mode" "$kind" 0 "blocks3-$kind.bin" && r10_compare "$root/target/release/entity_compare11.exe" "$kind" 0 "$root/tools/oracle11/out/blocks3-$kind.bin"; then :; else fail=1; fi
done
[ "$fail" -eq 0 ] && echo "BLOCKS3 ALL CASES EXACT" || echo "BLOCKS3 SOME CASES DIFFER"
exit "$fail"
