package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** Original Item methods, player use completion, and FoodStats, without terrain. */
public final class Mode_wbuse {
    static void i(DataOutputStream o,int v)throws Exception {EntityOracle.i(o,v);}
    static void f(DataOutputStream o,float v)throws Exception {EntityOracle.f(o,v);}
    static void l(DataOutputStream o,long v)throws Exception {EntityOracle.l(o,v);}
    static void state(DataOutputStream o,EntityPlayer p)throws Exception {
        Trace10.writeBoolean(o, "p.isUsingItem()", p.isUsingItem());i(o,Trace10.namedInt("p.func_35205_Y()", p.func_35205_Y()));Trace10.writeBoolean(o, "p.func_35162_ad()", p.isBlocking());
        ItemStack s=p.inventory.getCurrentItem();Trace10.writeBoolean(o, "s!=null", s!=null);if(s!=null){i(o,Trace10.namedInt("s.itemID", s.itemID));i(o,Trace10.namedInt("s.stackSize", s.stackSize));i(o,Trace10.namedInt("s.getItemDamage()", s.getItemDamage()));}
        NBTTagCompound n=new NBTTagCompound();p.getFoodStats().writeStatsToNBT(n);
        i(o,Trace10.namedInt("n.getInteger(\"foodLevel\")", n.getInteger("foodLevel")));f(o,Trace10.namedFloat("n.getFloat(\"foodSaturationLevel\")", n.getFloat("foodSaturationLevel")));f(o,Trace10.namedFloat("n.getFloat(\"foodExhaustionLevel\")", n.getFloat("foodExhaustionLevel")));i(o,Trace10.namedInt("n.getInteger(\"foodTickTimer\")", n.getInteger("foodTickTimer")));i(o,Trace10.namedInt("p.health", p.health));
        for(int id:new int[]{10,17}) {PotionEffect e=p.getActivePotionEffect(Potion.potionTypes[id]);Trace10.writeBoolean(o, "e!=null", e!=null);if(e!=null){i(o,Trace10.namedInt("e.getDuration()", e.getDuration()));i(o,Trace10.namedInt("e.getAmplifier()", e.getAmplifier()));}}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            Method finish=EntityPlayer.class.getDeclaredMethod("func_35208_ae");finish.setAccessible(true);
            Field rand=Item.class.getDeclaredField("itemRand");rand.setAccessible(true);
            for(int id:new int[]{260,282,297,319,320,322,349,350,357,360,363,364,365,366,367,267,261})for(int level:new int[]{0,19,20})for(int seed=0;seed<8;seed++) {
                World w=Mode_worldblocks10.fixture(true);EntityPlayer p=PlayerOracle.player(w);p.rand.setSeed(123);w.rand.setSeed(seed);EntityOracle.mathRandom().setSeed(456);
                p.setPosition(8.25,66.0,8.75);p.rotationYaw=31.0F;p.rotationPitch=-17.0F;
                p.getFoodStats().setFoodLevel(level);p.inventory.mainInventory[0]=new ItemStack(id,3,0);
                p.inventory.mainInventory[1]=new ItemStack(262,64,0);
                Item.itemsList[id].onItemRightClick(p.inventory.mainInventory[0],w,p);state(o,p);
                if(id!=267 && id!=261)finish.invoke(p);else p.stopUsingItem();
                state(o,p);l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));l(o,Trace10.namedLong("EntityOracle.rngState(p.rand)", EntityOracle.rngState(p.rand)));l(o,Trace10.namedLong("EntityOracle.rngState(EntityOracle.mathRandom())", EntityOracle.rngState(EntityOracle.mathRandom())));
                WorldEventOracle.snapshot(w,o);
            }
            for(int elapsed=-20;elapsed<=80;elapsed++) {
                World w=Mode_worldblocks10.fixture(true);OracleHooks.reset(100);EntityPlayer p=PlayerOracle.player(w);
                p.setPosition(8.25,66,8.75);p.rotationYaw=31;p.rotationPitch=-17;
                p.inventory.mainInventory[0]=new ItemStack(261,1,0);p.inventory.mainInventory[1]=new ItemStack(262,64,0);((Random)rand.get(null)).setSeed(123);
                Item.bow.onPlayerStoppedUsing(p.inventory.mainInventory[0],w,p,72000-elapsed);
                i(o,Trace10.namedInt("p.inventory.mainInventory[1].stackSize", p.inventory.mainInventory[1].stackSize));i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));l(o,Trace10.namedLong("EntityOracle.rngState((Random)rand.get(null))", EntityOracle.rngState((Random)rand.get(null))));
                if(!w.loadedEntityList.isEmpty()) {EntityArrow e=(EntityArrow)w.loadedEntityList.get(0);Trace10.writeBoolean(o, "e.arrowCritical", e.arrowCritical);for(double v:new double[]{Trace10.namedDouble("e.posX", e.posX),Trace10.namedDouble("e.posY", e.posY),Trace10.namedDouble("e.posZ", e.posZ),Trace10.namedDouble("e.motionX", e.motionX),Trace10.namedDouble("e.motionY", e.motionY),Trace10.namedDouble("e.motionZ", e.motionZ)})EntityOracle.d(o,v);f(o,Trace10.namedFloat("e.rotationYaw", e.rotationYaw));f(o,Trace10.namedFloat("e.rotationPitch", e.rotationPitch));l(o,Trace10.namedLong("EntityOracle.rngState(e.rand)", EntityOracle.rngState(e.rand)));}
                WorldEventOracle.snapshot(w,o);
            }
            for(int level:new int[]{0,6,17,18,20})for(int difficulty=0;difficulty<=3;difficulty++) {
                World w=Mode_worldblocks10.fixture(true);w.difficultySetting=difficulty;EntityPlayer p=PlayerOracle.player(w);p.health=12;
                FoodStats food=p.getFoodStats();food.setFoodLevel(level);food.setFoodSaturationLevel(1.25F);
                for(int t=0;t<240;t++){if(t%7==0)food.addExhaustion(0.73F);if(t%61==0)food.addStats(1,0.3F);food.onUpdate(p);state(o,p);}
            }
        }
    }
}
