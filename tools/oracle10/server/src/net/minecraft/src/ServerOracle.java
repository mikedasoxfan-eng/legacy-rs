package net.minecraft.src;

import java.io.*;
import java.lang.reflect.Field;
import java.nio.ByteBuffer;
import java.nio.ByteOrder;
import java.util.*;
import net.minecraft.server.MinecraftServer;

/**
 * Ground truth for server-mode chunk handling, running the original 1.0 *server*
 * classes (RetroMCP's deobfuscated server jar): WorldServer, ChunkProviderServer and
 * PlayerManager, with a stand-in player walking a scripted path.
 *
 *   serverwalk <seed> <wrand> <viewRadius> <dir> <ticks> <out>
 *
 * Per tick it runs MinecraftServer.doTick's world part (World.tick with mob spawning off,
 * draining lighting), then the player's movement update. The stand-in player is also in
 * playerEntities, so random block ticks run around it. At the end the player leaves and
 * the world is saved.
 */
public class ServerOracle {
    /** Network handler that drops every packet (never constructed: allocated via Unsafe). */
    static class NullHandler extends NetServerHandler {
        NullHandler() { super(null, null, null); }
        public void sendPacket(Packet p) {}
    }

    static sun.misc.Unsafe unsafe() throws Exception {
        Field f = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
        f.setAccessible(true);
        return (sun.misc.Unsafe) f.get(null);
    }

    static void seedRandom(Object object, Class<?> type, String name, long seed) throws Exception {
        Field field = type.getDeclaredField(name);
        field.setAccessible(true);
        ((Random) field.get(object)).setSeed(seed);
    }

    /** The scripted walk; must match `walk_path` in the Rust compare tool. */
    static double[] path(double sx, double sz, int t) {
        if (t < 600) return new double[] {sx + 3.0D * t, sz + 1.0D * t};
        double x600 = sx + 3.0D * 600, z600 = sz + 1.0D * 600;
        return new double[] {x600 - 3.0D * (t - 600), z600 + 0.5D * (t - 600)};
    }

    public static void main(String[] a) throws Exception {
        if (a.length < 7 || !a[0].equals("serverwalk")) {
            System.err.println("usage: serverwalk <seed> <wrand> <viewRadius> <dir> <ticks> <out>");
            System.exit(2);
        }
        final long seed = Long.parseLong(a[1]);
        long wrand = Long.parseLong(a[2]);
        int radius = Integer.parseInt(a[3]);
        File dir = new File(a[4]).getAbsoluteFile();
        int ticks = Integer.parseInt(a[5]);
        final String name = dir.getName();

        // Item constructors and machine/chest drops otherwise use clock seeds.
        seedRandom(null, Class.forName("java.lang.Math$RandomNumberGeneratorHolder"), "randomNumberGenerator", 200);
        seedRandom(Block.chest, BlockChest.class, "random", 500);
        seedRandom(Block.stoneOvenIdle, BlockFurnace.class, "furnaceRand", 700);
        seedRandom(Block.stoneOvenActive, BlockFurnace.class, "furnaceRand", 701);
        seedRandom(Block.dispenser, BlockDispenser.class, "random", 702);

        sun.misc.Unsafe u = unsafe();
        MinecraftServer mc = (MinecraftServer) u.allocateInstance(MinecraftServer.class);
        // Preset WorldInfo: skip the constructor's unseeded spawn search; redo it seeded below.
        SaveOldDir handler = new SaveOldDir(dir.getParentFile(), name, false) {
            boolean first = true;
            public WorldInfo loadWorldInfo() {
                if (first) { first = false; return new WorldInfo(new WorldSettings(seed, 0, true, false), name); }
                return super.loadWorldInfo();
            }
        };
        WorldServer w = new WorldServer(mc, handler, name, 0, new WorldSettings(seed, 0, true, false));
        mc.worldMngr = new WorldServer[] {w, null};
        w.rand.setSeed(wrand);
        w.generateSpawnPoint();
        ChunkCoordinates sp = w.getSpawnPoint();

        // MinecraftServer.initWorld: start region.
        short r = 196;
        for (int dx = -r; dx <= r; dx += 16) {
            for (int dz = -r; dz <= r; dz += 16) {
                w.chunkProviderServer.loadChunk(sp.posX + dx >> 4, sp.posZ + dz >> 4);
                while (w.updatingLighting()) {
                }
            }
        }

        PlayerManager pm = new PlayerManager(mc, 0, radius);
        EntityPlayerMP p = (EntityPlayerMP) u.allocateInstance(EntityPlayerMP.class);
        p.worldObj = w;
        // Unsafe skips the release Entity watcher, including the new air slot.
        p.dataWatcher = new DataWatcher();
        p.dataWatcher.addObject(0, Byte.valueOf((byte)0));
        p.dataWatcher.addObject(1, Short.valueOf((short)300));
        p.entityInit();
        p.itemInWorldManager = new ItemInWorldManager(w);
        p.field_35214_K = new PlayerCapabilities();
        p.field_35191_aF = new HashMap(); // Unsafe skips EntityLiving's field initializer.
        Field food = EntityPlayer.class.getDeclaredField("foodStats");
        food.setAccessible(true);
        food.set(p, new FoodStats());
        p.loadedChunks = new LinkedList();
        p.field_420_ah = new HashSet();
        p.playerNetServerHandler = (NetServerHandler) u.allocateInstance(NullHandler.class);
        double sx = sp.posX + 0.5D, sz = sp.posZ + 0.5D;
        p.posX = sx;
        p.posZ = sz;
        pm.addPlayer(p);
        p.inventory = new InventoryPlayer(p); // autosave writes the player into level.dat
        w.playerEntities.add(p);
        w.setAllowedSpawnTypes(false, false);
        // Unseeded in vanilla: the random-tick LCG and the cave-sound countdown.
        Field lcg = World.class.getDeclaredField("distHashCounter");
        lcg.setAccessible(true);
        lcg.setInt(w, 12345);
        Field amb = World.class.getDeclaredField("ambientTickCountdown");
        amb.setAccessible(true);
        amb.setInt(w, 100);

        Field active = World.class.getDeclaredField("activeChunkSet");
        active.setAccessible(true);
        try (final DataOutputStream identities = new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[6] + ".identities")))) {
        active.set(w, new HashSet<ChunkCoordIntPair>() {
            public boolean add(ChunkCoordIntPair pair) {
                try { identities.writeInt(System.identityHashCode(pair)); }
                catch (IOException e) { throw new UncheckedIOException(e); }
                return super.add(pair);
            }
        });
        for (int t = 0; t < ticks; t++) {
            w.tick();
            while (w.updatingLighting()) {
            }
            double[] pos = path(sx, sz, t);
            p.posX = pos[0];
            p.posZ = pos[1];
            pm.updateMountedMovingPlayer(p);
        }
        pm.removePlayer(p);
        w.saveWorld(true, null);
        dump(w, a[6], sp);
        try (PrintWriter state = new PrintWriter(a[6] + ".txt")) {
            state.println("time " + w.getWorldTime());
            state.println("lcg " + lcg.getInt(w));
            state.println("sound " + amb.getInt(w));
            state.println("rand " + w.rand.nextLong());
        }
        }
    }

    @SuppressWarnings("unchecked")
    static void dump(WorldServer w, String out, ChunkCoordinates sp) throws Exception {
        Field cm = ChunkProviderServer.class.getDeclaredField("field_727_f");
        cm.setAccessible(true);
        List<Chunk> chunks = new ArrayList<>((List<Chunk>) cm.get(w.chunkProviderServer));
        chunks.sort((p, q) -> p.xPosition != q.xPosition ? Integer.compare(p.xPosition, q.xPosition) : Integer.compare(p.zPosition, q.zPosition));
        Field mob = TileEntityMobSpawner.class.getDeclaredField("mobID");
        mob.setAccessible(true);
        ByteArrayOutputStream buf = new ByteArrayOutputStream();
        ByteBuffer bb = ByteBuffer.allocate(8).order(ByteOrder.LITTLE_ENDIAN);
        buf.write("B173WLD1".getBytes("US-ASCII"));
        int[] head = {sp.posX, sp.posY, sp.posZ, chunks.size()};
        for (int v : head) { bb.clear(); bb.putInt(v); buf.write(bb.array(), 0, 4); }
        for (Chunk c : chunks) {
            bb.clear(); bb.putInt(c.xPosition); buf.write(bb.array(), 0, 4);
            bb.clear(); bb.putInt(c.zPosition); buf.write(bb.array(), 0, 4);
            buf.write(c.isTerrainPopulated ? 1 : 0);
            buf.write(c.blocks);
            buf.write(c.data.data);
            buf.write(c.skylightMap.data);
            buf.write(c.blocklightMap.data);
            buf.write(c.heightMap);
            List<TileEntity> tes = new ArrayList<>((Collection<TileEntity>) c.chunkTileEntityMap.values());
            tes.sort((p, q) -> p.xCoord != q.xCoord ? Integer.compare(p.xCoord, q.xCoord)
                             : p.yCoord != q.yCoord ? Integer.compare(p.yCoord, q.yCoord) : Integer.compare(p.zCoord, q.zCoord));
            bb.clear(); bb.putInt(tes.size()); buf.write(bb.array(), 0, 4);
            for (TileEntity te : tes) {
                for (int v : new int[] {te.xCoord, te.yCoord, te.zCoord}) { bb.clear(); bb.putInt(v); buf.write(bb.array(), 0, 4); }
                if (te instanceof TileEntityChest) {
                    buf.write(1);
                    for (int i = 0; i < 27; i++) {
                        ItemStack st = ((TileEntityChest) te).getStackInSlot(i);
                        bb.clear();
                        if (st == null) bb.putShort((short) -1).put((byte) 0).putShort((short) 0);
                        else bb.putShort((short) st.itemID).put((byte) st.stackSize).putShort((short) st.getItemDamage());
                        buf.write(bb.array(), 0, 5);
                    }
                } else {
                    buf.write(2);
                    byte[] id = ((String) mob.get(te)).getBytes("US-ASCII");
                    buf.write(id.length);
                    buf.write(id);
                }
            }
        }
        // Pending block ticks in processing order (time, then entry id); the light queue
        // is drained.
        Field tts = World.class.getDeclaredField("scheduledTickTreeSet");
        tts.setAccessible(true);
        Collection<NextTickListEntry> ticks = (Collection<NextTickListEntry>) tts.get(w);
        bb.clear(); bb.putInt(ticks.size()); buf.write(bb.array(), 0, 4);
        for (NextTickListEntry t : ticks) {
            for (int v : new int[] {t.xCoord, t.yCoord, t.zCoord, t.blockID}) { bb.clear(); bb.putInt(v); buf.write(bb.array(), 0, 4); }
            bb.clear(); bb.putLong(t.scheduledTime); buf.write(bb.array(), 0, 8);
        }
        bb.clear(); bb.putInt(0); buf.write(bb.array(), 0, 4);
        try (OutputStream os = new FileOutputStream(out)) { buf.writeTo(os); }
        System.out.println("java: " + chunks.size() + " chunks loaded at the end");
    }
}
