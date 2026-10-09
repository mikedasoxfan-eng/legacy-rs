package net.minecraft.src;
import java.io.*;
import java.util.*;

/** Original World.randomDisplayUpdates: controlled air/solid/mixed input, all providers/types. */
public final class DepthFogProbe11 {
    public static void main(String[] a)throws Exception {
        final long seed=Long.parseLong(a[0]);
        try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int dimension:new int[]{0,-1,1}) for(final EnumWorldType type:EnumWorldType.values()) for(int shape=0;shape<3;shape++) {
                final WorldSettings settings=new WorldSettings(seed,0,false,false,type);
                World w=new World(new Oracle.NullSaveHandler(seed){
                    public WorldInfo loadWorldInfo(){return new WorldInfo(settings,"oracle");}
                },"oracle",settings,WorldProvider.getProviderForDimension(dimension));
                EntityOracle.FlatProvider provider=new EntityOracle.FlatProvider(w);w.chunkProvider=provider;
                for(Chunk c:provider.chunks.values())for(int x=0;x<16;x++)for(int z=0;z<16;z++)for(int y=0;y<128;y++)
                    c.blocks[(x<<11)|(z<<7)|y]=(byte)(shape==1?1:shape==2&&(x+z+y)%2==0?1:0);
                WorldEventOracle.attach(w);w.rand.setSeed(seed);
                out.writeBoolean(w.worldProvider.func_46064_i());
                for(int[] q:new int[][]{{0,64,0},{0,4,0},{0,1,0},{-9,10,7},{5,0,-5},{0,-3,0},{0,127,0},{0,128,0}})for(int pass=0;pass<2;pass++) {
                    w.randomDisplayUpdates(q[0],q[1],q[2]);
                    WorldEventOracle.snapshot(w,out);EntityOracle.l(out,EntityOracle.rngState(w.rand));
                }
            }
        }
    }
}
