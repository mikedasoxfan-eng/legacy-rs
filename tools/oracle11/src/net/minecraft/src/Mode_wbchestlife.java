package net.minecraft.src;
import java.io.*;
import java.util.*;
public final class Mode_wbchestlife {
    static void stack(DataOutputStream o,ItemStack s)throws Exception {Trace10.writeBoolean(o, "s!=null", s!=null);if(s!=null){EntityOracle.i(o,Trace10.namedInt("s.itemID", s.itemID));EntityOracle.i(o,Trace10.namedInt("s.stackSize", s.stackSize));EntityOracle.i(o,Trace10.namedInt("s.getItemDamage()", s.getItemDamage()));}}
    static void cache(DataOutputStream o,TileEntityChest t)throws Exception {
        EntityOracle.i(o,Trace10.namedInt("t.getBlockMetadata()", t.getBlockMetadata()));Block b=t.getBlockType();EntityOracle.i(o,Trace10.namedInt("b==null?0:b.blockID", b==null?0:b.blockID));Trace10.writeBoolean(o, "t.isInvalid()", t.isInvalid());Trace10.writeBoolean(o, "t.field_35155_a", t.adjacentChestChecked);
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            World w=Mode_worldblocks10.fixture(true);Mode_worldblocks10.raw(w,8,64,8,54,3);TileEntityChest t=(TileEntityChest)w.getBlockTileEntity(8,64,8);
            Random r=new Random(1729);
            for(int n=0;n<400;n++) {
                int slot=r.nextInt(36),op=r.nextInt(3),amount=r.nextInt(150)-30;
                if(op==0)t.setInventorySlotContents(slot,new ItemStack(n%2==0?1:260,amount,n));
                if(op==1)stack(o,t.decrStackSize(slot,amount));
                for(int k=0;k<36;k++)stack(o,t.getStackInSlot(k));cache(o,t);
            }
            NBTTagCompound n=new NBTTagCompound();t.writeToNBT(n);TileEntityChest loaded=new TileEntityChest();loaded.readFromNBT(n);loaded.worldObj=w;
            EntityOracle.i(o,Trace10.namedInt("loaded.getSizeInventory()", loaded.getSizeInventory()));for(int k=0;k<27;k++)stack(o,loaded.getStackInSlot(k));
            try{loaded.getStackInSlot(27);Trace10.writeBoolean(o, "false", false);}catch(ArrayIndexOutOfBoundsException e){Trace10.writeBoolean(o, "true", true);}
            t.updateContainingBlockInfo();cache(o,t);Mode_worldblocks10.raw(w,8,64,8,1,7);cache(o,t);t.onInventoryChanged();cache(o,t);t.updateContainingBlockInfo();cache(o,t);
            Mode_worldblocks10.raw(w,8,64,8,54,2);t.updateContainingBlockInfo();t.checkForAdjacentChests();cache(o,t);
            w.setBlockMetadata(8,64,8,19);cache(o,t);t.invalidate();cache(o,t);t.validate();cache(o,t);
        }
    }
}
