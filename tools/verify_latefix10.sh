#!/usr/bin/env bash
# Final setup rechecks: window summaries, ray hook, input buffers and trace suffix.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
source "$root/tools/r10/verify_common10.sh"
fail=0
echo "== renderer window sizes and Java negative-coordinate wrapping"
windowfail=0
for distance in 0 1 2 3; do
 for position in 0 -1 8 -8 208 -209 30000000 -30000000; do
  j=$(r10_oracle tools/oracle10/oracle.sh videowindow "$distance" "$position")
  r=$(r10_client "$root/target/release/beta" --mc 1.0 --videowindow "$distance" "$position")
  if [ "$j" != "$r" ]; then echo "window differs: $distance $position"; echo "java $j"; echo "rust $r"; windowfail=1;fail=1; fi
 done
done
[ "$windowfail" = 0 ] && echo "WINDOWS EXACT: 32 comparisons"
for kind in buckethit7 bucketray7; do
 echo "== entities7: $kind"
 if r10_oracle tools/oracle10/oracle.sh itembucket7_181 "$kind" 0 "latefix10-$kind.bin"; then
  r10_compare "$root/target/release/entity_compare10" "$kind" 0 "$root/tools/oracle10/out/latefix10-$kind.bin" || fail=1
 else fail=1; fi
done
for mode in guislot8_181 stats8_181; do
 echo "== player-gui round 8: $mode"
 if r10_oracle tools/oracle10/oracle.sh "$mode" "latefix10-$mode.bin"; then
  r10_client "$root/target/release/beta" --mc 1.0 "--${mode/_181/}check" "$root/tools/oracle10/out/latefix10-$mode.bin" || fail=1
 else fail=1; fi
done
bash tools/verify_drawtrace10.sh || fail=1
exit "$fail"
