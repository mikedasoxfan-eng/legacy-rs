package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;

/** ItemFood/ItemAppleGold/ItemPotion/ItemBucketMilk onFoodEaten (via EntityPlayer.func_35208_ae) and ItemBow release. */
public final class Mode_wbb4eat {
    static final int[] FOODS={260,282,297,319,320,322,349,350,357,360,363,364,365,366,367,375,335,373,261,267};
    static final int[] POTIONS={0,16,8193,8194,8196,8201,8204,8225,8257,16385,16389,16396,32767};
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        Field count=EntityPlayer.class.getDeclaredField("itemInUseCount");count.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:FOODS)for(int damage:id==373?POTIONS:new int[]{0})for(int amount:new int[]{1,2})for(int food:new int[]{20,3})for(int effects=0;effects<3;effects++)for(boolean remote:new boolean[]{false,true}) {
                World w=Mode_worldblocks10.fixture(true);EntityPlayer p=Blocks4Oracle.newPlayer(w);Blocks4Oracle.place(p,8.5,65,8.5,30F,20F);
                p.getFoodStats().setFoodLevel(food);
                if(effects>=1){p.addPotionEffect(new PotionEffect(10,40,0));p.addPotionEffect(new PotionEffect(17,2000,1));}
                if(effects==2){p.addPotionEffect(new PotionEffect(1,9000,3));p.addPotionEffect(new PotionEffect(19,10,0));p.addPotionEffect(new PotionEffect(6,1,0));}
                if(amount==2)for(int slot=1;slot<36;slot++)p.inventory.mainInventory[slot]=new ItemStack(1,64,0);
                ItemStack s=new ItemStack(id,amount,damage);p.inventory.mainInventory[0]=s;p.setItemInUse(s,1);
                w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);
                p.func_35208_ae();
                Blocks4Oracle.player(o,p);Blocks4Oracle.entities(o,w);Blocks4Oracle.rngs(o,w);WorldEventOracle.snapshot(w,o);
            }
            for(int remaining:new int[]{72000,71999,71998,71997,71995,71992,71990,71986,71980,71970,71950,71900,0,-5})for(int arrows=0;arrows<3;arrows++)for(boolean creative:new boolean[]{false,true})for(int damage:new int[]{0,383,384})for(boolean remote:new boolean[]{false,true}) {
                World w=Mode_worldblocks10.fixture(true);EntityPlayer p=Blocks4Oracle.newPlayer(w);Blocks4Oracle.place(p,8.5,65,8.5,-60F,-10F);
                p.capabilities.depleteBuckets=creative;p.capabilities.disableDamage=creative;
                if(arrows>=1)p.inventory.mainInventory[7]=new ItemStack(262,arrows,0);
                ItemStack bow=new ItemStack(261,1,damage);p.inventory.mainInventory[0]=bow;p.setItemInUse(bow,72000);count.setInt(p,remaining);
                w.multiplayerWorld=remote;Blocks4Oracle.seeds(w);
                p.stopUsingItem();
                Blocks4Oracle.player(o,p);Blocks4Oracle.entities(o,w);Blocks4Oracle.rngs(o,w);WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
