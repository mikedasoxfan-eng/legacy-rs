package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;
public final class Mode_wbunload6 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;Field queued=World.class.getDeclaredField("entityRemoval");queued.setAccessible(true);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int block:new int[]{25,84,54})for(boolean repeated:new boolean[]{false,true}) {
                World w=Mode_worldblocks10.fixture(true);Mode_wbtrapdoors6.seed(w);TileEntity[] tiles=new TileEntity[2];
                for(int k=0;k<2;k++){int x=k==0?8:24;Mode_worldblocks10.raw(w,x,72,8,block,3);tiles[k]=w.getBlockTileEntity(x,72,8);}
                if(block==54)for(TileEntity tile:tiles)((TileEntityChest)tile).field_35156_h=1;
                for(int step=0;step<7;step++) {
                    if(step==1){w.getChunkFromChunkCoords(0,0).onChunkUnload();if(repeated)w.getChunkFromChunkCoords(0,0).onChunkUnload();TileEntity t=w.getBlockTileEntity(8,72,8);if(t instanceof TileEntityNote)((TileEntityNote)t).note=7;else if(t instanceof TileEntityRecordPlayer)((TileEntityRecordPlayer)t).record=2256;}
                    else if(step==2||step==4||step==6)w.updateEntities();
                    else if(step==3)w.getChunkFromChunkCoords(0,0).onChunkLoad();
                    else if(step==5)w.getChunkFromChunkCoords(1,0).onChunkUnload();
                    EntityOracle.i(o,Trace10.namedInt("w.loadedTileEntityList.size()", w.loadedTileEntityList.size()));for(Object object:w.loadedTileEntityList){TileEntity t=(TileEntity)object;EntityOracle.i(o,Trace10.namedInt("t.xCoord", t.xCoord));EntityOracle.i(o,Trace10.namedInt("t.yCoord", t.yCoord));EntityOracle.i(o,Trace10.namedInt("t.zCoord", t.zCoord));}
                    EntityOracle.i(o,Trace10.namedInt("((java.util.List)queued.get(w)).size()", ((java.util.List)queued.get(w)).size()));
                    for(TileEntity t:tiles){Trace10.writeBoolean(o, "t.isInvalid()", t.isInvalid());if(t instanceof TileEntityChest){TileEntityChest c=(TileEntityChest)t;EntityOracle.f(o,Trace10.namedFloat("c.field_35148_f", c.field_35148_f));EntityOracle.f(o,Trace10.namedFloat("c.field_35149_g", c.field_35149_g));EntityOracle.i(o,Trace10.namedInt("c.field_35156_h", c.field_35156_h));}else if(t instanceof TileEntityNote)EntityOracle.i(o,Trace10.namedInt("((TileEntityNote)t).note", ((TileEntityNote)t).note));else if(t instanceof TileEntityRecordPlayer)EntityOracle.i(o,Trace10.namedInt("((TileEntityRecordPlayer)t).record", ((TileEntityRecordPlayer)t).record));}
                    EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));WorldEventOracle.snapshot(w,o);
                }
            }
        }
    }
}
