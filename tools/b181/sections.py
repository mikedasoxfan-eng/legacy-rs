"""Per-section pass/fail summary of a verify log (sections start with '== ').
Usage: python tools/b181/sections.py target/verify181.log [--failing]
"""
import re
import sys

BAD = re.compile(rb'(?i:DIFFER|MISMATCH|differ:|panicked|error)|FAIL')
SOFT = re.compile(rb'Exception|^	at ')  # Java stack traces the original prints and survives
GOOD = re.compile(rb'EXACT|MATCH|exact|identical|OK\b')

data = open(sys.argv[1], 'rb').read().split(b'\n')
sections, cur = [], None
for line in data:
    if b'MinecraftResources' in line:
        continue  # the old client's resource download thread: the server is gone, harmless
    if line.startswith(b'== '):
        cur = [line[3:].strip().decode('utf-8', 'replace'), 0, 0, 0]
        sections.append(cur)
    elif cur is not None and re.match(rb'^([A-Z0-9 .()-]*CASES[A-Z ]*(EXACT|DIFFER)|exit [0-9]+)[ \t\r]*$', line):
        cur = None  # the suite's own summary line, not part of the last section
    elif cur is not None:
        if re.search(rb'\b0 (of \d+ \w+ |\w+ )?(differ|mismatch|failed)|test result: ok', line):
            cur[2] += 1
        elif b'warning:' in line or line.startswith(b'  ') and b'(' in line and b';' in line:
            continue  # javac warnings and the source lines they quote
        elif SOFT.search(line):
            cur[3] += 1
        elif BAD.search(line):
            cur[1] += 1
        elif GOOD.search(line):
            cur[2] += 1
for s in sections:
    if s[3] and not s[2]:
        s[1] += s[3]  # exceptions with no exact result
ok = [s for s in sections if s[1] == 0]
bad = [s for s in sections if s[1] != 0]
print(f'{len(ok)} exact / {len(sections)} sections')
for name, nbad, ngood, _ in (bad if '--failing' in sys.argv else sections):
    print(('  ok  ' if nbad == 0 else '  XX  ') + name)
