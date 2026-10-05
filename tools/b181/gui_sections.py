"""Turn a tools/verify_gui181.sh log into per-scene sections ("== gui181: <scene>") so
progress.py can credit screens by the pixel comparisons that cover them.
Usage: python tools/b181/gui_sections.py gui.log > gui_sections.log
"""
import re
import sys

for line in open(sys.argv[1], encoding='utf-8', errors='replace'):
    line = line.rstrip('\n')
    t = re.match(r'^(TEXTURES|COLOR MASKS) (EXACT|DIFFER\w*)', line)
    if t:
        print(f'== gui181: {t.group(1).lower()} check')
        print(line)
        continue
    m = re.match(r'^([A-Za-z0-9_]+)(?:\.png)?: (.*)$', line)
    if not m or not re.search(r'EXACT|differ', m.group(2)):
        continue
    print(f'== gui181: {m.group(1)}')
    print('EXACT' if m.group(2).startswith('EXACT') else 'DIFFERS: ' + m.group(2))
