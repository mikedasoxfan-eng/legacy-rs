package net.minecraft.src;
import java.io.*;import java.util.*;
public final class Mode_gui3lifecycle10 {
 public static void main(String[] a)throws Exception {
  Block.stone.getClass();Random input=new Random(30040);try(DataOutputStream o=Gui2Fixture10.out(a[0])){for(int n=0;n<200;n++){
   Gui3Fixture10.ShelfWorld w=Gui3Fixture10.shelves(input);w.multiplayerWorld=n%3==0;Gui3Fixture10.Player p=Gui3Fixture10.player(w);ContainerEnchantment c=new ContainerEnchantment(p.inventory,w,0,0,0);Random rng=new Random(n);Gui2Fixture10.field(ContainerEnchantment.class,"field_40237_l").set(c,rng);
   for(int k=0;k<12;k++){if(k%3==0){c.slotClick(0,0,false,p);p.inventory.setItemStack(null);}else{int[] ids={276,278,340,1,373,359};int id=ids[input.nextInt(6)];c.getSlot(0).putStack(new ItemStack(id,1+input.nextInt(5),0));}Gui3Fixture10.enchstate(o,c,p,rng);Gui2Fixture10.l(o,rng.nextLong());}
  }}
 }
}
