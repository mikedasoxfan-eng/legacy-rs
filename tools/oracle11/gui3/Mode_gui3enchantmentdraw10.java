package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;import org.lwjgl.opengl.GL11;
public final class Mode_gui3enchantmentdraw10 {
 static class Book extends ModelBook {float[] pose;public void render(Entity e,float a,float b,float c,float d,float f,float scale){pose=new float[]{a,b,c,d,f,scale};}}
 static class Font extends FontRenderer {
  ArrayList<String> text=new ArrayList<String>();ArrayList<int[]> args=new ArrayList<int[]>();
  Font()throws Exception{super(new GameSettings(),"/font/default.png",(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class));}
  public void drawString(String s,int x,int y,int color){text.add(s);args.add(new int[]{0,x,y,0,color});}
  public void drawStringWithShadow(String s,int x,int y,int color){text.add(s);args.add(new int[]{1,x,y,0,color});}
  public void drawSplitString(String s,int x,int y,int w,int color){text.add(s);args.add(new int[]{2,x,y,w,color});}
 }
 static class Screen extends GuiEnchantment {
  ArrayList<int[]> rects;
  Screen(){super(null,null,0,0,0);}
  public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rects.add(new int[]{x,y,u,v,w,h});}
 }
 public static void main(String[] a)throws Exception {
  Block.stone.getClass();Screen g=(Screen)Oracle.allocate(Screen.class);g.rects=new ArrayList<int[]>();g.xSize=176;g.ySize=166;
  Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.gameSettings=new GameSettings();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);mc.thePlayer=Gui3Fixture10.player(null);g.mc=mc;
  Font font=new Font();g.fontRenderer=mc.fontRenderer=mc.standardGalacticFontRenderer=font;
  ContainerEnchantment c=(ContainerEnchantment)Oracle.allocate(ContainerEnchantment.class);c.enchantLevels=new int[3];Gui2Fixture10.field(GuiEnchantment.class,"containerEnchantment").set(g,c);
  Book book=new Book();Gui2Fixture10.field(GuiEnchantment.class,"field_40220_w").set(null,book);Random r=new Random(30034);
  try(DataOutputStream o=Gui2Fixture10.out(a[0])) {for(int n=0;n<1000;n++){
   mc.displayWidth=320+r.nextInt(1600);mc.displayHeight=240+r.nextInt(1000);mc.gameSettings.guiScale=n%4;ScaledResolution scaled=new ScaledResolution(mc.gameSettings,mc.displayWidth,mc.displayHeight);g.width=scaled.getScaledWidth();g.height=scaled.getScaledHeight();
   g.field_40225_j=r.nextFloat()*40-20;g.field_40229_i=r.nextFloat()*40-20;g.field_40221_n=r.nextFloat()*3-1;g.field_40224_m=r.nextFloat()*3-1;float partial=r.nextFloat();c.nameSeed=r.nextLong();for(int k=0;k<3;k++)c.enchantLevels[k]=r.nextInt(55)-5;mc.thePlayer.playerLevel=r.nextInt(55)-5;int mx=r.nextInt(g.width),my=r.nextInt(g.height);
   font.text.clear();font.args.clear();g.rects.clear();GL11.gui3Ops.clear();GL11.gui3Viewports.clear();g.drawGuiContainerBackgroundLayer(partial,mx,my);g.drawGuiContainerForegroundLayer();
   for(int k:GL11.gui3Viewports.get(0))Gui3Fixture10.i(o,k);
   ArrayList<float[]> ops=new ArrayList<float[]>();for(float[] op:GL11.gui3Ops)if(op[0]==5888)ops.add(op);Gui3Fixture10.i(o,ops.size());for(float[] op:ops)for(int k=1;k<6;k++)Gui3Fixture10.f(o,op[k]);for(float v:book.pose)Gui3Fixture10.f(o,v);
   Gui3Fixture10.i(o,g.rects.size());for(int[] rect:g.rects)for(int k:rect)Gui3Fixture10.i(o,k);
   Gui3Fixture10.i(o,font.text.size());for(int k=0;k<font.text.size();k++){Gui2Fixture10.s(o,font.text.get(k));for(int v:font.args.get(k))Gui3Fixture10.i(o,v);}
  }}
 }
}
