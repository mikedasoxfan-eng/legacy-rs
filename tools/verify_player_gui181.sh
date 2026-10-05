#!/usr/bin/env bash
# Focused player/gui checks; safe to invoke independently or from verify181.sh.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
lock="$root/target/verify181.lock"
if [ -z "${VERIFY181_LOCKED:-}" ]; then
  mkdir -p "$root/target"
  if ! mkdir "$lock" 2>/dev/null; then echo "another b1.8.1 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY181_LOCKED=1
fi
cd "$root"
cargo build --release -q -p worldgen181 -p client || exit 1
fail=0
echo "== player-gui b1.8.1 round 5: inventory/NBT and player callbacks"
bash tools/oracle181/oracle.sh playerstate181 playerstate181-java.bin && target/release/playerstate181 target/playerstate181-rust.bin tools/oracle181/out/playerstate181-java.bin || fail=1
echo "== player-gui b1.8.1 round 5: queued click/use transitions"
bash tools/oracle181/oracle.sh controllerinput181 controllerinput181-java.bin && target/release/controllerinput181 target/controllerinput181-rust.bin tools/oracle181/out/controllerinput181-java.bin || fail=1
echo "== player-gui b1.8.1 round 5: production create-world callbacks"
bash tools/oracle181/oracle.sh worldcallbacks181 worldcallbacks181-java.bin && target/release/beta --mc b1.8.1 --worldcallbackcheck tools/oracle181/out/worldcallbacks181-java.bin || fail=1
echo "== player-gui b1.8.1 round 5: inventory interfaces and base slots"
bash tools/oracle181/oracle.sh inventoryinterfaces181 inventoryinterfaces181-java.bin && target/release/inventoryinterfaces181 target/inventoryinterfaces181-rust.bin tools/oracle181/out/inventoryinterfaces181-java.bin || fail=1
echo "== player-gui b1.8.1 round 4: controllers, picking and creative reach"
bash tools/oracle181/oracle.sh controller181 controller181-java.bin && target/release/controller181 target/controller181-rust.bin tools/oracle181/out/controller181-java.bin || fail=1
echo "== player-gui b1.8.1 round 4: chest container lifecycle"
bash tools/oracle181/oracle.sh chesthooks181 chesthooks181-java.bin && target/release/chesthooks181 target/chesthooks181-rust.bin tools/oracle181/out/chesthooks181-java.bin || fail=1
echo "== player-gui b1.8.1 round 4: global key registry"
bash tools/oracle181/oracle.sh keyregistry181 keyregistry181-java.txt && target/release/beta --mc b1.8.1 --keyregistrycheck tools/oracle181/out/keyregistry181-java.txt || fail=1
echo "== player-gui b1.8.1 round 4: statistics comparators"
bash tools/oracle181/oracle.sh statsorder181 statsorder181-java.bin && target/release/beta --mc b1.8.1 --statsordercheck tools/oracle181/out/statsorder181-java.bin || fail=1
echo "== player-gui b1.8.1 round 4: create-world seeds"
bash tools/oracle181/oracle.sh worldseed181 worldseed181-java.bin && target/release/beta --mc b1.8.1 --worldseedcheck tools/oracle181/out/worldseed181-java.bin || fail=1
echo "== player-gui b1.8.1: whole-player ticks"
bash tools/verify_playertick181.sh --built || fail=1
echo "== player-gui b1.8.1: statistics synchronization"
bash tools/verify_statsync181.sh --built || fail=1
echo "== player-gui b1.8.1: settings before initial terrain and save"
cargo test --release -q -p worldgen181 new_settings_reach_preload_and_first_save_existing_settings_win --lib || fail=1
echo "== player-gui b1.8.1: melee and potion mining"
bash tools/oracle181/oracle.sh playercombat181 playercombat181-java.bin && target/release/playercombat181 target/playercombat181-rust.bin tools/oracle181/out/playercombat181-java.bin || fail=1
echo "== player-gui b1.8.1: scaling, buttons and font measurements"
bash tools/oracle181/oracle.sh guiutility181 guiutility181-java.bin && target/release/beta --mc b1.8.1 --guiutilitycheck tools/oracle181/out/guiutility181-java.bin || fail=1
echo "== player-gui b1.8.1: PlayerList hash table"
bash tools/oracle181/oracle.sh playerlist181 playerlist181-java.bin && target/release/playerlist181 target/playerlist181-rust.bin tools/oracle181/out/playerlist181-java.bin || fail=1
for mode in playercore creative; do
  echo "== player-gui b1.8.1: $mode"
  bash tools/oracle181/oracle.sh "$mode" "$mode-java.bin" || { fail=1; continue; }
  "target/release/${mode}181" "target/$mode-rust.bin" || { fail=1; continue; }
  python - "$mode" "tools/oracle181/out/$mode-java.bin" "target/$mode-rust.bin" <<'EOF'
from pathlib import Path
import sys
mode,java,rust=sys.argv[1:]
a,b=Path(java).read_bytes(),Path(rust).read_bytes()
if a!=b:
    at=next((i for i,(x,y) in enumerate(zip(a,b)) if x!=y),min(len(a),len(b)))
    print(f"{mode}: DIFFERS at byte {at}, lengths {len(a)}/{len(b)}")
    sys.exit(1)
print(f"{mode}: EXACT ({len(a)} bytes)")
EOF
  [ $? -eq 0 ] || fail=1
done
echo "== player-gui b1.8.1: options and keybindings"
bash tools/oracle181/oracle.sh options181 options181-java.txt && target/release/beta --mc b1.8.1 --playerguicheck tools/oracle181/out/options181-java.txt || fail=1
echo "== player-gui b1.8.1: original recipe and smelting catalog"
if bash tools/oracle181/oracle.sh recipeaudit181 player-gui3-recipes.bin && cmp -s tools/oracle181/out/player-gui3-recipes.bin crates/worldgen181/src/container/recipes.bin; then
  echo "RECIPE CATALOG EXACT: 161 recipes and 2258 smelting/item metadata rows"
else
  echo "RECIPE CATALOG DIFFERS"; fail=1
fi
if [ "${1:-}" != --no-gui ]; then
  for scenes in tools/oracle181/gui/player-gui-scenes.txt tools/oracle181/gui/player-gui-container-scenes.txt tools/oracle181/gui/player-gui2-scenes.txt tools/oracle181/gui/player-gui3-scenes.txt tools/oracle181/gui/player-gui4-scenes.txt tools/oracle181/gui/player-gui4-creative-scenes.txt tools/oracle181/gui/player-gui5-scenes.txt tools/oracle181/gui/player-gui5-hud/*.txt; do
  echo "== player-gui b1.8.1: GUI pixels $(basename "$scenes" .txt)"
  java=tools/oracle181/out/player-gui-java
  rust=target/player-gui-rust
  python - "$scenes" "$java" "$rust" <<'EOF'
from pathlib import Path
import sys
for line in Path(sys.argv[1]).read_text().splitlines():
    line=line.split('#',1)[0].strip()
    if line.startswith('shot '):
        for directory in sys.argv[2:]:
            (Path(directory)/(line.split(maxsplit=1)[1]+'.png')).unlink(missing_ok=True)
EOF
  bash tools/oracle181/guishot.sh "$scenes" "$java" || fail=1
  GUI_MSAA=1 target/release/beta --mc b1.8.1 --guishot "$scenes" "$rust" || fail=1
  python tools/oracle181/compare_gui_strict.py "$scenes" "$java" "$rust" target/player-gui-diff || fail=1
  done
fi
exit $fail
