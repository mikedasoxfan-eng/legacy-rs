package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbspawner8 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;BlockMobSpawner b=(BlockMobSpawner)Block.mobSpawner;Random r=new Random(91);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            o.writeBoolean(b.isOpaqueCube());
            for(int m:new int[]{Integer.MIN_VALUE,-1,0,1,2,15,16,Integer.MAX_VALUE})for(int n=0;n<32;n++) {
                EntityOracle.i(o,b.idDropped(m,r));EntityOracle.i(o,b.quantityDropped(r));EntityOracle.l(o,EntityOracle.rngState(r));
                TileEntityMobSpawner t=(TileEntityMobSpawner)b.getBlockEntity();EntityOracle.i(o,t.delay);byte[] mob=t.getMobID().getBytes("UTF-8");EntityOracle.i(o,mob.length);o.write(mob);EntityOracle.d(o,t.yaw);EntityOracle.d(o,t.yaw2);
                o.writeBoolean(t!=b.getBlockEntity());
            }
        }
    }
}
