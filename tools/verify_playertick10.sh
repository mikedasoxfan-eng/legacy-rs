#!/usr/bin/env bash
# Whole EntityPlayerSP ticks, original bytecode and complete per-tick state/events.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
cd "$root"
lock="$root/target/r10/verify10.lock"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  mkdir -p "$root/target/r10"
  mkdir "$lock" || { echo "another 1.0 verify is running" >&2; exit 3; }
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen10; fi
fail=0
for kind in idle movement sprint flight creativecontroller starve regen eat260 eat282 eat322 eat365 eat367 cancel identity bow xp damage creativedamage creativevoid speed haste fatigue nausea; do
  # The inherited Java flat fixture has seven chunks per axis. Longer forward
  # movement exits it and causes the original to request a nonexistent chunk.
  ticks=500
  case "$kind" in movement|sprint) ticks=240 ;; esac
  java="$root/tools/oracle10/out/playertick181-$kind.bin"
  rust="$root/target/r10/playertick181-$kind-rust.bin"
  [ "${VERIFY10_KEEP:-}" = 1 ] || rm -f "$java" "$rust"
  echo "== playertick10: $kind, $ticks ticks"
  if r10_oracle bash tools/oracle10/oracle.sh playertickintegration181 "$kind" "$ticks" "$java"; then
  r10_compare target/release/playertick10 "$kind" "$ticks" "$rust" "$java" || fail=1
  else fail=1; fi
done
exit "$fail"
