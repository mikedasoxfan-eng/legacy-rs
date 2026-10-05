package net.minecraft.src;

import java.io.PrintWriter;
import java.lang.reflect.Field;
import java.util.Random;

/** The original Teleporter after a dimension change: preload around the arrival point as
 *  Minecraft.func_6255_d does, then find or build the portal. */
class TeleportOracle {
    static void run(long seed, long wrand, int dim, double x, double y, double z, long trand, String out) throws Exception {
        World w = dim == -1 ? Oracle.newNetherWorld(seed) : Oracle.newWorld(seed);
        w.rand.setSeed(wrand);
        int cx = (int) x, cz = (int) z;
        for (int dx = -128; dx <= 128; dx += 16) {
            for (int dz = -128; dz <= 128; dz += 16) {
                w.getBlockId(cx + dx, 64, cz + dz);
                Oracle.drainLighting(w);
            }
        }
        EntityPig e = new EntityPig(w);
        e.setLocationAndAngles(x, y, z, 0.0F, 0.0F);
        Teleporter t = new Teleporter();
        Field f = Teleporter.class.getDeclaredField("field_4232_a");
        f.setAccessible(true);
        ((Random) f.get(t)).setSeed(trand);
        t.placeInPortal(w, e);
        Oracle.dumpWorld(w, out, cx, 64, cz);
        try (PrintWriter pw = new PrintWriter(out + ".txt")) {
            pw.println("pos " + Double.doubleToLongBits(e.posX) + " " + Double.doubleToLongBits(e.posY - e.yOffset)
                + " " + Double.doubleToLongBits(e.posZ));
            pw.println("rand " + w.rand.nextLong());
        }
    }

    /** Minecraft.usePortal from the Overworld, line by line with the original World, Entity
     *  and Teleporter methods (the Minecraft instance itself can't run headless). */
    static void usePortal(long seed, long wrand, double x, double y, double z, long fresh, long trand, String out) throws Exception {
        World w = Oracle.newWorld(seed);
        w.rand.setSeed(wrand);
        w.getInitialSpawnLocation();
        ChunkCoordinates sp = w.getSpawnPoint();
        for (int dx = -128; dx <= 128; dx += 16) {
            for (int dz = -128; dz <= 128; dz += 16) {
                w.getBlockId(sp.posX + dx, 64, sp.posZ + dz);
                Oracle.drainLighting(w);
            }
        }
        Field ids = Entity.class.getDeclaredField("nextEntityID"); ids.setAccessible(true); ids.setInt(null, 1000);
        Item.itemRand.setSeed(600); OracleHooks.reset(100); EntityOracle.mathRandom().setSeed(200);
        Field chest = BlockChest.class.getDeclaredField("random"); chest.setAccessible(true); ((Random) chest.get(Block.chest)).setSeed(500);
        PlayerOracle.HeadlessPlayer p = PlayerOracle.player(w);
        p.setLocationAndAngles(x, y, z, 30.0F, 10.0F);
        w.entityJoinedWorld(p);

        p.dimension = p.dimension == -1 ? 0 : -1;
        w.setEntityDead(p);
        p.isDead = false;
        double nx = p.posX / 8.0D, nz = p.posZ / 8.0D;
        p.setLocationAndAngles(nx, p.posY, nz, p.rotationYaw, p.rotationPitch);
        if (p.isEntityAlive()) w.updateEntityWithOptionalForce(p, false);
        World n = new World(w, WorldProvider.getProviderForDimension(-1));
        n.rand.setSeed(fresh);
        Field sc = World.class.getDeclaredField("soundCounter"); sc.setAccessible(true); sc.setInt(n, n.rand.nextInt(12000));
        n.updateLCG = new Random(fresh ^ 0x5DEECE66DL).nextInt();
        // changeWorld: save the old world, func_6255_d, dropOldChucks, spawnPlayerWithLoadedChunks.
        w.saveWorld(true, null);
        int cx = (int) p.posX, cz = (int) p.posZ;
        for (int dx = -128; dx <= 128; dx += 16) {
            for (int dz = -128; dz <= 128; dz += 16) {
                n.getBlockId(cx + dx, 64, cz + dz);
                Oracle.drainLighting(n);
            }
        }
        n.dropOldChucks();
        n.spawnPlayerWithLoadedChunks(p);
        p.worldObj = n;
        if (p.isEntityAlive()) {
            p.setLocationAndAngles(nx, p.posY, nz, p.rotationYaw, p.rotationPitch);
            n.updateEntityWithOptionalForce(p, false);
            Teleporter t = new Teleporter();
            Field f = Teleporter.class.getDeclaredField("field_4232_a"); f.setAccessible(true);
            ((Random) f.get(t)).setSeed(trand);
            t.placeInPortal(n, p);
        }
        ChunkCoordinates ns = n.getSpawnPoint();
        Oracle.dumpWorld(n, out, ns.posX, ns.posY, ns.posZ);
        try (PrintWriter pw = new PrintWriter(out + ".txt")) {
            pw.println("pos " + Double.doubleToLongBits(p.posX) + " " + Double.doubleToLongBits(p.posY) + " " + Double.doubleToLongBits(p.posZ));
            pw.println("prev " + Double.doubleToLongBits(p.lastTickPosX) + " " + Double.doubleToLongBits(p.lastTickPosY) + " " + Double.doubleToLongBits(p.lastTickPosZ));
            pw.println("look " + Float.floatToIntBits(p.rotationYaw) + " " + Float.floatToIntBits(p.rotationPitch) + " " + Float.floatToIntBits(p.prevRotationYaw) + " " + Float.floatToIntBits(p.prevRotationPitch));
            pw.println("dimension " + p.dimension + " entities " + n.loadedEntityList.size() + " players " + n.playerEntities.size());
            pw.println("rand " + n.rand.nextLong());
        }
    }

    /** Minecraft.respawn(false, 0) in the Overworld, line by line with original methods.
     *  scenario: plain (no bed), bed (a bed by spawn), nobed (spawn point set, bed gone). */
    static void respawn(long seed, long wrand, String scenario, String out) throws Exception {
        World w = Oracle.newWorld(seed);
        w.rand.setSeed(wrand);
        w.getInitialSpawnLocation();
        ChunkCoordinates sp = w.getSpawnPoint();
        for (int dx = -128; dx <= 128; dx += 16) {
            for (int dz = -128; dz <= 128; dz += 16) {
                w.getBlockId(sp.posX + dx, 64, sp.posZ + dz);
                Oracle.drainLighting(w);
            }
        }
        Field ids = Entity.class.getDeclaredField("nextEntityID"); ids.setAccessible(true); ids.setInt(null, 1000);
        Item.itemRand.setSeed(600); OracleHooks.reset(100); EntityOracle.mathRandom().setSeed(200);
        Field chest = BlockChest.class.getDeclaredField("random"); chest.setAccessible(true); ((Random) chest.get(Block.chest)).setSeed(500);
        PlayerOracle.HeadlessPlayer p = PlayerOracle.player(w);
        p.setLocationAndAngles(sp.posX + 20.5, 90, sp.posZ - 7.5, 45, 5);
        w.entityJoinedWorld(p);
        int bx = sp.posX + 4, bz = sp.posZ, by = 127;
        while (by > 0 && w.getBlockId(bx, by - 1, bz) == 0) by--;
        if (scenario.equals("bed")) { EntityOracle.rawBlock(w, bx, by, bz, 26, 0); EntityOracle.rawBlock(w, bx, by, bz + 1, 26, 8); }
        if (!scenario.equals("plain")) p.setPlayerSpawnCoordinate(new ChunkCoordinates(bx, by, bz));
        p.health = 0;

        ChunkCoordinates var3 = p.getPlayerSpawnCoordinate(), var4 = null;
        boolean chat = false, var5 = true;
        if (var3 != null) { var4 = EntityPlayer.verifyRespawnCoordinates(w, var3); chat = var4 == null; }
        if (var4 == null) { var4 = w.getSpawnPoint(); var5 = false; }
        w.setSpawnLocation();
        w.updateEntityList();
        int var8 = p.entityId;
        w.setEntityDead(p);
        PlayerOracle.HeadlessPlayer q = PlayerOracle.player(w);
        q.dimension = 0;
        q.preparePlayerToSpawn();
        if (var5) {
            q.setPlayerSpawnCoordinate(var3);
            q.setLocationAndAngles((double) ((float) var4.posX + 0.5F), (double) ((float) var4.posY + 0.1F), (double) ((float) var4.posZ + 0.5F), 0.0F, 0.0F);
        }
        q.rotationYaw = -180.0F;
        w.spawnPlayerWithLoadedChunks(q);
        q.entityId = var8;
        int cx = (int) q.posX, cz = (int) q.posZ;
        for (int dx = -128; dx <= 128; dx += 16) {
            for (int dz = -128; dz <= 128; dz += 16) {
                w.getBlockId(cx + dx, 64, cz + dz);
                Oracle.drainLighting(w);
            }
        }
        w.dropOldChucks();
        ChunkCoordinates ns = w.getSpawnPoint();
        Oracle.dumpWorld(w, out, ns.posX, ns.posY, ns.posZ);
        ChunkCoordinates home = q.getPlayerSpawnCoordinate();
        try (PrintWriter pw = new PrintWriter(out + ".txt")) {
            pw.println("pos " + Double.doubleToLongBits(q.posX) + " " + Double.doubleToLongBits(q.posY) + " " + Double.doubleToLongBits(q.posZ));
            pw.println("look " + Float.floatToIntBits(q.rotationYaw) + " " + Float.floatToIntBits(q.rotationPitch));
            pw.println("id " + q.entityId + " health " + q.health + " chat " + chat + " home " + (home == null ? "none" : home.posX + "," + home.posY + "," + home.posZ));
            int alive = 0;
            for (Object o : w.loadedEntityList) if (!((Entity) o).isDead) alive++;
            pw.println("entities " + alive + " players " + w.playerEntities.size());
            pw.println("rand " + w.rand.nextLong());
        }
    }
}
