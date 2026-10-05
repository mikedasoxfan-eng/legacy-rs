package net.minecraft.src;

import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Whole original EntityPlayerSP.onUpdate, without the legacy World dump schema. */
public final class Mode_playertick181 {
    static void i(DataOutputStream o,int v)throws Exception { EntityOracle.i(o,v); }
    static void f(DataOutputStream o,float v)throws Exception { EntityOracle.f(o,v); }
    static int field(Class<?> c,String name,Object p)throws Exception { return EntityOracle.intField(c,name,p); }
    static void frame(World w,PlayerOracle.HeadlessPlayer p,DataOutputStream out)throws Exception {
        ByteArrayOutputStream bytes=new ByteArrayOutputStream();DataOutputStream o=new DataOutputStream(bytes);
        Mode_entities181.state(w,p,o);PlayerOracle.state(p,o);
        FoodStats food=p.func_35191_at();NBTTagCompound n=new NBTTagCompound();food.func_35763_b(n);
        for(int v:new int[]{food.func_35765_a(),food.func_35769_b(),n.getInteger("foodTickTimer"),p.field_35211_aX,p.field_35210_aY,p.field_35209_aZ,p.field_35214_aG,p.field_35221_d,field(EntityPlayerSP.class,"field_35224_c",p),p.field_35216_aw,p.func_35205_Y()})i(o,v);
        for(float v:new float[]{n.getFloat("foodSaturationLevel"),n.getFloat("foodExhaustionLevel"),p.func_35220_u_(),p.swingProgress,p.prevSwingProgress,p.movementInput.moveForward,p.field_35222_e,p.field_35223_ap,p.field_35226_aq,p.field_35225_ar})f(o,v);
        PlayerCapabilities c=p.field_35212_aW;
        for(boolean v:new boolean[]{p.func_35117_Q(),c.field_35759_a,c.field_35757_b,c.field_35758_c,c.field_35756_d,p.movementInput.jump,p.func_35162_ad()})o.writeBoolean(v);
        PlayerOracle.stack(o,p.func_35195_X());
        int arrows=0;for(Object e:w.loadedEntityList)if(e instanceof EntityArrow)arrows++;i(o,arrows);
        for(Object object:w.loadedEntityList)if(object instanceof EntityArrow){EntityArrow e=(EntityArrow)object;o.writeBoolean(e.field_35140_d);for(double v:new double[]{e.motionX,e.motionY,e.motionZ})EntityOracle.d(o,v);}
        i(o,field(p.mc.playerController instanceof PlayerControllerTest?PlayerControllerTest.class:PlayerControllerSP.class,p.mc.playerController instanceof PlayerControllerTest?"field_35647_c":"blockHitWait",p.mc.playerController));
        i(o,w.getBlockId(4,64,4));i(o,w.getBlockId(4,64,5));
        i(out,bytes.size());out.write(bytes.toByteArray());
    }
    static void food(PlayerOracle.HeadlessPlayer p,int level,float saturation)throws Exception {
        NBTTagCompound n=new NBTTagCompound();n.setInteger("foodLevel",level);n.setFloat("foodSaturationLevel",saturation);p.func_35191_at().func_35766_a(n);
    }
    public static void main(String[] a)throws Exception {
        String kind=a[1];int ticks=Integer.parseInt(a[2]);World w=Mode_entities181.world();
        // Mode_entities181.world installs the single complete event recorder.
        Item.itemRand.setSeed(600);
        PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.mc.gameSettings=new GameSettings();
        p.mc.playerController=new PlayerControllerSP(p.mc);
        p.movementInput=new MovementInputFromOptions(p.mc.gameSettings);
        p.setLocationAndAngles(0.5,64,0.5,0,0);
        if(kind.equals("flight"))PlayerControllerTest.func_35646_d(p);
        if(kind.equals("creativecontroller")){p.mc.playerController=new PlayerControllerTest(p.mc);p.inventory.mainInventory[0]=new ItemStack(260,37,0);p.mc.playerController.func_6473_b(p);}
        if(kind.equals("starve")){food(p,0,0);p.health=12;for(int j=0;j<4;j++)p.inventory.armorInventory[j]=new ItemStack(310+j,1,0);}
        if(kind.equals("regen")){food(p,20,5);p.health=10;}
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            frame(w,p,o);
            for(int t=0;t<ticks;t++) {
                GameSettings g=p.mc.gameSettings;
                g.keyBindForward.field_35965_e=(kind.equals("movement")&&t%80<60)||(kind.equals("sprint")&&(t%60==0||t%60>=2&&t%60<45));
                g.keyBindLeft.field_35965_e=kind.equals("movement")&&t%80>=60;
                g.keyBindJump.field_35965_e=(kind.equals("movement")&&t%31<8)||(kind.equals("flight")&&(t==1||t==3||t>=10&&t<18||t==40||t==42));
                g.keyBindSneak.field_35965_e=(kind.equals("movement")&&t%80>=70)||(kind.equals("flight")&&t>=22&&t<30);
                if(kind.equals("flight")&&t==0)p.setLocationAndAngles(0.5,75,0.5,0,0);
                if(kind.equals("sprint")&&t==90)food(p,6,0);
                if(kind.equals("sprint")&&t==150)food(p,20,5);
                if(kind.startsWith("eat")&&t%80==0) {
                    int id=Integer.parseInt(kind.substring(3));food(p,10,0);p.inventory.mainInventory[0]=new ItemStack(id,3,0);p.inventory.currentItem=0;p.mc.playerController.sendUseItem(p,w,p.inventory.mainInventory[0]);
                }
                if(kind.equals("cancel")&&t%40==0){food(p,10,0);p.inventory.mainInventory[0]=new ItemStack(260,3,0);p.inventory.currentItem=0;p.func_35199_b(p.inventory.mainInventory[0],32);}
                if(kind.equals("cancel")&&t%40==10)p.inventory.currentItem=1;
                if(kind.equals("xp")&&t%3==0){EntityXPOrb orb=new EntityXPOrb(w,p.posX,p.boundingBox.minY,p.posZ,1+t%37);w.entityJoinedWorld(orb);}
                if(kind.equals("damage")&&t%30==0){p.health=20;p.heartsLife=0;for(int j=0;j<4;j++)p.inventory.armorInventory[j]=new ItemStack(310+j,1,j*40);p.inventory.mainInventory[0]=new ItemStack(267,1,0);p.func_35199_b(p.inventory.mainInventory[0],72000);}
                if(kind.equals("damage")&&t%5==0)p.attackEntityFrom(t%10==0?DamageSource.field_35536_f:DamageSource.field_35537_g,5);
                if(kind.equals("creativevoid")&&t==0){PlayerControllerTest.func_35646_d(p);p.attackEntityFrom(DamageSource.field_35550_i,4);}
                if(kind.equals("speed")&&t==0)p.func_35165_a(new PotionEffect(1,80,1));
                if(kind.equals("haste")&&t%100==0)p.func_35165_a(new PotionEffect(3,80,t/100%8));
                if(kind.equals("fatigue")&&t%100==0)p.func_35165_a(new PotionEffect(4,80,t/100%4));
                if(kind.equals("nausea")&&t==0)p.func_35165_a(new PotionEffect(9,300,0));
                if(kind.equals("creativedamage")){PlayerControllerTest.func_35646_d(p);if(t%5==0)p.attackEntityFrom(t%10==0?DamageSource.field_35536_f:DamageSource.field_35537_g,5);}
                if(kind.equals("identity")) {
                    if(t%80==0){food(p,10,0);p.inventory.mainInventory[0]=new ItemStack(260,3,0);p.inventory.currentItem=0;p.func_35199_b(p.inventory.mainInventory[0],32);}
                    if(t%80==4)p.func_35199_b(p.inventory.getCurrentItem(),32);
                    if(t%80==8)p.inventory.mainInventory[0].stackSize=2;
                    if(t%80==12){p.inventory.mainInventory[1]=p.inventory.mainInventory[0];p.inventory.mainInventory[0]=null;p.inventory.currentItem=1;}
                    if(t%80==18)p.inventory.mainInventory[1]=new ItemStack(260,2,0);
                }
                if(kind.equals("bow")) {
                    if(t%40==0){p.inventory.mainInventory[0]=new ItemStack(261,1,0);p.inventory.mainInventory[1]=new ItemStack(262,16,0);p.inventory.currentItem=0;p.mc.playerController.sendUseItem(p,w,p.inventory.mainInventory[0]);}
                    if(t%40==new int[]{1,3,10,20,30}[(t/40)%5])p.func_35206_ab();
                }
                if(kind.equals("creativecontroller")) {
                    w.setBlockWithNotify(4,64,4,1);if(t%11==0)w.setBlockWithNotify(4,64,5,51);
                    if(t%13==0)p.mc.playerController.clickBlock(4,64,4,3);
                    else if(t%13==4)p.mc.playerController.resetBlockRemoving();
                    else p.mc.playerController.sendBlockRemoving(4,64,4,3);
                }
                if(t%13==0)p.swingItem();
                AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();p.onUpdate();frame(w,p,o);
            }
        }
    }
}
