package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;
/** Concrete original EntityList constructors and malformed NBT, including null World. */
public final class Mode_entitylist10_10 {
 // Every 1.0 EntityList.addMapping, in registration (= id) order, then unregistered names.
 static final String[] names={"Item","XPOrb","Painting","Arrow","Snowball","Fireball","SmallFireball","ThrownEnderpearl","EyeOfEnderSignal","PrimedTnt","FallingSand","Minecart","Boat","Mob","Monster","Creeper","Skeleton","Spider","Giant","Zombie","Slime","Ghast","PigZombie","Enderman","CaveSpider","Silverfish","Blaze","LavaSlime","EnderDragon","Pig","Sheep","Cow","Chicken","Squid","Wolf","MushroomCow","SnowMan","Villager","EnderCrystal",null,"unknown","Player",""};
 static final int[] ids={1,2,9,10,11,12,13,14,15,20,21,40,41,48,49,50,51,52,53,54,55,56,57,58,59,60,61,62,63,90,91,92,93,94,95,96,97,120,200};
 static final String[] fields={"FallDistance","Fire","Air","OnGround","Health","HurtTime","DeathTime","AttackTime","ActiveEffects","Age","Value","carried","carryingData","Anger","Angry","Sitting","Owner","Saddle","Sheared","Color","powered","Size","Fuse","Tile","Dir","TileX","TileY","TileZ","Motive","xTile","yTile","zTile","inTile","inData","shake","inGround","player","Item","Type","Items","PushX","PushZ","Fuel"};
 static NBTTagList doubles(double... values){NBTTagList l=new NBTTagList();for(double v:values)l.setTag(new NBTTagDouble("", v));return l;}
 static NBTTagList floats(float... values){NBTTagList l=new NBTTagList();for(float v:values)l.setTag(new NBTTagFloat("", v));return l;}
 static NBTTagCompound nbt(String name,int scenario){
  NBTTagCompound n=new NBTTagCompound();if(name!=null)n.setString("id",name);
  if(scenario!=1)n.setTag("Pos",doubles(-3.5,70.25,4.5));if(scenario!=2)n.setTag("Motion",doubles(11,-.25,-10));if(scenario!=3)n.setTag("Rotation",floats(395,-377));
  if(scenario==4)n.setInteger("Pos",1);if(scenario==5)n.setTag("Motion",doubles(1,2));if(scenario==6)n.setTag("Pos",floats(1,2,3));if(scenario==7)n.setTag("Rotation",doubles(1,2));
  if(scenario==8){NBTTagList l=doubles(1,2,3);l.setTag(new NBTTagString("extra"));n.setTag("Motion",l);}
  if(scenario==9){NBTTagList l=doubles(1,2,3);l.setTag(new NBTTagString("extra"));n.setTag("Pos",l);}
  if(scenario==10){NBTTagList l=floats(1,2);l.setTag(new NBTTagString("extra"));n.setTag("Rotation",l);}
  if(scenario==11)n.setTag("Motion",doubles(Double.NaN,Double.POSITIVE_INFINITY,-Double.POSITIVE_INFINITY));
  if(scenario==12)n.setInteger("id",1);if(scenario==14)n.setString("id","unregistered");
  if(scenario==15){
   n.setFloat("FallDistance",-3.25F);for(String key:new String[]{"Fire","Air","Health","HurtTime","DeathTime","AttackTime","Age","Value","carried","carryingData","Anger","Fuel"})n.setShort(key,(short)-1);
   for(String key:new String[]{"OnGround","Angry","Sitting","Saddle","Sheared","Color","powered","Fuse","Tile","Dir","inTile","inData","shake","inGround","player"})n.setByte(key,(byte)-1);
   n.setString("Owner","Player");n.setString("Motive","Pointer");for(String key:new String[]{"Size","TileX","TileY","TileZ"})n.setInteger(key,17);n.setInteger("Type",1);
   NBTTagCompound item=new NBTTagCompound();item.setShort("id",(short)35);item.setByte("Count",(byte)-1);item.setShort("Damage",(short)32767);item.setByte("Slot",(byte)26);n.setCompoundTag("Item",item);NBTTagList items=new NBTTagList();items.setTag(item);n.setTag("Items",items);
   NBTTagCompound effect=new NBTTagCompound();effect.setByte("Id",(byte)19);effect.setByte("Amplifier",(byte)-1);effect.setInteger("Duration",137);NBTTagList effects=new NBTTagList();effects.setTag(effect);n.setTag("ActiveEffects",effects);
  }
  if(scenario>=16&&scenario<16+fields.length){n.setString(fields[scenario-16],"wrong");if(fields[scenario-16].equals("Items"))n.setInteger("Type",1);if(fields[scenario-16].startsWith("Push")||fields[scenario-16].equals("Fuel"))n.setInteger("Type",2);}
  if(scenario>=59){
   NBTTagCompound item=new NBTTagCompound();item.setShort("id",(short)new int[]{-1,0,2258,32767,35}[scenario-59]);item.setByte("Count",(byte)-128);item.setShort("Damage",(short)-32768);n.setCompoundTag("Item",item);item.setByte("Slot",(byte)0);NBTTagList items=new NBTTagList();items.setTag(item);n.setTag("Items",items);n.setInteger("Type",1);
  }
  return scenario==13?null:n;
 }
 static void extra(Entity e,DataOutputStream o)throws Exception {
  if(e instanceof EntityItem){ItemStack s=((EntityItem)e).item;Trace10.writeBoolean(o, "s!=null", s!=null);if(s!=null)for(int v:new int[]{Trace10.namedInt("s.itemID", s.itemID),Trace10.namedInt("s.stackSize", s.stackSize),Trace10.namedInt("s.getItemDamage()", s.getItemDamage())})EntityOracle.i(o,v);}
  if(e instanceof EntityMinecart){EntityMinecart m=(EntityMinecart)e;EntityOracle.i(o,Trace10.namedInt("m.minecartType", m.minecartType));for(int slot=0;slot<27;slot++){ItemStack s=m.getStackInSlot(slot);Trace10.writeBoolean(o, "s!=null", s!=null);if(s!=null)for(int v:new int[]{Trace10.namedInt("s.itemID", s.itemID),Trace10.namedInt("s.stackSize", s.stackSize),Trace10.namedInt("s.getItemDamage()", s.getItemDamage())})EntityOracle.i(o,v);}}
  if(e instanceof EntityPainting){EnumArt art=((EntityPainting)e).art;Trace10.writeBoolean(o, "art!=null", art!=null);if(art!=null)WorldEventOracle.text(o,art.title);}
  if(e instanceof EntitySheep)EntityOracle.i(o,Trace10.namedInt("((EntitySheep)e).getFleeceColor()", ((EntitySheep)e).getFleeceColor()));
  if(e instanceof EntityWolf)WorldEventOracle.text(o,((EntityWolf)e).getOwner());
 }
 static void result(World allocator,Entity e,String error,DataOutputStream o)throws Exception {
  WorldEventOracle.text(o,error);Trace10.writeBoolean(o, "e!=null", e!=null);if(e!=null){WorldEventOracle.text(o,EntityList.getEntityString(e));EntityOracle.i(o,Trace10.namedInt("EntityList.getEntityID(e)", EntityList.getEntityID(e)));Mode_entities10.state(allocator,e,o);extra(e,o);}
  Field ids=Entity.class.getDeclaredField("nextEntityID");ids.setAccessible(true);EntityOracle.i(o,Trace10.namedInt("ids.getInt(null)", ids.getInt(null)));EntityOracle.l(o,Trace10.namedLong("OracleHooks.nextSeed()", OracleHooks.nextSeed()));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(allocator.rand)", EntityOracle.rngState(allocator.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));
 }
 public static void main(String[] a)throws Exception {
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
   for(String name:names)for(boolean nullWorld:new boolean[]{false,true})for(int scenario=-2;scenario<64;scenario++) {
    World allocator=Mode_entities10.world(),w=nullWorld?null:allocator;Entity e=null;String error="";
    try{if(scenario==-2)e=EntityList.createEntityInWorld(name,w);else if(scenario==-1){int id=-1;for(int j=0;j<ids.length;j++)if(names[j].equals(name))id=ids[j];e=EntityList.createEntity(id,w);}else e=EntityList.createEntityFromNBT(nbt(name,scenario),w);}catch(RuntimeException ex){error=ex.getClass().getSimpleName();}
    result(allocator,e,error,o);
   }
   World w=Mode_entities10.world();Entity[] unregistered={null,new Mode_entities10.Player(w),new EntityEgg(w),new EntityFireball(w),new EntityLightningBolt(w,0,0,0)};
   for(Entity e:unregistered){String error="",name=null;int id=0;try{id=EntityList.getEntityID(e);}catch(RuntimeException ex){error=ex.getClass().getSimpleName();}EntityOracle.i(o,id);WorldEventOracle.text(o,error);error="";try{name=EntityList.getEntityString(e);}catch(RuntimeException ex){error=ex.getClass().getSimpleName();}WorldEventOracle.text(o,error);Trace10.writeBoolean(o, "name!=null", name!=null);if(name!=null)WorldEventOracle.text(o,name);}
  }
 }
}
