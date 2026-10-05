"""One-off API adaptations of tools/oracle181 to b1.8.1 (after port_oracle.py's renames)."""
import os
import re

ROOT = os.path.dirname(os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
O = os.path.join(ROOT, 'tools/oracle181/src/net/minecraft/src')


def edit(name, pairs):
    p = os.path.join(O, name)
    s = open(p, encoding='utf-8', newline='').read()
    for a, b in pairs:
        if a not in s:
            print('MISSING in', name, ':', a[:70])
        s = s.replace(a, b)
    open(p, 'w', encoding='utf-8', newline='').write(s)


edit('Oracle.java', [
    ('''    static final BiomeGenBase[] BIOMES = {
        BiomeGenBase.rainforest, BiomeGenBase.swampland, BiomeGenBase.seasonalForest,
        BiomeGenBase.forest, BiomeGenBase.savanna, BiomeGenBase.shrubland, BiomeGenBase.taiga,
        BiomeGenBase.desert, BiomeGenBase.plains, BiomeGenBase.iceDesert, BiomeGenBase.tundra,
        BiomeGenBase.hell, BiomeGenBase.sky,
    };

    static int biomeId(BiomeGenBase b) {
        for (int i = 0; i < BIOMES.length; i++) if (BIOMES[i] == b) return i;
        throw new IllegalStateException("unknown biome " + b.biomeName);
    }
''', '''    /** b1.8.1 biomes carry their own id (BiomeGenBase.field_35486_a is the id table). */
    static int biomeId(BiomeGenBase b) {
        return b.field_35494_y;
    }

    /** New-world settings: seed, survival, structures on (the b1.8.1 create-world defaults). */
    static WorldSettings settings(long seed) {
        return new WorldSettings(seed, 0, true);
    }
'''),
    ('public WorldInfo loadWorldInfo() { return new WorldInfo(seed, "oracle"); }',
     'public WorldInfo loadWorldInfo() { return new WorldInfo(settings(seed), "oracle"); }\n        public void checkSessionLock() {}'),
    ('return new World(new NullSaveHandler(seed), "oracle", seed);', 'return new World(new NullSaveHandler(seed), "oracle", settings(seed));'),
    ('new ChunkProviderGenerate(world, seed)', 'new ChunkProviderGenerate(world, seed, true)'),
    ('public WorldInfo loadWorldInfo() { return new WorldInfo(seed, name); }', 'public WorldInfo loadWorldInfo() { return new WorldInfo(settings(seed), name); }'),
    ('new World(presetSaveHandler(dir, seed), new File(dir).getName(), seed)', 'new World(presetSaveHandler(dir, seed), new File(dir).getName(), settings(seed))'),
    ('new World(saveHandler(dir), new File(dir).getName(), 0L)', 'new World(saveHandler(dir), new File(dir).getName(), settings(0L))'),
    ('new World(new NullSaveHandler(seed), "oracle", seed, WorldProvider.getProviderForDimension(-1))',
     'new World(new NullSaveHandler(seed), "oracle", settings(seed), WorldProvider.getProviderForDimension(-1))'),
])

# Climate: b1.8.1 keeps float temperature/rainfall per column, read through the WCM getters.
p = os.path.join(O, 'Oracle.java')
s = open(p, encoding='utf-8', newline='').read()
s = s.replace('for (int i = 0; i < 256; i++) bb.putDouble(wcm.temperature[i]);',
              'float[] temps = wcm.getTemperatures(null, cx * 16, cz * 16, 16, 16);\n                    for (int i = 0; i < 256; i++) bb.putDouble(temps[i]);')
s = s.replace('for (int i = 0; i < 256; i++) bb.putDouble(wcm.humidity[i]);',
              'float[] rain = wcm.func_35560_b(null, cx * 16, cz * 16, 16, 16);\n                    for (int i = 0; i < 256; i++) bb.putDouble(rain[i]);')
open(p, 'w', encoding='utf-8', newline='').write(s)

# Damage: attackEntityFrom(Entity, int) became attackEntityFrom(DamageSource, int).
for name in ('EntityOracle.java', 'PlayerOracle.java', 'VehicleOracle.java'):
    p = os.path.join(O, name)
    s = open(p, encoding='utf-8', newline='').read()
    s = s.replace('attackEntityFrom(p,', 'attackEntityFrom(DamageSource.func_35527_a(p),')
    s = s.replace('attackEntityFrom(mob,', 'attackEntityFrom(DamageSource.func_35525_a(mob),')
    s = s.replace('new EntityArrow(w,p)', 'new EntityArrow(w,p,1.0F)')
    s = s.replace('attackEntityFrom(arrow,', 'attackEntityFrom(DamageSource.func_35535_a(arrow,p),')
    open(p, 'w', encoding='utf-8', newline='').write(s)
print('ok')
