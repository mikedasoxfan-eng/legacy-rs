package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
public final class Mode_gui3enchantclick10 {
 static class Controller extends PlayerControllerSP {
  ArrayList<int[]> calls;Controller(){super(null);}public void func_40593_a(int window,int offer){calls.add(new int[]{window,offer});}
 }
 public static void main(String[] a)throws Exception {
  Block.stone.getClass();Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.gameSettings=new GameSettings();World w=(World)Oracle.allocate(World.class);w.multiplayerWorld=true;mc.theWorld=w;mc.thePlayer=Gui3Fixture10.player(w);Controller controller=(Controller)Oracle.allocate(Controller.class);controller.calls=new ArrayList<int[]>();mc.playerController=controller;Random r=new Random(30043);
  try(DataOutputStream o=Gui2Fixture10.out(a[0])){for(int n=0;n<1000;n++){
   GuiEnchantment g=new GuiEnchantment(mc.thePlayer.inventory,w,0,0,0);g.mc=mc;g.controlList=new ArrayList();g.width=300+r.nextInt(1000);g.height=220+r.nextInt(600);g.initGui();ContainerEnchantment c=(ContainerEnchantment)g.inventorySlots;c.windowId=n;
   if(r.nextBoolean())c.tableInventory.setInventorySlotContents(0,new ItemStack(276,1,0));for(int k=0;k<3;k++)c.enchantLevels[k]=r.nextInt(40)-5;mc.thePlayer.playerLevel=r.nextInt(45)-5;int row=r.nextInt(3),mx=g.guiLeft+60+r.nextInt(112)-2,my=g.guiTop+14+19*row+r.nextInt(23)-2,button=r.nextInt(3);controller.calls.clear();g.mouseClicked(mx,my,button);Gui3Fixture10.i(o,controller.calls.size());for(int[] call:controller.calls)for(int v:call)Gui3Fixture10.i(o,v);
  }}
 }
}
