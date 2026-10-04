"""Write tools/oracle/showcase.txt: a flat test world holding every block id with every
metadata value, plus arrangements where a block's look depends on its neighbours (fences,
redstone, doors, beds, fire, chests, portals, glass and ice, slabs, pistons, snow on grass).
The Java oracle (`rendershowcase`) and `beta --meshcheck ... showcase=FILE` both build it
and mesh it, so every RenderBlocks render type is compared vertex by vertex.

Format: `chunks x0 z0 x1 z1` (the meshed chunks; one more ring is created around them),
then `x y z id meta` per block. Everything else is air above a stone floor at y 63.
Light: sky (x*7 + y*3 + z*5) & 15, block (x*3 + y*11 + z) & 15 on every position.
"""
import os

blocks = []


def put(x, y, z, block, meta=0):
    blocks.append((x, y, z, block, meta))


# Every id with every metadata value, standing alone.
for block in range(1, 97):
    for meta in range(16):
        put(2 * block, 64, 2 * meta, block, meta)

X = 200
# Fences: a plus and an L.
for x, z in [(0, 2), (1, 2), (2, 2), (1, 1), (1, 3), (4, 0), (4, 1), (4, 2), (5, 2), (6, 2)]:
    put(X + x, 64, z, 85)
# Redstone: a powered patch with torches and repeaters, and wire climbing a block.
for x in range(10, 15):
    for z in range(0, 5):
        put(X + x, 64, z, 55, (x * 3 + z * 5) % 16)
put(X + 15, 64, 2, 76, 5)
put(X + 15, 64, 0, 75, 5)
put(X + 12, 64, 5, 93, 0)
put(X + 13, 64, 5, 94, 6)
put(X + 12, 64, 7, 1)
put(X + 12, 65, 7, 55, 9)
put(X + 12, 64, 8, 55, 15)
put(X + 11, 64, 7, 55, 3)
put(X + 13, 65, 7, 55, 0)
# Doors: lower and upper halves for each lower metadata.
for d in range(8):
    for block, z in [(64, 10), (71, 13)]:
        put(X + 2 * d, 64, z, block, d)
        put(X + 2 * d, 65, z, block, d | 8)
# Beds: foot and head for each direction (BlockBed.headBlockToFootBlockMap).
for d, (dx, dz) in enumerate([(0, 1), (-1, 0), (0, -1), (1, 0)]):
    fx, fz = X + 20 + 4 * d, 10
    put(fx, 64, fz, 26, d)
    put(fx + dx, 64, fz + dz, 26, d | 8)
    put(fx + 1, 64, fz + 3, 26, d | 4)
# Fire: floating with flammable neighbours on each side, on netherrack, over leaves.
for i, offsets in enumerate([[(-1, 0, 0)], [(1, 0, 0)], [(0, 0, -1)], [(0, 0, 1)], [(0, 1, 0)],
                             [(-1, 0, 0), (1, 0, 0), (0, 0, -1), (0, 0, 1), (0, 1, 0)], []]):
    fx, fz = X + 40 + 3 * i, 2
    put(fx, 70, fz, 51, i)
    for dx, dy, dz in offsets:
        put(fx + dx, 70 + dy, fz + dz, 5)
put(X + 40, 64, 6, 87)
put(X + 40, 65, 6, 51, 3)
put(X + 43, 66, 6, 18)
put(X + 43, 67, 6, 51, 7)
put(X + 46, 64, 6, 35, 4)
put(X + 46, 65, 6, 51, 15)
# Chests: double along x and along z, single, locked chest.
for x, z in [(60, 0), (61, 0), (64, 0), (64, 1), (68, 0)]:
    put(X + x, 64, z, 54)
put(X + 70, 64, 0, 95)
# Portals in obsidian frames along x and along z.
for x in range(-1, 3):
    for y in range(-1, 4):
        frame = x in (-1, 2) or y in (-1, 3)
        put(X + 75 + x, 65 + y, 2, 49 if frame else 90)
        put(X + 82, 65 + y, 2 + x, 49 if frame else 90)
# Glass and ice clusters, touching each other.
for x in range(3):
    for z in range(3):
        put(X + 86 + x, 64, z, 20)
        put(X + 86 + x, 65, z, 20 if (x + z) % 2 else 79)
        put(X + 89 + x, 64, z, 79)
# Slabs: next to each other, on top of each other, beside a double slab.
for i, meta in enumerate(range(4)):
    put(X + 94 + i, 64, 0, 44, meta)
put(X + 94, 65, 0, 44, 1)
put(X + 98, 64, 0, 43, 2)
put(X + 94, 64, 1, 44, 3)
# Pistons: extended bases with their heads, every facing; sticky too.
for f, (dx, dy, dz) in enumerate([(0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1), (-1, 0, 0), (1, 0, 0)]):
    for k, base in enumerate([33, 29]):
        bx, by, bz = X + 102 + 3 * f, 66, 2 + 4 * k
        put(bx, by, bz, base, f | 8)
        put(bx + dx, by + dy, bz + dz, 34, f | (8 if base == 29 else 0))
# Snow on grass, grass under other blocks, farmland and stairs beside light.
put(X + 122, 64, 0, 2)
put(X + 122, 65, 0, 78, 0)
put(X + 123, 64, 0, 2)
put(X + 123, 65, 0, 80)
put(X + 124, 64, 0, 60, 7)
put(X + 125, 64, 0, 53, 1)
put(X + 126, 64, 0, 67, 2)
put(X + 124, 64, 1, 89)
# Torches, levers and buttons against walls, ladders on walls.
for m, (dx, dz) in [(1, (-1, 0)), (2, (1, 0)), (3, (0, -1)), (4, (0, 1))]:
    for k, block in enumerate([50, 76, 69, 77, 65]):
        wx, wz = X + 130 + 3 * m, 2 + 3 * k
        put(wx + dx, 64, wz + dz, 1)
        put(wx, 64, wz, block, m if block != 65 else m + 1)

xmax = max(b[0] for b in blocks)
zmax = max(b[2] for b in blocks)
out = os.path.join(os.path.dirname(os.path.abspath(__file__)), "showcase.txt")
with open(out, "w", newline="\n") as f:
    f.write(f"chunks 0 0 {xmax >> 4} {zmax >> 4}\n")
    for b in blocks:
        f.write("%d %d %d %d %d\n" % b)
print(out, len(blocks), "blocks, chunks 0..%d x 0..%d" % (xmax >> 4, zmax >> 4))
