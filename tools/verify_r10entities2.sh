#!/usr/bin/env bash
# Release-only original-bytecode checks. Each section is independent.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p target/r10/entities2
fail=0
if [ "${1:-}" != --built ]; then
  cargo build --release -p worldgen10 --bin r10entities2 || exit 1
else
  shift
fi
selected=("$@")
python tools/r10/run_entities2_oracle.py --compile || exit 1
run() {
  local kind="$1"
  if [ "${#selected[@]}" -gt 0 ]; then
    local match=0 candidate
    for candidate in "${selected[@]}"; do
      [ "$candidate" = "$kind" ] && match=1
    done
    [ "$match" -eq 1 ] || return 0
  fi
  echo "== entities2: $kind"
  if python tools/r10/run_entities2_oracle.py "$kind" "target/r10/entities2/$kind.bin"; then
    target/release/r10entities2 "$kind" "target/r10/entities2/$kind.bin" || fail=1
  else
    echo "ORACLE FAILED: $kind"
    fail=1
  fi
}
for kind in dragonhistory dragondamage dragonflight dragondeath fishhook living6 livingdamage6 livingnbt6 rules egg snowball pearl splashpotion breeding blaze endereye playerxp replay-damage replay-potions; do
  run "$kind"
done
for mob in Cow Pig Sheep Chicken Zombie Skeleton Spider CaveSpider Creeper Squid Slime Ghast Wolf PigZombie Giant Enderman Silverfish Blaze LavaSlime MushroomCow SnowMan Villager; do
  run "drops-$mob"
done
for mob in Cow Pig Sheep Chicken Zombie Skeleton Spider CaveSpider Creeper Squid Slime Ghast Wolf PigZombie Giant Enderman Silverfish XPOrb Arrow Item; do
  run "replay-$mob"
done
for kind in inventory cartinventory api-registry api-watchers api-coordinates potionregistry damagesources pathvalues registry10 mob-SnowMan mob-LavaSlime mob-Villager mooshroom crystal smallfireball fireball food settings inventorycopy sleep fallingsand itempickup playersp explosion fortressspawns; do
  run "$kind"
done
for scene in rails train boats riding nbt longrail ridingnbt placing; do
  run "vehicle-$scene"
done
for scene in movement sprint regen haste fatigue nausea creativedamage creativevoid creativecontroller identity cancel xp damage flight speed bow eat260 starve eat335 eat373; do
  run "player-$scene"
done
for scene in slimebranches skeletonbranches wolfbranches vehiclebranches fireballbranches spawnorder spawnbranches factorybranches; do
  run "branches-$scene"
done
for scene in sun4 network4 retaliation4 impact4 flying4 explosionblocks4 creeperdeath4 creeper4; do
  run "round4-$scene"
done
for scene in vehicleriders5 watchers5 wolfremote5 health5 wolfattack5 wolffollow5 vehiclecollision5 vehicleinteract5 skeletondeath5 potionexpiry5; do
  run "round5-$scene"
done
exit "$fail"
