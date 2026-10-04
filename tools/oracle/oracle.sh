#!/usr/bin/env bash
# Build (if needed) and run the Java oracle against Mojang's original b1.7.3 bytecode.
# Usage: tools/oracle/oracle.sh <mode> [args...]   (see Oracle.java)
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd -W)"  # -W: Windows-style path for java/javac
jar="$here/../retromcp/minecraft/jars/deobfuscated.jar"
[ -f "$jar" ] || { echo "missing $jar - run RetroMCP setup+decompile for b1.7.3 first" >&2; exit 1; }
mkdir -p "$here/build" "$here/out"
if [ ! -f "$here/build/Oracle.class" ] || [ "$here/src/Oracle.java" -nt "$here/build/Oracle.class" ]; then
  javac -nowarn -cp "$jar" -d "$here/build" "$here/src/Oracle.java"
fi
cd "$here/out"
java -cp "$here/build;$jar" Oracle "$@" 2>&1 | { grep -v -E '^(16 achievements|151 recipes)$' || true; }
