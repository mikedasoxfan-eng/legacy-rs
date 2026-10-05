package net.minecraft.src;
import java.io.*;
public final class Mode_combat181 {
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(String name:new String[]{"Pig","Cow","Chicken","Sheep","Wolf","Zombie","Skeleton","Spider","CaveSpider","Creeper","PigZombie","Ghast","Squid","Slime","Enderman","Silverfish"})
   for(int source=0;source<6;source++)for(int hearts:new int[]{0,11,20})for(int amount:new int[]{0,1,4,25}) {
    World w=Mode_entities181.world();EntityLiving e=(EntityLiving)EntityList.createEntityInWorld(name,w);e.setLocationAndAngles(-3.5,64,4.5,35,0);e.heartsLife=hearts;e.health=20;
    EntityLiving attacker=source==2?new EntityZombie(w):new Mode_entities181.Player(w);attacker.setPosition(.5,65.62,.5);w.entityJoinedWorld(attacker);
    EntityArrow arrow=new EntityArrow(w);DamageSource damage=source==0?DamageSource.field_35547_j:source==1?DamageSource.field_35538_d:source==2?DamageSource.func_35525_a(attacker):source==3?DamageSource.func_35527_a((EntityPlayer)attacker):DamageSource.func_35535_a(arrow,source==4?attacker:null);
    for(int j=0;j<3;j++){o.writeBoolean(e.attackEntityFrom(damage,amount+j));Mode_entities181.state(w,e,o);}
   }
  }
 }
}
