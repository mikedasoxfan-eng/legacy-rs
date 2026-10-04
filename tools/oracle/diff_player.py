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
            elif kind=='FallingSand': self.many(p,'block falltime','<i')
            elif kind not in ("Arrow","Snowball","ThrownEgg","PrimedTnt","FallingSand","Fireball","LightningBolt"):
                self.many(p,'health prevhealth hearts halves hurt maxhurt death attack sound age lastdamage','<i')
                self.many(p,'attackyaw body prevbody limb amount prevamount swing prevswing walk prevwalk phase prevphase random speed scale strafe forward randomyaw','<f');self.read(p+'jump','<?')
            if kind in ('Snowball','ThrownEgg'):
                self.many(p,'tilex tiley tilez block','<i');self.read(p+'inground','<?');self.many(p,'shake owner groundticks airticks','<i')
            if kind=='PrimedTnt': self.read(p+'fuse','<i')
            if kind=='Creeper':self.many(p,'ignition prevignition state','<i');self.read(p+'powered','<?')
            if kind=='Ghast':self.read(p+'course','<i');self.many(p,'wayx wayy wayz','<d');self.many(p,'aggro prevattack attack','<i');self.read(p+'charged','<?')
            if kind=='Fireball':self.many(p,'tilex tiley tilez block','<i');self.read(p+'ground','<?');self.many(p,'shake owner groundticks airticks','<i');self.many(p,'ax ay az','<d')
            if kind=='LightningBolt':self.read(p+'state','<i');self.read(p+'seed','<q');self.read(p+'remaining','<i')
            if kind=='Slime':self.read(p+'size','<i');self.many(p,'squish prevsquish','<f');self.read(p+'jumpdelay','<i')
            if kind=='Squid':self.many(p,'sq0 sq1 sq2 sq3 sq4 sq5 sq6 sq7 sq8 sq9 sq10 sq11 sq12 sq13','<f')
            if kind=='PigZombie': self.many(p,'anger sounddelay','<i')
            if kind=='Arrow':
                self.many(p,'tilex tiley tilez block meta','<i');self.many(p,'inground belongs','<?');self.many(p,'shake owner groundticks airticks','<i')
            if kind=='Sheep': self.read(p+'sheared','<?');self.read(p+'color','<i')
            if kind=='Pig': self.read(p+'saddled','<?')
            if kind=='Wolf':
                self.read(p+'flags','<i');n=self.read(p+'ownerlen','>H');self.at+=n;self.read(p+'watchedhealth','<i');self.many(p,'interested shaking active','<?');self.many(p,'interest previnterest shaketime prevshaketime','<f')
            if kind=='Chicken':
                self.many(p,'flap prevflap flapamount prevflapamount flapspeed','<f');self.read(p+'eggtimer','<i')
            if kind=='Player':
                self.many(p,'prevcamerayaw camerayaw prevcamerapitch camerapitch portal prevportal','<f')
                self.many(p,'swingticks portalcooldown sleeptimer armor currentitem remainder','<i');self.many(p,'swinging sleeping sneaking','<?')
                self.many(p,'prevcloakx prevcloaky prevcloakz cloakx cloaky cloakz','<d')
                for slot in range(40): self.stack(p+f'slot{slot}.')
            self.read(p+'rng','<q')
        self.many('world.','rng mathrng entityseed explosionseed time','<q')
        self.many('world.','lcg sound sky flash skyflash raintime thundertime','<i');self.many('world.','rain thunder','<?');self.many('world.','rainstrength prevrain thunderstrength prevthunder','<f')
        for j in range(self.read('spawners','<i')): self.many(f'spawner{j}.','x y z delay','<i');self.many(f'spawner{j}.','yaw prev','<d')
        events=[]
        for j in range(self.read('events.count','<i')):
            tag=self.read(f'event{j}.type','B');prefix=f'event{j}.'
            if tag in (0,1,4):
                n=self.read(prefix+'namelen','<i');raw=self.data[self.at:self.at+max(n,0)];self.at+=max(n,0);self.fields[prefix+'name']=(raw,raw.decode() if n>=0 else None)
                if tag!=4 or self.read(prefix+'positioned','<?'):self.many(prefix,'x y z','<d')
                if tag==1:self.many(prefix,'vx vy vz','<d')
                else:self.many(prefix,'volume pitch','<f')
            elif tag==2:
                if self.read(prefix+'hasname','<i'):
                    n=self.read(prefix+'namelen','<i');raw=self.data[self.at:self.at+max(n,0)];self.at+=max(n,0);self.fields[prefix+'name']=(raw,raw.decode() if n>=0 else None)
                self.many(prefix,'x y z','<i')
            elif tag==3:self.many(prefix,'code x y z data','<i')
            elif tag==5:self.many(prefix,'x y z side block metadata','<i')
            else:raise ValueError(tag)
        if len(sys.argv)>4 and sys.argv[4] in ('look','pickentities'):
            for partial in range(3):
                p=f'pick{partial}.';self.many(p,'x y z','<d');kind=self.read(p+'kind','<i')
                if kind==1:self.read(p+'entity','<i')
                elif kind==0:self.many(p,'bx by bz side','<i');self.many(p,'hitx hity hitz','<d')
        if len(sys.argv)>4 and sys.argv[4] in ('digextras','dig','place','use','activate','shoot','interact','fertilize','api','pipeline','bucket'):
            self.read('controller.result','<i');self.many('controller.','damage prevdamage sounds','<f');self.many('controller.','x y z wait','<i');self.read('item_rng','<q')
        if len(sys.argv)>4 and sys.argv[4] in ('api','pipeline','bucket'): self.many('api.','leftcounter lastclick','<i');self.read('api.brightness','<f')
        if len(sys.argv)>4 and sys.argv[4]=='nbt':
            n=self.read('nbt_len','<i');data=self.data[self.at:self.at+n];self.at+=n;self.fields['nbt']=(data,data)
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
