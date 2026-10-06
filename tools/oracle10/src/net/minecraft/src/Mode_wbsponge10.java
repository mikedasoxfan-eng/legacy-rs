package net.minecraft.src;
import java.io.*;
/** BlockSponge (unchanged since b1.7.3): onBlockAdded reads the 5x5x5 materials, onBlockRemoval
 * notifies all 125 positions. Water and an unsupported torch inside the area show the notifications. */
public final class Mode_wbsponge10 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks10.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int layout=0;layout<4;layout++) {
                for(int x=6;x<=10;x++)for(int y=62;y<=66;y++)for(int z=6;z<=10;z++)Mode_worldblocks10.raw(w,x,y,z,y<64?1:0,0);
                if((layout&1)!=0)Mode_worldblocks10.raw(w,7,64,8,9,0);
                if((layout&2)!=0){Mode_worldblocks10.raw(w,9,64,9,0,0);Mode_worldblocks10.raw(w,9,63,9,0,0);Mode_worldblocks10.raw(w,9,64,9,50,5);}
                Mode_wbtrapdoors6.seed(w);w.loadedEntityList.clear();
                w.setBlockWithNotify(8,64,8,19);Mode_wbtrapdoors6.result(o,w);
                w.setBlockWithNotify(8,64,8,0);Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
