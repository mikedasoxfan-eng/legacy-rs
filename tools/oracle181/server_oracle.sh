#!/usr/bin/env bash
# Build (if needed) and run the server oracle against Mojang's original b1.7.3 server bytecode.
# Usage: tools/oracle/server_oracle.sh serverwalk <seed> <wrand> <radius> <dir> <ticks> <out>
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd -W)"  # -W: Windows-style path for java/javac
jar="$here/../retromcp-b181/minecraft_server/jars/deobfuscated.jar"
[ -f "$jar" ] || { echo "missing $jar - run RetroMCP setup+decompile for b1.7.3 first" >&2; exit 1; }
src="$here/server/src/net/minecraft/src/ServerOracle.java"
build="$here/server/build"
mkdir -p "$build" "$here/out"
if [ ! -f "$build/net/minecraft/src/ServerOracle.class" ] || [ "$src" -nt "$build/net/minecraft/src/ServerOracle.class" ]; then
  javac -nowarn -cp "$jar" -d "$build" "$src" 2>&1 | grep -v -i "warning\|unsafe\|^\s*\^\|^ *sun.misc" || true
fi
java=java
for jl in "$LOCALAPPDATA"/Packages/Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe \
          "$APPDATA"/.minecraft/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe; do
  [ -x "$jl" ] && { java="$jl"; break; }
done
cd "$here/out"
"$java" -cp "$build;$jar" net.minecraft.src.ServerOracle "$@"
