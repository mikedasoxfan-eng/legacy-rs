package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbmath {
    public static void main(String[] a)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            Random r=new Random(1729);
            for(int n=0;n<20000;n++) {
                double d=Double.longBitsToDouble(r.nextLong());float f=Float.intBitsToFloat(r.nextInt());double d2=Double.longBitsToDouble(r.nextLong());
                EntityOracle.f(o,MathHelper.sin(f));EntityOracle.f(o,MathHelper.cos(f));EntityOracle.f(o,MathHelper.sqrt_float(f));EntityOracle.f(o,MathHelper.sqrt_double(d));
                EntityOracle.i(o,MathHelper.floor_float(f));EntityOracle.i(o,MathHelper.floor_double(d));EntityOracle.l(o,MathHelper.func_35599_c(d));EntityOracle.f(o,MathHelper.abs(f));EntityOracle.d(o,MathHelper.abs_max(d,d2));
                int i=r.nextInt(),div=r.nextInt(255)-127;if(div==0)div=1;EntityOracle.i(o,MathHelper.func_35597_a(i));EntityOracle.i(o,MathHelper.bucketInt(i,div));
            }
            for(int i:new int[]{Integer.MIN_VALUE,-32,-1,0,1,32,Integer.MAX_VALUE})for(int b:new int[]{-32,-1,1,2,16,32,Integer.MAX_VALUE})EntityOracle.i(o,MathHelper.bucketInt(i,b));
            for(String s:new String[]{null,"","a"," ","\u0000"})o.writeBoolean(MathHelper.stringNullOrLengthZero(s));
            for(int min=-20;min<=20;min++)for(int max=-20;max<=20;max++){Random rr=new Random(min*31+max);EntityOracle.i(o,MathHelper.func_35598_a(rr,min,max));EntityOracle.l(o,EntityOracle.rngState(rr));}
        }
    }
}
