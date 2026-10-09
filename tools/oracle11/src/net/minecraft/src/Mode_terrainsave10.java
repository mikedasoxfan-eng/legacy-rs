package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** Independently generated saves with the otherwise time-seeded item/entity RNGs pinned. */
public final class Mode_terrainsave10 {
    public static void main(String[] a) throws Exception {
        long seed=Long.parseLong(a[1]),wrand=Long.parseLong(a[2]);String dir=a[3];
        OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(200);
        Field f=BlockChest.class.getDeclaredField("random");f.setAccessible(true);((Random)f.get(Block.chest)).setSeed(500);
        World w=new World(Oracle.presetSaveHandler(dir,seed),new File(dir).getName(),Oracle.settings(seed));w.rand.setSeed(wrand);
        // Saved TileTicks follow scheduledTickSet's HashMap order, whose tree bins break
        // equal-hash ties by System.identityHashCode: record them for the Rust replay.
        Mode_r10world4.recordTickIdentities(w);w.getInitialSpawnLocation();ChunkCoordinates sp=w.getSpawnPoint();
        for(int dx=-128;dx<=128;dx+=16)for(int dz=-128;dz<=128;dz+=16){w.getBlockId(sp.posX+dx,64,sp.posZ+dz);Oracle.drainLighting(w);}
        w.saveWorld(true,null);Oracle.dumpWorld(w,a[4],sp.posX,sp.posY,sp.posZ);
        try(DataOutputStream ids=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[4]+".tickids")))){ids.writeInt(Mode_r10world4.identities.size());for(int v:Mode_r10world4.identities)ids.writeInt(v);}
    }
}
