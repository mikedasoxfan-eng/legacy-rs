"""Release RenderBlocks fixtures: every id/meta, neighbours, negative positions.

Retain the inherited mixed-block showcase, then exercise release types with all
six adjacent faces, AO diagonals, connected fences, vines, and both door halves.
"""
from pathlib import Path
import sys

root = Path(__file__).resolve().parents[2]
blocks = [tuple(map(int, line.split())) for line in
          (root / "tools/oracle10/showcase.txt").read_text().splitlines()[1:] if line.strip()]


def put(x, y, z, block, meta=0):
    blocks.append((x, y, z, block, meta))


# All release ids, including the changed old block renderers, without neighbours.
for block in range(1, 123):
    for meta in range(16):
        put(-4 * block, 64, 4 * meta, block, meta)

changed = [8, 9, 10, 11, 18, 44, 51, 55, 64, 71, 85, 96, 101, 102, 106,
           107, 111, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122]
directions = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)]
for index, block in enumerate(changed):
    for meta in range(16):
        x, z = -4 - 4 * index, 72 + 4 * meta
        put(x, 66, z, block, meta)
        for side, (dx, dy, dz) in enumerate(directions):
            # Rotate full/inset/transparent/matching neighbours through every face.
            other = [1, 44, 20, block][(meta + side) % 4]
            put(x + dx, 66 + dy, z + dz, other, meta)
        for dx, dz in [(-1, -1), (-1, 1), (1, -1), (1, 1)]:
            put(x + dx, 67 if meta & 1 else 65, z + dz, 1 if meta & 2 else 20)

# Door upper metadata now independently determines hinge and powered state.
for block in [64, 71]:
    for lower in range(8):
        for upper in range(8, 12):
            x, z = -4 - lower * 4, 144 + (upper - 8) * 4 + (block == 71) * 20
            put(x, 64, z, block, lower)
            put(x, 65, z, block, upper)

x0 = min(b[0] for b in blocks) >> 4
x1 = max(b[0] for b in blocks) >> 4
z0 = min(b[2] for b in blocks) >> 4
z1 = max(b[2] for b in blocks) >> 4
path = Path(sys.argv[1])
path.parent.mkdir(parents=True, exist_ok=True)
with path.open("w", newline="\n") as output:
    output.write(f"chunks {x0} {z0} {x1} {z1}\n")
    output.writelines("%d %d %d %d %d\n" % block for block in blocks)
print(f"Release RenderBlocks: {len(blocks)} blocks; {x0}..{x1} x {z0}..{z1} chunks")
