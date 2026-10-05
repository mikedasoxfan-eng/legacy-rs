package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
/** Original MCHash duplicate insertion, rehashing, null values and live key set. */
public final class Mode_wbmchash5 {
    static Field slots,count,threshold,version;
    static void value(DataOutputStream o,Object v)throws Exception{o.writeBoolean(v!=null);if(v!=null)EntityOracle.i(o,(Integer)v);}
    static void state(DataOutputStream o,MCHash map)throws Exception {
        MCHashEntry[] table=(MCHashEntry[])slots.get(map);EntityOracle.i(o,table.length);EntityOracle.i(o,count.getInt(map));EntityOracle.i(o,threshold.getInt(map));EntityOracle.i(o,version.getInt(map));
        for(MCHashEntry first:table) {int n=0;for(MCHashEntry e=first;e!=null;e=e.nextEntry)n++;EntityOracle.i(o,n);for(MCHashEntry e=first;e!=null;e=e.nextEntry){EntityOracle.i(o,e.hashEntry);EntityOracle.i(o,e.slotHash);value(o,e.valueEntry);}}
        Set keys=map.func_35860_b();EntityOracle.i(o,keys.size());for(Object key:keys)EntityOracle.i(o,(Integer)key);
        for(int key:new int[]{0,1,-1,Integer.MIN_VALUE,Integer.MAX_VALUE,16,64,128}) {o.writeBoolean(map.func_35858_b(key));value(o,map.lookup(key));}
    }
    public static void main(String[] a)throws Exception {
        slots=MCHash.class.getDeclaredField("slots");count=MCHash.class.getDeclaredField("count");threshold=MCHash.class.getDeclaredField("threshold");version=MCHash.class.getDeclaredField("versionStamp");for(Field f:new Field[]{slots,count,threshold,version})f.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            MCHash map=new MCHash();Random rng=new Random(42);
            for(int t=0;t<1024;t++) {
                int key=t<32?t%8:t<64?(t-32)*16:t<96?new int[]{Integer.MIN_VALUE,Integer.MAX_VALUE,-1,0}[t%4]:rng.nextInt(257)-128;
                EntityOracle.i(o,new MCHashEntry(0,key,null,null).hashCode());
                int action=t<128?0:rng.nextInt(8);
                if(action<4)map.addKey(key,t%7==0?null:Integer.valueOf(t));
                else if(action==4)value(o,map.removeObject(key));
                else if(action==5){MCHashEntry e=map.removeEntry(key);value(o,e==null?null:e.valueEntry);}
                else if(action==6){map.func_35860_b().add(key);}
                else {map.func_35860_b().remove(key);}
                if(t==250||t==750)map.clearMap();if(t==500)version.setInt(map,Integer.MAX_VALUE);
                state(o,map);
            }
        }
    }
}
