"""Generate Java8-compatible Unicode6.1 Bidi classes from licensed Unicode data."""
from pathlib import Path
import hashlib
import argparse
ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / 'tools/r11/data/DerivedBidiClass-6.1.0.txt'
NAMES = 'L R EN ES ET AN CS B S WS ON LRE LRO AL RLE RLO PDF NSM BN'.split()
def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--verify-java-data', type=Path)
    args = parser.parse_args()
    data = bytearray(0x110000)
    for line in SOURCE.read_text(encoding='utf-8').splitlines():
        line = line.split('#')[0].strip()
        if not line:
            continue
        points, cls = map(str.strip, line.split(';'))
        bounds = points.split('..')
        start, end = int(bounds[0],16), int(bounds[-1],16)
        data[start:end+1] = bytes([NAMES.index(cls)]) * (end-start+1)
    if args.verify_java_data:
        original = args.verify_java_data.read_bytes()
        if original != data:
            raise SystemExit('Unicode6.1 classes differ from pinned JVM')
        print('BIDI CLASS DATA EXACT: 1114112 code points')
    ranges = []
    start = 0
    for point in range(1,len(data)+1):
        if point == len(data) or data[point] != data[start]:
            if data[start]:
                ranges.append((start,point-1,data[start]))
            start = point
    header = '// Generated from Unicode 6.1.0 DerivedBidiClass.txt.\n// Copyright 1991-2011 Unicode, Inc.; see tools/r11/data/UNICODE-LICENSE.txt.\n// Source SHA256: ' + hashlib.sha256(SOURCE.read_bytes()).hexdigest() + '\n'
    output = header + 'pub(super) const RANGES:&[(u32,u32,u8)]=&[\n' + ''.join(f' (0x{a:X},0x{b:X},{c}),\n' for a,b,c in ranges) + '];\n'
    (ROOT/'crates/client11/src/gui/bidi_data11.rs').write_text(output,encoding='utf-8')
if __name__ == '__main__':
    main()
