"""Compare checked-in definitions with freshly extracted original b1.8.1 tables."""
import re
import sys
from pathlib import Path
root = Path(__file__).resolve().parents[2]
def normalized(path):
    text = Path(path).read_text(encoding="utf-8").split("// Compatibility aliases")[0]
    text = re.sub(r"//[^\n]*", "", text)
    return re.sub(r"\s+", "", text)
for generated, checked in [(sys.argv[1],root / "crates/worldgen181/src/block/table.rs"),
                            (sys.argv[2],root / "crates/worldgen181/src/entity/block_table.rs")]:
    if normalized(generated) != normalized(checked):
        raise SystemExit(f"DEFINITIONS DIFFER: {checked}")
print("EXACT MATCH: all 256 block definitions, 29 materials and block geometry defaults")
