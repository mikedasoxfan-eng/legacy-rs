package net.minecraft.src;
import java.io.*;
public final class Mode_wbsides {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int fancy=0;fancy<2;fancy++) {
                ((BlockLeaves)Block.leaves).setGraphicsLevel(fancy!=0);
                for(int id=1;id<=109;id++)if(Block.blocksList[id]!=null)for(float[] bounds:new float[][]{{0,0,0,1,1,1},{.2F,.3F,.4F,.8F,.7F,.6F},{-.1F,-.1F,-.1F,1.1F,1.1F,1.1F}}) {
                    Block b=Block.blocksList[id];b.setBlockBounds(bounds[0],bounds[1],bounds[2],bounds[3],bounds[4],bounds[5]);
                    for(int neighbor=0;neighbor<=109;neighbor++) {
                        Mode_worldblocks181.raw(w,8,64,8,neighbor,0);
                        for(int side=0;side<6;side++){o.writeBoolean(b.shouldSideBeRendered(w,8,64,8,side));o.writeBoolean(b.getIsBlockSolid(w,8,64,8,side));}
                    }
                }
            }
            for(int mask=0;mask<256;mask++) {
                Mode_worldblocks181.raw(w,8,64,8,0,0);int[] dx={-1,-2,1,2,0,0,0,0},dz={0,0,0,0,-1,-2,1,2};
                for(int k=0;k<8;k++)Mode_worldblocks181.raw(w,8+dx[k],64,8+dz[k],(mask&(1<<k))!=0?90:0,0);
                for(int side=0;side<6;side++)o.writeBoolean(Block.portal.shouldSideBeRendered(w,8,64,8,side));
            }
            for(int id:new int[]{101,102})EntityOracle.i(o,((BlockPane)Block.blocksList[id]).func_35299_s());
            for(int id:new int[]{29,33})for(float[] bounds:new float[][]{{0,0,0,1,1,1},{0,0,0,.75F,1,1},{0,0,.25F,1,1,1}}) {
                Block b=Block.blocksList[id];b.setBlockBounds(bounds[0],bounds[1],bounds[2],bounds[3],bounds[4],bounds[5]);
                for(int m=0;m<16;m++){Mode_worldblocks181.raw(w,8,64,8,id,m);for(int side=0;side<6;side++)EntityOracle.i(o,b.getBlockTexture(w,8,64,8,side));}
            }
        }
    }
}
