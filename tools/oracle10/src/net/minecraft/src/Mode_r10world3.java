package net.minecraft.src;

import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Round 3 fixtures call the original release classes; no game methods are recompiled. */
public final class Mode_r10world3 {
    static Object field(Object obj,String name)throws Exception {return Mode_structures10.field(obj,name);}
    static void i(DataOutputStream o,int n)throws Exception {Trace10.writeInt(o,"r10world3 integer",n);}
    static void l(DataOutputStream o,long n)throws Exception {Trace10.writeLong(o,"r10world3 long",n);}
    static void b(DataOutputStream o,boolean n)throws Exception {Trace10.writeBoolean(o,"r10world3 boolean",n);}
    static void value(DataOutputStream o,Object v)throws Exception {b(o,v!=null);if(v!=null)i(o,(Integer)v);}
    static World flat(int dimension)throws Exception {
        World w=new World(new Oracle.NullSaveHandler(0),"oracle",Oracle.settings(0),WorldProvider.getProviderForDimension(dimension));
        w.chunkProvider=new EntityOracle.FlatProvider(w);w.rand.setSeed(300);
        for(int x=-3;x<=3;x++)for(int z=-3;z<=3;z++) {
            Chunk c=w.getChunkFromChunkCoords(x,z);Arrays.fill(c.blocks,(byte)0);
            Arrays.fill(c.data.data,(byte)0);Arrays.fill(c.skylightMap.data,(byte)0);
            Arrays.fill(c.blocklightMap.data,(byte)0);Arrays.fill(c.heightMap,(byte)0);
        }
        return w;
    }
    public static void main(String[] a)throws Exception {
        String kind=a[1];long seed=Long.parseLong(a[2]);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            if(kind.equals("worldheight"))worldHeight(o);
            else if(kind.equals("intmap"))intMap(o,seed);
            else if(kind.equals("longmap"))longMap(o,seed);
            else if(kind.equals("profiler"))profiler(o);
            else if(kind.equals("endterrain"))endTerrain(o,seed);
            else if(kind.equals("endpopulation")||kind.equals("spikes"))endPopulation(o,seed,kind);
            else if(kind.equals("endprovider"))endProvider(o);
            else throw new IllegalArgumentException(kind);
        }
    }
    static void worldHeight(DataOutputStream o)throws Exception {
        for(int dimension:new int[]{0,-1,1}) {
            World w=flat(dimension);w.skylightSubtracted=7;
            EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;
            for(int x:new int[]{-1875000,1874999})provider.chunks.put(provider.key(x,0),new Chunk(w,new byte[32768],x,0));
            for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<128;y++) {
                Chunk c=w.getChunkFromChunkCoords(0,0);c.blocks[(x<<11)|(z<<7)|y]=(byte)(y%7==0?1:0);
                c.data.setNibble(x,y,z,y%16);c.skylightMap.setNibble(x,y,z,y%16);c.blocklightMap.setNibble(x,y,z,(y+3)%16);
            }
            for(int height:new int[]{128,64,32,1,0,-1}) {
                w.field_35472_c=height;
                i(o,w.func_35452_b());
                ChunkCache cache=new ChunkCache(w,-16,0,-16,31,127,31);
                i(o,cache.func_35452_b());
                for(int y:new int[]{-100,-1,0,1,31,32,63,64,126,127,128,300}) {
                    i(o,w.getBlockId(8,y,8));i(o,w.getBlockMetadata(8,y,8));
                    b(o,w.blockExists(8,y,8));b(o,w.canBlockSeeTheSky(8,y,8));
                    b(o,w.checkChunksExist(0,y,0,15,y,15));
                    i(o,w.getSavedLightValue(EnumSkyBlock.Sky,8,y,8));
                    i(o,w.getSavedLightValue(EnumSkyBlock.Block,8,y,8));
                    i(o,w.getSkyBlockTypeBrightness(EnumSkyBlock.Sky,8,y,8));
                    i(o,w.getSkyBlockTypeBrightness(EnumSkyBlock.Block,8,y,8));
                    i(o,cache.getBlockId(8,y,8));i(o,cache.getBlockMetadata(8,y,8));
                    i(o,cache.getLightValue(8,y,8));
                    i(o,cache.getSkyBlockTypeBrightness(EnumSkyBlock.Sky,8,y,8));
                    i(o,cache.getSkyBlockTypeBrightness(EnumSkyBlock.Block,8,y,8));
                }
                for(int x:new int[]{-30000001,-30000000,29999999,30000000}) {
                    i(o,w.getBlockId(x,64,8));i(o,w.getSavedLightValue(EnumSkyBlock.Sky,x,64,8));
                    b(o,w.blockExists(x,128,8));
                }
            }
        }
    }
    static void intEntry(DataOutputStream o,IntHashMapEntry e)throws Exception {
        b(o,e!=null);if(e==null)return;i(o,e.getHash());value(o,e.getValue());i(o,e.slotHash);i(o,e.hashCode());
        b(o,e.equals(new IntHashMapEntry(e.slotHash,e.hashEntry,e.valueEntry,null)));
        b(o,e.equals(new IntHashMapEntry(e.slotHash,e.hashEntry^1,e.valueEntry,null)));
        o.writeUTF(e.toString());
    }
    static void intSnapshot(DataOutputStream o,IntHashMap m,int key)throws Exception {
        IntHashMapEntry[] slots=(IntHashMapEntry[])field(m,"slots");
        i(o,slots.length);i(o,(Integer)field(m,"count"));i(o,(Integer)field(m,"threshold"));i(o,(Integer)field(m,"versionStamp"));
        ArrayList<Integer> keys=new ArrayList<Integer>((Set<Integer>)field(m,"keySet"));Collections.sort(keys);
        i(o,keys.size());for(int k:keys)i(o,k);
        b(o,m.containsKey(key));value(o,m.lookup(key));intEntry(o,m.lookupEntry(key));
        for(IntHashMapEntry e:slots){int n=0;for(IntHashMapEntry p=e;p!=null;p=p.nextEntry)n++;i(o,n);for(;e!=null;e=e.nextEntry)intEntry(o,e);}
    }
    static void intMap(DataOutputStream o,long seed)throws Exception {
        Random r=new Random(seed);IntHashMap m=new IntHashMap();
        for(int n=0;n<320;n++) {
            int key=n<40?-1:n<120?n-80:r.nextInt(64)-32;
            Integer val=n%7==0?null:r.nextInt();
            if(n==240)m.clearMap();
            else if(n<160||n%4==0)m.addKey(key,val);
            else if(n%4==1)value(o,m.removeObject(key));
            else if(n%4==2)intEntry(o,m.removeEntry(key));
            intSnapshot(o,m,key);
        }
        l(o,EntityOracle.rngState(r));
    }
    static void longEntry(DataOutputStream o,LongHashMapEntry e)throws Exception {
        b(o,e!=null);if(e==null)return;l(o,e.func_35830_a());value(o,e.func_35829_b());i(o,e.field_35831_d);i(o,e.hashCode());
        b(o,e.equals(new LongHashMapEntry(e.field_35831_d,e.field_35834_a,e.field_35832_b,null)));
        b(o,e.equals(new LongHashMapEntry(e.field_35831_d,e.field_35834_a^1,e.field_35832_b,null)));
        o.writeUTF(e.toString());
    }
    static void longSnapshot(DataOutputStream o,LongHashMap m,long key)throws Exception {
        LongHashMapEntry[] slots=(LongHashMapEntry[])field(m,"playerListEntries");
        i(o,slots.length);i(o,m.getNumHashElements());i(o,(Integer)field(m,"capacity"));i(o,(Integer)field(m,"field_35581_e"));
        b(o,m.func_35575_b(key));value(o,m.getValueByKey(key));longEntry(o,m.func_35569_c(key));
        for(LongHashMapEntry e:slots){int n=0;for(LongHashMapEntry p=e;p!=null;p=p.field_35833_c)n++;i(o,n);for(;e!=null;e=e.field_35833_c)longEntry(o,e);}
    }
    static void longMap(DataOutputStream o,long seed)throws Exception {
        Random r=new Random(seed);LongHashMap m=new LongHashMap();
        for(int n=0;n<320;n++) {
            long key=n<40?Long.MIN_VALUE+123:n<120?((long)(n-80)<<32)^(n-80):r.nextInt(64)-32;
            Integer val=n%7==0?null:r.nextInt();
            if(n<160||n%4==0)m.add(key,val);
            else if(n%4==1)value(o,m.remove(key));
            else if(n%4==2)longEntry(o,m.removeKey(key));
            longSnapshot(o,m,key);
        }
        l(o,EntityOracle.rngState(r));
    }
    static void endTerrain(DataOutputStream o,long seed)throws Exception {
        World w=flat(1);ChunkProviderEnd g=new ChunkProviderEnd(w,seed);
        l(o,EntityOracle.rngState((Random)field(g,"field_40396_i")));
        for(int[] q:new int[][]{{0,0},{-1,1},{3,4},{6,0},{-8,-7},{100,100},{784425,-3},{Integer.MAX_VALUE,Integer.MIN_VALUE}}) {
            Chunk c=g.provideChunk(q[0],q[1]);o.write(c.blocks);o.write(c.heightMap);o.write(c.skylightMap.data);
            double[] density=(double[])field(g,"field_40389_n");i(o,density.length);for(double v:density)o.writeLong(Double.doubleToLongBits(v));
            l(o,EntityOracle.rngState((Random)field(g,"field_40396_i")));
        }
    }
    static void endPopulation(DataOutputStream o,long seed,String kind)throws Exception {
        for(int fixture=0;fixture<5;fixture++) {
            World w=flat(1);OracleHooks.reset(500);
            Field next=Entity.class.getDeclaredField("nextEntityID");next.setAccessible(true);next.setInt(null,100);
            for(int x=-3;x<=3;x++)for(int z=-3;z<=3;z++) {
                Chunk c=w.getChunkFromChunkCoords(x,z);
                if(fixture!=4)for(int bx=0;bx<16;bx++)for(int bz=0;bz<16;bz++)c.blocks[(bx<<11)|(bz<<7)|63]=121;
            }
            if(fixture==1)EntityOracle.rawBlock(w,8,64,8,1,0);
            if(fixture==2)EntityOracle.rawBlock(w,8,63,8,1,0);
            if(fixture==3)EntityOracle.rawBlock(w,5,63,8,0,0);
            w.rand.setSeed(seed);boolean result=true;
            if(kind.equals("spikes"))result=new WorldGenSpikes(121).generate(w,w.rand,8,64,8);
            else {ChunkProviderEnd g=new ChunkProviderEnd(w,42);g.populate(w.chunkProvider,fixture==4?1:0,0);}
            b(o,result);b(o,BlockSand.fallInstantly);l(o,EntityOracle.rngState(w.rand));l(o,OracleHooks.nextSeed());i(o,next.getInt(null));
            for(int x=-1;x<=2;x++)for(int z=-1;z<=2;z++){Chunk c=w.getChunkFromChunkCoords(x,z);o.write(c.blocks);o.write(c.data.data);}
            i(o,w.loadedEntityList.size());
            for(Object obj:w.loadedEntityList) {
                Entity e=(Entity)obj;i(o,e.entityId);o.writeUTF(EntityList.getEntityString(e));
                o.writeDouble(e.posX);o.writeDouble(e.posY);o.writeDouble(e.posZ);o.writeFloat(e.rotationYaw);
                l(o,EntityOracle.rngState(e.rand));b(o,e.addedToChunk);i(o,e.chunkCoordX);i(o,e.chunkCoordY);i(o,e.chunkCoordZ);
                Entity[] parts=e.func_40048_X();i(o,parts==null?0:parts.length);
                if(parts!=null)for(Entity p:parts){i(o,p.entityId);l(o,EntityOracle.rngState(p.rand));}
            }
        }
    }
    static void endProvider(DataOutputStream o)throws Exception {
        World w=flat(1);WorldProviderEnd p=(WorldProviderEnd)w.worldProvider;
        i(o,p.worldType);b(o,p.hasNoSky);b(o,p.isNether);b(o,p.canRespawnHere());b(o,p.func_28112_c());o.writeFloat(p.getCloudHeight());
        ChunkCoordinates s=p.func_40469_f();i(o,s.posX);i(o,s.posY);i(o,s.posZ);
        for(long t:new long[]{Long.MIN_VALUE,-24001,-1,0,23999,Long.MAX_VALUE})for(float f:new float[]{-1,0,0.25F,1,Float.NaN}) {
            o.writeFloat(p.calculateCelestialAngle(t,f));b(o,p.calcSunriseSunsetColors(f,f)==null);
            Vec3D fog=p.getFogColor(f,f);o.writeDouble(fog.xCoord);o.writeDouble(fog.yCoord);o.writeDouble(fog.zCoord);
        }
        for(int id:new int[]{0,1,2,8,9,18,49,78,121}) {EntityOracle.rawBlock(w,8,63,8,id,0);b(o,p.canCoordinateBeSpawn(8,8));}
    }
    static void profiler(DataOutputStream o)throws Exception {
        Field mapField=Profiler.class.getDeclaredField("profilingMap");mapField.setAccessible(true);
        Map<String,Long> map=(Map<String,Long>)mapField.get(null);
        String[] names={"root","root.a","root.b","root.deep.x","root..edge","root.\uD83D\uDE00","root.\uE000","zero","zero.a"};
        long[][] times={{100,20,20,9,3,2,2,0,0},{0,0,0,0,0,0,0,0,0},{Long.MAX_VALUE,Long.MAX_VALUE,-1,3,10,6,7,-1,-1},{10000001,1,2,3,4,5,6,7,8}};
        Profiler.profilingEnabled=false;b(o,Profiler.getProfilingData("root")==null);Profiler.startSection("ignored");Profiler.endSection();
        for(long[] row:times) {
            map.clear();for(int j=0;j<names.length;j++)map.put(names[j],row[j]);Profiler.profilingEnabled=true;
            for(String section:new String[]{"root","zero","missing","","root.deep"}) {
                List<ProfilerResult> result=Profiler.getProfilingData(section);i(o,result.size());
                for(ProfilerResult r:result){o.writeUTF(r.field_40703_c);o.writeLong(Double.doubleToLongBits(r.field_40704_a));o.writeLong(Double.doubleToLongBits(r.field_40702_b));i(o,r.func_40700_a());}
                for(String name:names)l(o,map.get(name));
            }
        }
        Profiler.clearProfiling();i(o,map.size());Profiler.profilingEnabled=false;
    }
}
