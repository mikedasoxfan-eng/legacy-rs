package net.minecraft.src;
import java.io.*;
import java.util.*;

/** Round 4 calls original entity bytecode; shared fixtures/recorders are unchanged. */
public final class Mode_entityround4_10 {
 static void i(DataOutputStream o,int n)throws Exception{EntityOracle.i(o,n);}
 static void state(World w,Entity e,DataOutputStream o)throws Exception{
  Mode_entities10.state(w,e,o);
  if(e instanceof EntityPigZombie){i(o,Trace10.namedInt("EntityOracle.intField(EntityPigZombie.class,\"angerLevel\",e)", EntityOracle.intField(EntityPigZombie.class,"angerLevel",e)));i(o,Trace10.namedInt("EntityOracle.intField(EntityPigZombie.class,\"randomSoundDelay\",e)", EntityOracle.intField(EntityPigZombie.class,"randomSoundDelay",e)));}
  if(e instanceof EntityWolf){EntityWolf q=(EntityWolf)e;Trace10.writeBoolean(o, "q.isWolfAngry()", q.isAngry());Trace10.writeBoolean(o, "q.isWolfTamed()", q.isTamed());Trace10.writeBoolean(o, "q.isWolfSitting()", q.isSitting());}
  if(e instanceof EntityBoat){EntityBoat q=(EntityBoat)e;for(int n:new int[]{Trace10.namedInt("q.func_41020_g()", q.getDamageTaken()),Trace10.namedInt("q.func_41018_h()", q.getTimeSinceHit()),Trace10.namedInt("q.func_41016_i()", q.getForwardDirection()),Trace10.namedInt("EntityOracle.intField(EntityBoat.class,\"boatPosRotationIncrements\",q)", EntityOracle.intField(EntityBoat.class,"boatPosRotationIncrements",q))})i(o,n);}
  if(e instanceof EntityMinecart){EntityMinecart q=(EntityMinecart)e;for(int n:new int[]{Trace10.namedInt("q.func_41025_i()", q.func_41025_i()),Trace10.namedInt("q.func_41023_l()", q.func_41023_l()),Trace10.namedInt("q.func_41030_m()", q.func_41030_m()),Trace10.namedInt("EntityOracle.intField(EntityMinecart.class,\"minecartPosRotationIncrements\",q)", EntityOracle.intField(EntityMinecart.class,"minecartPosRotationIncrements",q))})i(o,n);}
  if(e instanceof EntityLiving)i(o,Trace10.namedInt("EntityOracle.intField(EntityLiving.class,\"newPosRotationIncrements\",e)", EntityOracle.intField(EntityLiving.class,"newPosRotationIncrements",e)));
 }
 static Entity create(World w,String name){
  if(name.equals("Fish"))return new EntityFishHook(w);
  if(name.equals("Item"))return new EntityItem(w,-3.5,68,4.5,new ItemStack(263,1,0));
  if(name.equals("Boat"))return new EntityBoat(w,-3.5,68,4.5);
  if(name.equals("Minecart"))return new EntityMinecart(w,-3.5,68,4.5,0);
  if(name.equals("Player"))return new Mode_entities10.Player(w);
  return EntityList.createEntityInWorld(name,w);
 }
 static void network(DataOutputStream o)throws Exception{
  for(String name:new String[]{"Item","Zombie","Ghast","Boat","Minecart","Fish"})for(boolean remote:new boolean[]{false,true})for(int count:new int[]{-4,0,1,3,7})for(double y:new double[]{62.5,64,70.25})for(float yaw:new float[]{-720,-179,181,721}){
   World w=Mode_entities10.world();w.multiplayerWorld=remote;Entity e=create(w,name);e.setPosition(-3.5,68,4.5);e.rotationYaw=179;e.rotationPitch=-179;e.onGround=true;Harness10.setInt(e,"fire",41);
   if(e instanceof EntityLiving); /* removed in original 1.1 */
   e.setVelocity(.125,-.25,.375);e.motionX=.9;e.motionY=.8;e.motionZ=.7;
   e.setPositionAndRotation2(-2.25,y,5.75,yaw,541,count);state(w,e,o);
   // Base Fish has a null angler; stop before entering its ordinary gameplay branch.
   for(int tick=0;tick<(name.equals("Fish")?Math.max(count,0):12);tick++){e.onUpdate();state(w,e,o);}
  }
 }
 static void lightning(DataOutputStream o)throws Exception{
  for(String name:new String[]{"Pig","Cow","Chicken","Sheep","Wolf","Zombie","Skeleton","Spider","CaveSpider","Creeper","PigZombie","Ghast","Squid","Slime","Silverfish","Player"})for(int fire:new int[]{-1,0,19,20,100})for(int hearts:new int[]{0,20}){
   World w=Mode_entities10.world();EntityLiving e=(EntityLiving)create(w,name);e.setPosition(-3.5,68,4.5);e.health=20;e.heartsLife=hearts;Harness10.setInt(e,"fire",fire);
   if(e instanceof EntityPlayer)((EntityPlayer)e).inventory.armorInventory[3]=new ItemStack(306,1,0);
   EntityLightningBolt bolt=new EntityLightningBolt(w,0,85,0);
   for(int tick=0;tick<3;tick++){e.onStruckByLightning(bolt);state(w,e,o);}
  }
  for(int difficulty=0;difficulty<4;difficulty++)for(int seed=0;seed<8;seed++){
   World w=Mode_entities10.world();w.difficultySetting=difficulty;OracleHooks.reset(100+seed);
   EntityLightningBolt bolt=new EntityLightningBolt(w,-3.5,64,4.5);
   for(String name:new String[]{"Pig","Creeper","Wolf","PigZombie"}){Entity e=create(w,name);e.setPosition(-3.5,64,4.5);w.spawnEntityInWorld(e);}
   for(int tick=0;tick<40;tick++){
    bolt.onUpdate();state(w,bolt,o);i(o,Trace10.namedInt("EntityOracle.intField(EntityLightningBolt.class,\"lightningState\",bolt)", EntityOracle.intField(EntityLightningBolt.class,"lightningState",bolt)));i(o,Trace10.namedInt("EntityOracle.intField(EntityLightningBolt.class,\"boltLivingTime\",bolt)", EntityOracle.intField(EntityLightningBolt.class,"boltLivingTime",bolt)));EntityOracle.l(o,Trace10.namedLong("bolt.boltVertex", bolt.boltVertex));i(o,Trace10.namedInt("w.lightningFlash", w.lightningFlash));
    for(int x=-5;x<=-2;x++)for(int y=63;y<=65;y++)for(int z=3;z<=6;z++)i(o,Trace10.namedInt("w.getBlockId(x,y,z)", w.getBlockId(x,y,z)));
    for(Object q:new ArrayList(w.loadedEntityList))state(w,(Entity)q,o);
   }
  }
 }
 static void retaliation(DataOutputStream o)throws Exception{
  for(String name:new String[]{"PigZombie","Wolf"})for(int source=0;source<6;source++)for(int damage:new int[]{0,4,25})for(int hearts:new int[]{0,20})for(boolean dead:new boolean[]{false,true}){
   World w=Mode_entities10.world();EntityLiving e=(EntityLiving)create(w,name);e.setPosition(-3.5,68,4.5);e.health=20;e.heartsLife=hearts;e.isDead=dead;
   EntityLiving attacker=(EntityLiving)create(w,source==2?"Zombie":"Player");attacker.setPosition(0.5,68,.5);w.spawnEntityInWorld(attacker);
   for(int j=0;j<5;j++){Entity a=create(w,name);a.setPosition(-3.5+(j==4?40:j*3),68,4.5);if(a instanceof EntityWolf&&j==2)((EntityWolf)a).setIsTamed(true);w.spawnEntityInWorld(a);}
   EntityArrow arrow=new EntityArrow(w);
   DamageSource s=source==0?DamageSource.generic:source==1?DamageSource.causePlayerDamage((EntityPlayer)attacker):source==2?DamageSource.causeMobDamage(attacker):source==3?DamageSource.causeArrowDamage(arrow,attacker):source==4?DamageSource.causeArrowDamage(arrow,null):DamageSource.causeFireballDamage(new EntityFireball(w),attacker);
   for(int j=0;j<3;j++){Trace10.writeBoolean(o, "e.attackEntityFrom(s,damage)", e.attackEntityFrom(s,damage));state(w,e,o);for(Object q:new ArrayList(w.loadedEntityList))state(w,(Entity)q,o);}
  }
 }
 static void impact(DataOutputStream o)throws Exception{
  for(String name:new String[]{"Pig","Ghast","Enderman","Creeper"})for(int owner=0;owner<3;owner++)for(boolean remote:new boolean[]{false,true})for(boolean block:new boolean[]{false,true}){
   World w=Mode_entities10.world();w.multiplayerWorld=remote;
   EntityLiving target=(EntityLiving)create(w,name);target.setPosition(-3.5,85.25,5.5);w.spawnEntityInWorld(target);
   EntityLiving shooter=owner==0?null:(EntityLiving)create(w,owner==1?"Zombie":"Player");if(shooter!=null){shooter.setPosition(0,90,0);w.spawnEntityInWorld(shooter);}
   EntityFireball ball=shooter==null?new EntityFireball(w,-3.5,85.25,4.5,.4,.2,1):new EntityFireball(w,shooter,.4,.2,1);ball.setPosition(-3.5,85.25,4.5);ball.setVelocity(0,0,1);
   if(block)EntityOracle.rawBlock(w,-4,85,5,1,0);
   state(w,ball,o);ball.onUpdate();state(w,ball,o);state(w,target,o);
  }
 }
 static void flying(DataOutputStream o)throws Exception{
  for(int medium=0;medium<3;medium++)for(boolean ground:new boolean[]{false,true})for(float strafe:new float[]{-1,0,.25F})for(float forward:new float[]{-1,0,1})for(float yaw:new float[]{0,35,181}){
   World w=Mode_entities10.world();EntityGhast e=new EntityGhast(w);e.setPosition(-3.5,64,4.5);e.rotationYaw=yaw;e.onGround=ground;e.motionX=.125;e.motionY=-.25;e.motionZ=.375;e.prevPosX=-3.75;e.prevPosY=64;e.prevPosZ=4.125;
   if(medium!=0)for(int x=-6;x<=0;x++)for(int y=64;y<=69;y++)for(int z=2;z<=7;z++)EntityOracle.rawBlock(w,x,y,z,medium==1?9:11,0);
   e.inWater=medium==1;
   for(int j=0;j<4;j++){e.moveEntityWithHeading(strafe,forward);state(w,e,o);Trace10.writeBoolean(o, "e.isOnLadder()", e.isOnLadder());}
  }
 }
 static void explosionBlocks(DataOutputStream o)throws Exception{
  for(float size:new float[]{.25F,1,2,4})for(int scene=0;scene<8;scene++)for(boolean flame:new boolean[]{false,true}){
   World w=Mode_entities10.world();int[] blocks={1,5,20,35,46,98,87,2};
   for(int x=-6;x<=-1;x++)for(int y=83;y<=86;y++)for(int z=2;z<=7;z++){
    int id=scene==7?blocks[Math.floorMod(x+y+z,blocks.length)]:blocks[scene];EntityOracle.rawBlock(w,x,y,z,id,id==35?12:id==98?2:0);
   }
   Explosion ex=w.newExplosion(null,-3.5,85.25,4.5,size,flame);
   i(o,Trace10.namedInt("ex.destroyedBlockPositions.size()", ex.destroyedBlockPositions.size()));for(Object p:ex.destroyedBlockPositions){ChunkPosition q=(ChunkPosition)p;i(o,Trace10.namedInt("q.x", q.x));i(o,Trace10.namedInt("q.y", q.y));i(o,Trace10.namedInt("q.z", q.z));}
   for(int x=-7;x<=0;x++)for(int y=82;y<=87;y++)for(int z=1;z<=8;z++){i(o,Trace10.namedInt("w.getBlockId(x,y,z)", w.getBlockId(x,y,z)));i(o,Trace10.namedInt("w.getBlockMetadata(x,y,z)", w.getBlockMetadata(x,y,z)));}
   i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));for(Object q:w.loadedEntityList){Entity e=(Entity)q;state(w,e,o);if(e instanceof EntityItem){ItemStack a=((EntityItem)e).item;i(o,Trace10.namedInt("a.itemID", a.itemID));i(o,Trace10.namedInt("a.stackSize", a.stackSize));i(o,Trace10.namedInt("a.getItemDamage()", a.getItemDamage()));}if(e instanceof EntityTNTPrimed)i(o,Trace10.namedInt("((EntityTNTPrimed)e).fuse", ((EntityTNTPrimed)e).fuse));}
   EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));WorldEventOracle.snapshot(w,o);
  }
 }
 static void creeper(DataOutputStream o)throws Exception{
  for(boolean powered:new boolean[]{false,true})for(float distance:new float[]{2.99F,3,6.99F,7,10})for(boolean visible:new boolean[]{false,true})for(int initial:new int[]{-1,1})for(int ignition:new int[]{0,1,29})for(boolean remote:new boolean[]{false,true}){
   World w=Mode_entities10.world();w.multiplayerWorld=remote;EntityCreeper e=new EntityCreeper(w);e.setPosition(-3.5,85.25,4.5);e.timeSinceIgnited=ignition;e.dataWatcher.updateObject(16,Byte.valueOf((byte)initial));e.dataWatcher.updateObject(17,Byte.valueOf((byte)(powered?1:0)));
   EntityPig p=new EntityPig(w);p.setPosition(-3.5,85.25,5.5);w.spawnEntityInWorld(p);
   for(int j=0;j<2;j++){if(visible)e.attackEntity(p,distance);else e.attackBlockedEntity(p,distance);state(w,e,o);i(o,Trace10.namedInt("e.timeSinceIgnited", e.timeSinceIgnited));i(o,Trace10.namedInt("e.dataWatcher.getWatchableObjectByte(16)", e.dataWatcher.getWatchableObjectByte(16)));Trace10.writeBoolean(o, "e.hasAttacked", e.hasAttacked);state(w,p,o);}
  }
 }
 static void creeperDeath(DataOutputStream o)throws Exception{
  for(int source=0;source<6;source++)for(boolean remote:new boolean[]{false,true})for(int seed=0;seed<3;seed++){
   World w=Mode_entities10.world();w.multiplayerWorld=remote;EntityCreeper e=new EntityCreeper(w);e.setPosition(-3.5,85.25,4.5);e.rand.setSeed(seed);e.health=4;
   EntityLiving attacker=(EntityLiving)create(w,source==2?"Player":"Skeleton");w.spawnEntityInWorld(attacker);EntityArrow arrow=new EntityArrow(w);
   DamageSource damage=source==0?DamageSource.generic:source==1?DamageSource.causeMobDamage(attacker):source==2?DamageSource.causePlayerDamage((EntityPlayer)attacker):source==3?DamageSource.causeArrowDamage(arrow,attacker):source==4?DamageSource.causeArrowDamage(arrow,null):DamageSource.causeFireballDamage(new EntityFireball(w),attacker);
   Trace10.writeBoolean(o, "e.attackEntityFrom(damage,10)", e.attackEntityFrom(damage,10));state(w,e,o);i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));for(Object q:w.loadedEntityList){Entity a=(Entity)q;state(w,a,o);if(a instanceof EntityItem){ItemStack item=((EntityItem)a).item;i(o,Trace10.namedInt("item.itemID", item.itemID));i(o,Trace10.namedInt("item.stackSize", item.stackSize));i(o,Trace10.namedInt("item.getItemDamage()", item.getItemDamage()));}}
  }
 }
 static void sun(DataOutputStream o)throws Exception{
  for(String name:new String[]{"Zombie","Skeleton","PigZombie"})for(boolean remote:new boolean[]{false,true})for(boolean day:new boolean[]{false,true})for(boolean covered:new boolean[]{false,true})for(int seed=0;seed<16;seed++){
   World w=Mode_entities10.world();w.multiplayerWorld=remote;w.skylightSubtracted=day?0:11;EntityLiving e=(EntityLiving)create(w,name);e.setPosition(-3.5,85.25,4.5);; /* removed in original 1.1 */e.rand.setSeed(seed);Harness10.setInt(e,"fire",13);
   if(covered)EntityOracle.rawBlock(w,-4,88,4,1,0);
   for(int j=0;j<8;j++){e.onUpdate();state(w,e,o);}
  }
 }
 public static void main(String[] a)throws Exception{
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))){
   switch(a[1]){case "creeperdeath4":creeperDeath(o);break;case "sun4":sun(o);break;case "network4":network(o);break;case "lightning4":lightning(o);break;case "retaliation4":retaliation(o);break;case "impact4":impact(o);break;case "flying4":flying(o);break;case "explosionblocks4":explosionBlocks(o);break;case "creeper4":creeper(o);break;default:throw new IllegalArgumentException(a[1]);}
  }
 }
}
