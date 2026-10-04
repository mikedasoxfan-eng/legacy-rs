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
  rm -f "$out/$f"
}
run_extra() { # like run, but the oracle takes an argument after the output file
  local label="$1" extra="$2"; shift 2
  local oargs=() cargs=()
  while [ "$1" != "--" ]; do oargs+=("$1"); shift; done; shift
  cargs=("$@")
  local f="case_$(echo "${oargs[*]}_$extra" | tr ' ' '_').bin"
  echo "== $label"
  if "$oracle" "${oargs[@]}" "$f" "$extra" >/dev/null && "$cmp" "${cargs[@]}" "$out/$f" "$extra"; then :; else fail=1; fi
  rm -f "$out/$f"
}

echo "== MathHelper sine table"
"$oracle" sin sin.bin && "$cmp" sin "$out/sin.bin" || fail=1

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

# ---- sky, fog and cloud colors, sun angle, night darkness
echo "== sky colors"
for args in "3257840388504953787 -39 65 6000 0.0" "0 1000 -2000 13000 0.5" "123 -500 77 23500 0.25" "42 99999 -12345 18000 0.9" "1 0 0 12500 0.75"; do
  j=$("$oracle" skycolor $args); r=$("$beta" --skycolor $args)
  [ "$j" = "$r" ] || { echo "sky differs for $args"; echo " java $j"; echo " rust $r"; fail=1; }
done
[ $fail -eq 0 ] && echo "sky colors exact"

[ $fail -eq 0 ] && echo "ALL CASES EXACT" || echo "SOME CASES DIFFER"
exit $fail
