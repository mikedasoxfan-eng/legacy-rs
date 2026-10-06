package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_wbfoodstats {
    static void state(DataOutputStream o,FoodStats f)throws Exception {NBTTagCompound n=new NBTTagCompound();f.writeStatsToNBT(n);EntityOracle.i(o,Trace10.namedInt("n.getInteger(\"foodLevel\")", n.getInteger("foodLevel")));EntityOracle.i(o,Trace10.namedInt("n.getInteger(\"foodTickTimer\")", n.getInteger("foodTickTimer")));EntityOracle.i(o,Trace10.namedInt("f.getPrevFoodLevel()", f.getPrevFoodLevel()));EntityOracle.f(o,Trace10.namedFloat("n.getFloat(\"foodSaturationLevel\")", n.getFloat("foodSaturationLevel")));EntityOracle.f(o,Trace10.namedFloat("n.getFloat(\"foodExhaustionLevel\")", n.getFloat("foodExhaustionLevel")));}
    public static void main(String[] a)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            Random r=new Random(123);for(int k=0;k<4096;k++){FoodStats f=new FoodStats();NBTTagCompound n=new NBTTagCompound();int level=r.nextInt(),timer=r.nextInt();float sat=Float.intBitsToFloat(r.nextInt()),ex=Float.intBitsToFloat(r.nextInt());if((k&7)!=0){n.setInteger("foodLevel",level);n.setInteger("foodTickTimer",timer);n.setFloat("foodSaturationLevel",sat);n.setFloat("foodExhaustionLevel",ex);}f.readStatsFromNBT(n);state(o,f);for(int op=0;op<4;op++){int amount=r.nextInt();float factor=Float.intBitsToFloat(r.nextInt()),exhaust=Float.intBitsToFloat(r.nextInt());f.addStats(amount,factor);f.addExhaustion(exhaust);state(o,f);}}
        }
    }
}
