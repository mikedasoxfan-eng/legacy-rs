"""Turn a McRegion save into the pre-McRegion layout (Alpha / early Beta): one gzip file per
chunk at <base36(x & 63)>/<base36(z & 63)>/c.<base36 x>.<base36 z>.dat, level.dat version 0.
Usage: make_alpha_world.py <mcregion save> <output dir>"""
import gzip
import os
import shutil
import struct
import sys
import zlib


def b36(n):
    digits = '0123456789abcdefghijklmnopqrstuvwxyz'
    if n == 0:
        return '0'
    neg, n = n < 0, abs(n)
    out = ''
    while n:
        out = digits[n % 36] + out
        n //= 36
    return '-' + out if neg else out


def regions(src, dst):
    rdir = os.path.join(src, 'region')
    if not os.path.isdir(rdir):
        return 0
    count = 0
    for name in os.listdir(rdir):
        parts = name.split('.')
        if len(parts) != 4 or parts[0] != 'r' or parts[3] != 'mcr':
            continue
        rx, rz = int(parts[1]), int(parts[2])
        data = open(os.path.join(rdir, name), 'rb').read()
        for i in range(1024):
            off = struct.unpack('>i', data[i * 4:i * 4 + 4])[0]
            if off == 0:
                continue
            sector = off >> 8
            length = struct.unpack('>i', data[sector * 4096:sector * 4096 + 4])[0]
            kind = data[sector * 4096 + 4]
            payload = data[sector * 4096 + 5:sector * 4096 + 4 + length]
            raw = zlib.decompress(payload) if kind == 2 else gzip.decompress(payload)
            cx, cz = rx * 32 + (i % 32), rz * 32 + (i // 32)
            folder = os.path.join(dst, b36(cx & 63), b36(cz & 63))
            os.makedirs(folder, exist_ok=True)
            with open(os.path.join(folder, 'c.%s.%s.dat' % (b36(cx), b36(cz))), 'wb') as f:
                f.write(gzip.compress(raw))
            count += 1
    return count


def set_version_zero(path):
    # level.dat is gzip NBT; find the "version" int tag (TAG_Int=3, name length 7) and zero it.
    raw = bytearray(gzip.decompress(open(path, 'rb').read()))
    key = b'\x03\x00\x07version'
    i = raw.find(key)
    if i >= 0:
        raw[i + len(key):i + len(key) + 4] = b'\x00\x00\x00\x00'
    open(path, 'wb').write(gzip.compress(bytes(raw)))


def main():
    src, dst = sys.argv[1], sys.argv[2]
    shutil.rmtree(dst, ignore_errors=True)
    os.makedirs(dst)
    for name in os.listdir(src):
        p = os.path.join(src, name)
        if os.path.isfile(p) and name.startswith('level.dat'):
            shutil.copy(p, dst)
        elif os.path.isdir(p) and name in ('players', 'data'):
            shutil.copytree(p, os.path.join(dst, name))
    n = regions(src, dst)
    nether = os.path.join(src, 'DIM-1')
    if os.path.isdir(nether):
        n += regions(nether, os.path.join(dst, 'DIM-1'))
    for name in ('level.dat', 'level.dat_old'):
        if os.path.exists(os.path.join(dst, name)):
            set_version_zero(os.path.join(dst, name))
    print('wrote %d chunk files' % n)


main()
