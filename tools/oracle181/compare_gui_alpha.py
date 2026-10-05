"""Compare original framebuffer alpha observations with zero tolerance."""
from pathlib import Path
import sys

scenes, java, rust = map(Path, sys.argv[1:])
names = [line.split('#', 1)[0].strip().split(maxsplit=1)[1]
         for line in scenes.read_text(encoding='utf-8').splitlines()
         if line.split('#', 1)[0].strip().startswith('shot ')]
assert names, 'No captures requested'
failed = False
total = 0
for name in names:
    a, b = java / (name + '.alpha'), rust / (name + '.alpha')
    if not a.is_file() or not b.is_file():
        print(name + ': missing alpha observation')
        failed = True
        continue
    original, port = a.read_bytes(), b.read_bytes()
    if len(original) != 854 * 480 or original != port:
        print(name + ': ALPHA DIFFERS')
        failed = True
    else:
        print(name + ': ALPHA EXACT')
        total += len(original)
print(f'GUI ALPHA {"DIFFERS" if failed else "EXACT"}: {total} original bytes')
sys.exit(1 if failed else 0)
