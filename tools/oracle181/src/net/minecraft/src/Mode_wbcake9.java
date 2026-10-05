package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** Full BlockCake surface; the range recorder isolates explicit block callbacks.
 * Existing WorldEventOracle continues to record every sound/particle callback. */
public final class Mode_wbcake9 {
    static final ArrayList<int[]> ranges=new ArrayList<int[]>();
    static void attach(World w) {
        w.addWorldAccess((IWorldAccess)Proxy.newProxyInstance(Mode_wbcake9.class.getClassLoader(),new Class[]{IWorldAccess.class},(p,m,a)->{
            if(m.getName().equals("markBlockRangeNeedsUpdate")) {
                // World lighting emits additional ranges, independently covered by lights.
                StackTraceElement[] s=Thread.currentThread().getStackTrace();boolean direct=false;
                for(int k=0;k<s.length;k++)if(s[k].getClassName().equals("net.minecraft.src.World")&&s[k].getMethodName().equals("markBlocksDirty")) {
                    direct=k+1<s.length&&(s[k+1].getClassName().equals("net.minecraft.src.BlockCake")||s[k+1].getClassName().equals("net.minecraft.src.BlockDoor")||s[k+1].getClassName().equals("net.minecraft.src.BlockPressurePlate"));break;
                } else if(s[k].getClassName().equals("net.minecraft.src.World")&&s[k].getMethodName().equals("markBlockAsNeedsUpdate")) {direct=true;break;}
                if(direct){int[] r=new int[6];for(int k=0;k<6;k++)r[k]=(Integer)a[k];ranges.add(r);}
            }
            return null;
        }));
    }
    static void ranges(DataOutputStream o)throws Exception {EntityOracle.i(o,ranges.size());for(int[] r:ranges)for(int n:r)EntityOracle.i(o,n);ranges.clear();}
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);Block b=Block.cake;EntityPlayer p=PlayerOracle.player(w);attach(w);
        int[] values={Integer.MIN_VALUE,-100,-1,0,1,2,3,4,5,6,15,16,100,Integer.MAX_VALUE};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            b.setBlockBoundsForItemRender();Mode_wbtrapdoor8.bounds(o,b);
            for(int meta:values) {
                final int m=meta;IBlockAccess access=(IBlockAccess)Proxy.newProxyInstance(Mode_wbcake9.class.getClassLoader(),new Class[]{IBlockAccess.class},(obj,method,args)->m);
                b.setBlockBoundsBasedOnState(access,8,64,8);Mode_wbtrapdoor8.bounds(o,b);
                for(int side:values){EntityOracle.i(o,b.getBlockTextureFromSide(side));EntityOracle.i(o,b.getBlockTextureFromSideAndMetadata(side,m));}
                Random r=new Random(91);EntityOracle.i(o,b.idDropped(meta,r));EntityOracle.i(o,b.quantityDropped(r));EntityOracle.l(o,EntityOracle.rngState(r));
            }
            for(int floor=0;floor<=109;floor++)for(int target=0;target<=109;target++) {
                Mode_worldblocks181.raw(w,8,63,8,floor,0);Mode_worldblocks181.raw(w,8,64,8,target,0);o.writeBoolean(b.canPlaceBlockAt(w,8,64,8));o.writeBoolean(b.canBlockStay(w,8,64,8));
            }
            for(boolean remote:new boolean[]{false,true})for(int floor=0;floor<=109;floor++)for(int meta=0;meta<16;meta++) {
                Mode_worldblocks181.raw(w,8,63,8,floor,0);Mode_worldblocks181.raw(w,8,64,8,92,meta);w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);ranges.clear();b.onNeighborBlockChange(w,8,64,8,1);Mode_wbtrapdoors6.result(o,w);ranges(o);
            }
            for(boolean remote:new boolean[]{false,true})for(int meta=0;meta<16;meta++)for(int food:new int[]{0,19,20})for(boolean creative:new boolean[]{false,true})for(int hook=0;hook<2;hook++) {
                Mode_worldblocks181.raw(w,8,63,8,1,0);Mode_worldblocks181.raw(w,8,64,8,92,meta);w.multiplayerWorld=remote;
                p.func_35191_at().func_35764_a(food);p.func_35191_at().func_35767_b(4.5F);p.field_35212_aW.field_35759_a=creative;
                w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);ranges.clear();if(hook==0)b.onBlockClicked(w,8,64,8,p);else o.writeBoolean(b.blockActivated(w,8,64,8,p));
                EntityOracle.i(o,p.func_35191_at().func_35765_a());EntityOracle.f(o,p.func_35191_at().func_35760_d());Mode_wbtrapdoors6.result(o,w);ranges(o);
            }
            EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;
            for(int x:new int[]{8,-17,16777216,-16777217,Integer.MIN_VALUE,Integer.MAX_VALUE})for(int y:new int[]{-1,64,127,128,16777216})for(int meta=0;meta<16;meta++) {
                if(y>=0&&y<128){Chunk c=new Chunk(w,new byte[32768],x>>4,0);c.isChunkLoaded=true;provider.chunks.put(provider.key(x>>4,0),c);Mode_worldblocks181.raw(w,x,y,8,92,meta);}
                b.setBlockBoundsForItemRender();Mode_worldblocks181.box(o,b.getCollisionBoundingBoxFromPool(w,x,y,8));Mode_wbtrapdoor8.bounds(o,b);Mode_worldblocks181.box(o,b.getSelectedBoundingBoxFromPool(w,x,y,8));Mode_wbtrapdoor8.bounds(o,b);
            }
        }
    }
}
