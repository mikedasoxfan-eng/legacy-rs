package net.minecraft.src;
import java.io.*;

/** Exhausts the signed-short item-id domain through actual InventoryPlayer.readFromNBT. */
public final class Mode_inventorybounds181 {
    static void tag(NBTTagList list,int slot,int id,int count,int damage) {
        NBTTagCompound n=new NBTTagCompound();n.setByte("Slot",(byte)slot);n.setShort("id",(short)id);
        n.setByte("Count",(byte)count);n.setShort("Damage",(short)damage);list.setTag(n);
    }
    public static void main(String[] a)throws Exception {
        World w=Mode_entities181.world();PlayerOracle.HeadlessPlayer p=PlayerOracle.player(w);
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
            for(int c=0;c<65536;c++) {
                InventoryPlayer inv=p.inventory;inv.currentItem=c%9;inv.inventoryChanged=(c&1)!=0;
                inv.setItemStack(new ItemStack(35,-3,14));inv.mainInventory[20]=new ItemStack(264,5,0);
                NBTTagList list=new NBTTagList();tag(list,0,260,3,0);tag(list,c*37,c,c,c*97);tag(list,100,310,1,7);
                boolean failed=false;try{inv.readFromNBT(list);}catch(ArrayIndexOutOfBoundsException expected){failed=true;}
                inv.func_35142_x_();inv.func_35141_y_();
                o.writeBoolean(failed);EntityOracle.i(o,inv.getSizeInventory());
                for(ItemStack s:inv.mainInventory)PlayerOracle.stack(o,s);for(ItemStack s:inv.armorInventory)PlayerOracle.stack(o,s);
                PlayerOracle.stack(o,inv.getItemStack());EntityOracle.i(o,inv.currentItem);o.writeBoolean(inv.inventoryChanged);
                o.writeBoolean(inv.func_35157_d((short)c));o.writeBoolean(inv.func_35157_d(260));o.writeBoolean(inv.func_35157_d(310));
            }
        }
    }
}
