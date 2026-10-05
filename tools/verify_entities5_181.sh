#!/usr/bin/env bash
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
fail=0
for kind in recorditem5 paintingitem5 vehicleriders5 dyesheep5 potionexpiry5 watchers5 wolfremote5 health5 wolfattack5 wolffollow5 vehiclecollision5 vehicleinteract5 skeletondeath5; do
 echo "== entities5: $kind"
 if tools/oracle181/oracle.sh entityround5_181 "$kind" 0 "$kind.bin" && target/release/entity_compare181.exe "$kind" 0 "tools/oracle181/out/$kind.bin"; then :; else fail=1; fi
done
for spec in "bipedequipment5_181 --newlivingcheck" "particlefactory5_181 --particlefactorycheck"; do
 read -r mode option <<< "$spec"
 echo "== entities5: $mode"
 if tools/oracle181/oracle.sh "$mode" "$mode.bin" && target/release/beta.exe --mc b1.8.1 "$option" "tools/oracle181/out/$mode.bin"; then :; else fail=1; fi
done
[ "$fail" -eq 0 ] && echo "ENTITIES5 ALL FOCUSED CASES EXACT" || echo "ENTITIES5 SOME CASES DIFFER"
exit "$fail"
