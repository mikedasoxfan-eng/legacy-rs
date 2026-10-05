"""First differing field in a vehicles181 replay (VehicleOracle vs entity::vehicle_verify).
Usage: python tools/b181/diagnose_vehicles181.py java.bin rust.bin ticks
"""
import struct
import sys
from diagnose_replay181 import Replay, Found


class VehicleReplay(Replay):
    def entity(self, frame, index):
        start = self.offset
        super().entity(frame, index)
        self.kinds.append(self.last_kind)

    def utf(self, name):
        value = super().utf(name)
        if name == 'kind': self.last_kind = value
        return value


def main():
    java, rust, ticks = sys.argv[1], sys.argv[2], int(sys.argv[3])
    r = VehicleReplay(open(java, 'rb').read(), open(rust, 'rb').read())
    try:
        r.bytes('header', 8); r.i('ticks')
        for frame in range(ticks + 1):
            r.kinds, ids = [], []
            # remember entity ids by re-reading through the snapshot hook
            r.snapshot(frame)
            r.context = f'frame {frame}, extra'
            carts = r.cart_ids
            for cid in carts:
                for k in range(5):
                    if r.b(f'cart{cid}.rail{k}.present'): r.fields('<d', f'cart{cid}.rail{k}.x cart{cid}.rail{k}.y cart{cid}.rail{k}.z')
            for s in range(r.i('statCount')): r.fields('<i', f'stat{s}.id stat{s}.amount')
            for v in range(r.i('vehicleCount')):
                r.i(f'vehicle{v}.id'); r.bytes(f'vehicle{v}.nbt', r.i(f'vehicle{v}.nbtLength'))
        print(f'byte {r.difference} in final world dump')
    except Found as e:
        print(e)


_orig_i = Replay.i
def _i(self, name):
    v = _orig_i(self, name)
    if name == 'entityCount': self.cart_ids, self._pending = [], None
    if name == 'id' and self.context.startswith('frame') and ', entity ' in self.context: self._pending = v
    return v
Replay.i = _i
_orig_utf = Replay.utf
def _utf(self, name):
    v = _orig_utf(self, name)
    if name == 'kind' and v == 'Minecart' and self._pending is not None and self._pending < 16: self.cart_ids.append(self._pending)
    return v
Replay.utf = _utf

if __name__ == '__main__': main()
