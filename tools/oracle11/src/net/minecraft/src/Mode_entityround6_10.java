package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** Round 6 singleplayer branch proofs; original classes are never replaced. */
public final class Mode_entityround6_10 {
 static void i(DataOutputStream o,int n)throws Exception {EntityOracle.i(o,n);}
 static void effects(Entity e,DataOutputStream o)throws Exception {
  NBTTagCompound n=new NBTTagCompound();e.writeToNBT(n);NBTTagList list=n.getTagList("ActiveEffects");i(o,Trace10.namedInt("list.tagCount()", list.tagCount()));
  for(int j=0;j<list.tagCount();j++){NBTTagCompound t=(NBTTagCompound)list.tagAt(j);i(o,Trace10.namedInt("t.getByte(\"Id\")", t.getByte("Id")));i(o,Trace10.namedInt("t.getByte(\"Amplifier\")", t.getByte("Amplifier")));i(o,Trace10.namedInt("t.getInteger(\"Duration\")", t.getInteger("Duration")));}
 }
 static void raw(World w,int x,int y,int z,int b){w.getChunkFromChunkCoords(x>>4,z>>4).blocks[((x&15)<<11)|((z&15)<<7)|y]=(byte)b;}
 static void living(DataOutputStream o)throws Exception {
  Field hit=EntityLiving.class.getDeclaredField("recentlyHit");hit.setAccessible(true);
  for(String name:new String[]{"Cow","Zombie","Skeleton","Squid"})for(int scene=0;scene<3;scene++)for(int air:new int[]{-20,-19,0,300})for(boolean breathing:new boolean[]{false,true})for(int health:new int[]{0,7}) {
   World w=Mode_entities10.world();EntityLiving e=(EntityLiving)EntityList.createEntityInWorld(name,w);e.setPosition(-3.5,65,4.5);e.setAir(air);e.health=health;Harness10.setInt(e,"fire",71);e.deathTime=18;hit.setInt(e,3);
   for(int x=-5;x<=-2;x++)for(int y=64;y<=69;y++)for(int z=3;z<=6;z++)if(scene>0)raw(w,x,y,z,scene==1?9:1);
   if(breathing)e.addPotionEffect(new PotionEffect(13,2,0));
   for(int tick=0;tick<4;tick++){e.onEntityUpdate();Mode_entities10.state(w,e,o);i(o,Trace10.namedInt("hit.getInt(e)", hit.getInt(e)));i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));for(Object a:new ArrayList(w.loadedEntityList))Mode_entities10.state(w,(Entity)a,o);}
  }
 }
 static void nbt(DataOutputStream o)throws Exception {
  for(int[] ids:new int[][]{{19,1,17,2,10,13,6,7},{17,1,19,2},{1,17,2,19},{13,10,7,6,2,1,19,17}}) {
   World w=Mode_entities10.world();EntityLiving e=new EntityCow(w);
   for(int id:ids){e.addPotionEffect(new PotionEffect(id,100+id,id%3));effects(e,o);}
   e.addPotionEffect(new PotionEffect(1,3,5));effects(e,o);e.damageEntity(DamageSource.generic, 17);effects(e,o);
   NBTTagCompound n=new NBTTagCompound();e.writeToNBT(n);n.setString("id","Cow");NBTTagList list=new NBTTagList();
   for(int[] v:new int[][]{{19,3,99},{1,5,17},{19,-1,0},{17,2,25},{1,0,5}}){NBTTagCompound t=new NBTTagCompound();t.setByte("Id",(byte)v[0]);t.setByte("Amplifier",(byte)v[1]);t.setInteger("Duration",v[2]);list.setTag(t);}
   n.setTag("ActiveEffects",list);e.readFromNBT(n);effects(e,o);effects(EntityList.createEntityFromNBT(n,w),o);
  }
 }
 static void sleep(DataOutputStream o)throws Exception {
  Field sleeping=EntityPlayer.class.getDeclaredField("sleeping");sleeping.setAccessible(true);
  for(int scene=0;scene<7;scene++)for(int seed=0;seed<64;seed++){
   World w=Mode_entities10.world();for(Object c0:((EntityOracle.FlatProvider)w.chunkProvider).chunks.values())Arrays.fill(((Chunk)c0).skylightMap.data,(byte)(scene==3?255:0));
   if(scene==1)for(int x=-32;x<=31;x++)for(int z=-32;z<=31;z++){raw(w,x,64,z,1);raw(w,x,65,z,1);}
   if(scene==2)for(int x=-32;x<=31;x++)for(int z=-32;z<=31;z++)raw(w,x,63,z,0);
   Mode_entities10.Player p=new Mode_entities10.Player(w);p.username="Player";p.setPosition(.5,64,.5);sleeping.setBoolean(p,true);Field bed=EntityPlayer.class.getDeclaredField("bedChunkCoordinates");bed.setAccessible(true);bed.set(p,new ChunkCoordinates(0,64,0));if(scene==4||scene==5){raw(w,0,64,0,26);raw(w,0,64,1,26);w.getChunkFromChunkCoords(0,0).setBlockMetadata(0,64,0,scene==4?4:6);}w.spawnEntityInWorld(p);
   if(scene==6){Mode_entities10.Player q=new Mode_entities10.Player(w);q.username="Player";q.setPosition(8.5,64,8.5);sleeping.setBoolean(q,true);bed.set(q,new ChunkCoordinates(8,64,8));w.spawnEntityInWorld(q);}w.rand.setSeed(seed);
   Trace10.writeBoolean(o, "Harness10.removedSleepSpawning(w,w.playerEntities)", Harness10.removedSleepSpawning(w,w.playerEntities));for(Object e:new ArrayList(w.loadedEntityList))Mode_entities10.state(w,(Entity)e,o);i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));
  }
 }
 static void damage(DataOutputStream o)throws Exception {
  Field hit=EntityLiving.class.getDeclaredField("recentlyHit");hit.setAccessible(true);Field owner=EntityLiving.class.getDeclaredField("attackingPlayer");owner.setAccessible(true);
  for(int source=0;source<5;source++)for(int immunity:new int[]{0,10,11,20})for(int damage:new int[]{Integer.MIN_VALUE,-1,0,1,7,Integer.MAX_VALUE}){
   World w=Mode_entities10.world();EntityLiving attacker=source==1?new Mode_entities10.Player(w):source==2||source==3?new EntityWolf(w):new EntityZombie(w);attacker.setPosition(-3.5,65,4.5);if(source==3)((EntityWolf)attacker).setIsTamed(true);
   w.loadedEntityList.add(attacker);if(source==1)w.playerEntities.add(attacker);EntityCow e=new EntityCow(w);e.setPosition(-3.5,65,4.5);e.health=7;e.heartsLife=immunity;e.naturalArmorRating=-1;
   DamageSource s=source==0?DamageSource.generic:new EntityDamageSource("mob",attacker);
   for(int j=0;j<2;j++){Trace10.writeBoolean(o, "e.attackEntityFrom(s,damage)", e.attackEntityFrom(s,damage));Mode_entities10.state(w,e,o);i(o,Trace10.namedInt("hit.getInt(e)", hit.getInt(e)));Entity p=(Entity)owner.get(e);i(o,Trace10.namedInt("p==null?-1:p.entityId", p==null?-1:p.entityId));Trace10.writeBoolean(o, "e.unused_flag", e.unused_flag);}
  }
 }
 public static void main(String[] a)throws Exception {try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))){switch(a[1]){case "livingdamage6":damage(o);break;case "living6":living(o);break;case "livingnbt6":nbt(o);break;case "sleep6":sleep(o);break;default:throw new IllegalArgumentException(a[1]);}}}
}
