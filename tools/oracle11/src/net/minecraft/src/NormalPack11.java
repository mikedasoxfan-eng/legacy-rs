package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
/** Direct original Tessellator.setNormal calls, including arbitrary IEEE-754 inputs. */
public final class NormalPack11 {
    public static void main(String[] args)throws Exception {
        Field packed=Tessellator.class.getDeclaredField("normal");packed.setAccessible(true);
        Field enabled=Tessellator.class.getDeclaredField("hasNormals");enabled.setAccessible(true);
        Tessellator tessellator=Tessellator.instance;Random random=new Random(813114);
        float[] edges={0,-0f,1,-1,.5f,-.5f,Float.NaN,Float.POSITIVE_INFINITY,Float.NEGATIVE_INFINITY,Float.MAX_VALUE,Float.MIN_VALUE};
        Oracle.DataOut out=new Oracle.DataOut();out.i32(edges.length*edges.length*edges.length+10000);
        for(float x:edges)for(float y:edges)for(float z:edges)record(out,tessellator,packed,enabled,x,y,z);
        for(int i=0;i<10000;i++)record(out,tessellator,packed,enabled,Float.intBitsToFloat(random.nextInt()),Float.intBitsToFloat(random.nextInt()),Float.intBitsToFloat(random.nextInt()));
        try(OutputStream file=new FileOutputStream(args[0])){out.writeTo(file);}
        System.out.println("NORMAL PACK ORIGINAL: "+(edges.length*edges.length*edges.length+10000)+" vectors");
    }
    static void record(Oracle.DataOut out,Tessellator tessellator,Field packed,Field enabled,float x,float y,float z)throws Exception {
        enabled.setBoolean(tessellator,false);tessellator.setNormal(x,y,z);
        out.i32(Float.floatToRawIntBits(x));out.i32(Float.floatToRawIntBits(y));out.i32(Float.floatToRawIntBits(z));
        out.i32(packed.getInt(tessellator));out.i32(enabled.getBoolean(tessellator)?1:0);
    }
}
