package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import net.minecraft.client.Minecraft;

/** Original Minecraft click/held/queued input callbacks, including repeated use and cooldowns. */
public final class Mode_controllerinput10 {
    static Field field(String name)throws Exception {Field f=Minecraft.class.getDeclaredField(name);f.setAccessible(true);return f;}
    public static void main(String[] args)throws Exception {
        Method click=Minecraft.class.getDeclaredMethod("clickMouse",int.class);click.setAccessible(true);
        Method held=Minecraft.class.getDeclaredMethod("sendClickBlockToController",int.class,boolean.class);held.setAccessible(true);
        Method pick=Minecraft.class.getDeclaredMethod("clickMiddleMouseButton");pick.setAccessible(true);
        Field left=field("leftClickCounter"),right=field("rightClickDelayTimer");
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(int mode=0;mode<2;mode++) {
                World w=Mode_entities10.world();Item.itemRand.setSeed(605);
                PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.onGround=true;p.setLocationAndAngles(0.5,64,0.5,0,0);
                Minecraft mc=p.mc;mc.gameSettings=new GameSettings();KeyBinding.unPressAllKeys();
                PlayerController controller=mode==0?new PlayerControllerSP(mc):new PlayerControllerCreative(mc);mc.playerController=controller;
                if(mode==1)PlayerControllerCreative.func_35646_d(p);
                mc.effectRenderer=new PlayerOracle.HeadlessEffects(w);mc.entityRenderer=(EntityRenderer)Oracle.allocate(EntityRenderer.class);mc.entityRenderer.itemRenderer=(ItemRenderer)Oracle.allocate(ItemRenderer.class);
                for(int t=0;t<1400;t++) {
                    int phase=t%70;
                    if(phase==0) {
                        p.clearItemInUse();p.inventory.currentItem=0;
                        p.inventory.mainInventory[0]=new ItemStack(new int[]{278,359,267,5,260,261,282}[t/70%7],new int[]{1,3,64,0,-1}[t/70%5],0);
                        p.inventory.mainInventory[8]=new ItemStack(262,0,0);p.getFoodStats().setFoodLevel(10);
                        EntityOracle.rawBlock(w,4,64,4,new int[]{1,3,18,31,106}[t/70%5],t/70%4);
                        EntityOracle.rawBlock(w,4,65,4,0,0);
                    }
                    mc.objectMouseOver=phase%11<8?new MovingObjectPosition(4,64,4,1,Vec3D.createVector(4,65,4)):null;
                    if(controller instanceof PlayerControllerSP) {Field f=PlayerControllerSP.class.getDeclaredField("prevBlockDamage");f.setAccessible(true);f.setFloat(controller,EntityOracle.floatField(PlayerControllerSP.class,"curBlockDamage",controller));}
                    if(right.getInt(mc)>0)right.setInt(mc,right.getInt(mc)-1);
                    if(left.getInt(mc)>0)left.setInt(mc,left.getInt(mc)-1);
                    boolean attack=phase<17||phase>=55,use=phase>=20&&phase<48;
                    KeyBinding.setKeyBindState(-100,attack);KeyBinding.setKeyBindState(-99,use);
                    for(int n=0;n<(phase==0?2:phase==55?1:0);n++)KeyBinding.onTick(-100);
                    for(int n=0;n<(phase==20?2:phase==30?1:0);n++)KeyBinding.onTick(-99);
                    // A pick press on a block broken earlier in the cast would read Item.itemsList[0] (1.0 NPE).
                    if((phase==25||phase==50)&&(mc.objectMouseOver==null||w.getBlockId(4,64,4)!=0))KeyBinding.onTick(-98);
                    if(p.isUsingItem()) {if(!use)controller.onStoppedUsingItem(p);while(mc.gameSettings.keyBindAttack.isPressed());while(mc.gameSettings.keyBindUseItem.isPressed());while(mc.gameSettings.keyBindPickBlock.isPressed());} else { // 1.0 runTick drains the presses while using
                        while(mc.gameSettings.keyBindAttack.isPressed())click.invoke(mc,0);
                        while(mc.gameSettings.keyBindUseItem.isPressed())click.invoke(mc,1);
                        while(mc.gameSettings.keyBindPickBlock.isPressed())pick.invoke(mc);
                    }
                    if(use&&right.getInt(mc)==0&&!p.isUsingItem())click.invoke(mc,1);
                    held.invoke(mc,0,attack);
                    ByteArrayOutputStream bytes=new ByteArrayOutputStream();DataOutputStream d=new DataOutputStream(bytes);
                    Mode_controller10.state(w,p,controller,-1,d);
                    for(int n:new int[]{Trace10.namedInt("left.getInt(mc)", left.getInt(mc)),Trace10.namedInt("right.getInt(mc)", right.getInt(mc)),Trace10.namedInt("p.swingProgressInt", p.swingProgressInt),Trace10.namedInt("p.func_35205_Y()", p.func_35205_Y()),Trace10.namedInt("p.inventory.currentItem", p.inventory.currentItem),Trace10.namedInt("mc.gameSettings.keyBindAttack.pressTime", mc.gameSettings.keyBindAttack.pressTime),Trace10.namedInt("mc.gameSettings.keyBindUseItem.pressTime", mc.gameSettings.keyBindUseItem.pressTime),Trace10.namedInt("mc.gameSettings.keyBindPickBlock.pressTime", mc.gameSettings.keyBindPickBlock.pressTime)})EntityOracle.i(d,n);
                    Trace10.writeBoolean(d, "p.isSwinging", p.isSwinging);PlayerOracle.stack(d,p.getItemInUse());
                    EntityOracle.i(o,Trace10.namedInt("bytes.size()", bytes.size()));Trace10.writeBytes(o, "bytes.toByteArray()", bytes.toByteArray());
                }
            }
        }
    }
}
