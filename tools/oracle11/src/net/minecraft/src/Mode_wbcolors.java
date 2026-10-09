package net.minecraft.src;
import java.io.*;
public final class Mode_wbcolors {
    static class Climate extends WorldChunkManager {
        public BiomeGenBase getBiomeGenAt(int x,int z){return BiomeGenBase.plains;}
        public float getTemperature(int x,int y,int z){return (float)((x*31+z*17)&15)/15.0F;}
        public float getRainfall(int x,int z){return (float)((x*7-z*11)&15)/15.0F;}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;int[] grass=new int[65536],foliage=new int[65536];for(int k=0;k<65536;k++){grass[k]=(k*997)&0xffffff;foliage[k]=(k*311+0x102030)&0xffffff;}ColorizerGrass.setGrassBiomeColorizer(grass);ColorizerFoliage.getFoilageBiomeColorizer(foliage);
        World w=Mode_worldblocks10.fixture(true);w.worldProvider.worldChunkMgr=new Climate();
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id=1;id<=109;id++)if(Block.blocksList[id]!=null)for(int meta=0;meta<16;meta++) {
                Block b=Block.blocksList[id];EntityOracle.i(o,Trace10.namedInt("b.getRenderColor(meta)", b.getRenderColor(meta)));
                for(int x:new int[]{-30000001,-17,-1,0,8,17,30000000})for(int z:new int[]{-17,-1,0,8,17}) {
                    final int m=meta;IBlockAccess access=(IBlockAccess)java.lang.reflect.Proxy.newProxyInstance(IBlockAccess.class.getClassLoader(),new Class[]{IBlockAccess.class},(p,method,v)->{
                        switch(method.getName()){case "getBlockMetadata":return m;case "getWorldChunkManager":return w.getWorldChunkManager();case "getBlockId":return 0;case "getBlockMaterial":return Material.air;default:throw new UnsupportedOperationException(method.getName());}
                    });
                    EntityOracle.i(o,Trace10.namedInt("b.colorMultiplier(access,x,64,z)", b.colorMultiplier(access,x,64,z)));
                }
            }
            for(int id:new int[]{104,105})for(int meta=0;meta<16;meta++)for(int mask=0;mask<16;mask++) {
                Mode_worldblocks10.raw(w,8,64,8,id,meta);int fruit=id==104?86:103;
                Mode_worldblocks10.raw(w,7,64,8,(mask&1)!=0?fruit:0,0);Mode_worldblocks10.raw(w,9,64,8,(mask&2)!=0?fruit:0,0);Mode_worldblocks10.raw(w,8,64,7,(mask&4)!=0?fruit:0,0);Mode_worldblocks10.raw(w,8,64,9,(mask&8)!=0?fruit:0,0);
                EntityOracle.i(o,Trace10.namedInt("((BlockStem)Block.blocksList[id]).func_35296_f(w,8,64,8)", ((BlockStem)Block.blocksList[id]).func_35296_f(w,8,64,8)));
            }
        }
    }
}
