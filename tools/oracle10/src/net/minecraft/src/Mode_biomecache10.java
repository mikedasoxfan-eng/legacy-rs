package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;
public final class Mode_biomecache10 {
 public static void main(String[] a)throws Exception {
  long seed=Long.parseLong(a[1]);World w=Oracle.newWorld(seed);
  try(DataOutputStream out=new DataOutputStream(new FileOutputStream(a[2]))) {
   for(int scenario=0;scenario<3;scenario++) {
    WorldChunkManager m=new WorldChunkManager(w);BiomeCache c=null;for(Field f:WorldChunkManager.class.getDeclaredFields())if(f.getType()==BiomeCache.class){f.setAccessible(true);c=(BiomeCache)f.get(m);}
    c.func_35725_a(-32,16);c.func_35722_b(0,0);c.func_35727_c(64,-48);
    Field list=BiomeCache.class.getDeclaredField("field_35728_d");list.setAccessible(true);List<BiomeCacheBlock> blocks=(List<BiomeCacheBlock>)list.get(c);
    long now=System.currentTimeMillis();for(int i=0;i<3;i++)blocks.get(i).field_35653_f=now+(i==0?-40000:i==1?-1000:100000);
    Field last=BiomeCache.class.getDeclaredField("field_35729_b");last.setAccessible(true);last.setLong(c,scenario==0?now-10000:scenario==1?now:now+10000);
    c.func_35724_a();ArrayList<BiomeCacheBlock> sorted=new ArrayList<>(blocks);Collections.sort(sorted,(b,d)->b.field_35655_d==d.field_35655_d?Integer.compare(b.field_35656_e,d.field_35656_e):Integer.compare(b.field_35655_d,d.field_35655_d));Trace10.writeInt(out, "sorted.size()", sorted.size());for(BiomeCacheBlock b:sorted){Trace10.writeInt(out, "b.field_35655_d", b.field_35655_d);Trace10.writeInt(out, "b.field_35656_e", b.field_35656_e);}
   }
   for(int id:new int[]{8,9}) {
    WorldChunkManagerHell m=new WorldChunkManagerHell(BiomeGenBase.biomeList[id],id==8?1.0F:0.5F,0.0F);Random r=new Random(seed);
    for(int radius:new int[]{0,1,16,256})for(boolean include:new boolean[]{true,false}) {List list=Arrays.asList(BiomeGenBase.biomeList[include?id:4]);ChunkPosition p=m.func_35556_a(-17,31,radius,list,r);Trace10.writeBoolean(out, "p!=null", p!=null);if(p!=null){Trace10.writeInt(out, "p.x", p.x);Trace10.writeInt(out, "p.z", p.z);}Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));Trace10.writeBoolean(out, "m.areBiomesViable(-17,31,radius,list)", m.areBiomesViable(-17,31,radius,list));}
    for(float v:m.getTemperatures(null,-21,35,7,9))Trace10.writeFloat(out, "v", v);for(float v:m.getRainfall(null,-21,35,7,9))Trace10.writeFloat(out, "v", v);for(BiomeGenBase b:m.loadBlockGeneratorData(null,-21,35,7,9))Trace10.writeByte(out, "b.biomeID", b.biomeID);
   }
  }
 }
}
