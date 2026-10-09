package net.minecraft.src;
import java.io.*;import java.util.*;
/** Actual original1.1 dragon part callback, including downstream living damage. */
public final class DragonOwner11 {
 static class W extends World {W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}public void setEntityState(Entity e,byte b){}public void playSoundAtEntity(Entity e,String s,float v,float p){}}
 public static void main(String[]args)throws Exception {int boot=Block.stone.blockID;PrintWriter out=new PrintWriter(new File(args[0],"dragonowner11.txt"));
  for(int source=0;source<9;source++)for(int head=0;head<2;head++)for(int remote=0;remote<2;remote++)for(int seed:new int[]{0,42})for(float yaw:new float[]{0,90,-179.5F}){
   W w=(W)SheepSlice11.unsafe().allocateInstance(W.class);w.rand=new Random(seed);w.multiplayerWorld=remote!=0;
   EntityDragon d=new EntityDragon(w);d.setPosition(2,80,-3);d.rotationYaw=yaw;d.rand.setSeed(seed);
   EntityPlayerSP p=(EntityPlayerSP)SheepSlice11.unsafe().allocateInstance(EntityPlayerSP.class);p.worldObj=w;p.posX=20;p.posZ=10;
   EntityZombie z=new EntityZombie(w);z.setPosition(20,80,10);EntityArrow arrow=new EntityArrow(w);EntitySnowball snow=new EntitySnowball(w);EntityFireball fire=new EntityFireball(w);
   DamageSource ds=source==0?DamageSource.causeArrowDamage(arrow,p):source==1?DamageSource.causeArrowDamage(arrow,z):source==2?DamageSource.causeArrowDamage(arrow,null):source==3?DamageSource.causeThrownDamage(snow,p):source==4?DamageSource.causeIndirectMagicDamage(snow,p):source==5?DamageSource.causeFireballDamage(fire,p):source==6?DamageSource.causePlayerDamage(p):source==7?DamageSource.causeMobDamage(z):DamageSource.explosion;
   boolean result=d.attackEntityFromPart(head!=0?d.dragonPartHead:d.dragonPartBody,ds,13);
   out.println(source+" "+head+" "+remote+" "+seed+" "+Float.floatToRawIntBits(yaw)+" "+(result?1:0)+" "+d.health+" "+d.hurtTime+" "+Double.doubleToRawLongBits(d.targetX)+" "+Double.doubleToRawLongBits(d.targetY)+" "+Double.doubleToRawLongBits(d.targetZ)+" "+d.rand.nextLong()+" "+w.rand.nextLong());
  }out.close();
 }
}
