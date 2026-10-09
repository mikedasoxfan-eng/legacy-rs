package net.minecraft.src;
import java.io.*;
public final class Mode_wbmusic6 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            World w=Mode_worldblocks10.fixture(true);
            EntityPlayer p=PlayerOracle.player(w);
            for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int note:new int[]{-128,-26,-1,0,23,24,25,126,127})for(int floor:new int[]{1,12,20,5,0})for(int roof:new int[]{0,1})for(int hook=0;hook<5;hook++) {
                w.multiplayerWorld=false;Mode_worldblocks10.raw(w,8,63,8,floor,0);Mode_worldblocks10.raw(w,8,65,8,roof,0);Mode_worldblocks10.raw(w,8,64,8,25,m);
                Mode_worldblocks10.raw(w,7,64,8,hook==3&&floor!=0?76:0,5);
                TileEntityNote t=new TileEntityNote();w.setBlockTileEntity(8,64,8,t);t.note=(byte)note;t.previousRedstoneState=(m&1)!=0;t.blockMetadata=-1;
                w.multiplayerWorld=remote;Mode_wbtrapdoors6.seed(w);
                if(hook==0)Trace10.writeBoolean(o, "Block.blocksList[25].blockActivated(w,8,64,8,p)", Block.blocksList[25].blockActivated(w,8,64,8,p));
                else if(hook==1)Block.blocksList[25].onBlockClicked(w,8,64,8,p);
                else if(hook==2)t.triggerNote(w,8,64,8);
                else if(hook==3)Block.blocksList[25].onNeighborBlockChange(w,8,64,8,76);
                else Block.blocksList[25].powerBlock(w,8,64,8,floor,note);
                EntityOracle.i(o,Trace10.namedInt("t.note", t.note));EntityOracle.i(o,Trace10.namedInt("t.blockMetadata", t.blockMetadata));Trace10.writeBoolean(o, "t.previousRedstoneState", t.previousRedstoneState);Mode_wbtrapdoors6.result(o,w);
            }
            BlockJukeBox b=(BlockJukeBox)Block.blocksList[84];
            for(boolean remote:new boolean[]{false,true})for(int m=0;m<16;m++)for(int record:new int[]{0,2256,2257})for(int hook=0;hook<4;hook++) {
                w.multiplayerWorld=false;Mode_worldblocks10.raw(w,7,64,8,0,0);Mode_worldblocks10.raw(w,8,63,8,1,0);Mode_worldblocks10.raw(w,8,65,8,0,0);Mode_worldblocks10.raw(w,8,64,8,84,m);
                TileEntityRecordPlayer t=new TileEntityRecordPlayer();w.setBlockTileEntity(8,64,8,t);t.record=record;t.blockMetadata=-1;
                w.multiplayerWorld=remote;w.loadedEntityList.clear();Mode_wbtrapdoors6.seed(w);
                if(hook==0)Trace10.writeBoolean(o, "b.blockActivated(w,8,64,8,p)", b.blockActivated(w,8,64,8,p));
                else if(hook==1)b.insertDisc(w,8,64,8);
                else if(hook==2)b.ejectRecord(w,8,64,8,record);
                else b.onBlockRemoval(w,8,64,8);
                EntityOracle.i(o,Trace10.namedInt("t.record", t.record));EntityOracle.i(o,Trace10.namedInt("t.blockMetadata", t.blockMetadata));Trace10.writeBoolean(o, "t.isInvalid()", t.isInvalid());Mode_wbtrapdoors6.result(o,w);
            }
        }
    }
}
