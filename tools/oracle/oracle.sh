#!/usr/bin/env bash
# Build (if needed) and run the Java oracle against Mojang's original b1.7.3 bytecode.
# Usage: tools/oracle/oracle.sh <mode> [args...]   (see Oracle.java)
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd -W)"  # -W: Windows-style path for java/javac
jar="$here/../retromcp/minecraft/jars/deobfuscated.jar"
# Client libraries (LWJGL etc.) are only needed so client classes can load; nothing opens a window.
libs=$(find "$here/../retromcp/libraries" -name '*.jar' ! -name '*sources*' ! -name '*natives*' 2>/dev/null | while read -r f; do printf ';%s' "$(cygpath -w "$f")"; done)
[ -f "$jar" ] || { echo "missing $jar - run RetroMCP setup+decompile for b1.7.3 first" >&2; exit 1; }
mkdir -p "$here/build" "$here/out"
if [ ! -f "$here/build/.compiled" ] || find "$here/src" "$here/headless" -name '*.java' -newer "$here/build/.compiled" -print -quit | grep -q .; then
  javac -nowarn -cp "$jar$libs" -d "$here/build" "$here/src/net/minecraft/src/Oracle.java" "$here/src/net/minecraft/src/RendererOracle.java" "$here/src/net/minecraft/src/EntityOracle.java" "$here/src/net/minecraft/src/PlayerOracle.java" "$here/src/net/minecraft/src/ContainerOracle.java" "$here/src/net/minecraft/src/OracleHooks.java" "$here/src/net/minecraft/src/TeleportOracle.java" "$here/src/net/minecraft/src/ShowcaseOracle.java" "$here/src/net/minecraft/src/RedstoneOracle.java" "$here/src/net/minecraft/src/StatsOracle.java" "$here/src/net/minecraft/src/FirstPersonOracle.java" "$here/src/net/minecraft/src/DecorationOracle.java" "$here/src/net/minecraft/src/PistonRendererOracle.java" "$here/src/net/minecraft/src/WorldEventOracle.java" "$here/src/net/minecraft/src/ParticleOracle.java" "$here/src/net/minecraft/src/SoundOracle.java" "$here"/headless/org/lwjgl/opengl/*.java
  touch "$here/build/.compiled"
fi
if [ ! -f "$here/build/net/minecraft/src/Explosion.class" ] || [ ! -f "$here/build/net/minecraft/src/Entity.class" ] || [ "$here/build_entity.py" -nt "$here/build/net/minecraft/src/Entity.class" ] || [ "$here/src/net/minecraft/src/OracleHooks.java" -nt "$here/build/net/minecraft/src/Entity.class" ]; then
  python "$here/build_entity.py" >&2
fi
# Prefer the JRE Mojang's launcher runs b1.7.3 on (8u51): Math.sin & co. differ
# between Java builds, and that is the one vanilla actually uses.
java=java
for jl in "$LOCALAPPDATA"/Packages/Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe           "$APPDATA"/.minecraft/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe; do
  [ -x "$jl" ] && { java="$jl"; break; }
done
[ -n "${ORACLE_JAVA:-}" ] && java="$ORACLE_JAVA"
"$java" -version 2>&1 | grep -q '1.8.0_51' || { echo "oracle requires Mojang Java 8u51; refusing another JVM" >&2; exit 1; }
cd "$here/out"
"$java" -cp "$here/build;$jar$libs" net.minecraft.src.Oracle "$@" 2>&1 | { grep -v -E '^(16 achievements|151 recipes)$' || true; }
