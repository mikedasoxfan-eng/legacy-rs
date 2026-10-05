package net.minecraft.src;
import java.io.*;
public final class Mode_wbsign8 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;World w=Mode_worldblocks181.fixture(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{63,68}) {
                BlockSign b=(BlockSign)Block.blocksList[id];o.writeBoolean(b.getBlockEntity() instanceof TileEntitySign);o.writeBoolean(b.getBlockEntity()!=b.getBlockEntity());
                for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int support=0;support<=109;support++) {
                    w.getChunkFromChunkCoords(0,0).chunkTileEntityMap.clear();w.loadedTileEntityList.clear();
                    for(int x=7;x<=9;x++)for(int y=63;y<=65;y++)for(int z=7;z<=9;z++)Mode_worldblocks181.raw(w,x,y,z,0,0);
                    Mode_worldblocks181.raw(w,8,63,8,support,0);for(int[] d:new int[][]{{-1,0},{1,0},{0,-1},{0,1}})Mode_worldblocks181.raw(w,8+d[0],64,8+d[1],support,0);
                    Mode_worldblocks181.raw(w,8,64,8,id,m);w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);b.onBlockAdded(w,8,64,8);TileEntity old=w.getBlockTileEntity(8,64,8);
                    b.onNeighborBlockChange(w,8,64,8,1);Mode_wbtrapdoors6.result(o,w);o.writeBoolean(old.isInvalid());o.writeBoolean(w.getBlockTileEntity(8,64,8)==old);
                }
            }
            EntityOracle.FlatProvider provider=(EntityOracle.FlatProvider)w.chunkProvider;Chunk far=new Chunk(w,new byte[32768],16777216>>4,0);far.isChunkLoaded=true;provider.chunks.put(provider.key(16777216>>4,0),far);
            for(int id:new int[]{63,68})for(int x:new int[]{8,-17,16777216})for(int m=0;m<16;m++)for(int shape=0;shape<2;shape++) {
                Block b=Block.blocksList[id];Mode_worldblocks181.raw(w,x,64,8,id,m);b.setBlockBounds(shape==0?.25F:-.25F,0F,.25F,shape==0?.75F:1.25F,1F,.75F);
                Mode_worldblocks181.box(o,b.getSelectedBoundingBoxFromPool(w,x,64,8));Mode_wbtrapdoor8.bounds(o,b);Mode_worldblocks181.box(o,b.getCollisionBoundingBoxFromPool(w,x,64,8));
                b.setBlockBoundsBasedOnState(w,x,64,8);Mode_wbtrapdoor8.bounds(o,b);
            }
        }
    }
}
