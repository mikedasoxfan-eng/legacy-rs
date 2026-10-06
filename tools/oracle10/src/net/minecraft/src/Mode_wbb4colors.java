package net.minecraft.src;
import java.io.*;

/** Release Block.colorMultiplier dispatch (BlockGrass, BlockTallGrass, BlockLeaves, BlockVine, BlockFluid,
 * BlockLilyPad, BlockStem) over biome-owned grass/foliage/water callbacks, swamp tinting and per-block climate. */
public final class Mode_wbb4colors {
    static final BiomeGenBase[] BIOMES={BiomeGenBase.plains,BiomeGenBase.swampland,BiomeGenBase.forest,BiomeGenBase.desert};
    static class Climate extends WorldChunkManager {
        public BiomeGenBase getBiomeGenAt(int x,int z){return BIOMES[(x*3+z*5)&3];}
        public float func_35554_b(int x,int y,int z){return (float)((x*31+z*17+y)&15)/15.0F;}
        public float func_35558_c(int x,int z){return (float)((x*7-z*11)&15)/15.0F;}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;int[] grass=new int[65536],foliage=new int[65536];for(int k=0;k<65536;k++){grass[k]=(k*997)&0xffffff;foliage[k]=(k*311+0x102030)&0xffffff;}
        ColorizerGrass.setGrassBiomeColorizer(grass);ColorizerFoliage.getFoilageBiomeColorizer(foliage);
        World w=Mode_worldblocks10.fixture(true);w.worldProvider.worldChunkMgr=new Climate();
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{1,2,8,9,10,11,18,31,55,104,105,106,110,111,115})for(int meta=0;meta<16;meta++)for(int x:new int[]{-17,-1,0,8,17,31})for(int y:new int[]{1,64,127})for(int z:new int[]{-17,-1,0,9,30}) {
                Mode_worldblocks10.raw(w,x,y,z,id,meta);EntityOracle.i(o,Trace10.namedInt("colorMultiplier",Block.blocksList[id].colorMultiplier(w,x,y,z)));
            }
        }
    }
}
