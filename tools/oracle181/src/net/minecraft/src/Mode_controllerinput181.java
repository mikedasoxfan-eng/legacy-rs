package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;

/** Original Minecraft click/held/queued input callbacks, including repeated use and cooldowns. */
public final class Mode_controllerinput181 {
    static Field field(String name)throws Exception {Field f=Minecraft.class.getDeclaredField(name);f.setAccessible(true);return f;}
    public static void main(String[] args)throws Exception {
        Method click=Minecraft.class.getDeclaredMethod("clickMouse",int.class);click.setAccessible(true);
        Method held=Minecraft.class.getDeclaredMethod("sendClickBlockToController",int.class,boolean.class);held.setAccessible(true);
        Method pick=Minecraft.class.getDeclaredMethod("clickMiddleMouseButton");pick.setAccessible(true);
        Field left=field("leftClickCounter"),right=field("field_35001_ab");
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(int mode=0;mode<2;mode++) {
                World w=Mode_entities181.world();Item.itemRand.setSeed(605);
                PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.onGround=true;p.setLocationAndAngles(0.5,64,0.5,0,0);
                Minecraft mc=p.mc;mc.gameSettings=new GameSettings();KeyBinding.func_35959_a();
                PlayerController controller=mode==0?new PlayerControllerSP(mc):new PlayerControllerTest(mc);mc.playerController=controller;
                if(mode==1)PlayerControllerTest.func_35646_d(p);
                mc.effectRenderer=new PlayerOracle.HeadlessEffects(w);mc.entityRenderer=(EntityRenderer)Oracle.allocate(EntityRenderer.class);mc.entityRenderer.itemRenderer=(ItemRenderer)Oracle.allocate(ItemRenderer.class);
                for(int t=0;t<1400;t++) {
                    int phase=t%70;
                    if(phase==0) {
                        p.func_35207_ac();p.inventory.currentItem=0;
                        p.inventory.mainInventory[0]=new ItemStack(new int[]{278,359,267,5,260,261,282}[t/70%7],new int[]{1,3,64,0,-1}[t/70%5],0);
                        p.inventory.mainInventory[8]=new ItemStack(262,0,0);p.func_35191_at().func_35764_a(10);
                        EntityOracle.rawBlock(w,4,64,4,new int[]{1,3,18,31,106}[t/70%5],t/70%4);
                        EntityOracle.rawBlock(w,4,65,4,0,0);
                    }
                    mc.objectMouseOver=phase%11<8?new MovingObjectPosition(4,64,4,1,Vec3D.createVector(4,65,4)):null;
                    if(controller instanceof PlayerControllerSP) {Field f=PlayerControllerSP.class.getDeclaredField("prevBlockDamage");f.setAccessible(true);f.setFloat(controller,EntityOracle.floatField(PlayerControllerSP.class,"curBlockDamage",controller));}
                    if(right.getInt(mc)>0)right.setInt(mc,right.getInt(mc)-1);
                    if(left.getInt(mc)>0)left.setInt(mc,left.getInt(mc)-1);
                    boolean attack=phase<17||phase>=55,use=phase>=20&&phase<48;
                    KeyBinding.func_35963_a(-100,attack);KeyBinding.func_35963_a(-99,use);
                    for(int n=0;n<(phase==0?2:phase==55?1:0);n++)KeyBinding.func_35960_a(-100);
                    for(int n=0;n<(phase==20?2:phase==30?1:0);n++)KeyBinding.func_35960_a(-99);
                    if(phase==25||phase==50)KeyBinding.func_35960_a(-98);
                    if(p.func_35196_Z()) {if(!use)controller.func_35638_c(p);} else {
                        while(mc.gameSettings.field_35382_v.func_35962_c())click.invoke(mc,0);
                        while(mc.gameSettings.field_35381_w.func_35962_c())click.invoke(mc,1);
                        while(mc.gameSettings.field_35383_y.func_35962_c())pick.invoke(mc);
                    }
                    if(use&&right.getInt(mc)==0&&!p.func_35196_Z())click.invoke(mc,1);
                    held.invoke(mc,0,attack);
                    ByteArrayOutputStream bytes=new ByteArrayOutputStream();DataOutputStream d=new DataOutputStream(bytes);
                    Mode_controller181.state(w,p,controller,-1,d);
                    for(int n:new int[]{left.getInt(mc),right.getInt(mc),p.swingProgressInt,p.func_35205_Y(),p.inventory.currentItem,mc.gameSettings.field_35382_v.field_35964_f,mc.gameSettings.field_35381_w.field_35964_f,mc.gameSettings.field_35383_y.field_35964_f})EntityOracle.i(d,n);
                    d.writeBoolean(p.isSwinging);PlayerOracle.stack(d,p.func_35195_X());
                    EntityOracle.i(o,bytes.size());o.write(bytes.toByteArray());
                }
            }
        }
    }
}
