#!/usr/bin/env bash
# Release world round four. Each section compares original bytecode to Rust.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r10/world4" "$root/tools/oracle10/out"
source "$root/tools/r10/verify_common10.sh"
if [ -z "${VERIFY10_LOCKED:-}" ]; then
  if ! mkdir "$root/target/r10/verify10.lock" 2>/dev/null; then echo "another 1.0 verify is running" >&2; exit 3; fi
  trap 'rmdir "$root/target/r10/verify10.lock" 2>/dev/null' EXIT
  export VERIFY10_LOCKED=1
fi
if [ "${1:-}" != --built ]; then
  cargo build --release --manifest-path "$root/Cargo.toml" || exit 1
fi
oracle="$root/tools/oracle10/oracle.sh"
terrain="$root/target/release/terrain_compare10"
world="$root/target/release/compare10"
fail=0
check() {
  local label="$1" binary="$2"; shift 2
  local oargs=()
  while [ "$1" != -- ]; do oargs+=("$1"); shift; done; shift
  local file="world4-$(echo "$label" | tr ' /:,' '_____').bin"
  echo "== r10world4 $label"
  if ! r10_oracle "$oracle" "${oargs[@]}" "$file" >"$root/target/r10/world4/$file.log" 2>&1; then
    cat "$root/target/r10/world4/$file.log"; echo "ORACLE FAILED: $label"; fail=1
  elif ! r10_compare "$binary" "$@" "$root/tools/oracle10/out/$file"; then
    echo "DIFFERS: $label"; fail=1
  fi
}
group="${WORLD4_GROUP:-all}"
case "$group" in all|new|inherited) ;; *) echo "unknown WORLD4_GROUP: $group" >&2; exit 2 ;; esac
if [ "$group" = all ] || [ "$group" = new ]; then
  check geometry "$root/target/release/beta" r10world4geometry -- --mc 1.0 --world4geometrycheck
  for mode in intcache misc savefolders notify endteleport; do
    check "$mode" "$terrain" r10world4 "$mode" 0 -- world4 "$mode" 0
  done
  for seed in 0 42 -1; do
    for mode in nbt gzipbytes colors nearest waterlily population hellpopulation ticks hellticks fortressspawn providerapi worldinfo regionbytes hellregionbytes saveformat; do
      check "$mode/$seed" "$terrain" r10world4 "$mode" "$seed" -- world4 "$mode" "$seed"
    done
  done
fi
if [ "$group" = all ] || [ "$group" = inherited ]; then
  for mode in threaded worldheight endprovider; do
    if [ "$mode" = threaded ]; then oracle_mode=r10world3threaded; else oracle_mode=r10world3; fi
    if [ "$mode" = threaded ]; then
      check "$mode" "$terrain" "$oracle_mode" -- world3 "$mode" 42
    else
      check "$mode" "$terrain" "$oracle_mode" "$mode" 42 -- world3 "$mode" 42
    fi
  done
  for seed in 0 42 -1; do
    for mode in endterrain endpopulation; do
      check "$mode/$seed" "$terrain" r10world3 "$mode" "$seed" -- world3 "$mode" "$seed"
    done
    check "biomes/$seed" "$terrain" r10world3biomes "$seed" -- world3 biomes "$seed"
    check "biomecache/$seed" "$terrain" biomecache10 "$seed" -- cache "$seed"
    check "fortress/map/$seed" "$terrain" r10world3structures map "$seed" -- world3 fortressmap "$seed"
    for mode in cache null failure; do
      check "loadorgenerate/$mode/$seed" "$terrain" chunkcache10 "$mode" "$seed" -- structures "chunkcache-$mode" "$seed"
    done
    for mode in longset scan saves; do
      check "provider/$mode/$seed" "$terrain" provider10 "$mode" "$seed" -- structures "provider-$mode" "$seed"
    done
  done
  for mode in arrays cache weather nextticks frost savelife saves storage relocation regioncache4 oldformat4 emptycache4 legacy5; do
    if [ "$mode" = saves ]; then
      check "world/$mode" "$world" worldblocks181 "$mode" -- worldblocks181 "$mode"
    else
      check "world/$mode" "$world" "wb$mode" -- worldblocks181 "$mode"
    fi
  done
  check world/lights "$world" worldblocks10 lights -- worldblocks181 lights
  check teleporter "$world" worldblocks10 teleporter -- worldblocks181 teleporter
fi
[ "$fail" = 0 ] && echo "R10 WORLD ROUND 4 EXACT" || echo "R10 WORLD ROUND 4 HAS DIFFERENCES"
exit "$fail"
