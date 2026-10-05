#!/usr/bin/env bash
set -euo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
oracle="$root/tools/oracle/oracle.sh"
out="$root/tools/oracle/out"
cargo build --release -q --manifest-path "$root/Cargo.toml" -p worldgen -p client
"$oracle" signs signs.bin
"$root/target/release/compare" signs "$out/signs.bin"
"$oracle" signrenderers sign-renderers.bin
"$root/target/release/beta" --signrenderercheck "$out/sign-renderers.bin"
echo "DECORATIONS EXACT"
