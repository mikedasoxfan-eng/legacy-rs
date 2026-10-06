#!/usr/bin/env bash
# Original Release 1.0 bytecode vs worldgen10, round 3. Keep every failure.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10/world3" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  if ! mkdir "$root/target/r10/verify10.lock" 2>/dev/null; then echo "another 1.0 verify is running" >&2; exit 3; fi
  trap 'rmdir "$root/target/r10/verify10.lock" 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
if [ "${1:-}" != --built ]; then
  cargo build --release --manifest-path "$root/Cargo.toml" -p worldgen10 --bin terrain_compare10 --bin compare10 || exit 1
fi
oracle="$root/tools/oracle10/oracle.sh"
terrain="$root/target/release/terrain_compare10"
world="$root/target/release/compare10"
fail=0
check() {
  local label="$1" binary="$2"; shift 2
  local oargs=() cargs=()
  while [ "$1" != -- ]; do oargs+=("$1"); shift; done; shift
  cargs=("$@")
  local file="world3-$(echo "$label" | tr ' /:,' '_____').bin"
  echo "== r10world3 $label"
  if ! r10_oracle "$oracle" "${oargs[@]}" "$file" >"$root/target/r10/world3/$file.log" 2>&1; then
    cat "$root/target/r10/world3/$file.log"; echo "ORACLE FAILED: $label"; fail=1
  elif ! r10_compare "$binary" "${cargs[@]}" "$root/tools/oracle10/out/$file"; then
    echo "DIFFERS: $label"; fail=1
  fi
}
group="${WORLD3_GROUP:-all}"
case "$group" in all|additions|core|terrain|features|world|structures) ;; *) echo "unknown WORLD3_GROUP: $group" >&2; exit 2 ;; esac
if [ "$group" = all ] || [ "$group" = additions ]; then
  check threaded "$terrain" r10world3threaded -- world3 threaded 42
  check guard "$terrain" r10world3guard -- world3 guard 42
  for seed in 0 42 -1 9223372036854775807 -9223372036854775808; do
    check "biomes/$seed" "$terrain" r10world3biomes "$seed" -- world3 biomes "$seed"
    check "misc/$seed" "$terrain" r10world3misc "$seed" -- world3 misc "$seed"
  done
  for kind in 0 1 2 3 4 7 8; do
    check "village/inhabitants/$kind" "$terrain" r10world3village "$kind" 42 -- world3 "village$kind" 42
  done
  for seed in 0 42 -1; do
    for mode in layout map blocks; do
      check "mineshaft/$mode/$seed" "$terrain" structures10 "$mode" "$seed" -- structures "$mode" "$seed"
    done
  done
  for mode in sky skygrid colors locations lifetime dungeon; do
    check "round2/$mode/42" "$terrain" terrainround2 "$mode" 42 -- round2 "$mode" 42
  done
fi
if [ "$group" = all ] || [ "$group" = core ]; then
  check worldheight "$terrain" r10world3 worldheight 42 -- world3 worldheight 42
  check profiler "$terrain" r10world3 profiler 42 -- world3 profiler 42
  check endprovider "$terrain" r10world3 endprovider 42 -- world3 endprovider 42
  check path "$terrain" r10world3path -- world3 path 42
  for seed in 0 42 -1 9223372036854775807 -9223372036854775808; do
    for mode in endterrain endpopulation spikes intmap longmap; do
      check "$mode/$seed" "$terrain" r10world3 "$mode" "$seed" -- world3 "$mode" "$seed"
    done
  done
fi
if [ "$group" = all ] || [ "$group" = terrain ]; then
  for c in '0 -37 -29 63 41' '3257840388504953787 -256 -512 79 63' '-4172144997902289642 300 -100 43 29' '9223372036854775807 -1 -1 17 13' '-9223372036854775808 12550784 -12550784 31 17'; do
    read -r seed x z width height <<< "$c"
    check "layers/$seed" "$terrain" terrain layers "$seed" "$x" "$z" "$width" "$height" -- layers "$seed" "$x" "$z" "$width" "$height"
    check "utilities/$seed" "$terrain" terrainutils10 "$seed" -- utilities "$seed"
    check "biomecache/$seed" "$terrain" biomecache10 "$seed" -- cache "$seed"
    check "structurestart/$seed" "$terrain" structurestarts10 "$seed" -- starts "$seed"
    check "spawn/$seed" "$terrain" spawn10 "$seed" -- spawn "$seed"
    check "raw/$seed" "$terrain" terrain raw "$seed" -3 -3 6 6 -- raw "$seed" -3 -3 6 6
  done
fi
if [ "$group" = all ] || [ "$group" = features ]; then
  for name in oak birch taiga1 taiga2 big swamp brown red lake lava dungeon; do
    for fixture in 0 1 2 3; do
      check "feature/$name/$fixture" "$terrain" features10 "$name" 42 "$fixture" -- feature "$name" 42 "$fixture"
    done
  done
  check feature/dungeon/chests "$terrain" features10 dungeon 42 4 -- feature dungeon 42 4
  for id in 0 1 2 3 4 5 6 7 8 10 11 12 13 14 15; do
    for fixture in 0 1; do
      check "decorator/$id/$fixture" "$terrain" features10 "decorate$id" 42 "$fixture" -- feature "decorate$id" 42 "$fixture"
    done
  done
fi
if [ "$group" = all ] || [ "$group" = world ]; then
  for mode in cache weather arrays savelife nextticks frost fileclasses math storage relocation regioncache4 oldformat4 emptycache4 legacy5 conversion6; do
    check "world/$mode" "$world" "wb$mode" -- worldblocks181 "$mode"
  done
  check world/lights "$world" worldblocks10 lights -- worldblocks181 lights
fi
if [ "$group" = all ] || [ "$group" = structures ]; then
  for seed in 0 42 -1 9223372036854775807 -9223372036854775808; do
    check "fortress/layout/$seed" "$terrain" r10world3structures layout "$seed" -- world3 fortresslayout "$seed"
    check "fortress/map/$seed" "$terrain" r10world3structures map "$seed" -- world3 fortressmap "$seed"
  done
  for kind in {0..13}; do
    check "fortress/piece/$kind" "$terrain" r10world3structures "fortress$kind" 42 -- world3 "fortress$kind" 42
  done
  for kind in portal chest; do
    check "stronghold/new/$kind" "$terrain" r10world3structures "$kind" 42 -- world3 "$kind" 42
  done
  for seed in 0 42 -1; do
    check "component/$seed" "$terrain" component10 "$seed" -- component "$seed"
    for mode in layout map blocks wet pieces; do
      check "stronghold/$mode/$seed" "$terrain" strongholds10 "$mode" "$seed" -- structures "stronghold-$mode" "$seed"
    done
    for mode in village-layout village-map village-blocks; do
      check "$mode/$seed" "$terrain" structures10 "$mode" "$seed" -- structures "$mode" "$seed"
    done
    for mode in longset scan saves; do
      check "provider/$mode/$seed" "$terrain" provider10 "$mode" "$seed" -- structures "provider-$mode" "$seed"
    done
    for mode in cache null failure; do
      check "loadorgenerate/$mode/$seed" "$terrain" chunkcache10 "$mode" "$seed" -- structures "chunkcache-$mode" "$seed"
    done
    check "nbt/$seed" "$terrain" terrainnbt10 "$seed" -- structures terrain-nbt "$seed"
  done
fi
[ "$fail" = 0 ] && echo "R10 WORLD ROUND 3 EXACT" || echo "R10 WORLD ROUND 3 HAS DIFFERENCES"
exit "$fail"
