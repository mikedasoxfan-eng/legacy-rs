"""Identifier renames from b1.7.3 to b1.8.1 (the two decompilations use different MCP names).

Aligns every class present in both trees line by line; where a replaced line differs from its
b1.7.3 counterpart only in identifiers, each old/new identifier pair is a rename vote. Writes
tools/b181/renames.json: {old: [new, votes]} keeping the pair with the most votes.
Usage: python tools/b181/renames.py [identifier ...]   (with arguments: print those lookups)
"""
import difflib
import json
import os
import re
import sys
from collections import Counter, defaultdict

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
OLD = os.path.join(ROOT, 'tools/retromcp/minecraft/src/net/minecraft')
NEW = os.path.join(ROOT, 'tools/retromcp-b181/minecraft/src/net/minecraft')
OUT = os.path.join(ROOT, 'tools/b181/renames.json')
TOK = re.compile(r'[A-Za-z_$][\w$]*|\d[\w.]*|\S')


def read(p):
    with open(p, encoding='utf-8', errors='replace') as f:
        return [l.strip() for l in f]


def build():
    votes = defaultdict(Counter)
    for sub in ('src', 'client'):
        for fn in os.listdir(os.path.join(NEW, sub)):
            op = os.path.join(OLD, sub, fn)
            if not fn.endswith('.java') or not os.path.exists(op):
                continue
            old, new = read(op), read(os.path.join(NEW, sub, fn))
            sm = difflib.SequenceMatcher(None, old, new, autojunk=False)
            for tag, i1, i2, j1, j2 in sm.get_opcodes():
                if tag != 'replace':
                    continue
                for a, b in zip(old[i1:i2], new[j1:j2]):
                    ta, tb = TOK.findall(a), TOK.findall(b)
                    if len(ta) != len(tb):
                        continue
                    diffs = [(x, y) for x, y in zip(ta, tb) if x != y]
                    if not diffs or any(not re.match(r'[A-Za-z_$]', x) or not re.match(r'[A-Za-z_$]', y) for x, y in diffs):
                        continue
                    if any(re.match(r'var\d+$', x) for x, _ in diffs):
                        continue
                    for x, y in set(diffs):
                        votes[x][y] += 1
    out = {k: list(c.most_common(1)[0]) for k, c in votes.items()}
    json.dump(out, open(OUT, 'w'), indent=0, sort_keys=True)
    return out


def main():
    m = build() if not os.path.exists(OUT) or '--build' in sys.argv else json.load(open(OUT))
    for a in sys.argv[1:]:
        if a != '--build':
            print(a, '->', m.get(a))
    if len(sys.argv) == 1 or sys.argv[1:] == ['--build']:
        print(len(m), 'renames')


main()
