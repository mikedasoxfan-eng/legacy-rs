#!/usr/bin/env bash
# Compare the Rust generator against the Java oracle (Mojang's original b1.7.3 code
# running on the launcher's Java 8u51) across a fixed set of seeds and scenarios.
# Usage: tools/verify.sh [quick]     exit code 0 only if every case is an exact match
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
oracle="$root/tools/oracle/oracle.sh"
out="$root/tools/oracle/out"
cargo build --release -q --manifest-path "$root/Cargo.toml" -p worldgen -p client || exit 1
cmp="$root/target/release/compare"
beta="$root/target/release/beta"
quick="${1:-}"

fail=0
run() { # label, oracle args..., then "--", compare args (oracle output file appended)
  local label="$1"; shift
  local oargs=() cargs=()
  while [ "$1" != "--" ]; do oargs+=("$1"); shift; done; shift
  cargs=("$@")
  local f="case_$(echo "${oargs[*]}" | tr ' ' '_').bin"
  echo "== $label"
  if "$oracle" "${oargs[@]:0:1}" "${oargs[@]:1}" "$f" >/dev/null && "$cmp" "${cargs[@]}" "$out/$f"; then :; else fail=1; fi
  rm -f "$out/$f" "$out/$f.txt"
}
run_extra() { # like run, but the oracle takes an argument after the output file
  local label="$1" extra="$2"; shift 2
  local oargs=() cargs=()
  while [ "$1" != "--" ]; do oargs+=("$1"); shift; done; shift
  cargs=("$@")
  local f="case_$(echo "${oargs[*]}_$extra" | tr ' :,' '_xc' | cut -c1-80).bin"
  echo "== $label"
  if "$oracle" "${oargs[@]}" "$f" "$extra" >/dev/null && "$cmp" "${cargs[@]}" "$out/$f" "$extra"; then :; else fail=1; fi
  rm -f "$out/$f" "$out/$f.txt"
}

echo "== MathHelper sine table"
"$oracle" sin sin.bin && "$cmp" sin "$out/sin.bin" || fail=1

echo "== player inventory (10000 original-bytecode operations)"
"$oracle" playerinventory player_inventory.bin && "$cmp" playerinventory "$out/player_inventory.bin" || fail=1
for player_case in "movement 1000" "damage 1000" "pickup 1000" "sleep 300" "portal 300" "nbt 300" "look 1000" "dig 1000" "place 180" "use 300" "shoot 300" "interact 210" "fertilize 50" "activate 100" "api 200" "pipeline 100" "night 150" "sleepworld 150" "survival 800" "bucket 100"; do
  read -r kind ticks <<<"$player_case"
  run "player: $kind, every frame and final world" player "$kind" "$ticks" -- player "$kind" "$ticks"
done

echo "== entity geometry (10000 original-bytecode cases)"
"$oracle" entitygeometry entity_geometry.bin && "$cmp" entitygeometry "$out/entity_geometry.bin" || fail=1
echo "== all Beta block collision/selection states (10000 cases)"
"$oracle" entityblocks entity_blocks.bin && "$cmp" entityblocks "$out/entity_blocks.bin" || fail=1
for entity_case in "items 6005" "sand 300" "tnt 300" "arrows 1300" "passive 13000" "aquatic 3000" "hostile 3000" "special 3000" "spawning 3000" "spawner 3000" "client 3000" "nbt 1000" "nbtall 1000" "slimes 3000" "hooks 1000" "combat 3000" "stormclient 3000" "lightning 1000" "rider 3000" "nether 3000"; do
  read -r kind ticks <<<"$entity_case"
  run "entities: $kind, every frame for $ticks ticks" entities "$kind" "$ticks" -- entities "$kind" "$ticks"
done

# ---- base terrain (order-independent): seed x0 z0 x1 z1
raw_cases=(
  "0                    -8 -8 7 7          origin"
  "3257840388504953787  -6 -6 5 5          pack.png seed"
  "-4172144997902289642 -4 -4 3 3          negative seed"
  "123456789            100 -300 107 -293  mid-range"
  "42                   784420 0 784431 3  far lands x"
  "42                   -3 -784431 0 -784420  far lands -z"
  "9223372036854775807  -2 -2 2 2          max long seed"
)
for c in "${raw_cases[@]}"; do
  read -r seed x0 z0 x1 z1 label <<<"$c"
  run "raw terrain: $label" raw "$seed" "$x0" "$z0" "$x1" "$z1" -- raw "$seed"
done

# ---- Nether raw terrain (including lava-avoiding caves and Far Lands)
nether_raw_cases=(
  "0                    -8 -8 7 7          origin"
  "-4172144997902289642 -4 -4 3 3          negative seed"
  "123456789            100 -300 107 -293  mid-range"
  "42                   784420 0 784431 3  far lands x"
  "42                   -3 -784431 0 -784420  far lands -z"
  "9223372036854775807  -2 -2 2 2          max long seed"
)
for c in "${nether_raw_cases[@]}"; do
  read -r seed x0 z0 x1 z1 label <<<"$c"
  run "raw Nether: $label" rawnether "$seed" "$x0" "$z0" "$x1" "$z1" -- rawnether "$seed"
done

# Every lighting schedule is exercised, including shared RNG reseeds from nested loads.
for light in each end none; do
  run "Nether grid 8x8, light $light" gridnether 0 1 -4 -4 3 3 "$light" -- gridnether 0 1 -4 -4 3 3 "$light"
  run "Nether grid, negative seed, light $light" gridnether -4172144997902289642 99 -8 4 -1 11 "$light" -- gridnether -4172144997902289642 99 -8 4 -1 11 "$light"
  run "Nether grid, far coordinates, light $light" gridnether 42 7 784424 -3 784427 0 "$light" -- gridnether 42 7 784424 -3 784427 0 "$light"
done

# ---- populated worlds: vanilla new-world flow (spawn search + preload), seed wrand
pre_cases=(
  "0 1"
  "3257840388504953787 7"
  "-4172144997902289642 99"
  "123456789 2024"
  "404 5"
  "-1 -1"
)
[ "$quick" = quick ] && pre_cases=("0 1")
for c in "${pre_cases[@]}"; do
  read -r seed wr <<<"$c"
  run "new world: seed $seed, world rand $wr" preload "$seed" "$wr" -- preload "$seed" "$wr"
done
# Seed 25 has snowy forest at spawn: snow on leaves only survives with Fast graphics,
# so the two settings generate different terrain in vanilla.
run_extra "new world, Fast graphics: seed 25" fast preload 25 1 -- preload 25 1
run_extra "new world, Fancy graphics: seed 25" fancy preload 25 1 -- preload 25 1

# ---- populated worlds: plain grids with different lighting schedules
if [ "$quick" != quick ]; then
  run "grid 24x24, light after each chunk"  grid 77 3 -12 -12 11 11 each -- grid 77 3 -12 -12 11 11 each
  run "grid 16x16, light only at the end"   grid 5555 8 0 0 15 15 end -- grid 5555 8 0 0 15 15 end
  run "grid 16x16, no light processing"     grid -98765 4 -40 20 -25 35 none -- grid -98765 4 -40 20 -25 35 none
fi

# ---- rendering: RenderBlocks vertex output (smooth lighting, Fancy) over 11x11 chunks
mesh_cases=("25 1" "1 1")   # snowy taiga; desert with cactus
[ "$quick" != quick ] && mesh_cases+=("-1 -1" "0 1")
for c in "${mesh_cases[@]}"; do
  read -r seed wr <<<"$c"
  echo "== chunk meshes: seed $seed, world rand $wr"
  if "$oracle" rendermesh "$seed" "$wr" mesh_case.bin auto 5 2>/dev/null | grep -v -i "warning\|unsafe\|\^" ;      "$beta" --meshcheck "$seed" "$wr" "$out/mesh_case.bin" | tail -1 | grep -q "MESHES EXACT"; then
    echo "MESHES EXACT"
  else
    echo "MESHES DIFFER"; fail=1
  fi
  rm -f "$out/mesh_case.bin"
done

# ---- saves: real game reads ours, we read the real game's
echo "== save format: Java saves, Rust loads"
rm -rf "$root/saves/_verify_java" "$root/saves/_verify_rust"
if "$oracle" savepreload 0 1 ../../../saves/_verify_java v_mem.bin 2>/dev/null | grep -v -i "warning\|unsafe\|\^";    "$cmp" savecheck "$root/saves/_verify_java" "$out/v_mem.bin" | tail -1 | grep -q "EXACT MATCH"; then echo "EXACT MATCH"; else echo "DIFFERS"; fail=1; fi
echo "== save format: Rust saves, Java loads"
"$cmp" makesave 0 1 "$root/saves/_verify_rust" >/dev/null
if "$oracle" readsave ../../../saves/_verify_rust v_read.bin 2>/dev/null | grep -v -i "warning\|unsafe\|\^";    "$cmp" preload-worldonly 0 1 "$out/v_read.bin" | tail -1 | grep -q "EXACT MATCH"; then echo "EXACT MATCH"; else echo "DIFFERS"; fail=1; fi

# ---- server chunk handling: original server classes walking a player out and back
if [ "$quick" != quick ]; then
  echo "== server mode: 900-tick walk (real World.tick), unload/reload through region files"
  rm -rf "$root/saves/_verify_srv_java" "$root/saves/_verify_srv_rust"
  "$root/tools/oracle/server_oracle.sh" serverwalk 0 1 10 ../../../saves/_verify_srv_java 900 v_srv.bin >/dev/null 2>&1
  "$cmp" serverwalk 0 1 10 "$root/saves/_verify_srv_rust" 900 "$out/v_srv.bin" | tail -1 | grep -q "EXACT MATCH" && echo "in memory: EXACT MATCH" || { echo "in memory: DIFFERS"; fail=1; }
  "$oracle" readsave ../../../saves/_verify_srv_java v_srv_saved.bin >/dev/null 2>&1
  "$cmp" savecheck "$root/saves/_verify_srv_rust" "$out/v_srv_saved.bin" | tail -1 | grep -q "EXACT MATCH" && echo "on disk: EXACT MATCH" || { echo "on disk: DIFFERS"; fail=1; }
fi
rm -rf "$root"/saves/_verify_* "$out"/v_*.bin

# ---- world ticking: weather, scheduled and random block ticks, cave-sound and lightning
# draws, chunk loading around the player. seed wrand lcg sound ticks weather dx dz [fires]
tick_cases=(
  "2 3 -77 3000 2000 thunder 0 0"      # snow and ice forming, lightning
  "1 7 999 100 2000 rain 40 -30"
  "-1 5 4242 0 2000 rain -100 60"
)
[ "$quick" = quick ] && tick_cases=("1 1 12345 5 300 clear 0 0")
for c in "${tick_cases[@]}"; do
  run "ticks: $c" ticks $c -- ticks $c
done
if [ "$quick" != quick ]; then
  # Seed 6 spawns in forest: set it alight (fire spread, logs burning, leaf decay).
  fires="12:0,12:12,16:0,-16:-16,20:0,20:20,-20:-20,24:0,0:-24,28:28,-28:-28,0:-32,32:32"
  run_extra "ticks: forest fire, seed 6" "$fires" ticks 6 1 777 5 4000 clear 0 0 -- ticks 6 1 777 5 4000 clear 0 0
fi

# ---- mob geometry, UVs, successive poses and packed normals
echo "== mob models: boxes, UVs, successive poses and wolf living animations"
if "$oracle" models models_case.bin && "$beta" --modelcheck "$out/models_case.bin"; then :; else fail=1; fi
rm -f "$out/models_case.bin"

# ---- sky, fog and cloud colors, sun angle, night darkness
echo "== sky colors"
for args in "3257840388504953787 -39 65 6000 0.0" "0 1000 -2000 13000 0.5" "123 -500 77 23500 0.25" "42 99999 -12345 18000 0.9" "1 0 0 12500 0.75" \
  "1 0 0 6000 0.5 1 1 2" "0 1000 -2000 13000 0.5 0.75 0.4 1" \
  "123 -500 77 23500 0.25 0.33 0.9 0" "42 99999 -12345 18000 0.9 1 1 1" \
  "2 -1 -17 12500 0.0 0.1 1 2 0.9 0.3" "11 0 0 0 1.0 1 0.7 1 0.2 0.9" \
  "42 0 0 6000 0.25 1 1 2 0.9 0.7 0.0 0.1"; do
  j=$("$oracle" skycolor $args); r=$("$beta" --skycolor $args)
  [ "$j" = "$r" ] || { echo "sky differs for $args"; echo " java $j"; echo " rust $r"; fail=1; }
done
[ $fail -eq 0 ] && echo "sky colors exact"

# ---- original EntityRenderer precipitation quads and particle spawn RNG,
# RenderLightningBolt strips. Inputs: seed camera-x y z renderer-tick partial rain.
echo "== ambient weather vertices"
for args in "2 0.5 70 0.5 100 0.75 0.8" "11 0.5 80 0.5 2147483647 0.25 1" \
  "3257840388504953787 -39.5 75 65.5 23 0.5 1" "1 0.5 70 0.5 0 0 1" \
  "42 -20.25 90 -31.75 -2147483648 1.0 0.01"; do
  if "$oracle" weathermesh $args weather_case.bin && "$beta" --weathercheck $args "$out/weather_case.bin"; then :; else fail=1; fi
done
for scene in lava roof; do
  args="42 0.5 80 0.5 123 0.5 1"
  if "$oracle" weathermesh $args weather_case.bin "$scene" 5 && "$beta" --weathercheck $args "$out/weather_case.bin" "$scene" 5; then :; else fail=1; fi
done
rm -f "$out/weather_case.bin"

[ $fail -eq 0 ] && echo "ALL CASES EXACT" || echo "SOME CASES DIFFER"
exit $fail
