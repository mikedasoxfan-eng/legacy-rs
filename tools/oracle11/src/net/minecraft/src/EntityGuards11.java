package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;
/** Executes original methods; only environmental/superclass work is isolated. */
public final class EntityGuards11 {
 static class S extends EntitySheep {S(World w){super(w);} void leap(){super.jump();} boolean ceased(){return super.isMovementCeased();}}
 static class Q extends EntitySquid {boolean water;Q(World w){super(w);}protected boolean func_44001_ad(){return false;}public boolean isInWater(){return water;}public boolean handleLavaMovement(){return false;}public void moveEntityWithHeading(float a,float b){}}
 static String[] fields={"field_21089_a","field_21088_b","field_21087_c","field_21086_f","field_21085_g","field_21084_h","tentacleAngle","lastTentacleAngle","randomMotionSpeed","field_21080_l","field_21079_m","randomMotionVecX","randomMotionVecY","randomMotionVecZ"};
 static Field f(String n)throws Exception{Field f=EntitySquid.class.getDeclaredField(n);f.setAccessible(true);return f;}
 public static void main(String[] args)throws Exception{
  int boot=Block.stone.blockID;File dir=new File(args[0]);PrintWriter o=new PrintWriter(new File(dir,"sheep_status11.txt"));Field t=SheepSlice11.timer();
  for(int timer:new int[]{-1,0,1,4,36,40})for(int status:new int[]{0,1,10})for(int sprint=0;sprint<2;sprint++){
   SheepSlice11.W w=SheepSlice11.world();S s=new S(w);t.setInt(s,timer);s.handleHealthUpdate((byte)status);s.motionX=.125;s.motionY=.25;s.motionZ=-.375;s.rotationYaw=37.5F;s.setSprinting(sprint!=0);s.rand.setSeed(42);boolean ceased=s.ceased();s.leap();
   o.println(timer+" "+status+" "+sprint+" "+t.getInt(s)+" "+(ceased?1:0)+" "+Double.doubleToRawLongBits(s.motionX)+" "+Double.doubleToRawLongBits(s.motionY)+" "+Double.doubleToRawLongBits(s.motionZ)+" "+(s.isAirBorne?1:0)+" "+s.rand.nextLong());
  }o.close();o=new PrintWriter(new File(dir,"living_guard11.txt"));Method base=EntityLiving.class.getDeclaredMethod("func_44001_ad");base.setAccessible(true);Method local=EntityPlayerSP.class.getDeclaredMethod("func_44001_ad");local.setAccessible(true);
  for(int remote=0;remote<2;remote++)for(int legacy=0;legacy<2;legacy++){SheepSlice11.W w=SheepSlice11.world();w.multiplayerWorld=remote!=0;EntityCow e=new EntityCow(w);EntityPlayerSP p=(EntityPlayerSP)SheepSlice11.unsafe().allocateInstance(EntityPlayerSP.class);p.worldObj=w;o.println(remote+" "+legacy+" "+((Boolean)base.invoke(e)?1:0)+" "+((Boolean)local.invoke(p)?1:0));}o.close();
  o=new PrintWriter(new File(dir,"squid_guard11.txt"));
  for(int remote=0;remote<2;remote++)for(int legacy=0;legacy<2;legacy++)for(int water=0;water<2;water++)for(float rotation:new float[]{0F,2.5F,3.5F,6.25F,7F})for(int seed=0;seed<10;seed++){
   SheepSlice11.W w=SheepSlice11.world();w.multiplayerWorld=remote!=0;Q q=new Q(w);q.water=water!=0;for(int i=0;i<fields.length;i++)f(fields[i]).setFloat(q,(i+1)*.1F);q.field_21085_g=rotation;q.motionX=.125;q.motionY=.25;q.motionZ=-.375;q.renderYawOffset=20F;q.rotationYaw=30F;q.rand.setSeed(seed);q.onLivingUpdate();
   o.print(remote+" "+legacy+" "+water+" "+Float.floatToRawIntBits(rotation)+" "+seed);for(String n:fields)o.print(" "+Float.floatToRawIntBits(f(n).getFloat(q)));o.println(" "+Double.doubleToRawLongBits(q.motionX)+" "+Double.doubleToRawLongBits(q.motionY)+" "+Double.doubleToRawLongBits(q.motionZ)+" "+Float.floatToRawIntBits(q.renderYawOffset)+" "+Float.floatToRawIntBits(q.rotationYaw)+" "+q.rand.nextLong());
  }o.close();
 }
}
