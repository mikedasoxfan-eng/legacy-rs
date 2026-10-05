package net.minecraft.src;
import java.io.*;
public final class Mode_wbplates4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{70,72})for(int floor=0;floor<=109;floor++)for(int meta:new int[]{0,1}) {
                World w=Mode_worldblocks181.fixture(true);w.rand.setSeed(42);EntityOracle.mathRandom().setSeed(123);OracleHooks.reset(100);WorldEventOracle.attach(w);
                Mode_worldblocks181.raw(w,8,63,8,floor,0);Mode_worldblocks181.raw(w,8,64,8,id,meta);
                o.writeBoolean(Block.blocksList[id].canPlaceBlockAt(w,8,64,8));Block.blocksList[id].onNeighborBlockChange(w,8,64,8,1);
                o.writeByte(w.getBlockId(8,64,8));EntityOracle.i(o,w.getBlockMetadata(8,64,8));EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.i(o,w.loadedEntityList.size());WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
