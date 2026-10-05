package net.minecraft.src;
import java.io.*;
public final class Mode_wbslabs6 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{43,44})for(int below:new int[]{0,1,43,44})for(int m=0;m<16;m++)for(int n=0;n<16;n++) {
                Mode_worldblocks181.raw(w,8,62,8,1,0);Mode_worldblocks181.raw(w,8,63,8,below,n);Mode_worldblocks181.raw(w,8,64,8,id,m);
                Mode_wbtrapdoors6.seed(w);Block.blocksList[id].onBlockAdded(w,8,64,8);Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
