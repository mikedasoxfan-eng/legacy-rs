#!/usr/bin/env bash
# Focused 1.0 entity checks; replays original Mojang bytecode.
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
oracle="$root/tools/oracle11/oracle.sh"
out="$root/tools/oracle11/out"
cmp="$root/target/release/entity_compare11.exe"
beta="$root/target/release/beta.exe"
fail=0
for spec in "rules 0" "indirect 20" "stare 10" "starepumpkin 10" "population 0" "carry 500" "allies 40" "attract 100" "Boat 100" "Minecart 100" "Item 100" "Lightning 20" "Snowball 30" "Egg 30" "Fireball 20" "Enderman 100" "teleport 40" "CaveSpider 100" "Silverfish 100" "XPOrb 100" "Arrow 100" "Pig 100" "Cow 100" "Chicken 100" "Sheep 100" "Wolf 100" "Zombie 100" "Skeleton 100" "Spider 100" "Creeper 100" "PigZombie 100" "Ghast 100" "Squid 100" "Slime 100" "potions 100" "damage 100" "poison 40" "kill-Cow 40" "kill-Chicken 40" "kill-Sheep 40" "kill-Zombie 40" "kill-Enderman 40"; do
  read -r kind ticks <<< "$spec"
  echo "== entities10: $kind"
  if r10_oracle "$oracle" entities181 "$kind" "$ticks" "entity181_${kind}.bin" && r10_compare "$cmp" "$kind" "$ticks" "$out/entity181_${kind}.bin"; then :; else fail=1; fi
done
echo "== entities10: entitygeometry"
if r10_oracle "$oracle" entitygeometry entitygeometry181.bin && r10_compare "$root/target/release/compare11.exe" entitygeometry "$out/entitygeometry181.bin"; then :; else fail=1; fi
echo "== entities10: models10"
if r10_oracle "$oracle" models181 models181.bin && r10_client "$beta" --mc 1.1 --modelcheck "$out/models181.bin"; then :; else fail=1; fi
echo "== entities10: particles10"
if r10_oracle "$oracle" particles181 particles181.bin && r10_client "$beta" --mc 1.1 --particlecheck "$out/particles181.bin"; then :; else fail=1; fi
echo "== entities10: xprender10"
if r10_oracle "$oracle" xprender181 xprender181.bin && r10_client "$beta" --mc 1.1 --renderercheck "$out/xprender181.bin"; then :; else fail=1; fi
if r10_oracle "$oracle" vehiclerenders181 vehiclerenders181.bin && r10_client "$beta" --mc 1.1 --vehiclerenderercheck "$out/vehiclerenders181.bin"; then :; else fail=1; fi
if r10_oracle "$oracle" bedmodel181 bedmodel181.bin && r10_client "$beta" --mc 1.1 --bedmodelcheck "$out/bedmodel181.bin"; then :; else fail=1; fi
if r10_oracle "$oracle" arrows181 arrows181.bin && r10_client "$beta" --mc 1.1 --arrowcheck "$out/arrows181.bin"; then :; else fail=1; fi
# Round 2: packed light, wire APIs, combat, explosions and renderer callbacks.
for name in Pig Wolf Skeleton Spider CaveSpider Creeper PigZombie Ghast Squid Slime Silverfish; do
  if r10_oracle "$oracle" entities181 "kill-$name" 40 "entity181_kill-${name}.bin" && r10_compare "$cmp" "kill-$name" 40 "$out/entity181_kill-${name}.bin"; then :; else fail=1; fi
done
for kind in watchers coordinates light registry; do
  if r10_oracle "$oracle" entityapi181 "$kind" 0 "entityapi181_${kind}.bin" && r10_compare "$cmp" "$kind" 0 "$out/entityapi181_${kind}.bin"; then :; else fail=1; fi
done
if r10_oracle "$oracle" combat181 combat 0 combat181.bin && r10_compare "$cmp" combat 0 "$out/combat181.bin"; then :; else fail=1; fi
if r10_oracle "$oracle" explosion181 explosion 0 explosion181.bin && r10_compare "$cmp" explosion 0 "$out/explosion181.bin"; then :; else fail=1; fi
if r10_oracle "$oracle" particles2_181 particles2_181.bin && r10_client "$beta" --mc 1.1 --particlecheck "$out/particles2_181.bin"; then :; else fail=1; fi
if r10_oracle "$oracle" emitters181 emitters181.bin && r10_client "$beta" --mc 1.1 --particleemittercheck "$out/emitters181.bin"; then :; else fail=1; fi
if r10_oracle "$oracle" chests181 chests181.bin && r10_client "$beta" --mc 1.1 --chestcheck "$out/chests181.bin"; then :; else fail=1; fi
if r10_oracle "$oracle" modelregistry181 modelregistry181.bin && r10_client "$beta" --mc 1.1 --modelregistrycheck "$out/modelregistry181.bin"; then :; else fail=1; fi
if r10_oracle "$oracle" newliving181 newliving181.bin && r10_client "$beta" --mc 1.1 --newlivingcheck "$out/newliving181.bin"; then :; else fail=1; fi
[ "$fail" -eq 0 ] && echo "ENTITIES181 ALL FOCUSED CASES EXACT" || echo "ENTITIES181 SOME CASES DIFFER"
exit "$fail"
