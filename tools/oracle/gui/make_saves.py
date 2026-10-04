"""Write a fixed set of save folders (level.dat only) for the GUI screenshot scenes."""
import gzip, os, struct, sys

def tag_string(name, s):
    b = s.encode('utf-8')
    return bytes([8]) + struct.pack('>H', len(name)) + name.encode() + struct.pack('>H', len(b)) + b

def tag_long(name, v):
    return bytes([4]) + struct.pack('>H', len(name)) + name.encode() + struct.pack('>q', v)

def tag_int(name, v):
    return bytes([3]) + struct.pack('>H', len(name)) + name.encode() + struct.pack('>i', v)

def compound(name, body):
    return bytes([10]) + struct.pack('>H', len(name)) + name.encode() + body + b'\x00'

WORLDS = [
    # folder, LevelName, LastPlayed (ms), SizeOnDisk, version
    ("World1", "My First World", 1302652800000, 5242880, 19132),
    ("Desert", "Desert Survival", 1310000000000, 123456789, 19132),
    ("Old", "", 1290000000000, 1048576, 0),
    ("World2", "World2", 1310000000000, 0, 19132),
]

def main(out):
    for folder, name, played, size, version in WORLDS:
        d = os.path.join(out, folder)
        os.makedirs(d, exist_ok=True)
        data = (tag_long("RandomSeed", 42) + tag_int("SpawnX", 0) + tag_int("SpawnY", 64) + tag_int("SpawnZ", 0)
                + tag_long("Time", 0) + tag_long("SizeOnDisk", size) + tag_long("LastPlayed", played)
                + tag_string("LevelName", name) + tag_int("version", version))
        root = compound("", compound("Data", data))
        with gzip.open(os.path.join(d, "level.dat"), "wb") as f:
            f.write(root)

if __name__ == "__main__":
    main(sys.argv[1])
