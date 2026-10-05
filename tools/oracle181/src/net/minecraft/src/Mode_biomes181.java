package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
public final class Mode_biomes181 {
    public static void main(String[] a) throws Exception {
        long seed=Long.parseLong(a[1]);World world=Oracle.newWorld(seed);
        for(Field f:IntCache.class.getDeclaredFields()) {f.setAccessible(true);if(f.getType()==int.class)f.setInt(null,256);else ((List)f.get(null)).clear();}
        WorldChunkManager m=new WorldChunkManager(world);
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[2])))) {
            for(int[] q:new int[][]{{0,0,16,16},{-19,31,17,13},{-1,-1,1,1},{512,-1024,32,16},{-33,-41,9,7}}) {
                int n=q[2]*q[3];BiomeGenBase[] b=m.loadBlockGeneratorData(null,q[0],q[1],q[2],q[3]);
                for(int i=0;i<n;i++)out.writeByte(b[i].field_35494_y);
                float[] t=m.getTemperatures(null,q[0],q[1],q[2],q[3]);for(float v:t)out.writeFloat(v);
                float[] r=m.func_35560_b(null,q[0],q[1],q[2],q[3]);for(float v:r)out.writeFloat(v);
                b=m.func_35557_b(null,q[0],q[1],q[2],q[3]);for(int i=0;i<n;i++)out.writeByte(b[i].field_35494_y);
            }
            List allowed=Arrays.asList(BiomeGenBase.forest,BiomeGenBase.swampland,BiomeGenBase.taiga);
            Random rand=new Random(seed);
            for(int radius:new int[]{4,64,256,16,0}) {
                ChunkPosition p=m.func_35556_a(-17,31,radius,allowed,rand);out.writeBoolean(p!=null);
                if(p!=null) {out.writeInt(p.x);out.writeInt(p.z);}
                out.writeLong(EntityOracle.rngState(rand));out.writeBoolean(m.func_35562_a(-17,31,radius,allowed));
            }
            for(int[] p:new int[][]{{-21,79},{-21,79},{1025,-512},{0,0}}) {out.writeByte(m.getBiomeGenAt(p[0],p[1]).field_35494_y);out.writeFloat(m.func_35554_b(p[0],p[1]));}
        }
    }
}
