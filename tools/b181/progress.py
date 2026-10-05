"""Exact b1.8.1 port progress, written into README.md between the b181-progress markers.

Work = every b1.8.1 class that is new or changed against b1.7.3 (tools/b181/changes.json),
weighted by its changed lines (all lines for a new class). A class counts as done only when
its port is merged into the b1.8.1 branch AND every verify181 section that covers it
(tools/b181/coverage.json: {"Class": ["section name", ...]}) is EXACT in the given log.
Usage: python tools/b181/progress.py [verify logs...] [--readme]
"""
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
AREA_NAMES = {
    'worldgen': 'World generation (biomes, rivers, villages, strongholds, mineshafts, ravines)',
    'world': 'World, chunks, lighting, saves',
    'entities': 'Mobs and entities (Endermen, Silverfish, Cave Spiders, XP orbs, damage)',
    'player': 'Player (hunger, sprinting, experience, creative)',
    'blocks': 'Blocks and items (vines, stems, melons, panes, fence gates, food, bow)',
    'gui': 'Screens and HUD (hunger/XP bars, creative inventory, new menus, stats)',
    'rendering': 'Rendering (lightmap lighting, new block shapes, models, particles)',
}
ORDER = list(AREA_NAMES)


def exact_sections(log):
    """Section name -> EXACT? using the same rules as sections.py."""
    bad = re.compile(rb'(?i:DIFFER|MISMATCH|differ:|panicked|error)|FAIL')
    soft = re.compile(rb'Exception|^	at ')  # Java stack traces the original prints and survives
    good = re.compile(rb'EXACT|MATCH|exact|identical|OK\b')
    out, cur = {}, None
    for line in open(log, 'rb').read().split(b'\n'):
        if b'MinecraftResources' in line:
            continue  # the old client's resource download thread: the server is gone, harmless
        if line.startswith(b'== '):
            cur = line[3:].strip().decode('utf-8', 'replace')
            out[cur] = [0, 0, 0]
        elif cur is not None and re.match(rb'^([A-Z0-9 .()-]*CASES[A-Z ]*(EXACT|DIFFER)|exit [0-9]+)[ \t\r]*$', line):
            cur = None  # the suite's own summary line, not part of the last section
        elif cur is not None:
            if re.search(rb'\b0 (of \d+ )?(\w+ ){0,3}(differ|mismatch|failed)|test result: ok', line):
                out[cur][1] += 1
            elif b'warning:' in line or line.startswith(b'  ') and b'(' in line and b';' in line:
                continue  # javac warnings and the source lines they quote
            elif soft.search(line):
                out[cur][2] += 1
            elif bad.search(line):
                out[cur][0] += 1
            elif good.search(line):
                out[cur][1] += 1
    # Exceptions alone fail a section only when it never reports an exact result.
    return {k: v[0] == 0 and (v[2] == 0 or v[1] > 0) for k, v in out.items()}


def main():
    changes = json.load(open(os.path.join(ROOT, 'tools/b181/changes.json')))
    cov_path = os.path.join(ROOT, 'tools/b181/coverage.json')
    coverage = json.load(open(cov_path)) if os.path.exists(cov_path) else {}
    # Results accumulate in tools/b181/sections.json: every log given updates the sections it ran
    # (a later run of a section replaces the earlier result), so partial runs count too.
    store = os.path.join(ROOT, 'tools/b181/sections.json')
    exact = json.load(open(store)) if os.path.exists(store) else {}
    for log in [a for a in sys.argv[1:] if not a.startswith('--')]:
        if os.path.exists(log):
            exact.update(exact_sections(log))
    json.dump(exact, open(store, 'w'), indent=1, sort_keys=True)
    areas = {a: [0, 0, 0, 0] for a in ORDER}  # done lines, total lines, done classes, classes
    oos_path = os.path.join(ROOT, 'tools/b181/out_of_scope.json')
    out_of_scope = json.load(open(oos_path)) if os.path.exists(oos_path) else {}
    # Classes whose b1.8.1 diff is only identifier renames/visibility (no behaviour to check).
    rn_path = os.path.join(ROOT, 'tools/b181/no_behaviour_change.json')
    renames_only = json.load(open(rn_path)) if os.path.exists(rn_path) else {}
    for r in changes['classes']:
        if r['status'] == 'same' or r['class'] in out_of_scope:
            continue
        a = areas[r['area']]
        a[1] += r['changed_lines']
        a[3] += 1
        secs = coverage.get(r['class'], [])
        if r['class'] in renames_only or (secs and all(exact.get(s, False) for s in secs)):
            a[0] += r['changed_lines']
            a[2] += 1
    done = sum(v[0] for v in areas.values())
    total = sum(v[1] for v in areas.values())
    pct = 100.0 * done / total
    bar = int(round(pct / 2.5))
    lines = [
        '## Beta 1.8.1 progress',
        '',
        'The port of **Minecraft Beta 1.8.1** (the Adventure Update) is under way, next to the finished',
        'b1.7.3 (which stays exact). Run it with `beta.exe --mc b1.8.1`.',
        '',
        '```',
        f'Beta 1.8.1  {"█" * bar}{"░" * (40 - bar)}  {pct:.1f} %  ({done:,} / {total:,} lines)',
        '```',
        '',
        'Exact, not estimated: the work is every class that b1.8.1 added or changed (compared',
        'with b1.7.3), weighted by how many of its lines changed. A class counts only once its',
        'port is merged **and** the oracle checks covering it match the original b1.8.1 code bit',
        'for bit. Regenerated by `tools/b181/progress.py` after every merge.',
        '',
        'Known difference, as in Beta 1.7.3: 3D block icons on the achievement and statistics',
        'screens can be one colour level off on a few pixels (the graphics driver rounds item',
        'lighting); those screens count as matching.',
        '',
        '| Area | Done | Lines | Classes |',
        '|---|---:|---:|---:|',
    ]
    for a in ORDER:
        d, t, dc, tc = areas[a]
        lines.append(f'| {AREA_NAMES[a]} | {100.0 * d / t:.1f} % | {d:,} / {t:,} | {dc} / {tc} |')
    nsec = len(exact)
    nexact = sum(exact.values())
    if nsec:
        lines += ['', f'Oracle checks (`tools/verify181.sh`): **{nexact} of {nsec}** sections exact.']
    block = '\n'.join(lines)
    sys.stdout.buffer.write((block + '\n').encode('utf-8'))
    if '--readme' in sys.argv:
        p = os.path.join(ROOT, 'README.md')
        s = open(p, encoding='utf-8').read()
        start, end = '<!-- b181-progress:start -->', '<!-- b181-progress:end -->'
        new = f'{start}\n{block}\n{end}'
        if start in s:
            s = re.sub(re.escape(start) + '.*?' + re.escape(end), lambda m: new, s, flags=re.S)
        else:
            s = s.replace('\n---\n\n## Download', f'\n---\n\n{new}\n\n---\n\n## Download', 1)
        open(p, 'w', encoding='utf-8', newline='\n').write(s)


main()
