package net.minecraft.src;
import java.io.*;

/** Calls original IInventory and Slot methods, including the b10 open/close additions. */
public final class Mode_inventoryinterfaces10 {
    public static void main(String[] args)throws Exception {
        World w=Mode_entities10.world();PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(int mode=0;mode<3;mode++) {
                ContainerPlayer owner=new ContainerPlayer(p.inventory);IInventory inv=mode==0?new InventoryBasic("fixture",9):mode==1?owner.craftMatrix:owner.craftResult;
                for(int t=0;t<800;t++) {
                    int index=t%inv.getSizeInventory(),count=new int[]{-1,0,1,2,63,64,65,127}[t%8];
                    Slot slot=new Slot(inv,index,8+index*18,18);
                    ItemStack s=t%5==0?null:new ItemStack(new int[]{1,35,260,267}[t%4],count,t%16);
                    slot.putStack(s);inv.openChest();inv.closeChest();
                    Trace10.writeBoolean(o, "slot.getHasStack()", slot.getHasStack());PlayerOracle.stack(o,slot.getStack());
                    EntityOracle.i(o,Trace10.namedInt("slot.getSlotStackLimit()", slot.getSlotStackLimit()));EntityOracle.i(o,Trace10.namedInt("slot.getBackgroundIconIndex()", slot.getBackgroundIconIndex()));
                    Trace10.writeBoolean(o, "slot.isItemValid(new ItemStack(267,1,0))", slot.isItemValid(new ItemStack(267,1,0)));Trace10.writeBoolean(o, "inv.isUseableByPlayer(p)", inv.isUseableByPlayer(p));
                    PlayerOracle.stack(o,slot.decrStackSize(t%7-1));slot.onPickupFromSlot(new ItemStack(1,1,0));
                    for(int n=0;n<inv.getSizeInventory();n++)PlayerOracle.stack(o,inv.getStackInSlot(n));
                    if(mode==1)PlayerOracle.stack(o,owner.craftResult.getStackInSlot(0));
                }
            }
        }
    }
}
