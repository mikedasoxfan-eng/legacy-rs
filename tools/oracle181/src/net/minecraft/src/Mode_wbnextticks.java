package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;
public final class Mode_wbnextticks {
    static Field field(String name)throws Exception {Field f=World.class.getDeclaredField(name);f.setAccessible(true);return f;}
    static void queue(DataOutputStream o,World w)throws Exception {
        TreeSet<NextTickListEntry> set=(TreeSet<NextTickListEntry>)field("scheduledTickTreeSet").get(w);
        EntityOracle.i(o,set.size());
        Field id=NextTickListEntry.class.getDeclaredField("tickEntryID");id.setAccessible(true);
        for(NextTickListEntry t:set){EntityOracle.i(o,t.xCoord);EntityOracle.i(o,t.yCoord);EntityOracle.i(o,t.zCoord);EntityOracle.i(o,t.blockID);EntityOracle.l(o,t.scheduledTime);EntityOracle.l(o,id.getLong(t));}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;Random r=new Random(9831);
        Field counter=NextTickListEntry.class.getDeclaredField("nextTickEntryID");counter.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            counter.setLong(null,Long.MAX_VALUE-100);
            for(int n=0;n<10000;n++) {
                int x=r.nextInt(),y=r.nextInt(),z=r.nextInt(),block=r.nextInt();long time=r.nextLong();
                NextTickListEntry t=new NextTickListEntry(x,y,z,block);o.writeBoolean(t==t.setScheduledTime(time));
                EntityOracle.i(o,t.hashCode());o.writeBoolean(t.equals(new NextTickListEntry(x,y,z,block)));o.writeBoolean(t.equals(new NextTickListEntry(x,y,z,block^1)));o.writeBoolean(t.equals("other"));
                NextTickListEntry v=new NextTickListEntry(x,y,z,block).setScheduledTime(n%3==0?time:time^1);
                EntityOracle.i(o,t.comparer(v));EntityOracle.i(o,t.comparer(t));
            }
            for(long time:new long[]{0,4000,Long.MIN_VALUE,Long.MAX_VALUE-10}) {
                World w=Mode_worldblocks181.fixture(true);w.worldInfo.setWorldTime(time);counter.setLong(null,Long.MAX_VALUE-100);r.setSeed(33);
                for(int n=0;n<5000;n++) {
                    int x=r.nextInt(101)-50,y=r.nextInt(160)-16,z=r.nextInt(101)-50,block=new int[]{0,1,98}[r.nextInt(3)],delay=r.nextInt(120)-3;
                    w.scheduleBlockUpdate(x,y,z,block,delay);
                    if(n%10==0)w.scheduleBlockUpdate(x,y,z,block,delay+100);
                    if(n%500==499)queue(o,w);
                }
                for(int n=0;n<6;n++){o.writeBoolean(w.tickUpdates(n>=2));queue(o,w);}
            }
            counter.setLong(null,0);World wa=Mode_worldblocks181.fixture(true),wb=Mode_worldblocks181.fixture(true);
            for(int n=0;n<1000;n++) {
                World w=(n&1)==0?wa:wb;int x=8+n%32,z=8+n/32;
                w.scheduleBlockUpdate(x,64,z,1,20);if(n%3==0)w.scheduleBlockUpdate(x,64,z,1,30);
                if(n%100==99){queue(o,wa);queue(o,wb);}
            }
        }
    }
}
