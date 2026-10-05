package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbgrowth4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{2,18,31,32,37,38,78,79,80,81,83})for(int config=0;config<16;config++) {
                World w=Mode_worldblocks181.fixture((config&2)==0);w.rand.setSeed(901+config);EntityOracle.mathRandom().setSeed(123);OracleHooks.reset(100);
                Mode_worldblocks181.raw(w,8,63,8,new int[]{1,2,3,12,60,0,79,81,83,1,2,3,12,60,0,1}[config],0);Mode_worldblocks181.raw(w,8,64,8,id,config);
                if((config&4)!=0)Mode_worldblocks181.raw(w,7,63,8,9,0);if((config&8)!=0)Mode_worldblocks181.raw(w,9,64,8,17,0);
                if((config&1)!=0)Arrays.fill(w.getChunkFromChunkCoords(0,0).blocklightMap.data,(byte)255);
                Block.blocksList[id].onNeighborBlockChange(w,8,64,8,1);
                for(int t=0;t<48;t++) {
                    if(w.getBlockId(8,64,8)==id)Block.blocksList[id].updateTick(w,8,64,8,w.rand);
                    Mode_wbplacement4.cells(o,w,5,11,61,68);EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.i(o,w.loadedEntityList.size());
                }
            }
        }
    }
}
