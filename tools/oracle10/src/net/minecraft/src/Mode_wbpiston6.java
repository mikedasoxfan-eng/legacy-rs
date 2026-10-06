package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;

public final class Mode_wbpiston6 {
    static void snapshot(DataOutputStream o,World w)throws Exception {
        Mode_wbtrapdoors6.result(o,w);
        for(int x=7;x<=9;x++)for(int y=63;y<=65;y++)for(int z=7;z<=9;z++) {
            TileEntity t=w.getBlockTileEntity(x,y,z);Trace10.writeBoolean(o, "t instanceof TileEntityPiston", t instanceof TileEntityPiston);
            if(t instanceof TileEntityPiston)Mode_wbpiston5.tile(o,(TileEntityPiston)t);
        }
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        Mode_wbpiston5.progress=TileEntityPiston.class.getDeclaredField("progress");Mode_wbpiston5.progress.setAccessible(true);
        Field guard=BlockPistonBase.class.getDeclaredField("ignoreUpdates");guard.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            World w=Mode_worldblocks10.fixture(true);EntityPlayer p=PlayerOracle.player(w);
            for(int id:new int[]{29,33})for(boolean remote:new boolean[]{false,true})for(double dx:new double[]{-3,-1.999,0,1.999,2})for(double dz:new double[]{-3,-1.999,0,1.999,2})for(double py:new double[]{61,63,64,65,67,69})for(float yaw:new float[]{-360F,-45F,0F,45F,180F,Float.POSITIVE_INFINITY,Float.NaN}) {
                Mode_worldblocks10.raw(w,8,64,8,id,7);w.multiplayerWorld=remote;p.posX=8+dx;p.posY=py;p.posZ=8+dz;p.rotationYaw=yaw;
                Mode_wbtrapdoors6.seed(w);Block.blocksList[id].onBlockPlacedBy(w,8,64,8,p);snapshot(o,w);
            }
            int[][] power={{0,-1,0},{0,1,0},{0,0,-1},{0,0,1},{1,0,0},{-1,0,0},{0,0,0},{0,2,0},{0,1,-1},{0,1,1},{-1,1,0},{1,1,0}};
            for(int id:new int[]{29,33})for(boolean remote:new boolean[]{false,true})for(boolean ignored:new boolean[]{false,true})for(int meta:new int[]{0,1,2,3,4,5,7,8,9,10,11,12,13})for(int source=-1;source<12;source++)for(int hook=0;hook<2;hook++) {
                w=Mode_worldblocks10.fixture(true);Mode_worldblocks10.raw(w,8,64,8,id,meta);
                if(source>=0){int[] d=power[source];Mode_worldblocks10.raw(w,8+d[0],64+d[1],8+d[2],76,5);}
                w.multiplayerWorld=remote;guard.setBoolean(Block.blocksList[id],ignored);Mode_wbtrapdoors6.seed(w);
                if(hook==0)Block.blocksList[id].onBlockAdded(w,8,64,8);else Block.blocksList[id].onNeighborBlockChange(w,8,64,8,76);
                snapshot(o,w);guard.setBoolean(Block.blocksList[id],false);
            }
        }
    }
}
