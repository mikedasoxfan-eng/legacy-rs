#!/usr/bin/env bash
# Exact original Release bytecode comparisons; continue after individual failures.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p target/r11/entities3
fail=0
if [ "${1:-}" != --built ]; then
  cargo build --release -p worldgen11 --bin r10entities3_11 || exit 1
else
  shift
fi
selected=("$@")
run() {
  local kind="$1" match=0 candidate
  if [ "${#selected[@]}" -gt 0 ]; then
    for candidate in "${selected[@]}"; do [ "$candidate" = "$kind" ] && match=1; done
    [ "$match" -eq 1 ] || return 0
  fi
  echo "== entities3: $kind"
  local output="$root/target/r11/entities3/$kind.bin"
  if bash tools/oracle11/oracle.sh r10entities3 "$kind" "$(cygpath -w "$output")" >"$output.log" 2>&1; then
    target/release/r10entities3_11 "$kind" "$output" || fail=1
  else
    cat "$output.log"
    echo "ORACLE FAILED: $kind"
    fail=1
  fi
}
for kind in player-copy-alias player-respawn-copy playersp-end-events player-end-travel playersp-magic-request pathfinder-obstacles; do run "$kind"; done
for mob in Creeper Ghast Giant LavaSlime PigZombie Skeleton SnowMan Villager; do run "fall-$mob"; done
# Re-run existing focused bytecode fixtures under explicit class-specific sections.
for kind in playersp playerxp inventorycopy sleep player-damage player-flight player-bow player-eat373 player-regen player-starve; do run "covered-$kind"; done
for kind in replay-Arrow vehicle-boats round4-creeper4 round4-creeperdeath4 replay-Ghast round4-retaliation4 replay-Giant mob-LavaSlime replay-PigZombie branches-skeletonbranches round5-skeletondeath5 mob-SnowMan mob-Villager; do run "covered-$kind"; done
exit "$fail"
