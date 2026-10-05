#!/usr/bin/env bash
# Compile the b1.8.1 oracle (and ModelShot) into a scratch dir; errors go to target/javac181.log.
# Usage: tools/oracle181/compile_check.sh
here="$(cd "$(dirname "$0")" && pwd -W)"
jar="$here/../retromcp-b181/minecraft/jars/deobfuscated.jar"
libs=$(find "$here/../retromcp-b181/libraries" -name '*.jar' ! -name '*sources*' ! -name '*natives*' 2>/dev/null | while read -r f; do printf ';%s' "$(cygpath -w "$f")"; done)
rm -rf "$here/build_try"; mkdir -p "$here/build_try"
s="$here/src/net/minecraft/src"
javac -nowarn -cp "$jar$libs" -d "$here/build_try" "$s"/Oracle.java "$s"/RendererOracle.java "$s"/EntityOracle.java "$s"/PlayerOracle.java "$s"/ContainerOracle.java "$s"/OracleHooks.java "$s"/TeleportOracle.java "$s"/ShowcaseOracle.java "$s"/RedstoneOracle.java "$s"/StatsOracle.java "$s"/FirstPersonOracle.java "$s"/VehicleOracle.java "$s"/DecorationOracle.java "$s"/PistonRendererOracle.java "$s"/WorldEventOracle.java "$s"/ParticleOracle.java "$s"/SoundOracle.java "$here"/headless/org/lwjgl/opengl/*.java > "$here/../../target/javac181.log" 2>&1
javac -nowarn -cp "$jar$libs" -d "$here/build_try/visual" "$s"/ModelShot.java >> "$here/../../target/javac181.log" 2>&1
echo "errors: $(grep -c 'error:' "$here/../../target/javac181.log")"
