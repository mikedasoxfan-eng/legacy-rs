package net.minecraft.src;
import java.io.*;
/** BlockCake.eatCakeSlice (1.0: canEat(false), FoodStats.addStats(2, 0.1F)) through both of
 * its callers, blockActivated (right click) and onBlockClicked (left click). */
public final class Mode_wbcake10 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks10.fixture(true);EntityPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int meta=0;meta<8;meta++)for(int food:new int[]{0,7,19,20})for(float sat:new float[]{0F,4.5F,20F})for(boolean creative:new boolean[]{false,true})for(int hook=0;hook<2;hook++) {
                Mode_worldblocks10.raw(w,8,63,8,1,0);Mode_worldblocks10.raw(w,8,64,8,92,meta);
                p.getFoodStats().setFoodLevel(food);p.getFoodStats().setFoodSaturationLevel(sat);p.capabilities.disableDamage=creative;
                if(hook==0)Block.cake.onBlockClicked(w,8,64,8,p);else Trace10.writeBoolean(o, "Block.cake.blockActivated(w,8,64,8,p)", Block.cake.blockActivated(w,8,64,8,p));
                EntityOracle.i(o,Trace10.namedInt("p.getFoodStats().getFoodLevel()", p.getFoodStats().getFoodLevel()));EntityOracle.f(o,Trace10.namedFloat("p.getFoodStats().getFoodSaturationLevel()", p.getFoodStats().getFoodSaturationLevel()));
                EntityOracle.i(o,Trace10.namedInt("w.getBlockId(8,64,8)", w.getBlockId(8,64,8)));EntityOracle.i(o,Trace10.namedInt("w.getBlockMetadata(8,64,8)", w.getBlockMetadata(8,64,8)));
            }
        }
    }
}
