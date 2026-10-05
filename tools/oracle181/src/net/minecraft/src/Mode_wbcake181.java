package net.minecraft.src;
import java.io.*;
/** BlockCake.eatCakeSlice (b1.8.1: canEat(false), FoodStats.addStats(2, 0.1F)) through both of
 * its callers, blockActivated (right click) and onBlockClicked (left click). */
public final class Mode_wbcake181 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);EntityPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int meta=0;meta<8;meta++)for(int food:new int[]{0,7,19,20})for(float sat:new float[]{0F,4.5F,20F})for(boolean creative:new boolean[]{false,true})for(int hook=0;hook<2;hook++) {
                Mode_worldblocks181.raw(w,8,63,8,1,0);Mode_worldblocks181.raw(w,8,64,8,92,meta);
                p.func_35191_at().func_35764_a(food);p.func_35191_at().func_35767_b(sat);p.field_35212_aW.field_35759_a=creative;
                if(hook==0)Block.cake.onBlockClicked(w,8,64,8,p);else o.writeBoolean(Block.cake.blockActivated(w,8,64,8,p));
                EntityOracle.i(o,p.func_35191_at().func_35765_a());EntityOracle.f(o,p.func_35191_at().func_35760_d());
                EntityOracle.i(o,w.getBlockId(8,64,8));EntityOracle.i(o,w.getBlockMetadata(8,64,8));
            }
        }
    }
}
