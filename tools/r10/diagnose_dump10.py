"""Locate a first difference using replay schemas or opt-in Java fixture tracing.

Usage: python tools/r10/diagnose_dump10.py JAVA.bin RUST.bin
Pass --oracle-args MODE ... when the dump predates argument sidecars. The trace
is checked against the existing dump: non-repeatable fixtures are reported.
"""
import argparse
import contextlib
import io
import json
import os
import re
import subprocess
import struct
import hashlib
from pathlib import Path

from diagnose_replay10 import Replay, Found

ROOT = Path(__file__).resolve().parents[2]


class JavaRandom:
    def __init__(self, seed): self.seed = (seed ^ 0x5DEECE66D) & ((1 << 48) - 1)
    def bits(self, count):
        self.seed = (self.seed * 0x5DEECE66D + 11) & ((1 << 48) - 1)
        return self.seed >> (48-count)
    def integer(self, bound):
        if bound & (bound-1) == 0: return (bound * self.bits(31)) >> 31
        while True:
            bits = self.bits(31); value = bits % bound
            if bits-value+bound-1 < 1 << 31: return value


def inventory_difference(java, rust):
    replay = Replay(java,rust); random = JavaRandom(1729)
    try:
        for record in range(10000):
            op=random.integer(7);random.integer(13);random.integer(80);random.integer(150)
            replay.context=f'inventory operation {record}, opcode {op}'
            if op in {0,1,3}: replay.fields('<i','resultStack.id resultStack.count resultStack.damage resultStack.animations')
            if op == 3: random.integer(40)
            if op == 5: random.integer(4);random.integer(20)
            replay.i('result')
            for slot in range(36): replay.fields('<i',f'main[{slot}].id main[{slot}].count main[{slot}].damage main[{slot}].animations')
            for slot in range(4): replay.fields('<i',f'armor[{slot}].id armor[{slot}].count armor[{slot}].damage armor[{slot}].animations')
            replay.fields('<i','currentItem totalArmorValue')
    except Found as found: return str(found)


def block_difference(java,rust):
    replay=Replay(java,rust)
    try:
        for record in range(10000):
            replay.context=f'block geometry case {record}'
            for box in range(replay.i('collisionBoxCount')): replay.fields('<d',f'box[{box}].minX box[{box}].minY box[{box}].minZ box[{box}].maxX box[{box}].maxY box[{box}].maxZ')
            side=replay.i('hitSide')
            if side != -1:
                replay.fields('<i','hitBlockX hitBlockY hitBlockZ');replay.fields('<d','hitVecX hitVecY hitVecZ')
    except Found as found: return str(found)


def replay_difference(mode, scenario, ticks, java, rust):
    replay = Replay(java, rust)
    try:
        if mode == 'vehicles':
            replay.bytes('vehicle header', 8); replay.i('ticks')
        for frame in range(int(ticks) + 1):
            replay.snapshot(frame); replay.extras(mode, scenario, frame)
        return f'byte {replay.difference} in final world dump (starts at {replay.offset})'
    except Found as found:
        return str(found)


def playertick_difference(java, rust):
    replay = Replay(java,rust)
    frame=0
    try:
        while replay.offset < len(java):
            replay.context=f'frame {frame}'
            count=replay.i('payloadLength');end=replay.offset+count
            if replay.difference < end:
                replay.fields('<d','posX posY posZ motionX motionY motionZ prevPosX prevPosY prevPosZ boxMinX boxMinY boxMinZ boxMaxX boxMaxY boxMaxZ')
                replay.fields('<f','yaw pitch prevYaw prevPitch width height yOffset ySize stepHeight fallDistance distanceWalked')
                replay.fields('<i','ticksExisted fire air')
                replay.fields('<B','onGround dead inWater beenAttacked isAirBorne')
                replay.fields('<i','health prevHealth heartsLife hurtTime maxHurtTime deathTime attackTime entityAge naturalArmorRating field35171bJ field35172bP field35173bQ')
                replay.fields('<f','attackedAtYaw renderYawOffset prevRenderYawOffset field705Q field704R field703S moveStrafing moveForward randomYawVelocity moveSpeed landMovementFactor jumpMovementFactor')
                replay.b('jumping')
                for potion in range(replay.i('potionCount')): replay.fields('<i',f'potion{potion}.id potion{potion}.duration potion{potion}.amplifier')
                replay.fields('<q','entityRand worldRand mathRand')
                return None
            replay.offset=end;frame+=1
    except Found as found: return str(found)
    return None


def terrain_difference(mode, arguments, java, rust, offset):
    """Fixed terrain envelopes: report a coordinate instead of an opaque byte."""
    prefix = 0
    context = ''
    if mode == 'terrainround2' and arguments[1] == 'lifetime':
        # Four feature packets, each followed by variable UTF entity records.
        # The first unequal byte still has the same prefix in both files.
        cursor = 0
        for record in range(4):
            packet_end = cursor + 17 + 49*49152
            if offset < packet_end:
                context = f'lifetime world[{record}], '
                if offset < cursor+17:
                    names = [(cursor, 1, 'generationResult', '>B'), (cursor+1,8,'generatorRand','>q'), (cursor+9,8,'worldRand','>q')]
                    for start,size,name,fmt in names:
                        if start <= offset < start+size:
                            return f'byte {offset}, {context}{name}: Java {struct.unpack_from(fmt,java,start)[0]}, Rust {struct.unpack_from(fmt,rust,start)[0]}'
                prefix = cursor+17
                break
            if packet_end+4 > len(java): return None
            count = struct.unpack_from('>i',java,packet_end)[0]
            cursor = packet_end+4
            if offset < cursor:
                return f'byte {offset}, lifetime world[{record}].loadedEntityCount: Java {count}, Rust {struct.unpack_from(">i",rust,packet_end)[0]}'
            for entity in range(count):
                length = struct.unpack_from('>H',java,cursor)[0]
                end = cursor+2+length+32
                if offset < end:
                    return f'byte {offset}, lifetime world[{record}].entity[{entity}] record byte {offset-cursor}: Java {java[offset]}, Rust {rust[offset]}'
                cursor = end
        else: return None
        chunk, local = divmod(offset-prefix,49152)
        cx,cz = chunk//7-3,chunk%7-3
    elif mode == 'features10' or mode == 'terrainround2' and arguments[1] in {'sky','dungeon'}:
        replay = Replay(java, rust); replay.context = 'feature fixture'
        try:
            replay.b('generationResult'); replay.field('generatorRand', '>q'); replay.field('worldRand', '>q')
        except Found as found: return str(found)
        prefix = 17; chunk, local = divmod(offset-prefix, 49152)
        if chunk >= 49: return None
        cx, cz = chunk//7-3, chunk%7-3
    elif mode == 'terrain' and arguments[1] in {'raw','sky'}:
        x,z,width,height = map(int, arguments[3:7])
        chunk, local = divmod(offset,32768);cx,cz=x+chunk//height,z+chunk%height
    elif mode == 'terrain' and arguments[1] == 'layers':
        x,z,width,height = map(int,arguments[3:7]);layer,index=divmod(offset//4,width*height)
        start=offset//4*4
        if start+4 <= min(len(java),len(rust)):
            j=struct.unpack_from('>i',java,start)[0];r=struct.unpack_from('>i',rust,start)[0]
            return f'byte {offset}, GenLayer result[{layer}] at ({x+index%width},{z+index//width}): Java {j}, Rust {r}'
        return f'byte {offset}, GenLayer result array length: Java {len(java)//4}, Rust {len(rust)//4} cells'
    else: return None
    if offset >= min(len(java),len(rust)): return f'byte {offset}, terrain packet length: Java {len(java)}, Rust {len(rust)} bytes'
    if local < 32768:
        bx,bz,y=local>>11,(local>>7)&15,local&127
        return f'byte {offset}, {context}chunk ({cx},{cz}).block ({bx},{y},{bz}): Java {java[offset]}, Rust {rust[offset]}'
    local-=32768;index=local*2
    bx,bz,y=index>>11,(index>>7)&15,index&127
    nibble=0 if java[offset]&15 != rust[offset]&15 else 1
    shift=nibble*4
    return f'byte {offset}, {context}chunk ({cx},{cz}).metadata ({bx},{y+nibble},{bz}): Java {(java[offset]>>shift)&15}, Rust {(rust[offset]>>shift)&15}'


def event_difference(java, rust, offset):
    """Decode original WorldEventOracle sound payloads embedded in fixture dumps."""
    for start in range(max(0, offset-280), offset+1):
        if start+5>min(len(java),len(rust)) or java[start]!=0 or rust[start]!=0: continue
        jl=struct.unpack_from('<i',java,start+1)[0];rl=struct.unpack_from('<i',rust,start+1)[0]
        if not 1<jl<256 or not 1<rl<256: continue
        je=start+5+jl;re=start+5+rl
        if je+32>len(java) or re+32>len(rust): continue
        j=java[start+5:je];r=rust[start+5:re]
        prefixes=(b'random.',b'mob.',b'damage.',b'ambient.',b'dig.',b'step.')
        if not j.startswith(prefixes) or not r.startswith(prefixes) or not all(32<=b<127 for b in j+r): continue
        if start+1<=offset<je:
            return f'byte {offset}, sound.name: Java {j.decode()!r}, Rust {r.decode()!r}'
        cursor=je;rcursor=re
        for name,fmt in [('x','<d'),('y','<d'),('z','<d'),('volume','<f'),('pitch','<f')]:
            size=struct.calcsize(fmt)
            if cursor<=offset<cursor+size:
                return f'byte {offset}, sound.{name}: Java {struct.unpack_from(fmt,java,cursor)[0]}, Rust {struct.unpack_from(fmt,rust,rcursor)[0]}'
            cursor+=size;rcursor+=size
    return None


def chunk_snapshot_difference(java, rust, offset, cursor=0, context='chunk'):
    """Walk Mode_wbarrays snapshots, including cached precipitation heights."""
    start=cursor
    def field(name,size,fmt=None):
        nonlocal cursor
        if cursor<=offset<cursor+size:
            if fmt: j=struct.unpack_from(fmt,java,cursor)[0];r=struct.unpack_from(fmt,rust,cursor)[0]
            else: j=java[offset];r=rust[offset];name+=f'[{offset-cursor}]'
            raise Found(f'byte {offset}, {context}.{name}: Java {j}, Rust {r}')
        cursor+=size
    field('xPosition',4,'<i');field('zPosition',4,'<i')
    for name in ['blocks','data','skyLight','blockLight','heightMap']:
        length=struct.unpack_from('<i',java,cursor)[0]
        field(name+'.length',4,'<i');field(name,length)
    field('lowestBlockHeight',4,'<i')
    for name in ['isModified','isTerrainPopulated','hasEntities']:field(name,1,'<B')
    for index in range(256):field(f'precipitationHeightMap[{index}]',4,'<i')
    for index in range(256):field(f'updateSkylightColumns[{index}]',1,'<B')
    return cursor


def arrays_difference(java,rust,offset):
    cursor=0
    try:
        for case in range(5):cursor=chunk_snapshot_difference(java,rust,offset,cursor,f'constructor[{case}]')
        for case in range(40):
            cursor=chunk_snapshot_difference(java,rust,offset,cursor,f'loadedChunk[{case}]')
            length=struct.unpack_from('<i',java,cursor)[0]
            if cursor<=offset<cursor+4: return f'byte {offset}, loadedChunk[{case}].afterFinalize.blocks.length: Java {length}, Rust {struct.unpack_from("<i",rust,cursor)[0]}'
            cursor+=4
            if cursor<=offset<cursor+length:return f'byte {offset}, loadedChunk[{case}].afterFinalize.blocks[{offset-cursor}]: Java {java[offset]}, Rust {rust[offset]}'
            cursor+=length
    except Found as found:return str(found)
    except (IndexError,struct.error):return None
    return None


def legacy_paths_difference(java,rust,offset):
    cursor=0
    for create in [False,True]:
        for x in [0,-1,63,64,-64,-2147483648,2147483647]:
            for z in [0,-1,64]:
                if cursor==offset:return f'byte {offset}, paths(create={create}, x={x}, z={z}).filePresent: Java {java[cursor]}, Rust {rust[cursor]}'
                present=java[cursor];cursor+=1
                if present:
                    length=struct.unpack_from('<i',java,cursor)[0];cursor+=4+length
                for name in ['xDirectoryExists','zDirectoryExists','loadedChunkPresent']:
                    if cursor==offset:return f'byte {offset}, paths(create={create}, x={x}, z={z}).{name}: Java {java[cursor]}, Rust {rust[cursor]}'
                    cursor+=1
    if cursor<=offset<cursor+1:return f'byte {offset}, loadFixture[0].chunkPresent: Java {java[cursor]}, Rust {rust[cursor]}'
    return None


def diagnose(java_path, rust_path, arguments=None):
    java, rust = java_path.read_bytes(), rust_path.read_bytes()
    if java == rust: return 'EXACT'
    meta = Path(str(java_path) + '.oracle10.json')
    if arguments is None and meta.exists(): arguments = json.loads(meta.read_text())['arguments']
    if arguments and arguments[0].replace('181','10')=='particlefactory6_10' and java[:4]==struct.pack('<i',0x18160007) and rust[:4]!=java[:4]:
        prefix=8
        for _ in range(struct.unpack_from('<i',java,4)[0]):
            count=struct.unpack_from('<i',java,prefix)[0];prefix+=4+count
        rust=java[:prefix]+rust
    offset = next((i for i, (a,b) in enumerate(zip(java,rust)) if a != b), min(len(java),len(rust)))
    meta = Path(str(java_path) + '.oracle10.json')
    if arguments is None and meta.exists(): arguments = json.loads(meta.read_text())['arguments']
    if arguments:
        mode = arguments[0].replace('181','10')
        terrain = terrain_difference(mode, arguments, java, rust, offset)
        if terrain: return terrain
        if mode in {'playertickintegration10','playertick10'}:
            detail=playertick_difference(java,rust)
            if detail: return detail
        if mode == 'guichest10':
            for start in range(max(4,offset-256),offset+1):
                jl=struct.unpack_from('<i',java,start-4)[0];rl=struct.unpack_from('<i',rust,start-4)[0]
                if jl!=rl or not 0<jl<256 or not start<=offset<start+jl or start+jl>min(len(java),len(rust)): continue
                j=java[start:start+jl];r=rust[start:start+rl]
                if j.startswith(b'random.') and r.startswith(b'random.') and all(32<=v<127 for v in j+r):
                    return f'byte {offset}, sound.name: Java {j.decode()!r}, Rust {r.decode()!r}'
        event=event_difference(java,rust,offset)
        if event:return event
        if mode == 'wbarrays':
            detail=arrays_difference(java,rust,offset)
            if detail:return detail
        if mode == 'wblegacy5':
            detail=legacy_paths_difference(java,rust,offset)
            if detail:return detail
        if mode == 'playerinventory': return inventory_difference(java,rust)
        if mode == 'entityblocks': return block_difference(java,rust)
        envelope = {'entityintegration10':'entities', 'playerintegration10':'player', 'vehicles':'vehicles', 'containertiles':'containers'}
        if mode in envelope:
            try:
                scenario = arguments[1] if mode != 'containertiles' else 'tiles'
                ticks = arguments[2] if mode != 'containertiles' else arguments[1]
                return replay_difference(envelope[mode], scenario, ticks, java, rust)
            except (ValueError,IndexError,KeyError,OverflowError,struct.error):
                pass
    if not arguments:
        return f'byte {offset}; dump has no invocation sidecar, supply --oracle-args to locate its fixture field'
    # Recreate in an isolated output name, retaining every other fixture argument.
    key=hashlib.sha256(str(java_path.resolve()).encode()).hexdigest()[:12]
    output = ROOT / 'tools/oracle10/out' / ('diagnose10-' + key + '-' + java_path.name)
    args = list(arguments)
    indexes = [i for i, arg in enumerate(args) if arg.endswith('.bin')]
    if not indexes: return f'byte {offset}; invocation has no binary output argument'
    args[indexes[-1]] = str(output)
    env = dict(os.environ, ORACLE10_TRACE_BYTE=str(offset))
    bash = os.environ.get('VERIFY10_BASH', 'C:/Program Files/Git/usr/bin/bash.exe')
    result = subprocess.run([bash, str(ROOT/'tools/oracle10/oracle.sh'), *args], env=env, cwd=ROOT, capture_output=True, text=True)
    match = re.search(r'^FIRST FIELD: (.*)$', result.stdout + result.stderr, re.M)
    if output.exists() and output.read_bytes() != java:
        return f'byte {offset}; original fixture was not repeatable under tracing; {match[1] if match else "field trace unavailable"}'
    if match:
        detail = match[1]
        info = re.search(r'field starts (\d+), (\d+) bytes, (\w+)', detail)
        if info:
            start, size, typ = int(info[1]), int(info[2]), info[3]
            formats = {'Double':'<d','Float':'<f','Long':'<q','Integer':'<i','Boolean':'<?'}
            if '[big endian]' in detail: formats = {k:v.replace('<','>') for k,v in formats.items()}
            if typ == 'Integer' and size == 2: formats[typ] = '>h' if '[big endian]' in detail else '<h'
            if typ == 'Integer' and size == 1: formats[typ] = 'B'
            if typ in formats and struct.calcsize(formats[typ]) == size and start+size <= len(rust):
                detail += '; Rust ' + repr(struct.unpack_from(formats[typ], rust, start)[0])
            elif typ == 'String' and size > 8 and offset < min(len(java),len(rust)):
                name = detail.split(' = ',1)[0]
                detail = f'{name}[{offset-start}]: Java {java[offset]}, Rust {rust[offset]} (array starts {start})'
            else: detail += '; Rust bytes ' + rust[start:start+min(size,8)].hex()
        return detail
    return f'byte {offset}; field is written outside the scalar trace helpers (Java {java[offset:offset+8].hex()}, Rust {rust[offset:offset+8].hex()})'


def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('java', type=Path);parser.add_argument('rust',type=Path)
    parser.add_argument('--oracle-args', nargs=argparse.REMAINDER)
    args=parser.parse_args()
    print(diagnose(args.java, args.rust, args.oracle_args))


if __name__ == '__main__': main()
