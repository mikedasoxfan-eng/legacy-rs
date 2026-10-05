#!/usr/bin/env bash
# Everything a release must pass: both versions, simulation, renderer inputs and screens.
# Usage: tools/verify_release.sh            logs in target/release-verify/, exit 0 only if all pass
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
logs="$root/target/release-verify"
mkdir -p "$logs"
cd "$root"
fail=0
step() { # name, command...
  local name="$1"; shift
  local start=$SECONDS
  if "$@" >"$logs/$name.log" 2>&1; then r=PASS; else r=FAIL; fail=1; fi
  printf '%-28s %s  (%d min)\n' "$name" "$r" $(( (SECONDS - start) / 60 ))
}
# Beta 1.8.1: the full suite (focused suites included), then the parts that open windows.
step b181-verify        bash tools/verify181.sh
step b181-render        bash tools/verify_render181.sh
step b181-gui           bash tools/verify_gui181.sh
step b181-ingame        bash tools/verify_ingame181.sh
# Beta 1.7.3 must stay exact.
step b173-verify        bash tools/verify.sh
step b173-gui           bash tools/verify_gui.sh
[ $fail -eq 0 ] && echo "RELEASE CHECKS PASS" || echo "RELEASE CHECKS FAIL (logs in $logs)"
exit $fail
