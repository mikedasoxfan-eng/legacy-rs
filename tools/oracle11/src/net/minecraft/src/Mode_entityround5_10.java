package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;

/** Round 5 status, AI and vehicle calls run the original deobfuscated bytecode. */
public final class Mode_entityround5_10 {
 static void i(DataOutputStream o,int n)throws Exception{EntityOracle.i(o,n);}
 static void state(World w,Entity e,DataOutputStream o)throws Exception{
  Mode_entities10.state(w,e,o);
  i(o,Trace10.namedInt("e.ridingEntity==null?-1:e.ridingEntity.entityId", e.ridingEntity==null?-1:e.ridingEntity.entityId));i(o,Trace10.namedInt("e.riddenByEntity==null?-1:e.riddenByEntity.entityId", e.riddenByEntity==null?-1:e.riddenByEntity.entityId));
  if(e instanceof EntityWolf){EntityWolf q=(EntityWolf)e;i(o,Trace10.namedInt("q.dataWatcher.getWatchableObjectByte(16)", q.dataWatcher.getWatchableObjectByte(16)));i(o,Trace10.namedInt("q.dataWatcher.getWatchableObjectInt(18)", q.dataWatcher.getWatchableObjectInt(18)));
   for(String n:new String[]{"looksWithInterest","isWolfShaking","field_25052_g"}){Field f=EntityWolf.class.getDeclaredField(n);f.setAccessible(true);Trace10.writeBoolean(o, "f.getBoolean(q)", f.getBoolean(q));}
   for(String n:new String[]{"field_25048_b","field_25054_c","timeWolfIsShaking","prevTimeWolfIsShaking"}){Field f=EntityWolf.class.getDeclaredField(n);f.setAccessible(true);EntityOracle.f(o,Trace10.namedFloat("f.getFloat(q)", f.getFloat(q)));}
  }
  if(e instanceof EntityCreature){Field field=EntityCreature.class.getDeclaredField("pathToEntity");field.setAccessible(true);PathEntity p=(PathEntity)field.get(e);i(o,Trace10.namedInt("p==null?-1:p.pathLength", p==null?-1:p.pathLength));if(p!=null)for(int j=0;j<p.pathLength;j++){Field points=PathEntity.class.getDeclaredField("points");points.setAccessible(true);PathPoint v=((PathPoint[])points.get(p))[j];i(o,Trace10.namedInt("v.xCoord", v.xCoord));i(o,Trace10.namedInt("v.yCoord", v.yCoord));i(o,Trace10.namedInt("v.zCoord", v.zCoord));}}
 }
 static void health(DataOutputStream o)throws Exception{
  for(String name:new String[]{"Pig","Wolf","Zombie","Skeleton","Creeper","Squid","Slime","Enderman","Silverfish"})for(boolean remote:new boolean[]{false,true})for(int health:new int[]{0,7,20})for(byte status:new byte[]{0,2,3,6,7,8}){
   World w=Mode_entities10.world();w.multiplayerWorld=remote;EntityLiving e=(EntityLiving)EntityList.createEntityInWorld(name,w);e.setPosition(-3.5,70.25,4.5);e.health=health;e.attackedAtYaw=47;e.handleHealthUpdate(status);state(w,e,o);
   e.performHurtAnimation();state(w,e,o);
  }
  for(int health:new int[]{-1,0,1,19,20,100,Integer.MAX_VALUE})for(int amount:new int[]{-20,-1,0,1,20,Integer.MAX_VALUE,Integer.MIN_VALUE}){World w=Mode_entities10.world();EntityPig e=new EntityPig(w);e.health=health;e.heartsLife=47;e.heal(amount);state(w,e,o);}
 }
 static void wolfAttack(DataOutputStream o)throws Exception{
  for(int flags:new int[]{0,4})for(int seed=0;seed<24;seed++)for(float distance:new float[]{0,1.49F,1.5F,2,2.01F,5.99F,6})for(boolean ground:new boolean[]{false,true})for(int dy:new int[]{0,3}){
   World w=Mode_entities10.world();EntityWolf e=new EntityWolf(w);EntityPig p=new EntityPig(w);e.setLocationAndAngles(-3.5,70.25,4.5,35,0);p.setPosition(.5,70.25+dy,7.25);w.spawnEntityInWorld(p);e.dataWatcher.updateObject(16,Byte.valueOf((byte)flags));e.rand.setSeed(seed);e.onGround=ground;e.motionX=.125;e.motionZ=-.375;e.attackTime=17;
   e.attackEntity(p,distance);state(w,e,o);state(w,p,o);
  }
 }
 static void wolfFollow(DataOutputStream o)throws Exception{
  Method follow=EntityWolf.class.getDeclaredMethod("getPathOrWalkableBlock",Entity.class,float.class);follow.setAccessible(true);
  for(double distance:new double[]{5,5.01,12,12.01,25})for(int scene=0;scene<4;scene++){
   World w=Mode_entities10.world();EntityWolf e=new EntityWolf(w);EntityPig target=new EntityPig(w);e.setLocationAndAngles(-3.5,64,4.5,35,-17);target.setPosition(-3.5+distance,64,4.5);
   if(scene==1)for(int x=-5;x<=24;x++)for(int z=2;z<=7;z++)for(int y=64;y<=66;y++)EntityOracle.rawBlock(w,x,y,z,1,0);
   if(scene==2)for(int x=-6;x<=28;x++)for(int z=0;z<=8;z++)EntityOracle.rawBlock(w,x,63,z,0,0);
   if(scene==3)for(int x=-5;x<=24;x++)for(int z=2;z<=7;z++)for(int y=64;y<=66;y++)EntityOracle.rawBlock(w,x,y,z,9,0);
   follow.invoke(e,target,(float)distance);state(w,e,o);
  }
  for(boolean remote:new boolean[]{false,true})for(int item:new int[]{-1,352,319,320,363,364,365,366,367,260}){
   World w=Mode_entities10.world();w.multiplayerWorld=remote;EntityWolf e=new EntityWolf(w);Mode_entities10.Player p=new Mode_entities10.Player(w);; /* removed in original 1.1 */e.health=7;e.setIsTamed(true);Field current=EntityLiving.class.getDeclaredField("currentTarget");current.setAccessible(true);current.set(e,p);if(item>=0)p.inventory.mainInventory[0]=new ItemStack(item,1,0);e.onLivingUpdate();state(w,e,o);
  }
 }
 static void collisions(DataOutputStream o)throws Exception{
  for(int kind=-1;kind<3;kind++)for(String name:new String[]{"Pig","Player","Boat","Minecart0","Minecart1","Minecart2"})for(boolean remote:new boolean[]{false,true})for(double offset:new double[]{0,.009,.01,.5,4})for(double speed:new double[]{0,.1,.4})for(float reduction:new float[]{0,.5F,1}){
   World w=Mode_entities10.world();w.multiplayerWorld=remote;Entity a=kind<0?new EntityBoat(w,-3.5,68,4.5):new EntityMinecart(w,-3.5,68,4.5,kind);
   Entity b=name.equals("Player")?new Mode_entities10.Player(w):name.equals("Boat")?new EntityBoat(w):name.startsWith("Minecart")?new EntityMinecart(w,0,0,0,Integer.parseInt(name.substring(8))):new EntityPig(w);
   b.setPosition(-3.5+offset,68,4.5+offset);b.prevPosX=2.5;b.motionX=-.125;b.motionZ=.25;a.motionX=speed;a.motionZ=speed/2;a.entityCollisionReduction=reduction;
   a.applyEntityCollision(b);state(w,a,o);state(w,b,o);
  }
 }
 static final class TestPlayer extends EntityPlayer {
  int chests;int sniper;TestPlayer(World w){super(w);username="Player";}public void func_6420_o(){}
  public void displayGUIChest(IInventory v){chests++;}public void triggerAchievement(StatBase v){if(v==AchievementList.snipeSkeleton)sniper++;}
 }
 static void interactions(DataOutputStream o)throws Exception{
  for(int kind=-1;kind<3;kind++)for(boolean remote:new boolean[]{false,true})for(int occupied=0;occupied<4;occupied++)for(int count:new int[]{-1,0,1,2})for(int item:new int[]{-1,263,1}){
   World w=Mode_entities10.world();w.multiplayerWorld=remote;Entity a=kind<0?new EntityBoat(w,-3.5,68,4.5):new EntityMinecart(w,-3.5,68,4.5,kind);TestPlayer p=new TestPlayer(w);p.setPosition(.5,68,.5);if(item>=0)p.inventory.mainInventory[0]=new ItemStack(item,count,0);
   if(kind==2)Harness10.setInt(a,"fuel",Integer.MAX_VALUE-100);
   if(occupied>0){Entity r=occupied==1?p:occupied==2?new TestPlayer(w):new EntityPig(w);a.riddenByEntity=r;r.ridingEntity=a;w.spawnEntityInWorld(r);}
   Trace10.writeBoolean(o, "a.interact(p)", a.interact(p));state(w,a,o);state(w,p,o);i(o,Trace10.namedInt("p.chests", p.chests));ItemStack stack=p.inventory.getCurrentItem();i(o,Trace10.namedInt("stack==null?-1:stack.itemID", stack==null?-1:stack.itemID));i(o,Trace10.namedInt("stack==null?0:stack.stackSize", stack==null?0:stack.stackSize));
   if(a instanceof EntityMinecart){EntityMinecart m=(EntityMinecart)a;i(o,Trace10.namedInt("Harness10.getInt(m,\"fuel\")", Harness10.getInt(m,"fuel")));EntityOracle.d(o,Trace10.namedDouble("m.pushX", m.pushX));EntityOracle.d(o,Trace10.namedDouble("m.pushZ", m.pushZ));}
  }
 }
 static void skeleton(DataOutputStream o)throws Exception{
  for(int source=0;source<5;source++)for(double distance:new double[]{0,49.99,50,50.01,100})for(boolean remote:new boolean[]{false,true}){
   World w=Mode_entities10.world();w.multiplayerWorld=remote;EntitySkeleton e=new EntitySkeleton(w);TestPlayer p=new TestPlayer(w);p.setPosition(e.posX+distance,70,e.posZ);w.playerEntities.add(p);EntityArrow arrow=new EntityArrow(w);
   DamageSource s=source==0?DamageSource.generic:source==1?DamageSource.causePlayerDamage(p):source==2?DamageSource.causeArrowDamage(arrow,p):source==3?DamageSource.causeArrowDamage(arrow,null):new EntityDamageSource("mob",arrow);
   i(o,Trace10.namedInt("s.getEntity()==null?-1:s.getEntity().entityId", s.getEntity()==null?-1:s.getEntity().entityId));i(o,Trace10.namedInt("s.getSourceOfDamage()==null?-1:s.getSourceOfDamage().entityId", s.getSourceOfDamage()==null?-1:s.getSourceOfDamage().entityId));e.onDeath(s);state(w,e,o);i(o,Trace10.namedInt("p.sniper", p.sniper));
  }
 }

 static void watched(Entity e,DataOutputStream o)throws Exception {
  ByteArrayOutputStream bytes=new ByteArrayOutputStream();e.dataWatcher.writeWatchableObjects(new DataOutputStream(bytes));i(o,Trace10.namedInt("bytes.size()", bytes.size()));Trace10.writeBytes(o, "bytes.toByteArray()", bytes.toByteArray());
  Field changed=DataWatcher.class.getDeclaredField("objectChanged");changed.setAccessible(true);Trace10.writeBoolean(o, "changed.getBoolean(e.dataWatcher)", changed.getBoolean(e.dataWatcher));
 }
 static void watchers(DataOutputStream o)throws Exception {
  Method flag=Entity.class.getDeclaredMethod("setFlag",int.class,boolean.class);flag.setAccessible(true);
  Method get=Entity.class.getDeclaredMethod("getEntityFlag",int.class);get.setAccessible(true);
  for(String name:new String[]{"Item","XPOrb","Arrow","Snowball","Painting","Creeper","Skeleton","Spider","Giant","Zombie","Slime","PigZombie","Enderman","CaveSpider","Silverfish","Ghast","Pig","Sheep","Cow","Chicken","Squid","Wolf","Boat","Minecart"}) {
   World w=Mode_entities10.world();Entity e=EntityList.createEntityInWorld(name,w);watched(e,o);
   for(int bit:new int[]{-1,0,1,2,3,4,7,8,31,32,33,35})for(boolean value:new boolean[]{true,false}){flag.invoke(e,bit,value);watched(e,o);for(int n=0;n<5;n++)Trace10.writeBoolean(o, "(Boolean)get.invoke(e,n)", (Boolean)get.invoke(e,n));}
   java.util.List<WatchableObject> list=new java.util.ArrayList<WatchableObject>();list.add(new WatchableObject(0,0,Byte.valueOf((byte)27)));list.add(new WatchableObject(2,29,Integer.valueOf(123)));e.dataWatcher.updateWatchedObjectsFromList(list);watched(e,o);for(int n=0;n<5;n++)Trace10.writeBoolean(o, "(Boolean)get.invoke(e,n)", (Boolean)get.invoke(e,n));
  }
  for(boolean remote:new boolean[]{false,true})for(int fire:new int[]{-1,0,1,20})for(boolean riding:new boolean[]{false,true}) {
   World w=Mode_entities10.world();w.multiplayerWorld=remote;EntityPig e=new EntityPig(w);Harness10.setInt(e,"fire",fire);e.setPosition(-3.5,85.25,4.5);if(riding)e.ridingEntity=new EntityPig(w);e.setSprinting(true);e.setFlag(1,true);
   for(int tick=0;tick<3;tick++){e.onEntityUpdate();watched(e,o);i(o,Trace10.namedInt("Harness10.getInt(e,\"fire\")", Harness10.getInt(e,"fire")));i(o,Trace10.namedInt("e.func_41001_Z()", e.getAir()));}
  }
 }
 static void wolfRemote(DataOutputStream o)throws Exception {
  for(boolean remote:new boolean[]{false,true})for(int seed=0;seed<6;seed++)for(int flags:new int[]{0,2,4,5,6})for(int item:new int[]{-1,352,319,320,363,364,365,366,367,260})for(int health:new int[]{0,7,20}) {
   World w=Mode_entities10.world();w.multiplayerWorld=remote;EntityWolf e=new EntityWolf(w);Mode_entities10.Player p=new Mode_entities10.Player(w);p.username="Player";e.rand.setSeed(seed);e.setPosition(-3.5,64,4.5);e.dataWatcher.updateObject(16,Byte.valueOf((byte)flags));e.setOwner(seed%2==0?"pLaYeR":"Other");e.dataWatcher.updateObject(18,Integer.valueOf(health));e.health=health;e.isJumping=true;if(item>=0)p.inventory.mainInventory[0]=new ItemStack(item,seed%2+1,0);
   Trace10.writeBoolean(o, "e.interact(p)", e.interact(p));state(w,e,o);watched(e,o);ItemStack stack=p.inventory.getCurrentItem();i(o,Trace10.namedInt("stack==null?-1:stack.itemID", stack==null?-1:stack.itemID));i(o,Trace10.namedInt("stack==null?0:stack.stackSize", stack==null?0:stack.stackSize));
  }
 }
 static void potionExpiry(DataOutputStream o)throws Exception {
  for(boolean remote:new boolean[]{false,true})for(int id:new int[]{1,2,6,7,10,13,17,19})for(int duration:new int[]{-1,0,1,2,25})for(int amplifier:new int[]{0,1,31}) {
   World w=Mode_entities10.world();w.multiplayerWorld=remote;EntityPig e=new EntityPig(w);e.health=7;e.addPotionEffect(new PotionEffect(id,duration,amplifier));
   for(int tick=0;tick<4;tick++){e.updatePotionEffects();state(w,e,o);}
  }
 }
 static void dyeSheep(DataOutputStream o)throws Exception {
  for(String name:new String[]{"Sheep","Pig","Wolf"})for(boolean remote:new boolean[]{false,true})for(boolean sheared:new boolean[]{false,true})for(int color:new int[]{0,1,7,15})for(int damage:new int[]{Integer.MIN_VALUE,-17,-1,0,1,7,15,16,Integer.MAX_VALUE})for(int count:new int[]{Integer.MIN_VALUE,-1,0,1,2,Integer.MAX_VALUE}) {
   World w=Mode_entities10.world();w.multiplayerWorld=remote;EntityLiving e=(EntityLiving)EntityList.createEntityInWorld(name,w);if(e instanceof EntitySheep){((EntitySheep)e).setFleeceColor(color);((EntitySheep)e).setSheared(sheared);}
   ItemStack stack=new ItemStack(351,count,damage);Item.itemsList[351].useItemOnEntity(stack,e);state(w,e,o);watched(e,o);i(o,Trace10.namedInt("stack.itemID", stack.itemID));i(o,Trace10.namedInt("stack.stackSize", stack.stackSize));i(o,Trace10.namedInt("stack.getItemDamage()", stack.getItemDamage()));
  }
 }
 static void vehicleRiders(DataOutputStream o)throws Exception {
  for(int kind=-1;kind<3;kind++)for(String name:new String[]{"Player","Pig","Boat"})for(boolean remote:new boolean[]{false,true})for(boolean dead:new boolean[]{false,true})for(int scene=0;scene<3;scene++)for(double speed:new double[]{-.4,0,.4}) {
   World w=Mode_entities10.world();w.multiplayerWorld=remote;
   for(int x=-20;x<=20;x++)for(int z=2;z<=7;z++){if(scene==1)for(int y=64;y<68;y++)EntityOracle.rawBlock(w,x,y,z,9,0);if(scene==2)EntityOracle.rawBlock(w,x,64,z,66,1);}
   Entity a=kind<0?new EntityBoat(w,-3.5,65,4.5):new EntityMinecart(w,-3.5,65,4.5,kind);Entity r=name.equals("Player")?new TestPlayer(w):name.equals("Boat")?new EntityBoat(w):new EntityPig(w);r.isDead=dead;r.motionX=.3;r.motionZ=-.2;r.ridingEntity=a;a.riddenByEntity=r;a.motionX=speed;a.motionZ=-speed/2;a.rotationYaw=35;
   for(int tick=0;tick<12;tick++){a.onUpdate();if(a.riddenByEntity!=null)a.updateRiderPosition();state(w,a,o);state(w,r,o);}
  }
 }
 static void paintingItem(DataOutputStream o)throws Exception {
  for(int side:new int[]{-1,0,1,2,3,4,5,6})for(boolean remote:new boolean[]{false,true})for(int scene=0;scene<3;scene++)for(int y:new int[]{-1,0,63,64,127,128})for(int count:new int[]{Integer.MIN_VALUE,-1,0,1,2,Integer.MAX_VALUE}) {
   World w=Mode_entities10.world();w.multiplayerWorld=remote;TestPlayer p=new TestPlayer(w);
   if(scene>0)for(int x=-7;x<=1;x++)for(int z=0;z<=8;z++)for(int yy=0;yy<128;yy++)if((scene==1&&z==4)||(scene==2&&x==-3))EntityOracle.rawBlock(w,x,yy,z,1,0);
   ItemStack stack=new ItemStack(321,count,0);Trace10.writeBoolean(o, "Item.painting.onItemUse(stack,p,w,-3,y,4,side)", Item.painting.onItemUse(stack,p,w,-3,y,4,side));i(o,Trace10.namedInt("stack.stackSize", stack.stackSize));state(w,p,o);
   for(Object q:w.loadedEntityList){EntityPainting e=(EntityPainting)q;state(w,e,o);i(o,Trace10.namedInt("e.art.ordinal()", e.art.ordinal()));i(o,Trace10.namedInt("e.direction", e.direction));}
  }
 }
 static void recordItem(DataOutputStream o)throws Exception {
  for(int record:new int[]{2256,2257})for(boolean remote:new boolean[]{false,true})for(int block:new int[]{0,1,84})for(int meta:new int[]{0,1,15})for(int count:new int[]{Integer.MIN_VALUE,-1,0,1,2,Integer.MAX_VALUE})for(int side:new int[]{-1,0,1,2,5,6}) {
   World w=Mode_entities10.world();w.multiplayerWorld=remote;TestPlayer p=new TestPlayer(w);EntityOracle.rawBlock(w,-3,65,4,block,meta);ItemStack stack=new ItemStack(record,count,0);
   Trace10.writeBoolean(o, "stack.getItem().onItemUse(stack,p,w,-3,65,4,side)", stack.getItem().onItemUse(stack,p,w,-3,65,4,side));i(o,Trace10.namedInt("stack.stackSize", stack.stackSize));i(o,Trace10.namedInt("w.getBlockMetadata(-3,65,4)", w.getBlockMetadata(-3,65,4)));TileEntity tile=w.getBlockTileEntity(-3,65,4);i(o,Trace10.namedInt("tile instanceof TileEntityRecordPlayer?((TileEntityRecordPlayer)tile).record:-1", tile instanceof TileEntityRecordPlayer?((TileEntityRecordPlayer)tile).record:-1));state(w,p,o);
  }
 }
 public static void main(String[] a)throws Exception{
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))){
   switch(a[1]){case "recorditem5":recordItem(o);break;case "paintingitem5":paintingItem(o);break;case "vehicleriders5":vehicleRiders(o);break;case "dyesheep5":dyeSheep(o);break;case "potionexpiry5":potionExpiry(o);break;case "watchers5":watchers(o);break;case "wolfremote5":wolfRemote(o);break;case "health5":health(o);break;case "wolfattack5":wolfAttack(o);break;case "wolffollow5":wolfFollow(o);break;case "vehiclecollision5":collisions(o);break;case "vehicleinteract5":interactions(o);break;case "skeletondeath5":skeleton(o);break;default:throw new IllegalArgumentException(a[1]);}
  }
 }
}
