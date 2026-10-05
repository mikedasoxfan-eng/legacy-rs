#!/usr/bin/env bash
# Focused b1.8.1 entity checks; replays original Mojang bytecode.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p target
  if ! mkdir target/verify181.lock 2>/dev/null; then echo "another b1.8.1 verify is running" >&2; exit 3; fi
  trap 'rmdir target/verify181.lock 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
cargo build --release -p worldgen181 -p client -j 2 || exit 1
oracle="$root/tools/oracle181/oracle.sh"
out="$root/tools/oracle181/out"
cmp="$root/target/release/entity_compare181.exe"
beta="$root/target/release/beta.exe"
fail=0
for spec in "rules 0" "Enderman 100" "teleport 40" "CaveSpider 100" "Silverfish 100" "XPOrb 100" "Arrow 100" "Pig 100" "Cow 100" "Chicken 100" "Sheep 100" "Wolf 100" "Zombie 100" "Skeleton 100" "Spider 100" "Creeper 100" "PigZombie 100" "Ghast 100" "Squid 100" "Slime 100" "potions 100" "damage 100" "poison 40" "kill-Cow 40" "kill-Chicken 40" "kill-Sheep 40" "kill-Zombie 40" "kill-Enderman 40"; do
  read -r kind ticks <<< "$spec"
  if "$oracle" entities181 "$kind" "$ticks" "entity181_${kind}.bin" && "$cmp" "$kind" "$ticks" "$out/entity181_${kind}.bin"; then :; else fail=1; fi
done
if "$oracle" entitygeometry entitygeometry181.bin && "$root/target/release/compare181.exe" entitygeometry "$out/entitygeometry181.bin"; then :; else fail=1; fi
if "$oracle" models181 models181.bin && "$beta" --mc b1.8.1 --modelcheck "$out/models181.bin"; then :; else fail=1; fi
exit "$fail"
