#!/usr/bin/env bash
# Original GUI bytecode stays ahead of the merged base's compiled classes.
root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$root"
mkdir -p target/r11gui3 tools/oracle11/out
status=0
if [ "${1:-}" != --built ]; then cargo build --release || exit 1; fi
if ! bash tools/oracle11/oracle.sh raw 0 0 0 0 0 r10gui3-base.bin >target/r11gui3/base-oracle.log 2>&1; then
    cat target/r11gui3/base-oracle.log
    exit 1
fi
for check in enchanting containers creative slots recipes enchantmentdraw winflow dynamic containerdraw lighting hud creativegui lifecycle endcopy progress enchantclick; do
    echo "== r10gui3 $check original bytecode"
    python tools/r11/gui3_oracle.py "gui3${check}10" "target/r11gui3/$check.bin" || { status=1; continue; }
    target/release/beta --mc 1.1 --r10gui3check "$check" "target/r11gui3/$check.bin" || status=1
done
# Earlier fixtures cover unchanged font behavior and animation/parser dependencies.
for check in enchantment win font boss registry brewing; do
    echo "== r10gui3 regression $check original bytecode"
    python tools/r11/gui2_oracle.py "gui2${check}10" "target/r11gui3/gui2-$check.bin" || { status=1; continue; }
    target/release/beta --mc 1.1 --r10guicheck "$check" "target/r11gui3/gui2-$check.bin" || status=1
done
exit "$status"
