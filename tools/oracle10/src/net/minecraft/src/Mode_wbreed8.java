package net.minecraft.src;
import java.io.*;
public final class Mode_wbreed8 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks10.fixture(true);Block b=Block.reed;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(boolean remote:new boolean[]{false,true})for(int floor=0;floor<=109;floor++)for(int mask=0;mask<16;mask++)for(int m:new int[]{0,15}) {
                for(int k=0;k<4;k++){int[] d=new int[][]{{-1,0},{1,0},{0,-1},{0,1}}[k];Mode_worldblocks10.raw(w,8+d[0],63,8+d[1],(mask&(1<<k))==0?0:(k&1)==0?8:9,0);}
                Mode_worldblocks10.raw(w,8,63,8,floor,0);Mode_worldblocks10.raw(w,8,64,8,83,m);w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                Trace10.writeBoolean(o, "b.canPlaceBlockAt(w,8,64,8)", b.canPlaceBlockAt(w,8,64,8));Trace10.writeBoolean(o, "b.canBlockStay(w,8,64,8)", b.canBlockStay(w,8,64,8));b.onNeighborBlockChange(w,8,64,8,1);Mode_wbtrapdoors6.result(o,w);
            }
            for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int height=1;height<=5;height++)for(int above:new int[]{0,1,83}) {
                for(int x=7;x<=9;x++)for(int y=58;y<=69;y++)for(int z=7;z<=9;z++)Mode_worldblocks10.raw(w,x,y,z,0,0);
                int bottom=65-height;Mode_worldblocks10.raw(w,8,bottom-1,8,12,0);Mode_worldblocks10.raw(w,7,bottom-1,8,9,0);
                for(int y=bottom;y<=64;y++)Mode_worldblocks10.raw(w,8,y,8,83,y==64?m:0);Mode_worldblocks10.raw(w,8,65,8,above,0);
                w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                for(int n=0;n<32;n++){b.updateTick(w,8,64,8,w.rand);Mode_wbplacement4.cells(o,w,7,9,58,69);EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));WorldEventOracle.snapshot(w,o);}
            }
        }
    }
}
