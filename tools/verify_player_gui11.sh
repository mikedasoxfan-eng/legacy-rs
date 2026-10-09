#!/usr/bin/env bash
# Focused player/gui checks; safe to invoke independently or from verify11.sh.
set -uo pipefail
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p "$root/target/r11" "$root/tools/oracle11/out"
source "$root/tools/r11/verify_common10.sh"
lock="$root/target/r11/verify11.lock"
if [ -z "${VERIFY11_LOCKED:-}" ]; then
  mkdir -p "$root/target/r11"
  if ! mkdir "$lock" 2>/dev/null; then echo "another 1.0 verify is running ($lock)" >&2; exit 3; fi
  trap 'rmdir "$lock" 2>/dev/null' EXIT
  export VERIFY11_LOCKED=1
fi
cd "$root"
cargo build --release -q -p worldgen11 -p client || exit 1
# The world-list scenes need the fixture saves (a fresh tree has none).
python tools/oracle11/gui/make_saves.py "$root/tools/oracle11/out/guisaves" >/dev/null || exit 1
fail=0
echo "== player-gui 1.0 round 6: creative initialization and controller update policy"
bash tools/oracle11/oracle.sh controllermode181 controllermode181-java.bin && r10_compare target/release/controllermode11 target/r11/controllermode181-rust.bin tools/oracle11/out/controllermode181-java.bin || fail=1
echo "== player-gui 1.0 round 6: SP action state, FOV and sprint expiration"
bash tools/oracle11/oracle.sh playersp181 playersp181-java.bin && r10_compare target/release/playersp11 target/r11/playersp181-rust.bin tools/oracle11/out/playersp181-java.bin || fail=1
echo "== player-gui 1.0 round 6: actual GUI container close callbacks"
bash tools/oracle11/oracle.sh guiclose181 guiclose181-java.bin && r10_client target/release/beta --mc 1.1 --guiclosecheck tools/oracle11/out/guiclose181-java.bin || fail=1
echo "== player-gui 1.0 round 6: world-side GUI chest close hooks"
bash tools/oracle11/oracle.sh guichest181 guichest181-java.bin && r10_compare target/release/guichest11 target/r11/guichest181-rust.bin tools/oracle11/out/guichest181-java.bin || fail=1
echo "== player-gui 1.0 round 5: inventory/NBT and player callbacks"
bash tools/oracle11/oracle.sh playerstate181 playerstate181-java.bin && r10_compare target/release/playerstate11 target/r11/playerstate181-rust.bin tools/oracle11/out/playerstate181-java.bin || fail=1
echo "== player-gui 1.0 round 5: queued click/use transitions"
bash tools/oracle11/oracle.sh controllerinput181 controllerinput181-java.bin && r10_compare target/release/controllerinput11 target/r11/controllerinput181-rust.bin tools/oracle11/out/controllerinput181-java.bin || fail=1
echo "== player-gui 1.0 round 5: production create-world callbacks"
bash tools/oracle11/oracle.sh worldcallbacks181 worldcallbacks181-java.bin && r10_client target/release/beta --mc 1.1 --worldcallbackcheck tools/oracle11/out/worldcallbacks181-java.bin || fail=1
echo "== player-gui 1.0 round 5: inventory interfaces and base slots"
bash tools/oracle11/oracle.sh inventoryinterfaces181 inventoryinterfaces181-java.bin && r10_compare target/release/inventoryinterfaces11 target/r11/inventoryinterfaces181-rust.bin tools/oracle11/out/inventoryinterfaces181-java.bin || fail=1
echo "== player-gui 1.0 round 4: controllers, picking and creative reach"
bash tools/oracle11/oracle.sh controller181 controller181-java.bin && r10_compare target/release/controller11 target/r11/controller181-rust.bin tools/oracle11/out/controller181-java.bin || fail=1
echo "== player-gui 1.0 round 4: chest container lifecycle"
bash tools/oracle11/oracle.sh chesthooks181 chesthooks181-java.bin && r10_compare target/release/chesthooks11 target/r11/chesthooks181-rust.bin tools/oracle11/out/chesthooks181-java.bin || fail=1
echo "== player-gui 1.0 round 4: global key registry"
bash tools/oracle11/oracle.sh keyregistry181 keyregistry181-java.txt && r10_client target/release/beta --mc 1.1 --keyregistrycheck tools/oracle11/out/keyregistry181-java.txt || fail=1
echo "== player-gui 1.0 round 4: statistics comparators"
bash tools/oracle11/oracle.sh statsorder181 statsorder181-java.bin && r10_client target/release/beta --mc 1.1 --statsordercheck tools/oracle11/out/statsorder181-java.bin || fail=1
echo "== player-gui 1.0 round 4: create-world seeds"
bash tools/oracle11/oracle.sh worldseed181 worldseed181-java.bin && r10_client target/release/beta --mc 1.1 --worldseedcheck tools/oracle11/out/worldseed181-java.bin || fail=1
echo "== player-gui 1.0: whole-player ticks"
bash tools/verify_playertick11.sh --built || fail=1
echo "== player-gui 1.0: statistics synchronization"
bash tools/verify_statsync11.sh --built || fail=1
echo "== player-gui 1.0: settings before initial terrain and save"
cargo test --release -q -p worldgen11 new_settings_reach_preload_and_first_save_existing_settings_win --lib || fail=1
echo "== player-gui 1.0: melee and potion mining"
bash tools/oracle11/oracle.sh playercombat181 playercombat181-java.bin && r10_compare target/release/playercombat11 target/r11/playercombat181-rust.bin tools/oracle11/out/playercombat181-java.bin || fail=1
echo "== player-gui 1.0: scaling, buttons and font measurements"
bash tools/oracle11/oracle.sh guiutility181 guiutility181-java.bin && r10_client target/release/beta --mc 1.1 --guiutilitycheck tools/oracle11/out/guiutility181-java.bin || fail=1
echo "== player-gui 1.0: PlayerList hash table"
bash tools/oracle11/oracle.sh playerlist181 playerlist181-java.bin && r10_compare target/release/playerlist11 target/r11/playerlist181-rust.bin tools/oracle11/out/playerlist181-java.bin || fail=1
for mode in playercore creative; do
  echo "== player-gui 1.0: $mode"
  bash tools/oracle11/oracle.sh "$mode" "$mode-java.bin" || { fail=1; continue; }
  "target/release/${mode}11" "target/r11/$mode-rust.bin" || { fail=1; continue; }
  python - "$mode" "tools/oracle11/out/$mode-java.bin" "target/r11/$mode-rust.bin" <<'EOF'
from pathlib import Path
import sys
mode,java,rust=sys.argv[1:]
a,b=Path(java).read_bytes(),Path(rust).read_bytes()
if a!=b:
    at=next((i for i,(x,y) in enumerate(zip(a,b)) if x!=y),min(len(a),len(b)))
    print(f"{mode}: DIFFERS at byte {at}, lengths {len(a)}/{len(b)}")
    sys.path.insert(0,str(Path('tools/r11').resolve()))
    from diagnose_dump10 import diagnose
    print('FIRST DIFFERENCE: '+diagnose(Path(java),Path(rust)))
    sys.exit(1)
print(f"{mode}: EXACT ({len(a)} bytes)")
EOF
  [ $? -eq 0 ] || fail=1
done
echo "== player-gui 1.0: options and keybindings"
bash tools/oracle11/oracle.sh options181 options181-java.txt && r10_client target/release/beta --mc 1.1 --playerguicheck tools/oracle11/out/options181-java.txt || fail=1
echo "== player-gui 1.0: original recipe and smelting catalog"
if bash tools/oracle11/oracle.sh recipeaudit181 player-gui3-recipes.bin && cmp -s tools/oracle11/out/player-gui3-recipes.bin crates/worldgen11/src/container/recipes.bin; then
  echo "RECIPE CATALOG EXACT: 161 recipes and 2258 smelting/item metadata rows"
else
  echo "RECIPE CATALOG DIFFERS"; python "$root/tools/r11/diagnose_dump10.py" tools/oracle11/out/player-gui3-recipes.bin crates/worldgen11/src/container/recipes.bin; fail=1
fi
if [ "${1:-}" != --no-gui ]; then
  for scenes in tools/oracle11/gui/player-gui-scenes.txt tools/oracle11/gui/player-gui-container-scenes.txt tools/oracle11/gui/player-gui2-scenes.txt tools/oracle11/gui/player-gui3-scenes.txt tools/oracle11/gui/player-gui4-scenes.txt tools/oracle11/gui/player-gui4-creative-scenes.txt tools/oracle11/gui/player-gui5-scenes.txt tools/oracle11/gui/player-gui5-hud/*.txt tools/oracle11/gui/container-scenes.txt tools/oracle11/gui/player-gui6-scenes.txt; do
  echo "== player-gui 1.0: GUI pixels $(basename "$scenes" .txt)"
  java=tools/oracle11/out/player-gui-java
  rust=target/r11/player-gui-rust
  python - "$scenes" "$java" "$rust" <<'EOF'
from pathlib import Path
import sys
for line in Path(sys.argv[1]).read_text().splitlines():
    line=line.split('#',1)[0].strip()
    if line.startswith('shot '):
        for directory in sys.argv[2:]:
            (Path(directory)/(line.split(maxsplit=1)[1]+'.png')).unlink(missing_ok=True)
EOF
  bash tools/oracle11/guishot.sh "$scenes" "$java" || fail=1
  GUI_MSAA=1 r10_client target/release/beta --mc 1.1 --guishot "$scenes" "$rust" || fail=1
  python tools/oracle11/compare_gui_strict.py "$scenes" "$java" "$rust" target/r11/player-gui-diff || fail=1
  done
fi
exit $fail
