package net.minecraft.src;
import java.io.*;import java.util.*;
/** Keep real JVM identities as replay inputs, with original HashSet/Spawner bytecode. */
public final class Mode_entityspawninputs6_10 {
 static final class RecordedMap extends HashMap<ChunkCoordIntPair,Boolean> {
  final ArrayList<Integer> values;
  RecordedMap(ArrayList<Integer> values){this.values=values;}
  public Boolean put(ChunkCoordIntPair p,Boolean boundary){values.add(System.identityHashCode(p));return super.put(p,boundary);}
 }
 static void i(DataOutputStream o,int n)throws Exception{EntityOracle.i(o,n);}
 static void state(World w,Entity e,DataOutputStream o)throws Exception{Mode_entities10.state(w,e,o);}
 static void spawning(DataOutputStream o, ArrayList<Integer> identities)throws Exception {
  for(int scenario=0;scenario<4;scenario++)for(int seed=0;seed<8;seed++)for(int players=1;players<=2;players++){
   World w=Mode_entities10.world();EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;
   for(int x=-14;x<=14;x++)for(int z=-14;z<=14;z++){
    byte[] b=new byte[32768];for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)b[(bx<<11)|(bz<<7)|63]=(byte)(scenario==3?1:2);
    Chunk c=new Chunk(w,b,x,z);c.isChunkLoaded=true;c.isTerrainPopulated=true;Arrays.fill(c.heightMap,(byte)64);Arrays.fill(c.skylightMap.data,(byte)(scenario==0?0:255));
    if(scenario==3)for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)for(int y=64;y<80;y++)c.blocks[(bx<<11)|(bz<<7)|y]=9;
    provider.chunks.put(provider.key(x,z),c);
   }
   java.lang.reflect.Field field=SpawnerAnimals.class.getDeclaredField("eligibleChunksForSpawning");field.setAccessible(true);field.set(null,new RecordedMap(identities));
   w.skylightSubtracted=scenario==0?11:0;
   for(String name:new String[]{"spawnX","spawnY","spawnZ"}){java.lang.reflect.Field sp=WorldInfo.class.getDeclaredField(name);sp.setAccessible(true);sp.setInt(w.worldInfo,name.equals("spawnY")?64:0);}
   w.rand.setSeed(seed);
   Mode_entities10.Player p=new Mode_entities10.Player(w);p.setPosition(.5,64,.5);w.playerEntities.add(p);
   if(players==2){Mode_entities10.Player q=new Mode_entities10.Player(w);q.setPosition(-16.01,64,16.01);w.playerEntities.add(q);}
   for(int t=0;t<2;t++){
    i(o,Trace10.namedInt("SpawnerAnimals.performSpawning(w,scenario==0,scenario!=0)", SpawnerAnimals.performSpawning(w,scenario==0,scenario!=0)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
    for(Object obj:w.loadedEntityList){Entity e=(Entity)obj;state(w,e,o);if(e instanceof EntitySheep)i(o,Trace10.namedInt("((EntitySheep)e).getFleeceColor()", ((EntitySheep)e).getFleeceColor()));}
   }
  }
 }

 public static void main(String[] a)throws Exception {
  ArrayList<Integer> identities=new ArrayList<Integer>();ByteArrayOutputStream bytes=new ByteArrayOutputStream();DataOutputStream states=new DataOutputStream(bytes);spawning(states,identities);states.close();
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))){i(o,Trace10.namedInt("identities.size()", identities.size()));for(int id:identities)i(o,id);Trace10.writeBytes(o, "bytes.toByteArray()", bytes.toByteArray());}
 }
}
