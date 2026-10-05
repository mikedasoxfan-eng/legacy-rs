"""Extract immutable Beta Item/Block defaults from Mojang's original classes."""
from pathlib import Path
import subprocess
import os
root = Path(__file__).resolve().parents[2]
bash = "C:/Program Files/Git/usr/bin/bash.exe" if os.name == "nt" else "bash"
subprocess.run([bash, str(root / "tools/oracle181/oracle.sh"), "playerregistry", "player_registry.bin"], check=True)
data = (root / "tools/oracle181/out/player_registry.bin").read_bytes()
assert len(data) == 2258 * (16 + 256 * 5) + 256 * 5 + 2258 * 4
(root / "crates/worldgen/src/entity/player/registry.bin").write_bytes(data)
