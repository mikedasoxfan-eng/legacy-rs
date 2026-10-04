"""Deterministic recipes, offsets/mirrors/wildcards and container click replays."""
from pathlib import Path
import random
import struct

root = Path(__file__).resolve().parents[2]
data = memoryview((root / 'crates/worldgen/src/container/recipes.bin').read_bytes())
offset = 0
def byte():
    global offset
    v = data[offset]; offset += 1; return v
def integer():
    global offset
    v = struct.unpack_from('>i', data, offset)[0]; offset += 4; return v
def stack():
    return (integer(), integer(), integer()) if byte() else (-1, 0, 0)

lines = []
def emit(*args): lines.append(' '.join(map(str, args)))
def put(i, s): emit('put', i, *s)
n = integer()
for ri in range(n):
    shaped, w, h, result = byte(), integer(), integer(), stack()
    ingredients = [stack() for _ in range(integer())]
    for mirror in [False, True]:
        for ox, oy in [(0, 0), (3-w, 3-h)] if shaped else [(0, 0)]:
            emit('open', 1)
            for i, s in enumerate(ingredients):
                if s[0] < 0: continue
                x, y = (i % w, i // w) if shaped else (i % 3, i // 3)
                if mirror and shaped: x = w - x - 1
                if not shaped and mirror: x, y = 2-x, 2-y
                # Three batches exercise the shift recursion/replenishment bug.
                put(1 + x + ox + (y+oy)*3, (s[0], 3, 2 if s[2] == -1 else s[2]))
            emit('expect', *result)
            emit('click', 0, 0, 0)
            emit('cursor', -1, 0, 0)
            emit('click', 0, 1, 1)
            emit('close')

rng = random.Random(173)
ids = [1, 5, 6, 17, 35, 86, 263, 264, 265, 280, 281, 282, 296, 298, 299, 300, 301, 310, 311, 312, 313, 325, 327, 335, 351, 354]
for kind, size in enumerate([45, 46, 63, 90, 39, 45]):
    emit('open', kind)
    for t in range(10000):
        op = rng.randrange(12)
        if op < 2:
            ident = rng.choice(ids)
            put(rng.randrange(size), (ident, rng.randrange(1, 65), rng.randrange(4) if ident in [6,17,35,263,351] else 0))
        elif op == 2:
            ident = rng.choice(ids)
            emit('cursor', ident, rng.randrange(1, 65), rng.randrange(4) if ident in [6,17,35,263,351] else 0)
        elif op == 3: emit('cursor', -1, 0, 0)
        else: emit('click', -999 if t % 29 == 0 else rng.randrange(size), rng.randrange(3), rng.randrange(2))
    emit('close')
path = root / 'target/container-cases.txt'
path.write_text('\n'.join(lines) + '\n')
print(f'{n} recipes; {len(lines)} container replay records')
