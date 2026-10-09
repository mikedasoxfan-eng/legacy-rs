#!/usr/bin/env bash
# StatFileWriter, StatsSyncher and original receive/send threads; disk maps are
# normalized by stat id because Java StatBase keys have VM identity hash codes.
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r11" "$root/tools/oracle11/out"
source "$root/tools/r11/verify_common10.sh"
cd "$root"
lock="$root/target/r11/verify11.lock"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
  mkdir -p "$root/target/r11"
  mkdir "$lock" || { echo "another 1.0 verify is running" >&2; exit 3; }
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY11_LOCKED=1
fi
if [ "${1:-}" != --built ]; then cargo build --release -q -p client; fi
echo "== statsync10: receive, send and file writer"
fixture=$(mktemp -d "$root/target/r11/statsync181.XXXXXX")
bash tools/oracle11/oracle.sh statsync181 "$(cygpath -m "$fixture")" > "$fixture/java.log" 2>&1
r10_client target/release/beta --mc 1.1 --statsynccheck "$(cygpath -m "$fixture")" > "$fixture/rust.log" 2>&1 || { tail -5 "$fixture/rust.log"; python "$root/tools/r11/diagnose_text10.py" "$fixture/expected.txt" "$fixture/actual.txt"; exit 1; }
tail -1 "$fixture/rust.log"
echo "Statistics fixtures and traces: $fixture"
