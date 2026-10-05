"""Mechanical Java -> Rust translation of straight-line RenderBlocks vertex code.

Handles the subset used by the pane/vine/gate renderers: addVertexWithUV calls, if/else
chains on boolean locals, isAirBlock, and (double) casts of the block coordinates. Locals
keep their Java names (var16...). Usage (from Python): translate(lines, coords) -> str.
"""
import re


def expr(e, coords):
    x, y, z = coords
    e = e.strip()
    # (double)(varY + k) / (double)varY
    for jv, rv in ((x, 'x'), (y, 'y'), (z, 'z')):
        e = re.sub(r'\(double\)\(%s ([+-]) (\d+)\)' % jv, r'((%s \1 \2) as f64)' % rv, e)
        e = re.sub(r'\(double\)%s\b' % jv, r'(%s as f64)' % rv, e)
        e = re.sub(r'\b%s\b' % jv, rv, e)
    e = re.sub(r'this\.blockAccess\.isAirBlock\(', 'self.w.is_air(', e)
    e = re.sub(r'(\d+\.\d+)D\b', r'\1', e)
    return e


def split_args(s):
    out, depth, cur = [], 0, ''
    for ch in s:
        if ch == ',' and depth == 0:
            out.append(cur)
            cur = ''
            continue
        if ch == '(':
            depth += 1
        elif ch == ')':
            depth -= 1
        cur += ch
    out.append(cur)
    return out


def translate(lines, coords, tess='var6'):
    out = []
    for raw in lines:
        l = raw.strip()
        ind = '    ' * (2 + len(raw) - len(raw.lstrip('\t')) - 2)
        if not l:
            continue
        m = re.match(r'%s\.addVertexWithUV\((.*)\);$' % tess, l)
        if m:
            args = [expr(a, coords) for a in split_args(m.group(1))]
            out.append(f'{ind}self.t.vertex_uv({", ".join(args)});')
            continue
        m = re.match(r'(\} else )?if\((.*)\) \{$', l)
        if m:
            out.append(f'{ind}{"} else " if m.group(1) else ""}if {expr(m.group(2), coords)} {{')
            continue
        if l in ('}', '} else {'):
            out.append(ind + l)
            continue
        raise ValueError('cannot translate: ' + l)
    return '\n'.join(out)
