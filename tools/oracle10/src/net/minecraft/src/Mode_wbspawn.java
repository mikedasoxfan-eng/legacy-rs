package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;
public final class Mode_wbspawn {
    static class Grid extends GenLayer {
        int pattern;Grid(int p){super(0L);pattern=p;}
        public int[] func_35500_a(int x,int z,int w,int h){int[] a=new int[w*h];for(int k=0;k<a.length;k++)a[k]=pattern==0?0:pattern==1?4:((x+k%w)*31+(z+k/w)*17)&7;return a;}
    }
    static WorldChunkManager manager(int p)throws Exception {WorldChunkManager m=new WorldChunkManager();Field f=WorldChunkManager.class.getDeclaredField("field_34903_b");f.setAccessible(true);f.set(m,new Grid(p));return m;}
    static class Provider extends WorldProvider {int mask;public boolean canCoordinateBeSpawn(int x,int z){return mask!=-1&&((x*3+z*7)&mask)==0;}}
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int p=0;p<3;p++)for(long seed=0;seed<32;seed++)for(int radius:new int[]{0,1,4,17,256}) {
                Random r=new Random(seed);ChunkPosition q=manager(p).func_35556_a(-17,8,radius,Arrays.asList(BiomeGenBase.forest,BiomeGenBase.swampland,BiomeGenBase.taiga),r);
                Trace10.writeBoolean(o, "q!=null", q!=null);if(q!=null){EntityOracle.i(o,Trace10.namedInt("q.x", q.x));EntityOracle.i(o,Trace10.namedInt("q.z", q.z));}EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(r)", EntityOracle.rngState(r)));
            }
            for(int p=0;p<3;p++)for(long seed=0;seed<16;seed++)for(int mask:new int[]{0,15,255,-1}) {
                World w=Mode_worldblocks10.fixture(true);NBTTagCompound n=new NBTTagCompound();n.setLong("RandomSeed",seed);w.worldInfo=new WorldInfo(n);
                Provider provider=new Provider();provider.mask=mask;provider.worldObj=w;provider.worldChunkMgr=manager(p);Field pf=World.class.getDeclaredField("worldProvider");pf.setAccessible(true);pf.set(w,provider);w.rand.setSeed(123);
                w.getInitialSpawnLocation();ChunkCoordinates q=w.getSpawnPoint();EntityOracle.i(o,Trace10.namedInt("q.posX", q.posX));EntityOracle.i(o,Trace10.namedInt("q.posY", q.posY));EntityOracle.i(o,Trace10.namedInt("q.posZ", q.posZ));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));Trace10.writeBoolean(o, "w.findingSpawnPoint", w.findingSpawnPoint);
            }
            World w=Mode_worldblocks10.fixture(true);WorldProvider[] providers={new WorldProviderSurface(),new WorldProviderHell(),new WorldProviderEnd()};
            for(WorldProvider p:providers){p.worldObj=w;for(int id=0;id<110;id++){Mode_worldblocks10.raw(w,8,63,8,id,0);Trace10.writeBoolean(o, "p.canCoordinateBeSpawn(8,8)", p.canCoordinateBeSpawn(8,8));}}
        }
    }
}
