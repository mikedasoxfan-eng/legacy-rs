package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
public final class Mode_biomes10 {
    public static void main(String[] a) throws Exception {
        long seed=Long.parseLong(a[1]);World world=Oracle.newWorld(seed);
        for(Field f:IntCache.class.getDeclaredFields()) {f.setAccessible(true);if(f.getType()==int.class)f.setInt(null,256);else ((List)f.get(null)).clear();}
        WorldChunkManager m=new WorldChunkManager(world);
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[2])))) {
            for(int[] q:new int[][]{{0,0,16,16},{-19,31,17,13},{-1,-1,1,1},{512,-1024,32,16},{-33,-41,9,7}}) {
                int n=q[2]*q[3];BiomeGenBase[] b=m.loadBlockGeneratorData(null,q[0],q[1],q[2],q[3]);
                for(int i=0;i<n;i++)Trace10.writeByte(out, "b[i].biomeID", b[i].biomeID);
                float[] t=m.getTemperatures(null,q[0],q[1],q[2],q[3]);for(float v:t)Trace10.writeFloat(out, "v", v);
                float[] r=m.getRainfall(null,q[0],q[1],q[2],q[3]);for(float v:r)Trace10.writeFloat(out, "v", v);
                b=m.func_35557_b(null,q[0],q[1],q[2],q[3]);for(int i=0;i<n;i++)Trace10.writeByte(out, "b[i].biomeID", b[i].biomeID);
            }
            List allowed=Arrays.asList(BiomeGenBase.forest,BiomeGenBase.swampland,BiomeGenBase.taiga);
            Random rand=new Random(seed);
            for(int radius:new int[]{4,64,256,16,0}) {
                ChunkPosition p=m.func_35556_a(-17,31,radius,allowed,rand);Trace10.writeBoolean(out, "p!=null", p!=null);
                if(p!=null) {Trace10.writeInt(out, "p.x", p.x);Trace10.writeInt(out, "p.z", p.z);}
                Trace10.writeLong(out, "EntityOracle.rngState(rand)", EntityOracle.rngState(rand));Trace10.writeBoolean(out, "m.areBiomesViable(-17,31,radius,allowed)", m.areBiomesViable(-17,31,radius,allowed));
            }
            for(int[] p:new int[][]{{-21,79},{-21,79},{1025,-512},{0,0}}) {Trace10.writeByte(out, "m.getBiomeGenAt(p[0],p[1]).biomeID", m.getBiomeGenAt(p[0],p[1]).biomeID);Trace10.writeFloat(out, "m.func_35554_b(p[0],64,p[1])", m.func_35554_b(p[0],64,p[1]));}
        }
    }
}
