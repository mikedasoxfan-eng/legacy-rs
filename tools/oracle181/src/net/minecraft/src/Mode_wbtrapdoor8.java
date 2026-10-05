package net.minecraft.src;
import java.io.*;
/** Every remaining BlockTrapDoor override calls original bytecode. */
public final class Mode_wbtrapdoor8 {
    static void bounds(DataOutputStream o,Block b)throws Exception {
        Mode_worldblocks181.box(o,AxisAlignedBB.getBoundingBox(b.minX,b.minY,b.minZ,b.maxX,b.maxY,b.maxZ));
    }
    static void hit(DataOutputStream o,MovingObjectPosition h)throws Exception {
        o.writeBoolean(h!=null);if(h!=null){EntityOracle.i(o,h.blockX);EntityOracle.i(o,h.blockY);EntityOracle.i(o,h.blockZ);EntityOracle.i(o,h.sideHit);EntityOracle.d(o,h.hitVec.xCoord);EntityOracle.d(o,h.hitVec.yCoord);EntityOracle.d(o,h.hitVec.zCoord);}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);BlockTrapDoor b=(BlockTrapDoor)Block.trapdoor;EntityPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            b.setBlockBoundsForItemRender();bounds(o,b);
            for(int target:new int[]{0,1,96})for(int support=0;support<=109;support++)for(int side:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,5,6,Integer.MAX_VALUE}) {
                for(int[] d:new int[][]{{-1,0},{1,0},{0,-1},{0,1}})Mode_worldblocks181.raw(w,8+d[0],64,8+d[1],support,0);
                Mode_worldblocks181.raw(w,8,64,8,target,0);o.writeBoolean(b.canPlaceBlockOnSide(w,8,64,8,side));
            }
            for(int[] d:new int[][]{{-1,0},{1,0},{0,-1},{0,1}})Mode_worldblocks181.raw(w,8+d[0],64,8+d[1],0,0);
            for(int m:new int[]{Integer.MIN_VALUE,-17,-16,-9,-8,-1,0,1,2,3,4,5,6,7,8,15,16,31,Integer.MAX_VALUE}) {b.setBlockBoundsForBlockRender(m);bounds(o,b);o.writeBoolean(BlockTrapDoor.isTrapdoorOpen(m));}
            for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int hook=0;hook<3;hook++)for(int side:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,5,6,Integer.MAX_VALUE}) {
                Mode_worldblocks181.raw(w,8,64,8,96,m);w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                if(hook==0)b.onBlockPlaced(w,8,64,8,side);else if(hook==1){o.writeBoolean(b.blockActivated(w,8,64,8,p));}else b.onBlockClicked(w,8,64,8,p);
                Mode_wbtrapdoors6.result(o,w);
            }
            EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;
            Chunk far=new Chunk(w,new byte[32768],16777216>>4,0);far.isChunkLoaded=true;provider.chunks.put(provider.key(16777216>>4,0),far);
            for(int x:new int[]{8,-17,16777216})for(int m=0;m<16;m++) {
                Mode_worldblocks181.raw(w,x,64,8,96,m);b.setBlockBoundsForItemRender();
                Mode_worldblocks181.box(o,b.getSelectedBoundingBoxFromPool(w,x,64,8));bounds(o,b);
                Mode_worldblocks181.box(o,b.getCollisionBoundingBoxFromPool(w,x,64,8));bounds(o,b);
                for(int axis=0;axis<3;axis++)for(double offset:new double[]{0,0.0625,0.1875,0.5,0.8125,0.9375,1})for(boolean reverse:new boolean[]{false,true}) {
                    double[] s={x+offset,64+offset,8+offset},e=s.clone();s[axis]=new double[]{x,64,8}[axis]-1;e[axis]=new double[]{x,64,8}[axis]+2;
                    Vec3D start=Vec3D.createVector(s[0],s[1],s[2]),end=Vec3D.createVector(e[0],e[1],e[2]);hit(o,b.collisionRayTrace(w,x,64,8,reverse?end:start,reverse?start:end));bounds(o,b);
                }
            }
        }
    }
}
