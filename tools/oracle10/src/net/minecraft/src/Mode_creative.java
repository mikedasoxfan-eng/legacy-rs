package net.minecraft.src;
import java.io.*;import java.util.*;
import net.minecraft.client.Minecraft;
public final class Mode_creative {
 static void stack(DataOutputStream out,ItemStack s)throws Exception {Trace10.writeInt(out, "s==null?-1:s.itemID", s==null?-1:s.itemID);Trace10.writeInt(out, "s==null?0:s.stackSize", s==null?0:s.stackSize);Trace10.writeInt(out, "s==null?0:s.getItemDamage()", s==null?0:s.getItemDamage());}
 static void state(DataOutputStream out,Container c,EntityPlayer p)throws Exception {for(Object o:c.inventorySlots)stack(out,((Slot)o).getStack());stack(out,p.inventory.getItemStack());}
 public static void main(String[] a)throws Exception {
  Block init=Block.stone;
  ContainerOracle.StatsPlayer p=(ContainerOracle.StatsPlayer)Oracle.allocate(ContainerOracle.StatsPlayer.class);p.inventory=new InventoryPlayer(p);
  Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.thePlayer=p;mc.playerController=new PlayerControllerCreative(mc);
  GuiContainerCreative gui=new GuiContainerCreative(p);gui.mc=mc;ContainerCreative c=(ContainerCreative)gui.inventorySlots;
  try(DataOutputStream out=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(a[1])))) {
   Trace10.writeInt(out, "c.itemList.size()", c.itemList.size());for(Object o:c.itemList)stack(out,(ItemStack)o);
   for(float f:new float[]{-1,0,0.01F,0.1F,0.5F,0.99F,1,1.5F}) {c.func_35374_a(f);state(out,c,p);}
   Random r=new Random(181);
   for(int i=0;i<10000;i++) {if(i%13==0)c.func_35374_a(r.nextFloat());int slot=r.nextInt(81);int button=r.nextInt(2);boolean shift=r.nextBoolean();gui.func_35309_a(c.getSlot(slot),slot,button,shift);state(out,c,p);}
  }
 }
}
