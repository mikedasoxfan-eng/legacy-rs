"""Add a worker report's Done classes to tools/r10/coverage.json.

For each `| Class | Done | <proof> |` row, the check names in the proof (words before ':' or
listed keys like `entities181`) are matched against the section names of the given verify log;
a class is credited to every section whose name contains one of its check names. If none
match, the class is tied to all sections of that log (it then counts only if the whole
focused suite is exact). Classes the report marks Not done are removed again.
Usage: python tools/r10/coverage_from_report.py report.md focused.log
"""
import json
import os
import re
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
report, log = sys.argv[1], sys.argv[2]
sections = [l[3:].strip().decode('utf-8', 'replace') for l in open(log, 'rb').read().split(b'\n') if l.startswith(b'== ')]
path = os.path.join(ROOT, 'tools/r10/coverage.json')
cov = json.load(open(path)) if os.path.exists(path) else {}
changes = {r['class'] for r in json.load(open(os.path.join(ROOT, 'tools/r10/changes.json')))['classes']}
KEEP = set(json.load(open(os.path.join(ROOT, 'tools/r10/integrator_credits.json'))))
added = removed = 0
for line in open(report, encoding='utf-8'):
    cells = [c.strip() for c in line.split('|')]
    if len(cells) < 4 or cells[1].rstrip('*') not in changes:
        continue
    cls, status, proof = cells[1].rstrip('*'), cells[2].lower(), cells[3]
    if (status.startswith('not') or 'partial' in status) and cls in KEEP:
        continue  # verified by the integrator's own checks (tools/r10/integrator_credits.json)
    if status.startswith('not') or 'partial' in status:
        removed += cov.pop(cls, None) is not None
        continue
    if not status.startswith('done'):
        continue
    keys = set(re.findall(r'([A-Za-z][\w/+-]*?)\s*:', proof)) | set(re.findall(r'\b(\w+181)\b', proof))
    keys = {k.lower() for k in keys for k in re.split(r'[/+]', k) if len(k) > 2}
    hit = [s for s in sections if any(k in s.lower().replace(' ', '') or k in s.lower() for k in keys)]
    cov[cls] = sorted(set(cov.get(cls, [])) | set(hit or sections))
    added += 1
json.dump(cov, open(path, 'w'), indent=1, sort_keys=True)
print(f'{added} classes credited, {removed} removed, {len(cov)} in coverage')
