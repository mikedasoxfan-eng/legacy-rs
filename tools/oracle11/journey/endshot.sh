#!/usr/bin/env bash
# The original Release 1.0 client in the End (opens a game window for about a minute): saves what
# it renders standing on the arrival platform, for comparison with the port's play test
# journey_end_view (QA journey, Part C).
# Usage: tools/oracle11/journey/endshot.sh [outdir]   (default target/oracle11-endshot)
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd)"
root="$(cd "$here/../../.." && pwd)"
jar="$(cygpath -w "$root/tools/retromcp-r11/minecraft/jars/deobfuscated.jar")"
natives="$(cygpath -w "$root/tools/retromcp-r11/libraries/natives")"
libs=$(find "$root/tools/retromcp-r11/libraries" -name '*.jar' ! -name '*sources*' ! -name '*natives*' | while read -r f; do printf ';%s' "$(cygpath -w "$f")"; done)
out="${1:-$root/target/oracle11-endshot}"
build="$root/target/oracle11-endshot-build"
data="$root/target/oracle11-endshot-mc"
python - "$build" "$data" "$root/target" <<'PY'
import shutil, sys
from pathlib import Path
target = Path(sys.argv[3]).resolve()
for d in sys.argv[1:3]:
    p = Path(d).resolve()
    if p.parent != target:
        raise SystemExit('refusing cleanup outside target/')
    shutil.rmtree(p, ignore_errors=True)
PY
mkdir -p "$build" "$data" "$out"
javac -nowarn -cp "$jar$libs" -d "$(cygpath -w "$build")" "$(cygpath -w "$here/EndShot.java")"
java=java
for jl in "$LOCALAPPDATA"/Packages/Microsoft.4297127D64EC6_*/LocalCache/Local/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe \
          "$APPDATA"/.minecraft/runtime/jre-legacy/windows-x64/jre-legacy/bin/java.exe; do
  [ -x "$jl" ] && { java="$jl"; break; }
done
[ -n "${ORACLE_JAVA:-}" ] && java="$ORACLE_JAVA"
"$java" -version 2>&1 | grep -q '1.8.0_51' || { echo "oracle requires Mojang Java 8u51; refusing another JVM" >&2; exit 1; }
# The same views as tools/playtest10/journey_end_view.txt (yaw 90 faces -X, the island).
"$java" -Djava.library.path="$natives" -cp "$(cygpath -w "$build");$jar$libs" net.minecraft.src.EndShot 7 \
  "$(cygpath -w "$out")" "$(cygpath -w "$data")" island:90:0 ground:90:45 sky:270:-30
