"""Rename b1.7.3 member names used as reflection strings in tools/oracle181 ("field_22228_r",
getDeclaredField("...")) when the name no longer exists in the b1.8.1 sources.
Prints names it could not map. Usage: python tools/b181/port_strings.py
"""
import json
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
SRC = os.path.join(ROOT, 'tools/retromcp-b181/minecraft/src/net/minecraft')
ren = json.load(open(os.path.join(ROOT, 'tools/b181/renames.json')))
idents = set()
for dp, _, fs in os.walk(SRC):
    for f in fs:
        if f.endswith('.java'):
            idents.update(re.findall(r'\b[A-Za-z_]\w*\b', open(os.path.join(dp, f), encoding='utf-8', errors='replace').read()))
unmapped = set()
for dp, _, fs in os.walk(os.path.join(ROOT, 'tools/oracle181')):
    if 'build' in dp or 'retromcp' in dp:
        continue
    for f in fs:
        if not f.endswith('.java'):
            continue
        p = os.path.join(dp, f)
        s = open(p, encoding='utf-8', newline='').read()

        def fix(m):
            name = m.group(1)
            if name in idents or not re.match(r'^[a-z]\w*$', name):
                return m.group(0)
            if name in ren:
                return '"' + ren[name][0] + '"'
            if name.startswith(('field_', 'func_')):
                unmapped.add((f, name))
            return m.group(0)
        t = re.sub(r'"([A-Za-z_]\w*)"', fix, s)
        if t != s:
            open(p, 'w', encoding='utf-8', newline='').write(t)
            print('fixed', f)
for f, n in sorted(unmapped):
    print('UNMAPPED', f, n)
