package net.minecraft.src;
import java.io.*;
/** Original jukebox tile NBT, insertion, ejection and null-tile behavior. */
public final class Mode_wbrecord4 {
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int value:new int[]{Integer.MIN_VALUE,-1,0,1,2256,2257,Integer.MAX_VALUE}) {
                TileEntityRecordPlayer tile=new TileEntityRecordPlayer();tile.record=value;
                NBTTagCompound n=new NBTTagCompound();tile.writeToNBT(n);
                Trace10.writeBoolean(o, "n.hasKey(\"Record\")", n.hasKey("Record"));EntityOracle.i(o,Trace10.namedInt("n.getInteger(\"Record\")", n.getInteger("Record")));
                TileEntityRecordPlayer loaded=new TileEntityRecordPlayer();loaded.readFromNBT(n);EntityOracle.i(o,Trace10.namedInt("loaded.record", loaded.record));
            }
            for(int record:new int[]{2256,2257})for(int config=0;config<4;config++) {
                World w=Mode_worldblocks10.fixture(true);
                EntityPlayer p=new Mode_wbplacement4.Player((net.minecraft.client.Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class),w);
                w.setBlock(8,64,8,84);TileEntityRecordPlayer tile=(TileEntityRecordPlayer)w.getBlockTileEntity(8,64,8);
                if((config&1)!=0)tile.invalidate();if((config&2)!=0)w.setBlockMetadata(8,64,8,1);
                w.rand.setSeed(42);OracleHooks.reset(100);EntityOracle.mathRandom().setSeed(456);WorldEventOracle.attach(w);
                ItemStack stack=new ItemStack(record,2,0);
                Trace10.writeBoolean(o, "stack.getItem().onItemUse(stack,p,w,8,64,8,1)", stack.getItem().onItemUse(stack,p,w,8,64,8,1));EntityOracle.i(o,Trace10.namedInt("stack.stackSize", stack.stackSize));
                EntityOracle.i(o,Trace10.namedInt("w.getBlockMetadata(8,64,8)", w.getBlockMetadata(8,64,8)));WorldEventOracle.snapshot(w,o);
                ((BlockJukeBox)Block.jukebox).func_28038_b_(w,8,64,8);
                EntityOracle.i(o,Trace10.namedInt("w.getBlockMetadata(8,64,8)", w.getBlockMetadata(8,64,8)));EntityOracle.l(o,Trace10.namedLong("EntityOracle.rngState(w.rand)", EntityOracle.rngState(w.rand)));EntityOracle.i(o,Trace10.namedInt("w.loadedEntityList.size()", w.loadedEntityList.size()));WorldEventOracle.snapshot(w,o);
            }
        }
    }
}
