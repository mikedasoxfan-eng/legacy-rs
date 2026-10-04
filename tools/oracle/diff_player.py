"""Decode player/entity replay differences into frame, entity and field names."""
import struct
import sys
from pathlib import Path
class Reader:
    def __init__(self,path): self.data=Path(path).read_bytes();self.at=0;self.fields={}
    def read(self,name,fmt):
        size=struct.calcsize(fmt);raw=self.data[self.at:self.at+size];self.at+=size
        value=struct.unpack(fmt,raw)[0];self.fields[name]=(raw,value);return value
    def many(self,prefix,names,fmt):
        for name in names.split(): self.read(prefix+name,fmt)
    def stack(self,prefix): self.many(prefix,'id count damage animations','<i')
    def frame(self):
        self.fields={};count=self.read('count','<i')
        for _ in range(count):
            eid=self.read('entity.id','<i');n=self.read('entity.name.len','>H');kind=self.data[self.at:self.at+n].decode();self.at+=n;p=f'{eid}:{kind}.'
            self.many(p,'x y z mx my mz prevx prevy prevz lastx lasty lastz minx miny minz maxx maxy maxz','<d')
            self.many(p,'yaw pitch prevyaw prevpitch fall ysize distance','<f')
            self.many(p,'fire air ticks chunkx chunky chunkz','<i');self.many(p,'ground dead water filed','<?')
            if kind=='Item':
                self.many(p,'age delay item count damage','<i');self.read(p+'hover','<f');self.read(p+'health','<i')
            else:
                self.many(p,'health prevhealth hearts halves hurt maxhurt death attack sound age lastdamage','<i')
                self.many(p,'attackyaw body prevbody limb amount prevamount swing prevswing walk prevwalk phase prevphase random speed scale strafe forward randomyaw','<f');self.read(p+'jump','<?')
            if kind=='Player':
                self.many(p,'prevcamerayaw camerayaw prevcamerapitch camerapitch portal prevportal','<f')
                self.many(p,'swingticks portalcooldown sleeptimer armor currentitem remainder','<i');self.many(p,'swinging sleeping sneaking','<?')
                self.many(p,'prevcloakx prevcloaky prevcloakz cloakx cloaky cloakz','<d')
                for slot in range(40): self.stack(p+f'slot{slot}.')
            self.read(p+'rng','<q')
        self.many('world.','rng mathrng entityseed explosionseed time','<q')
        self.many('world.','lcg sound sky flash skyflash raintime thundertime','<i');self.many('world.','rain thunder','<?');self.many('world.','rainstrength prevrain thunderstrength prevthunder','<f')
        assert self.read('spawners','<i')==0
        if len(sys.argv)>4 and sys.argv[4]=='look':
            for partial in range(3):
                p=f'pick{partial}.';self.many(p,'x y z','<d');kind=self.read(p+'kind','<i')
                if kind==1:self.read(p+'entity','<i')
                elif kind==0:self.many(p,'bx by bz side','<i');self.many(p,'hitx hity hitz','<d')
        return self.fields
a,b=Reader(sys.argv[1]),Reader(sys.argv[2])
for frame in range(int(sys.argv[3])+1):
    x,y=a.frame(),b.frame()
    diff=[(k,x[k][1],y.get(k,('',None))[1]) for k in x if x[k][0]!=y.get(k,('',None))[0]]
    if diff:
        print('frame',frame,'offsets',a.at,b.at)
        for row in diff[:30]: print(*row)
        break
else: print('Frames exact; world starts at',a.at,b.at)
