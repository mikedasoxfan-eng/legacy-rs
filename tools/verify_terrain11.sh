#!/usr/bin/env bash
# Original 1.0 bytecode checks for the terrain worker's additions.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r11" "$root/tools/oracle11/out"
source "$root/tools/r11/verify_common10.sh"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
  mkdir -p "$root/target/r11"
  if ! mkdir "$root/target/r11/verify11.lock" 2>/dev/null; then echo "another 1.0 verify is running" >&2; exit 3; fi
  trap 'rmdir "$root/target/r11/verify11.lock" 2>/dev/null' EXIT
  export VERIFY11_LOCKED=1
fi
if [ "${1:-}" = --built ]; then shift; else cargo build --release --manifest-path "$root/Cargo.toml" -p worldgen11 --bin terrain_compare11 || exit 1; fi
oracle="$root/tools/oracle11/oracle.sh"
cmp="$root/target/release/terrain_compare11"
out="$root/tools/oracle11/out"
fail=0
check() { local label="$1" mode="$2"; shift 2; echo "== $label seed $1"; r10_oracle "$oracle" "$mode" "$@" terrain-worker.bin >/dev/null && r10_compare "$cmp" "$label" "$@" "$out/terrain-worker.bin" || fail=1; }
for c in '0 -37 -29 63 41' '3257840388504953787 -256 -512 79 63' '-4172144997902289642 300 -100 43 29' '9223372036854775807 -1 -1 17 13' '-9223372036854775808 12550784 -12550784 31 17'; do
  read -r seed x z w h <<< "$c"
  echo "== GenLayer: $c"
  r10_oracle "$oracle" terrain layers "$seed" "$x" "$z" "$w" "$h" terrain-worker.bin >/dev/null && r10_compare "$cmp" layers "$seed" "$x" "$z" "$w" "$h" "$out/terrain-worker.bin" || fail=1
done
for seed in 0 3257840388504953787 -4172144997902289642 9223372036854775807 -9223372036854775808; do
  check biomes biomes181 "$seed"
  check spawn spawn181 "$seed"
  check utilities terrainutils181 "$seed"
  check cache biomecache181 "$seed"
  check starts structurestarts181 "$seed"
done
for c in '0 -4 -4 8 8' '3257840388504953787 -3 -3 6 6' '-4172144997902289642 -2 -2 4 4' '42 784420 0 4 2' '42 -3 -784431 2 4' '9223372036854775807 -1 -1 3 3'; do
  read -r seed x z w h <<< "$c"
  echo "== Sky: $c"
  r10_oracle "$oracle" terrain sky "$seed" "$x" "$z" "$w" "$h" terrain-worker.bin >/dev/null && r10_compare "$cmp" sky "$seed" "$x" "$z" "$w" "$h" "$out/terrain-worker.bin" || fail=1
done
for name in oak birch taiga1 taiga2 big swamp brown red clay sand gravel ore lake lava dungeon; do
  for fixture in 0 1 2 3; do
    echo "== feature $name fixture $fixture"
    r10_oracle "$oracle" features181 "$name" 42 "$fixture" terrain-worker.bin >/dev/null && r10_compare "$cmp" feature "$name" 42 "$fixture" "$out/terrain-worker.bin" || fail=1
  done
done
# Retain these regression cases even while their lighting/chest dependencies differ.
for fixture in 0 1; do
  for id in {0..9}; do
    echo "== BiomeDecorator $id fixture $fixture"
    r10_oracle "$oracle" features181 "decorate$id" 42 "$fixture" terrain-worker.bin >/dev/null && r10_compare "$cmp" feature "decorate$id" 42 "$fixture" "$out/terrain-worker.bin" || fail=1
  done
done
echo "== dungeon with successful chest placement"
r10_oracle "$oracle" features181 dungeon 42 4 terrain-worker.bin >/dev/null && r10_compare "$cmp" feature dungeon 42 4 "$out/terrain-worker.bin" || fail=1
for seed in 4096 4097 4100 4105; do
  for id in {0..9}; do
    echo "== population animals $id seed $seed"
    r10_oracle "$oracle" features181 "animals$id" "$seed" 0 terrain-worker.bin >/dev/null && r10_compare "$cmp" feature "animals$id" "$seed" 0 "$out/terrain-worker.bin" || fail=1
  done
done
[ "$fail" -eq 0 ] && echo "TERRAIN CASES EXACT" || echo "TERRAIN CASES HAVE DIFFERENCES (see docs/b181-terrain-report.md)"
exit "$fail"
