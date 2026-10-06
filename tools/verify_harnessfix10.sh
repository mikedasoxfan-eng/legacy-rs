#!/usr/bin/env bash
# Targeted reruns for fixture setup issues exposed by the full baseline.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
source "$root/tools/r10/verify_common10.sh"
lock="$root/target/r10/verify10.lock"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  mkdir "$lock" || exit 3
  trap 'rmdir "$lock" 2>/dev/null' EXIT
fi
fail=0
echo "== world-blocks 1.0: trapdoors6"
if r10_oracle "$root/tools/oracle10/oracle.sh" wbtrapdoors6 harness-fix10-trapdoors.bin; then
  r10_compare "$root/target/release/compare10" worldblocks181 trapdoors6 "$root/tools/oracle10/out/harness-fix10-trapdoors.bin" || fail=1
else fail=1; fi
echo "== first-person view, player and block targeting"
if r10_oracle "$root/tools/oracle10/oracle.sh" firstperson harness-fix10-firstperson.bin; then
  "$root/target/release/beta" --mc 1.0 --firstpersoncheck "$root/tools/oracle10/out/harness-fix10-firstperson.bin" || fail=1
else fail=1; fi
echo "== EffectRenderer destruction, hits, tint, eviction and vertex bits"
if r10_oracle "$root/tools/oracle10/oracle.sh" particleeffects harness-fix10-particles.bin; then
  "$root/target/release/beta" --mc 1.0 --particlefxcheck "$root/tools/oracle10/out/harness-fix10-particles.bin" || fail=1
else fail=1; fi
echo "== sky GL colors and sunrise vertex colors, anaglyph on/off"
if r10_oracle "$root/tools/oracle10/oracle.sh" skydraw harness-fix10-sky.bin; then
  "$root/target/release/beta" --mc 1.0 --skydrawcheck "$root/tools/oracle10/out/harness-fix10-sky.bin" || fail=1
else fail=1; fi
echo "== world-blocks 1.0: colors"
if r10_oracle "$root/tools/oracle10/oracle.sh" wbcolors harness-fix10-colors.bin; then
  r10_compare "$root/target/release/compare10" worldblocks181 colors "$root/tools/oracle10/out/harness-fix10-colors.bin" || fail=1
else fail=1; fi
echo "== decorations10: map files"
if r10_oracle "$root/tools/oracle10/oracle.sh" mapfiles harness-fix10-mapfiles.bin; then
  r10_compare "$root/target/release/compare10" mapfiles "$root/tools/oracle10/out/harness-fix10-mapfiles.bin" || fail=1
else fail=1; fi
echo "== decorations10: map files read back"
if r10_oracle "$root/tools/oracle10/oracle.sh" mapread "$root/target/r10/map-rust-save" harness-fix10-mapread.bin; then
  r10_compare "$root/target/release/compare10" mapread "$root/tools/oracle10/out/harness-fix10-mapfiles.bin" "$root/tools/oracle10/out/harness-fix10-mapread.bin" || fail=1
else fail=1; fi
exit "$fail"
