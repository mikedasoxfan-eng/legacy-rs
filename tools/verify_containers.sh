#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
python tools/oracle/container_cases.py
tools/oracle/oracle.sh containerreplay "$(cygpath -w "$root/target/container-cases.txt")" container-replay.bin
cargo run --release -q -p worldgen --bin container_compare -- target/container-cases.txt tools/oracle/out/container-replay.bin
