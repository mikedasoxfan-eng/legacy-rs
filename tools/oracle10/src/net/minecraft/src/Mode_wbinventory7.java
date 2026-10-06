package net.minecraft.src;
import java.io.*;
import java.lang.reflect.*;
import java.util.*;

/** TileEntityFurnace/Dispenser inventory, NBT, scaling and interaction methods. */
public final class Mode_wbinventory7 {
    static void nbt(DataOutputStream o,TileEntity t)throws Exception {
        NBTTagCompound n=new NBTTagCompound();t.writeToNBT(n);ByteArrayOutputStream b=new ByteArrayOutputStream();CompressedStreamTools.writeTo(n,new DataOutputStream(b));EntityOracle.i(o,Trace10.namedInt("b.size()", b.size()));Trace10.writeBytes(o, "b.toByteArray()", b.toByteArray());
    }
    static void snapshot(DataOutputStream o,World w,TileEntity t,IInventory inv)throws Exception {
        for(int k=0;k<inv.getSizeInventory();k++)Mode_wbfurnace4.stack(o,inv.getStackInSlot(k));
        EntityOracle.i(o,Trace10.namedInt("t.blockMetadata", t.blockMetadata));EntityOracle.i(o,Trace10.namedInt("t.field_35146_o==null?0:t.field_35146_o.blockID", t.field_35146_o==null?0:t.field_35146_o.blockID));Trace10.writeBoolean(o, "t.isInvalid()", t.isInvalid());Trace10.writeBoolean(o, "w.getChunkFromChunkCoords(0,0).isModified", w.getChunkFromChunkCoords(0,0).isModified);
    }
    public static void main(String[] a)throws Exception {
        int init=Block.stone.blockID;
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int id:new int[]{61,23}) {
                World w=Mode_worldblocks10.fixture(true);Mode_worldblocks10.raw(w,8,64,8,id,3);TileEntity t=w.getBlockTileEntity(8,64,8);IInventory inv=(IInventory)t;
                EntityOracle.i(o,Trace10.namedInt("inv.getSizeInventory()", inv.getSizeInventory()));EntityOracle.i(o,Trace10.namedInt("inv.getInventoryStackLimit()", inv.getInventoryStackLimit()));o.writeUTF(inv.getInvName());
                Random r=new Random(1729);
                for(int n=0;n<400;n++) {
                    int slot=r.nextInt(inv.getSizeInventory()),op=r.nextInt(4),amount=r.nextInt(150)-30;
                    Mode_worldblocks10.raw(w,8,64,8,id,n&15);w.getChunkFromChunkCoords(0,0).isModified=false;
                    if(op==0)inv.setInventorySlotContents(slot,new ItemStack(n%2==0?15:263,amount,n));
                    if(op==1)Mode_wbfurnace4.stack(o,inv.decrStackSize(slot,amount));
                    if(op==2)inv.setInventorySlotContents(slot,null);
                    if(op==3)t.onInventoryChanged();snapshot(o,w,t,inv);
                }
                nbt(o,t);
                for(int c=0;c<16;c++) {
                    NBTTagCompound tag=new NBTTagCompound();tag.setString("id",id==61?"Furnace":"Trap");tag.setInteger("x",8);tag.setInteger("y",64);tag.setInteger("z",8);tag.setShort("BurnTime",(short)(c*5000-32768));tag.setShort("CookTime",(short)(32767-c*5000));
                    NBTTagList list=new NBTTagList();
                    for(int k=0;k<20;k++) {NBTTagCompound s=new NBTTagCompound();s.setByte("Slot",(byte)(k-3));s.setShort("id",(short)new int[]{15,263,12345,500,0,5,327,351}[((k+c)&7)]);s.setByte("Count",(byte)(c*17+k));s.setShort("Damage",(short)(c*5000+k));list.setTag(s);}
                    tag.setTag("Items",list);t.readFromNBT(tag);nbt(o,t);snapshot(o,w,t,inv);
                    if(id==61){TileEntityFurnace f=(TileEntityFurnace)t;EntityOracle.i(o,Trace10.namedInt("f.furnaceBurnTime", f.furnaceBurnTime));EntityOracle.i(o,Trace10.namedInt("f.currentItemBurnTime", f.currentItemBurnTime));EntityOracle.i(o,Trace10.namedInt("f.furnaceCookTime", f.furnaceCookTime));}
                }
                PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);
                for(boolean replaced:new boolean[]{false,true})for(boolean invalid:new boolean[]{false,true})for(double x:new double[]{.5,8.5,16.5,16.50001,Double.NaN,Double.POSITIVE_INFINITY}) {
                    t.validate();w.setBlockTileEntity(8,64,8,t);if(replaced)w.setBlockTileEntity(8,64,8,id==61?new TileEntityFurnace():new TileEntityDispenser());if(invalid)t.invalidate();
                    p.posX=x;p.posY=64.5;p.posZ=8.5;Trace10.writeBoolean(o, "inv.isUseableByPlayer(p)", inv.isUseableByPlayer(p));inv.openChest();inv.closeChest();
                }
            }
            int[] values={Integer.MIN_VALUE,-65536,-1,0,1,199,200,32767,Integer.MAX_VALUE};
            for(int burn:values)for(int cook:values)for(int current:values)for(int scale:values) {
                TileEntityFurnace f=new TileEntityFurnace();f.furnaceBurnTime=burn;f.furnaceCookTime=cook;f.currentItemBurnTime=current;
                EntityOracle.i(o,Trace10.namedInt("f.getCookProgressScaled(scale)", f.getCookProgressScaled(scale)));EntityOracle.i(o,Trace10.namedInt("f.getBurnTimeRemainingScaled(scale)", f.getBurnTimeRemainingScaled(scale)));EntityOracle.i(o,Trace10.namedInt("f.currentItemBurnTime", f.currentItemBurnTime));Trace10.writeBoolean(o, "f.isBurning()", f.isBurning());
            }
            Method fuel=TileEntityFurnace.class.getDeclaredMethod("getItemBurnTime",ItemStack.class);fuel.setAccessible(true);
            for(int id=0;id<2258;id++)if(Item.itemsList[id]!=null)EntityOracle.i(o,Trace10.namedInt("(Integer)fuel.invoke(new TileEntityFurnace(),new ItemStack(id,-1,32767))", (Integer)fuel.invoke(new TileEntityFurnace(),new ItemStack(id,-1,32767))));
            Method can=TileEntityFurnace.class.getDeclaredMethod("canSmelt");can.setAccessible(true);
            for(boolean remote:new boolean[]{false,true})for(int count:new int[]{Integer.MIN_VALUE,-1,0,1,64,Integer.MAX_VALUE})for(int cook:values)for(int burn:new int[]{0,1,100})for(int output:new int[]{-1,0,63,64,65}) {
                World w=Mode_worldblocks10.fixture(true);Mode_worldblocks10.raw(w,8,64,8,61,5);TileEntityFurnace f=(TileEntityFurnace)w.getBlockTileEntity(8,64,8);
                ItemStack input=new ItemStack(15,count,0);f.setInventorySlotContents(0,input);input.stackSize=count;
                if(output!=-1){ItemStack result=new ItemStack(265,output,0);f.setInventorySlotContents(2,result);result.stackSize=output;}
                f.furnaceCookTime=cook;f.furnaceBurnTime=burn;w.multiplayerWorld=remote;w.getChunkFromChunkCoords(0,0).isModified=false;
                Trace10.writeBoolean(o, "(Boolean)can.invoke(f)", (Boolean)can.invoke(f));f.updateEntity();snapshot(o,w,f,f);EntityOracle.i(o,Trace10.namedInt("f.furnaceBurnTime", f.furnaceBurnTime));EntityOracle.i(o,Trace10.namedInt("f.currentItemBurnTime", f.currentItemBurnTime));EntityOracle.i(o,Trace10.namedInt("f.furnaceCookTime", f.furnaceCookTime));
            }
        }
    }
}
