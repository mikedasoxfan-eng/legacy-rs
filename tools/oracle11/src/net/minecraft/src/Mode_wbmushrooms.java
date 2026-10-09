package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbmushrooms {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{39,40})for(int config=0;config<10;config++)for(int seed=0;seed<16;seed++) {
                World w=Mode_worldblocks10.fixture(true);int y=config==7?122:config==8?123:64;
                for(int x=3;x<=13;x++)for(int z=3;z<=13;z++)Mode_worldblocks10.raw(w,x,y-1,z,config==1?1:config==2?60:config==3?0:config==4?2:3,0);
                Mode_worldblocks10.raw(w,8,y,8,id,5);
                if(config==5)Mode_worldblocks10.raw(w,9,y+1,8,1,0);
                if(config==6)for(int yy=y+1;yy<128&&yy<y+8;yy++)Mode_worldblocks10.raw(w,9,yy,8,18,0);
                w.rand.setSeed(seed);EntityPlayer p=PlayerOracle.player(w);ItemStack s=new ItemStack(351,4,15);
                Trace10.writeBoolean(o, "Item.dyePowder.onItemUse(s,p,w,8,y,8,1)", Item.dyePowder.onItemUse(s,p,w,8,y,8,1));EntityOracle.i(o,Trace10.namedInt("s.stackSize", s.stackSize));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));
                for(int x=3;x<=13;x++)for(int z=3;z<=13;z++)for(int yy=y-1;yy<=Math.min(127,y+9);yy++){Trace10.writeByte(o, "w.getBlockId(x,yy,z)", w.getBlockId(x,yy,z));Trace10.writeByte(o, "w.getBlockMetadata(x,yy,z)", w.getBlockMetadata(x,yy,z));}
            }
            for(int kind=-1;kind<=1;kind++)for(int seed=0;seed<32;seed++) {
                World w=Mode_worldblocks10.fixture(true);Random r=new Random(seed);WorldGenBigMushroom g=kind<0?new WorldGenBigMushroom():new WorldGenBigMushroom(kind);
                Trace10.writeBoolean(o, "g.generate(w,r,8,64,8)", g.generate(w,r,8,64,8));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(r)", EntityOracle.rngState(r)));
                for(int x=4;x<=12;x++)for(int z=4;z<=12;z++)for(int y=63;y<=74;y++){Trace10.writeByte(o, "w.getBlockId(x,y,z)", w.getBlockId(x,y,z));Trace10.writeByte(o, "w.getBlockMetadata(x,y,z)", w.getBlockMetadata(x,y,z));}
            }
        }
    }
}
