package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbmath {
    public static void main(String[] a)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            Random r=new Random(1729);
            for(int n=0;n<20000;n++) {
                double d=Double.longBitsToDouble(r.nextLong());float f=Float.intBitsToFloat(r.nextInt());double d2=Double.longBitsToDouble(r.nextLong());
                EntityOracle.f(o,Trace10.namedFloat("MathHelper.sin(f)", MathHelper.sin(f)));EntityOracle.f(o,Trace10.namedFloat("MathHelper.cos(f)", MathHelper.cos(f)));EntityOracle.f(o,Trace10.namedFloat("MathHelper.sqrt_float(f)", MathHelper.sqrt_float(f)));EntityOracle.f(o,Trace10.namedFloat("MathHelper.sqrt_double(d)", MathHelper.sqrt_double(d)));
                EntityOracle.i(o,Trace10.namedInt("MathHelper.floor_float(f)", MathHelper.floor_float(f)));EntityOracle.i(o,Trace10.namedInt("MathHelper.floor_double(d)", MathHelper.floor_double(d)));EntityOracle.l(o,Trace10.namedLong("MathHelper.floor_double_long(d)", MathHelper.floor_double_long(d)));EntityOracle.f(o,Trace10.namedFloat("MathHelper.abs(f)", MathHelper.abs(f)));EntityOracle.d(o,Trace10.namedDouble("MathHelper.abs_max(d,d2)", MathHelper.abs_max(d,d2)));
                int i=r.nextInt(),div=r.nextInt(255)-127;if(div==0)div=1;EntityOracle.i(o,Trace10.namedInt("MathHelper.abs_int(i)", MathHelper.abs(i)));EntityOracle.i(o,Trace10.namedInt("MathHelper.bucketInt(i,div)", MathHelper.bucketInt(i,div)));
            }
            for(int i:new int[]{Integer.MIN_VALUE,-32,-1,0,1,32,Integer.MAX_VALUE})for(int b:new int[]{-32,-1,1,2,16,32,Integer.MAX_VALUE})EntityOracle.i(o,Trace10.namedInt("MathHelper.bucketInt(i,b)", MathHelper.bucketInt(i,b)));
            for(String s:new String[]{null,"","a"," ","\u0000"})Trace10.writeBoolean(o, "MathHelper.stringNullOrLengthZero(s)", MathHelper.stringNullOrLengthZero(s));
            for(int min=-20;min<=20;min++)for(int max=-20;max<=20;max++){Random rr=new Random(min*31+max);EntityOracle.i(o,Trace10.namedInt("MathHelper.getRandomIntegerInRange(rr,min,max)", MathHelper.getRandomIntegerInRange(rr,min,max)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(rr)", EntityOracle.rngState(rr)));}
        }
    }
}
