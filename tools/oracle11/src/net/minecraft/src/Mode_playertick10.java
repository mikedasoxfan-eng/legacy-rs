package net.minecraft.src;

import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Whole original EntityPlayerSP.onUpdate, without the legacy World dump schema. */
public final class Mode_playertick10 {
    static void i(DataOutputStream o,int v)throws Exception { EntityOracle.i(o,v); }
    static void f(DataOutputStream o,float v)throws Exception { EntityOracle.f(o,v); }
    static int field(Class<?> c,String name,Object p)throws Exception { return EntityOracle.intField(c,name,p); }
    static void frame(World w,PlayerOracle.HeadlessPlayer p,DataOutputStream out)throws Exception {
        ByteArrayOutputStream bytes=new ByteArrayOutputStream();DataOutputStream o=new DataOutputStream(bytes);
        Mode_entities10.state(w,p,o);PlayerOracle.state(p,o);
        FoodStats food=p.getFoodStats();NBTTagCompound n=new NBTTagCompound();food.writeStatsToNBT(n);
        for(int v:new int[]{Trace10.namedInt("food.getFoodLevel()", food.getFoodLevel()),Trace10.namedInt("food.getPrevFoodLevel()", food.getPrevFoodLevel()),Trace10.namedInt("n.getInteger(\"foodTickTimer\")", n.getInteger("foodTickTimer")),Trace10.namedInt("Float.floatToIntBits(p.currentXP)", Float.floatToIntBits(p.currentXP)),Trace10.namedInt("p.playerLevel", p.playerLevel),Trace10.namedInt("p.totalXP", p.totalXP),Trace10.namedInt("p.xpCooldown", p.xpCooldown),Trace10.namedInt("p.sprintingTicksLeft", p.sprintingTicksLeft),Trace10.namedInt("field(EntityPlayerSP.class,\"sprintToggleTimer\",p)", field(EntityPlayerSP.class,"sprintToggleTimer",p)),Trace10.namedInt("p.flyToggleTimer", p.flyToggleTimer),Trace10.namedInt("p.func_35205_Y()", p.func_35205_Y())})i(o,v);
        for(float v:new float[]{Trace10.namedFloat("n.getFloat(\"foodSaturationLevel\")", n.getFloat("foodSaturationLevel")),Trace10.namedFloat("n.getFloat(\"foodExhaustionLevel\")", n.getFloat("foodExhaustionLevel")),Trace10.namedFloat("p.getFOVMultiplier()", p.getFOVMultiplier()),Trace10.namedFloat("p.swingProgress", p.swingProgress),Trace10.namedFloat("p.prevSwingProgress", p.prevSwingProgress),Trace10.namedFloat("p.movementInput.moveForward", p.movementInput.moveForward),Trace10.namedFloat("p.renderArmYaw", p.renderArmYaw),Trace10.namedFloat("p.renderArmPitch", p.renderArmPitch),Trace10.namedFloat("p.prevRenderArmYaw", p.prevRenderArmYaw),Trace10.namedFloat("p.prevRenderArmPitch", p.prevRenderArmPitch)})f(o,v);
        PlayerCapabilities c=p.capabilities;
        for(boolean v:new boolean[]{p.isSprinting(),c.disableDamage,c.isFlying,c.allowFlying,c.depleteBuckets,p.movementInput.jump,p.isBlocking()})Trace10.writeBoolean(o, "v", v);
        PlayerOracle.stack(o,p.getItemInUse());
        int arrows=0;for(Object e:w.loadedEntityList)if(e instanceof EntityArrow)arrows++;i(o,arrows);
        for(Object object:w.loadedEntityList)if(object instanceof EntityArrow){EntityArrow e=(EntityArrow)object;Trace10.writeBoolean(o, "e.arrowCritical", e.arrowCritical);for(double v:new double[]{Trace10.namedDouble("e.motionX", e.motionX),Trace10.namedDouble("e.motionY", e.motionY),Trace10.namedDouble("e.motionZ", e.motionZ)})EntityOracle.d(o,v);}
        i(o,Trace10.namedInt("field(p.mc.playerController instanceof PlayerControllerCreative?PlayerControllerCreative.class:PlayerControllerSP.class,p.mc.playerController instanceof PlayerControllerCreative?\"field_35647_c\":\"blockHitWait\",p.mc.playerController)", field(p.mc.playerController instanceof PlayerControllerCreative?PlayerControllerCreative.class:PlayerControllerSP.class,p.mc.playerController instanceof PlayerControllerCreative?"field_35647_c":"blockHitWait",p.mc.playerController)));
        i(o,Trace10.namedInt("w.getBlockId(4,64,4)", w.getBlockId(4,64,4)));i(o,Trace10.namedInt("w.getBlockId(4,64,5)", w.getBlockId(4,64,5)));
        i(out,Trace10.namedInt("bytes.size()", bytes.size()));Trace10.writeBytes(out, "bytes.toByteArray()", bytes.toByteArray());
    }
    static void food(PlayerOracle.HeadlessPlayer p,int level,float saturation)throws Exception {
        NBTTagCompound n=new NBTTagCompound();n.setInteger("foodLevel",level);n.setFloat("foodSaturationLevel",saturation);p.getFoodStats().readStatsFromNBT(n);
    }
    public static void main(String[] a)throws Exception {
        String kind=a[1];int ticks=Integer.parseInt(a[2]);World w=Mode_entities10.world();
        // Mode_entities10.world installs the single complete event recorder.
        Item.itemRand.setSeed(600);
        PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.mc.gameSettings=new GameSettings();
        p.mc.playerController=new PlayerControllerSP(p.mc);
        p.movementInput=new MovementInputFromOptions(p.mc.gameSettings);
        p.setLocationAndAngles(0.5,64,0.5,0,0);
        if(kind.equals("flight"))PlayerControllerCreative.enableAbilities(p);
        if(kind.equals("creativecontroller")){p.mc.playerController=new PlayerControllerCreative(p.mc);p.inventory.mainInventory[0]=new ItemStack(260,37,0);p.mc.playerController.func_6473_b(p);}
        if(kind.equals("starve")){food(p,0,0);p.health=12;for(int j=0;j<4;j++)p.inventory.armorInventory[j]=new ItemStack(310+j,1,0);}
        if(kind.equals("regen")){food(p,20,5);p.health=10;}
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            frame(w,p,o);
            for(int t=0;t<ticks;t++) {
                GameSettings g=p.mc.gameSettings;
                g.keyBindForward.pressed=(kind.equals("movement")&&t%80<60)||(kind.equals("sprint")&&(t%60==0||t%60>=2&&t%60<45));
                g.keyBindLeft.pressed=kind.equals("movement")&&t%80>=60;
                g.keyBindJump.pressed=(kind.equals("movement")&&t%31<8)||(kind.equals("flight")&&(t==1||t==3||t>=10&&t<18||t==40||t==42));
                g.keyBindSneak.pressed=(kind.equals("movement")&&t%80>=70)||(kind.equals("flight")&&t>=22&&t<30);
                if(kind.equals("flight")&&t==0)p.setLocationAndAngles(0.5,75,0.5,0,0);
                if(kind.equals("sprint")&&t==90)food(p,6,0);
                if(kind.equals("sprint")&&t==150)food(p,20,5);
                if(kind.startsWith("eat")&&t%80==0) {
                    int id=Integer.parseInt(kind.substring(3));food(p,10,0);p.inventory.mainInventory[0]=new ItemStack(id,3,0);p.inventory.currentItem=0;p.mc.playerController.sendUseItem(p,w,p.inventory.mainInventory[0]);
                }
                if(kind.equals("cancel")&&t%40==0){food(p,10,0);p.inventory.mainInventory[0]=new ItemStack(260,3,0);p.inventory.currentItem=0;p.setItemInUse(p.inventory.mainInventory[0],32);}
                if(kind.equals("cancel")&&t%40==10)p.inventory.currentItem=1;
                if(kind.equals("xp")&&t%3==0){EntityXPOrb orb=new EntityXPOrb(w,p.posX,p.boundingBox.minY,p.posZ,1+t%37);w.spawnEntityInWorld(orb);}
                if(kind.equals("damage")&&t%30==0){p.health=20;p.heartsLife=0;for(int j=0;j<4;j++)p.inventory.armorInventory[j]=new ItemStack(310+j,1,j*40);p.inventory.mainInventory[0]=new ItemStack(267,1,0);p.setItemInUse(p.inventory.mainInventory[0],72000);}
                if(kind.equals("damage")&&t%5==0)p.attackEntityFrom(t%10==0?DamageSource.starve:DamageSource.cactus,5);
                if(kind.equals("creativevoid")&&t==0){PlayerControllerCreative.enableAbilities(p);p.attackEntityFrom(DamageSource.outOfWorld,4);}
                if(kind.equals("speed")&&t==0)p.addPotionEffect(new PotionEffect(1,80,1));
                if(kind.equals("haste")&&t%100==0)p.addPotionEffect(new PotionEffect(3,80,t/100%8));
                if(kind.equals("fatigue")&&t%100==0)p.addPotionEffect(new PotionEffect(4,80,t/100%4));
                if(kind.equals("nausea")&&t==0)p.addPotionEffect(new PotionEffect(9,300,0));
                if(kind.equals("creativedamage")){PlayerControllerCreative.enableAbilities(p);if(t%5==0)p.attackEntityFrom(t%10==0?DamageSource.starve:DamageSource.cactus,5);}
                if(kind.equals("identity")) {
                    if(t%80==0){food(p,10,0);p.inventory.mainInventory[0]=new ItemStack(260,3,0);p.inventory.currentItem=0;p.setItemInUse(p.inventory.mainInventory[0],32);}
                    if(t%80==4)p.setItemInUse(p.inventory.getCurrentItem(),32);
                    if(t%80==8)p.inventory.mainInventory[0].stackSize=2;
                    if(t%80==12){p.inventory.mainInventory[1]=p.inventory.mainInventory[0];p.inventory.mainInventory[0]=null;p.inventory.currentItem=1;}
                    if(t%80==18)p.inventory.mainInventory[1]=new ItemStack(260,2,0);
                }
                if(kind.equals("bow")) {
                    if(t%40==0){p.inventory.mainInventory[0]=new ItemStack(261,1,0);p.inventory.mainInventory[1]=new ItemStack(262,16,0);p.inventory.currentItem=0;p.mc.playerController.sendUseItem(p,w,p.inventory.mainInventory[0]);}
                    if(t%40==new int[]{1,3,10,20,30}[(t/40)%5])p.stopUsingItem();
                }
                if(kind.equals("creativecontroller")) {
                    w.setBlockWithNotify(4,64,4,1);if(t%11==0)w.setBlockWithNotify(4,64,5,51);
                    if(t%13==0)p.mc.playerController.clickBlock(4,64,4,3);
                    else if(t%13==4)p.mc.playerController.resetBlockRemoving();
                    else p.mc.playerController.onPlayerDamageBlock(4,64,4,3);
                }
                if(t%13==0)p.swingItem();
                AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();p.onUpdate();frame(w,p,o);
            }
        }
    }
}
