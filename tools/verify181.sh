#!/usr/bin/env bash
# Compare the Rust generator against the Java oracle (Mojang's original b1.8.1 code
# running on the launcher's Java 8u51) across a fixed set of seeds and scenarios.
# Usage: tools/verify181.sh [quick]     exit code 0 only if every case is an exact match
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
# One run at a time per tree (two runs would overwrite each other's oracle output).
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p "$root/target"
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock); remove it if stale" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
beta() { "$root/target/release/beta" --mc b1.8.1 "$@"; }
oracle="$root/tools/oracle181/oracle.sh"
out="$root/tools/oracle181/out"
cargo build --release -q --manifest-path "$root/Cargo.toml" -p worldgen181 -p client || exit 1
cmp="$root/target/release/compare181"
quick="${1:-}"

fail=0
bash "$root/tools/verify_vehicles181.sh" --built || fail=1
run() { # label, oracle args..., then "--", compare args (oracle output file appended)
  local label="$1"; shift
  local oargs=() cargs=()
  while [ "$1" != "--" ]; do oargs+=("$1"); shift; done; shift
  cargs=("$@")
  local f="case_$(echo "${oargs[*]}" | tr ' ' '_').bin"
  echo "== $label"
  if ! "$oracle" "${oargs[@]:0:1}" "${oargs[@]:1}" "$f" >"$out/$f.log" 2>&1; then
    cat "$out/$f.log"; echo "ORACLE FAILED: $label"; fail=1
  elif ! "$cmp" "${cargs[@]}" "$out/$f"; then
    echo "DIFFERS: $label"; fail=1
  fi
  rm -f "$out/$f" "$out/$f.txt" "$out/$f.events" "$out/$f.identities" "$out/$f.frames" "$out/$f.log"
}
run_extra() { # like run, but the oracle takes an argument after the output file
  local label="$1" extra="$2"; shift 2
  local oargs=() cargs=()
  while [ "$1" != "--" ]; do oargs+=("$1"); shift; done; shift
  cargs=("$@")
  local f="case_$(echo "${oargs[*]}_$extra" | tr ' :,' '_xc' | cut -c1-80).bin"
  echo "== $label"
  if ! "$oracle" "${oargs[@]}" "$f" "$extra" >"$out/$f.log" 2>&1; then
    cat "$out/$f.log"; echo "ORACLE FAILED: $label"; fail=1
  elif ! "$cmp" "${cargs[@]}" "$out/$f" "$extra"; then
    echo "DIFFERS: $label"; fail=1
  fi
  rm -f "$out/$f" "$out/$f.txt" "$out/$f.events" "$out/$f.identities" "$out/$f.frames" "$out/$f.log"
}

echo "== MathHelper sine table"
"$oracle" sin sin.bin && "$cmp" sin "$out/sin.bin" && echo "sine table EXACT" || fail=1

echo "== player inventory (10000 original-bytecode operations)"
"$oracle" playerinventory player_inventory.bin && "$cmp" playerinventory "$out/player_inventory.bin" || fail=1
echo "== container clicks, recipes, furnace and dispenser world replay"
bash "$root/tools/verify_containers181.sh" || fail=1
for player_case in "movement 1000" "damage 1000" "pickup 1000" "sleep 300" "portal 300" "nbt 300" "look 1000" "dig 1000" "place 360" "use 300" "shoot 300" "interact 210" "fertilize 50" "activate 100" "api 200" "pipeline 100" "night 150" "sleepworld 150" "survival 800" "bucket 100" "combat 400" "pickentities 200" "digextras 1600" "slimecontact 360" "stats 400"; do
  read -r kind ticks <<<"$player_case"
  run "player: $kind, every frame and final world" playerintegration181 "$kind" "$ticks" -- player "$kind" "$ticks"
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
  for mode in "fancy on" "fast on" "fancy off" "fast off"; do
    read -r graphics ao <<<"$mode"
    echo "== chunk meshes: seed $seed, world rand $wr, $graphics, AO $ao"
    if "$oracle" rendermesh "$seed" "$wr" mesh_case.bin auto 5 "$graphics" "$ao" >"$out/mesh_case.log" 2>&1 &&
       beta --meshcheck "$seed" "$wr" "$out/mesh_case.bin" "$graphics" "$ao" >"$out/mesh_compare.log" 2>&1; then
      tail -1 "$out/mesh_compare.log"
    else
      cat "$out/mesh_case.log" "$out/mesh_compare.log"
      echo "MESHES DIFFER"; fail=1
    fi
    rm -f "$out/mesh_case.bin" "$out/mesh_case.log" "$out/mesh_compare.log"
  done
done

# ---- saves: real game reads ours, we read the real game's
echo "== save format: Java saves, Rust loads"
rm -rf "$root/b1.8.1/saves/_verify_java" "$root/b1.8.1/saves/_verify_rust"
if "$oracle" savepreload 0 1 ../../../b1.8.1/saves/_verify_java v_mem.bin 2>/dev/null | grep -v -i "warning\|unsafe\|\^";    "$cmp" savecheck "$root/b1.8.1/saves/_verify_java" "$out/v_mem.bin" | tail -1 | grep -q "EXACT MATCH"; then echo "EXACT MATCH"; else echo "DIFFERS"; fail=1; fi
echo "== save format: Rust saves, Java loads"
"$cmp" makesave 0 1 "$root/b1.8.1/saves/_verify_rust" >/dev/null
if "$oracle" readsave ../../../b1.8.1/saves/_verify_rust v_read.bin 2>/dev/null | grep -v -i "warning\|unsafe\|\^";    "$cmp" preload-worldonly 0 1 "$out/v_read.bin" | tail -1 | grep -q "EXACT MATCH"; then echo "EXACT MATCH"; else echo "DIFFERS"; fail=1; fi

# ---- server chunk handling: original server classes walking a player out and back
if [ "$quick" != quick ]; then
  echo "== server mode: 900-tick walk (real World.tick), unload/reload through region files"
  rm -rf "$root/b1.8.1/saves/_verify_srv_java" "$root/b1.8.1/saves/_verify_srv_rust"
  "$root/tools/oracle181/server_oracle.sh" serverwalk 0 1 10 ../../../b1.8.1/saves/_verify_srv_java 900 v_srv.bin >"$out/v_srv.log" 2>&1 || { cat "$out/v_srv.log"; echo "ORACLE FAILED: server walk"; fail=1; }
  "$cmp" serverwalk 0 1 10 "$root/b1.8.1/saves/_verify_srv_rust" 900 "$out/v_srv.bin" | tail -1 | grep -q "EXACT MATCH" && echo "in memory: EXACT MATCH" || { echo "in memory: DIFFERS"; fail=1; }
  "$oracle" readsave ../../../b1.8.1/saves/_verify_srv_java v_srv_saved.bin >/dev/null 2>&1
  "$cmp" savecheck "$root/b1.8.1/saves/_verify_srv_rust" "$out/v_srv_saved.bin" | tail -1 | grep -q "EXACT MATCH" && echo "on disk: EXACT MATCH" || { echo "on disk: DIFFERS"; fail=1; }
  python "$root/tools/oracle181/compare_regions.py" "$root/b1.8.1/saves/_verify_srv_java" "$root/b1.8.1/saves/_verify_srv_rust" && echo "serialized chunk NBT: EXACT MATCH" || { echo "serialized chunk NBT: DIFFERS"; fail=1; }
fi
rm -rf "$root"/b1.8.1/saves/_verify_* "$out"/v_*.bin "$out"/v_*.bin.txt "$out"/v_*.bin.identities "$out"/v_*.bin.events "$out"/v_*.bin.frames "$out/v_srv.log"

# ---- world ticking: weather, scheduled and random block ticks, cave-sound and lightning
# draws, chunk loading around the player. seed wrand lcg sound ticks weather dx dz [fires]
tick_cases=(
  "2 3 -77 3000 2000 thunder 0 0"      # snow and ice forming, lightning
  "1 7 999 100 2000 rain 40 -30"
  "-1 5 4242 0 2000 rain -100 60"
)
[ "$quick" = quick ] && tick_cases=("1 1 12345 5 300 clear 0 0")
for c in "${tick_cases[@]}"; do
  run "ticks: $c" ticksintegration181 $c -- ticks $c
done
if [ "$quick" != quick ]; then
  # Seed 6 spawns in forest: set it alight (fire spread, logs burning, leaf decay).
  fires="12:0,12:12,16:0,-16:-16,20:0,20:20,-20:-20,24:0,0:-24,28:28,-28:-28,0:-32,32:32"
  run_extra "ticks: forest fire, seed 6" "$fires" ticksintegration181 6 1 777 5 4000 clear 0 0 -- ticks 6 1 777 5 4000 clear 0 0
  # 49 saplings of each kind around spawn, four days: BlockSapling growth and growTree.
  saplings=$(cat "$root/tools/oracle181/saplings.txt")
  run_extra "ticks: saplings growing, seed 6" "$saplings" ticksintegration181 6 1 777 5 96000 clear 0 0 -- ticks 6 1 777 5 96000 clear 0 0
fi

# ---- every RenderBlocks render type: each block id x metadata plus neighbour arrangements
# (tools/oracle181/make_showcase.py), in all four graphics / smooth-lighting combinations
showcase="$root/tools/oracle181/showcase.txt"
for m in "fancy on" "fancy off" "fast on" "fast off"; do
  [ "$quick" = quick ] && [ "$m" != "fancy on" ] && continue
  for stereo in normal anaglyph; do
  set -- $m
  echo "== block showcase: $m, $stereo"
  if "$oracle" rendershowcase "$(cygpath -w "$showcase")" showcase_mesh.bin "$1" "$2" "$stereo" >/dev/null 2>&1     && beta --meshcheck 0 0 "$out/showcase_mesh.bin" "$1" "$2" "showcase=$showcase" "$stereo" | tail -1 | grep -q "MESHES EXACT"; then
    echo "MESHES EXACT"; else echo "MESHES DIFFER"; fail=1; fi
  rm -f "$out/showcase_mesh.bin"
  done
done
if [ "$quick" != quick ]; then
  echo "== Nether terrain meshes (fire, glowstone, the Nether light table)"
  if "$oracle" rendermeshnether 5 1 mesh_nether.bin auto 2 fancy on >/dev/null 2>&1     && beta --meshcheck 5 1 "$out/mesh_nether.bin" fancy on nether | tail -1 | grep -q "MESHES EXACT"; then
    echo "MESHES EXACT"; else echo "MESHES DIFFER"; fail=1; fi
  rm -f "$out/mesh_nether.bin"
fi

echo "== moving piston renderer (original TileEntityRendererPiston vertex bits)"
if "$oracle" pistonrender piston_renderer.bin && beta --pistoncheck "$out/piston_renderer.bin"; then :; else fail=1; fi
rm -f "$out/piston_renderer.bin"

# ---- redstone circuits and rails: scripted block worlds, every tick of the circuit compared
for script in "$root"/tools/oracle181/redstone/*.txt; do
  name=$(basename "$script" .txt)
  echo "== redstone: $name"
  if "$oracle" redstone "$(cygpath -w "$script")" "rs_$name.bin" >/dev/null 2>&1     && "$cmp" redstone "$script" "$out/rs_$name.bin" | tail -1 | grep -q "EXACT MATCH"; then
    echo "EXACT MATCH"; else echo "DIFFERS"; fail=1; fi
  rm -f "$out/rs_$name.bin" "$out/rs_$name.bin.txt"
done

# ---- dimension travel: Teleporter (find, build, platform over the ocean) and the whole
# Minecraft.usePortal sequence. seed wrand dim x y z teleporter-rand / seed wrand x y z fresh trand
teleport_cases=(
  "3 1 0 100.5 70 -40.5 7"
  "3 1 -1 12.5 64 -5.5 9"
  "3 1 0 1000.5 70 1000.5 1"
  "5 2 -1 200.5 100 -33.5 5"
)
[ "$quick" = quick ] && teleport_cases=("3 1 -1 12.5 64 -5.5 9")
for c in "${teleport_cases[@]}"; do
  run "teleporter: $c" teleport $c -- teleport $c
done
if [ "$quick" != quick ]; then
  for c in "3 1 100.5 70 -40.5 11 7" "404 5 -17.5 66 0.5 12 8"; do
    run "use portal: $c" useportal $c -- useportal $c
  done
  # Minecraft.respawn: at spawn, at a bed, bed gone ("tile.bed.notValid")
  for c in "3 1 plain" "3 1 bed" "404 5 nobed"; do
    run "respawn: $c" respawn $c -- respawn $c
  done
fi

# ---- mob geometry, UVs, successive poses and packed normals
echo "== mob models: boxes, UVs, successive poses and wolf living animations"
if "$oracle" models181 models_case.bin && beta --modelcheck "$out/models_case.bin"; then :; else fail=1; fi
rm -f "$out/models_case.bin"

# ---- dropped/GUI items, falling blocks, TNT, arrows and projectile sprites
# Compare original Tessellator bits and per-draw GL transforms/state, before GPU math.
echo "== remaining entity renderers and GUI item geometry"
if "$oracle" renderers renderers_case.bin && beta --renderercheck "$out/renderers_case.bin"; then :; else fail=1; fi
echo "== ghast, pig zombie and giant full renderer draws"
if "$oracle" livingrenderers living_case.bin && beta --livingrenderercheck "$out/living_case.bin"; then :; else fail=1; fi
echo "== entity shadows and fire"
if "$oracle" rendereffects effects_case.bin && beta --effectcheck "$out/effects_case.bin"; then :; else fail=1; fi
rm -f "$out/renderers_case.bin" "$out/effects_case.bin" "$out/living_case.bin"

# ---- first-person camera, hand, overlays, player armor and targeting
# Original bytecode GL inputs, projection matrices and Tessellator vertex bits.
echo "== first-person view, player and block targeting"
if "$oracle" firstperson firstperson_case.bin && beta --firstpersoncheck "$out/firstperson_case.bin"; then :; else fail=1; fi
rm -f "$out/firstperson_case.bin"

# ---- sky, fog and cloud colors, sun angle, night darkness
echo "== renderer window sizes and Java negative-coordinate wrapping"
window_ok=1
for distance in 0 1 2 3; do
  for position in 0 -1 8 -8 208 -209 30000000 -30000000; do
    j=$("$oracle" videowindow "$distance" "$position"); r=$(beta --videowindow "$distance" "$position")
    [ "$j" = "$r" ] || { echo "window differs: $distance $position"; echo "java $j"; echo "rust $r"; fail=1; window_ok=0; }
  done
done
[ "$window_ok" -eq 1 ] && echo "renderer windows EXACT"
echo "== sky colors"
sky_ok=1
for args in "3257840388504953787 -39 65 6000 0.0" "0 1000 -2000 13000 0.5" "123 -500 77 23500 0.25" "42 99999 -12345 18000 0.9" "1 0 0 12500 0.75" \
  "1 0 0 6000 0.5 1 1 2" "0 1000 -2000 13000 0.5 0.75 0.4 1" \
  "123 -500 77 23500 0.25 0.33 0.9 0" "42 99999 -12345 18000 0.9 1 1 1" \
  "2 -1 -17 12500 0.0 0.1 1 2 0.9 0.3" "11 0 0 0 1.0 1 0.7 1 0.2 0.9" \
  "42 0 0 6000 0.25 1 1 2 0.9 0.7 0.0 0.1"; do
  for distance in 0 1 2 3; do
    read -r -a skyargs <<<"$args"
    defaults=(0 0 0 0 0 1 1)
    for ((i=${#skyargs[@]}; i<12; i++)); do
      # Previous rain/thunder default to current values, like the oracle.
      if [ "$i" = 8 ]; then skyargs+=("${skyargs[5]}")
      elif [ "$i" = 9 ]; then skyargs+=("${skyargs[6]}")
      else skyargs+=("${defaults[i-5]}"); fi
    done
    j=$("$oracle" skycolor "${skyargs[@]}" "$distance"); r=$(beta --skycolor "${skyargs[@]}" "$distance")
  [ "$j" = "$r" ] || { echo "sky differs for $args"; echo " java $j"; echo " rust $r"; fail=1; sky_ok=0; }
  done
done
# The Nether: WorldProviderHell's fog color, celestial angle 0.5, fog from the eye.
for args in "5 10 -20 6000 0.5" "5 300 40 18000 0.25 1 0 0 1 0" "9 -7 7 100 0.75 0.4 0.6 0 0.4 0.6 0.3 0.8"; do
  for distance in 0 2 3; do
    read -r -a skyargs <<<"$args"
    defaults=(0 0 0 0 0 1 1)
    for ((i=${#skyargs[@]}; i<12; i++)); do
      if [ "$i" = 8 ]; then skyargs+=("${skyargs[5]}")
      elif [ "$i" = 9 ]; then skyargs+=("${skyargs[6]}")
      else skyargs+=("${defaults[i-5]}"); fi
    done
    j=$("$oracle" skycolornether "${skyargs[@]}" "$distance"); r=$(beta --skycolor-nether "${skyargs[@]}" "$distance")
  [ "$j" = "$r" ] || { echo "nether sky differs for $args"; echo " java $j"; echo " rust $r"; fail=1; sky_ok=0; }
  done
done
[ "$sky_ok" -eq 1 ] && echo "sky colors EXACT"

echo "== original MusInputStream encrypted audio bytes"
if "$oracle" musaudio mus_case.bin && beta --muscheck "$out/mus_case.bin"; then :; else fail=1; fi
rm -f "$out/mus_case.bin"

echo "== EntityPickupFX item and arrow renderer draws"
if "$oracle" pickuprenderers pickup_draws.bin && beta --renderercheck "$out/pickup_draws.bin"; then :; else fail=1; fi
if "$oracle" pickuprenderersfast pickup_draws_fast.bin && beta --renderercheck "$out/pickup_draws_fast.bin"; then :; else fail=1; fi
rm -f "$out/pickup_draws.bin" "$out/pickup_draws_fast.bin"
echo "== note instruments, jukebox insertion/ejection and every step/break sound"
if "$oracle" musicevents music_case.bin && "$cmp" musicevents "$out/music_case.bin"; then :; else fail=1; fi
rm -f "$out/music_case.bin"
echo "== all ambient block callbacks, metadata and adjacent faces"
if "$oracle" displayevents display_case.bin && "$cmp" displayevents "$out/display_case.bin"; then :; else fail=1; fi
rm -f "$out/display_case.bin"
echo "== original EntityFX motion and vertex bits"
if "$oracle" particles particles_case.bin && beta --particlecheck "$out/particles_case.bin"; then :; else fail=1; fi
rm -f "$out/particles_case.bin"
echo "== EffectRenderer destruction, hits, tint, eviction and vertex bits"
if "$oracle" particleeffects particlefx_case.bin && beta --particlefxcheck "$out/particlefx_case.bin"; then :; else fail=1; fi
rm -f "$out/particlefx_case.bin"

# ---- original EntityRenderer precipitation quads and particle spawn RNG,
# RenderLightningBolt strips. Inputs: seed camera-x y z renderer-tick partial rain.
echo "== ambient weather vertices"
for args in "2 0.5 70 0.5 100 0.75 0.8" "11 0.5 80 0.5 2147483647 0.25 1" \
  "3257840388504953787 -39.5 75 65.5 23 0.5 1" "1 0.5 70 0.5 0 0 1" \
  "42 -20.25 90 -31.75 -2147483648 1.0 0.01"; do
  for graphics in fancy fast; do
    if "$oracle" weathermesh $args weather_case.bin natural 0 "$graphics" && beta --weathercheck $args "$out/weather_case.bin" natural 0 "$graphics"; then :; else fail=1; fi
  done
done
for scene in lava roof; do
  args="42 0.5 80 0.5 123 0.5 1"
  for graphics in fancy fast; do
    if "$oracle" weathermesh $args weather_case.bin "$scene" 5 "$graphics" && beta --weathercheck $args "$out/weather_case.bin" "$scene" 5 "$graphics"; then :; else fail=1; fi
  done
done
rm -f "$out/weather_case.bin"

# RenderGlobal cloud geometry, camera wrapping, above/below and within the cloud layer.
echo "== Fast and Fancy clouds"
for args in "1 0.5 70 0.5 100 0.75 6000 0 0" "42 -20.25 110 -31.75 123 0.5 13000 0.75 0.4" \
  "2 30000.25 120 -50000.75 2147483647 0.25 23500 1 1" "11 2048 107 0 -2147483648 1.0 18000 0.1 0.9"; do
  for graphics in fancy fast; do
    for stereo in normal anaglyph; do
      if "$oracle" cloudmesh $args "$graphics" cloud_case.bin "$stereo" && beta --cloudcheck $args "$graphics" "$out/cloud_case.bin" "$stereo"; then :; else fail=1; fi
    done
  done
done
rm -f "$out/cloud_case.bin"

echo "== sky GL colors and sunrise vertex colors, anaglyph on/off"
if "$oracle" skydraw sky_draw_case.bin && beta --skydrawcheck "$out/sky_draw_case.bin"; then :; else fail=1; fi
rm -f "$out/sky_draw_case.bin"

# ---- the statistics file: StatFileWriter's JSON and MD5 checksum, both directions, plus a
# tampered file and a stat id the original doesn't register.
echo "== statistics file"
sf="$out/statsfile"; rm -rf "$sf"; mkdir -p "$sf"
stats_rows() { grep -E '^(-?[0-9]+ -?[0-9]+|null)$' || true; }
pushd "$sf" >/dev/null
beta --statsfile write rust "1000=3,1001=1,1002=4,1100=72345,2000=123456,2010=42,16777217=1234,16908564=250,16974100=2,5242880=1,5242881=1"
beta --statsfile write odd "1000=1,999999=5"
cp rust/stats/stats_player_unsent.dat tampered.dat
sed -i 's/{"1000":3}/{"1000":4}/' tampered.dat
sf_ok=1
for f in rust/stats/stats_player_unsent.dat odd/stats/stats_player_unsent.dat tampered.dat; do
  j=$("$oracle" statsfile "statsfile/$f" "statsfile/$f.java" | stats_rows)  # oracle.sh runs in $out
  r=$(beta --statsfile read "$f" | stats_rows)
  [ "$j" = "$r" ] || { echo "stats file $f: java [$j] rust [$r]"; sf_ok=0; }
  if [ -f "$f.java" ]; then
    j2=$("$oracle" statsfile "statsfile/$f.java" "statsfile/$f.java2" | stats_rows)
    r2=$(beta --statsfile read "$f.java" | stats_rows)
    [ "$j2" = "$r2" ] && [ "$j2" = "$j" ] || { echo "stats file $f.java: java [$j2] rust [$r2]"; sf_ok=0; }
  fi
done
popd >/dev/null
[ "$(printf '%s' "$j" | head -1)" = "null" ] || { echo "tampered stats file was accepted"; sf_ok=0; }
[ $sf_ok -eq 1 ] && echo "STATS FILE EXACT: written and read by both, checksum rejection and unknown ids" || fail=1
rm -rf "$sf"

# ---- converting a pre-McRegion (Alpha) save: the original's SaveConverterMcRegion and the port's
# on two copies of the same world; progress steps, level.dat and every chunk's data must match.
echo "== world conversion from the old chunk-file format"
cv="$out/convert"; rm -rf "$cv"; mkdir -p "$cv"
"$cmp" makesave 42 1 "$cv/src" >/dev/null
python "$root/tools/oracle181/make_alpha_world.py" "$cv/src" "$cv/javaw" >/dev/null
python "$root/tools/oracle181/make_alpha_world.py" "$cv/src" "$cv/rustw" >/dev/null
j=$("$oracle" convertworld "$(cygpath -w "$cv")" javaw | grep -E '^(old|progress) ')
r=$("$cmp" convertworld "$cv" rustw | grep -E '^(old|progress) ')
if c=$(python "$root/tools/oracle181/compare_regions.py" "$cv/javaw" "$cv/rustw") && [ "$j" = "$r" ] \
   && ! ls "$cv/rustw" | grep -qE '^[0-9a-z]{1,2}$' && [ ! -e "$cv/rustw/DIM-1" ]; then
  echo "CONVERSION EXACT: $(echo "$c" | head -1), $(echo "$r" | grep -c progress) progress steps"
else
  echo "conversion differs: $c"; [ "$j" = "$r" ] || echo "progress: java [$(echo $j)] rust [$(echo $r)]"; fail=1
fi
rm -rf "$cv"

bash "$root/tools/verify_decorations181.sh" || fail=1

# --- render-world (b1.8.1) ---
echo "== lightmap texels (EntityRenderer.func_35808_e): day, night, rain, thunder, flashes, flicker, brightness"
if "$oracle" lightmap lightmap_case.bin >/dev/null 2>&1 && beta --lightmapcheck "$out/lightmap_case.bin" | tail -1 | grep -q " 0 differences"; then echo "LIGHTMAP EXACT"; else echo "lightmap DIFFERS"; fail=1; fi
# --- end render-world ---
# --- world-blocks (b1.8.1) ---
bash "$root/tools/verify_world_blocks181.sh" --built || fail=1
# --- end world-blocks ---
# --- terrain (b1.8.1) ---
bash "$root/tools/verify_terrain181.sh" --built || fail=1
bash "$root/tools/verify_terrain2_181.sh" --built || fail=1
bash "$root/tools/verify_terrain3_181.sh" --built || fail=1
# --- end terrain ---
# --- entities (b1.8.1) ---
bash "$root/tools/verify_entities181.sh" || fail=1
# --- end entities ---
# --- player-gui (b1.8.1) ---
echo "== player/gui: hunger, experience, FOV, creative catalog, options, keys and pixels"
bash "$root/tools/verify_player_gui181.sh" || fail=1
# --- end player-gui ---
# --- entities (b1.8.1) ---
bash "$root/tools/verify_entities3_181.sh" --built || fail=1
# --- end entities ---
# --- entities (b1.8.1) ---
bash "$root/tools/verify_entities4_181.sh" --built || fail=1
# --- end entities ---
# --- entities (b1.8.1) ---
bash "$root/tools/verify_entities5_181.sh" --built || fail=1
# --- end entities ---
# --- player-gui (b1.8.1), round 7 ---
bash tools/verify_player_gui7_181.sh --built || fail=1
# --- end player-gui ---
[ $fail -eq 0 ] && echo "ALL CASES EXACT" || echo "SOME CASES DIFFER"
exit $fail
