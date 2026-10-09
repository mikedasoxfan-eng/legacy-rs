package net.minecraft.src;

import java.io.*;
import java.util.*;

/** Standalone unchanged-bytecode biome graph probe; no World or RNG hooks needed. */
public final class LayerProbe11 {
    public static void main(String[] args) throws Exception {
        long seed = Long.parseLong(args[0]);
        if(args.length>2 && args[2].equals("flat")) { flat(seed,args[1]); return; }
        if(args.length>2 && args[2].equals("flatworld")) { flatWorld(seed,args[1]); return; }
        GenLayer[] layers = GenLayer.func_35497_a(seed);
        try (DataOutputStream out = new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for (int[] q : new int[][]{{0,0,16,16},{-19,31,17,13},{-1,-1,1,1},{512,-1024,32,16},{-33,-41,9,7},{-512,-512,64,64}}) {
                for (int layer = 0; layer < 4; layer++) {
                    IntCache.resetIntCache();
                    int[] values = layers[layer].getInts(q[0],q[1],q[2],q[3]);
                    for (int i = 0; i < q[2]*q[3]; i++) out.writeInt(values[i]);
                }
            }
        }
    }
    private static void flat(long seed,String path) throws Exception {
        WorldChunkManager m=new WorldChunkManagerHell(BiomeGenBase.plains,.5F,.5F);
        Random r=new Random(seed);
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(path))) {
            for(int[] q:new int[][]{{0,0,16,16},{-19,31,17,13},{Integer.MAX_VALUE,Integer.MIN_VALUE,1,1}}) {
                int n=q[2]*q[3];
                o.writeInt(m.getBiomeGenAt(q[0],q[1]).biomeID);
                for(BiomeGenBase[] a:new BiomeGenBase[][]{m.func_35557_b(null,q[0],q[1],q[2],q[3]),m.loadBlockGeneratorData(null,q[0],q[1],q[2],q[3])})
                    for(int i=0;i<n;i++)o.writeInt(a[i].biomeID);
                for(float v:m.getTemperatures(null,q[0],q[1],q[2],q[3]))o.writeFloat(v);
                for(float v:m.getRainfall(null,q[0],q[1],q[2],q[3]))o.writeFloat(v);
                o.writeFloat(m.getTemperature(q[0],64,q[1]));o.writeFloat(m.getRainfall(q[0],q[1]));
            }
            for(int radius:new int[]{0,4,256}) for(List allowed:new List[]{Arrays.asList(BiomeGenBase.desert),Arrays.asList(BiomeGenBase.plains)}) {
                o.writeBoolean(m.areBiomesViable(-17,31,radius,allowed));
                ChunkPosition p=m.func_35556_a(-17,31,radius,allowed,r);
                o.writeBoolean(p!=null);if(p!=null){o.writeInt(p.x);o.writeInt(p.z);}
                o.writeLong(r.nextLong());
            }
            BiomeGenBase[] reused=new BiomeGenBase[7];Arrays.fill(reused,BiomeGenBase.desert);
            reused=m.loadBlockGeneratorData(reused,0,0,2,2);
            for(BiomeGenBase b:reused)o.writeInt(b.biomeID);
        }
    }
    private static void flatWorld(final long seed,String path)throws Exception {
        final WorldSettings settings=new WorldSettings(seed,0,false,false,EnumWorldType.FLAT);
        World w=new World(new Oracle.NullSaveHandler(seed) {
            public WorldInfo loadWorldInfo(){return new WorldInfo(settings,"flatprobe");}
        },"flatprobe",settings);
        ChunkProviderFlat provider=new ChunkProviderFlat(w,seed,false);
        java.lang.reflect.Field random=ChunkProviderFlat.class.getDeclaredField("field_46053_b");random.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new FileOutputStream(path))) {
            for(int[] q:new int[][]{{0,0},{-19,31},{512,-1024}}) {
                Chunk c=provider.provideChunk(q[0],q[1]);
                o.write(c.blocks);o.write(c.data.data);o.write(c.skylightMap.data);o.write(c.blocklightMap.data);o.write(c.heightMap);o.writeInt(c.lowestBlockHeight);
                provider.populate(provider,q[0],q[1]);o.writeLong(((Random)random.get(provider)).nextLong());
            }
        }
    }
}
