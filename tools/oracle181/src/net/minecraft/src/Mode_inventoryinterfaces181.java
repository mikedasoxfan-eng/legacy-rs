package net.minecraft.src;
import java.io.*;

/** Calls original IInventory and Slot methods, including the b181 open/close additions. */
public final class Mode_inventoryinterfaces181 {
    public static void main(String[] args)throws Exception {
        World w=Mode_entities181.world();PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[1])))) {
            for(int mode=0;mode<3;mode++) {
                ContainerPlayer owner=new ContainerPlayer(p.inventory);IInventory inv=mode==0?new InventoryBasic("fixture",9):mode==1?owner.craftMatrix:owner.craftResult;
                for(int t=0;t<800;t++) {
                    int index=t%inv.getSizeInventory(),count=new int[]{-1,0,1,2,63,64,65,127}[t%8];
                    Slot slot=new Slot(inv,index,8+index*18,18);
                    ItemStack s=t%5==0?null:new ItemStack(new int[]{1,35,260,267}[t%4],count,t%16);
                    slot.putStack(s);inv.func_35142_x_();inv.func_35141_y_();
                    o.writeBoolean(slot.getHasStack());PlayerOracle.stack(o,slot.getStack());
                    EntityOracle.i(o,slot.getSlotStackLimit());EntityOracle.i(o,slot.getBackgroundIconIndex());
                    o.writeBoolean(slot.isItemValid(new ItemStack(267,1,0)));o.writeBoolean(inv.canInteractWith(p));
                    PlayerOracle.stack(o,slot.decrStackSize(t%7-1));slot.onPickupFromSlot(new ItemStack(1,1,0));
                    for(int n=0;n<inv.getSizeInventory();n++)PlayerOracle.stack(o,inv.getStackInSlot(n));
                    if(mode==1)PlayerOracle.stack(o,owner.craftResult.getStackInSlot(0));
                }
            }
        }
    }
}
