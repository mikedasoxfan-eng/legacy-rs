#!/usr/bin/env bash
# Release 1.0 blocks, round 4: item use/right-click/eating, harvesting, block callbacks,
# colours, tile factory and ItemStack damage against the original 1.0 bytecode, plus the
# inherited block checks fixed this round (World/Chunk tile lookup, piston guard, door power).
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
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen11 || exit 1; fi
oracle="$root/tools/oracle11/oracle.sh"
cmp="$root/target/release/compare11"
out="$root/tools/oracle11/out"
fail=0
check() { # name, then the command; prints EXACT or DIFFERS per section
  local name="$1"; shift
  if "$@"; then echo "EXACT: blocks4: $name"; else echo "DIFFERS: blocks4: $name"; fail=1; fi
}
for kind in right eat use items harvest ticks colors tiles stack; do
  echo "== blocks4: $kind"
  check "$kind" eval 'r10_oracle "$oracle" "wbb4$kind" "b4-$kind.bin" && r10_compare "$cmp" worldblocks181 "b4$kind" "$out/b4-$kind.bin"'
done
for suite in piston5 piston6 push6 bed9 door9 doors6 cake9 furnace7 inventory7 machines7; do
  echo "== blocks4: $suite"
  check "$suite" eval 'r10_oracle "$oracle" "wb$suite" "b4-wb-$suite.bin" && r10_compare "$cmp" worldblocks181 "$suite" "$out/b4-wb-$suite.bin"'
done
echo "== blocks4: containertiles"
check containertiles eval 'r10_oracle "$oracle" containertiles 1000 b4-container-tiles.bin && r10_compare "$root/target/release/container_compare11" tiles 1000 "$out/b4-container-tiles.bin"'
echo "== blocks4: maps"
check maps eval 'r10_oracle "$oracle" maps b4-maps.bin && r10_compare "$cmp" maps "$out/b4-maps.bin"'
[ "$fail" -eq 0 ] && echo "BLOCKS4 ALL CASES EXACT" || echo "BLOCKS4 SOME CASES DIFFER"
exit "$fail"
