package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Original singleplayer provider scanning and signed-Long HashSet removals. */
public final class Mode_provider181 {
    static void dump(DataOutputStream out,ChunkProvider p) throws Exception {
        out.writeInt((Integer)Mode_structures181.field(p,"field_35392_h"));List cs=(List)Mode_structures181.field(p,"chunkList");
        out.writeInt(cs.size());for(Object o:cs){Chunk c=(Chunk)o;out.writeInt(c.xPosition);out.writeInt(c.zPosition);out.writeBoolean(c.isChunkLoaded);out.writeBoolean(c.isTerrainPopulated);out.writeBoolean(c.isModified);out.writeBoolean(c.neverSave);out.writeBoolean(c.hasEntities);out.writeLong(c.lastSaveTime);}
        Set<Long> ds=(Set<Long>)Mode_structures181.field(p,"droppedChunksSet");out.writeInt(ds.size());for(long k:ds)out.writeLong(k);
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
                    long k=pool.get(i<161?i:r.nextInt(pool.size()));boolean add=i<161||r.nextBoolean();out.writeBoolean(add?s.add(k):s.remove(k));out.writeInt(s.size());for(long v:s)out.writeLong(v);
                }
                while(!s.isEmpty()){long k=s.iterator().next();s.remove(k);out.writeLong(k);out.writeInt(s.size());for(long v:s)out.writeLong(v);}out.writeLong(EntityOracle.rngState(r));return;
            }
            World w=Oracle.newWorld(0);w.rand.setSeed(300);w.worldInfo.setSpawn(0,64,0);ChunkProvider p=new ChunkProvider(w,null,new EntityOracle.FlatProvider(w));w.chunkProvider=p;
            PlayerList map=(PlayerList)Mode_structures181.field(p,"chunkMap");List cs=(List)Mode_structures181.field(p,"chunkList");
            for(int i=-80;i<=80;i++){Chunk c=new Chunk(w,new byte[32768],i,i);c.isTerrainPopulated=true;c.isChunkLoaded=true;map.func_35577_a(ChunkCoordIntPair.chunkXZ2Int(i,i),c);cs.add(c);}
            if(mode.equals("saves")) {
                for(Object o:cs){Chunk c=(Chunk)o;c.hasEntities=r.nextBoolean();c.neverSave=r.nextInt(7)==0;c.isModified=r.nextBoolean();c.lastSaveTime=r.nextInt(1000);}
                for(long t:new long[]{0,599,600,1000,Long.MAX_VALUE,Long.MIN_VALUE}){w.worldInfo.setWorldTime(t);for(int i=0;i<3;i++){out.writeBoolean(p.saveChunks(false,null));dump(out,p);}out.writeBoolean(p.saveChunks(true,null));dump(out,p);}out.writeLong(EntityOracle.rngState(r));return;
            }
            EntityPlayer player=new EntityPlayerSP(null,w,new Session("provider",""),0);w.playerEntities.add(player);
            for(int i=0;i<70;i++){
                player.setPosition(i<12?0:i<32?900:-900,i%3==0?400:64,i<12?0:i<32?900:-900);
                if(i==4){for(Object o:cs){Chunk c=(Chunk)o;p.func_35391_d(c.xPosition,c.zPosition);}p.loadChunk(42,42);p.provideChunk(-42,-42);}
                if(i==10){w.worldInfo.setSpawn(880,64,880);}
                if(i==20){w.worldInfo.setSpawn(-880,64,-880);}
                // Out-of-range chunks return the shared EmptyChunk and stay out of the map.
                if(i==35){for(int[] q:new int[][]{{1875004,0},{-1875005,0},{0,1875004},{0,-1875005}}){Chunk c=p.loadChunk(q[0],q[1]);out.writeBoolean(c instanceof EmptyChunk);out.writeBoolean(p.chunkExists(q[0],q[1]));}}
                p.unload100OldestChunks();dump(out,p);
            }
        }
    }
}
