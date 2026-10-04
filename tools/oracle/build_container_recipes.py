"""Extract sorted recipes and Item container/armor/fuel defaults from original bytecode."""
from pathlib import Path
import os
import subprocess

root = Path(__file__).resolve().parents[2]
bash = 'C:/Program Files/Git/usr/bin/bash.exe' if os.name == 'nt' else 'bash'
subprocess.run([bash, str(root / 'tools/oracle/oracle.sh'), 'containerrecipes', 'container_recipes.bin'], check=True)
data = (root / 'tools/oracle/out/container_recipes.bin').read_bytes()
assert int.from_bytes(data[:4], 'big') == 151
(root / 'crates/worldgen/src/container/recipes.bin').write_bytes(data)
