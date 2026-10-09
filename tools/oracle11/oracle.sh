#!/usr/bin/env bash
# Build (if needed) and run the Java oracle against Mojang's original 1.0 bytecode.
# Usage: tools/oracle11/oracle.sh <mode> [args...]   (see Oracle.java)
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd -W)"  # -W: Windows-style path for java/javac
jar="$here/../retromcp-r11/minecraft/jars/deobfuscated.jar"
# Client libraries (LWJGL etc.) are only needed so client classes can load; nothing opens a window.
libs=$(find "$here/../retromcp-r11/libraries" -name '*.jar' ! -name '*sources*' ! -name '*natives*' 2>/dev/null | while read -r f; do printf ';%s' "$(cygpath -w "$f")"; done)
[ -f "$jar" ] || { echo "missing $jar - run RetroMCP setup+decompile for 1.0 first" >&2; exit 1; }
mkdir -p "$here/build" "$here/out"
# The parallel verifier can start several first-use invocations simultaneously.
# Serialize compilation and hook installation, then release before running Java.
while ! mkdir "$here/build/.compile-lock" 2>/dev/null; do sleep .1; done
trap 'rmdir "$here/build/.compile-lock" 2>/dev/null' EXIT
# 1.0's LWJGL 2.9.4 jar is sealed, which forbids the headless org.lwjgl.opengl stubs in
# build/: use a copy with the seal removed.
unsealed="$here/build/lwjgl-unsealed.jar"
sealed=$(find "$here/../retromcp-r11/libraries" -name 'lwjgl-2*.jar' ! -name '*natives*' ! -name '*sources*' | head -1)
if [ ! -f "$unsealed" ]; then
  python "$here/unseal.py" "$(cygpath -w "$sealed")" "$unsealed"
fi
libs=${libs//"$(cygpath -w "$sealed")"/"$(cygpath -w "$unsealed")"}
# Input stubs are isolated from every existing oracle and from guishot's real LWJGL.
input="$here/build/input"
mkdir -p "$input"
if [ ! -f "$input/.compiled" ] || find "$here/headless/org/lwjgl/input" "$here/headless-input" -name '*.java' -newer "$input/.compiled" -print -quit | grep -q .; then
  javac -nowarn -cp "$jar$libs" -d "$input" "$here"/headless/org/lwjgl/input/*.java "$here"/headless-input/org/lwjgl/opengl/*.java
  touch "$input/.compiled"
fi
if [ ! -f "$here/build/.compiled" ] || find "$here/src" "$here/headless" -name '*.java' -newer "$here/build/.compiled" -print -quit | grep -q .; then
  # Windows caps the command line; javac's argument file also preserves paths with spaces.
  sources="$here/build/sources.args"
  find "$here/src/net/minecraft/src" "$here/headless/org/lwjgl/opengl" -name '*.java' ! -name ModelShot.java | while IFS= read -r path; do printf '"%s"\n' "$path"; done > "$sources"
  javac -nowarn -cp "$input;$jar$libs" -d "$here/build" "@$sources"
  touch "$here/build/.compiled"
fi
if [ ! -f "$here/build/net/minecraft/src/Explosion.class" ] || [ ! -f "$here/build/net/minecraft/src/Entity.class" ] || [ "$here/build_entity.py" -nt "$here/build/net/minecraft/src/Entity.class" ] || [ "$here/src/net/minecraft/src/OracleHooks.java" -nt "$here/build/net/minecraft/src/Entity.class" ]; then
  python "$here/build_entity.py" >&2
fi
rmdir "$here/build/.compile-lock"
trap - EXIT
# Prefer the JRE Mojang's launcher runs b1.7.3 on (8u51): Math.sin & co. differ
# between Java builds, and that is the one vanilla actually uses.
java=java
for jl in "$LOCALAPPDATA"/Packages/Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe           "$APPDATA"/.minecraft/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe; do
  [ -x "$jl" ] && { java="$jl"; break; }
done
[ -n "${ORACLE_JAVA:-}" ] && java="$ORACLE_JAVA"
"$java" -version 2>&1 | grep -q '1.8.0_51' || { echo "oracle requires Mojang Java 8u51; refusing another JVM" >&2; exit 1; }
input_cp=""
case "${1:-}" in runtick10|runtick181|guislot8_10|guislot8_181|stats8_10|stats8_181) input_cp="$input;";; esac
cd "$here/out"
export ORACLE10_SOURCE="$here/src/net/minecraft/src"
export ORACLE10_JAR="$jar"
# 1.0 prints its achievement and recipe counts and IntCache growth on stdout.
"$java" -cp "$input_cp$here/build;$jar$libs" net.minecraft.src.OracleRunner10 "$@" 2>&1 | { grep -v -E '^([0-9]+ achievements|[0-9]+ recipes|New max size: [0-9]+|Placed stronghold in [A-Za-z ]+ biome at .*)$' || true; }
