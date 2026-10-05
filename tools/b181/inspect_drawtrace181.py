"""Validate complete logical renderer captures without dropping light or modes.

The legacy companion is checked byte for byte against the trace's common fields.
This proves fixture consistency, not Rust renderer/lightmap parity. The particle
companion describes actual EffectRenderer batches rather than individual FX calls.
"""
import collections
import json
import struct
import sys
from pathlib import Path


class Reader:
    def __init__(self, path):
        self.path, self.data, self.at = str(path), Path(path).read_bytes(), 0

    def take(self, size):
        if size < 0 or self.at + size > len(self.data):
            raise ValueError(f'{self.path}: truncated at byte {self.at}, need {size}')
        result = self.data[self.at:self.at + size]
        self.at += size
        return result

    def u(self):
        return struct.unpack('<I', self.take(4))[0]

    def end(self):
        if self.at != len(self.data):
            raise ValueError(f'{self.path}: {len(self.data) - self.at} trailing bytes')


def traces(path):
    r = Reader(path)
    if r.take(8) != b'B181TRC1':
        raise ValueError(f'{path}: wrong trace header')
    draws = []
    for index in range(r.u()):
        mode, mask, n = r.u(), r.u(), r.u()
        if mode not in (1, 3, 4, 7) or mask & ~15 or n == 0:
            raise ValueError(f'{path}: invalid mode/mask/count in draw {index}')
        if mode == 7 and n % 4 or mode == 4 and n % 3 or mode == 1 and n % 2:
            raise ValueError(f'{path}: incomplete primitive in draw {index}')
        header = r.take(48)
        texture = struct.unpack_from('<I', header)[0]
        flags = struct.unpack_from('<I', header, 32)[0]
        if (flags & 512 != 0) != (mask & 1 != 0) or (flags & 64 != 0) != (mask & 2 != 0) or (flags & 128 != 0) != (mask & 4 != 0):
            raise ValueError(f'{path}: inconsistent enabled arrays in draw {index}')
        lightmap = r.take(8)
        ops = r.u()
        transforms = r.take(ops * 20)
        for at in range(0, len(transforms), 20):
            if struct.unpack_from('<I', transforms, at)[0] not in (0, 1, 2):
                raise ValueError(f'{path}: invalid transform in draw {index}')
        vertices = r.take(n * 32)
        projected = bytearray(header + struct.pack('<I', ops) + transforms + struct.pack('<I', n))
        lights = []
        for at in range(0, len(vertices), 32):
            words = struct.unpack_from('<8I', vertices, at)
            for bit, indices in ((1, (3, 4)), (2, (5,)), (4, (6,)), (8, (7,))):
                if not mask & bit and any(words[j] for j in indices):
                    raise ValueError(f'{path}: disabled attribute has data in draw {index}')
            projected.extend(vertices[at:at + 28])
            lights.append(words[7])
        draws.append(dict(mode=mode, mask=mask, n=n, texture=texture,
                          lightmap=lightmap, lights=lights, projected=projected))
    r.end()
    return draws


def legacy_draw(r):
    start = r.at
    r.take(48)
    r.take(r.u() * 20)
    r.take(r.u() * 28)
    return r.data[start:r.at]


def inspect(path):
    draws = traces(str(path) + '.trace181')
    r = Reader(path)
    magic = r.take(8)
    at = 0
    if magic == b'B181PKL1':
        count = r.u()
        for case in range(count):
            sky, block, kind, age = r.u(), r.u(), r.u(), r.u()
            partial = struct.unpack('<f', r.take(4))[0]
            n = r.u()
            batch = draws[at:at + n]
            if not batch or len(batch) != n:
                raise ValueError(f'{path}: missing pickup draws in case {case}')
            expected = struct.pack('<ff', block * 16, sky * 16)
            if any(d['lightmap'] != expected for d in batch):
                raise ValueError(f'{path}: wrong pickup lightmap in case {case}: sky={sky}, block={block}, kind={kind}, age={age}, partial={partial}')
            if kind == 1 and any(d['projected'][4:20] != struct.pack('<4f', 1, 1, 1, 1) for d in batch):
                raise ValueError(f'{path}: arrow pickup material is not white in case {case}')
            at += n
        boundary = 'actual EntityPickupFX current unit-1 coordinates and arrow material'
    elif magic == b'B181PIP1':
        count = r.u()
        for case in range(count):
            sky, block = r.u(), r.u()
            partial = struct.unpack('<f', r.take(4))[0]
            n = r.u()
            batch = draws[at:at + n]
            if n != 4 or len(batch) != n or [d['texture'] for d in batch] != [17, 1, 2, 3]:
                raise ValueError(f'{path}: wrong particle layer batches in case {case}')
            if [d['n'] for d in batch] != [56, 4, 8, 4] or any(d['mode'] != 7 for d in batch):
                raise ValueError(f'{path}: wrong original particle geometry in case {case}')
            # The batch sets brightness on each particle; check ordinary FX and
            # both item particles against the supplied nibble maps. Flame/lava/
            # portal have original brightness overrides and retain their words.
            expected = sky * 16 << 16 | block * 16
            for d in batch[:3]:
                if not d['mask'] & 8:
                    raise ValueError(f'{path}: missing pipeline packed-light array')
            for d, indices in ((batch[0], range(4)), (batch[1], range(4)), (batch[2], range(8))):
                if any(d['lights'][i] != expected for i in indices):
                    raise ValueError(f'{path}: wrong light for sky={sky}, block={block}, partial={partial}')
            # Footsteps start a new batch internally, clearing the light array
            # just set by EffectRenderer's layer-3 callback. Keep vanilla behavior.
            if batch[3]['mask'] & 8:
                raise ValueError(f'{path}: footprint unexpectedly retained packed-light array')
            at += n
        boundary = 'actual EffectRenderer batches'
    else:
        if magic[:4] != b'R173' or struct.unpack_from('<I', magic, 4)[0] not in (1, 2):
            raise ValueError(f'{path}: unknown companion format')
        count = r.u()
        for case in range(count):
            r.take(108)  # unchanged RendererOracle/ParticleOracle case inputs
            n = r.u()
            for draw in range(n):
                if at >= len(draws) or legacy_draw(r) != draws[at]['projected']:
                    raise ValueError(f'{path}: companion disagrees at case {case}, draw {draw}')
                at += 1
        boundary = 'legacy companion fields, including display-list playback'
    r.end()
    if at != len(draws):
        raise ValueError(f'{path}: {len(draws) - at} unmatched trace draws')
    return dict(path=str(path), cases=count, logical_draws=len(draws),
                vertices=sum(d['n'] for d in draws),
                primitives=dict(collections.Counter(d['mode'] for d in draws)),
                packed_light_draws=sum(bool(d['mask'] & 8) for d in draws),
                nonzero_packed_light_vertices=sum(sum(bool(v) for v in d['lights']) for d in draws),
                nonzero_current_lightmap_draws=sum(d['lightmap'] != bytes(8) for d in draws),
                verified_boundary=boundary)


if __name__ == '__main__':
    if len(sys.argv) < 2:
        raise SystemExit('Usage: inspect_drawtrace181.py companion.bin [...]')
    for path in sys.argv[1:]:
        print(json.dumps(inspect(path)))
    print('capture consistency EXACT (fixture integrity; Rust renderer parity is separate)')
