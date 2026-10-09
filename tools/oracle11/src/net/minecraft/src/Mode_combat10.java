package net.minecraft.src;
import java.io.*;
public final class Mode_combat10 {
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(String name:new String[]{"Pig","Cow","Chicken","Sheep","Wolf","Zombie","Skeleton","Spider","CaveSpider","Creeper","PigZombie","Ghast","Squid","Slime","Enderman","Silverfish"})
   for(int source=0;source<6;source++)for(int hearts:new int[]{0,11,20})for(int amount:new int[]{0,1,4,25}) {
    World w=Mode_entities10.world();EntityLiving e=(EntityLiving)EntityList.createEntityInWorld(name,w);e.setLocationAndAngles(-3.5,64,4.5,35,0);e.heartsLife=hearts;e.health=20;
    EntityLiving attacker=source==2?new EntityZombie(w):new Mode_entities10.Player(w);attacker.setPosition(.5,65.62,.5);w.spawnEntityInWorld(attacker);
    EntityArrow arrow=new EntityArrow(w);DamageSource damage=source==0?DamageSource.generic:source==1?DamageSource.inWall:source==2?DamageSource.causeMobDamage(attacker):source==3?DamageSource.causePlayerDamage((EntityPlayer)attacker):DamageSource.causeArrowDamage(arrow,source==4?attacker:null);
    for(int j=0;j<3;j++){Trace10.writeBoolean(o, "e.attackEntityFrom(damage,amount+j)", e.attackEntityFrom(damage,amount+j));Mode_entities10.state(w,e,o);}
   }
  }
 }
}
