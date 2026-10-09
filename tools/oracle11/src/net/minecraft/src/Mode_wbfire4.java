package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** Original fire ticks around the new vine fuel, retaining age, scheduling and RNG. */
public final class Mode_wbfire4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;Field q=World.class.getDeclaredField("scheduledTickSet");q.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int config=0;config<24;config++) {
                World w=Mode_worldblocks10.fixture(true);w.rand.setSeed(777+config);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(123);
                for(int[] v:new int[][]{{-1,0},{1,0},{0,-1},{0,1}})for(int y=64;y<=67;y++)Mode_worldblocks10.raw(w,8+v[0],y,8+v[1],(config&1)==0?106:5,config&15);
                if((config&2)!=0)Mode_worldblocks10.raw(w,8,63,8,87,0);
                Mode_worldblocks10.raw(w,8,64,8,51,config%16);
                for(int t=0;t<80;t++) {
                    if(w.getBlockId(8,64,8)==51)Block.fire.updateTick(w,8,64,8,w.rand);
                    Mode_wbplacement4.cells(o,w,5,11,62,69);EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.i(o,Trace10.namedInt("((Set)q.get(w)).size()", ((Set)q.get(w)).size()));
                }
            }
        }
    }
}
