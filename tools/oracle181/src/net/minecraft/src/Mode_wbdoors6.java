package net.minecraft.src;
import java.io.*;
public final class Mode_wbdoors6 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);EntityPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{64,71})for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int config=0;config<16;config++)for(int hook=0;hook<4;hook++) {
                for(int x=7;x<=9;x++)for(int y=62;y<=66;y++)for(int z=7;z<=9;z++)Mode_worldblocks181.raw(w,x,y,z,0,0);
                Mode_worldblocks181.raw(w,8,63,8,(m&8)!=0?((config&1)!=0?id:0):((config&1)!=0?1:0),m&7);
                Mode_worldblocks181.raw(w,8,65,8,(config&2)!=0?id:0,(m&7)|8);
                Mode_worldblocks181.raw(w,7,64,8,(config&4)!=0?76:0,5);Mode_worldblocks181.raw(w,8,64,8,id,m);
                w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);BlockDoor b=(BlockDoor)Block.blocksList[id];
                if(hook==0)b.onNeighborBlockChange(w,8,64,8,(config&8)!=0?76:1);
                else if(hook==1||hook==2)b.onPoweredBlockChange(w,8,64,8,hook==2);
                else o.writeBoolean(b.blockActivated(w,8,64,8,p));
                Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
