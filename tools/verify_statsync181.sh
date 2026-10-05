#!/usr/bin/env bash
# StatFileWriter, StatsSyncher and original receive/send threads; disk maps are
# normalized by stat id because Java StatBase keys have VM identity hash codes.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p "$root/target"
  mkdir "$lock" || { echo "another b1.8.1 verify is running" >&2; exit 3; }
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p client; fi
fixture=$(mktemp -d "$root/target/statsync181.XXXXXX")
bash tools/oracle181/oracle.sh statsync181 "$(cygpath -m "$fixture")" > "$fixture/java.log" 2>&1
target/release/beta --mc b1.8.1 --statsynccheck "$(cygpath -m "$fixture")" > "$fixture/rust.log" 2>&1 || { tail -5 "$fixture/rust.log"; exit 1; }
tail -1 "$fixture/rust.log"
echo "Statistics fixtures and traces: $fixture"
