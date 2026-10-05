package net.minecraft.src;
import java.io.*;
public final class Mode_wbfurnace4 {
    static void stack(DataOutputStream o,ItemStack s)throws Exception {
        o.writeBoolean(s!=null);if(s!=null){EntityOracle.i(o,s.itemID);EntityOracle.i(o,s.stackSize);EntityOracle.i(o,s.getItemDamage());}
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int fuel:new int[]{0,5,6,17,18,99,100,103,107,263,327,368})for(int input:new int[]{12,15,319,363,365,1})for(int config=0;config<8;config++) {
                World w=Mode_worldblocks181.fixture(true);w.setBlock(8,64,8,61);TileEntityFurnace f=new TileEntityFurnace();f.setInventorySlotContents(0,new ItemStack(input,3,0));if(fuel!=0)f.setInventorySlotContents(1,new ItemStack(fuel,2,0));
                if((config&1)!=0){ItemStack s=FurnaceRecipes.smelting().getSmeltingResult(input);if(s!=null){s=s.copy();s.stackSize=(config&2)==0?63:64;f.setInventorySlotContents(2,s);}}
                f.furnaceCookTime=new int[]{0,190,199,200,0,190,199,32767}[config];f.furnaceBurnTime=new int[]{0,0,1,-1,200,0,1,-1}[config];
                NBTTagCompound tag=new NBTTagCompound();f.writeToNBT(tag);f.readFromNBT(tag);w.setBlockTileEntity(8,64,8,f);
                for(int t=0;t<24;t++) {
                    w.getChunkFromChunkCoords(0,0).isModified=false;f.updateEntity();
                    for(int n=0;n<3;n++)stack(o,f.getStackInSlot(n));EntityOracle.i(o,f.furnaceBurnTime);EntityOracle.i(o,f.currentItemBurnTime);EntityOracle.i(o,f.furnaceCookTime);
                    o.writeBoolean(f.isInvalid());o.writeBoolean(w.getChunkFromChunkCoords(0,0).isModified);o.writeByte(w.getBlockId(8,64,8));EntityOracle.i(o,w.getBlockMetadata(8,64,8));
                }
            }
        }
    }
}
