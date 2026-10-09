package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
/** Only World storage/query replaced; animal find/attack methods are original bytecode. */
public final class AnimalDelta11 {
 static class W extends World {
  List<Entity> es;
  W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}
  public List getEntitiesWithinAABB(Class c,AxisAlignedBB b){List<Entity> out=new ArrayList<Entity>();for(Entity e:es)if(c.isAssignableFrom(e.getClass())&&e.boundingBox.intersectsWith(b))out.add(e);return out;}
  public void spawnParticle(String n,double x,double y,double z,double a,double b,double c){}
 }
 static W world()throws Exception{W w=(W)SheepSlice11.unsafe().allocateInstance(W.class);w.es=new ArrayList<Entity>();return w;}
 static Field field(String n)throws Exception{Field f=EntityAnimal.class.getDeclaredField(n);f.setAccessible(true);return f;}
 public static void main(String[]args)throws Exception{
  int boot=Block.stone.blockID;File d=new File(args[0]);Field love=field("inLove"),breed=field("breeding");Method find=EntityAnimal.class.getDeclaredMethod("findPlayerToAttack");find.setAccessible(true);Method attack=EntityAnimal.class.getDeclaredMethod("attackEntity",Entity.class,float.class);attack.setAccessible(true);
  PrintWriter o=new PrintWriter(new File(d,"animalfind11.txt"));
  for(int mode=0;mode<3;mode++)for(int axis=0;axis<3;axis++)for(double offset:new double[]{-9,-8.4,-8,-1,0,1,8,8.4,9})for(int flee=0;flee<2;flee++){
   W w=world();EntityCow a=new EntityCow(w);a.setPosition(0,64,0);a.setDelay(mode==2?1:0);love.setInt(a,mode==0?600:0);a.fleeingTick=flee;
   EntityLiving b;if(mode==1){EntityPlayerSP p=(EntityPlayerSP)SheepSlice11.unsafe().allocateInstance(EntityPlayerSP.class);p.worldObj=w;p.width=.6F;p.height=1.8F;Field bb=Entity.class.getDeclaredField("boundingBox");bb.setAccessible(true);bb.set(p,AxisAlignedBB.getBoundingBox(0,0,0,0,0,0));p.inventory=new InventoryPlayer(p);p.inventory.mainInventory[0]=new ItemStack(Item.wheat);b=p;}else{EntityCow cow=new EntityCow(w);cow.setDelay(-1);love.setInt(cow,600);b=cow;}
   b.setPosition(axis==0?offset:0,64+(axis==1?offset:0),axis==2?offset:0);w.es.add(b);a.rand.setSeed(42);o.println(mode+" "+axis+" "+Double.doubleToRawLongBits(offset)+" "+flee+" "+(find.invoke(a)==b?1:0)+" "+a.rand.nextLong());
  }o.close();o=new PrintWriter(new File(d,"animalattack11.txt"));
  for(int age:new int[]{-1,0,1})for(int al:new int[]{0,600})for(int bl:new int[]{0,600})for(float distance:new float[]{2,2.5F,3.5F,4}){
   W w=world();EntityCow a=new EntityCow(w),b=new EntityCow(w);a.setDelay(age);b.setDelay(age>0?-1:0);love.setInt(a,al);love.setInt(b,bl);breed.setInt(a,7);a.entityToAttack=b;a.rand.setSeed(42);attack.invoke(a,b,distance);o.println(age+" "+al+" "+bl+" "+Float.floatToRawIntBits(distance)+" "+(a.entityToAttack==b?1:0)+" "+breed.getInt(a)+" "+love.getInt(a)+" "+love.getInt(b)+" "+(a.hasAttacked?1:0)+" "+a.rand.nextLong());
  }o.close();
 }
}
