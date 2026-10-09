package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
public final class SheepSlice11 {
 static final class W extends World {
  int feet,meta,below,sfx;List<Entity> drops=new ArrayList<Entity>();
  W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}
  public int getBlockId(int x,int y,int z){return y==64?feet:y==63?below:0;}
  public int getBlockMetadata(int x,int y,int z){return meta;}
  public boolean setBlockWithNotify(int x,int y,int z,int id){if(y==64)feet=id;if(y==63)below=id;return true;}
  public void playAuxSFX(int code,int x,int y,int z,int data){sfx=data;}
  public void setEntityState(Entity e,byte b){}
  public EntityPlayer getClosestPlayerToEntity(Entity e,double d){return null;}
  public List getEntitiesWithinAABBExcludingEntity(Entity e,AxisAlignedBB box){return Collections.emptyList();}
  public boolean spawnEntityInWorld(Entity e){drops.add(e);return true;}
 }
 static final class S extends EntitySheep {
  S(World w){super(w);}
  protected Entity findPlayerToAttack(){return null;}
  protected void despawnEntity(){}
  public boolean handleLavaMovement(){return false;}
  public boolean isInWater(){return false;}
  public void moveEntityWithHeading(float a,float b){}
  void action(){super.updateEntityActionState();}
 }
 static final class M extends EntityMagmaCube {M(World w){super(w);}void drops(int l){super.dropFewItems(false,l);}}
 static sun.misc.Unsafe unsafe()throws Exception{Field f=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");f.setAccessible(true);return(sun.misc.Unsafe)f.get(null);}
 static Field timer()throws Exception{Field f=EntitySheep.class.getDeclaredField("field_44004_b");f.setAccessible(true);return f;}
 static W world()throws Exception{W w=(W)unsafe().allocateInstance(W.class);w.drops=new ArrayList<Entity>();return w;}
 public static void main(String[] args)throws Exception{
  int boot=Block.stone.blockID;File dir=new File(args[0]);Field t=timer();
  long adultSeed=200;for(;;adultSeed++){W w=world();w.below=2;S sheep=new S(w);sheep.setPosition(0,64,0);sheep.entityAge=100;sheep.rand.setSeed(adultSeed);sheep.action();if(t.getInt(sheep)==40)break;}
  PrintWriter out=new PrintWriter(new File(dir,"sheep_actions11.txt"));
  for(int ix=0;ix<=200;ix++)for(int age:new int[]{0,-1,-1200,-2400})for(int timer:new int[]{0,4,5,40})for(int food=0;food<4;food++){
   W w=world();w.feet=food>=2?31:0;w.meta=food==3?2:1;w.below=food==1?2:3;
   long seed=ix==200?adultSeed:ix;S s=new S(w);s.setPosition(0,64,0);s.entityAge=100;s.setDelay(age);s.setSheared(true);s.rand.setSeed(seed);t.setInt(s,timer);s.action();
   out.println(seed+" "+age+" "+timer+" "+food+" "+t.getInt(s)+" "+s.getDelay()+" "+(s.getSheared()?1:0)+" "+w.feet+" "+w.below+" "+w.sfx+" "+s.rand.nextLong());
  }out.close();
  out=new PrintWriter(new File(dir,"sheep_ticks11.txt"));
  for(int age:new int[]{0,-1,-1199,-1200,-1201,-2400,1})for(int timer:new int[]{0,1,4,5,39,40})for(int food=0;food<4;food++){W w=world();w.feet=food>=2?31:0;w.meta=food==3?2:1;w.below=food==1?2:3;S sheep=new S(w);sheep.setPosition(0,64,0);sheep.entityAge=100;sheep.setDelay(age);sheep.setSheared(true);sheep.rand.setSeed(42);t.setInt(sheep,timer);sheep.onLivingUpdate();out.println(age+" "+timer+" "+food+" "+t.getInt(sheep)+" "+sheep.getDelay()+" "+(sheep.getSheared()?1:0)+" "+w.feet+" "+w.below+" "+sheep.rand.nextLong());}out.close();
  out=new PrintWriter(new File(dir,"sheep_heads11.txt"));S s=new S(world());s.rotationPitch=37.5F;
  for(int timer:new int[]{-1,0,1,3,4,5,35,36,37,39,40,41})for(float p:new float[]{-.25F,0F,.25F,.5F,1F}){t.setInt(s,timer);out.println(timer+" "+Float.floatToRawIntBits(p)+" "+Float.floatToRawIntBits(s.func_44003_c(p))+" "+Float.floatToRawIntBits(s.func_44002_d(p)));}out.close();
  out=new PrintWriter(new File(dir,"magma11.txt"));
  for(long ix=0;ix<100;ix++)for(int size:new int[]{1,2,4})for(int loot:new int[]{0,1,3}){long seed=ix*341873128712L;W w=world();M m=new M(w);m.setSlimeSize(size);m.rand.setSeed(seed);m.drops(loot);out.println(seed+" "+size+" "+loot+" "+w.drops.size()+" "+m.rand.nextLong());}out.close();
  out=new PrintWriter(new File(dir,"shearing11.txt"));
  for(long seed=0;seed<100;seed++)for(int child=0;child<2;child++)for(int remote=0;remote<2;remote++)for(int sh=0;sh<2;sh++){
   W w=world();w.multiplayerWorld=remote==1;S sheep=new S(w);sheep.setDelay(child==1?-1:0);sheep.setSheared(sh==1);sheep.setFleeceColor(7);sheep.rand.setSeed(seed);
   EntityPlayerSP p=(EntityPlayerSP)unsafe().allocateInstance(EntityPlayerSP.class);p.worldObj=w;p.rand=new Random(42);p.inventory=new InventoryPlayer(p);p.capabilities=new PlayerCapabilities();ItemStack shears=new ItemStack(Item.shears);p.inventory.mainInventory[0]=shears;
   boolean result=sheep.interact(p);out.println(seed+" "+child+" "+remote+" "+sh+" "+w.drops.size()+" "+shears.getItemDamage()+" "+(sheep.getSheared()?1:0)+" "+(result?1:0)+" "+sheep.rand.nextLong());
  }out.close();
 }
}
