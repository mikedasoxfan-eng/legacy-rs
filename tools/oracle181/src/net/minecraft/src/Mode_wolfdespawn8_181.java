package net.minecraft.src;
import java.io.*;
/** EntityWolf.canDespawn overrides EntityAnimal: only tamed wolves opt out. */
public final class Mode_wolfdespawn8_181 {
 public static void main(String[] a)throws Exception {
  World w=Mode_entities181.world();EntityOtherPlayerMP player=new EntityOtherPlayerMP(w,"Player");
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))){
   int cases=0;
   for(int flags:new int[]{0,1,2,4,5,6,7})for(int age:new int[]{0,600,601,1000})
    for(double distance:new double[]{0,31.999,32,127.999,128,128.001,1000})
     for(boolean present:new boolean[]{false,true})for(int seed=0;seed<1600;seed++){
      w.playerEntities.clear();player.setPosition(distance,85,0);if(present)w.playerEntities.add(player);
      EntityWolf e=new EntityWolf(w);e.setPosition(0,85,0);e.dataWatcher.updateObject(16,Byte.valueOf((byte)flags));e.entityAge=age;e.rand.setSeed(seed);
      e.despawnEntity();EntityOracle.i(o,e.entityAge);o.writeBoolean(e.isDead);EntityOracle.l(o,EntityOracle.rngState(e.rand));cases++;
     }
   System.out.println("wolfdespawn8: "+cases+" tamed/wild, age/distance/RNG cases");
  }
 }
}
