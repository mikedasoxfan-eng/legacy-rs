package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
/** Serialized virtual draw calls, including the zLevel supplied to RenderItem. */
final class Gui3Draw10 {
 static ByteArrayOutputStream bytes;static DataOutputStream out;static int count;
 static void begin(){bytes=new ByteArrayOutputStream();out=new DataOutputStream(bytes);count=0;}
 static void rect(int kind,int[] args,float z){try{count++;Gui3Fixture10.i(out,kind);for(int v:args)Gui3Fixture10.i(out,v);Gui3Fixture10.f(out,z);}catch(Exception e){throw new RuntimeException(e);}}
 static void text(String s,int[] args){try{count++;Gui3Fixture10.i(out,2);Gui2Fixture10.s(out,s);for(int v:args)Gui3Fixture10.i(out,v);}catch(Exception e){throw new RuntimeException(e);}}
 static void write(DataOutputStream o)throws Exception{Gui3Fixture10.i(o,count);o.write(bytes.toByteArray());}
 static class Font extends FontRenderer {
  Font()throws Exception{super(new GameSettings(),"/font/default.png",(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class));}
  public void drawString(String s,int x,int y,int color){text(s,new int[]{0,x,y,0,color});}
  public void drawStringWithShadow(String s,int x,int y,int color){text(s,new int[]{1,x,y,0,color});}
 }
 static class Item extends RenderItem {
  void record(int kind,ItemStack s,int x,int y){rect(3,new int[]{kind,s==null?-1:s.itemID,s==null?0:s.stackSize,s==null?0:s.getItemDamage(),x,y},zLevel);}
  public void renderItemIntoGUI(FontRenderer f,RenderEngine e,ItemStack s,int x,int y){record(0,s,x,y);}
  public void renderItemOverlayIntoGUI(FontRenderer f,RenderEngine e,ItemStack s,int x,int y){record(1,s,x,y);super.renderItemOverlayIntoGUI(f,e,s,x,y);}
 }
 static class Screen extends GuiDispenser {
  Screen(InventoryPlayer p,TileEntityDispenser tile){super(p,tile);}
  public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rect(0,new int[]{x,y,u,v,w,h},zLevel);}
  protected void drawGradientRect(int x,int y,int w,int h,int c,int d){rect(1,new int[]{x,y,w,h,c,d},zLevel);}
 }
}
