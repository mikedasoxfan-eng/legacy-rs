package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbore7 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks10.fixture(true);EntityPlayer p=PlayerOracle.player(w);int[][] sides={{0,1,0},{0,-1,0},{0,0,1},{0,0,-1},{1,0,0},{-1,0,0}};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{73,74})for(boolean remote:new boolean[]{false,true})for(int m:new int[]{0,15})for(int mask=0;mask<64;mask++)for(int hook=0;hook<5;hook++) {
                for(int k=0;k<6;k++)Mode_worldblocks10.raw(w,8+sides[k][0],64+sides[k][1],8+sides[k][2],(mask&(1<<k))!=0?1:0,0);
                Mode_worldblocks10.raw(w,8,64,8,id,m);w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);w.loadedEntityList.clear();Random r=new Random(567);Block b=Block.blocksList[id];
                if(hook==0)b.onBlockClicked(w,8,64,8,p);if(hook==1)b.onEntityWalking(w,8,64,8,p);if(hook==2)Trace10.writeBoolean(o, "b.blockActivated(w,8,64,8,p)", b.blockActivated(w,8,64,8,p));if(hook==3)b.updateTick(w,8,64,8,r);if(hook==4)b.randomDisplayTick(w,8,64,8,r);
                EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(r)", EntityOracle.rngState(r)));Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
