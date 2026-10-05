"""Locate the first differing field in inherited EntityOracle/PlayerOracle dumps.

Usage: python tools/b181/diagnose_replay181.py entities|player|containers scenario ticks java.bin rust.bin
Reads the existing fixture format; does not change simulation or oracle output.
"""
import struct
import sys
from pathlib import Path


class Found(Exception):
    pass


class Replay:
    def __init__(self, java, rust):
        self.java, self.rust = java, rust
        self.offset = 0
        self.context = ""
        self.difference = next((i for i, (a, b) in enumerate(zip(java, rust)) if a != b), min(len(java), len(rust)))

    def field(self, name, fmt):
        size = struct.calcsize(fmt)
        start = self.offset
        self.offset += size
        if start <= self.difference < self.offset:
            j = struct.unpack_from(fmt, self.java, start)[0]
            r = struct.unpack_from(fmt, self.rust, start)[0]
            detail = ''
            if name == 'statCount':
                js = [struct.unpack_from('<ii', self.java, self.offset + i * 8) for i in range(j)]
                rs = [struct.unpack_from('<ii', self.rust, self.offset + i * 8) for i in range(r)]
                detail = f'; Java stats {js}, Rust stats {rs}'
            raise Found(f"byte {self.difference}, {self.context}.{name}: Java {j!r}, Rust {r!r}{detail}")
        return struct.unpack_from(fmt, self.java, start)[0]

    def i(self, name): return self.field(name, '<i')
    def b(self, name): return self.field(name, '<B')
    def f(self, name): return self.field(name, '<f')
    def d(self, name): return self.field(name, '<d')
    def l(self, name): return self.field(name, '<q')

    def bytes(self, name, count):
        start = self.offset
        self.offset += count
        if start <= self.difference < self.offset:
            raise Found(f"byte {self.difference}, {self.context}.{name}: Java {self.java[start:self.offset][:80]!r}, Rust {self.rust[start:self.offset][:80]!r}")
        return self.java[start:self.offset]

    def utf(self, name):
        return self.bytes(name, self.field(name + '.length', '>H')).decode('utf-8')

    def text(self, name):
        n = self.i(name + '.length')
        return None if n < 0 else self.bytes(name, n).decode('utf-8')

    def fields(self, fmt, names):
        for name in names.split(): self.field(name, fmt)

    def player(self):
        self.fields('<f', 'prevCameraYaw cameraYaw prevCameraPitch cameraPitch timeInPortal prevTimeInPortal')
        self.fields('<i', 'swingProgressInt timeUntilPortal sleepTimer armor currentItem damageRemainder')
        self.fields('<B', 'isSwinging sleeping sneaking')
        self.fields('<d', 'chasePrevX chasePrevY chasePrevZ chaseX chaseY chaseZ')
        for slot in range(40): self.fields('<i', f'inventory{slot}.id inventory{slot}.count inventory{slot}.damage inventory{slot}.animations')
        for stat in range(self.i('statCount')): self.fields('<i', f'stat{stat}.id stat{stat}.amount')

    def entity(self, frame, index):
        self.context = f'frame {frame}, entity {index}'
        eid = self.i('id'); kind = self.utf('kind')
        self.context = f'frame {frame}, {kind} id {eid}'
        self.fields('<d', 'posX posY posZ motionX motionY motionZ prevPosX prevPosY prevPosZ lastTickPosX lastTickPosY lastTickPosZ boxMinX boxMinY boxMinZ boxMaxX boxMaxY boxMaxZ')
        self.fields('<f', 'yaw pitch prevYaw prevPitch fallDistance ySize distanceWalked')
        self.fields('<i', 'fire air ticksExisted chunkX chunkY chunkZ')
        self.fields('<B', 'onGround dead inWater addedToChunk')
        if kind == 'Item':
            self.fields('<i', 'age pickupDelay itemId itemCount itemDamage'); self.f('hoverStart'); self.i('itemHealth')
        if kind == 'FallingSand': self.fields('<i', 'blockId fallTime')
        if kind in {'Player', 'Pig', 'Cow', 'Sheep', 'Chicken', 'Squid', 'Slime', 'Wolf', 'Zombie', 'Skeleton', 'Spider', 'Creeper', 'PigZombie', 'Giant', 'Ghast', 'Enderman', 'Silverfish', 'CaveSpider'}:
            self.fields('<i', 'health prevHealth heartsLife heartsHalvesLife hurtTime maxHurtTime deathTime attackTime livingSoundTime entityAge field9346af')
            self.fields('<f', 'attackedAtYaw renderYawOffset prevRenderYawOffset field703S field704R field705Q swingProgress prevSwingProgress field9361v field9362u field9360w field9359x field9365p field9363r field9325U moveStrafing moveForward randomYawVelocity')
            self.b('jumping')
            if kind == 'Pig': self.b('saddled')
            if kind == 'Sheep': self.b('sheared'); self.i('fleeceColor')
            if kind == 'Chicken': self.fields('<f', 'wingRotation prevWingRotation destPos prevDestPos wingSpeed'); self.i('eggTime')
        if kind == 'PrimedTnt': self.i('fuse')
        if kind in {'Snowball', 'ThrownEgg'}:
            self.fields('<i', 'xTile yTile zTile inTile'); self.b('inGround'); self.fields('<i', 'shake owner ticksInGround ticksInAir')
        if kind == 'Arrow':
            self.fields('<i', 'xTile yTile zTile inTile inData'); self.fields('<B', 'inGround playerArrow'); self.fields('<i', 'shake owner ticksInGround ticksInAir')
        if kind == 'Creeper': self.fields('<i', 'ignited lastActive creeperState'); self.b('powered')
        if kind == 'PigZombie': self.fields('<i', 'anger soundDelay')
        if kind == 'Wolf':
            self.i('wolfFlags'); self.utf('owner'); self.i('wolfHealth'); self.fields('<B', 'looksWithInterest shaking field25052g'); self.fields('<f', 'field25048b field25054c shakeTime prevShakeTime')
        if kind == 'Ghast':
            self.i('courseCooldown'); self.fields('<d', 'waypointX waypointY waypointZ'); self.fields('<i', 'aggroCooldown prevAttackCounter attackCounter'); self.b('attacking')
        if kind == 'Fireball':
            self.fields('<i', 'xTile yTile zTile inTile'); self.b('inGround'); self.fields('<i', 'shake owner ticksAlive ticksInAir'); self.fields('<d', 'accelerationX accelerationY accelerationZ')
        if kind == 'Slime': self.i('slimeSize'); self.fields('<f', 'squish prevSquish'); self.i('jumpDelay')
        if kind == 'Squid': self.fields('<f', 'squidPitch prevSquidPitch squidYaw prevSquidYaw squidRotation prevSquidRotation tentacleAngle prevTentacleAngle randomMotionSpeed rotationVelocity field21079m randomMotionX randomMotionY randomMotionZ')
        if kind == 'LightningBolt': self.i('lightningState'); self.l('boltVertex'); self.i('boltLivingTime')
        if kind == 'Player': self.player()
        if kind in {'Minecart', 'Boat'}:
            if kind == 'Minecart':
                self.fields('<i', 'cartType cartDamage timeSinceHit rockDirection fuel'); self.fields('<d', 'pushX pushZ'); self.b('reverse')
                for slot in range(self.i('cargoCount')): self.fields('<i', f'cargo{slot}.id cargo{slot}.count cargo{slot}.damage cargo{slot}.animations')
            else: self.fields('<i', 'boatDamage timeSinceHit direction')
            self.fields('<B', 'collidedX collidedY collided beenAttacked firstUpdate preventSpawning'); self.fields('<f', 'collisionReduction width height yOffset stepHeight prevDistanceWalked')
        self.fields('<i', 'riding riddenBy'); self.fields('<d', 'riderYawDelta riderPitchDelta'); self.l('rand')

    def events(self, frame):
        self.context = f'frame {frame}, events'
        for event in range(self.i('count')):
            self.context = f'frame {frame}, event {event}'
            tag = self.b('tag')
            if tag in {0, 1, 4}:
                name = self.text('name')
                self.context += f' {name!r} (tag {tag})'
                if tag != 4 or self.b('positioned'): self.fields('<d', 'x y z')
                if tag == 1: self.fields('<d', 'vx vy vz')
                else: self.fields('<f', 'volume pitch')
            elif tag == 2:
                if self.i('hasRecord'): self.text('record')
                self.fields('<i', 'x y z')
            elif tag == 3: self.fields('<i', 'effect x y z data')
            elif tag == 5: self.fields('<i', 'x y z side block meta')
            else: raise ValueError(f'bad event {tag}')

    def snapshot(self, frame):
        self.context = f'frame {frame}'
        for index in range(self.i('entityCount')): self.entity(frame, index)
        self.context = f'frame {frame}, world'
        self.fields('<q', 'rand mathRand nextEntitySeed explosionSeed time')
        self.fields('<i', 'lcg sound skylight flash autosave rainTime thunderTime')
        self.fields('<B', 'raining thundering'); self.fields('<f', 'rain prevRain thunder prevThunder')
        for spawner in range(self.i('spawnerCount')):
            self.fields('<i', f'spawner{spawner}.x spawner{spawner}.y spawner{spawner}.z spawner{spawner}.delay'); self.fields('<d', f'spawner{spawner}.yaw spawner{spawner}.prevYaw')
        self.events(frame)

    def extras(self, mode, scenario, frame):
        if mode == 'containers':
            self.context = f'frame {frame}, furnaces'
            for furnace in range(12):
                if self.b(f'furnace{furnace}.present'):
                    for slot in range(3):
                        if self.b(f'furnace{furnace}.slot{slot}.present'):
                            self.fields('>i', f'furnace{furnace}.slot{slot}.id furnace{furnace}.slot{slot}.count furnace{furnace}.slot{slot}.damage')
                    self.fields('>i', f'furnace{furnace}.burn furnace{furnace}.fuel furnace{furnace}.cook')
            self.field('worldHash', '>q')
            return
        if mode != 'player': return
        self.context = f'frame {frame}, player extras'
        if scenario in {'digextras', 'dig', 'place', 'use', 'shoot', 'interact', 'fertilize', 'activate', 'api', 'pipeline', 'bucket'}:
            self.i('result'); self.fields('<f', 'blockDamage prevBlockDamage field1069h'); self.fields('<i', 'blockX blockY blockZ hitWait'); self.l('itemRand')
        if scenario in {'api', 'pipeline'}: self.fields('<i', 'leftClickCounter mouseTicks'); self.f('brightness')
        if scenario == 'nbt': self.bytes('nbt', self.i('nbtLength'))
        if scenario in {'look', 'pickentities'}:
            for partial in range(3):
                self.fields('<d', 'lookX lookY lookZ'); tag=self.i('hitType')
                if tag == 1: self.i('hitEntity')
                elif tag == 0: self.fields('<i', 'hitX hitY hitZ hitSide'); self.fields('<d', 'hitVecX hitVecY hitVecZ')


def main():
    mode, scenario, ticks, java, rust = sys.argv[1:]
    replay = Replay(Path(java).read_bytes(), Path(rust).read_bytes())
    if replay.java == replay.rust:
        print('EXACT'); return
    try:
        for frame in range(int(ticks) + 1):
            replay.snapshot(frame); replay.extras(mode, scenario, frame)
        print(f'byte {replay.difference} in final world dump (starts at {replay.offset})')
    except Found as result:
        print(result)


if __name__ == '__main__': main()
