package net.minecraft.src;
import java.io.*;
import java.util.*;
/** Calls the original ChunkCache, including its different World boundary rules. */
public final class Mode_wbcache {
    static void fill(Chunk c) {
        for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<128;y++) {
            c.skylightMap.setNibble(x,y,z,(x*3+z*5+y*7)&15);
            c.blocklightMap.setNibble(x,y,z,(x*7+z*11+y*3)&15);
        }
    }
    static void sample(DataOutputStream o,ChunkCache c,int x,int y,int z,int min)throws Exception {
        EntityOracle.i(o,Trace10.namedInt("c.getBlockId(x,y,z)", c.getBlockId(x,y,z)));EntityOracle.i(o,Trace10.namedInt("c.getBlockMetadata(x,y,z)", c.getBlockMetadata(x,y,z)));
        EntityOracle.i(o,Trace10.namedInt("c.getLightValue(x,y,z)", c.getLightValue(x,y,z)));EntityOracle.i(o,Trace10.namedInt("c.getLightValueExt(x,y,z,false)", c.getLightValueExt(x,y,z,false)));
        for(EnumSkyBlock k:new EnumSkyBlock[]{EnumSkyBlock.Sky,EnumSkyBlock.Block}) {
            EntityOracle.i(o,Trace10.namedInt("c.getSkyBlockTypeBrightness(k,x,y,z)", c.getSkyBlockTypeBrightness(k,x,y,z)));EntityOracle.i(o,Trace10.namedInt("c.getSpecialBlockBrightness(k,x,y,z)", c.getSpecialBlockBrightness(k,x,y,z)));
        }
        EntityOracle.i(o,Trace10.namedInt("c.getLightBrightnessForSkyBlocks(x,y,z,min)", c.getLightBrightnessForSkyBlocks(x,y,z,min)));EntityOracle.f(o,Trace10.namedFloat("c.getBrightness(x,y,z,min)", c.getBrightness(x,y,z,min)));EntityOracle.f(o,Trace10.namedFloat("c.getLightBrightness(x,y,z)", c.getLightBrightness(x,y,z)));
        Trace10.writeBoolean(o, "c.isAirBlock(x,y,z)", c.isAirBlock(x,y,z));Trace10.writeBoolean(o, "c.isBlockOpaqueCube(x,y,z)", c.isBlockOpaqueCube(x,y,z));Trace10.writeBoolean(o, "c.isBlockNormalCube(x,y,z)", c.isBlockNormalCube(x,y,z));
        Trace10.writeBoolean(o, "c.getBlockMaterial(x,y,z).getIsSolid()", c.getBlockMaterial(x,y,z).getIsSolid());Trace10.writeBoolean(o, "c.getBlockMaterial(x,y,z).getIsLiquid()", c.getBlockMaterial(x,y,z).getIsLiquid());
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID; World w=Mode_worldblocks10.fixture(true);
        EntityOracle.FlatProvider p=(EntityOracle.FlatProvider)w.chunkProvider;for(Chunk c:p.chunks.values())fill(c);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int fancy=0;fancy<2;fancy++) {
                ((BlockLeaves)Block.leaves).setGraphicsLevel(fancy!=0);
                for(int sub:new int[]{0,3,15,20}) {
                    w.skylightSubtracted=sub;ChunkCache c=new ChunkCache(w,-17,-40,-17,32,170,32);
                    EntityOracle.i(o,Trace10.namedInt("c.func_35452_b()", c.func_35452_b()));Trace10.writeBoolean(o, "c.getWorldChunkManager()==w.getWorldChunkManager()", c.getWorldChunkManager()==w.getWorldChunkManager());
                    for(int id=0;id<=109;id++)if(id==0||Block.blocksList[id]!=null)for(int meta=0;meta<16;meta++) {
                        Mode_worldblocks10.raw(w,8,64,8,id,meta);
                        sample(o,c,8,64,8,meta);
                        Trace10.writeBoolean(o, "w.isBlockNormalCube(8,64,8)", w.isBlockNormalCube(8,64,8));
                    }
                    Mode_worldblocks10.raw(w,8,64,8,0,0);
                    for(int x:new int[]{-17,-16,-1,0,15,16,31,32})for(int z:new int[]{-17,-1,0,15,16,32})for(int y:new int[]{-100,-1,0,1,63,64,126,127,128,300})sample(o,c,x,y,z,0);
                    for(int x:new int[]{-33,48})for(int y:new int[]{-1,0,128})EntityOracle.i(o,Trace10.namedInt("c.getBlockId(x,y,0)", c.getBlockId(x,y,0)));
                }
            }
            ((BlockLeaves)Block.leaves).setGraphicsLevel(true);
            for(int x:new int[]{-30000001,-30000000,29999999,30000000})for(int z:new int[]{-30000001,-30000000,29999999,30000000,30000001}) {
                Chunk ch=new Chunk(w,new byte[32768],x>>4,z>>4);fill(ch);p.chunks.put(p.key(x>>4,z>>4),ch);
                ChunkCache c=new ChunkCache(w,x,0,z,x,127,z);
                for(int y:new int[]{-1,0,127,128})sample(o,c,x,y,z,0);
            }
            Mode_worldblocks10.raw(w,8,64,8,54,3);ChunkCache c=new ChunkCache(w,0,0,0,15,127,15);
            TileEntity t=c.getBlockTileEntity(8,64,8);Trace10.writeBoolean(o, "t instanceof TileEntityChest", t instanceof TileEntityChest);Trace10.writeBoolean(o, "t==c.getBlockTileEntity(8,64,8)", t==c.getBlockTileEntity(8,64,8));
            EntityOracle.i(o,Trace10.namedInt("t.getBlockMetadata()", t.getBlockMetadata()));Trace10.writeBoolean(o, "c.getBlockTileEntity(8,63,8)==null", c.getBlockTileEntity(8,63,8)==null);
        }
    }
}
