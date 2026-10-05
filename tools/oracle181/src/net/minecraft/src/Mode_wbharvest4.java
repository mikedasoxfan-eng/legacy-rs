package net.minecraft.src;
import java.io.*;
/** Shears and ordinary harvest for leaves, tall grass and vines at every metadata value. */
public final class Mode_wbharvest4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{18,31,106})for(int meta=0;meta<16;meta++)for(int tool:new int[]{0,359}) {
                World w=Mode_worldblocks181.fixture(true);EntityPlayer p=PlayerOracle.player(w);w.rand.setSeed(42);EntityOracle.mathRandom().setSeed(123);OracleHooks.reset(100);
                if(tool!=0)p.inventory.mainInventory[0]=new ItemStack(tool,1,0);
                Block.blocksList[id].harvestBlock(w,p,8,64,8,meta);EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.i(o,w.loadedEntityList.size());
                for(Object v:w.loadedEntityList) {EntityItem e=(EntityItem)v;EntityOracle.i(o,e.item.itemID);EntityOracle.i(o,e.item.stackSize);EntityOracle.i(o,e.item.getItemDamage());EntityOracle.d(o,e.posX);EntityOracle.d(o,e.posY);EntityOracle.d(o,e.posZ);}
            }
        }
    }
}
