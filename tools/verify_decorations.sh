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
"$oracle" paintings paintings.bin
"$root/target/release/compare" paintings "$out/paintings.bin"
"$oracle" paintingrenderers painting-renderers.bin
"$root/target/release/beta" --paintingrenderercheck "$out/painting-renderers.bin"
"$oracle" maps maps.bin
"$root/target/release/compare" maps "$out/maps.bin"
"$oracle" mapfiles mapfiles.bin
"$root/target/release/compare" mapfiles "$out/mapfiles.bin"
"$oracle" mapread "$root/target/map-rust-save" mapread.bin
"$root/target/release/compare" mapread "$out/mapfiles.bin" "$out/mapread.bin"
"$oracle" fishing fishing.bin
"$root/target/release/compare" fishing "$out/fishing.bin"
"$oracle" fishrenderers fish-renderers.bin
"$root/target/release/beta" --fishrenderercheck "$out/fish-renderers.bin"
echo "DECORATIONS EXACT"
