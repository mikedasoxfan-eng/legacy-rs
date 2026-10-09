package net.minecraft.src;
import java.io.*;
public final class Mode_explosion10 {
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(float size:new float[]{.25F,1,2,4})for(int c=0;c<4;c++) {
    World w=Mode_entities10.world();Entity[] mobs={new EntityPig(w),new EntityWolf(w),new EntitySlime(w)};
    for(int j=0;j<3;j++){mobs[j].setPosition(-3.5+j*.9,85.25+j*.125,4.5-j*.75);w.spawnEntityInWorld(mobs[j]);}
    Explosion ex=w.newExplosion(null,-3.5+c*.137,85.25+c*.25,4.5-c*.073,size,c%2!=0);
    EntityOracle.i(o,Trace10.namedInt("ex.destroyedBlockPositions.size()", ex.destroyedBlockPositions.size()));for(Object p:ex.destroyedBlockPositions){ChunkPosition q=(ChunkPosition)p;EntityOracle.i(o,Trace10.namedInt("q.x", q.x));EntityOracle.i(o,Trace10.namedInt("q.y", q.y));EntityOracle.i(o,Trace10.namedInt("q.z", q.z));}
    for(Entity e:mobs)Mode_entities10.state(w,e,o);
   }
  }
 }
}
