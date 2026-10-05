"""Build a focused verify script from tools/verify181.sh: its header plus only the top-level
blocks whose section titles match the given patterns.
Usage: python tools/b181/extract_sections.py out.sh 'chunk meshes' 'block showcase' ...
A block runs from a blank line / comment before its first `echo "== ` up to the next one.
"""
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
lines = open(os.path.join(ROOT, 'tools/verify181.sh'), encoding='utf-8').read().split('\n')
start = next(i for i, l in enumerate(lines) if l.startswith('fail=0'))
header = lines[:start + 1] + [l for l in lines[start + 1:start + 40] if False]
# helper functions defined at top level (run, run_extra, ...) are kept
funcs, i = [], start + 1
while i < len(lines):
    if re.match(r'^\w+\(\) \{', lines[i]):
        j = i
        while not (lines[j].rstrip().endswith('}') and (j > i or lines[j].count('{') <= lines[j].count('}'))) and not lines[j].startswith('}'):
            j += 1
        funcs += lines[i:j + 1]
        i = j + 1
    else:
        i += 1
# split body into blank-line separated paragraphs
body = lines[start + 1:]
paras, cur = [], []
for l in body:
    if l.strip() == '' and cur:
        paras.append(cur)
        cur = []
    else:
        cur.append(l)
if cur:
    paras.append(cur)
pats = [p.lower() for p in sys.argv[2:]]
keep = [p for p in paras if any(pt in '\n'.join(p).lower() for pt in pats)
        and any('echo "== ' in l for l in p) and not any(re.match(r'^\w+\(\) \{', l) for l in p)]
out = header + funcs + [''] + [l for p in keep for l in p + ['']] + [
    '[ $fail -eq 0 ] && echo "ALL CASES EXACT" || echo "SOME CASES DIFFER"', 'exit $fail', '']
open(sys.argv[1], 'w', encoding='utf-8', newline='\n').write('\n'.join(out))
print(sys.argv[1], len(keep), 'blocks')
