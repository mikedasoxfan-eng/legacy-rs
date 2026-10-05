#!/usr/bin/env bash
# Minecraft-owned checks; original b1.8.1 bytecode on Mojang Java 8u51.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p target
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p worldgen181 -p client || exit 1; fi
fail=0
oracle=tools/oracle181/oracle.sh
out=tools/oracle181/out
beta() { target/release/beta --mc b1.8.1 "$@"; }

echo "== minecraft b1.8.1: original runTick event queues, clicks, screen/focus callbacks and world policy"
bash "$oracle" runtick181 runtick181-java.bin && beta --runtickcheck "$out/runtick181-java.bin" || fail=1

echo "== minecraft b1.8.1: original preloadWorld coordinate, progress and lighting callbacks"
bash "$oracle" minecraftpreload181 minecraftpreload181-java.bin && beta --minecraftpreloadcheck "$out/minecraftpreload181-java.bin" || fail=1

echo "== minecraft b1.8.1: retained clickMouse and sendClickBlockToController effects"
bash "$oracle" controllerinput181 minecraft-controllerinput-java.bin && target/release/controllerinput181 target/minecraft-controllerinput-rust.bin "$out/minecraft-controllerinput-java.bin" || fail=1

echo "== minecraft b1.8.1: original debugInfoEntities counts and excluded particle layer"
bash "$oracle" minecraftdebug181 minecraftdebug181-java.bin && beta --minecraftdebugcheck "$out/minecraftdebug181-java.bin" || fail=1

echo "== minecraft b1.8.1: retained global key registry and press-count overflow"
bash "$oracle" keyregistry181 minecraft-keyregistry-java.txt && beta --keyregistrycheck "$out/minecraft-keyregistry-java.txt" || fail=1

echo "== minecraft b1.8.1: retained controller factory, spawn yaw and item-use release"
bash "$oracle" controllerlife181 minecraft-controllerlife-java.bin && target/release/controllerlife181 target/minecraft-controllerlife-rust.bin "$out/minecraft-controllerlife-java.bin" || fail=1

echo "== minecraft b1.8.1: retained create-world callbacks and WorldSettings"
bash "$oracle" worldcallbacks181 minecraft-worldcallbacks-java.bin && beta --worldcallbackcheck "$out/minecraft-worldcallbacks-java.bin" || fail=1

echo "== minecraft b1.8.1: retained world seed parsing and settings"
bash "$oracle" worldseed181 minecraft-worldseed-java.bin && beta --worldseedcheck "$out/minecraft-worldseed-java.bin" || fail=1

echo "== minecraft b1.8.1: retained saved-world controller selection"
bash "$oracle" select8_181 minecraft-select-java.bin && beta --select8check "$out/minecraft-select-java.bin" || fail=1

[ "$fail" -eq 0 ] && echo "MINECRAFT ROUND 1 CHECKS EXACT" || echo "MINECRAFT ROUND 1 CHECKS DIFFER"
exit "$fail"
