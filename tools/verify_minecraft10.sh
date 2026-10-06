#!/usr/bin/env bash
# Minecraft-owned checks; original 1.0 bytecode on Mojang Java 8u51.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
cd "$root"
lock="$root/target/r10/verify10.lock"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  mkdir -p target/r10
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen10 -p client || exit 1; fi
fail=0
oracle=tools/oracle10/oracle.sh
out=tools/oracle10/out
beta() { r10_client target/release/beta --mc 1.0 "$@"; }

echo "== minecraft 1.0: original runTick event queues, clicks, screen/focus callbacks and world policy"
r10_oracle bash "$oracle" runtick181 runtick181-java.bin && beta --runtickcheck "$out/runtick181-java.bin" || fail=1

echo "== minecraft 1.0: original preloadWorld coordinate, progress and lighting callbacks"
r10_oracle bash "$oracle" minecraftpreload181 minecraftpreload181-java.bin && beta --minecraftpreloadcheck "$out/minecraftpreload181-java.bin" || fail=1

echo "== minecraft 1.0: retained clickMouse and sendClickBlockToController effects"
r10_oracle bash "$oracle" controllerinput181 minecraft-controllerinput-java.bin && r10_compare target/release/controllerinput10 target/r10/minecraft-controllerinput-rust.bin "$out/minecraft-controllerinput-java.bin" || fail=1

echo "== minecraft 1.0: original debugInfoEntities counts and excluded particle layer"
r10_oracle bash "$oracle" minecraftdebug181 minecraftdebug181-java.bin && beta --minecraftdebugcheck "$out/minecraftdebug181-java.bin" || fail=1

echo "== minecraft 1.0: retained global key registry and press-count overflow"
r10_oracle bash "$oracle" keyregistry181 minecraft-keyregistry-java.txt && beta --keyregistrycheck "$out/minecraft-keyregistry-java.txt" || fail=1

echo "== minecraft 1.0: retained controller factory, spawn yaw and item-use release"
r10_oracle bash "$oracle" controllerlife181 minecraft-controllerlife-java.bin && r10_compare target/release/controllerlife10 target/r10/minecraft-controllerlife-rust.bin "$out/minecraft-controllerlife-java.bin" || fail=1

echo "== minecraft 1.0: retained create-world callbacks and WorldSettings"
r10_oracle bash "$oracle" worldcallbacks181 minecraft-worldcallbacks-java.bin && beta --worldcallbackcheck "$out/minecraft-worldcallbacks-java.bin" || fail=1

echo "== minecraft 1.0: retained world seed parsing and settings"
r10_oracle bash "$oracle" worldseed181 minecraft-worldseed-java.bin && beta --worldseedcheck "$out/minecraft-worldseed-java.bin" || fail=1

echo "== minecraft 1.0: retained saved-world controller selection"
r10_oracle bash "$oracle" select8_181 minecraft-select-java.bin && beta --select8check "$out/minecraft-select-java.bin" || fail=1

[ "$fail" -eq 0 ] && echo "MINECRAFT ROUND 1 CHECKS EXACT" || echo "MINECRAFT ROUND 1 CHECKS DIFFER"
exit "$fail"
