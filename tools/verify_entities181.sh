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
for spec in "rules 0" "indirect 20" "stare 10" "starepumpkin 10" "population 0" "carry 500" "allies 40" "attract 100" "Boat 100" "Minecart 100" "Item 100" "Lightning 20" "Snowball 30" "Egg 30" "Fireball 20" "Enderman 100" "teleport 40" "CaveSpider 100" "Silverfish 100" "XPOrb 100" "Arrow 100" "Pig 100" "Cow 100" "Chicken 100" "Sheep 100" "Wolf 100" "Zombie 100" "Skeleton 100" "Spider 100" "Creeper 100" "PigZombie 100" "Ghast 100" "Squid 100" "Slime 100" "potions 100" "damage 100" "poison 40" "kill-Cow 40" "kill-Chicken 40" "kill-Sheep 40" "kill-Zombie 40" "kill-Enderman 40"; do
  read -r kind ticks <<< "$spec"
  if "$oracle" entities181 "$kind" "$ticks" "entity181_${kind}.bin" && "$cmp" "$kind" "$ticks" "$out/entity181_${kind}.bin"; then :; else fail=1; fi
done
if "$oracle" entitygeometry entitygeometry181.bin && "$root/target/release/compare181.exe" entitygeometry "$out/entitygeometry181.bin"; then :; else fail=1; fi
if "$oracle" models181 models181.bin && "$beta" --mc b1.8.1 --modelcheck "$out/models181.bin"; then :; else fail=1; fi
if "$oracle" particles181 particles181.bin && "$beta" --mc b1.8.1 --particlecheck "$out/particles181.bin"; then :; else fail=1; fi
if "$oracle" xprender181 xprender181.bin && "$beta" --mc b1.8.1 --renderercheck "$out/xprender181.bin"; then :; else fail=1; fi
if "$oracle" vehiclerenders181 vehiclerenders181.bin && "$beta" --mc b1.8.1 --vehiclerenderercheck "$out/vehiclerenders181.bin"; then :; else fail=1; fi
if "$oracle" bedmodel181 bedmodel181.bin && "$beta" --mc b1.8.1 --bedmodelcheck "$out/bedmodel181.bin"; then :; else fail=1; fi
if "$oracle" arrows181 arrows181.bin && "$beta" --mc b1.8.1 --arrowcheck "$out/arrows181.bin"; then :; else fail=1; fi
# Round 2: packed light, wire APIs, combat, explosions and renderer callbacks.
for name in Pig Wolf Skeleton Spider CaveSpider Creeper PigZombie Ghast Squid Slime Silverfish; do
  if "$oracle" entities181 "kill-$name" 40 "entity181_kill-${name}.bin" && "$cmp" "kill-$name" 40 "$out/entity181_kill-${name}.bin"; then :; else fail=1; fi
done
for kind in watchers coordinates light registry; do
  if "$oracle" entityapi181 "$kind" 0 "entityapi181_${kind}.bin" && "$cmp" "$kind" 0 "$out/entityapi181_${kind}.bin"; then :; else fail=1; fi
done
if "$oracle" combat181 combat 0 combat181.bin && "$cmp" combat 0 "$out/combat181.bin"; then :; else fail=1; fi
if "$oracle" explosion181 explosion 0 explosion181.bin && "$cmp" explosion 0 "$out/explosion181.bin"; then :; else fail=1; fi
if "$oracle" particles2_181 particles2_181.bin && "$beta" --mc b1.8.1 --particlecheck "$out/particles2_181.bin"; then :; else fail=1; fi
if "$oracle" emitters181 emitters181.bin && "$beta" --mc b1.8.1 --particleemittercheck "$out/emitters181.bin"; then :; else fail=1; fi
if "$oracle" chests181 chests181.bin && "$beta" --mc b1.8.1 --chestcheck "$out/chests181.bin"; then :; else fail=1; fi
if "$oracle" modelregistry181 modelregistry181.bin && "$beta" --mc b1.8.1 --modelregistrycheck "$out/modelregistry181.bin"; then :; else fail=1; fi
if "$oracle" newliving181 newliving181.bin && "$beta" --mc b1.8.1 --newlivingcheck "$out/newliving181.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES181 ALL FOCUSED CASES EXACT" || echo "ENTITIES181 SOME CASES DIFFER"
exit "$fail"
