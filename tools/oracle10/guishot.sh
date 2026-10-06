#!/usr/bin/env bash
# Capture GUI scenes from the original Release 1.0 client (opens a real game window briefly).
# Usage: tools/oracle10/guishot.sh [scenes.txt] [outdir]
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd -W)"
jar="$here/../retromcp-r10/minecraft/jars/deobfuscated.jar"
natives="$(cygpath -w "$here/../retromcp-r10/libraries/natives")"
libs=$(find "$here/../retromcp-r10/libraries" -name '*.jar' ! -name '*sources*' ! -name '*natives*' | while read -r f; do printf ';%s' "$(cygpath -w "$f")"; done)
scenes="${1:-$here/gui/scenes.txt}"
outdir="${2:-$here/out/gui_java}"
build="$here/gui/build"
mkdir -p "$build/src" "$here/out"

# GuiMainMenu animates the splash from System.currentTimeMillis(): recompile it reading
# GuiShot.millis instead (the only change; javap-compare it if in doubt).
orig="$here/../retromcp-r10/minecraft/src/net/minecraft/src/GuiMainMenu.java"
sed 's/System\.currentTimeMillis()/GuiShot.millis/' "$orig" > "$build/src/GuiMainMenu.java"
sed 's/System\.currentTimeMillis()/GuiShot.millis/g' "${orig%GuiMainMenu.java}GuiAchievements.java" > "$build/src/GuiAchievements.java"
sed 's/System\.currentTimeMillis()/GuiShot.millis/g' "${orig%GuiMainMenu.java}GuiAchievement.java" > "$build/src/GuiAchievement.java"
# Compile the clock-patched 1.0 classes too, even after compile_check built GuiShot.
if true; then
  javac -nowarn -cp "$jar$libs" -d "$build" "$here/gui/GuiShot.java" "$here/gui/GuiLightmapFixture.java" "$here/gui/GuiPanoramaProbe10.java" "$build/src/GuiMainMenu.java" "$build/src/GuiAchievements.java" "$build/src/GuiAchievement.java"
fi

# A throwaway .minecraft: default options, the fixture saves.
data="$here/out/guimc"
python - "$data" "$here/out" <<'EOF'
import shutil, sys
from pathlib import Path
target = Path(sys.argv[1]).resolve()
output = Path(sys.argv[2]).resolve()
if target.parent != output or target.name != 'guimc':
    raise SystemExit('refusing cleanup outside the oracle10 GUI output directory')
shutil.rmtree(target, ignore_errors=True)
EOF
mkdir -p "$data"
python "$here/gui/make_saves.py" "$data/saves"

java=java
for jl in "$LOCALAPPDATA"/Packages/Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe \
          "$APPDATA"/.minecraft/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe; do
  [ -x "$jl" ] && { java="$jl"; break; }
done
[ -n "${ORACLE_JAVA:-}" ] && java="$ORACLE_JAVA"
"$java" -version 2>&1 | grep -q '1.8.0_51' || { echo "oracle requires Mojang Java 8u51; refusing another JVM" >&2; exit 1; }
"$java" -Djava.library.path="$natives" -cp "$build;$jar$libs" net.minecraft.src.GuiShot "$(cygpath -w "$scenes")" "$(cygpath -w "$outdir")" "$(cygpath -w "$data")"
