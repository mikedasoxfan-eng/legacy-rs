#!/usr/bin/env bash
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
fail=0
for kind in recorditem5 paintingitem5 vehicleriders5 dyesheep5 potionexpiry5 watchers5 wolfremote5 health5 wolfattack5 wolffollow5 vehiclecollision5 vehicleinteract5 skeletondeath5; do
 echo "== entities5: $kind"
 if tools/oracle11/oracle.sh entityround5_181 "$kind" 0 "$kind.bin" && r10_compare target/release/entity_compare11.exe "$kind" 0 "tools/oracle11/out/$kind.bin"; then :; else fail=1; fi
done
for spec in "bipedequipment5_181 --newlivingcheck" "particlefactory5_181 --particlefactorycheck"; do
 read -r mode option <<< "$spec"
 echo "== entities5: $mode"
 if tools/oracle11/oracle.sh "$mode" "$mode.bin" && target/release/beta.exe --mc 1.1 "$option" "tools/oracle11/out/$mode.bin"; then :; else fail=1; fi
done
[ "$fail" -eq 0 ] && echo "ENTITIES5 ALL FOCUSED CASES EXACT" || echo "ENTITIES5 SOME CASES DIFFER"
exit "$fail"
