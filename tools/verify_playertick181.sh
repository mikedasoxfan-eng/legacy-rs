#!/usr/bin/env bash
# Whole EntityPlayerSP ticks, original bytecode and complete per-tick state/events.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p "$root/target"
  mkdir "$lock" || { echo "another b1.8.1 verify is running" >&2; exit 3; }
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen181; fi
for kind in idle movement sprint flight creativecontroller starve regen eat260 eat282 eat322 eat365 eat367 cancel identity bow xp damage creativedamage creativevoid speed haste fatigue nausea; do
  # The inherited Java flat fixture has seven chunks per axis. Longer forward
  # movement exits it and causes the original to request a nonexistent chunk.
  ticks=500
  case "$kind" in movement|sprint) ticks=240 ;; esac
  java="$root/tools/oracle181/out/playertick181-$kind.bin"
  rust="$root/target/playertick181-$kind-rust.bin"
  rm -f "$java" "$rust"
  bash tools/oracle181/oracle.sh playertickintegration181 "$kind" "$ticks" "$java"
  target/release/playertick181 "$kind" "$ticks" "$rust" "$java"
done
