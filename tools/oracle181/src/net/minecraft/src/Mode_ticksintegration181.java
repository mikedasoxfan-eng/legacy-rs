package net.minecraft.src;
import java.io.*;
import java.lang.reflect.Field;
import java.util.HashSet;
/** Original whole-world tick replay, recording the VM identity hashes used by
 * Java 8 HashMap tree bins. World/event serialization is the shared fixture. */
public class Mode_ticksintegration181 {
    public static void main(String[] a) throws Exception {
        a = java.util.Arrays.copyOfRange(a, 1, a.length);
        ticks(Long.parseLong(a[0]), Long.parseLong(a[1]), Integer.parseInt(a[2]),
            Integer.parseInt(a[3]), Integer.parseInt(a[4]), a[5],
            Double.parseDouble(a[6]), Double.parseDouble(a[7]), a[8], a.length > 9 ? a[9] : "");
    }
    static void ticks(long seed, long wrand, int lcg, int sound, int n, String weather,
                      double dx, double dz, String out, String fires) throws Exception {
        World w = Oracle.newWorld(seed);
        w.rand.setSeed(wrand);
        w.getInitialSpawnLocation();
        ChunkCoordinates sp = w.getSpawnPoint();
        for (int x = -128; x <= 128; x += 16) {
            for (int z = -128; z <= 128; z += 16) {
                w.getBlockId(sp.posX + x, 64, sp.posZ + z);
                Oracle.drainLighting(w);
            }
        }
        Field positions = World.class.getDeclaredField("positionsToUpdate");
        positions.setAccessible(true);
        try (DataOutputStream identities = new DataOutputStream(new BufferedOutputStream(new FileOutputStream(out + ".identities")))) {
        positions.set(w, new HashSet<ChunkCoordIntPair>() {
            public boolean add(ChunkCoordIntPair p) {
                try { identities.writeInt(System.identityHashCode(p)); }
                catch (IOException e) { throw new UncheckedIOException(e); }
                return super.add(p);
            }
        });
        w.updateLCG = lcg;
        Field sc = World.class.getDeclaredField("soundCounter");
        sc.setAccessible(true);
        sc.setInt(w, sound);
        if (!weather.equals("clear")) {
            w.worldInfo.setIsRaining(true);
            w.worldInfo.setRainTime(1000000);
            w.rainingStrength = w.prevRainingStrength = 1.0F;
            if (weather.equals("thunder")) {
                w.worldInfo.setIsThundering(true);
                w.worldInfo.setThunderTime(1000000);
                w.thunderingStrength = w.prevThunderingStrength = 1.0F;
            }
        }
        w.skylightSubtracted = w.calculateSkylightSubtracted(1.0F);
        Field uf = sun.misc.Unsafe.class.getDeclaredField("theUnsafe");
        uf.setAccessible(true);
        sun.misc.Unsafe U = (sun.misc.Unsafe) uf.get(null);
        EntityPlayer pl = (EntityPlayer) U.allocateInstance(EntityPlayerSP.class);
        pl.posX = sp.posX + 0.5 + dx;
        pl.posY = sp.posY + 1.62;
        pl.posZ = sp.posZ + 0.5 + dz;
        w.playerEntities.add(pl);
        w.setAllowedMobSpawns(false, false);
        for (String f : fires.split(",")) {
            if (f.isEmpty()) continue;
            String[] xz = f.split(":");
            int fx = sp.posX + Integer.parseInt(xz[0]), fz = sp.posZ + Integer.parseInt(xz[1]);
            if (xz.length >= 3) {
                // dx:dz:id[:meta] places that block on top instead of fire.
                w.setBlockAndMetadataWithNotify(fx, w.getHeightValue(fx, fz), fz, Integer.parseInt(xz[2]),
                    xz.length >= 4 ? Integer.parseInt(xz[3]) : 0);
            } else {
                w.setBlockWithNotify(fx, w.getHeightValue(fx, fz), fz, Block.fire.blockID);
            }
        }
        Oracle.drainLighting(w);
        OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);WorldEventOracle.attach(w);
        try(DataOutputStream events=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(out+".events")));
            DataOutputStream frames=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(out+".frames")))) {
            for (int t = 0; t < n; t++) {
                w.tick();Oracle.drainLighting(w);WorldEventOracle.snapshot(w,events);
                frames.writeLong(w.getWorldTime());frames.writeInt(w.updateLCG);
                frames.writeInt(sc.getInt(w));frames.writeLong(EntityOracle.rngState(w.rand));
            }
        }
        Oracle.dumpWorld(w, out, sp.posX, sp.posY, sp.posZ);
        try (PrintWriter pw = new PrintWriter(out + ".txt")) {
            pw.println("time " + w.getWorldTime());
            pw.println("lcg " + w.updateLCG);
            pw.println("sound " + sc.getInt(w));
            pw.println("rain " + w.worldInfo.getIsRaining() + " " + w.worldInfo.getRainTime() + " " + Float.floatToIntBits(w.rainingStrength));
            pw.println("thunder " + w.worldInfo.getIsThundering() + " " + w.worldInfo.getThunderTime() + " " + Float.floatToIntBits(w.thunderingStrength));
            pw.println("skylight " + w.skylightSubtracted);
            pw.println("flash " + w.lastLightningBolt + " bolts " + w.weatherEffects.size());
            pw.println("rand " + w.rand.nextLong());
        }
        }
    }

}
