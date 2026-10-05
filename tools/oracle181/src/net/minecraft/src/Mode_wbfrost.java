package net.minecraft.src;
import java.io.*;
import java.util.*;
import java.lang.reflect.*;
public final class Mode_wbfrost {
    static Field field(String name)throws Exception {Field f=World.class.getDeclaredField(name);f.setAccessible(true);return f;}
    static class OneChunk extends AbstractSet<ChunkCoordIntPair> {
        public void clear(){}public int size(){return 1;}
        public Iterator<ChunkCoordIntPair> iterator(){return Collections.singleton(new ChunkCoordIntPair(0,0)).iterator();}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;BiomeGenBase snow=new BiomeGenPlains(254);Field enabled=BiomeGenBase.class.getDeclaredField("enableSnow");enabled.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int snows=0;snows<2;snows++)for(int rain=0;rain<2;rain++)for(int pattern=0;pattern<2;pattern++)for(int light=0;light<2;light++) {
                World w=Mode_worldblocks181.fixture(true);enabled.setBoolean(snow,snows!=0);w.worldProvider.worldChunkMgr=new WorldChunkManagerHell(snow,.5F,.5F);
                field("positionsToUpdate").set(w,new OneChunk());field("soundCounter").setInt(w,100000);w.updateLCG=3457;w.rand.setSeed(1337);w.rainingStrength=rain;
                Chunk c=w.getChunkFromChunkCoords(0,0);Arrays.fill(c.blocklightMap.data,(byte)(light==0?0:255));
                for(int x=0;x<16;x++)for(int z=0;z<16;z++)Mode_worldblocks181.raw(w,x,63,z,pattern==0?9:(x+z)%2==0?1:9,0);
                for(int tick=0;tick<100;tick++) {
                    w.updateBlocksAndPlayCaveSounds();EntityOracle.l(o,EntityOracle.rngState(w.rand));EntityOracle.i(o,w.updateLCG);EntityOracle.i(o,field("soundCounter").getInt(w));
                    for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=62;y<=65;y++) {
                        o.writeByte(w.getBlockId(x,y,z));o.writeByte(w.getBlockMetadata(x,y,z));o.writeByte(w.getSavedLightValue(EnumSkyBlock.Sky,x,y,z));o.writeByte(w.getSavedLightValue(EnumSkyBlock.Block,x,y,z));
                    }
                    for(int p:c.field_35845_c)EntityOracle.i(o,p);
                }
            }
            // All registered Beta 1.8.1 biome flags, including Taiga (no snow).
            for(int id=0;id<10;id++){BiomeGenBase b=BiomeGenBase.field_35486_a[id];o.writeBoolean(b.getEnableSnow());o.writeBoolean(b.canSpawnLightningBolt());}
        }
    }
}
