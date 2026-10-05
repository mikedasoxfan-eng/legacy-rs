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
oracle="$root/tools/oracle181/oracle.sh"
out="$root/tools/oracle181/out"
cargo build --release -q --manifest-path "$root/Cargo.toml" -p worldgen181 -p client
"$oracle" signs signs.bin
"$root/target/release/compare181" signs "$out/signs.bin"
"$oracle" signrenderers sign-renderers.bin
"$root/target/release/beta" --signrenderercheck "$out/sign-renderers.bin"
"$oracle" paintings paintings.bin
"$root/target/release/compare181" paintings "$out/paintings.bin"
"$oracle" paintingrenderers painting-renderers.bin
"$root/target/release/beta" --paintingrenderercheck "$out/painting-renderers.bin"
"$oracle" maps maps.bin
"$root/target/release/compare181" maps "$out/maps.bin"
"$oracle" mapfiles mapfiles.bin
"$root/target/release/compare181" mapfiles "$out/mapfiles.bin"
"$oracle" mapread "$root/target/map-rust-save" mapread.bin
"$root/target/release/compare181" mapread "$out/mapfiles.bin" "$out/mapread.bin"
"$oracle" fishing fishing.bin
"$root/target/release/compare181" fishing "$out/fishing.bin"
"$oracle" fishrenderers fish-renderers.bin
"$root/target/release/beta" --fishrenderercheck "$out/fish-renderers.bin"
echo "DECORATIONS EXACT"
