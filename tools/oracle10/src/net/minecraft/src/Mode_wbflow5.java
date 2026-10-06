package net.minecraft.src;
import java.io.*;
public final class Mode_wbflow5 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;int[] above={0,1,79,30,106,20};int[][] offsets={{-1,0},{0,-1},{1,0},{0,1}};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int block=8;block<=11;block++){World w=Mode_worldblocks10.fixture(true);for(int meta=0;meta<16;meta++)for(int config=0;config<256;config++) {
                Mode_worldblocks10.raw(w,8,64,8,block,meta);
                for(int k=0;k<4;k++){int x=8+offsets[k][0],z=8+offsets[k][1];
                    Mode_worldblocks10.raw(w,x,64,z,(config+37*k)%110,(config+3*k)&15);
                    Mode_worldblocks10.raw(w,x,63,z,8+(config+3*k)%4,(config+5*k)&15);
                    Mode_worldblocks10.raw(w,x,65,z,above[(config/16+k)%6],0);
                }
                Vec3D v=Vec3D.createVector(0,0,0);Block.blocksList[block].velocityToAddToEntity(w,8,64,8,null,v);
                EntityOracle.d(o,Trace10.namedDouble("v.xCoord", v.xCoord));EntityOracle.d(o,Trace10.namedDouble("v.yCoord", v.yCoord));EntityOracle.d(o,Trace10.namedDouble("v.zCoord", v.zCoord));
            }}
        }
    }
}
