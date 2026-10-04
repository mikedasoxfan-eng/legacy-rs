"""Build disposable Beta saves and play scripts for first-person visual review.

Usage: python tools/firstperson_scene.py target/firstperson-scenes
Then run beta --world-dir <root>/<scene>/world --play-script <root>/<scene>/play.txt.
Only writes beneath the supplied output folder; never opens the user's saves.
"""
import gzip
from pathlib import Path
import struct
import sys
import zlib


def string(s):
    b = s.encode("utf-8")
    return struct.pack(">H", len(b)) + b


def tag(kind, name, value):
    return bytes([kind]) + string(name) + value


def number(kind, name, fmt, value):
    return tag(kind, name, struct.pack(">" + fmt, value))


def compound(name, body):
    return tag(10, name, body + b"\0")


def array(name, value):
    return tag(7, name, struct.pack(">i", len(value)) + value)


def listing(name, kind, values):
    return tag(9, name, bytes([kind]) + struct.pack(">i", len(values)) + b"".join(values))


def item(slot, ident):
    return (number(1, "Slot", "b", slot) + number(2, "id", "h", ident)
            + number(1, "Count", "b", 1) + number(2, "Damage", "h", 0) + b"\0")


def make(root, name, liquid=0, fire=0, bed=False):
    base = root / name
    world = base / "world"
    (world / "region").mkdir(parents=True, exist_ok=True)
    blocks = bytearray(32768)
    metadata = bytearray(16384)
    for x in range(16):
        for z in range(16):
            for y in range(60, 64):
                blocks[(x << 11) | (z << 7) | y] = 1
    for y in (64, 65):
        blocks[(8 << 11) | (11 << 7) | y] = 1
    if bed:
        blocks[(8 << 11) | (11 << 7) | 65] = 0
        for z, data in ((11, 0), (12, 8)):
            i = (8 << 11) | (z << 7) | 64
            blocks[i] = 26
            metadata[i >> 1] |= data << ((i & 1) * 4)
    if liquid:
        for x in range(6, 11):
            for z in range(6, 11):
                for y in range(64, 68):
                    blocks[(x << 11) | (z << 7) | y] = liquid
    heights = bytes(max(y + 1 for y in range(128) if blocks[(x << 11) | (z << 7) | y])
                    for z in range(16) for x in range(16))
    sky = bytearray(16384)
    for x in range(16):
        for z in range(16):
            for y in range(heights[z * 16 + x], 128):
                i = (x << 11) | (z << 7) | y
                sky[i >> 1] |= 15 << ((i & 1) * 4)
    level = (number(3, "xPos", "i", 0) + number(3, "zPos", "i", 0)
             + number(4, "LastUpdate", "q", 0) + number(1, "TerrainPopulated", "b", 1)
             + array("Blocks", blocks) + array("Data", metadata)
             + array("SkyLight", sky) + array("BlockLight", bytes(16384))
             + array("HeightMap", heights) + listing("Entities", 10, [])
             + listing("TileEntities", 10, []))
    compressed = zlib.compress(compound("", compound("Level", level)))
    chunk = struct.pack(">i", len(compressed) + 1) + b"\2" + compressed
    sectors = (len(chunk) + 4095) // 4096
    header = bytearray(8192)
    header[:4] = struct.pack(">I", (2 << 8) | sectors)
    (world / "region" / "r.0.0.mcr").write_bytes(header + chunk.ljust(sectors * 4096, b"\0"))
    inventory = [item(0, 1), item(1, 276), item(2, 358)]
    inventory += [item(100 + i, ident) for i, ident in enumerate((313, 312, 311, 310))]
    position = (8.5, 66.1825, 11.5) if bed else (8.5, 65.62, 8.5)
    player = (listing("Pos", 6, [struct.pack(">d", v) for v in position])
              + listing("Motion", 6, [struct.pack(">d", 0)] * 3)
              + listing("Rotation", 5, [struct.pack(">f", 0)] * 2)
              + number(2, "Health", "h", 20) + number(2, "Air", "h", 300)
              + number(2, "Fire", "h", fire) + number(1, "OnGround", "b", 1)
              + number(5, "FallDistance", "f", 0) + listing("Inventory", 10, inventory)
              + number(3, "Dimension", "i", 0) + number(1, "Sleeping", "b", 0)
              + number(2, "SleepTimer", "h", 0) + number(3, "Score", "i", 0))
    data = (number(4, "RandomSeed", "q", 404) + number(3, "SpawnX", "i", 8)
            + number(3, "SpawnY", "i", 64) + number(3, "SpawnZ", "i", 8)
            + number(4, "Time", "q", 18000 if bed else 6000) + number(4, "SizeOnDisk", "q", 0)
            + number(4, "LastPlayed", "q", 0) + tag(8, "LevelName", string(name))
            + number(3, "version", "i", 19132) + compound("Player", player))
    with gzip.open(world / "level.dat", "wb") as f:
        f.write(compound("", compound("Data", data)))
    (world / "data").mkdir(exist_ok=True)
    colors = bytes(((x // 16 + y // 16) % 14) * 4 + ((x + y) % 4)
                   for y in range(128) for x in range(128))
    map_data = (number(1, "dimension", "b", 0) + number(1, "scale", "b", 0)
                + number(3, "xCenter", "i", 8) + number(3, "zCenter", "i", 8)
                + number(2, "width", "h", 128) + number(2, "height", "h", 128)
                + array("colors", colors))
    with gzip.open(world / "data" / "map_0.dat", "wb") as f:
        f.write(compound("", compound("data", map_data)))
    if bed:
        script = ("0 look 0 400\n0 key Digit4 down\n1 key Digit4 up\n10 shot bed-target\n15 mouse right down\n16 mouse right up\n"
                  "25 shot sleeping-camera\n40 key F5 down\n41 key F5 up\n"
                  "50 shot sleeping-player\n60 quit\n")
    elif liquid or fire:
        script = "8 shot overlay\n20 shot moving-overlay\n22 quit\n"
    else:
        script = ("15 shot held-block-outline\n20 key Digit2 down\n21 key Digit2 up\n"
                  "35 shot sword\n40 key Digit3 down\n41 key Digit3 up\n55 shot map\n"
                  "60 key Digit4 down\n61 key Digit4 up\n75 shot empty-hand\n"
                  "80 mouse left down\n135 shot mining-cracks\n140 mouse left up\n"
                  "145 key Digit2 down\n146 key Digit2 up\n160 key F5 down\n"
                  "161 key F5 up\n170 shot armor-third-person\n180 quit\n")
    (base / "play.txt").write_text(script, encoding="utf-8")


if __name__ == "__main__":
    output = Path(sys.argv[1]).resolve()
    for args in (("equipment",), ("water", 9), ("lava", 11), ("fire", 0, 80), ("bed", 0, 0, True)):
        make(output, *args)
