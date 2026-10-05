package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbextension9 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);BlockPistonExtension b=(BlockPistonExtension)Block.pistonExtension;
        int[] values={Integer.MIN_VALUE,-100,-1,0,1,2,3,4,5,6,7,8,15,16,100,Integer.MAX_VALUE};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int override:values) {b.func_31052_a_(override);for(int m:values)for(int side:values){try {int t=b.getBlockTextureFromSideAndMetadata(side,m);o.writeBoolean(true);EntityOracle.i(o,t);}catch(ArrayIndexOutOfBoundsException e){o.writeBoolean(false);}}b.func_31051_a();for(int side=0;side<6;side++)EntityOracle.i(o,b.getBlockTextureFromSideAndMetadata(side,side));}
            for(boolean remote:new boolean[]{false,true})for(int face=0;face<6;face++)for(int base:new int[]{0,1,29,33})for(int bm=0;bm<16;bm++)for(int hook=0;hook<2;hook++) {
                for(int x=6;x<=10;x++)for(int y=61;y<=67;y++)for(int z=6;z<=10;z++)Mode_worldblocks181.raw(w,x,y,z,0,0);
                int x=8-PistonBlockTextures.offsetsXForSide[face],y=64-PistonBlockTextures.offsetsYForSide[face],z=8-PistonBlockTextures.offsetsZForSide[face];
                Mode_worldblocks181.raw(w,x,y,z,base,bm);Mode_worldblocks181.raw(w,8,64,8,34,face|8);w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                try {if(hook==0)b.onBlockRemoval(w,8,64,8);else b.onNeighborBlockChange(w,8,64,8,1);o.writeBoolean(true);}catch(ArrayIndexOutOfBoundsException e){o.writeBoolean(false);}Mode_wbtrapdoors6.result(o,w);
            }
            EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;Chunk far=new Chunk(w,new byte[32768],16777216>>4,0);far.isChunkLoaded=true;provider.chunks.put(provider.key(16777216>>4,0),far);
            for(int x:new int[]{8,-17,16777216})for(int m=0;m<16;m++)for(int shape=0;shape<2;shape++) {
                Mode_worldblocks181.raw(w,x,64,8,34,m);b.setBlockBounds(shape==0?.25F:-.25F,0,.25F,shape==0?.75F:1.25F,1,.75F);
                Mode_worldblocks181.box(o,b.getSelectedBoundingBoxFromPool(w,x,64,8));Mode_worldblocks181.box(o,b.getCollisionBoundingBoxFromPool(w,x,64,8));Mode_wbtrapdoor8.bounds(o,b);
                b.setBlockBoundsBasedOnState(w,x,64,8);Mode_wbtrapdoor8.bounds(o,b);
                for(double offset:new double[]{-0.5,0,0.25,0.5,0.75,1,1.5}) {
                    ArrayList boxes=new ArrayList();b.getCollidingBoundingBoxes(w,x,64,8,AxisAlignedBB.getBoundingBox(x+offset,64+offset,8+offset,x+offset+.25,64+offset+.25,8+offset+.25),boxes);
                    EntityOracle.i(o,boxes.size());for(Object box:boxes)Mode_worldblocks181.box(o,(AxisAlignedBB)box);Mode_wbtrapdoor8.bounds(o,b);
                }
            }
        }
    }
}
