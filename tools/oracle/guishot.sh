#!/usr/bin/env bash
# Capture GUI scenes from the original b1.7.3 client (opens a real game window briefly).
# Usage: tools/oracle/guishot.sh [scenes.txt] [outdir]
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd -W)"
jar="$here/../retromcp/minecraft/jars/deobfuscated.jar"
natives="$(cygpath -w "$here/../retromcp/libraries/natives")"
libs=$(find "$here/../retromcp/libraries" -name '*.jar' ! -name '*sources*' ! -name '*natives*' | while read -r f; do printf ';%s' "$(cygpath -w "$f")"; done)
scenes="${1:-$here/gui/scenes.txt}"
outdir="${2:-$here/out/gui_java}"
build="$here/gui/build"
mkdir -p "$build/src" "$here/out"

# GuiMainMenu animates the splash from System.currentTimeMillis(): recompile it reading
# GuiShot.millis instead (the only change; javap-compare it if in doubt).
orig="$here/../retromcp/minecraft/src/net/minecraft/src/GuiMainMenu.java"
sed 's/System\.currentTimeMillis()/GuiShot.millis/' "$orig" > "$build/src/GuiMainMenu.java"
if [ ! -f "$build/net/minecraft/src/GuiShot.class" ] || [ "$here/gui/GuiShot.java" -nt "$build/net/minecraft/src/GuiShot.class" ]; then
  javac -nowarn -cp "$jar$libs" -d "$build" "$here/gui/GuiShot.java" "$build/src/GuiMainMenu.java" 2>&1 | grep -v -i "^Note:" || true
fi

# A throwaway .minecraft: default options, the fixture saves.
data="$here/out/guimc"
python - "$data" <<'EOF'
import shutil, sys
shutil.rmtree(sys.argv[1], ignore_errors=True)
EOF
mkdir -p "$data"
python "$here/gui/make_saves.py" "$data/saves"

java=java
for jl in "$LOCALAPPDATA"/Packages/Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe \
          "$APPDATA"/.minecraft/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe; do
  [ -x "$jl" ] && { java="$jl"; break; }
done
"$java" -Djava.library.path="$natives" -cp "$build;$jar$libs" net.minecraft.src.GuiShot "$(cygpath -w "$scenes")" "$(cygpath -w "$outdir")" "$(cygpath -w "$data")"
