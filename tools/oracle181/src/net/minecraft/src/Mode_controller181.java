package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;

/** Original controllers: transitions, placement restoration, reach and physical pick policy. */
public final class Mode_controller181 {
    static void i(DataOutputStream o,int v)throws Exception {EntityOracle.i(o,v);}
    static void f(DataOutputStream o,float v)throws Exception {EntityOracle.f(o,v);}
    static void raw(World w,int x,int y,int z,int id,int meta){EntityOracle.rawBlock(w,x,y,z,id,meta);}
    static void state(World w,PlayerOracle.HeadlessPlayer p,PlayerController c,int result,DataOutputStream o)throws Exception {
        i(o,result);
        if(c instanceof PlayerControllerSP) {
            for(String n:new String[]{"curBlockDamage","prevBlockDamage","field_1069_h"})f(o,EntityOracle.floatField(PlayerControllerSP.class,n,c));
            for(String n:new String[]{"curBlockX","curBlockY","curBlockZ","blockHitWait"})i(o,EntityOracle.intField(PlayerControllerSP.class,n,c));
        } else i(o,EntityOracle.intField(PlayerControllerTest.class,"field_35647_c",c));
        for(int y=64;y<=65;y++)for(int x=3;x<=5;x++)for(int z=3;z<=5;z++){i(o,w.getBlockId(x,y,z));i(o,w.getBlockMetadata(x,y,z));}
        for(ItemStack s:p.inventory.mainInventory)PlayerOracle.stack(o,s);
        NBTTagCompound food=new NBTTagCompound();p.func_35191_at().func_35763_b(food);f(o,food.getFloat("foodExhaustionLevel"));
        java.util.ArrayList<int[]> log=PlayerOracle.stats.remove(p);i(o,log==null?0:log.size());if(log!=null)for(int[] row:log){i(o,row[0]);i(o,row[1]);}
        EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.l(o,EntityOracle.rngState(Item.itemRand));
        WorldEventOracle.snapshot(w,o);
        i(o,w.loadedEntityList.size());for(Object e:w.loadedEntityList)Mode_entities181.state(w,(Entity)e,o);
    }
    public static void main(String[] args)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(int mode=0;mode<2;mode++) {
                World w=Mode_entities181.world();Item.itemRand.setSeed(600);
                PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.setLocationAndAngles(0.5,64,0.5,0,0);p.onGround=true;
                PlayerController c=mode==0?new PlayerControllerSP(p.mc):new PlayerControllerTest(p.mc);p.mc.playerController=c;
                if(mode==1)PlayerControllerTest.func_35646_d(p);
                f(o,c.getBlockReachDistance());for(boolean b:new boolean[]{c.isInTestMode,c.shouldDrawHUD(),c.func_35643_e(),c.func_35642_f(),c.func_35641_g(),c.func_35640_h(),c.func_35636_i()})o.writeBoolean(b);
                for(int t=0;t<1200;t++) {
                    int phase=t%60,result=-1;
                    if(phase==0) {w.loadedEntityList.clear();raw(w,4,64,4,new int[]{1,3,7,18,31,106}[t/60%6],t/60%4);p.inventory.mainInventory[0]=new ItemStack(new int[]{278,359,268,257}[t/60%4],1,0);}
                    if(phase==0||phase==20)c.clickBlock(4,64,4,3);
                    else if(phase==10)c.resetBlockRemoving();
                    else c.sendBlockRemoving(4,64,4,3);
                    if(phase==30) {raw(w,4,64,4,1,0);raw(w,4,64,5,51,0);c.clickBlock(4,64,4,3);}
                    if(phase==40) {raw(w,4,64,4,3,0);raw(w,4,65,4,0,0);p.inventory.mainInventory[0]=new ItemStack(new int[]{5,35,259,290,351}[t/60%5],3,t/60%5==4?0:2);result=c.sendPlaceBlock(p,w,p.inventory.mainInventory[0],4,64,4,1)?1:0;}
                    if(phase==45){p.inventory.mainInventory[0]=new ItemStack(267,1,0);result=c.sendUseItem(p,w,p.inventory.mainInventory[0])?1:0;}
                    if(phase==46)c.func_35638_c(p);
                    if(c instanceof PlayerControllerSP) {Field prev=PlayerControllerSP.class.getDeclaredField("prevBlockDamage");prev.setAccessible(true);prev.setFloat(c,EntityOracle.floatField(PlayerControllerSP.class,"curBlockDamage",c));}
                    ByteArrayOutputStream frame=new ByteArrayOutputStream();state(w,p,c,result,new DataOutputStream(frame));i(o,frame.size());o.write(frame.toByteArray());
                }
            }
            // Real held-button callback, including mining while blocking with a sword.
            Method pick=Minecraft.class.getDeclaredMethod("clickMiddleMouseButton");pick.setAccessible(true);
            Method held=Minecraft.class.getDeclaredMethod("sendClickBlockToController",int.class,boolean.class);held.setAccessible(true);
            for(int mode=0;mode<2;mode++)for(int using=0;using<2;using++) {
                World w=Mode_entities181.world();Item.itemRand.setSeed(601);
                PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.setLocationAndAngles(0.5,64,0.5,0,0);p.onGround=true;
                PlayerController c=mode==0?new PlayerControllerSP(p.mc):new PlayerControllerTest(p.mc);p.mc.playerController=c;p.mc.effectRenderer=new PlayerOracle.HeadlessEffects(w);p.mc.gameSettings=new GameSettings();
                if(mode==1)PlayerControllerTest.func_35646_d(p);
                p.inventory.mainInventory[8]=new ItemStack(1,4,0);p.inventory.mainInventory[0]=new ItemStack(267,1,0);if(using!=0)p.func_35199_b(p.inventory.mainInventory[0],72000);
                for(int t=0;t<200;t++) {
                    if(t%20==0)raw(w,4,64,4,1,0);
                    p.mc.objectMouseOver=t%20<15?new MovingObjectPosition(4,64,4,1,Vec3D.createVector(4,65,4)):null;
                    if(c instanceof PlayerControllerSP){Field prev=PlayerControllerSP.class.getDeclaredField("prevBlockDamage");prev.setAccessible(true);prev.setFloat(c,EntityOracle.floatField(PlayerControllerSP.class,"curBlockDamage",c));}
                    // Source runTick queue branch; handlers and registry are original bytecode.
                    if(t%20==5||t%20==6)KeyBinding.func_35960_a(-98);
                    if(p.func_35196_Z()){if(t>=100)c.func_35638_c(p);}else while(p.mc.gameSettings.field_35383_y.func_35962_c())pick.invoke(p.mc);
                    held.invoke(p.mc,0,t%20<18);
                    ByteArrayOutputStream frame=new ByteArrayOutputStream();DataOutputStream d=new DataOutputStream(frame);state(w,p,c,-1,d);i(d,p.swingProgressInt);d.writeBoolean(p.isSwinging);i(d,p.func_35205_Y());i(d,p.inventory.currentItem);i(d,p.mc.gameSettings.field_35383_y.field_35964_f);i(o,frame.size());o.write(frame.toByteArray());
                }
            }
            // Use the real Minecraft method (entity hits deliberately pick at 0,0,0).
            for(int mode=0;mode<2;mode++)for(int block:new int[]{0,1,2,7,35,43,44,97})for(int slot:new int[]{0,8,9,35})for(int hit=0;hit<3;hit++) {
                World w=Mode_entities181.world();PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.mc.playerController=mode==0?new PlayerControllerSP(p.mc):new PlayerControllerTest(p.mc);
                p.inventory.mainInventory[slot]=new ItemStack(block==2?3:block==7?1:block==43?44:block,1,0);p.inventory.currentItem=5;
                raw(w,4,64,4,block,0);raw(w,0,0,0,block,0);
                p.mc.objectMouseOver=hit==0?null:hit==1?new MovingObjectPosition(4,64,4,1,Vec3D.createVector(4,65,4)):new MovingObjectPosition(p);
                pick.invoke(p.mc);i(o,p.inventory.currentItem);
            }
            // Original getMouseOver tests creative's 32-block entity ray and 5-block block ray.
            for(int mode=0;mode<2;mode++)for(int distance=2;distance<=35;distance++)for(int obstruction=0;obstruction<2;obstruction++) {
                World w=Mode_entities181.world();PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.setLocationAndAngles(0.5,64,0.5,0,0);
                p.mc.renderViewEntity=p;p.mc.playerController=mode==0?new PlayerControllerSP(p.mc):new PlayerControllerTest(p.mc);
                EntityPig pig=new EntityPig(w);pig.setLocationAndAngles(0.5,64,distance+0.5,0,0);w.entityJoinedWorld(pig);
                if(obstruction!=0)raw(w,0,64,4,1,0);
                EntityRenderer renderer=(EntityRenderer)Oracle.allocate(EntityRenderer.class);Field mc=EntityRenderer.class.getDeclaredField("mc");mc.setAccessible(true);mc.set(renderer,p.mc);renderer.getMouseOver(1);
                MovingObjectPosition h=p.mc.objectMouseOver;i(o,h==null?-1:h.typeOfHit==EnumMovingObjectType.ENTITY?1:0);
                if(h!=null) {if(h.typeOfHit==EnumMovingObjectType.ENTITY)i(o,h.entityHit.entityId);else {i(o,h.blockX);i(o,h.blockY);i(o,h.blockZ);i(o,h.sideHit);for(double v:new double[]{h.hitVec.xCoord,h.hitVec.yCoord,h.hitVec.zCoord})EntityOracle.d(o,v);}}
            }
        }
    }
}
