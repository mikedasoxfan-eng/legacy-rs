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
def nbt_difference(java, rust):
    at=next((i for i,(a,b) in enumerate(zip(java,rust)) if a!=b),min(len(java),len(rust)))
    cursor=0
    class Found(Exception): pass
    def take(n,name,fmt=None):
        nonlocal cursor
        start=cursor;cursor+=n
        if start<=at<cursor:
            if fmt and cursor<=min(len(java),len(rust)):
                a=struct.unpack_from(fmt,java,start)[0];b=struct.unpack_from(fmt,rust,start)[0]
            else:
                name+=f'[{at-start}]';a=java[at] if at<len(java) else 'EOF';b=rust[at] if at<len(rust) else 'EOF'
            raise Found(f'NBT byte {at}, {name}: Java {a!r}, Rust {b!r}')
        return java[start:cursor]
    def number(fmt,name):return struct.unpack(fmt,take(struct.calcsize(fmt),name,fmt))[0]
    def text(name):return take(number('>H',name+'.length'),name).decode('utf-8',errors='replace')
    def payload(kind,name):
        if kind in {1,2,3,4,5,6}: number({1:'>b',2:'>h',3:'>i',4:'>q',5:'>f',6:'>d'}[kind],name)
        elif kind in {7,11,12}:take(number('>i',name+'.length')*{7:1,11:4,12:8}[kind],name)
        elif kind==8:text(name)
        elif kind==9:
            typ=number('>B',name+'.elementType');count=number('>i',name+'.count')
            for index in range(count):payload(typ,f'{name}[{index}]')
        elif kind==10:
            index=0
            while True:
                typ=number('>B',f'{name}.entry[{index}].type')
                if typ==0:break
                tag=text(f'{name}.entry[{index}].tagName');payload(typ,name+'.'+tag);index+=1
        else:raise ValueError('unknown NBT kind '+str(kind))
    try:
        typ=number('>B','root.type');name=text('root.tagName');payload(typ,name or 'root')
    except Found as found:return str(found)
    return f'NBT packet length: Java {len(java)}, Rust {len(rust)}'


def main():
    a, b = chunks(sys.argv[1]), chunks(sys.argv[2])
    same = sum(1 for k in a if k in b and a[k][0] == b[k][0])
    layout = sum(1 for k in a if k in b and a[k][1] == b[k][1])
    print('java chunks', len(a), 'rust chunks', len(b), 'identical data', same, 'identical sector layout', layout)
    files_same=sorted(os.listdir(os.path.join(sys.argv[1], 'region'))) == sorted(os.listdir(os.path.join(sys.argv[2], 'region')))
    print('region files',files_same)
    ok = len(a) == len(b) == same > 0 and files_same
    if not ok:
        for key in sorted(a.keys()|b.keys()):
            if key not in a or key not in b:
                print(f'FIRST DIFFERENCE: chunk {key}.present: Java {key in a}, Rust {key in b}');break
            if a[key][0]!=b[key][0]:
                print(f'FIRST DIFFERENCE: chunk {key}.'+nbt_difference(a[key][0],b[key][0]));break
        else:print('FIRST DIFFERENCE: region file names: Java '+str(sorted(os.listdir(os.path.join(sys.argv[1],'region')))))
    return 0 if ok else 1


if __name__=='__main__':sys.exit(main())
