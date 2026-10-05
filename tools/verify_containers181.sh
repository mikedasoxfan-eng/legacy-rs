#!/usr/bin/env bash
set -euo pipefail
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
cd "$root"
export JAVA_TOOL_OPTIONS="${JAVA_TOOL_OPTIONS:-} -Xmx512m"
python tools/oracle181/container_cases.py
tools/oracle181/oracle.sh containerreplay "$(cygpath -w "$root/target/container-cases.txt")" container-replay.bin
cargo run --release -q -p worldgen181 --bin container_compare181 -- target/container-cases.txt tools/oracle181/out/container-replay.bin
tools/oracle181/oracle.sh containertiles 1000 container-tiles.bin
cargo run --release -q -p worldgen181 --bin container_compare181 -- tiles 1000 tools/oracle181/out/container-tiles.bin
