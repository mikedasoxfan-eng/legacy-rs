package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
/** Direct-mapped cache: identities, callback order, eviction, save throttles and progress. */
public final class Mode_chunkcache181 {
    static ArrayList<String> events=new ArrayList<String>();static int next;
    static void set(Object o,String n,Object v) throws Exception {Field f=o.getClass().getDeclaredField(n);f.setAccessible(true);f.set(o,v);}
    static class C extends Chunk {
        final int id;
        C(World w,int x,int z){super(w,new byte[32768],x,z);id=++next;}
        public void onChunkLoad(){isChunkLoaded=true;events.add("load:"+id);}
        public void onChunkUnload(){isChunkLoaded=false;events.add("unload:"+id);}
        public void func_25124_i(){events.add("translate:"+id);super.func_25124_i();}
    }
    static class G implements IChunkProvider {
        World w;G(World w){this.w=w;}
        public boolean chunkExists(int x,int z){return true;}
        public Chunk loadChunk(int x,int z){return provideChunk(x,z);}
        public boolean saveChunks(boolean b,IProgressUpdate p){return true;}
        public boolean canSave(){return true;}
        public String makeString(){return "fixture";}
        public Chunk provideChunk(int x,int z){events.add("generate:"+x+","+z);return new C(w,x,z);}
        public void populate(IChunkProvider p,int x,int z){events.add("populate:"+x+","+z);}
        public boolean unload100OldestChunks(){events.add("generator-tick");return false;}
    }
    static class L implements IChunkLoader {
        boolean fail;L(boolean f){fail=f;}
        public Chunk loadChunk(World w,int x,int z){events.add("read:"+x+","+z);if(fail && (x&3)==0)throw new IllegalStateException("fixture loader error");return null;}
        public void saveChunk(World w,Chunk c){events.add("save:"+(c instanceof C?((C)c).id:0)+":"+c.lastSaveTime);}
        public void saveExtraChunkData(World w,Chunk c){events.add("extra:"+(c instanceof C?((C)c).id:0));}
        public void saveExtraData(){events.add("flush");}
        public void func_814_a(){events.add("loader-tick");}
    }
    static void chunk(DataOutputStream out,Chunk c,Chunk blank) throws Exception {
        out.writeInt(c==null?-1:c==blank?0:((C)c).id);if(c==null)return;
        out.writeInt(c.xPosition);out.writeInt(c.zPosition);out.writeBoolean(c.isChunkLoaded);out.writeBoolean(c.isTerrainPopulated);out.writeBoolean(c.isModified);out.writeBoolean(c.neverSave);out.writeBoolean(c.hasEntities);out.writeLong(c.lastSaveTime);
    }
    static void dump(DataOutputStream out,ChunkProviderLoadOrGenerate p) throws Exception {
        Chunk blank=(Chunk)Mode_structures181.field(p,"blankChunk");out.writeInt(p.lastQueriedChunkXPos);out.writeInt(p.lastQueriedChunkZPos);chunk(out,(Chunk)Mode_structures181.field(p,"lastQueriedChunk"),blank);
        Chunk[] cs=(Chunk[])Mode_structures181.field(p,"chunks");for(int i=0;i<cs.length;i++)if(cs[i]!=null){out.writeInt(i);chunk(out,cs[i],blank);}out.writeInt(-1);
        out.writeInt(events.size());for(String e:events)out.writeUTF(e);events.clear();out.writeUTF(p.makeString());
    }
    public static void main(String[] a) throws Exception {
        String mode=a[1];World w=Oracle.newWorld(0);next=0;events.clear();Random r=new Random(Long.parseLong(a[2]));
        Field uf=sun.misc.Unsafe.class.getDeclaredField("theUnsafe");uf.setAccessible(true);ChunkProviderLoadOrGenerate p=(ChunkProviderLoadOrGenerate)((sun.misc.Unsafe)uf.get(null)).allocateInstance(ChunkProviderLoadOrGenerate.class);Chunk blank=new EmptyChunk(w,new byte[32768],0,0);
        set(p,"worldObj",w);set(p,"blankChunk",blank);set(p,"chunks",new Chunk[1024]);set(p,"chunkProvider",new G(w));set(p,"chunkLoader",mode.equals("null")?null:new L(mode.equals("failure")));w.chunkProvider=p;
        IProgressUpdate progress=new IProgressUpdate(){public void displayLoadingString(String s){}public void func_594_b(String s){}public void setLoadingProgress(int n){events.add("progress:"+n);}};
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[3])))) {
            for(int x=-6;x<=6;x++)for(int z=-6;z<=6;z++)p.provideChunk(x,z);dump(out,p);
            w.worldInfo.setWorldTime(1000);Chunk[] cs=(Chunk[])Mode_structures181.field(p,"chunks");for(int i=0;i<cs.length;i++)if(cs[i]!=null && cs[i]!=blank){cs[i].hasEntities=r.nextBoolean();cs[i].neverSave=r.nextInt(9)==0;cs[i].isModified=r.nextBoolean();}
            for(int i=0;i<4;i++){out.writeBoolean(p.saveChunks(false,progress));dump(out,p);}out.writeBoolean(p.saveChunks(true,progress));dump(out,p);
            p.setCurrentChunkOver(64,-64);chunk(out,p.provideChunk(6,6),blank);out.writeBoolean(p.chunkExists(6,6));dump(out,p);
            for(int x=58;x<=70;x++)for(int z=-70;z<=-58;z++)p.loadChunk(x,z);dump(out,p);
            w.findingSpawnPoint=true;for(int x=96;x<=99;x++)for(int z=96;z<=99;z++)p.provideChunk(x,z);w.findingSpawnPoint=false;dump(out,p);
            p.setCurrentChunkOver(-96,-96);for(int x=-98;x<=-94;x++)for(int z=-98;z<=-94;z++)p.provideChunk(x,z);dump(out,p);
            for(int[] q:new int[][]{{-111,-111},{-112,-112},{-81,-81},{-80,-80},{Integer.MAX_VALUE,Integer.MIN_VALUE}}){out.writeBoolean(p.canChunkExist(q[0],q[1]));out.writeBoolean(p.chunkExists(q[0],q[1]));chunk(out,p.provideChunk(q[0],q[1]),blank);}dump(out,p);
            out.writeBoolean(p.unload100OldestChunks());out.writeBoolean(p.canSave());dump(out,p);out.writeLong(EntityOracle.rngState(r));
        }
    }
}
