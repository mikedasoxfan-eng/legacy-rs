package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;
public final class Mode_biomecache181 {
 public static void main(String[] a)throws Exception {
  long seed=Long.parseLong(a[1]);World w=Oracle.newWorld(seed);
  try(DataOutputStream out=new DataOutputStream(new FileOutputStream(a[2]))) {
   for(int scenario=0;scenario<3;scenario++) {
    WorldChunkManager m=new WorldChunkManager(w);BiomeCache c=null;for(Field f:WorldChunkManager.class.getDeclaredFields())if(f.getType()==BiomeCache.class){f.setAccessible(true);c=(BiomeCache)f.get(m);}
    c.func_35725_a(-32,16);c.func_35722_b(0,0);c.func_35727_c(64,-48);
    Field list=BiomeCache.class.getDeclaredField("field_35728_d");list.setAccessible(true);List<BiomeCacheBlock> blocks=(List<BiomeCacheBlock>)list.get(c);
    long now=System.currentTimeMillis();for(int i=0;i<3;i++)blocks.get(i).field_35653_f=now+(i==0?-40000:i==1?-1000:100000);
    Field last=BiomeCache.class.getDeclaredField("field_35729_b");last.setAccessible(true);last.setLong(c,scenario==0?now-10000:scenario==1?now:now+10000);
    c.func_35724_a();ArrayList<BiomeCacheBlock> sorted=new ArrayList<>(blocks);Collections.sort(sorted,(b,d)->b.field_35655_d==d.field_35655_d?Integer.compare(b.field_35656_e,d.field_35656_e):Integer.compare(b.field_35655_d,d.field_35655_d));out.writeInt(sorted.size());for(BiomeCacheBlock b:sorted){out.writeInt(b.field_35655_d);out.writeInt(b.field_35656_e);}
   }
   for(int id:new int[]{8,9}) {
    WorldChunkManagerHell m=new WorldChunkManagerHell(BiomeGenBase.field_35486_a[id],id==8?1.0F:0.5F,0.0F);Random r=new Random(seed);
    for(int radius:new int[]{0,1,16,256})for(boolean include:new boolean[]{true,false}) {List list=Arrays.asList(BiomeGenBase.field_35486_a[include?id:4]);ChunkPosition p=m.func_35556_a(-17,31,radius,list,r);out.writeBoolean(p!=null);if(p!=null){out.writeInt(p.x);out.writeInt(p.z);}out.writeLong(EntityOracle.rngState(r));out.writeBoolean(m.func_35562_a(-17,31,radius,list));}
    for(float v:m.getTemperatures(null,-21,35,7,9))out.writeFloat(v);for(float v:m.func_35560_b(null,-21,35,7,9))out.writeFloat(v);for(BiomeGenBase b:m.loadBlockGeneratorData(null,-21,35,7,9))out.writeByte(b.field_35494_y);
   }
  }
 }
}
