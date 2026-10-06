package net.minecraft.src;
import java.io.*;
public final class Mode_wbtextures {
    static void texture(DataOutputStream o,Block b,IBlockAccess w,int side)throws Exception {
        try {EntityOracle.i(o,Trace10.namedInt("b.getBlockTexture(w,8,64,8,side)", b.getBlockTexture(w,8,64,8,side)));}catch(ArrayIndexOutOfBoundsException e){EntityOracle.i(o,Trace10.namedInt("Integer.MIN_VALUE", Integer.MIN_VALUE));}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks10.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int fancy=0;fancy<2;fancy++) {
                ((BlockLeaves)Block.leaves).setGraphicsLevel(fancy!=0);
                for(int id=1;id<=109;id++)if(Block.blocksList[id]!=null)for(int m=0;m<16;m++) {
                    Mode_worldblocks10.raw(w,8,64,8,id,m);
                    for(int above:new int[]{0,1,78,80}) {
                        Mode_worldblocks10.raw(w,8,65,8,above,0);
                        for(int side=0;side<6;side++)texture(o,Block.blocksList[id],w,side);
                    }
                }
            }
            for(int id:new int[]{54,95})for(int pair=0;pair<16;pair++)for(int mask=0;mask<256;mask++) {
                Mode_worldblocks10.raw(w,8,64,8,id,mask&15);
                int[] dx={0,0,-1,1,-1,1,-1,1},dz={-1,1,0,0,-1,-1,1,1};
                for(int k=0;k<8;k++)Mode_worldblocks10.raw(w,8+dx[k],64,8+dz[k],k<4&&(pair&(1<<k))!=0?id:(mask&(1<<k))!=0?1:0,0);
                for(int side=0;side<6;side++)texture(o,Block.blocksList[id],w,side);
            }
        }
    }
}
