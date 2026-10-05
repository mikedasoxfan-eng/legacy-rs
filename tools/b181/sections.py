"""Per-section pass/fail summary of a verify log (sections start with '== ').
Usage: python tools/b181/sections.py target/verify181.log [--failing]
"""
import re
import sys

BAD = re.compile(rb'DIFFER|MISMATCH|mismatch|FAIL|differ:|Exception|panicked|error', re.I)
GOOD = re.compile(rb'EXACT|MATCH|exact|identical|OK\b')

data = open(sys.argv[1], 'rb').read().split(b'\n')
sections, cur = [], None
for line in data:
    if line.startswith(b'== '):
        cur = [line[3:].strip().decode('utf-8', 'replace'), 0, 0]
        sections.append(cur)
    elif cur is not None:
        if re.search(rb'\b0 (of \d+ \w+ )?differ', line):
            cur[2] += 1
        elif BAD.search(line):
            cur[1] += 1
        elif GOOD.search(line):
            cur[2] += 1
ok = [s for s in sections if s[1] == 0 and s[2] > 0]
bad = [s for s in sections if not (s[1] == 0 and s[2] > 0)]
print(f'{len(ok)} exact / {len(sections)} sections')
for name, nbad, ngood in (bad if '--failing' in sys.argv else sections):
    print(('  ok  ' if (nbad == 0 and ngood > 0) else '  XX  ') + name)
