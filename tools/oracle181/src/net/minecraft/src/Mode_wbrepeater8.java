package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Complete direct callbacks of both original repeater instances. */
public final class Mode_wbrepeater8 {
    static void queue(DataOutputStream o,World w)throws Exception {
        TreeSet<NextTickListEntry> set=(TreeSet<NextTickListEntry>)Mode_wbnextticks.field("scheduledTickTreeSet").get(w);
        EntityOracle.i(o,set.size());for(NextTickListEntry t:set){for(int v:new int[]{t.xCoord,t.yCoord,t.zCoord,t.blockID})EntityOracle.i(o,v);EntityOracle.l(o,t.scheduledTime);}
    }
    static void clear(World w)throws Exception {
        ((Set)Mode_wbnextticks.field("scheduledTickTreeSet").get(w)).clear();((Set)Mode_wbnextticks.field("scheduledTickSet").get(w)).clear();
        w.loadedEntityList.clear();for(int x=7;x<=9;x++)for(int y=63;y<=65;y++)for(int z=7;z<=9;z++)Mode_worldblocks181.raw(w,x,y,z,0,0);
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);EntityPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{93,94})for(int floor=0;floor<=109;floor++)for(int target:new int[]{0,1,31,78,93}) {
                Mode_worldblocks181.raw(w,8,63,8,floor,0);Mode_worldblocks181.raw(w,8,64,8,target,0);Block b=Block.blocksList[id];o.writeBoolean(b.canPlaceBlockAt(w,8,64,8));o.writeBoolean(b.canBlockStay(w,8,64,8));
            }
            for(int id:new int[]{93,94})for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int config=0;config<8;config++)for(int hook=0;hook<6;hook++) {
                clear(w);Mode_worldblocks181.raw(w,8,63,8,(config&1)==0?0:1,0);Mode_worldblocks181.raw(w,8,64,8,id,m);
                int[][] input={{0,1},{-1,0},{0,-1},{1,0}};int[] d=input[m&3];Mode_worldblocks181.raw(w,8+d[0],64,8+d[1],new int[]{0,55,76,94}[config>>1],(config>>1)==1?15:(config>>1)==2?5:(m+2)&3);
                w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);Block b=Block.blocksList[id];
                if(hook==0)b.onBlockAdded(w,8,64,8);else if(hook==1)b.onNeighborBlockChange(w,8,64,8,1);else if(hook==2)b.updateTick(w,8,64,8,w.rand);else if(hook==3)o.writeBoolean(b.blockActivated(w,8,64,8,p));else if(hook==4){p.rotationYaw=m*22.5F-180;b.onBlockPlacedBy(w,8,64,8,p);}else b.onBlockRemoval(w,8,64,8);
                for(int side:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,5,6,Integer.MAX_VALUE}){o.writeBoolean(b.isPoweringTo(w,8,64,8,side));o.writeBoolean(b.isIndirectlyPoweringTo(w,8,64,8,side));}
                Mode_wbtrapdoors6.result(o,w);queue(o,w);
            }
            for(int id:new int[]{93,94})for(float yaw:new float[]{Float.NEGATIVE_INFINITY,-Float.MAX_VALUE,-360,-45,0,45,180,360,Float.MAX_VALUE,Float.POSITIVE_INFINITY,Float.NaN}) {
                clear(w);Mode_worldblocks181.raw(w,8,63,8,1,0);Mode_worldblocks181.raw(w,8,64,8,id,15);p.rotationYaw=yaw;Mode_wbtrapdoors6.seed(w);Block.blocksList[id].onBlockPlacedBy(w,8,64,8,p);Mode_wbtrapdoors6.result(o,w);queue(o,w);
            }
            EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;Chunk far=new Chunk(w,new byte[32768],16777216>>4,0);far.isChunkLoaded=true;provider.chunks.put(provider.key(16777216>>4,0),far);
            Random r=new Random(91);
            for(int id:new int[]{93,94})for(int x:new int[]{8,-17,16777216})for(int m=0;m<16;m++)for(int n=0;n<16;n++) {
                Mode_worldblocks181.raw(w,x,64,8,id,m);Mode_wbtrapdoors6.seed(w);Block.blocksList[id].randomDisplayTick(w,x,64,8,r);EntityOracle.l(o,EntityOracle.rngState(r));EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
