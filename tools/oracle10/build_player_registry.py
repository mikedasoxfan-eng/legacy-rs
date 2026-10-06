"""Extract release Item/Block defaults in the original Block-first bootstrap order."""
from pathlib import Path
import subprocess
import os
root = Path(__file__).resolve().parents[2]
bash = "C:/Program Files/Git/usr/bin/bash.exe" if os.name == "nt" else "bash"
subprocess.run([bash, str(root / "tools/oracle10/oracle.sh"), "wbregistry", "player_registry.bin"], check=True)
data = (root / "tools/oracle10/out/player_registry.bin").read_bytes()
assert len(data) == 2258 * (16 + 256 * 5) + 256 * 5 + 2258 * 4
(root / "crates/worldgen10/src/entity/player/registry.bin").write_bytes(data)
