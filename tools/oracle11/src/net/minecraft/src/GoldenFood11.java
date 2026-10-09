package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;
public final class GoldenFood11 {
 static class W extends World {W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}public void playSoundAtEntity(Entity e,String n,float v,float p){}public void spawnParticle(String n,double x,double y,double z,double vx,double vy,double vz){}}
 static class P extends EntityPlayerSP {P(){super(null,null,new Session("probe",""),0);}void init()throws Exception{dataWatcher=new DataWatcher();dataWatcher.addObject(0,(byte)0);dataWatcher.addObject(1,(short)300);super.entityInit();foodStats=new FoodStats();Field f=EntityLiving.class.getDeclaredField("activePotionsMap");f.setAccessible(true);f.set(this,new HashMap());}void finish(){super.func_35208_ae();}}
 static void set(Object o,String key,Object value)throws Exception{Field f=FoodStats.class.getDeclaredField(key);f.setAccessible(true);f.set(o,value);}
 static float saturation(FoodStats food)throws Exception{Field f=FoodStats.class.getDeclaredField("foodSaturationLevel");f.setAccessible(true);return f.getFloat(food);}
 public static void main(String[]args)throws Exception{int boot=Block.stone.blockID;PrintWriter o=new PrintWriter(new File(args[0],"goldfood11.txt"));
  for(int level:new int[]{0,19,20})for(float saturation:new float[]{0,5})for(int remote=0;remote<2;remote++)for(int disabled=0;disabled<2;disabled++)for(int count:new int[]{1,2})for(int seed:new int[]{0,42}){
   W w=(W)SheepSlice11.unsafe().allocateInstance(W.class);w.rand=new Random(seed);w.multiplayerWorld=remote!=0;P p=(P)SheepSlice11.unsafe().allocateInstance(P.class);p.init();p.worldObj=w;p.rand=new Random(seed);p.inventory=new InventoryPlayer(p);p.capabilities=new PlayerCapabilities();p.capabilities.disableDamage=disabled!=0;set(p.foodStats,"foodLevel",level);set(p.foodStats,"foodSaturationLevel",saturation);ItemStack stack=new ItemStack(322,count,0);p.inventory.mainInventory[0]=stack;Item.appleGold.onItemRightClick(stack,w,p);boolean started=p.isUsingItem();p.finish();PotionEffect effect=p.getActivePotionEffect(Potion.regeneration);ItemStack held=p.inventory.mainInventory[0];o.println(level+" "+Float.floatToRawIntBits(saturation)+" "+remote+" "+disabled+" "+count+" "+seed+" "+(started?1:0)+" "+(held==null?0:held.stackSize)+" "+p.foodStats.getFoodLevel()+" "+Float.floatToRawIntBits(saturation(p.foodStats))+" "+(effect==null?0:effect.getDuration())+" "+(effect==null?-1:effect.getAmplifier())+" "+p.rand.nextLong()+" "+w.rand.nextLong());
  }o.close();
 }
}
