"""Compare the chunk data of two McRegion saves: compare_regions.py <save a> <save b>."""
import os, struct, zlib, gzip, sys
def chunks(d):
    out = {}
    r = os.path.join(d, 'region')
    for n in sorted(os.listdir(r)):
        data = open(os.path.join(r, n), 'rb').read()
        rx, rz = map(int, n.split('.')[1:3])
        for i in range(1024):
            off = struct.unpack('>i', data[i*4:i*4+4])[0]
            if off == 0: continue
            s = off >> 8
            ln = struct.unpack('>i', data[s*4096:s*4096+4])[0]
            k = data[s*4096+4]
            p = data[s*4096+5:s*4096+4+ln]
            out[(rx*32+i%32, rz*32+i//32)] = (zlib.decompress(p) if k == 2 else gzip.decompress(p), off)
    return out
a, b = chunks(sys.argv[1]), chunks(sys.argv[2])
same = sum(1 for k in a if k in b and a[k][0] == b[k][0])
layout = sum(1 for k in a if k in b and a[k][1] == b[k][1])
print('java chunks', len(a), 'rust chunks', len(b), 'identical data', same, 'identical sector layout', layout)
print('region files', sorted(os.listdir(os.path.join(sys.argv[1], 'region'))) == sorted(os.listdir(os.path.join(sys.argv[2], 'region'))))
ok = len(a) == len(b) == same > 0 and sorted(os.listdir(os.path.join(sys.argv[1], 'region'))) == sorted(os.listdir(os.path.join(sys.argv[2], 'region')))
sys.exit(0 if ok else 1)
