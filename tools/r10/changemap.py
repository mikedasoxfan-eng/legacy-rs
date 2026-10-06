"""Class-level change map from Beta 1.8.1 to Minecraft 1.0 (both decompiled with RetroMCP).

Writes tools/r10/changes.json in the format of tools/b181/changes.json. The 1.0 mappings name many
methods that were still numbered in 1.8.1, so a changed line that is identical once identifiers are
blanked out counts as a rename, not as work.
Usage: python tools/r10/changemap.py
"""
import difflib
import json
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
OLD = os.path.join(ROOT, 'tools/retromcp-b181/minecraft/src/net/minecraft/src')
NEW = os.path.join(ROOT, 'tools/retromcp-r10/minecraft/src/net/minecraft/src')
_b181 = {'__file__': os.path.join(ROOT, 'tools/b181/changemap.py')}
exec(open(_b181['__file__'], encoding='utf-8').read().split('\ndef main')[0], _b181)
SKIP, _area181 = _b181['SKIP'], _b181['area']


def area(name):
    # 1.0's new families that the 1.8.1 patterns do not know.
    if re.match(r'^(Enchantment\w*|EnumEnchantmentType|EnumRarity|PotionHelper|EnumArmorMaterial)$', name):
        return 'blocks'
    if re.match(r'^(DragonPart|EnumCreatureAttribute)$', name):
        return 'entities'
    return _area181(name)
IDENT = re.compile(r'\b[A-Za-z_][A-Za-z0-9_]*\b')
KEEP = set('if else for while return new int float double boolean byte short long char void this super null '
           'true false public private protected static final class extends implements try catch throw throws '
           'instanceof break continue switch case default do abstract synchronized'.split())


def lines(path):
    text = open(path, encoding='utf-8', errors='replace').read().replace('\r', '')
    return [l.strip() for l in text.split('\n')
            if l.strip() and not l.strip().startswith(('import ', 'package ', '//', '*', '/*'))]


def shape(line):
    return IDENT.sub(lambda m: m.group(0) if m.group(0) in KEEP else 'I', line)


def main():
    old = {f[:-5] for f in os.listdir(OLD) if f.endswith('.java')}
    classes = []
    for name in sorted(f[:-5] for f in os.listdir(NEW) if f.endswith('.java')):
        if SKIP.match(name):
            continue
        new_lines = lines(os.path.join(NEW, name + '.java'))
        if name not in old:
            classes.append({'class': name, 'status': 'new', 'lines': len(new_lines),
                            'changed_lines': len(new_lines), 'area': area(name)})
            continue
        old_lines = lines(os.path.join(OLD, name + '.java'))
        changed = 0
        for tag, i1, i2, j1, j2 in difflib.SequenceMatcher(None, old_lines, new_lines, autojunk=False).get_opcodes():
            if tag == 'equal':
                continue
            if tag == 'replace' and i2 - i1 == j2 - j1:
                changed += sum(shape(a) != shape(b) for a, b in zip(old_lines[i1:i2], new_lines[j1:j2]))
            else:
                changed += max(i2 - i1, j2 - j1)
        classes.append({'class': name, 'status': 'changed' if changed else 'same', 'lines': len(new_lines),
                        'changed_lines': changed, 'area': area(name)})
    out = os.path.join(ROOT, 'tools/r10/changes.json')
    json.dump({'from': 'b1.8.1', 'to': '1.0', 'classes': classes}, open(out, 'w'), indent=1)
    work = [c for c in classes if c['status'] != 'same']
    print(f"{len(work)} classes, {sum(c['changed_lines'] for c in work):,} lines of work")
    for a in sorted({c['area'] for c in work}):
        sel = [c for c in work if c['area'] == a]
        print(f"  {a:10} {len(sel):4} classes {sum(c['changed_lines'] for c in sel):6,} lines")


main()
