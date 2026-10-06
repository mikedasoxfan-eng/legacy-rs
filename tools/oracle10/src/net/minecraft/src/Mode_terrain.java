package net.minecraft.src;

import java.io.*;

/** Original bytecode biome maps and terrain with an explicit structures switch. */
public final class Mode_terrain {
    public static void main(String[] a) throws Exception {
        a=java.util.Arrays.copyOfRange(a,1,a.length);
        String mode=a[0]; long seed=Long.parseLong(a[1]);
        int x=Integer.parseInt(a[2]), z=Integer.parseInt(a[3]);
        int w=Integer.parseInt(a[4]), h=Integer.parseInt(a[5]);
        try (DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[6])))) {
            if(mode.equals("layers")) {
                GenLayer[] layers=GenLayer.func_35497_a(seed);
                for(GenLayer layer:layers) {
                    IntCache.func_35268_a();
                    int[] values=layer.func_35500_a(x,z,w,h);
                    for(int i=0;i<w*h;i++) Trace10.writeInt(out, "values[i]", values[i]);
                }
            } else if(mode.equals("raw")) {
                World world=Oracle.newWorld(seed);
                ChunkProviderGenerate gen=new ChunkProviderGenerate(world,seed,false);
                for(int cx=x;cx<x+w;cx++) for(int cz=z;cz<z+h;cz++) Trace10.writeBytes(out, "gen.provideChunk(cx,cz).blocks", gen.provideChunk(cx,cz).blocks);
            } else if(mode.equals("sky")) {
                World world=Oracle.newWorld(seed);world.worldProvider.worldChunkMgr=new WorldChunkManagerHell(BiomeGenBase.sky,0.5F,0.0F);
                ChunkProviderEnd gen=new ChunkProviderEnd(world,seed);
                for(int cx=x;cx<x+w;cx++)for(int cz=z;cz<z+h;cz++)Trace10.writeBytes(out, "gen.provideChunk(cx,cz).blocks", gen.provideChunk(cx,cz).blocks);
            } else throw new IllegalArgumentException(mode);
        }
    }
}
