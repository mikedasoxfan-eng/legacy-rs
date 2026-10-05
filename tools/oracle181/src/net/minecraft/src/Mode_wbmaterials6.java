package net.minecraft.src;
import java.io.*;
public final class Mode_wbmaterials6 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(String s:BlockStep.field_22037_a){byte[] bytes=s.getBytes("UTF-8");EntityOracle.i(o,bytes.length);o.write(bytes);}
            for(int m:new int[]{Integer.MIN_VALUE,-2147483647,-65536,-257,-256,-17,-16,-1,0,1,2,3,4,5,6,15,16,17,255,256,65535,Integer.MAX_VALUE}) {
                EntityOracle.i(o,BlockCloth.getBlockFromDye(m));EntityOracle.i(o,BlockCloth.getDyeFromBlock(m));
                for(int side:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,5,6,Integer.MAX_VALUE})for(int id:new int[]{35,43,44})EntityOracle.i(o,Block.blocksList[id].getBlockTextureFromSideAndMetadata(side,m));
            }
        }
    }
}
