package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
public final class Mode_wbwire8 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);BlockRedstoneWire b=(BlockRedstoneWire)Block.redstoneWire;
        Field enabled=BlockRedstoneWire.class.getDeclaredField("wiresProvidePower");enabled.setAccessible(true);
        int[][] ds={{-1,0},{1,0},{0,-1},{0,1}};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int floor=0;floor<=109;floor++)for(int target:new int[]{0,1,31,55}) {Mode_worldblocks181.raw(w,8,63,8,floor,0);Mode_worldblocks181.raw(w,8,64,8,target,0);o.writeBoolean(b.canPlaceBlockAt(w,8,64,8));}
            for(int id=0;id<=109;id++)for(int m=0;m<16;m++)for(int side:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,5,6,Integer.MAX_VALUE}) {Mode_worldblocks181.raw(w,8,64,8,id,m);o.writeBoolean(BlockRedstoneWire.isPowerProviderOrWire(w,8,64,8,side));}
            for(boolean flag:new boolean[]{false,true})for(int m=0;m<16;m++)for(int roof=0;roof<2;roof++)for(int config=0;config<256;config++) {
                Mode_worldblocks181.raw(w,8,64,8,55,m);Mode_worldblocks181.raw(w,8,65,8,roof,0);
                int v=config;for(int[] d:ds){int type=v&3;v>>=2;for(int dy=-1;dy<=1;dy++)Mode_worldblocks181.raw(w,8+d[0],64+dy,8+d[1],0,0);if(type==1)Mode_worldblocks181.raw(w,8+d[0],64,8+d[1],55,0);if(type==2){Mode_worldblocks181.raw(w,8+d[0],64,8+d[1],1,0);Mode_worldblocks181.raw(w,8+d[0],65,8+d[1],55,0);}if(type==3)Mode_worldblocks181.raw(w,8+d[0],63,8+d[1],55,0);}
                enabled.setBoolean(b,flag);o.writeBoolean(b.canProvidePower());for(int side:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,5,6,Integer.MAX_VALUE}){o.writeBoolean(b.isPoweringTo(w,8,64,8,side));o.writeBoolean(b.isIndirectlyPoweringTo(w,8,64,8,side));}
            }
            enabled.setBoolean(b,true);
            for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int mask=0;mask<16;mask++)for(boolean source:new boolean[]{false,true})for(int hook=0;hook<3;hook++) {
                Mode_wbrepeater8.clear(w);for(int x=6;x<=10;x++)for(int y=61;y<=66;y++)for(int z=6;z<=10;z++)Mode_worldblocks181.raw(w,x,y,z,y==62?1:0,0);
                Mode_worldblocks181.raw(w,8,63,8,(mask==15)?0:1,0);Mode_worldblocks181.raw(w,8,64,8,55,m);
                for(int k=0;k<4;k++){int[] d=ds[k];Mode_worldblocks181.raw(w,8+d[0],63,8+d[1],1,0);boolean up=(mask&(1<<k))!=0;Mode_worldblocks181.raw(w,8+d[0],64,8+d[1],up?1:55,(m+7)&15);if(up)Mode_worldblocks181.raw(w,8+d[0],65,8+d[1],55,(m+7)&15);}
                Mode_worldblocks181.raw(w,8,63,6,1,0);Mode_worldblocks181.raw(w,8,64,6,source?76:0,5);w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);
                if(hook==0)b.onBlockAdded(w,8,64,8);else if(hook==1)b.onNeighborBlockChange(w,8,64,8,76);else b.onBlockRemoval(w,8,64,8);
                Mode_wbtrapdoors6.result(o,w);Mode_wbrepeater8.queue(o,w);
            }
            EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;Chunk far=new Chunk(w,new byte[32768],16777216>>4,0);far.isChunkLoaded=true;provider.chunks.put(provider.key(16777216>>4,0),far);Random r=new Random(91);
            for(int x:new int[]{8,-17,16777216})for(int m=0;m<16;m++)for(int n=0;n<16;n++){Mode_worldblocks181.raw(w,x,64,8,55,m);Mode_wbtrapdoors6.seed(w);b.randomDisplayTick(w,x,64,8,r);EntityOracle.l(o,EntityOracle.rngState(r));EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);}
        }
    }
}
