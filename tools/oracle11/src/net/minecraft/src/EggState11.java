package net.minecraft.src;
import java.io.*;
/** Original constructor, tick and NBT methods; only Entity's Random seed is injected. */
public final class EggState11 {
 static final int[] IDS={96,50,51,52,54,55,56,120,57,58,90,59,91,60,92,61,93,62,94,95};
 static void state(World w,Entity e,DataOutputStream o)throws Exception{
  Mode_entities10.state(w,e,o);
  if(e instanceof EntitySlime){EntitySlime s=(EntitySlime)e;EntityOracle.i(o,s.getSlimeSize());EntityOracle.i(o,EntityOracle.intField(EntitySlime.class,"slimeJumpDelay",e));for(float f:new float[]{s.field_40139_a,s.field_768_a,s.field_767_b})EntityOracle.f(o,f);}
  if(e instanceof EntityChicken){EntityChicken c=(EntityChicken)e;EntityOracle.i(o,c.timeUntilNextEgg);for(float f:new float[]{c.field_752_b,c.field_756_e,c.destPos,c.field_757_d,c.field_755_h})EntityOracle.f(o,f);}
  if(e instanceof EntitySquid){for(String key:new String[]{"field_21089_a","field_21088_b","field_21087_c","field_21086_f","field_21085_g","field_21084_h","tentacleAngle","lastTentacleAngle","randomMotionSpeed","field_21080_l","field_21079_m","randomMotionVecX","randomMotionVecY","randomMotionVecZ"})EntityOracle.f(o,EntityOracle.floatField(EntitySquid.class,key,e));}
 }
 public static void main(String[] args)throws Exception {
  File dir=new File(args[0]);dir.mkdirs();
  DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(new File(dir,"eggstate11.bin"))));
  for(int joined=0;joined<2;joined++)for(long seed:new long[]{0,1,42,100,-1,341873128712L})for(int id:IDS)for(int ticks:new int[]{0,1,5,20}){
   World w=Mode_entities10.world();OracleHooks.reset(seed);Entity e=EntityList.createEntity(id,w);e.setLocationAndAngles(.5,65,.5,0,0);
   if(joined!=0)w.spawnEntityInWorld(e);for(int t=0;t<ticks;t++){if(joined==0)e.onUpdate();else w.updateEntityWithOptionalForce(e,true);}
   ByteArrayOutputStream bytes=new ByteArrayOutputStream();DataOutputStream state=new DataOutputStream(bytes);state(w,e,state);state.close();EntityOracle.i(out,bytes.size());out.write(bytes.toByteArray());
   NBTTagCompound n=new NBTTagCompound();if(!e.addEntityID(n)){EntityOracle.i(out,0);EntityOracle.i(out,0);continue;}bytes=new ByteArrayOutputStream();state=new DataOutputStream(bytes);NBTBase.writeTag(n,state);state.close();EntityOracle.i(out,bytes.size());out.write(bytes.toByteArray());
   Entity loaded=EntityList.createEntityFromNBT(n,w);bytes=new ByteArrayOutputStream();state=new DataOutputStream(bytes);state(w,loaded,state);state.close();EntityOracle.i(out,bytes.size());out.write(bytes.toByteArray());
   for(int t=0;t<5;t++)loaded.onUpdate();bytes=new ByteArrayOutputStream();state=new DataOutputStream(bytes);state(w,loaded,state);state.close();EntityOracle.i(out,bytes.size());out.write(bytes.toByteArray());
  }
  out.close();
 }
}
