package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_gui3endcopy10 {
 static void state(DataOutputStream o,EntityPlayer p)throws Exception {
  for(int v:new int[]{p.health,p.dimension,p.playerLevel,p.totalXP,p.getScore(),p.inventory.currentItem})Gui3Fixture10.i(o,v);Gui3Fixture10.f(o,p.currentXP);FoodStats f=p.getFoodStats();for(String k:new String[]{"foodLevel","foodTickTimer","prevFoodLevel"})Gui3Fixture10.i(o,Gui2Fixture10.field(FoodStats.class,k).getInt(f));for(String k:new String[]{"foodSaturationLevel","foodExhaustionLevel"})Gui3Fixture10.f(o,Gui2Fixture10.field(FoodStats.class,k).getFloat(f));
  for(ItemStack s:p.inventory.mainInventory)Gui3Fixture10.stack(o,s);for(ItemStack s:p.inventory.armorInventory)Gui3Fixture10.stack(o,s);Gui3Fixture10.stack(o,p.inventory.getItemStack());
 }
 public static void main(String[] a)throws Exception {
  Block.stone.getClass();Random r=new Random(30041);World w=(World)Oracle.allocate(World.class);
  try(DataOutputStream o=Gui2Fixture10.out(a[0])){for(int n=0;n<200;n++){
   Gui3Fixture10.Player p=Gui3Fixture10.player(w),q=Gui3Fixture10.player(w);p.health=r.nextInt(21);p.dimension=1;p.playerLevel=r.nextInt(70);p.totalXP=r.nextInt(3000);p.currentXP=r.nextFloat();Gui2Fixture10.field(EntityPlayer.class,"score").setInt(p,r.nextInt());p.inventory.currentItem=r.nextInt(9);p.inventory.setItemStack(new ItemStack(264,5,0));
   FoodStats f=new FoodStats();Gui2Fixture10.field(EntityPlayer.class,"foodStats").set(p,f);f.setFoodLevel(r.nextInt(21));f.setFoodSaturationLevel(r.nextFloat()*5);Gui2Fixture10.field(FoodStats.class,"foodExhaustionLevel").setFloat(f,r.nextFloat()*4);Gui2Fixture10.field(FoodStats.class,"foodTickTimer").setInt(f,r.nextInt(80));
   for(int k=0;k<40;k++)if(r.nextBoolean()){int[] ids={1,264,276,306};ItemStack s=new ItemStack(ids[r.nextInt(4)],1+r.nextInt(5),r.nextInt(20));p.inventory.setInventorySlotContents(k,s);}
   q.copyPlayer(p);q.dimension=0;state(o,q);Gui2Fixture10.field(Entity.class,"boundingBox").set(q,AxisAlignedBB.getBoundingBox(0,0,0,0,0,0));q.preparePlayerToSpawn();state(o,q);
  }}
 }
}
