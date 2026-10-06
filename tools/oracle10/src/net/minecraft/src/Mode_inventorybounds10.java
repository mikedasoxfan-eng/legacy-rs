package net.minecraft.src;
import java.io.*;

/** Exhausts the signed-short item-id domain through actual InventoryPlayer.readFromNBT. */
public final class Mode_inventorybounds10 {
    static void tag(NBTTagList list,int slot,int id,int count,int damage) {
        NBTTagCompound n=new NBTTagCompound();n.setByte("Slot",(byte)slot);n.setShort("id",(short)id);
        n.setByte("Count",(byte)count);n.setShort("Damage",(short)damage);list.setTag(n);
    }
    public static void main(String[] a)throws Exception {
        World w=Mode_entities10.world();PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<65536;c++) {
                InventoryPlayer inv=p.inventory;inv.currentItem=c%9;inv.inventoryChanged=(c&1)!=0;
                inv.setItemStack(new ItemStack(35,-3,14));inv.mainInventory[20]=new ItemStack(264,5,0);
                NBTTagList list=new NBTTagList();tag(list,0,260,3,0);tag(list,c*37,c,c,c*97);tag(list,100,310,1,7);
                boolean failed=false;try{inv.readFromNBT(list);}catch(ArrayIndexOutOfBoundsException expected){failed=true;}
                inv.openChest();inv.closeChest();
                Trace10.writeBoolean(o, "failed", failed);EntityOracle.i(o,Trace10.namedInt("inv.getSizeInventory()", inv.getSizeInventory()));
                for(ItemStack s:inv.mainInventory)PlayerOracle.stack(o,s);for(ItemStack s:inv.armorInventory)PlayerOracle.stack(o,s);
                PlayerOracle.stack(o,inv.getItemStack());EntityOracle.i(o,Trace10.namedInt("inv.currentItem", inv.currentItem));Trace10.writeBoolean(o, "inv.inventoryChanged", inv.inventoryChanged);
                Trace10.writeBoolean(o, "inv.hasItem((short)c)", inv.hasItem((short)c));Trace10.writeBoolean(o, "inv.hasItem(260)", inv.hasItem(260));Trace10.writeBoolean(o, "inv.hasItem(310)", inv.hasItem(310));
            }
        }
    }
}
