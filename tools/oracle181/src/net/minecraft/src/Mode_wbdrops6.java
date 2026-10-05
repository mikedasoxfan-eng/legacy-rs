package net.minecraft.src;
import java.io.*;
public final class Mode_wbdrops6 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);float[] chances={-1F,0F,.25F,.75F,1F,2F,Float.NaN,Float.POSITIVE_INFINITY};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id=1;id<=109;id++)for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int c=0;c<8;c++) {
                w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);w.rand.setSeed(c);
                Block.blocksList[id].dropBlockAsItemWithChance(w,8,64,8,m,chances[c]);Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
