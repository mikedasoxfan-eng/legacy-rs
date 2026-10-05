package net.minecraft.src;
import java.io.*;
public final class Mode_wbplacepred4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{1,6,31,32,37,38,39,40,53,54,64,67,70,71,72,81,83,86,91,98,99,100,101,102,103,104,105,106,107,108,109})for(int floor=0;floor<=109;floor++)for(int target:new int[]{0,1,8,9,10,11,51,78,106})for(int side=0;side<6;side++) {
                Mode_worldblocks181.raw(w,8,63,8,floor,0);Mode_worldblocks181.raw(w,8,64,8,target,0);o.writeBoolean(w.canBlockBePlacedAt(id,8,64,8,false,side));
            }
        }
    }
}
