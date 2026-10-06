package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbspawner8 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;BlockMobSpawner b=(BlockMobSpawner)Block.mobSpawner;Random r=new Random(91);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            Trace10.writeBoolean(o, "b.isOpaqueCube()", b.isOpaqueCube());
            for(int m:new int[]{Integer.MIN_VALUE,-1,0,1,2,15,16,Integer.MAX_VALUE})for(int n=0;n<32;n++) {
                EntityOracle.i(o,Trace10.namedInt("b.idDropped(m,r, 0)", b.idDropped(m,r, 0)));EntityOracle.i(o,Trace10.namedInt("b.quantityDropped(r)", b.quantityDropped(r)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(r)", EntityOracle.rngState(r)));
                TileEntityMobSpawner t=(TileEntityMobSpawner)b.getBlockEntity();EntityOracle.i(o,Trace10.namedInt("t.delay", t.delay));byte[] mob=t.getMobID().getBytes("UTF-8");EntityOracle.i(o,Trace10.namedInt("mob.length", mob.length));Trace10.writeBytes(o, "mob", mob);EntityOracle.d(o,Trace10.namedDouble("t.yaw", t.yaw));EntityOracle.d(o,Trace10.namedDouble("t.yaw2", t.yaw2));
                Trace10.writeBoolean(o, "t!=b.getBlockEntity()", t!=b.getBlockEntity());
            }
        }
    }
}
