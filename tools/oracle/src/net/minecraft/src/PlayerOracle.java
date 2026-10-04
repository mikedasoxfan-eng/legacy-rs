package net.minecraft.src;

import java.io.*;
import java.util.*;

/** Player tests call the original jar's inventory, entity and controller methods. */
public final class PlayerOracle {
    static void i(DataOutputStream o,int n)throws IOException {EntityOracle.i(o,n);}
    static void stack(DataOutputStream o,ItemStack s)throws IOException {
        for(int n:s==null?new int[]{-1,0,0,0}:new int[]{s.itemID,s.stackSize,s.getItemDamage(),s.animationsToGo})i(o,n);
    }
    static void registry(String path)throws Exception {
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            for(int id=0;id<=2257;id++) {
                Item a=Item.itemsList[id];
                i(o,a==null?0:a.getItemStackLimit());i(o,a==null?0:a.getMaxDamage());i(o,a!=null&&a.getHasSubtypes()?1:0);i(o,a instanceof ItemArmor?((ItemArmor)a).damageReduceAmount:0);
                for(int b=0;b<256;b++) {
                    Block block=Block.blocksList[b];
                    EntityOracle.f(o,a==null||block==null?1:a.getStrVsBlock(new ItemStack(id,1,0),block));
                    o.writeBoolean(a!=null&&block!=null&&a.canHarvestBlock(block));
                }
            }
            java.lang.reflect.Field strength=Block.class.getDeclaredField("blockHardness");strength.setAccessible(true);
            for(int b=0;b<256;b++) {Block a=Block.blocksList[b];EntityOracle.f(o,a==null?0:strength.getFloat(a));o.writeBoolean(a!=null&&a.blockMaterial.getIsHarvestable());}
        }
    }
    static void inventory(String path)throws Exception {
        World w=Oracle.newWorld(0);w.chunkProvider=new EntityOracle.FlatProvider(w);
        EntityOtherPlayerMP player=new EntityOtherPlayerMP(w,"Player");InventoryPlayer inv=player.inventory;
        Random r=new Random(1729);int[] ids={1,35,263,256,257,298,299,300,301,332,351,357,2256};
        try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(path)))) {
            for(int t=0;t<10000;t++) {
                int op=r.nextInt(7),id=ids[r.nextInt(ids.length)],count=r.nextInt(80)+1,damage=r.nextInt(150),result=0;
                if(op<=1) {ItemStack s=new ItemStack(id,count,damage);result=inv.addItemStackToInventory(s)?1:0;stack(o,s);}
                else if(op==2)result=inv.consumeInventoryItem(id)?1:0;
                else if(op==3)stack(o,inv.decrStackSize(r.nextInt(40),count));
                else if(op==4) {inv.changeCurrentItem(count-40);result=inv.currentItem;}
                else if(op==5) {int slot=r.nextInt(4);inv.armorInventory[slot]=new ItemStack(298+r.nextInt(20),1,damage);inv.damageArmor(count);result=inv.getTotalArmorValue();}
                else {inv.decrementAnimations();inv.readFromNBT(inv.writeToNBT(new NBTTagList()));}
                i(o,result);for(ItemStack s:inv.mainInventory)stack(o,s);for(ItemStack s:inv.armorInventory)stack(o,s);i(o,inv.currentItem);i(o,inv.getTotalArmorValue());
            }
        }
    }
}
