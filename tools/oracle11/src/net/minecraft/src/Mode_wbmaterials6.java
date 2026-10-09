package net.minecraft.src;
import java.io.*;
public final class Mode_wbmaterials6 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(String s:BlockStep.blockStepTypes){byte[] bytes=s.getBytes("UTF-8");EntityOracle.i(o,Trace10.namedInt("bytes.length", bytes.length));Trace10.writeBytes(o, "bytes", bytes);}
            for(int m:new int[]{Integer.MIN_VALUE,-2147483647,-65536,-257,-256,-17,-16,-1,0,1,2,3,4,5,6,15,16,17,255,256,65535,Integer.MAX_VALUE}) {
                EntityOracle.i(o,Trace10.namedInt("BlockCloth.getBlockFromDye(m)", BlockCloth.getBlockFromDye(m)));EntityOracle.i(o,Trace10.namedInt("BlockCloth.getDyeFromBlock(m)", BlockCloth.getDyeFromBlock(m)));
                for(int side:new int[]{Integer.MIN_VALUE,-1,0,1,2,3,4,5,6,Integer.MAX_VALUE})for(int id:new int[]{35,43,44})EntityOracle.i(o,Trace10.namedInt("Block.blocksList[id].getBlockTextureFromSideAndMetadata(side,m)", Block.blocksList[id].getBlockTextureFromSideAndMetadata(side,m)));
            }
        }
    }
}
