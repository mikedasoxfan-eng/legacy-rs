#!/usr/bin/env bash
# Optional visual oracle using the actual RenderLiving subclasses in an offscreen GL pbuffer.
# Usage: beta --modelshot shots/models.png --dump-scene shots/scene.txt
#        tools/oracle/modelshot.sh shots/scene.txt shots/models-java.png
set -euo pipefail
root="$(cd "$(dirname "$0")/../.." && pwd)"
here="$(cygpath -w "$root/tools/oracle")"
jar="$here/../retromcp/minecraft/jars/deobfuscated.jar"
libs=$(find "$root/tools/retromcp/libraries" -name '*.jar' ! -name '*sources*' ! -name '*natives*' | while read -r f; do printf ';%s' "$(cygpath -w "$f")"; done)
mkdir -p "$root/tools/oracle/build"
javac -nowarn -cp "$jar$libs" -d "$here/build" "$here/src/net/minecraft/src/ModelShot.java"
java=java
for jl in "$LOCALAPPDATA"/Packages/Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe "$APPDATA"/.minecraft/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe; do
  [ -x "$jl" ] && { java="$jl"; break; }
done
[ -n "${ORACLE_JAVA:-}" ] && java="$ORACLE_JAVA"
"$java" "-Dorg.lwjgl.librarypath=$(cygpath -w "$root/tools/retromcp/libraries/natives")" -cp "$here/build;$jar$libs" net.minecraft.src.ModelShot "$(cygpath -w "$1")" "$(cygpath -w "$2")"
