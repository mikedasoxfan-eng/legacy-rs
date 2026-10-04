#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
export JAVA_TOOL_OPTIONS="${JAVA_TOOL_OPTIONS:-} -Xmx512m"
python tools/oracle/container_cases.py
tools/oracle/oracle.sh containerreplay "$(cygpath -w "$root/target/container-cases.txt")" container-replay.bin
cargo run --release -q -p worldgen --bin container_compare -- target/container-cases.txt tools/oracle/out/container-replay.bin
tools/oracle/oracle.sh containertiles 1000 container-tiles.bin
cargo run --release -q -p worldgen --bin container_compare -- tiles 1000 tools/oracle/out/container-tiles.bin
