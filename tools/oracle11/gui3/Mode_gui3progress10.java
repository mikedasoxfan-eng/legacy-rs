package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_gui3progress10 {
 static ArrayList<int[]> updates=new ArrayList<int[]>();
 static class Listener implements ICrafting {int index;Listener(int n){index=n;}public void updateCraftingInventory(Container c,List list){}public void updateCraftingInventorySlot(Container c,int id,ItemStack s){}public void updateCraftingInventoryInfo(Container c,int id,int value){updates.add(new int[]{index,id,value});}}
 static void write(DataOutputStream o)throws Exception{Gui3Fixture10.i(o,updates.size());for(int[] u:updates)for(int v:u)Gui3Fixture10.i(o,v);updates.clear();}
 public static void main(String[] a)throws Exception{
  Block.stone.getClass();Random r=new Random(30042);try(DataOutputStream o=Gui2Fixture10.out(a[0])){for(int kind=0;kind<2;kind++){
   World w=(World)Oracle.allocate(World.class);Gui3Fixture10.Player p=Gui3Fixture10.player(w);TileEntityBrewingStand tile=new TileEntityBrewingStand();Container c=kind==0?new ContainerEnchantment(p.inventory,w,0,0,0):new ContainerBrewingStand(p.inventory,tile);
   for(int n=0;n<1000;n++){for(int id=-1;id<4;id++)c.updateProgressBar(id,n%4==0?0:r.nextInt(500)-50);c.crafters.clear();for(int k=0;k<n%5;k++)c.crafters.add(new Listener(k));updates.clear();c.updateCraftingResults();write(o);int[] values=kind==0?((ContainerEnchantment)c).enchantLevels:new int[]{tile.getBrewTime(),0,0};for(int v:values)Gui3Fixture10.i(o,v);c.updateCraftingResults();write(o);}
  }}
 }
}
