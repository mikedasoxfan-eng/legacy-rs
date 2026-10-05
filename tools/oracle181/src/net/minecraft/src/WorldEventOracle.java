package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Records original IWorldAccess calls without constructing an EffectRenderer. */
public final class WorldEventOracle {
    // Weak keys (World uses identity equality): harnesses that build thousands of worlds
    // must not keep them all alive.
    static final java.util.Map<World, ArrayList<byte[]>> events=new java.util.WeakHashMap<>();
    static void i(DataOutputStream o,int n)throws IOException {EntityOracle.i(o,n);}
    static void text(DataOutputStream o,String s)throws IOException {if(s==null){i(o,-1);return;}byte[] b=s.getBytes("UTF-8");i(o,b.length);o.write(b);}
    static void pos(DataOutputStream o,double x,double y,double z)throws IOException {EntityOracle.d(o,x);EntityOracle.d(o,y);EntityOracle.d(o,z);}
    static void cell(DataOutputStream o,int x,int y,int z)throws IOException {i(o,x);i(o,y);i(o,z);}
    public static void attach(World w) {
        if(events.containsKey(w))return;
        events.put(w,new ArrayList<byte[]>());
        w.addWorldAccess((IWorldAccess)Proxy.newProxyInstance(WorldEventOracle.class.getClassLoader(),new Class[]{IWorldAccess.class},(proxy,method,a)->{
            ByteArrayOutputStream b=new ByteArrayOutputStream();DataOutputStream o=new DataOutputStream(b);
            switch(method.getName()) {
            case "playSound": o.writeByte(0);text(o,(String)a[0]);pos(o,(Double)a[1],(Double)a[2],(Double)a[3]);EntityOracle.f(o,(Float)a[4]);EntityOracle.f(o,(Float)a[5]);break;
            case "spawnParticle": o.writeByte(1);text(o,(String)a[0]);pos(o,(Double)a[1],(Double)a[2],(Double)a[3]);pos(o,(Double)a[4],(Double)a[5],(Double)a[6]);break;
            case "playRecord": o.writeByte(2);i(o,a[0]==null?0:1);if(a[0]!=null)text(o,(String)a[0]);cell(o,(Integer)a[1],(Integer)a[2],(Integer)a[3]);break;
            case "playAuxSFX": o.writeByte(3);i(o,(Integer)a[1]);cell(o,(Integer)a[2],(Integer)a[3],(Integer)a[4]);i(o,(Integer)a[5]);break;
            default:return null;
            }
            events.get(w).add(b.toByteArray());return null;
        }));
    }
    static void direct(World w,String name,boolean positioned,double x,double y,double z,float volume,float pitch) {
        if(!events.containsKey(w))return;
        try {ByteArrayOutputStream b=new ByteArrayOutputStream();DataOutputStream o=new DataOutputStream(b);o.writeByte(4);text(o,name);o.writeBoolean(positioned);if(positioned)pos(o,x,y,z);EntityOracle.f(o,volume);EntityOracle.f(o,pitch);events.get(w).add(b.toByteArray());}catch(IOException e){throw new RuntimeException(e);}
    }
    static void hit(World w,int x,int y,int z,int side) {
        try {ByteArrayOutputStream b=new ByteArrayOutputStream();DataOutputStream o=new DataOutputStream(b);o.writeByte(5);cell(o,x,y,z);i(o,side);i(o,w.getBlockId(x,y,z));i(o,w.getBlockMetadata(x,y,z));events.get(w).add(b.toByteArray());}catch(IOException e){throw new RuntimeException(e);}
    }
    static void snapshot(World w,DataOutputStream o)throws IOException {
        ArrayList<byte[]> list=events.get(w);i(o,list==null?0:list.size());if(list!=null){for(byte[] b:list)o.write(b);list.clear();}
    }
    static class Sounds extends SoundManager {
        World world;
        public void playSound(String n,float x,float y,float z,float v,float p){direct(world,n,true,(double)x,(double)y,(double)z,v,p);}
        public void playSoundFX(String n,float v,float p){direct(world,n,false,0,0,0,v,p);}
    }
}
