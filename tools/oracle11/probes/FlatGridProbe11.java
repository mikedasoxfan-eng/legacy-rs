package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Real original flat provider with structures enabled; inherited world dump plus RNG sidecar. */
public final class FlatGridProbe11 {
    public static void main(String[] a) throws Exception {
        final long seed=Long.parseLong(a[0]);
        final WorldSettings settings=new WorldSettings(seed,0,true,false,EnumWorldType.FLAT);
        World w=new World(new Oracle.NullSaveHandler(seed) {
            public WorldInfo loadWorldInfo(){return new WorldInfo(settings,"oracle");}
        },"oracle",settings);
        w.rand.setSeed(Long.parseLong(a[1]));
        for(int x=Integer.parseInt(a[2]);x<=Integer.parseInt(a[4]);x++)
            for(int z=Integer.parseInt(a[3]);z<=Integer.parseInt(a[5]);z++) {
                w.getChunkFromChunkCoords(x,z);
                if(a[6].equals("each"))Oracle.drainLighting(w);
            }
        if(a[6].equals("end"))Oracle.drainLighting(w);
        ChunkCoordinates sp=w.getSpawnPoint();Oracle.dumpWorld(w,a[7],sp.posX,sp.posY,sp.posZ);
        Object provider=Mode_structures10.field(w.chunkProvider,"chunkProvider");
        Random random=(Random)Mode_structures10.field(provider,"field_46053_b");
        try(DataOutputStream out=new DataOutputStream(new FileOutputStream(a[7]+".rng"))) {
            out.writeLong(EntityOracle.rngState(w.rand));out.writeLong(EntityOracle.rngState(random));
        }
    }
}
