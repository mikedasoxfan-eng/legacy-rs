package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** Execute the original TileEntityRendererPiston, recording its Tessellator bits. */
class PistonRendererOracle {
    static void run(String out)throws Exception {
        Oracle.DataOut o=new Oracle.DataOut();o.bytes("B181PST1".getBytes("US-ASCII"));
        int count=0;
        Oracle.DataOut cases=new Oracle.DataOut();
        for(boolean ao:new boolean[]{false,true})for(boolean fancy:new boolean[]{false,true}) {
            Oracle.meshSetup(fancy,ao);
            Block.leaves.setGraphicsLevel(fancy);RenderBlocks.fancyGrass=fancy;
            ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));
            ColorizerFoliage.getFoilageBiomeColorizer(Oracle.jarImage("misc/foliagecolor.png"));
            World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);w.skylightSubtracted=3;
            for(int cx=-3;cx<=3;cx++)for(int cz=-3;cz<=3;cz++) {
                Chunk c=w.getChunkFromChunkCoords(cx,cz);
                for(int x=0;x<16;x++)for(int y=0;y<128;y++)for(int z=0;z<16;z++) {
                    int wx=cx*16+x,wz=cz*16+z;
                    c.skylightMap.setNibble(x,y,z,(wx*7+y*3+wz*5)&15);
                    c.blocklightMap.setNibble(x,y,z,(wx*3+y*11+wz)&15);
                }
            }
            TileEntityRendererPiston renderer=new TileEntityRendererPiston();
            TileEntityRenderer dispatcher=(TileEntityRenderer)Oracle.allocate(TileEntityRenderer.class);
            dispatcher.renderEngine=RendererOracle.manager(w).renderEngine;
            renderer.setTileEntityRenderer(dispatcher);renderer.func_31069_a(w);
            Field progress=TileEntityPiston.class.getDeclaredField("progress");progress.setAccessible(true);
            Field previous=TileEntityPiston.class.getDeclaredField("lastProgress");previous.setAccessible(true);
            for(int id:new int[]{1,2,4,5,12,17,18,20,27,28,29,33,34,35,43,44,53,60,66,67,79,85,89})for(int facing=0;facing<6;facing++)for(int mode=0;mode<3;mode++)for(int phase=0;phase<3;phase++) {
                int meta=id==34?facing|8:(id==29||id==33?facing:phase);
                boolean extending=mode==0,head=mode==2&&(id==29||id==33);
                int x=-17,y=80,z=15;
                EntityOracle.rawBlock(w,x,y,z,36,meta);
                TileEntityPiston p=new TileEntityPiston(id,meta,facing,extending,head);p.worldObj=w;p.xCoord=x;p.yCoord=y;p.zCoord=z;
                progress.setFloat(p,phase*0.5F);previous.setFloat(p,Math.max(0,phase-1)*0.5F);
                for(float partial:new float[]{0,0.25F,0.5F,0.999F,1}) {
                    double[] pos={3.125,-7.375,1.0/3.0};
                    for(int v:new int[]{id,meta,facing,extending?1:0,head?1:0,Float.floatToRawIntBits(phase*0.5F),Float.floatToRawIntBits(Math.max(0,phase-1)*0.5F),Float.floatToRawIntBits(partial),x,y,z,ao?1:0,fancy?1:0})cases.i32(v);
                    for(double v:pos)cases.i64(Double.doubleToRawLongBits(v));
                    RendererOracle.pistonCapture=true;RendererOracle.pistonRaw=null;
                    renderer.a(p,pos[0],pos[1],pos[2],partial);
                    RendererOracle.pistonCapture=false;
                    RendererOracle.Raw raw=RendererOracle.pistonRaw;
                    // 1.0 draws quads (no triangle conversion) with a packed lightmap brightness.
                    cases.i32(raw==null?0:raw.n);
                    if(raw!=null)for(int v=0;v<raw.n;v++){for(int f=0;f<6;f++)cases.i32(raw.data[v*8+f]);cases.i32(raw.data[v*8+7]);}
                    count++;
                }
            }
        }
        o.i32(count);try(OutputStream f=new BufferedOutputStream(new FileOutputStream(out))){o.writeTo(f);cases.writeTo(f);}
        System.out.println(count+" piston renderer cases");
    }
}
