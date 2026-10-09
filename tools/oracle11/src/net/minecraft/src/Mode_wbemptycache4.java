package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
public final class Mode_wbemptycache4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int sky:new int[]{0,7,15,20}) {
                final World w=Oracle.newWorld(0);w.skylightSubtracted=sky;
                w.chunkProvider=(IChunkProvider)Proxy.newProxyInstance(IChunkProvider.class.getClassLoader(),new Class[]{IChunkProvider.class},(p,m,v)-> {
                    if(m.getReturnType()==Chunk.class)return new EmptyChunk(w,(Integer)v[0],(Integer)v[1]);
                    if(m.getReturnType()==boolean.class)return false;
                    if(m.getReturnType()==String.class)return "empty";return null;
                });
                ChunkCache c=new ChunkCache(w,-16,0,-16,32,127,32);
                for(int x:new int[]{-16,-1,0,15,16,32})for(int z:new int[]{-16,-1,0,15,16,32})for(int y:new int[]{-1,0,63,127,128}) {
                    EntityOracle.i(o,Trace10.namedInt("c.getBlockId(x,y,z)", c.getBlockId(x,y,z)));EntityOracle.i(o,Trace10.namedInt("c.getBlockMetadata(x,y,z)", c.getBlockMetadata(x,y,z)));EntityOracle.i(o,Trace10.namedInt("c.getLightValue(x,y,z)", c.getLightValue(x,y,z)));EntityOracle.i(o,Trace10.namedInt("c.getLightBrightnessForSkyBlocks(x,y,z,7)", c.getLightBrightnessForSkyBlocks(x,y,z,7)));
                    if(c.getBlockMaterial(x,y,z)!=Material.air)throw new AssertionError();EntityOracle.i(o,0);Trace10.writeBoolean(o, "c.isAirBlock(x,y,z)", c.isAirBlock(x,y,z));Trace10.writeBoolean(o, "c.isBlockNormalCube(x,y,z)", c.isBlockNormalCube(x,y,z));Trace10.writeBoolean(o, "c.getBlockTileEntity(x,y,z)!=null", c.getBlockTileEntity(x,y,z)!=null);
                }
            }
        }
    }
}
