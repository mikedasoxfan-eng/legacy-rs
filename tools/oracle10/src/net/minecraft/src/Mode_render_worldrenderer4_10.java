package net.minecraft.src;
import java.io.*;
import java.util.*;

/** Original 1.0 WorldRenderer.updateRenderer on real terrain: per section and pass the
 * display-list vertices (relative to the section origin), skipRenderPass, the special
 * tile-entity list and isChunkLit. Only GL endpoints are substituted. */
public final class Mode_render_worldrenderer4_10 {
    public static void main(String[] a)throws Exception {
        long seed=Long.parseLong(a[0]),wrand=Long.parseLong(a[1]);boolean fancy=!a[2].equals("fast"),ao=!a[3].equals("off");
        Oracle.meshSetup(fancy,ao);
        Block.leaves.setGraphicsLevel(false);
        World w=Oracle.newWorld(seed);w.rand.setSeed(wrand);w.getInitialSpawnLocation();ChunkCoordinates sp=w.getSpawnPoint();
        for(int dx=-128;dx<=128;dx+=16)for(int dz=-128;dz<=128;dz+=16){w.getBlockId(sp.posX+dx,64,sp.posZ+dz);Oracle.drainLighting(w);}
        Block.leaves.setGraphicsLevel(fancy);RenderBlocks.fancyGrass=fancy;
        ColorizerGrass.setGrassBiomeColorizer(Oracle.jarImage("misc/grasscolor.png"));ColorizerFoliage.getFoilageBiomeColorizer(Oracle.jarImage("misc/foliagecolor.png"));
        RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
        TileEntityRenderer.instance.worldObj=w;
        int scx=sp.posX>>4,scz=sp.posZ>>4,r=Integer.parseInt(a[4]);
        Oracle.DataOut o=new Oracle.DataOut();o.i32(0x34525257);o.i32(w.skylightSubtracted);o.i32((2*r+1)*(2*r+1)*8);
        for(int cx=scx-r;cx<=scx+r;cx++)for(int cz=scz-r;cz<=scz+r;cz++)for(int sy=0;sy<8;sy++){
            ArrayList tiles=new ArrayList();int base=RendererOracle.genLists(3);
            WorldRenderer wr=new WorldRenderer(w,tiles,cx*16,sy*16,cz*16,16,base);
            wr.markDirty();Chunk.isLit=true;wr.updateRenderer();
            o.i32(cx);o.i32(sy);o.i32(cz);
            for(int pass=0;pass<2;pass++){
                ArrayList<RendererOracle.Raw> list=RendererOracle.lists.get(base+pass);
                o.i32(list==null?0:1);o.i32(wr.skipRenderPass[pass]?1:0);
                int n=0;if(list!=null)for(RendererOracle.Raw raw:list)if(raw.command==null)n+=raw.n;o.i32(n);
                if(list!=null)for(RendererOracle.Raw raw:list)if(raw.command==null)for(int v=0;v<raw.n;v++){for(int k=0;k<6;k++)o.i32(raw.data[v*8+k]);o.i32(raw.packedLight?raw.data[v*8+7]:0);}
            }
            o.i32(wr.tileEntityRenderers.size());for(Object t:wr.tileEntityRenderers){TileEntity te=(TileEntity)t;o.i32(te.xCoord);o.i32(te.yCoord);o.i32(te.zCoord);}
            o.i32(tiles.size());o.i32(wr.isChunkLit?1:0);
            for(int k=0;k<3;k++)RendererOracle.lists.remove(base+k);
        }
        try(OutputStream f=new FileOutputStream(a[5])){o.writeTo(f);}
        System.out.println("Original 1.0 WorldRenderer: "+(2*r+1)*(2*r+1)*8+" sections");
    }
}
