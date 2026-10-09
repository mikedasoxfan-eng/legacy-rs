package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
public final class Mode_gui3containerdraw10 {
 public static void main(String[] a)throws Exception {
  Block.stone.getClass();Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.gameSettings=new GameSettings();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);mc.theWorld=(World)Oracle.allocate(World.class);mc.thePlayer=Gui3Fixture10.player(mc.theWorld);Gui3Draw10.Font font=new Gui3Draw10.Font();mc.fontRenderer=font;GuiContainer.itemRenderer=new Gui3Draw10.Item();Random r=new Random(30037);
  try(DataOutputStream o=Gui2Fixture10.out(a[0])){for(int n=0;n<400;n++){
   mc.thePlayer.inventory=new InventoryPlayer(mc.thePlayer);Gui3Draw10.Screen g=new Gui3Draw10.Screen(mc.thePlayer.inventory,new TileEntityDispenser());g.mc=mc;g.fontRenderer=font;g.controlList=new ArrayList();g.width=300+r.nextInt(1000);g.height=220+r.nextInt(600);g.initGui();
   int[] ids={1,264,276,373,2256};for(int k=0;k<45;k++)if(r.nextInt(4)==0){int id=ids[r.nextInt(ids.length)];ItemStack s=new ItemStack(id,1+r.nextInt(5),id==373?16:r.nextInt(5));if(id==276)s.addEnchantment(Enchantment.sharpness,3);g.inventorySlots.getSlot(k).putStack(s);}
   if(n%3==0)mc.thePlayer.inventory.setItemStack(new ItemStack(264,2,0));int slot=r.nextInt(45);Slot hover=g.inventorySlots.getSlot(slot);int mx=g.guiLeft+hover.xDisplayPosition+r.nextInt(19)-1;int my=g.guiTop+hover.yDisplayPosition+r.nextInt(19)-1;
   Gui3Draw10.begin();g.drawScreen(mx,my,0.5F);Gui3Draw10.write(o);
  }}
 }
}
