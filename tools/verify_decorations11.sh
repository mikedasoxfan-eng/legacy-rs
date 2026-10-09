#!/usr/bin/env bash
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r11" "$root/tools/oracle11/out"
source "$root/tools/r11/verify_common10.sh"
# One run at a time per tree (two runs would overwrite each other's oracle output).
lock="$root/target/r11/verify11.lock"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
  mkdir -p "$root/target/r11"
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock); remove it if stale" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY11_LOCKED=1
fi
beta() { r10_client "$root/target/release/beta" --mc 1.1 "$@"; }
oracle="$root/tools/oracle11/oracle.sh"
out="$root/tools/oracle11/out"
cargo build --release -q --manifest-path "$root/Cargo.toml" -p worldgen11 -p client || exit 1
fail=0
echo "== decorations10: signs: placement, editing, NBT"
{ r10_oracle "$oracle" signs signs.bin && r10_compare "$root/target/release/compare11" signs "$out/signs.bin"; } || { echo "DIFFERS: decorations181: signs: placement, editing, NBT"; fail=1; }
echo "== decorations10: sign renderer (TileEntitySignRenderer)"
{ r10_oracle "$oracle" signrenderers sign-renderers.bin && beta --signrenderercheck "$out/sign-renderers.bin"; } || { echo "DIFFERS: decorations181: sign renderer (TileEntitySignRenderer)"; fail=1; }
echo "== decorations10: paintings"
{ r10_oracle "$oracle" paintings paintings.bin && r10_compare "$root/target/release/compare11" paintings "$out/paintings.bin"; } || { echo "DIFFERS: decorations181: paintings"; fail=1; }
echo "== decorations10: painting renderer"
{ r10_oracle "$oracle" paintingrenderers painting-renderers.bin && beta --paintingrenderercheck "$out/painting-renderers.bin"; } || { echo "DIFFERS: decorations181: painting renderer"; fail=1; }
echo "== decorations10: maps: palette, terrain, markers, NBT"
{ r10_oracle "$oracle" maps maps.bin && r10_compare "$root/target/release/compare11" maps "$out/maps.bin"; } || { echo "DIFFERS: decorations181: maps: palette, terrain, markers, NBT"; fail=1; }
echo "== decorations10: map files"
{ r10_oracle "$oracle" mapfiles mapfiles.bin && r10_compare "$root/target/release/compare11" mapfiles "$out/mapfiles.bin"; } || { echo "DIFFERS: decorations181: map files"; fail=1; }
echo "== decorations10: map files read back"
{ r10_oracle "$oracle" mapread "$root/target/r11/map-rust-save" mapread.bin && r10_compare "$root/target/release/compare11" mapread "$out/mapfiles.bin" "$out/mapread.bin"; } || { echo "DIFFERS: decorations181: map files read back"; fail=1; }
echo "== decorations10: fishing"
{ r10_oracle "$oracle" fishing fishing.bin && r10_compare "$root/target/release/compare11" fishing "$out/fishing.bin"; } || { echo "DIFFERS: decorations181: fishing"; fail=1; }
echo "== decorations10: fishing line renderer"
{ r10_oracle "$oracle" fishrenderers fish-renderers.bin && beta --fishrenderercheck "$out/fish-renderers.bin"; } || { echo "DIFFERS: decorations181: fishing line renderer"; fail=1; }
[ $fail -eq 0 ] && echo "DECORATIONS EXACT"
exit $fail
