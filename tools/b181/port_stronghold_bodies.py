"""Translate the original 1.8.1 stronghold placement bodies, rejecting unknown calls."""
from pathlib import Path
import re
import subprocess

src = Path('tools/retromcp-b181/minecraft/src/net/minecraft/src')
ids = {'cloth': '35'}
for m in re.finditer(r'public static (?:final )?Block(?:\w*) (\w+) = \(?new \w+\((\d+)', (src/'Block.java').read_text()):
    ids[m[1]] = m[2]
classes = ['Straight', 'Prison', 'LeftTurn', 'RightTurn', 'RoomCrossing', 'StairsStraight', 'Stairs', 'Crossing', 'Library', 'Corridor']

def args(s):
    out = []; depth = 0; start = 0
    for i, c in enumerate(s):
        if c in '([': depth += 1
        elif c in ')]': depth -= 1
        elif c == ',' and depth == 0:
            out.append(s[start:i].strip()); start = i + 1
    out.append(s[start:].strip())
    return out

def calls(s):
    matches = list(re.finditer(r'(this\.func_\d+_[abc]|var2\.nextInt|StructureStrongholdPieces\.func_35852_b)\(', s))
    if not matches: return s
    m = matches[-1]; depth = 1; i = m.end()
    while depth:
        if s[i] == '(': depth += 1
        elif s[i] == ')': depth -= 1
        i += 1
    a = args(s[m.end():i-1]); n = m[1]; c = 'self.component'
    if n == 'StructureStrongholdPieces.func_35852_b': v = '&mut BlockSelector::default()'
    elif n == 'this.func_35013_a': v = f'{c}.liquids({",".join(a)})'
    elif n == 'this.func_35011_a': v = f'{c}.fill({a[0]},{a[1]},[{",".join(a[2:8])}],{",".join(a[8:])})'
    elif n == 'this.func_35022_a': v = f'{c}.fill_stones({a[0]},{a[1]},[{",".join(a[2:8])}],{",".join(a[8:])})'
    elif n == 'this.func_35018_a': v = f'{c}.put({a[0]},{a[6]},{",".join(a[1:6])})'
    elif n == 'this.func_35009_c': v = f'{c}.rotate({",".join(a)})'
    elif n == 'this.func_35033_a': v = f'self.entrance({",".join(a)})'
    elif n == 'this.func_35014_a': v = f'{c}.random_put({",".join(a[:4])},{",".join(a[7:9])},{",".join(a[4:7])})'
    elif n == 'this.func_35010_a': v = f'{c}.random_fill({",".join(a[:4])},[{",".join(a[4:10])}],{",".join(a[10:])})'
    elif n == 'this.func_35003_a': v = f'self.chest({",".join(a)})'
    elif n == 'var2.nextInt': v = f'component::random(w,r).next_int({",".join(a)})'
    else: raise ValueError(n)
    v = re.sub(r'\bvar1\b', 'w', v); v = re.sub(r'\bvar2\b', 'r', v); v = re.sub(r'\bvar3\b', 'clip', v)
    return calls(s[:m.start()] + v + s[i:])

def expr(s, fields):
    s = calls(s)
    for f,v in fields.items(): s = s.replace('this.'+f, 'self.'+v)
    s = s.replace('this.field_35025_h', 'self.component.orientation')
    s = re.sub(r'Block\.(\w+)\.blockID', lambda m:ids[m[1]], s)
    s = re.sub(r'(\d+\.\d+)F', r'\1f32', s)
    s = s.replace('EnumDoor.OPENING', 'Door::Opening')
    s = s.replace('field_35061_c', '&ROOM_LOOT').replace('field_35056_b', '&LIBRARY_LOOT')
    if 'this.' in s or 'func_' in s or 'Block.' in s: raise ValueError(s)
    return s

out = ['//! Literal placement bodies generated from the local Beta 1.8.1 source.', '#![allow(unused_variables, unused_mut, unused_parens)]', 'use super::*;', 'impl StrongholdPiece {']
for kind,cl in enumerate(classes):
    text = (src/f'ComponentStronghold{cl}.java').read_text()
    start = text.index('public boolean func_35023_a(')
    body = text[text.index('{', start)+1:]; body = body[:body.rfind('}')]; body = body[:body.rfind('}')]
    fields = {m[1]:'door' for m in re.finditer(r'(?:protected|private) final EnumDoor (\w+)',text)}
    if cl == 'RightTurn': fields['field_35046_a'] = 'door'
    fields.update({'field_35048_b':'flags[0]', 'field_35049_c':'flags[1]', 'field_35042_b':'flags[0]', 'field_35043_c':'flags[1]', 'field_35040_d':'flags[2]', 'field_35041_e':'flags[3]', 'field_35060_b':'style', 'field_35057_c':'flags[0]', 'field_35036_a':'flags[0]', 'field_35052_a':'length'})
    out.append(f'pub(super) fn generate_{kind}(&mut self,w:&mut World,r:&mut JavaRandom,clip:B)->bool {{')
    for raw in body.splitlines():
        line = raw.strip()
        if not line or line == 'break;': continue
        if line == 'switch(this.field_35060_b) {': continue
        if line.startswith('case '):
            n = int(line[5:-1]); line = ('if' if n == 0 else '} else if') + f' self.style == {n} {{'
        else:
            m = re.fullmatch(r'for\((?:int )?(\w+) = (.*); \1 (<|<=) (.*); (?:\+\+\1|\1 \+= (\d+))\) \{',line)
            if m:
                ran = f'({expr(m[2],fields)})..{"=" if m[3]=="<=" else ""}({expr(m[4],fields)})'
                if m[5]: ran = f'({ran}).step_by({m[5]})'
                line = f'for {m[1]} in {ran} {{'
            elif re.fullmatch(r'int (\w+);',line): line = re.sub(r'int (\w+);',r'let mut \1:i32;',line)
            elif line.startswith(('int ', 'byte ')):
                line = re.sub(r'(?:int|byte) (\w+) = (.*);',lambda m:f'let mut {m[1]}:i32 = {expr(m[2],fields)};',line)
            else: line = expr(line,fields)
            if line.startswith('if('): line = 'if '+line[3:-3]+' {'
            elif line.startswith('} else if('): line = '} else if '+line[10:-3]+' {'
        # Java evaluates the loot count before borrowing Random for the chest call.
        if 'self.chest(' in line:
            begin = line.index('1 + component::random'); end = line.index(')',line.index('.next_int(',begin))+1
            out.append('let loot_count = '+line[begin:end]+';')
            line = line[:begin]+'loot_count'+line[end:]
        out.append(line)
    out.append('}')
out.append('}')
Path('crates/worldgen181/src/structures/stronghold_generated.rs').write_text('\n'.join(out)+'\n')
subprocess.run(['rustfmt', '--edition', '2021', 'crates/worldgen181/src/structures/stronghold_generated.rs'], check=True)
