package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Original leaf vectors plus live singleplayer wrapper callers; no implementation copies. */
public final class FlatCallerProbe11 {
    static Field RAND,LIST,BACKEND;
    static {try {RAND=ChunkProviderFlat.class.getDeclaredField("field_46053_b");LIST=ChunkProvider.class.getDeclaredField("chunkList");BACKEND=ChunkProvider.class.getDeclaredField("chunkProvider");RAND.setAccessible(true);LIST.setAccessible(true);BACKEND.setAccessible(true);}catch(Exception e){throw new RuntimeException(e);}}
    static void text(DataOutputStream o,String s)throws Exception {byte[] b=s.getBytes("UTF-8");o.writeInt(b.length);o.write(b);}
    static void entry(DataOutputStream o,SpawnListEntry e)throws Exception {text(o,e.entityClass.getSimpleName().substring(6));o.writeInt(e.itemWeight);o.writeInt(e.field_35591_b);o.writeInt(e.field_35592_c);}
    static void state(DataOutputStream o,World w,ChunkProviderFlat flat)throws Exception {
        List chunks=(List)LIST.get(w.getIChunkProvider());o.writeInt(chunks.size());
        for(Object ob:chunks){Chunk c=(Chunk)ob;o.writeInt(c.xPosition);o.writeInt(c.zPosition);o.writeBoolean(c.isTerrainPopulated);o.writeBoolean(c.isModified);o.writeLong(c.lastSaveTime);}
        o.writeLong(EntityOracle.rngState(w.rand));o.writeLong(EntityOracle.rngState((Random)RAND.get(flat)));
    }
    public static void main(String[] a)throws Exception {
        final long seed=Long.parseLong(a[0]);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])));DataOutputStream raw=new DataOutputStream(new FileOutputStream(a[2]))) {
            for(final boolean features:new boolean[]{false,true}) {
                final WorldSettings settings=new WorldSettings(seed,0,features,false,EnumWorldType.FLAT);
                World w=new World(new Oracle.NullSaveHandler(seed){public WorldInfo loadWorldInfo(){return new WorldInfo(settings,"oracle");}},"oracle",settings);w.rand.setSeed(300L);
                IChunkProvider wrapper=w.getIChunkProvider();ChunkProviderFlat flat=(ChunkProviderFlat)BACKEND.get(wrapper);
                for(int[] q:new int[][]{{0,0},{-17,31},{Integer.MIN_VALUE,Integer.MAX_VALUE}})raw.writeBoolean(flat.chunkExists(q[0],q[1]));
                raw.writeBoolean(flat.saveChunks(false,null));raw.writeBoolean(flat.saveChunks(true,null));raw.writeBoolean(flat.unload100OldestChunks());raw.writeBoolean(flat.canSave());text(raw,flat.makeString());
                WorldChunkManager manager=w.worldProvider.worldChunkMgr;
                w.worldProvider.worldChunkMgr=null;raw.writeBoolean(flat.func_40377_a(EnumCreatureType.monster,0,0,0)==null);
                w.worldProvider.worldChunkMgr=new WorldChunkManagerHell(null,.5F,.5F);raw.writeBoolean(flat.func_40377_a(EnumCreatureType.monster,0,0,0)==null);
                w.worldProvider.worldChunkMgr=manager;raw.writeBoolean(flat.func_40377_a(null,0,0,0)==null);
                for(String name:new String[]{"Stronghold","Village","",null})raw.writeBoolean(flat.func_40376_a(w,name,0,64,0)==null);
                raw.writeLong(EntityOracle.rngState((Random)RAND.get(flat)));
                for(int[] q:new int[][]{{0,0,0},{-17,-1,31},{17,4,-19},{512,127,-1024},{Integer.MAX_VALUE,Integer.MIN_VALUE,Integer.MIN_VALUE},{Integer.MIN_VALUE,Integer.MAX_VALUE,Integer.MAX_VALUE}}) {
                    for(EnumCreatureType category:new EnumCreatureType[]{EnumCreatureType.monster,EnumCreatureType.creature,EnumCreatureType.waterCreature}) {
                        List<SpawnListEntry> entries=wrapper.func_40377_a(category,q[0],q[1],q[2]);o.writeInt(entries.size());for(SpawnListEntry e:entries)entry(o,e);
                        for(int n=0;n<8;n++){SpawnListEntry e=w.func_40474_a(category,q[0],q[1],q[2]);o.writeBoolean(e!=null);if(e!=null)entry(o,e);o.writeLong(EntityOracle.rngState(w.rand));}
                    }
                    o.writeBoolean(w.func_40474_a(null,q[0],q[1],q[2])==null);o.writeLong(EntityOracle.rngState(w.rand));
                    for(String name:new String[]{"Stronghold","Village",""})o.writeBoolean(wrapper.func_40376_a(w,name,q[0],q[1],q[2])!=null);
                }
                state(o,w,flat);o.writeBoolean(wrapper.saveChunks(false,null));o.writeBoolean(wrapper.saveChunks(true,null));wrapper.unload100OldestChunks();state(o,w,flat);
                for(int[] q:new int[][]{{0,0},{0,0},{17,-19},{-21,19}}) {o.writeBoolean(wrapper.chunkExists(q[0],q[1]));wrapper.loadChunk(q[0],q[1]);o.writeBoolean(wrapper.chunkExists(q[0],q[1]));state(o,w,flat);}
                o.writeBoolean(wrapper.saveChunks(false,null));state(o,w,flat);o.writeBoolean(wrapper.saveChunks(true,null));state(o,w,flat);
                for(int n=0;n<3;n++){wrapper.unload100OldestChunks();state(o,w,flat);}
            }
        }
    }
}
