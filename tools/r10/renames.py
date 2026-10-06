"""Identifier renames from Beta 1.8.1 to Minecraft 1.0 (MCP names in the two RetroMCP decompilations).

Lines that are identical once identifiers are blanked out, at the same place in the same class, pair their
identifiers token by token; a 1.8.1 name that consistently became one 1.0 name is a rename. Classes that
exist under one name only are paired by content similarity (MCHash -> IntHashMap, ...).
Writes tools/r10/renames.json: {"classes": {old: new}, "members": {"Class.old": "new"}, "global": {old: new}}.
Usage: python tools/r10/renames.py
"""
import collections
import difflib
import json
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
OLD = os.path.join(ROOT, 'tools/retromcp-b181/minecraft/src/net/minecraft/src')
NEW = os.path.join(ROOT, 'tools/retromcp-r10/minecraft/src/net/minecraft/src')
TOKEN = re.compile(r'[A-Za-z_][A-Za-z0-9_]*|\S')
KEEP = set('if else for while return new int float double boolean byte short long char void this super null '
           'true false public private protected static final class extends implements try catch throw throws '
           'instanceof break continue switch case default do abstract synchronized interface'.split())


def lines(path):
    text = open(path, encoding='utf-8', errors='replace').read().replace('\r', '')
    return [l.strip() for l in text.split('\n') if l.strip() and not l.strip().startswith(('import ', 'package '))]


def ident(t):
    return t[0].isalpha() or t[0] == '_'


def shape(tokens):
    return ['I' if ident(t) and t not in KEEP and not re.fullmatch(r'var\d+', t) else t for t in tokens]


def main():
    old = {f[:-5] for f in os.listdir(OLD) if f.endswith('.java')}
    new = {f[:-5] for f in os.listdir(NEW) if f.endswith('.java')}
    classes = {}
    gone, added = sorted(old - new), sorted(new - old)
    texts = {n: '\n'.join(lines(os.path.join(NEW, n + '.java'))) for n in added}
    for o in gone:
        t = '\n'.join(lines(os.path.join(OLD, o + '.java')))
        best = max(added, key=lambda n: difflib.SequenceMatcher(None, t, texts[n], autojunk=False).quick_ratio(), default=None)
        if best and difflib.SequenceMatcher(None, t, texts[best], autojunk=False).ratio() > 0.6:
            classes[o] = best
    pairs = [(c, c) for c in sorted(old & new)] + sorted(classes.items())
    member = collections.defaultdict(collections.Counter)
    glob = collections.defaultdict(collections.Counter)
    for o, n in pairs:
        a, b = lines(os.path.join(OLD, o + '.java')), lines(os.path.join(NEW, n + '.java'))
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, a, b, autojunk=False).get_opcodes():
            if tag != 'replace' or i2 - i1 != j2 - j1:
                continue
            for x, y in zip(a[i1:i2], b[j1:j2]):
                tx, ty = TOKEN.findall(x), TOKEN.findall(y)
                if len(tx) != len(ty) or shape(tx) != shape(ty):
                    continue
                for p, q in zip(tx, ty):
                    if p != q and ident(p) and not re.fullmatch(r'var\d+', p):
                        member[f'{o}.{p}'][q] += 1
                        glob[p][q] += 1
    members = {k: c.most_common(1)[0][0] for k, c in member.items() if c.most_common(1)[0][1] >= 1}
    # A name is renamed globally when every observation agrees (numbered MCP names are unique).
    globals_ = {k: c.most_common(1)[0][0] for k, c in glob.items()
                if len(c) == 1 or c.most_common(1)[0][1] >= 0.9 * sum(c.values())}
    for o, n in classes.items():
        globals_[o] = n
    out = os.path.join(ROOT, 'tools/r10/renames.json')
    json.dump({'classes': classes, 'members': members, 'global': globals_}, open(out, 'w'), indent=1, sort_keys=True)
    print(f'{len(classes)} class renames, {len(members)} member renames, {len(globals_)} global renames')
    print('classes:', classes)


main()
