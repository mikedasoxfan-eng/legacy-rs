package net.minecraft.src;
import java.io.*;
// BlockVine (b1.8.1): random-tick growth (up, sideways, around corners, downward), crowding,
// and losing support, in the flat entity fixture. Usage: vine181 <out>
public final class Mode_vine181 {
    public static void main(String[] a)throws Exception {
        World w=Mode_entities181.world();
        for(int x=0;x<=6;x++)for(int y=64;y<=72;y++)EntityOracle.rawBlock(w,x,y,0,1,0);
        for(int y=64;y<=70;y++)EntityOracle.rawBlock(w,10,y,3,1,0);
        EntityOracle.rawBlock(w,1,72,1,106,4);EntityOracle.rawBlock(w,4,70,1,106,4);EntityOracle.rawBlock(w,9,68,3,106,8);
        EntityOracle.rawBlock(w,5,66,-1,106,1);EntityOracle.rawBlock(w,7,71,0,106,2);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int step=0;step<40;step++) {
                for(int rep=0;rep<20;rep++) {
                    java.util.List<int[]> vines=new java.util.ArrayList<>();
                    for(int x=-3;x<=12;x++)for(int y=60;y<=80;y++)for(int z=-3;z<=5;z++)if(w.getBlockId(x,y,z)==106)vines.add(new int[]{x,y,z});
                    for(int[] v:vines)if(w.getBlockId(v[0],v[1],v[2])==106)Block.blocksList[106].updateTick(w,v[0],v[1],v[2],w.rand);
                }
                if(step==20)w.setBlockWithNotify(3,70,0,0);
                if(step==30)w.setBlockWithNotify(10,68,3,0);
                if(step==35)w.setBlockWithNotify(2,72,0,0);
                for(int x=-3;x<=12;x++)for(int y=60;y<=80;y++)for(int z=-3;z<=5;z++){o.writeByte(w.getBlockId(x,y,z));o.writeByte(w.getBlockMetadata(x,y,z));}
                EntityOracle.l(o,w.rand.nextLong());
            }
        }
    }
}
