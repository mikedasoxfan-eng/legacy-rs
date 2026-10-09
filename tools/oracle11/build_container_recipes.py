"""Extract sorted recipes and Item container/armor/fuel defaults from original bytecode."""
from pathlib import Path
import os
import subprocess

root = Path(__file__).resolve().parents[2]
bash = 'C:/Program Files/Git/usr/bin/bash.exe' if os.name == 'nt' else 'bash'
env = dict(os.environ)
env['JAVA_TOOL_OPTIONS'] = env.get('JAVA_TOOL_OPTIONS', '') + ' -Xmx512m'
subprocess.run([bash, str(root / 'tools/oracle11/oracle.sh'), 'containerrecipes', 'container_recipes.bin'], check=True, env=env)
data = (root / 'tools/oracle11/out/container_recipes.bin').read_bytes()
assert int.from_bytes(data[:4], 'big') > 0
(root / 'crates/worldgen10/src/container/recipes.bin').write_bytes(data)
(root / 'crates/client10/src/gui/screens/item_names.tsv').write_bytes((root / 'tools/oracle11/out/container_recipes.bin.names').read_bytes())
(root / 'crates/client10/src/gui/screens/item_render.bin').write_bytes((root / 'tools/oracle11/out/container_recipes.bin.render').read_bytes())
