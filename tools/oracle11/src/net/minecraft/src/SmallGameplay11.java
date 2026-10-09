package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
/** Actual original brewing updateEntity and Creeper.onDeath callbacks. */
public final class SmallGameplay11 {
 static final class W extends World {
  int meta,notifications;List<Entity> drops;
  W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}
  public void setBlockMetadataWithNotify(int x,int y,int z,int m){meta=m;notifications++;}
  public int getBlockMetadata(int x,int y,int z){return meta;}
  public void setEntityState(Entity e,byte b){}
  public boolean spawnEntityInWorld(Entity e){drops.add(e);return true;}
 }
 static final class B extends TileEntityBrewingStand {int dirty;public void onInventoryChanged(){dirty++;}}
 static W world()throws Exception{W w=(W)SheepSlice11.unsafe().allocateInstance(W.class);w.rand=new Random(42);w.drops=new ArrayList<Entity>();w.worldHeight=128;return w;}
 static int get(Object o,String key)throws Exception{Field f=o instanceof B?TileEntityBrewingStand.class.getDeclaredField(key):EntityLiving.class.getDeclaredField(key);f.setAccessible(true);return f.getInt(o);}
 static long rng(Random r)throws Exception{Field f=Random.class.getDeclaredField("seed");f.setAccessible(true);return ((java.util.concurrent.atomic.AtomicLong)f.get(r)).get();}
 public static void main(String[] args)throws Exception{
  int boot=Block.stone.blockID;Field tileNames=TileEntity.class.getDeclaredField("classToNameMap");tileNames.setAccessible(true);((Map)tileNames.get(null)).put(B.class,"Cauldron");File dir=new File(args[0]);dir.mkdirs();PrintWriter out=new PrintWriter(new File(dir,"brewing11.txt"));
  for(int ingredient:new int[]{372,353,289})for(int damage:new int[]{0,16})for(int scenario=0;scenario<4;scenario++){
   W w=world();B b=new B();b.worldObj=w;b.setInventorySlotContents(0,new ItemStack(373,1,damage));b.setInventorySlotContents(2,new ItemStack(373,1,damage));b.setInventorySlotContents(3,new ItemStack(ingredient,2,0));
   for(int step=0;step<402;step++){
    if(step==200&&scenario==1)b.setInventorySlotContents(3,new ItemStack(ingredient==372?353:372,2,0));
    if(step==399&&scenario==2)b.setInventorySlotContents(3,null);
    if(step==200&&scenario==3){NBTTagCompound n=new NBTTagCompound();b.writeToNBT(n);B replacement=new B();replacement.worldObj=w;replacement.readFromNBT(n);b=replacement;}
    b.dirty=0;w.notifications=0;b.updateEntity();out.print(ingredient+" "+damage+" "+scenario+" "+step+" "+b.getBrewTime()+" "+get(b,"ingredientID")+" "+get(b,"filledSlots")+" "+b.dirty+" "+w.notifications+" "+w.meta);
    for(int slot=0;slot<4;slot++){ItemStack s=b.getStackInSlot(slot);out.print(" "+(s==null?-1:s.itemID)+" "+(s==null?0:s.stackSize)+" "+(s==null?0:s.getItemDamage()));}out.println(" "+rng(w.rand));
   }
  }out.close();out=new PrintWriter(new File(dir,"creeperrecords11.txt"));
  for(int seed=0;seed<200;seed++)for(int source=0;source<3;source++)for(int remote=0;remote<2;remote++){
   W w=world();w.multiplayerWorld=remote!=0;EntityCreeper c=new EntityCreeper(w);c.setPosition(.5,64,.5);c.rand.setSeed(seed);Entity attacker=source==0?new EntitySkeleton(w):source==1?new EntityZombie(w):null;
   c.onDeath(attacker==null?DamageSource.generic:DamageSource.causeMobDamage((EntityLiving)attacker));out.print(seed+" "+source+" "+remote+" "+w.drops.size());for(Entity e:w.drops){ItemStack s=((EntityItem)e).item;out.print(" "+s.itemID+" "+s.stackSize+" "+s.getItemDamage());}out.println(" "+rng(c.rand)+" "+rng(w.rand));
  }out.close();
 }
}
