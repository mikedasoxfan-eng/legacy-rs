package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Original SaveOldDir/ThreadedChunkLoader write/read and WorldInfo dimension/type copy. */
public final class DimensionStorageProbe11 {
    static final int[][] COORD={{0,0},{-17,31},{512,-1024}};
    static void metadata(DataOutputStream o,World w)throws Exception {
        WorldInfo i=w.getWorldInfo();o.writeInt(w.worldProvider.worldType);o.writeInt(i.getDimension());o.writeLong(i.getRandomSeed());o.writeBoolean(i.func_46133_t()==EnumWorldType.FLAT);o.writeInt(i.getGameType());o.writeBoolean(i.isMapFeaturesEnabled());o.writeBoolean(i.isHardcoreModeEnabled());o.writeInt(i.getSaveVersion());o.writeInt(i.getSpawnX());o.writeInt(i.getSpawnY());o.writeInt(i.getSpawnZ());
    }
    static void write(long seed,File root)throws Exception {
        for(EnumWorldType type:EnumWorldType.values())for(int savedDim:new int[]{0,-1,1}) {
            SaveOldDir save=new SaveOldDir(root,type.name()+"-"+savedDim,false);
            WorldInfo info=new WorldInfo(new WorldSettings(seed,1,false,true,type),"saved");info.setSpawn(-17,4+savedDim,31);
            NBTTagCompound p=new NBTTagCompound();p.setInteger("Dimension",savedDim);info.setPlayerNBTTagCompound(p);
            // saveWorldInfoAndPlayer would replace Player with null when list is empty; retain it via saveWorldInfo.
            info.setSaveVersion(19132);save.saveWorldInfo(info);
            for(int dim:new int[]{0,-1,1}) {
                World w=new World(save,"saved",new WorldSettings(seed^17L,0,true,false,type==EnumWorldType.FLAT?EnumWorldType.DEFAULT:EnumWorldType.FLAT),WorldProvider.getProviderForDimension(dim));
                IChunkLoader loader=save.getChunkLoader(w.worldProvider);
                for(int k=0;k<COORD.length;k++) {
                    int tag=(dim+2)*7+k;byte[] blocks=new byte[32768];
                    for(int n=0;n<blocks.length;n++)blocks[n]=(byte)(1+(n+tag)%3);
                    Chunk c=new Chunk(w,blocks,COORD[k][0],COORD[k][1]);c.isTerrainPopulated=true;
                    for(int n=0;n<c.data.data.length;n++){c.data.data[n]=(byte)(n+tag);c.skylightMap.data[n]=(byte)(n*3+tag);c.blocklightMap.data[n]=(byte)(n*5+tag);}
                    for(int n=0;n<256;n++)c.heightMap[n]=(byte)((n+tag)%128);
                    loader.saveChunk(w,c);
                }
            }
        }
        ThreadedFileIOBase.threadedIOInstance.waitForFinish();RegionFileCache.clearRegionFileReferences();
    }
    static void read(long seed,File root,DataOutputStream o)throws Exception {
        for(EnumWorldType type:EnumWorldType.values())for(int savedDim:new int[]{0,-1,1}) {
            SaveOldDir save=new SaveOldDir(root,type.name()+"-"+savedDim,false);
            World w=new World(save,"saved",new WorldSettings(seed^17L,0,true,false,type==EnumWorldType.FLAT?EnumWorldType.DEFAULT:EnumWorldType.FLAT));metadata(o,w);
            World copy=new World(w,WorldProvider.getProviderForDimension(savedDim==0?-1:0));metadata(o,copy);
            if(savedDim==-1) {
                w.rand.setSeed(seed);
                for(int[] q:new int[][]{{0,0},{-17,31},{512,-1024},{Integer.MAX_VALUE,Integer.MIN_VALUE},{Integer.MIN_VALUE,Integer.MAX_VALUE}})o.writeBoolean(w.worldProvider.canCoordinateBeSpawn(q[0],q[1]));
                o.writeLong(EntityOracle.rngState(w.rand));java.lang.reflect.Field list=ChunkProvider.class.getDeclaredField("chunkList");list.setAccessible(true);o.writeInt(((List)list.get(w.chunkProvider)).size());
            }
            for(int dim:new int[]{0,-1,1}) {
                World explicit=new World(save,"saved",new WorldSettings(seed,0,true,false,type),WorldProvider.getProviderForDimension(dim));
                IChunkLoader loader=save.getChunkLoader(explicit.worldProvider);
                for(int[] q:COORD) {
                    Chunk c=loader.loadChunk(explicit,q[0],q[1]);o.writeBoolean(c!=null);if(c==null)continue;
                    o.writeInt(c.xPosition);o.writeInt(c.zPosition);o.writeBoolean(c.isTerrainPopulated);o.write(c.blocks);o.write(c.data.data);o.write(c.skylightMap.data);o.write(c.blocklightMap.data);o.write(c.heightMap);
                }
            }
        }
        RegionFileCache.clearRegionFileReferences();
    }
    public static void main(String[] a)throws Exception {
        long seed=Long.parseLong(a[0]);File root=new File(a[2]);
        if(!a[3].equals("read"))write(seed,root);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))){read(seed,root,o);}
        System.exit(0); // Original ThreadedFileIOBase owns a non-daemon forever thread.
    }
}
