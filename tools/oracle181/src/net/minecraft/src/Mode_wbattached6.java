package net.minecraft.src;
import java.io.*;
public final class Mode_wbattached6 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);int[][] sides={{-1,0,0},{1,0,0},{0,0,-1},{0,0,1},{0,-1,0}};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{69,77})for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int mask=0;mask<32;mask++) {
                for(int k=0;k<5;k++){int[] p=sides[k];Mode_worldblocks181.raw(w,8+p[0],64+p[1],8+p[2],(mask&(1<<k))!=0?1:0,0);}
                Mode_worldblocks181.raw(w,8,64,8,id,m);w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                Block.blocksList[id].onNeighborBlockChange(w,8,64,8,1);Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
