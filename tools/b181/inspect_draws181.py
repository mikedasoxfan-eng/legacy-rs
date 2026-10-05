"""Inspect Mode_drawintegration181 sidecars without dropping packed light.

Usage: python tools/b181/inspect_draws181.py path.bin.draws181 [...]
The legacy companion contains case inputs. This file contains the actual GL
submissions (including display-list definitions) with their primitive mode,
first vertex, enabled arrays, texture, GL color, and eight words per vertex.
Disabled vertex attributes are zero. This validates acquisition, not Rust parity.
"""
import collections
import json
import struct
import sys
from pathlib import Path


def inspect(path):
    data = Path(path).read_bytes()
    if data[:8] != b'B181DRW1':
        raise ValueError(f'{path}: wrong capture header')
    count, = struct.unpack_from('<I', data, 8)
    at = 12
    modes = collections.Counter()
    vertices = light_draws = light_vertices = nonzero_light = 0
    for _ in range(count):
        mode, first, mask, n, texture, *color = struct.unpack_from('<9I', data, at)
        at += 36
        if mode not in (1, 3, 4, 7) or mask & ~15:
            raise ValueError(f'{path}: invalid mode/mask at {at - 36}')
        if mode == 7 and n % 4:
            raise ValueError(f'{path}: incomplete quad at {at - 36}')
        if first != 0:
            raise ValueError(f'{path}: unexpected partial submission')
        modes[mode] += 1
        vertices += n
        if mask & 8:
            light_draws += 1
            light_vertices += n
        for i in range(n):
            words = struct.unpack_from('<8I', data, at)
            for bit, indices in ((1, (3, 4)), (2, (5,)), (4, (6,)), (8, (7,))):
                if not mask & bit and any(words[j] for j in indices):
                    raise ValueError(f'{path}: disabled array contains data')
            nonzero_light += bool(mask & 8 and words[7])
            at += 32
    if at != len(data):
        raise ValueError(f'{path}: {len(data) - at} trailing bytes')
    return dict(path=str(path), submissions=count, primitives=dict(modes),
                vertices=vertices, packed_light_submissions=light_draws,
                packed_light_vertices=light_vertices, nonzero_packed_light=nonzero_light)


if __name__ == '__main__':
    for path in sys.argv[1:]:
        print(json.dumps(inspect(path)))
