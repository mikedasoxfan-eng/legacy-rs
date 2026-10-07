package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;

/** Original controllers: transitions, placement restoration, reach and physical pick policy. */
public final class Mode_controller10 {
    static void i(DataOutputStream o,int v)throws Exception {EntityOracle.i(o,v);}
    static void f(DataOutputStream o,float v)throws Exception {EntityOracle.f(o,v);}
    static void raw(World w,int x,int y,int z,int id,int meta){EntityOracle.rawBlock(w,x,y,z,id,meta);}
    static void state(World w,PlayerOracle.HeadlessPlayer p,PlayerController c,int result,DataOutputStream o)throws Exception {
        i(o,result);
        if(c instanceof PlayerControllerSP) {
            for(String n:new String[]{"curBlockDamage","prevBlockDamage","field_1069_h"})f(o,Trace10.namedFloat("EntityOracle.floatField(PlayerControllerSP.class,n,c)", EntityOracle.floatField(PlayerControllerSP.class,n,c)));
            for(String n:new String[]{"curBlockX","curBlockY","curBlockZ","blockHitWait"})i(o,Trace10.namedInt("EntityOracle.intField(PlayerControllerSP.class,n,c)", EntityOracle.intField(PlayerControllerSP.class,n,c)));
        } else i(o,Trace10.namedInt("EntityOracle.intField(PlayerControllerCreative.class,\"field_35647_c\",c)", EntityOracle.intField(PlayerControllerCreative.class,"field_35647_c",c)));
        for(int y=64;y<=65;y++)for(int x=3;x<=5;x++)for(int z=3;z<=5;z++){i(o,Trace10.namedInt("w.getBlockId(x,y,z)", w.getBlockId(x,y,z)));i(o,Trace10.namedInt("w.getBlockMetadata(x,y,z)", w.getBlockMetadata(x,y,z)));}
        for(ItemStack s:p.inventory.mainInventory)PlayerOracle.stack(o,s);
        NBTTagCompound food=new NBTTagCompound();p.getFoodStats().writeStatsToNBT(food);f(o,Trace10.namedFloat("food.getFloat(\"foodExhaustionLevel\")", food.getFloat("foodExhaustionLevel")));
        java.util.ArrayList<int[]> log=PlayerOracle.stats.remove(p);i(o,Trace10.namedInt("log==null?0:log.size()", log==null?0:log.size()));if(log!=null)for(int[] row:log){i(o,Trace10.namedInt("row[0]", row[0]));i(o,Trace10.namedInt("row[1]", row[1]));}
        EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(Item.itemRand)", EntityOracle.rngState(Item.itemRand)));
        WorldEventOracle.snapshot(w,o);
        i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));for(Object e:w.loadedEntityList)Mode_entities10.state(w,(Entity)e,o);
    }
    public static void main(String[] args)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(int mode=0;mode<2;mode++) {
                World w=Mode_entities10.world();Item.itemRand.setSeed(600);
                PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.setLocationAndAngles(0.5,64,0.5,0,0);p.onGround=true;
                PlayerController c=mode==0?new PlayerControllerSP(p.mc):new PlayerControllerCreative(p.mc);p.mc.playerController=c;
                if(mode==1)PlayerControllerCreative.func_35646_d(p);
                f(o,Trace10.namedFloat("c.getBlockReachDistance()", c.getBlockReachDistance()));for(boolean b:new boolean[]{c.isInTestMode,c.shouldDrawHUD(),c.func_35643_e(),c.func_35642_f(),c.func_35641_g(),c.isInCreativeMode(),c.extendedReach()})Trace10.writeBoolean(o, "b", b);
                for(int t=0;t<1200;t++) {
                    int phase=t%60,result=-1;
                    if(phase==0) {w.loadedEntityList.clear();raw(w,4,64,4,new int[]{1,3,7,18,31,106}[t/60%6],t/60%4);p.inventory.mainInventory[0]=new ItemStack(new int[]{278,359,268,257}[t/60%4],1,0);}
                    if(phase==0||phase==20)c.clickBlock(4,64,4,3);
                    else if(phase==10)c.resetBlockRemoving();
                    else c.sendBlockRemoving(4,64,4,3);
                    if(phase==30) {raw(w,4,64,4,1,0);raw(w,4,64,5,51,0);c.clickBlock(4,64,4,3);}
                    if(phase==40) {raw(w,4,64,4,3,0);raw(w,4,65,4,0,0);p.inventory.mainInventory[0]=new ItemStack(new int[]{5,35,259,290,351}[t/60%5],3,t/60%5==4?0:2);result=c.sendPlaceBlock(p,w,p.inventory.mainInventory[0],4,64,4,1)?1:0;}
                    if(phase==45){p.inventory.mainInventory[0]=new ItemStack(267,1,0);result=c.sendUseItem(p,w,p.inventory.mainInventory[0])?1:0;}
                    if(phase==46)c.onStoppedUsingItem(p);
                    if(c instanceof PlayerControllerSP) {Field prev=PlayerControllerSP.class.getDeclaredField("prevBlockDamage");prev.setAccessible(true);prev.setFloat(c,EntityOracle.floatField(PlayerControllerSP.class,"curBlockDamage",c));}
                    ByteArrayOutputStream frame=new ByteArrayOutputStream();state(w,p,c,result,new DataOutputStream(frame));i(o,Trace10.namedInt("frame.size()", frame.size()));Trace10.writeBytes(o, "frame.toByteArray()", frame.toByteArray());
                }
            }
            // Real held-button callback, including mining while blocking with a sword.
            Method pick=Minecraft.class.getDeclaredMethod("clickMiddleMouseButton");pick.setAccessible(true);
            Method held=Minecraft.class.getDeclaredMethod("sendClickBlockToController",int.class,boolean.class);held.setAccessible(true);
            for(int mode=0;mode<2;mode++)for(int using=0;using<2;using++) {
                World w=Mode_entities10.world();Item.itemRand.setSeed(601);
                PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.setLocationAndAngles(0.5,64,0.5,0,0);p.onGround=true;
                PlayerController c=mode==0?new PlayerControllerSP(p.mc):new PlayerControllerCreative(p.mc);p.mc.playerController=c;p.mc.effectRenderer=new PlayerOracle.HeadlessEffects(w);p.mc.gameSettings=new GameSettings();
                if(mode==1)PlayerControllerCreative.func_35646_d(p);
                p.inventory.mainInventory[8]=new ItemStack(1,4,0);p.inventory.mainInventory[0]=new ItemStack(267,1,0);if(using!=0)p.setItemInUse(p.inventory.mainInventory[0],72000);
                for(int t=0;t<200;t++) {
                    if(t%20==0)raw(w,4,64,4,1,0);
                    p.mc.objectMouseOver=t%20<15?new MovingObjectPosition(4,64,4,1,Vec3D.createVector(4,65,4)):null;
                    if(c instanceof PlayerControllerSP){Field prev=PlayerControllerSP.class.getDeclaredField("prevBlockDamage");prev.setAccessible(true);prev.setFloat(c,EntityOracle.floatField(PlayerControllerSP.class,"curBlockDamage",c));}
                    // Source runTick queue branch; handlers and registry are original bytecode.
                    if(t%20==5||t%20==6)KeyBinding.onTick(-98);
                    // Minecraft.runTick: while an item is in use, queued pick presses are drained without picking.
                    if(p.isUsingItem()){if(t>=100)c.onStoppedUsingItem(p);while(p.mc.gameSettings.keyBindPickBlock.isPressed()){}}else while(p.mc.gameSettings.keyBindPickBlock.isPressed()){try{pick.invoke(p.mc);}catch(java.lang.reflect.InvocationTargetException ex){// The original 1.0 throws here when the picked block was mined to air earlier in the same tick
                        // (Item.itemsList[0] is null): the game would crash. The port treats it as a no-op pick; so does
                        // this check (nothing changed before the throw).
                        if(!(ex.getCause() instanceof NullPointerException)||w.getBlockId(4,64,4)!=0)throw ex;}}
                    held.invoke(p.mc,0,t%20<18);
                    ByteArrayOutputStream frame=new ByteArrayOutputStream();DataOutputStream d=new DataOutputStream(frame);state(w,p,c,-1,d);i(d,Trace10.namedInt("p.swingProgressInt", p.swingProgressInt));Trace10.writeBoolean(d, "p.isSwinging", p.isSwinging);i(d,Trace10.namedInt("p.func_35205_Y()", p.func_35205_Y()));i(d,Trace10.namedInt("p.inventory.currentItem", p.inventory.currentItem));i(d,Trace10.namedInt("p.mc.gameSettings.keyBindPickBlock.pressTime", p.mc.gameSettings.keyBindPickBlock.pressTime));i(o,Trace10.namedInt("frame.size()", frame.size()));Trace10.writeBytes(o, "frame.toByteArray()", frame.toByteArray());
                }
            }
            // Use the real Minecraft method (entity hits deliberately pick at 0,0,0).
            for(int mode=0;mode<2;mode++)for(int block:new int[]{0,1,2,7,35,43,44,97})for(int slot:new int[]{0,8,9,35})for(int hit=0;hit<3;hit++) {
                World w=Mode_entities10.world();PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.mc.playerController=mode==0?new PlayerControllerSP(p.mc):new PlayerControllerCreative(p.mc);
                p.inventory.mainInventory[slot]=new ItemStack(block==2?3:block==7?1:block==43?44:block,1,0);p.inventory.currentItem=5;
                raw(w,4,64,4,block,0);raw(w,0,0,0,block,0);
                p.mc.objectMouseOver=hit==0?null:hit==1?new MovingObjectPosition(4,64,4,1,Vec3D.createVector(4,65,4)):new MovingObjectPosition(p);
                // 1.0 clickMiddleMouseButton throws (Item.itemsList[0] is null) when the picked block is air: the
                // original would crash. The port treats it as a no-op pick; the slot stays as it was.
                try{pick.invoke(p.mc);}catch(java.lang.reflect.InvocationTargetException ex){if(!(ex.getCause() instanceof NullPointerException)||block!=0)throw ex;}
                i(o,Trace10.namedInt("p.inventory.currentItem", p.inventory.currentItem));
            }
            // Original getMouseOver tests creative's 32-block entity ray and 5-block block ray.
            for(int mode=0;mode<2;mode++)for(int distance=2;distance<=35;distance++)for(int obstruction=0;obstruction<2;obstruction++) {
                World w=Mode_entities10.world();PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.setLocationAndAngles(0.5,64,0.5,0,0);
                p.mc.renderViewEntity=p;p.mc.playerController=mode==0?new PlayerControllerSP(p.mc):new PlayerControllerCreative(p.mc);
                EntityPig pig=new EntityPig(w);pig.setLocationAndAngles(0.5,64,distance+0.5,0,0);w.entityJoinedWorld(pig);
                if(obstruction!=0)raw(w,0,64,4,1,0);
                EntityRenderer renderer=(EntityRenderer)Oracle.allocate(EntityRenderer.class);Field mc=EntityRenderer.class.getDeclaredField("mc");mc.setAccessible(true);mc.set(renderer,p.mc);renderer.getMouseOver(1);
                MovingObjectPosition h=p.mc.objectMouseOver;i(o,Trace10.namedInt("h==null?-1:h.typeOfHit==EnumMovingObjectType.ENTITY?1:0", h==null?-1:h.typeOfHit==EnumMovingObjectType.ENTITY?1:0));
                if(h!=null) {if(h.typeOfHit==EnumMovingObjectType.ENTITY)i(o,Trace10.namedInt("h.entityHit.entityId", h.entityHit.entityId));else {i(o,Trace10.namedInt("h.blockX", h.blockX));i(o,Trace10.namedInt("h.blockY", h.blockY));i(o,Trace10.namedInt("h.blockZ", h.blockZ));i(o,Trace10.namedInt("h.sideHit", h.sideHit));for(double v:new double[]{Trace10.namedDouble("h.hitVec.xCoord", h.hitVec.xCoord),Trace10.namedDouble("h.hitVec.yCoord", h.hitVec.yCoord),Trace10.namedDouble("h.hitVec.zCoord", h.hitVec.zCoord)})EntityOracle.d(o,v);}}
            }
        }
    }
}
