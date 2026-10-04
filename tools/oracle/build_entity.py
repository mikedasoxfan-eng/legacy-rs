"""Recompile the one-line source hook, then preserve original method bytecode.

javac optimizes the decompiler's casts/branches differently. The final class keeps
every original method and attribute; only new Random() in Entity's constructor is
replaced by invokestatic OracleHooks.newEntityRandom() plus padding NOPs. All
instruction offsets, exception tables and stack maps remain valid and unchanged.
"""
import pathlib, struct, subprocess, sys, zipfile

root = pathlib.Path(__file__).resolve().parents[2]
jar = root / "tools/retromcp/minecraft/jars/deobfuscated.jar"
build = root / "tools/oracle/build"
generated = root / "tools/oracle/generated/net/minecraft/src/Entity.java"
source = root / "tools/retromcp/minecraft/src/net/minecraft/src/Entity.java"
generated.parent.mkdir(parents=True, exist_ok=True)
text = source.read_text()
old = "protected Random rand = new Random();"
assert text.count(old) == 1
generated.write_text(text.replace(old, "protected Random rand = OracleHooks.newEntityRandom();"))
subprocess.run(["javac", "-nowarn", "-cp", str(jar)+";"+str(build), "-d", str(build), str(generated)], check=True)

def patch(class_name,hook):
    with zipfile.ZipFile(jar) as z:
        original = z.read("net/minecraft/src/"+class_name+".class")
    data = bytearray(original)
    u2 = lambda at: struct.unpack_from(">H", data, at)[0]
    u4 = lambda at: struct.unpack_from(">I", data, at)[0]
    count = u2(8)
    cp = [None] * count
    at = 10
    i = 1
    while i < count:
        tag = data[at]; at += 1
        if tag == 1:
            n = u2(at); at += 2; cp[i] = (tag, bytes(data[at:at+n]).decode()); at += n
        elif tag in (7,8,16): cp[i] = (tag, u2(at)); at += 2
        elif tag in (9,10,11,12,18): cp[i] = (tag, u2(at), u2(at+2)); at += 4
        elif tag in (3,4): cp[i] = (tag,); at += 4
        elif tag in (5,6): cp[i] = (tag,); at += 8; i += 1
        elif tag == 15: cp[i] = (tag,); at += 3
        else: raise ValueError(tag)
        i += 1
    end_cp = at
    utf = lambda idx: cp[idx][1]
    random_class = next(i for i,c in enumerate(cp) if c and c[0]==7 and utf(c[1])=="java/util/Random")
    random_init = next(i for i,c in enumerate(cp) if c and c[0]==10 and c[1]==random_class and utf(cp[c[2]][1])=="<init>" and utf(cp[c[2]][2])=="()V")
    pattern = b"\xbb"+struct.pack(">H",random_class)+b"\x59\xb7"+struct.pack(">H",random_init)
    at += 6
    at += 2+2*u2(at)  # interfaces
    fields = u2(at); at += 2
    def skip_member(at):
        attributes=u2(at+6);at+=8
        for _ in range(attributes):at+=6+u4(at+2)
        return at
    for _ in range(fields):at=skip_member(at)
    methods=u2(at);at+=2
    patched=0
    for _ in range(methods):
        name=utf(u2(at+2)); attributes=u2(at+6);at+=8
        for _ in range(attributes):
            attr=utf(u2(at));size=u4(at+2);body=at+6
            if name=="<init>" and attr=="Code":
                start=body+8;end=start+u4(body+4)
                index=data.find(pattern,start,end)
                assert index>=0 and data.find(pattern,index+1,end)<0
                data[index:index+7]=b"\xb8"+struct.pack(">H",count+5)+b"\x00"*4
                patched+=1
            at+=6+size
    assert patched==1
    def utf_entry(s):
        raw=s.encode();return b"\x01"+struct.pack(">H",len(raw))+raw
    extra=(utf_entry("net/minecraft/src/OracleHooks")+b"\x07"+struct.pack(">H",count)
           +utf_entry(hook)+utf_entry("()Ljava/util/Random;")
           +b"\x0c"+struct.pack(">HH",count+2,count+3)
           +b"\x0a"+struct.pack(">HH",count+1,count+4))
    data[8:10]=struct.pack(">H",count+6)
    data[end_cp:end_cp]=extra
    (build/("net/minecraft/src/"+class_name+".class")).write_bytes(data)
    # Check exact preservation, allowing only pool extension and those seven bytes.
    restored=bytearray(data);del restored[end_cp:end_cp+len(extra)]
    restored[8:10]=original[8:10];restored[index:index+7]=pattern
    assert restored==original, class_name+" bytecode changed outside its RNG initializer"
    print(class_name+" hook: all original methods and offsets preserved; RNG initializer only")

patch("Entity","newEntityRandom")
patch("Explosion","newExplosionRandom")
