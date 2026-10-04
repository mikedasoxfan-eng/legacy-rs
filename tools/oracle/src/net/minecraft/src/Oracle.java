package net.minecraft.src;

import java.io.*;
import java.lang.reflect.Field;
import java.lang.reflect.Modifier;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.*;

/**
 * Ground-truth dumper. Runs Mojang's original b1.7.3 bytecode (deobfuscated.jar,
 * names remapped only) and writes data the Rust port is compared against.
 * Lives in net.minecraft.src so it can call protected World methods.
 *
 *   raw <seed> <x0> <z0> <x1> <z1> <out>     unpopulated chunks (terrain+surface+caves)
 *   sin <out>                                MathHelper sine table
 *   blocktable <out.rs>                      static block/material properties as Rust source
 *   grid <seed> <wrand> <x0> <z0> <x1> <z1> <light:each|end|none> <out>
 *                                            load chunks x-major through the real World, dump world
 *   preload <seed> <wrand> <out> [fancy]     vanilla new-world flow: spawn search + "Building terrain"
 *   mathcheck <out>                          Math.sin/cos/pow results BigTree can observe
 *
 * <wrand> seeds World.rand, which vanilla leaves unseeded (it drives spawn search,
 * lava flow during generation and item-drop jitter).
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
        String mode = a.length > 0 ? a[0] : "";
        if (mode.equals("raw")) {
            raw(Long.parseLong(a[1]), Integer.parseInt(a[2]), Integer.parseInt(a[3]),
                Integer.parseInt(a[4]), Integer.parseInt(a[5]), a[6]);
        } else if (mode.equals("sin")) {
            sin(a[1]);
        } else if (mode.equals("blocktable")) {
            blockTable(a[1]);
        } else if (mode.equals("grid")) {
            grid(Long.parseLong(a[1]), Long.parseLong(a[2]), Integer.parseInt(a[3]), Integer.parseInt(a[4]),
                 Integer.parseInt(a[5]), Integer.parseInt(a[6]), a[7], a[8]);
        } else if (mode.equals("preload")) {
            // optional 5th arg "fancy": leaves render translucent (RenderGlobal already ran)
            Block.leaves.setGraphicsLevel(a.length > 4 && a[4].equals("fancy"));
            preload(Long.parseLong(a[1]), Long.parseLong(a[2]), a[3]);
        } else if (mode.equals("mathcheck")) {
            mathCheck(a[1]);
        } else {
            System.err.println("bad mode; see Oracle.java header");
            System.exit(2);
        }
    }

    // ---------------------------------------------------------------- raw terrain

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

    /** MathHelper.SIN_TABLE as little-endian float bits. */
    static void sin(String out) throws Exception {
        Field f = MathHelper.class.getDeclaredField("SIN_TABLE");
        f.setAccessible(true);
        float[] t = (float[]) f.get(null);
        ByteBuffer bb = ByteBuffer.allocate(t.length * 4).order(ByteOrder.LITTLE_ENDIAN);
        for (float v : t) bb.putInt(Float.floatToRawIntBits(v));
        try (OutputStream os = new FileOutputStream(out)) { os.write(bb.array()); }
    }

    /**
     * Every Math.sin/Math.cos input WorldGenBigTree can produce (nextFloat() * 2.0 * 3.14159
     * for all 2^24 floats), plus the Math.pow(x, 2.0) inputs it uses. Written as raw
     * little-endian double bits: Math.sin, Math.cos, StrictMath.sin, StrictMath.cos
     * (2^24 each), then pow pairs (x, result).
     */
    static void mathCheck(String out) throws IOException {
        int n = 1 << 24;
        try (OutputStream os = new BufferedOutputStream(new FileOutputStream(out), 1 << 20)) {
            ByteBuffer bb = ByteBuffer.allocate(8).order(ByteOrder.LITTLE_ENDIAN);
            for (int pass = 0; pass < 4; pass++) {
                for (int i = 0; i < n; i++) {
                    double a = (double) ((float) i / (float) (1 << 24)) * 2.0D * 3.14159D;
                    double v = pass == 0 ? Math.sin(a) : pass == 1 ? Math.cos(a)
                             : pass == 2 ? StrictMath.sin(a) : StrictMath.cos(a);
                    bb.clear(); bb.putLong(Double.doubleToRawLongBits(v)); os.write(bb.array());
                }
            }
            List<Double> xs = new ArrayList<>();
            for (int i = 0; i <= 64; i++) { xs.add((double) i); xs.add(i + 0.5D); xs.add((double) ((float) i / 2.0F)); }
            for (int h = 0; h <= 64; h++) xs.add(1.0D * (double) h / 13.0D);
            for (double x : xs) {
                bb.clear(); bb.putLong(Double.doubleToRawLongBits(x)); os.write(bb.array());
                bb.clear(); bb.putLong(Double.doubleToRawLongBits(Math.pow(x, 2.0D))); os.write(bb.array());
            }
        }
    }

    // ---------------------------------------------------------------- block table

    static void blockTable(String out) throws Exception {
        List<Material> mats = new ArrayList<>();
        List<String> matNames = new ArrayList<>();
        for (Field f : Material.class.getDeclaredFields()) {
            if (Modifier.isStatic(f.getModifiers()) && f.getType() == Material.class) {
                mats.add((Material) f.get(null));
                matNames.add(f.getName());
            }
        }
        StringBuilder s = new StringBuilder();
        s.append("// @generated by tools/oracle (Oracle blocktable) from Mojang's b1.7.3 classes. Do not edit.\n\n");
        s.append("use super::{BlockProps, MaterialProps};\n\n");
        s.append("pub const MATERIALS: [MaterialProps; ").append(mats.size()).append("] = [\n");
        for (int i = 0; i < mats.size(); i++) {
            Material m = mats.get(i);
            s.append(String.format(
                "    MaterialProps { name: \"%s\", is_solid: %b, blocks_movement: %b, is_liquid: %b, burns: %b, ground_cover: %b, translucent_solid: %b, blocks_grass: %b },\n",
                matNames.get(i), m.isSolid(), m.getIsSolid(), m.getIsLiquid(), m.getBurning(), m.getIsGroundCover(),
                m.getIsTranslucent(), m.getCanBlockGrass()));
        }
        s.append("];\n\n");
        for (int i = 0; i < mats.size(); i++) {
            s.append("pub const MAT_").append(matNames.get(i).toUpperCase()).append(": u8 = ").append(i).append(";\n");
        }
        s.append("\npub const BLOCKS: [BlockProps; 256] = [\n");
        for (int id = 0; id < 256; id++) {
            Block b = Block.blocksList[id];
            if (b == null) {
                s.append(String.format("    BlockProps::NONE, // %d\n", id));
                continue;
            }
            int mat = mats.indexOf(b.blockMaterial);
            s.append(String.format(
                "    BlockProps { exists: true, material: %d, light_opacity: %d, light_value: %d, opaque_cube: %b, normal_render: %b, container: %b, name: \"%s\" }, // %d\n",
                mat, Block.lightOpacity[id], Block.lightValue[id], Block.opaqueCubeLookup[id], b.renderAsNormalBlock(),
                Block.isBlockContainer[id], b.getBlockName(), id));
        }
        s.append("];\n");
        try (Writer w = new OutputStreamWriter(new FileOutputStream(out), "UTF-8")) { w.write(s.toString()); }
    }

    // ---------------------------------------------------------------- world scenarios

    static void drainLighting(World w) {
        while (w.updatingLighting()) {
        }
    }

    static void grid(long seed, long wrand, int x0, int z0, int x1, int z1, String light, String out) throws Exception {
        World w = newWorld(seed);
        w.rand.setSeed(wrand);
        for (int cx = x0; cx <= x1; cx++) {
            for (int cz = z0; cz <= z1; cz++) {
                w.getChunkFromChunkCoords(cx, cz);
                if (light.equals("each")) drainLighting(w);
            }
        }
        if (light.equals("end")) drainLighting(w);
        ChunkCoordinates sp = w.getSpawnPoint();
        dumpWorld(w, out, sp.x, sp.y, sp.z);
    }

    /** Minecraft.startWorld for a new world, through Minecraft.func_6255_d ("Building terrain"). */
    static void preload(long seed, long wrand, String out) throws Exception {
        World w = newWorld(seed);
        w.rand.setSeed(wrand);
        w.getInitialSpawnLocation();
        ChunkCoordinates sp = w.getSpawnPoint();
        short r = 128;
        for (int dx = -r; dx <= r; dx += 16) {
            for (int dz = -r; dz <= r; dz += 16) {
                w.getBlockId(sp.x + dx, 64, sp.z + dz);
                drainLighting(w);
            }
        }
        dumpWorld(w, out, sp.x, sp.y, sp.z);
    }

    @SuppressWarnings("unchecked")
    static void dumpWorld(World w, String out, int sx, int sy, int sz) throws Exception {
        Field cmF = ChunkProvider.class.getDeclaredField("chunkMap");
        cmF.setAccessible(true);
        Map<Integer, Chunk> map = (Map<Integer, Chunk>) cmF.get(w.chunkProvider);
        List<Chunk> chunks = new ArrayList<>(map.values());
        chunks.sort((p, q) -> p.xPosition != q.xPosition ? Integer.compare(p.xPosition, q.xPosition)
                                                         : Integer.compare(p.zPosition, q.zPosition));
        DataOut o = new DataOut();
        o.bytes("B173WLD1".getBytes("US-ASCII"));
        o.i32(sx); o.i32(sy); o.i32(sz);
        o.i32(chunks.size());
        for (Chunk c : chunks) {
            o.i32(c.xPosition); o.i32(c.zPosition);
            o.u8(c.isTerrainPopulated ? 1 : 0);
            o.bytes(c.blocks);
            o.bytes(c.data.data);
            o.bytes(c.skylightMap.data);
            o.bytes(c.blocklightMap.data);
            o.bytes(c.heightMap);
            List<TileEntity> tes = new ArrayList<>((Collection<TileEntity>) c.chunkTileEntityMap.values());
            tes.sort((p, q) -> p.xCoord != q.xCoord ? Integer.compare(p.xCoord, q.xCoord)
                             : p.yCoord != q.yCoord ? Integer.compare(p.yCoord, q.yCoord)
                             : Integer.compare(p.zCoord, q.zCoord));
            o.i32(tes.size());
            for (TileEntity te : tes) {
                o.i32(te.xCoord); o.i32(te.yCoord); o.i32(te.zCoord);
                if (te instanceof TileEntityChest) {
                    TileEntityChest ch = (TileEntityChest) te;
                    o.u8(1);
                    for (int i = 0; i < 27; i++) {
                        ItemStack st = ch.getStackInSlot(i);
                        if (st == null) { o.i16(-1); o.u8(0); o.i16(0); }
                        else { o.i16(st.itemID); o.u8(st.stackSize); o.i16(st.getItemDamage()); }
                    }
                } else if (te instanceof TileEntityMobSpawner) {
                    o.u8(2);
                    byte[] id = ((TileEntityMobSpawner) te).getMobID().getBytes("US-ASCII");
                    o.u8(id.length); o.bytes(id);
                } else {
                    throw new IllegalStateException("unexpected tile entity " + te);
                }
            }
        }
        // Pending (non-immediate) block ticks, in a canonical order.
        Field tts = World.class.getDeclaredField("scheduledTickTreeSet");
        tts.setAccessible(true);
        List<NextTickListEntry> ticks = new ArrayList<>((Collection<NextTickListEntry>) tts.get(w));
        ticks.sort((p, q) -> p.xCoord != q.xCoord ? Integer.compare(p.xCoord, q.xCoord)
                           : p.yCoord != q.yCoord ? Integer.compare(p.yCoord, q.yCoord)
                           : p.zCoord != q.zCoord ? Integer.compare(p.zCoord, q.zCoord)
                           : Integer.compare(p.blockID, q.blockID));
        o.i32(ticks.size());
        for (NextTickListEntry t : ticks) {
            o.i32(t.xCoord); o.i32(t.yCoord); o.i32(t.zCoord); o.i32(t.blockID); o.i64(t.scheduledTime);
        }
        Field ltu = World.class.getDeclaredField("lightingToUpdate");
        ltu.setAccessible(true);
        o.i32(((List<?>) ltu.get(w)).size());
        try (OutputStream os = new BufferedOutputStream(new FileOutputStream(out))) { o.writeTo(os); }
    }

    static class DataOut {
        final ByteArrayOutputStream buf = new ByteArrayOutputStream();
        final ByteBuffer bb = ByteBuffer.allocate(8).order(ByteOrder.LITTLE_ENDIAN);
        void u8(int v) { buf.write(v); }
        void i16(int v) { bb.clear(); bb.putShort((short) v); buf.write(bb.array(), 0, 2); }
        void i32(int v) { bb.clear(); bb.putInt(v); buf.write(bb.array(), 0, 4); }
        void i64(long v) { bb.clear(); bb.putLong(v); buf.write(bb.array(), 0, 8); }
        void bytes(byte[] b) { buf.write(b, 0, b.length); }
        void writeTo(OutputStream os) throws IOException { buf.writeTo(os); }
    }
}
