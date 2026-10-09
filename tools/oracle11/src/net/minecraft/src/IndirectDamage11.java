package net.minecraft.src;
import java.io.*;import java.util.*;
public final class IndirectDamage11 {
 static class W extends World {W(){super((ISaveHandler)null,"",new WorldSettings(0L,0,false,false,EnumWorldType.DEFAULT));}public void setEntityState(Entity e,byte b){}public boolean spawnEntityInWorld(Entity e){return true;}}
 static class P extends EntityPlayerSP {int snipes;P(){super(null,null,new Session("probe",""),0);}public void addStat(StatBase s,int n){if(s==AchievementList.snipeSkeleton)snipes+=n;}}
 public static void main(String[] args)throws Exception{int boot=Block.stone.blockID;PrintWriter o=new PrintWriter(new File(args[0],"indirectdamage11.txt"));
  for(int source=0;source<5;source++)for(double x:new double[]{0,49.999999,50,50.000001,-50,30})for(double z:new double[]{0,40})for(int remote=0;remote<2;remote++){
   W w=(W)SheepSlice11.unsafe().allocateInstance(W.class);w.rand=new Random(42);w.multiplayerWorld=remote!=0;P p=(P)SheepSlice11.unsafe().allocateInstance(P.class);p.worldObj=w;p.inventory=new InventoryPlayer(p);p.posX=x;p.posZ=z;p.posY=1000;
   EntitySkeleton s=new EntitySkeleton(w);s.setPosition(0,64,0);s.rand.setSeed(42);EntityArrow a=new EntityArrow(w);EntitySnowball snow=new EntitySnowball(w);DamageSource ds=source==0?DamageSource.causeArrowDamage(a,p):source==1?DamageSource.causeArrowDamage(a,a):source==2?DamageSource.causePlayerDamage(p):source==3?DamageSource.causeThrownDamage(snow,p):DamageSource.causeIndirectMagicDamage(snow,p);
   s.onDeath(ds);o.println(source+" "+Double.doubleToRawLongBits(x)+" "+Double.doubleToRawLongBits(z)+" "+remote+" "+(ds.getSourceOfDamage()==a?1:ds.getSourceOfDamage()==snow?2:ds.getSourceOfDamage()==p?3:0)+" "+(ds.getEntity()==p?1:0)+" "+p.snipes+" "+s.rand.nextLong()+" "+w.rand.nextLong());
  }o.close();
 }
}
