package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_wbfoodstats {
    static void state(DataOutputStream o,FoodStats f)throws Exception {NBTTagCompound n=new NBTTagCompound();f.func_35763_b(n);EntityOracle.i(o,n.getInteger("foodLevel"));EntityOracle.i(o,n.getInteger("foodTickTimer"));EntityOracle.i(o,f.func_35769_b());EntityOracle.f(o,n.getFloat("foodSaturationLevel"));EntityOracle.f(o,n.getFloat("foodExhaustionLevel"));}
    public static void main(String[] a)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            Random r=new Random(123);for(int k=0;k<4096;k++){FoodStats f=new FoodStats();NBTTagCompound n=new NBTTagCompound();int level=r.nextInt(),timer=r.nextInt();float sat=Float.intBitsToFloat(r.nextInt()),ex=Float.intBitsToFloat(r.nextInt());if((k&7)!=0){n.setInteger("foodLevel",level);n.setInteger("foodTickTimer",timer);n.setFloat("foodSaturationLevel",sat);n.setFloat("foodExhaustionLevel",ex);}f.func_35766_a(n);state(o,f);for(int op=0;op<4;op++){int amount=r.nextInt();float factor=Float.intBitsToFloat(r.nextInt()),exhaust=Float.intBitsToFloat(r.nextInt());f.func_35771_a(amount,factor);f.func_35762_a(exhaust);state(o,f);}}
        }
    }
}
