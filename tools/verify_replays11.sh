#!/usr/bin/env bash
# Recheck all player and entity frame streams with the release identity hooks.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
source "$root/tools/r11/verify_common10.sh"
mkdir -p "$root/target/r11" "$root/tools/oracle11/out"
lock="$root/target/r11/verify11.lock"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
  mkdir "$lock" || exit 3
  trap 'rmdir "$lock" 2>/dev/null' EXIT
fi
oracle="$root/tools/oracle11/oracle.sh"
out="$root/tools/oracle11/out"
cmp="$root/target/release/compare11"
fail=0
run_one() { # label, oracle args..., then "--", compare args (oracle output file appended)
  local label="$1"; shift
  local oargs=() cargs=()
  while [ "$1" != "--" ]; do oargs+=("$1"); shift; done; shift
  cargs=("$@")
  local rc=0
  local f="case_$(echo "${oargs[*]}" | tr ' ' '_').bin"
  echo "== $label"
  if ! r10_oracle "$oracle" "${oargs[@]:0:1}" "${oargs[@]:1}" "$f" >"$out/$f.log" 2>&1; then
    cat "$out/$f.log"; echo "ORACLE FAILED: $label"; rc=1
  elif ! r10_compare "$cmp" "${cargs[@]}" "$out/$f"; then
    echo "DIFFERS: $label"; rc=1
  fi
  [ "${VERIFY11_KEEP:-}" = 1 ] || rm -f "$out/$f" "$out/$f.txt" "$out/$f.events" "$out/$f.identities" "$out/$f.spawnidentities" "$out/$f.frames" "$out/$f.log"
  return $rc
}
run_extra_one() { # like run, but the oracle takes an argument after the output file
  local label="$1" extra="$2"; shift 2
  local oargs=() cargs=()
  while [ "$1" != "--" ]; do oargs+=("$1"); shift; done; shift
  cargs=("$@")
  local rc=0
  local f="case_$(echo "${oargs[*]}_$extra" | tr ' :,' '_xc' | cut -c1-80).bin"
  echo "== $label"
  if ! r10_oracle "$oracle" "${oargs[@]}" "$f" "$extra" >"$out/$f.log" 2>&1; then
    cat "$out/$f.log"; echo "ORACLE FAILED: $label"; rc=1
  elif ! r10_compare "$cmp" "${cargs[@]}" "$out/$f" "$extra"; then
    echo "DIFFERS: $label"; rc=1
  fi
  [ "${VERIFY11_KEEP:-}" = 1 ] || rm -f "$out/$f" "$out/$f.txt" "$out/$f.events" "$out/$f.identities" "$out/$f.spawnidentities" "$out/$f.frames" "$out/$f.log"
  return $rc
}
# Replay cases are independent (each has its own output file), so they run in parallel,
# VERIFY11_JOBS at a time (default 6). Their output is printed in order by pflush.
jobs_max="${VERIFY11_JOBS:-6}"
pdir="$root/target/r11/verify_replays10-par"
rm -rf "${pdir:?}"; mkdir -p "$pdir"; pn=0
par() {
  while [ "$(jobs -rp | wc -l)" -ge "$jobs_max" ]; do wait -n; done
  pn=$((pn + 1)); local o="$pdir/$(printf %05d "$pn")"
  ( "$@" >"$o.out" 2>&1; echo $? >"$o.rc" ) &
}
pflush() {
  wait
  local o
  for o in "$pdir"/*.out; do
    [ -e "$o" ] || continue
    cat "$o"; [ "$(cat "${o%.out}.rc")" = 0 ] || fail=1
    [ "${VERIFY11_KEEP:-}" = 1 ] || rm -f "$o" "${o%.out}.rc"
  done
}
run() { par run_one "$@"; }
run_extra() { par run_extra_one "$@"; }

for player_case in "movement 1000" "damage 1000" "pickup 1000" "sleep 300" "portal 300" "nbt 300" "look 1000" "dig 1000" "place 360" "use 300" "shoot 300" "interact 210" "fertilize 50" "activate 100" "api 200" "pipeline 100" "night 150" "sleepworld 150" "survival 800" "bucket 100" "combat 400" "pickentities 200" "digextras 1600" "slimecontact 360" "stats 400"; do
  read -r kind ticks <<<"$player_case"
  run "player: $kind, every frame and final world" playerintegration181 "$kind" "$ticks" -- player "$kind" "$ticks"
done


for entity_case in "items 6005" "sand 300" "tnt 300" "arrows 1300" "passive 13000" "aquatic 3000" "hostile 3000" "special 3000" "spawning 3000" "spawner 3000" "client 3000" "nbt 1000" "nbtall 1000" "slimes 3000" "hooks 1000" "combat 3000" "stormclient 3000" "lightning 1000" "rider 3000" "nether 3000"; do
  read -r kind ticks <<<"$entity_case"
  run "entities: $kind, every frame for $ticks ticks" entityintegration181 "$kind" "$ticks" -- entities "$kind" "$ticks"
done


pflush
exit "$fail"
