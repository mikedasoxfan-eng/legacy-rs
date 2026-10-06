package net.minecraft.src;
import java.io.*;
public final class Mode_wbplates4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{70,72})for(int floor=0;floor<=109;floor++)for(int meta:new int[]{0,1}) {
                World w=Mode_worldblocks10.fixture(true);w.rand.setSeed(42);EntityOracle.mathRandom().setSeed(123);OracleHooks.reset(100);WorldEventOracle.attach(w);
                Mode_worldblocks10.raw(w,8,63,8,floor,0);Mode_worldblocks10.raw(w,8,64,8,id,meta);
                Trace10.writeBoolean(o, "Block.blocksList[id].canPlaceBlockAt(w,8,64,8)", Block.blocksList[id].canPlaceBlockAt(w,8,64,8));Block.blocksList[id].onNeighborBlockChange(w,8,64,8,1);
                Trace10.writeByte(o, "w.getBlockId(8,64,8)", w.getBlockId(8,64,8));EntityOracle.i(o,Trace10.namedInt("w.getBlockMetadata(8,64,8)", w.getBlockMetadata(8,64,8)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
