package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Original singleplayer provider scanning and signed-Long HashSet removals. */
public final class Mode_provider10 {
    static void dump(DataOutputStream out,ChunkProvider p) throws Exception {
        Trace10.writeInt(out, "(Integer)Mode_structures10.field(p,\"field_35392_h\")", (Integer)Mode_structures10.field(p,"field_35392_h"));List cs=(List)Mode_structures10.field(p,"chunkList");
        Trace10.writeInt(out, "cs.size()", cs.size());for(Object o:cs){Chunk c=(Chunk)o;Trace10.writeInt(out, "c.xPosition", c.xPosition);Trace10.writeInt(out, "c.zPosition", c.zPosition);Trace10.writeBoolean(out, "c.isChunkLoaded", c.isChunkLoaded);Trace10.writeBoolean(out, "c.isTerrainPopulated", c.isTerrainPopulated);Trace10.writeBoolean(out, "c.isModified", c.isModified);Trace10.writeBoolean(out, "c.neverSave", c.neverSave);Trace10.writeBoolean(out, "c.hasEntities", c.hasEntities);Trace10.writeLong(out, "c.lastSaveTime", c.lastSaveTime);}
        Set<Long> ds=(Set<Long>)Mode_structures10.field(p,"droppedChunksSet");Trace10.writeInt(out, "ds.size()", ds.size());for(long k:ds)Trace10.writeLong(out, "k", k);
        out.writeUTF(p.makeString());
    }
    public static void main(String[] a) throws Exception {
        String mode=a[1];long seed=Long.parseLong(a[2]);Random r=new Random(seed);
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            if(mode.equals("longset")) {
                HashSet<Long> s=new HashSet<Long>();ArrayList<Long> pool=new ArrayList<Long>();
                for(int i=-80;i<=80;i++)pool.add(ChunkCoordIntPair.chunkXZ2Int(i,i));
                for(int i=0;i<80;i++)pool.add(r.nextLong());
                for(int i=0;i<2400;i++){
                    long k=pool.get(i<161?i:r.nextInt(pool.size()));boolean add=i<161||r.nextBoolean();Trace10.writeBoolean(out, "add?s.add(k):s.remove(k)", add?s.add(k):s.remove(k));Trace10.writeInt(out, "s.size()", s.size());for(long v:s)Trace10.writeLong(out, "v", v);
                }
                while(!s.isEmpty()){long k=s.iterator().next();s.remove(k);Trace10.writeLong(out, "k", k);Trace10.writeInt(out, "s.size()", s.size());for(long v:s)Trace10.writeLong(out, "v", v);}Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));return;
            }
            World w=Oracle.newWorld(0);w.rand.setSeed(300);w.worldInfo.setSpawn(0,64,0);ChunkProvider p=new ChunkProvider(w,null,new EntityOracle.FlatProvider(w));w.chunkProvider=p;
            LongHashMap map=(LongHashMap)Mode_structures10.field(p,"chunkMap");List cs=(List)Mode_structures10.field(p,"chunkList");
            for(int i=-80;i<=80;i++){Chunk c=new Chunk(w,new byte[32768],i,i);c.isTerrainPopulated=true;c.isChunkLoaded=true;map.add(ChunkCoordIntPair.chunkXZ2Int(i,i),c);cs.add(c);}
            if(mode.equals("saves")) {
                for(Object o:cs){Chunk c=(Chunk)o;c.hasEntities=r.nextBoolean();c.neverSave=r.nextInt(7)==0;c.isModified=r.nextBoolean();c.lastSaveTime=r.nextInt(1000);}
                for(long t:new long[]{0,599,600,1000,Long.MAX_VALUE,Long.MIN_VALUE}){w.worldInfo.setWorldTime(t);for(int i=0;i<3;i++){Trace10.writeBoolean(out, "p.saveChunks(false,null)", p.saveChunks(false,null));dump(out,p);}Trace10.writeBoolean(out, "p.saveChunks(true,null)", p.saveChunks(true,null));dump(out,p);}Trace10.writeLong(out, "EntityOracle.rngState(r)", EntityOracle.rngState(r));return;
            }
            EntityPlayer player=new EntityPlayerSP(null,w,new Session("provider",""),0);w.playerEntities.add(player);
            for(int i=0;i<70;i++){
                player.setPosition(i<12?0:i<32?900:-900,i%3==0?400:64,i<12?0:i<32?900:-900);
                if(i==4){for(Object o:cs){Chunk c=(Chunk)o;p.dropChunk(c.xPosition,c.zPosition);}p.loadChunk(42,42);p.provideChunk(-42,-42);}
                if(i==10){w.worldInfo.setSpawn(880,64,880);}
                if(i==20){w.worldInfo.setSpawn(-880,64,-880);}
                // Out-of-range chunks return the shared EmptyChunk and stay out of the map.
                if(i==35){for(int[] q:new int[][]{{1875004,0},{-1875005,0},{0,1875004},{0,-1875005}}){Chunk c=p.loadChunk(q[0],q[1]);Trace10.writeBoolean(out, "c instanceof EmptyChunk", c instanceof EmptyChunk);Trace10.writeBoolean(out, "p.chunkExists(q[0],q[1])", p.chunkExists(q[0],q[1]));}}
                p.unload100OldestChunks();dump(out,p);
            }
        }
    }
}
