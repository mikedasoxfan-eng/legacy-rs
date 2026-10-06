package net.minecraft.src;
import java.io.*;

/** Item.onItemUse for the changed/new release item classes (slab, lily pad, coloured/metadata blocks, seeds, dye, bed, records, reeds). */
public final class Mode_wbb4use {
    static final int[][] ITEMS={{44,0},{44,1},{44,2},{44,3},{44,4},{44,5},{111,0},{106,0},{31,0},{31,1},{31,2},{18,0},{18,1},{18,2},{6,0},{6,1},{6,2},{17,0},{17,1},{17,2},
        {98,0},{98,1},{98,2},{98,3},{35,0},{35,14},{295,0},{361,0},{362,0},{372,0},{351,-1},{351,0},{351,3},{351,4},{351,15},{351,16},{355,0},{2256,0},{2257,0},{2266,0},
        {324,0},{330,0},{379,0},{380,0},{354,0},{116,0},{120,0},{122,0},{110,0},{113,0},{120,0},{373,0},{373,16385}};
    static final int[][] CONFIGS={{0,0},{1,0},{2,0},{3,0},{60,0},{78,0},{106,1},{31,1},{8,1},{9,0},{51,0},{1,0},{88,0},{44,0},{44,3},{104,3},{59,2},{6,1},{39,0},{115,1},{110,0},{84,0},{84,1}};
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int[] item:ITEMS)for(int side=0;side<6;side++)for(int config=0;config<CONFIGS.length;config++) {
                World w=Mode_worldblocks10.fixture(true);EntityPlayer p=Blocks4Oracle.newPlayer(w);Blocks4Oracle.place(p,8.5,66.0,5.5,-450.0F+90.0F*config,30F);
                Mode_worldblocks10.raw(w,8,64,8,CONFIGS[config][0],CONFIGS[config][1]);
                if(config==11)for(int[] v:new int[][]{{-1,0},{1,0},{0,-1},{0,1}})Mode_worldblocks10.raw(w,8+v[0],64,8+v[1],1,0);
                if(config==12||config==19)Mode_worldblocks10.raw(w,8,63,8,88,0);
                if(config==15||config==16)Mode_worldblocks10.raw(w,8,63,8,60,7);
                if(config==18)Mode_worldblocks10.raw(w,8,63,8,3,0);
                if(config==20)Mode_worldblocks10.raw(w,9,65,8,2,0);
                if(config>=21)w.setBlockTileEntity(8,64,8,new TileEntityRecordPlayer());
                if(config==22)((TileEntityRecordPlayer)w.getBlockTileEntity(8,64,8)).record=2258;
                ItemStack s=new ItemStack(item[0],new int[]{1,4,0,1}[config%4],item[1]);
                Blocks4Oracle.seeds(w);
                Blocks4Oracle.bool(o,"onItemUse",s.getItem().onItemUse(s,p,w,8,64,8,side));Blocks4Oracle.stack(o,s);
                Mode_wbplacement4.cells(o,w,6,10,62,67);
                TileEntity t=w.getBlockTileEntity(8,64,8);Blocks4Oracle.bool(o,"record",t instanceof TileEntityRecordPlayer);if(t instanceof TileEntityRecordPlayer)Blocks4Oracle.i(o,((TileEntityRecordPlayer)t).record);
                Blocks4Oracle.entities(o,w);Blocks4Oracle.rngs(o,w);WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
