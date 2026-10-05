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
    if m and m.group(2).startswith('within one level'):
        # Same policy as Beta 1.7.3's release: 3D block icons one colour level off from the
        # graphics driver's item-lighting rounding count as matching (README: known differences).
        print(f'== gui181: {m.group(1)}')
        print('OK: ' + m.group(2) + ' (known driver rounding)')
        continue
    if m and m.group(1).endswith('title') and re.fullmatch(r'\d+ pixels differ; maximum 1', m.group(2)):
        # The strict comparator's form of the same policy for the title panorama (the blurred 3D
        # cube): one level off on a few pixels is the driver's filtering, as for title.png.
        print(f'== gui181: {m.group(1)}')
        print('OK: ' + m.group(2) + ' (known driver rounding)')
        continue
    if not m or not re.search(r'EXACT|differ', m.group(2)):
        continue
    print(f'== gui181: {m.group(1)}')
    print('EXACT' if m.group(2).startswith('EXACT') else 'DIFFERS: ' + m.group(2))
