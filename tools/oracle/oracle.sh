#!/usr/bin/env bash
# Build (if needed) and run the Java oracle against Mojang's original b1.7.3 bytecode.
# Usage: tools/oracle/oracle.sh <mode> [args...]   (see Oracle.java)
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd -W)"  # -W: Windows-style path for java/javac
jar="$here/../retromcp/minecraft/jars/deobfuscated.jar"
[ -f "$jar" ] || { echo "missing $jar - run RetroMCP setup+decompile for b1.7.3 first" >&2; exit 1; }
mkdir -p "$here/build" "$here/out"
if [ ! -f "$here/build/net/minecraft/src/Oracle.class" ] || [ "$here/src/net/minecraft/src/Oracle.java" -nt "$here/build/net/minecraft/src/Oracle.class" ]; then
  javac -nowarn -cp "$jar" -d "$here/build" "$here/src/net/minecraft/src/Oracle.java"
fi
# Prefer the JRE Mojang's launcher runs b1.7.3 on (8u51): Math.sin & co. differ
# between Java builds, and that is the one vanilla actually uses.
java=java
for jl in "$LOCALAPPDATA"/Packages/Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe           "$APPDATA"/.minecraft/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe; do
  [ -x "$jl" ] && { java="$jl"; break; }
done
[ -n "${ORACLE_JAVA:-}" ] && java="$ORACLE_JAVA"
cd "$here/out"
"$java" -cp "$here/build;$jar" net.minecraft.src.Oracle "$@" 2>&1 | { grep -v -E '^(16 achievements|151 recipes)$' || true; }
