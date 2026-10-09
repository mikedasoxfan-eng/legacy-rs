"""Print block id/metadata at positions from an oracle world dump (B173WLD1).
Usage: python peek_world.py dump.bin x,y,z [x,y,z ...]"""
import struct
import sys


def load(path):
    d = open(path, 'rb').read()
    assert d[:8] == b'B173WLD1', 'not a world dump'
    n = struct.unpack_from('<i', d, 20)[0]
    o = 24
    chunks = {}
    for _ in range(n):
        cx, cz = struct.unpack_from('<ii', d, o)
        o += 9
        blocks = d[o:o + 32768]
        o += 32768
        data = d[o:o + 16384]
        o += 16384 * 3 + 256
        nte = struct.unpack_from('<i', d, o)[0]
        o += 4
        for _ in range(nte):
            o += 12
            kind = d[o]
            o += 1
            if kind == 1:
                o += 27 * 5
            elif kind == 2:
                o += 1 + d[o]
            elif kind == 3:
                o += 3 * 5 + 4
            elif kind == 4:
                o += 9 * 5
        chunks[(cx, cz)] = (blocks, data)
    return chunks


def at(chunks, x, y, z):
    blocks, data = chunks[(x >> 4, z >> 4)]
    i = ((x & 15) << 11) | ((z & 15) << 7) | y
    meta = data[i >> 1] >> (4 * (i & 1)) & 15
    return blocks[i], meta


if __name__ == '__main__':
    c = load(sys.argv[1])
    for p in sys.argv[2:]:
        x, y, z = map(int, p.split(','))
        print(p, '-> id %d meta %d' % at(c, x, y, z))
