package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;
/** Original MCHash duplicate insertion, rehashing, null values and live key set. */
public final class Mode_wbmchash5 {
    static Field slots,count,threshold,version;
    static void value(DataOutputStream o,Object v)throws Exception{Trace10.writeBoolean(o, "v!=null", v!=null);if(v!=null)EntityOracle.i(o,Trace10.namedInt("(Integer)v", (Integer)v));}
    static void state(DataOutputStream o,IntHashMap map)throws Exception {
        IntHashMapEntry[] table=(IntHashMapEntry[])slots.get(map);EntityOracle.i(o,Trace10.namedInt("table.length", table.length));EntityOracle.i(o,Trace10.namedInt("count.getInt(map)", count.getInt(map)));EntityOracle.i(o,Trace10.namedInt("threshold.getInt(map)", threshold.getInt(map)));EntityOracle.i(o,Trace10.namedInt("version.getInt(map)", version.getInt(map)));
        for(IntHashMapEntry first:table) {int n=0;for(IntHashMapEntry e=first;e!=null;e=e.nextEntry)n++;EntityOracle.i(o,n);for(IntHashMapEntry e=first;e!=null;e=e.nextEntry){EntityOracle.i(o,Trace10.namedInt("e.hashEntry", e.hashEntry));EntityOracle.i(o,Trace10.namedInt("e.slotHash", e.slotHash));value(o,e.valueEntry);}}
        Set keys=map.getKeySet();EntityOracle.i(o,Trace10.namedInt("keys.size()", keys.size()));for(Object key:keys)EntityOracle.i(o,Trace10.namedInt("(Integer)key", (Integer)key));
        for(int key:new int[]{0,1,-1,Integer.MIN_VALUE,Integer.MAX_VALUE,16,64,128}) {Trace10.writeBoolean(o, "map.containsKey(key)", map.containsKey(key));value(o,map.lookup(key));}
    }
    public static void main(String[] a)throws Exception {
        slots=IntHashMap.class.getDeclaredField("slots");count=IntHashMap.class.getDeclaredField("count");threshold=IntHashMap.class.getDeclaredField("threshold");version=IntHashMap.class.getDeclaredField("versionStamp");for(Field f:new Field[]{slots,count,threshold,version})f.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            IntHashMap map=new IntHashMap();Random rng=new Random(42);
            for(int t=0;t<1024;t++) {
                int key=t<32?t%8:t<64?(t-32)*16:t<96?new int[]{Integer.MIN_VALUE,Integer.MAX_VALUE,-1,0}[t%4]:rng.nextInt(257)-128;
                EntityOracle.i(o,Trace10.namedInt("new IntHashMapEntry(0,key,null,null).hashCode()", new IntHashMapEntry(0,key,null,null).hashCode()));
                int action=t<128?0:rng.nextInt(8);
                if(action<4)map.addKey(key,t%7==0?null:Integer.valueOf(t));
                else if(action==4)value(o,map.removeObject(key));
                else if(action==5){IntHashMapEntry e=map.removeEntry(key);value(o,e==null?null:e.valueEntry);}
                else if(action==6){map.getKeySet().add(key);}
                else {map.getKeySet().remove(key);}
                if(t==250||t==750)map.clearMap();if(t==500)version.setInt(map,Integer.MAX_VALUE);
                state(o,map);
            }
        }
    }
}
