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
        o.writeBoolean(p.func_35196_Z());i(o,p.func_35205_Y());o.writeBoolean(p.func_35162_ad());
        ItemStack s=p.inventory.getCurrentItem();o.writeBoolean(s!=null);if(s!=null){i(o,s.itemID);i(o,s.stackSize);i(o,s.getItemDamage());}
        NBTTagCompound n=new NBTTagCompound();p.func_35191_at().func_35763_b(n);
        i(o,n.getInteger("foodLevel"));f(o,n.getFloat("foodSaturationLevel"));f(o,n.getFloat("foodExhaustionLevel"));i(o,n.getInteger("foodTickTimer"));i(o,p.health);
        for(int id:new int[]{10,17}) {PotionEffect e=p.func_35167_b(Potion.field_35678_a[id]);o.writeBoolean(e!=null);if(e!=null){i(o,e.func_35802_b());i(o,e.func_35801_c());}}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            Method finish=EntityPlayer.class.getDeclaredMethod("func_35208_ae");finish.setAccessible(true);
            Field rand=Item.class.getDeclaredField("itemRand");rand.setAccessible(true);
            for(int id:new int[]{260,282,297,319,320,322,349,350,357,360,363,364,365,366,367,267,261})for(int level:new int[]{0,19,20})for(int seed=0;seed<8;seed++) {
                World w=Mode_worldblocks181.fixture(true);EntityPlayer p=PlayerOracle.player(w);p.rand.setSeed(123);w.rand.setSeed(seed);EntityOracle.mathRandom().setSeed(456);
                p.setPosition(8.25,66.0,8.75);p.rotationYaw=31.0F;p.rotationPitch=-17.0F;
                p.func_35191_at().func_35764_a(level);p.inventory.mainInventory[0]=new ItemStack(id,3,0);
                p.inventory.mainInventory[1]=new ItemStack(262,64,0);
                Item.itemsList[id].onItemRightClick(p.inventory.mainInventory[0],w,p);state(o,p);
                if(id!=267 && id!=261)finish.invoke(p);else p.func_35206_ab();
                state(o,p);l(o,EntityOracle.rngState(w.rand));l(o,EntityOracle.rngState(p.rand));l(o,EntityOracle.rngState(EntityOracle.mathRandom()));
                WorldEventOracle.snapshot(w,o);
            }
            for(int elapsed=-20;elapsed<=80;elapsed++) {
                World w=Mode_worldblocks181.fixture(true);OracleHooks.reset(100);EntityPlayer p=PlayerOracle.player(w);
                p.setPosition(8.25,66,8.75);p.rotationYaw=31;p.rotationPitch=-17;
                p.inventory.mainInventory[0]=new ItemStack(261,1,0);p.inventory.mainInventory[1]=new ItemStack(262,64,0);((Random)rand.get(null)).setSeed(123);
                Item.bow.func_35414_a(p.inventory.mainInventory[0],w,p,72000-elapsed);
                i(o,p.inventory.mainInventory[1].stackSize);i(o,w.loadedEntityList.size());l(o,EntityOracle.rngState((Random)rand.get(null)));
                if(!w.loadedEntityList.isEmpty()) {EntityArrow e=(EntityArrow)w.loadedEntityList.get(0);o.writeBoolean(e.field_35140_d);for(double v:new double[]{e.posX,e.posY,e.posZ,e.motionX,e.motionY,e.motionZ})EntityOracle.d(o,v);f(o,e.rotationYaw);f(o,e.rotationPitch);l(o,EntityOracle.rngState(e.rand));}
                WorldEventOracle.snapshot(w,o);
            }
            for(int level:new int[]{0,6,17,18,20})for(int difficulty=0;difficulty<=3;difficulty++) {
                World w=Mode_worldblocks181.fixture(true);w.difficultySetting=difficulty;EntityPlayer p=PlayerOracle.player(w);p.health=12;
                FoodStats food=p.func_35191_at();food.func_35764_a(level);food.func_35767_b(1.25F);
                for(int t=0;t<240;t++){if(t%7==0)food.func_35762_a(0.73F);if(t%61==0)food.func_35771_a(1,0.3F);food.func_35768_a(p);state(o,p);}
            }
        }
    }
}
