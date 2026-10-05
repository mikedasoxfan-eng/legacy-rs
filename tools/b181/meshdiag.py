"""Group mesh-check differences by the showcase block they belong to.
Usage: python tools/b181/meshdiag.py target/sc.log tools/oracle181/showcase.txt
"""
import math
import re
import sys

lay = [l.split() for l in open(sys.argv[2]) if l.strip()][1:]
pos = {(int(a), int(b), int(c)): (int(d), int(e)) for a, b, c, d, e in lay}
seen = {}
pat = re.compile(r'\s*chunk \((-?\d+),(-?\d+)\) pass (\d) vertex \d+: java pos \(([-\d.e]+),([-\d.e]+),([-\d.e]+)\)(.*)')
for line in open(sys.argv[1], encoding='utf-8', errors='replace'):
    m = pat.match(line)
    if not m:
        continue
    cx, cz = int(m.group(1)), int(m.group(2))
    x, y, z = [float(m.group(i)) for i in (4, 5, 6)]
    cands = set()
    for dx in (0, -1):
        for dy in (0, -1):
            for dz in (0, -1):
                k = (cx * 16 + math.floor(x + dx * 0.001), math.floor(y + dy * 0.001), cz * 16 + math.floor(z + dz * 0.001))
                if k in pos:
                    cands.add(pos[k][0])
    key = tuple(sorted(cands))
    seen.setdefault(key, []).append(line.strip()[:330])
for k, v in sorted(seen.items()):
    print(k, len(v), '|', v[0])
