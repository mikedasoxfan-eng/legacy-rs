package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;import org.lwjgl.opengl.GL11;
public final class Mode_gui3winflow10 {
 static class Host extends Minecraft {
  ArrayList<int[]> actions;
  Host(){super(null,null,null,1,1,false);}
  public void displayUnexpectedThrowable(UnexpectedThrowable e){}
  public void displayGuiScreen(GuiScreen s){actions.add(new int[]{0,s==null?0:1,0,0});}
  public void respawn(boolean remote,int dimension,boolean keep){actions.add(new int[]{1,remote?1:0,dimension,keep?1:0});}
 }
 static class Font extends FontRenderer {
  ArrayList<String> texts;ArrayList<int[]> args;ArrayList<Long> seeds;
  Font()throws Exception{super(new GameSettings(),"/font/default.png",(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class));texts=new ArrayList<String>();args=new ArrayList<int[]>();seeds=new ArrayList<Long>();}
  public void drawStringWithShadow(String s,int x,int y,int color){texts.add(s);args.add(new int[]{1,x,y,0,color});seeds.add(0L);}
  public void func_40609_a(String s,int x,int y,int w,int color,boolean shadow){texts.add(s);args.add(new int[]{shadow?2:3,x,y,w,color});seeds.add(field_41064_c.nextLong());}
 }
 static class Screen extends GuiWinGame {
  ArrayList<int[]> rects=new ArrayList<int[]>();
  public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rects.add(new int[]{x,y,u,v,w,h});}
 }
 public static void main(String[] a)throws Exception {
  Block.stone.getClass();Host mc=(Host)Oracle.allocate(Host.class);mc.actions=new ArrayList<int[]>();mc.theWorld=(World)Oracle.allocate(World.class);mc.theWorld.multiplayerWorld=false;mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);
  Font font=new Font();Screen g=new Screen();g.mc=mc;g.fontRenderer=font;g.controlList=new ArrayList();
  String[] lines={"[C]Minecraft 1.0","one two three four five","", "\u00a7a\u00a7kXXXXXXXXXXX", "[C]", "last line"};int[] heights={12,24,12,12,12,12};Gui2Fixture10.field(GuiWinGame.class,"field_41044_b").set(g,Arrays.asList(lines));Gui2Fixture10.field(GuiWinGame.class,"field_41045_c").set(g,heights);Gui2Fixture10.field(GuiWinGame.class,"field_41042_d").setInt(g,84);Random r=new Random(30035);
  try(DataOutputStream o=Gui2Fixture10.out(a[0])){for(int n=0;n<1000;n++){
   g.width=300+r.nextInt(800);g.height=200+r.nextInt(400);int tick=r.nextInt(4000)-1000;float partial=r.nextFloat();Gui2Fixture10.field(GuiWinGame.class,"updateCounter").setInt(g,tick);
   font.texts.clear();font.args.clear();font.seeds.clear();g.rects.clear();GL11.gui3Ops.clear();g.drawScreen(0,0,partial);
   Gui3Fixture10.i(o,g.rects.size());for(int[] rect:g.rects)for(int v:rect)Gui3Fixture10.i(o,v);
   Gui3Fixture10.i(o,GL11.gui3Ops.size());for(float[] op:GL11.gui3Ops)for(int k=1;k<6;k++)Gui3Fixture10.f(o,op[k]);
   Gui3Fixture10.i(o,font.texts.size());for(int k=0;k<font.texts.size();k++){Gui2Fixture10.s(o,font.texts.get(k));for(int v:font.args.get(k))Gui3Fixture10.i(o,v);Gui2Fixture10.l(o,font.seeds.get(k));}Gui3Fixture10.i(o,font.FONT_HEIGHT);
   mc.actions.clear();g.updateScreen();g.keyTyped('x',n%3==0?1:2);Gui3Fixture10.i(o,Gui2Fixture10.field(GuiWinGame.class,"updateCounter").getInt(g));Gui3Fixture10.i(o,mc.actions.size());for(int[] action:mc.actions)for(int v:action)Gui3Fixture10.i(o,v);o.writeBoolean(g.doesGuiPauseGame());
  }}
 }
}
