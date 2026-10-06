"""Name the first field in the round-3 geometry fixture, without copying game logic."""
import struct
import sys
from pathlib import Path

def geometry(path):
    data = Path(path).read_bytes()
    at = 0
    def read(fmt):
        nonlocal at
        value = struct.unpack_from('>' + fmt, data, at)[0]
        at += struct.calcsize(fmt)
        return value
    def box(name):
        present = read('?')
        yield name + '.present', present
        if present:
            for field in ['minX', 'minY', 'minZ', 'maxX', 'maxY', 'maxZ']:
                yield name + '.' + field, read('d')
    for block in [85,110,111,112,113,114,115,116,117,118,119,120,121,122]:
        for meta in range(16):
            for neighbor in [0,1,20,54,85,86,107,113,117,118]:
                case = f'block={block} meta={meta} neighbor={neighbor}'
                yield from box(case + ' selection')
                yield from box(case + ' collision')
                count = read('i')
                yield case + ' boxes.count', count
                for index in range(count):
                    yield from box(case + f' boxes[{index}]')
                for side in range(6):
                    yield case + f' texture[{side}]', read('i')
    for height in [64,65,128]:
        for y in [-1,0,63,64,65,127,128]:
            for below in [0,1,8,9,88]:
                for meta in range(2):
                    for block in [111,115]:
                        yield f'height={height} y={y} below={below}:{meta} block={block} stay', read('?')
    assert at == len(data)

if __name__ == '__main__':
    for java, rust in zip(geometry(sys.argv[1]), geometry(sys.argv[2])):
        if java != rust:
            print('Java:', java)
            print('Rust:', rust)
            break
    else:
        print('Every named field matches')
