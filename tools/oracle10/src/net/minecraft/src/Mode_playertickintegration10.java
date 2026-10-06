package net.minecraft.src;
import java.io.*;
import static net.minecraft.src.Mode_playertick10.*;
/** Original whole-player driver and snapshots, with the potion inputs used by
 * the Rust replay. The older driver silently treated these scenarios as idle. */
public final class Mode_playertickintegration10 {
    public static void main(String[] a)throws Exception {
        String kind=a[1];int ticks=Integer.parseInt(a[2]);World w=Mode_entities10.world();
        // Mode_entities10.world() now attaches only WorldEventOracle's recorder.
        Item.itemRand.setSeed(600);
        PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);p.mc.gameSettings=new GameSettings();
        p.mc.playerController=new PlayerControllerSP(p.mc);
        p.movementInput=new MovementInputFromOptions(p.mc.gameSettings);
        p.setLocationAndAngles(0.5,64,0.5,0,0);
        if(kind.equals("flight"))PlayerControllerCreative.func_35646_d(p);
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
                if(kind.equals("xp")&&t%3==0){EntityXPOrb orb=new EntityXPOrb(w,p.posX,p.boundingBox.minY,p.posZ,1+t%37);w.entityJoinedWorld(orb);}
                if(kind.equals("damage")&&t%30==0){p.health=20;p.heartsLife=0;for(int j=0;j<4;j++)p.inventory.armorInventory[j]=new ItemStack(310+j,1,j*40);p.inventory.mainInventory[0]=new ItemStack(267,1,0);p.setItemInUse(p.inventory.mainInventory[0],72000);}
                if(kind.equals("damage")&&t%5==0)p.attackEntityFrom(t%10==0?DamageSource.starve:DamageSource.cactus,5);
                if(kind.equals("creativevoid")&&t==0){PlayerControllerCreative.func_35646_d(p);p.attackEntityFrom(DamageSource.outOfWorld,4);}
                if(kind.equals("speed")&&t==0)p.addPotionEffect(new PotionEffect(1,80,1));
                if(kind.equals("haste")&&t%100==0)p.addPotionEffect(new PotionEffect(3,80,t/100%8));
                if(kind.equals("fatigue")&&t%100==0)p.addPotionEffect(new PotionEffect(4,80,t/100%4));
                if(kind.equals("nausea")&&t==0)p.addPotionEffect(new PotionEffect(9,300,0));
                if(kind.equals("creativedamage")){PlayerControllerCreative.func_35646_d(p);if(t%5==0)p.attackEntityFrom(t%10==0?DamageSource.starve:DamageSource.cactus,5);}
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
                    else p.mc.playerController.sendBlockRemoving(4,64,4,3);
                }
                if(t%13==0)p.swingItem();
                AxisAlignedBB.clearBoundingBoxPool();Vec3D.initialize();p.onUpdate();frame(w,p,o);
            }
        }
    }
}
