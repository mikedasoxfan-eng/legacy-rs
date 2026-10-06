package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
public final class Mode_wbjukebox10 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;BlockJukeBox b=(BlockJukeBox)Block.jukebox;
        Field scanning=World.class.getDeclaredField("scanningTileEntities");scanning.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int side:new int[]{Integer.MIN_VALUE,-1,0,1,2,5,6,Integer.MAX_VALUE})EntityOracle.i(o,Trace10.namedInt("b.getBlockTextureFromSide(side)", b.getBlockTextureFromSide(side)));
            TileEntity t1=b.getBlockEntity(),t2=b.getBlockEntity();Trace10.writeBoolean(o, "t1 instanceof TileEntityRecordPlayer&&((TileEntityRecordPlayer)t1).record==0", t1 instanceof TileEntityRecordPlayer&&((TileEntityRecordPlayer)t1).record==0);Trace10.writeBoolean(o, "t1!=t2", t1!=t2);
            for(boolean remote:new boolean[]{false,true})for(int meta=0;meta<16;meta++)for(int record:new int[]{Integer.MIN_VALUE,-65537,-1,0,1,2256,2257,32768,65536,Integer.MAX_VALUE})for(int config=0;config<5;config++)for(int hook=0;hook<5;hook++) {
                World w=Mode_worldblocks10.fixture(true);Mode_worldblocks10.raw(w,8,64,8,84,meta);
                TileEntityRecordPlayer old=new TileEntityRecordPlayer();old.record=record;w.setBlockTileEntity(8,64,8,old);
                if(config==1)old.invalidate();
                if(config==2){w.removeBlockTileEntity(8,64,8);Mode_worldblocks10.raw(w,8,64,8,1,meta);}
                if(config==3)w.setBlockTileEntity(8,64,8,new TileEntityFurnace());
                if(config==4){w.removeBlockTileEntity(8,64,8);scanning.setBoolean(w,true);}
                w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);w.getChunkFromChunkCoords(0,0).isModified=false;
                int error=0;try {
                    if(hook==0)b.ejectRecord(w,8,64,8,record);
                    if(hook==1)b.func_28038_b_(w,8,64,8);
                    if(hook==2){boolean active=w.getBlockMetadata(8,64,8)!=0;Trace10.writeBoolean(o, "active", active);b.blockActivated(w,8,64,8,null);}
                    if(hook==3)b.onBlockRemoval(w,8,64,8);
                    if(hook==4)b.dropBlockAsItemWithChance(w,8,64,8,meta,0.75F, 0);
                }catch(Throwable ex){error=Mode_wbmachine10.error(ex);}Trace10.writeByte(o, "error", error);
                EntityOracle.i(o,Trace10.namedInt("old.record", old.record));EntityOracle.i(o,Trace10.namedInt("old.blockMetadata", old.blockMetadata));Trace10.writeBoolean(o, "old.isInvalid()", old.isInvalid());Trace10.writeBoolean(o, "w.getChunkFromChunkCoords(0,0).isModified", w.getChunkFromChunkCoords(0,0).isModified);
                Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
