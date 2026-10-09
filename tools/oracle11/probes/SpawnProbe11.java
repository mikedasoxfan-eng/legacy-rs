package net.minecraft.src;
import java.io.*;
/** Original spawn methods with a controlled uncovered-block seam and ordered query trace. */
public final class SpawnProbe11 {
    static final class SurfaceWorld extends World {
        int shape,calls;DataOutputStream trace;
        SurfaceWorld(final long seed,final EnumWorldType type,int dim) {
            super(new Oracle.NullSaveHandler(seed){public WorldInfo loadWorldInfo(){return new WorldInfo(new WorldSettings(seed,0,false,false,type),"oracle");}},"oracle",new WorldSettings(seed,0,false,false,type),WorldProvider.getProviderForDimension(dim));
        }
        SurfaceWorld(ISaveHandler save,WorldSettings settings) {super(save,"saved",settings);}
        public int getFirstUncoveredBlock(int x,int z) {
            int block=shape==0?2:shape==1?(calls<6?0:2):shape==3?1:0;
            calls++;
            try {trace.writeInt(x);trace.writeInt(z);trace.writeInt(block);trace.writeBoolean(findingSpawnPoint);EntityOracle.l(trace,EntityOracle.rngState(rand));}catch(Exception e){throw new RuntimeException(e);}
            return block;
        }
    }
    static void snapshot(DataOutputStream o,SurfaceWorld w,ByteArrayOutputStream trace)throws Exception {
        o.writeInt(w.calls);o.writeInt(trace.size());o.write(trace.toByteArray());
        WorldInfo i=w.getWorldInfo();o.writeInt(i.getSpawnX());o.writeInt(i.getSpawnY());o.writeInt(i.getSpawnZ());
        o.writeBoolean(w.findingSpawnPoint);EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);
    }
    public static void main(String[] a)throws Exception {
        long seed=Long.parseLong(a[0]);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int dim:new int[]{0,-1,1})for(EnumWorldType type:EnumWorldType.values())for(int shape=0;shape<4;shape++) {
                SurfaceWorld w=new SurfaceWorld(seed,type,dim);w.shape=shape;w.rand.setSeed(seed);WorldEventOracle.attach(w);
                ByteArrayOutputStream b=new ByteArrayOutputStream();w.trace=new DataOutputStream(b);
                w.getInitialSpawnLocation();snapshot(o,w,b);
            }
            for(int dim:new int[]{0,-1,1})for(EnumWorldType type:EnumWorldType.values())for(int shape=0;shape<4;shape++)for(int[] spawn:new int[][]{{-17,-1,31},{5,0,-9},{0,4,0},{Integer.MAX_VALUE,128,Integer.MIN_VALUE}}) {
                SurfaceWorld w=new SurfaceWorld(seed,type,dim);w.shape=shape;w.rand.setSeed(seed);WorldEventOracle.attach(w);
                w.getWorldInfo().setSpawn(spawn[0],spawn[1],spawn[2]);
                ByteArrayOutputStream b=new ByteArrayOutputStream();w.trace=new DataOutputStream(b);
                w.setSpawnLocation();snapshot(o,w,b);
            }
            for(int dim:new int[]{0,-1,1})for(EnumWorldType type:EnumWorldType.values()) {
                SaveHandler save=new SaveHandler(new File(a[2]),"saved-"+dim+"-"+type.name(),false);
                WorldInfo saved=new WorldInfo(new WorldSettings(seed,1,false,true,type),"saved");saved.setSpawn(-17,-1,31);
                NBTTagCompound p=new NBTTagCompound();p.setInteger("Dimension",dim);saved.setPlayerNBTTagCompound(p);save.saveWorldInfo(saved);
                SurfaceWorld w=new SurfaceWorld(save,new WorldSettings(seed^17L,0,true,false,type==EnumWorldType.DEFAULT?EnumWorldType.FLAT:EnumWorldType.DEFAULT));
                WorldInfo i=w.getWorldInfo();o.writeBoolean(w.isNewWorld);o.writeInt(w.worldProvider.worldType);o.writeLong(i.getRandomSeed());o.writeInt(i.getGameType());o.writeBoolean(i.isMapFeaturesEnabled());o.writeBoolean(i.isHardcoreModeEnabled());o.writeBoolean(i.func_46133_t()==EnumWorldType.FLAT);o.writeInt(i.getSpawnX());o.writeInt(i.getSpawnY());o.writeInt(i.getSpawnZ());o.writeInt(w.calls);
                w.shape=1;w.rand.setSeed(seed);WorldEventOracle.attach(w);ByteArrayOutputStream b=new ByteArrayOutputStream();w.trace=new DataOutputStream(b);
                w.setSpawnLocation();snapshot(o,w,b);
            }
            for(int dim:new int[]{0,-1,1})for(final EnumWorldType type:EnumWorldType.values())for(int shape=0;shape<4;shape++) {
                final WorldSettings settings=new WorldSettings(seed,0,false,false,type);
                World w=new World(new Oracle.NullSaveHandler(seed){public WorldInfo loadWorldInfo(){return new WorldInfo(settings,"oracle");}},"oracle",settings,WorldProvider.getProviderForDimension(dim));
                EntityOracle.FlatProvider provider=new EntityOracle.FlatProvider(w);w.chunkProvider=provider;
                for(Chunk c:provider.chunks.values())for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<128;y++)
                    c.blocks[(x<<11)|(z<<7)|y]=(byte)(shape==0&&y==3?2:shape==1&&y==63?2:shape==2&&y==63?1:shape==2&&y>63&&y<=67?9:shape==3?1:0);
                w.rand.setSeed(seed);WorldEventOracle.attach(w);
                for(int[] q:new int[][]{{0,0},{-17,31},{47,-48},{-48,47}})o.writeInt(w.getFirstUncoveredBlock(q[0],q[1]));
                EntityOracle.l(o,EntityOracle.rngState(w.rand));WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
