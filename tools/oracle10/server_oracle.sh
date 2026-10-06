#!/usr/bin/env bash
# Build (if needed) and run the server oracle against Mojang's original 1.0 server bytecode.
# Usage: tools/oracle10/server_oracle.sh serverwalk <seed> <wrand> <radius> <dir> <ticks> <out>
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd -W)"  # -W: Windows-style path for java/javac
jar="$here/../retromcp-r10/minecraft_server/jars/deobfuscated.jar"
[ -f "$jar" ] || { echo "missing $jar - run RetroMCP setup+decompile for 1.0 first" >&2; exit 1; }
src="$here/server/src/net/minecraft/src/ServerOracle.java"
build="$here/server/build"
mkdir -p "$build" "$here/out"
if [ ! -f "$build/net/minecraft/src/ServerRunner10.class" ] || [ "$src" -nt "$build/net/minecraft/src/ServerOracle.class" ] || [ "$here/server/src/net/minecraft/src/ServerRunner10.java" -nt "$build/net/minecraft/src/ServerRunner10.class" ]; then
  javac -nowarn -cp "$jar" -d "$build" "$src" "$here/server/src/net/minecraft/src/ServerRunner10.java"
fi
java=java
for jl in "$LOCALAPPDATA"/Packages/Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe \
          "$APPDATA"/.minecraft/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe; do
  [ -x "$jl" ] && { java="$jl"; break; }
done
[ -n "${ORACLE_JAVA:-}" ] && java="$ORACLE_JAVA"
"$java" -version 2>&1 | grep -q '1.8.0_51' || { echo "oracle requires Mojang Java 8u51" >&2; exit 1; }
cd "$here/out"
"$java" -cp "$build;$jar" net.minecraft.src.ServerRunner10 "$@"
