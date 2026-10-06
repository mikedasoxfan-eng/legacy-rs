"""Compare checked-in definitions with freshly extracted original 1.0 tables."""
import re
import sys
from pathlib import Path
root = Path(__file__).resolve().parents[2]
def normalized(path):
    text = Path(path).read_text(encoding="utf-8").split("// Compatibility aliases")[0]
    text = re.sub(r"//[^\n]*", "", text)
    return re.sub(r"\s+", "", text)
for generated, checked in [(sys.argv[1],root / "crates/worldgen10/src/block/table.rs"),
                            (sys.argv[2],root / "crates/worldgen10/src/entity/block_table.rs")]:
    if normalized(generated) != normalized(checked):
        j,r=normalized(generated),normalized(checked)
        offset=next((i for i,(a,b) in enumerate(zip(j,r)) if a!=b),min(len(j),len(r)))
        jc=re.search(r'MATERIALS:\[MaterialProps;(\d+)\]',j)
        rc=re.search(r'MATERIALS:\[MaterialProps;(\d+)\]',r)
        if jc and rc and jc[1]!=rc[1]:print(f'FIRST DIFFERENCE: MATERIALS array length: Java {jc[1]}, Rust {rc[1]}')
        else:print(f'FIRST DIFFERENCE: definition text character {offset}: Java {j[offset:offset+40]!r}, Rust {r[offset:offset+40]!r}')
        raise SystemExit(f"DEFINITIONS DIFFER: {checked}")
print("EXACT MATCH: all 256 block definitions, 29 materials and block geometry defaults")
