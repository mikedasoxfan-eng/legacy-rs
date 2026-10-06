package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Branch matrices executed on original entity bytecode, with pinned RNGs. */
public final class Mode_entitybranches10 {
 // HotSpot 8u51 stores a 31-bit identity hash in mark-word bits 8..38.
 // Pin that VM-dependent input while leaving SpawnerAnimals and HashMap bytecode intact.
 static final class PinnedSpawnSet extends HashSet<ChunkCoordIntPair> {
  final sun.misc.Unsafe unsafe;
  PinnedSpawnSet()throws Exception{java.lang.reflect.Field f=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");f.setAccessible(true);unsafe=(sun.misc.Unsafe)f.get(null);}
  public boolean add(ChunkCoordIntPair p){int hash=(((p.chunkXPos&0x7fff)<<16)|(p.chunkZPos&0xffff))+1;hash=Math.max(1,hash&0x7fffffff);unsafe.putLong(p,0L,1L|((long)hash<<8));return super.add(p);}
 }
 static final class PinnedSpawnMap extends HashMap<ChunkCoordIntPair,Boolean> {
  final sun.misc.Unsafe unsafe;
  PinnedSpawnMap()throws Exception{java.lang.reflect.Field f=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");f.setAccessible(true);unsafe=(sun.misc.Unsafe)f.get(null);}
  public Boolean put(ChunkCoordIntPair p,Boolean edge){int hash=(((p.chunkXPos&0x7fff)<<16)|(p.chunkZPos&0xffff))+1;hash=Math.max(1,hash&0x7fffffff);unsafe.putLong(p,0L,1L|((long)hash<<8));return super.put(p,edge);}
 }
 static void i(DataOutputStream o,int n)throws Exception{EntityOracle.i(o,n);}
 static void d(DataOutputStream o,double n)throws Exception{EntityOracle.d(o,n);}
 static void f(DataOutputStream o,float n)throws Exception{EntityOracle.f(o,n);}
 static void state(World w,Entity e,DataOutputStream o)throws Exception{Mode_entities10.state(w,e,o);}
 static void slime(DataOutputStream o)throws Exception {
  for(int size:new int[]{1,2,3,4,8,127,128,257})for(int health:new int[]{-1,0,1}){
   World w=Mode_entities10.world();EntitySlime e=new EntitySlime(w);e.setSlimeSize(size);e.health=health;e.setLocationAndAngles(-3.5,70.25,4.5,35,0);
   e.setEntityDead();state(w,e,o);i(o,Trace10.namedInt("e.getSlimeSize()", e.getSlimeSize()));
   for(Object c:w.loadedEntityList){EntitySlime s=(EntitySlime)c;state(w,s,o);i(o,Trace10.namedInt("s.getSlimeSize()", s.getSlimeSize()));}
  }
 }
 static void skeleton(DataOutputStream o)throws Exception {
  for(int cooldown:new int[]{0,1,60})for(float distance:new float[]{0,5,9.99F,10,12})for(float yaw:new float[]{0,35,179}){
   World w=Mode_entities10.world();EntitySkeleton e=new EntitySkeleton(w);EntityPig p=new EntityPig(w);
   e.setLocationAndAngles(-3.5,70.25,4.5,yaw,-17);p.setPosition(.5,71.125,7.25);e.attackTime=cooldown;
   e.attackEntity(p,distance);state(w,e,o);Trace10.writeBoolean(o, "e.hasAttacked", e.hasAttacked);
   for(Object a:w.loadedEntityList)state(w,(Entity)a,o);
  }
 }
 static void wolf(DataOutputStream o)throws Exception {
  for(int seed=0;seed<12;seed++)for(int flags:new int[]{0,2,4,5,6})for(int item:new int[]{-1,352,319,320,363,364,365,366,367,260})for(int health:new int[]{0,7,20}){
   World w=Mode_entities10.world();EntityWolf e=new EntityWolf(w);Mode_entities10.Player p=new Mode_entities10.Player(w);p.username="Player";
   e.setLocationAndAngles(-3.5,64,4.5,35,0);e.rand.setSeed(seed);e.dataWatcher.updateObject(16,Byte.valueOf((byte)flags));e.setOwner(seed%2==0?"pLaYeR":"Other");e.dataWatcher.updateObject(18,Integer.valueOf(health));e.health=health;e.isJumping=true;
   if(item>=0)p.inventory.mainInventory[0]=new ItemStack(item,seed%2+1,0);
   Trace10.writeBoolean(o, "e.interact(p)", e.interact(p));state(w,e,o);i(o,Trace10.namedInt("e.dataWatcher.getWatchableObjectByte(16)", e.dataWatcher.getWatchableObjectByte(16)));WorldEventOracle.text(o,e.getWolfOwner());i(o,Trace10.namedInt("e.dataWatcher.getWatchableObjectInt(18)", e.dataWatcher.getWatchableObjectInt(18)));
   ItemStack s=p.inventory.getCurrentItem();i(o,Trace10.namedInt("s==null?-1:s.itemID", s==null?-1:s.itemID));i(o,Trace10.namedInt("s==null?0:s.stackSize", s==null?0:s.stackSize));
  }
 }
 static void vehicles(DataOutputStream o)throws Exception {
  for(int kind=-1;kind<3;kind++)for(int damage:new int[]{-1,0,1,4,5,100,Integer.MAX_VALUE})for(boolean dead:new boolean[]{false,true}){
   World w=Mode_entities10.world();Entity e=kind<0?new EntityBoat(w,-3.5,68,4.5):new EntityMinecart(w,-3.5,68,4.5,kind);e.isDead=dead;
   if(kind>=0){EntityMinecart m=(EntityMinecart)e;m.setInventorySlotContents(0,new ItemStack(35,64,7));m.setInventorySlotContents(26,new ItemStack(263,9,0));}
   for(int j=0;j<3;j++){
    Trace10.writeBoolean(o, "e.attackEntityFrom(DamageSource.generic,damage)", e.attackEntityFrom(DamageSource.generic,damage));state(w,e,o);
    if(kind<0){EntityBoat b=(EntityBoat)e;i(o,Trace10.namedInt("b.func_41020_g()", b.func_41020_g()));i(o,Trace10.namedInt("b.func_41018_h()", b.func_41018_h()));i(o,Trace10.namedInt("b.func_41016_i()", b.func_41016_i()));}else{EntityMinecart m=(EntityMinecart)e;i(o,Trace10.namedInt("m.func_41025_i()", m.func_41025_i()));i(o,Trace10.namedInt("m.func_41023_l()", m.func_41023_l()));i(o,Trace10.namedInt("m.func_41030_m()", m.func_41030_m()));for(int slot:new int[]{0,26})i(o,Trace10.namedInt("m.getStackInSlot(slot).stackSize", m.getStackInSlot(slot).stackSize));}
    for(Object a:w.loadedEntityList){EntityItem b=(EntityItem)a;for(double v:new double[]{Trace10.namedDouble("b.motionX", b.motionX),Trace10.namedDouble("b.motionY", b.motionY),Trace10.namedDouble("b.motionZ", b.motionZ)})d(o,v);i(o,Trace10.namedInt("b.item.itemID", b.item.itemID));i(o,Trace10.namedInt("b.item.stackSize", b.item.stackSize));i(o,Trace10.namedInt("b.item.getItemDamage()", b.item.getItemDamage()));}
   }
  }
 }
 static void fireball(DataOutputStream o)throws Exception {
  for(int source=0;source<4;source++)for(float yaw:new float[]{0,35,179,-270})for(float pitch:new float[]{-90,-17,0,65,90}){
   World w=Mode_entities10.world();EntityFireball e=new EntityFireball(w,-3.5,85,4.5,.4,.2,1);EntityLiving p=source==1?new EntityPig(w):new Mode_entities10.Player(w);p.rotationYaw=yaw;p.rotationPitch=pitch;
   EntityArrow arrow=new EntityArrow(w);DamageSource s=source==0?DamageSource.generic:source==1?DamageSource.causeMobDamage(p):source==2?DamageSource.causeArrowDamage(arrow,p):DamageSource.causeArrowDamage(arrow,null);
   Trace10.writeBoolean(o, "e.attackEntityFrom(s,4)", e.attackEntityFrom(s,4));state(w,e,o);d(o,Trace10.namedDouble("e.accelerationX", e.accelerationX));d(o,Trace10.namedDouble("e.accelerationY", e.accelerationY));d(o,Trace10.namedDouble("e.accelerationZ", e.accelerationZ));
  }
 }
 static void spawning(DataOutputStream o)throws Exception {
  for(int scenario=0;scenario<4;scenario++)for(int seed=0;seed<16;seed++){
   World w=Mode_entities10.world();EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;
   for(int x=-10;x<=10;x++)for(int z=-10;z<=10;z++){
    byte[] b=new byte[32768];for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)b[(bx<<11)|(bz<<7)|63]=(byte)(scenario==3?1:2);
    Chunk c=new Chunk(w,b,x,z);c.isChunkLoaded=true;c.isTerrainPopulated=true;Arrays.fill(c.heightMap,(byte)64);Arrays.fill(c.skylightMap.data,(byte)(scenario==0?0:255));
    if(scenario==3)for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)for(int y=64;y<80;y++)c.blocks[(bx<<11)|(bz<<7)|y]=9;
    provider.chunks.put(provider.key(x,z),c);
   }
   java.lang.reflect.Field field=SpawnerAnimals.class.getDeclaredField("eligibleChunksForSpawning");field.setAccessible(true);field.set(null,new PinnedSpawnMap());
   w.skylightSubtracted=scenario==0?11:0;
   for(String name:new String[]{"spawnX","spawnY","spawnZ"}){java.lang.reflect.Field sp=WorldInfo.class.getDeclaredField(name);sp.setAccessible(true);sp.setInt(w.worldInfo,name.equals("spawnY")?64:0);}
   w.rand.setSeed(seed);
   Mode_entities10.Player p=new Mode_entities10.Player(w);p.setPosition(.5,64,.5);w.playerEntities.add(p);
   for(int t=0;t<2;t++){
    i(o,Trace10.namedInt("SpawnerAnimals.performSpawning(w,scenario==0,scenario!=0)", SpawnerAnimals.performSpawning(w,scenario==0,scenario!=0)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
    for(Object obj:w.loadedEntityList){Entity e=(Entity)obj;state(w,e,o);if(e instanceof EntitySheep)i(o,Trace10.namedInt("((EntitySheep)e).getFleeceColor()", ((EntitySheep)e).getFleeceColor()));}
   }
  }
 }
 static void spawnorder(DataOutputStream o)throws Exception {
  PinnedSpawnSet set=new PinnedSpawnSet();
  for(int t=0;t<3;t++) {set.clear();for(int x=-8;x<=8;x++)for(int z=-8;z<=8;z++)set.add(new ChunkCoordIntPair(x,z));
   for(int x=8;x>=-8;x--)for(int z=8;z>=-8;z--)Trace10.writeBoolean(o, "set.add(new ChunkCoordIntPair(x,z))", set.add(new ChunkCoordIntPair(x,z)));i(o,Trace10.namedInt("set.size()", set.size()));
   for(ChunkCoordIntPair p:set){i(o,Trace10.namedInt("p.chunkXPos", p.chunkXPos));i(o,Trace10.namedInt("p.chunkZPos", p.chunkZPos));i(o,Trace10.namedInt("System.identityHashCode(p)", System.identityHashCode(p)));}
  }
 }
 static void factories(DataOutputStream o)throws Exception {
  for(int id=-1;id<101;id++){
   World w=Mode_entities10.world();Entity e=EntityList.createEntity(id,w);Trace10.writeBoolean(o, "e!=null", e!=null);
   if(e==null)continue;WorldEventOracle.text(o,EntityList.getEntityString(e));i(o,Trace10.namedInt("EntityList.getEntityID(e)", EntityList.getEntityID(e)));state(w,e,o);
   // Registry's string constructor must consume the same constructor RNG/IDs.
   Entity named=EntityList.createEntityInWorld(EntityList.getEntityString(e),w);state(w,named,o);
   e.setLocationAndAngles(-3.5,70.25,4.5,395,-377);e.motionX=11;e.motionY=-.25;e.motionZ=-10;Harness10.setInt(e,"fire",71);e.func_41003_g(137);e.onGround=true;
   if(e instanceof EntityItem)((EntityItem)e).item=new ItemStack(35,7,12);
   if(e instanceof EntityPainting)((EntityPainting)e).art=EnumArt.values()[0];
   if(e instanceof EntityLiving){EntityLiving l=(EntityLiving)e;l.health=7;l.hurtTime=4;l.deathTime=3;l.attackTime=2;l.addPotionEffect(new PotionEffect(19,137,2));}
   if(e instanceof EntityEnderman){((EntityEnderman)e).setCarried(35);((EntityEnderman)e).setCarryingData(12);}
   NBTTagCompound n=new NBTTagCompound();n.setString("id",EntityList.getEntityString(e));e.writeToNBT(n);Entity loaded=EntityList.createEntityFromNBT(n,w);state(w,loaded,o);
  }
  for(String name:new String[]{"Mob","Monster","Player","Fireball","ThrownEgg","FishingHook","LightningBolt","unknown"}){World w=Mode_entities10.world();Trace10.writeBoolean(o, "EntityList.createEntityInWorld(name,w)!=null", EntityList.createEntityInWorld(name,w)!=null);}
 }
 public static void main(String[] a)throws Exception{
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))){
   switch(a[1]){case "slimebranches":slime(o);break;case "skeletonbranches":skeleton(o);break;case "wolfbranches":wolf(o);break;case "vehiclebranches":vehicles(o);break;case "fireballbranches":fireball(o);break;case "spawnbranches":spawning(o);break;case "spawnorder":spawnorder(o);break;case "factorybranches":factories(o);break;default:throw new IllegalArgumentException(a[1]);}
  }
 }
}
