import java.io.*;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.List;
import net.minecraft.src.*;

/**
 * Ground-truth dumper. Runs Mojang's original b1.7.3 bytecode (deobfuscated.jar,
 * names remapped only) and writes chunk data the Rust port is compared against.
 *
 *   raw <seed> <x0> <z0> <x1> <z1> <out>   unpopulated chunks (terrain+surface+caves)
 */
public class Oracle {
    static final BiomeGenBase[] BIOMES = {
        BiomeGenBase.rainforest, BiomeGenBase.swampland, BiomeGenBase.seasonalForest,
        BiomeGenBase.forest, BiomeGenBase.savanna, BiomeGenBase.shrubland, BiomeGenBase.taiga,
        BiomeGenBase.desert, BiomeGenBase.plains, BiomeGenBase.iceDesert, BiomeGenBase.tundra,
        BiomeGenBase.hell, BiomeGenBase.sky,
    };

    static int biomeId(BiomeGenBase b) {
        for (int i = 0; i < BIOMES.length; i++) if (BIOMES[i] == b) return i;
        throw new IllegalStateException("unknown biome " + b.biomeName);
    }

    /** Save handler with no disk I/O; hands the world a preset WorldInfo so no random spawn search runs. */
    static class NullSaveHandler implements ISaveHandler {
        final long seed;
        NullSaveHandler(long seed) { this.seed = seed; }
        public WorldInfo loadWorldInfo() { return new WorldInfo(seed, "oracle"); }
        public void func_22150_b() {}
        public IChunkLoader getChunkLoader(WorldProvider p) { return null; }
        public void saveWorldInfoAndPlayer(WorldInfo i, List l) {}
        public void saveWorldInfo(WorldInfo i) {}
        public File func_28113_a(String s) { return null; }
    }

    static World newWorld(long seed) {
        return new World(new NullSaveHandler(seed), "oracle", seed);
    }

    public static void main(String[] a) throws Exception {
        if (a.length > 0 && a[0].equals("raw")) {
            raw(Long.parseLong(a[1]), Integer.parseInt(a[2]), Integer.parseInt(a[3]),
                Integer.parseInt(a[4]), Integer.parseInt(a[5]), a[6]);
        } else if (a.length > 0 && a[0].equals("sin")) {
            sin(a[1]);
        } else {
            System.err.println("usage: raw <seed> <x0> <z0> <x1> <z1> <out> | sin <out>");
            System.exit(2);
        }
    }

    /** MathHelper.SIN_TABLE as little-endian float bits. */
    static void sin(String out) throws Exception {
        java.lang.reflect.Field f = MathHelper.class.getDeclaredField("SIN_TABLE");
        f.setAccessible(true);
        float[] t = (float[]) f.get(null);
        ByteBuffer bb = ByteBuffer.allocate(t.length * 4).order(ByteOrder.LITTLE_ENDIAN);
        for (float v : t) bb.putInt(Float.floatToRawIntBits(v));
        try (OutputStream os = new FileOutputStream(out)) { os.write(bb.array()); }
    }

    static void raw(long seed, int x0, int z0, int x1, int z1, String out) throws IOException {
        World world = newWorld(seed);
        ChunkProviderGenerate gen = new ChunkProviderGenerate(world, seed);
        WorldChunkManager wcm = world.getWorldChunkManager();
        try (OutputStream os = new BufferedOutputStream(new FileOutputStream(out))) {
            os.write("B173RAW1".getBytes("US-ASCII"));
            ByteBuffer bb = ByteBuffer.allocate(8 + 32768 + 16384 + 256 + 256 * 16).order(ByteOrder.LITTLE_ENDIAN);
            for (int cx = x0; cx <= x1; cx++) {
                for (int cz = z0; cz <= z1; cz++) {
                    Chunk c = gen.provideChunk(cx, cz);
                    // provideChunk leaves the chunk's own 16x16 climate in the shared WCM arrays
                    BiomeGenBase[] biomes = wcm.loadBlockGeneratorData(null, cx * 16, cz * 16, 16, 16);
                    bb.clear();
                    bb.putInt(cx).putInt(cz);
                    bb.put(c.blocks);
                    bb.put(c.data.data);
                    for (int i = 0; i < 256; i++) bb.put((byte) biomeId(biomes[i]));
                    for (int i = 0; i < 256; i++) bb.putDouble(wcm.temperature[i]);
                    for (int i = 0; i < 256; i++) bb.putDouble(wcm.humidity[i]);
                    os.write(bb.array(), 0, bb.position());
                }
            }
        }
    }
}
