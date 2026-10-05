package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.Random;
public final class Mode_wbdispensertile10 {
    public static void main(String[] a)throws Exception {
        Mode_wbmachine10.run(a,23);
        Field random=TileEntityDispenser.class.getDeclaredField("dispenserRandom");random.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1],true)))) {
            for(int mask=0;mask<512;mask++) {
                World w=Mode_worldblocks181.fixture(true);Mode_worldblocks181.raw(w,8,64,8,23,5);
                TileEntityDispenser t=(TileEntityDispenser)w.getBlockTileEntity(8,64,8);
                for(int slot=0;slot<9;slot++)if((mask&(1<<slot))!=0)t.setInventorySlotContents(slot,new ItemStack(263,slot-3,slot));
                if((mask&1)!=0)w.setBlockTileEntity(8,64,8,new TileEntityDispenser());
                Random r=(Random)random.get(t);r.setSeed(42);
                for(int k=0;k<12;k++) {
                    w.getChunkFromChunkCoords(0,0).isModified=false;
                    Mode_wbfurnace4.stack(o,t.getRandomStackFromInventory());
                    for(int slot=0;slot<9;slot++)Mode_wbfurnace4.stack(o,t.getStackInSlot(slot));
                    EntityOracle.l(o,EntityOracle.rngState(r));EntityOracle.i(o,t.field_35145_n);o.writeBoolean(w.getChunkFromChunkCoords(0,0).isModified);
                }
            }
        }
    }
}
