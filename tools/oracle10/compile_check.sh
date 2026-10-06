#!/usr/bin/env bash
# Compile all 1.0 main/mode, real-GL GUI, visual and server fixtures.
set -euo pipefail
here="$(cd "$(dirname "$0")" && pwd -W)"
python "$here/../r10/audit_fixture_constants10.py"
for group in main gui visual server; do
  python "$here/compile.py" "$group"
done
python "$here/build_entity.py"
touch "$here/build/.compiled"
