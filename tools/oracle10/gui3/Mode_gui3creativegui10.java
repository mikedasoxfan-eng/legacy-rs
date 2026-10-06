package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;import org.lwjgl.input.Mouse;
public final class Mode_gui3creativegui10 {
 static class Screen extends GuiContainerCreative {
  ArrayList<int[]> rects=new ArrayList<int[]>();Screen(EntityPlayer p){super(p);}
  public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rects.add(new int[]{x,y,u,v,w,h});}
 }
 public static void main(String[] a)throws Exception {
  Block.stone.getClass();Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.gameSettings=new GameSettings();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);mc.theWorld=(World)Oracle.allocate(World.class);mc.thePlayer=Gui3Fixture10.player(mc.theWorld);mc.playerController=(PlayerControllerCreative)Oracle.allocate(PlayerControllerCreative.class);mc.fontRenderer=Gui2Fixture10.font();GuiContainer.itemRenderer=new Gui3Draw10.Item();Random r=new Random(30039);Mouse.reset();
  try(DataOutputStream o=Gui2Fixture10.out(a[0])){for(int n=0;n<500;n++){
   Screen g=new Screen(mc.thePlayer);g.mc=mc;g.fontRenderer=mc.fontRenderer;g.controlList=new ArrayList();g.width=300+r.nextInt(1000);g.height=240+r.nextInt(600);g.initGui();Gui3Fixture10.i(o,g.field_40216_e);Gui3Fixture10.i(o,g.field_40215_f);Gui3Fixture10.i(o,g.controlList.size());
   float scroll=r.nextFloat();Gui2Fixture10.field(GuiContainerCreative.class,"field_35312_g").setFloat(g,scroll);((ContainerCreative)g.inventorySlots).func_35374_a(scroll);g.width+=17;g.height+=23;
   Gui3Draw10.begin();g.drawScreen(-100,-100,0.5F);Gui3Fixture10.i(o,g.rects.size());for(int[] v:g.rects)for(int k:v)Gui3Fixture10.i(o,k);
  }}
 }
}
