"""Mechanical b1.7.3 -> b1.8.1 renames in tools/oracle181 (identifier swaps from renames.py,
checked by hand). Idempotent: run again after copying more oracle code over.
Usage: python tools/b181/port_oracle.py
"""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
DIRS = ['tools/oracle181/src', 'tools/oracle181/headless', 'tools/oracle181/gui', 'tools/oracle181/server',
        'tools/oracle181/redstone']

WORDS = {
    'field_9437_g': 'updateLCG', 'func_27162_g': 'getRainStrength', 'func_628_d': 'drawClouds',
    'func_4079_a': 'getSkyColor', 'getRaining': 'getIsRaining', 'getThundering': 'getIsThundering',
    'setRaining': 'setIsRaining', 'setThundering': 'setIsThundering', 'field_27168_F': 'lastLightningBolt',
    'findTopSolidBlock': 'func_35461_e', 'setWolfSitting': 'setIsSitting', 'setWolfTamed': 'setIsTamed',
    'boatCurrentDamage': 'damageTaken', 'boatTimeSinceHit': 'timeSinceHit', 'boatRockDirection': 'forwardDirection',
    'field_774_f': 'cameraYaw', 'field_775_e': 'prevCameraYaw', 'field_22273_E': 'debugCamEnable',
    'TickUpdates': 'tickUpdates', 'func_28105_a': 'rayTraceBlocks_do_do', 'func_346_d': 'setAngles',
    'func_4107_a': 'placeInPortal', 'func_656_j': 'dropOldChucks', 'func_25060_a': 'verifyRespawnCoordinates',
    'musicBlock': 'music', 'mapItem': 'map', 'func_1145_d': 'stepSoundDir2', 'func_31006_g': 'isInvalid',
    'func_851_a': 'getDistanceToCamera', 'field_28135_a': 'anaglyphEnable', 'field_27086_j': 'timeStatType',
    'field_27085_k': 'distanceStatType', 'field_25185_d': 'objectMineStats', 'field_25158_z': 'objectCraftStats',
    'field_25172_A': 'objectUseStats', 'field_25170_B': 'objectBreakStats', 'func_28157_a': 'renderMap',
    'field_28108_z': 'mapStorage', 'func_410_i': 'canStay', 'func_28107_a': 'playAuxSFXAtEntity',
    'field_27029_a': 'boltVertex', 'field_9406_a': 'shake', 'field_9397_j': 'shootingEntity',
    'field_9405_b': 'accelerationX', 'field_9404_c': 'accelerationY', 'field_9403_d': 'accelerationZ',
    'func_27280_a': 'slotClick', 'func_27166_f': 'getWeightedThunderStrength', 'getIsTranslucent': 'getIsOpaque',
}


def fix(s):
    # Living camera pitch: b1.7.3 (cameraPitch, field_9328_R) = b1.8.1 (prevCameraPitch, cameraPitch).
    s = re.sub(r'\b(p|q|player|e)\.cameraPitch\b', r'\1.prevCameraPitch', s)
    s = re.sub(r'\bfield_9328_R\b', 'cameraPitch', s)
    s = re.sub(r'\b(' + '|'.join(map(re.escape, WORDS)) + r')\b', lambda m: WORDS[m.group(1)], s)
    # ChunkCoordinates fields.
    s = re.sub(r'\b(sp|ns|var4|home|spawn|c|cc|bed)\.(x|y|z)\b', lambda m: f'{m.group(1)}.pos{m.group(2).upper()}', s)
    # Arrow owner / container slots.
    s = re.sub(r'\b(a|arrow)\.owner\b', r'\1.shootingEntity', s)
    s = re.sub(r'\bc\.slots\b', 'c.inventorySlots', s)
    return s


n = 0
for d in DIRS:
    d = os.path.join(ROOT, d)
    if not os.path.isdir(d):
        continue
    for dp, _, fs in os.walk(d):
        for f in fs:
            if f.endswith('.java'):
                p = os.path.join(dp, f)
                s = open(p, encoding='utf-8', newline='').read()
                t = fix(s)
                if t != s:
                    open(p, 'w', encoding='utf-8', newline='').write(t)
                    n += 1
print('files changed', n)
