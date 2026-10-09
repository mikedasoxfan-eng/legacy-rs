package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
public final class SmallGuiLanguage11 {
 static File directory;static class Client extends Minecraft{GuiScreen returned;int transitions;String saved;Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}public void displayGuiScreen(GuiScreen s){returned=s;transitions++;try{saved=saved();}catch(Exception e){throw new RuntimeException(e);}}}
 static String saved()throws Exception{File p=new File(directory,"options.txt");if(!p.isFile())return "<missing>";for(String line:java.nio.file.Files.readAllLines(p.toPath(),java.nio.charset.StandardCharsets.UTF_8))if(line.startsWith("lang:"))return line.substring(5);return "<missing>";}
 static void text(DataOutputStream o,String s)throws Exception{o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 static void buttons(DataOutputStream o,GuiScreen s)throws Exception{o.writeInt(s.controlList.size());for(Object v:s.controlList){GuiButton b=(GuiButton)v;for(int n:new int[]{b.id,b.xPosition,b.yPosition,b.width,b.height})o.writeInt(n);o.writeBoolean(b.enabled);o.writeBoolean(b.drawButton);text(o,b.displayString);}}
 static class Button extends GuiButtonLanguage {ArrayList<int[]> rectangles=new ArrayList<int[]>();Button(int id,int x,int y){super(id,x,y);}public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rectangles.add(new int[]{x,y,u,v,w,h});super.drawTexturedModalRect(x,y,u,v,w,h);}}
 public static void main(String[] args)throws Exception{
  Block.stone.getClass();RenderManager manager=RendererOracle.manager(null);RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);Client mc=(Client)Oracle.allocate(Client.class);mc.renderEngine=manager.renderEngine;mc.gameSettings=new GameSettings();mc.fontRenderer=new FontRenderer(mc.gameSettings,"/font/default.png",mc.renderEngine,false);
  String[] codes={"en_US","ar_SA","he_IL","ja_JP","el_GR","de_DE"};
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x53474c31);o.writeInt(36);
   for(String code:codes)for(int[] size:new int[][]{{427,240},{854,480},{320,200}})for(boolean hide:new boolean[]{false,true}){
    StringTranslate.getInstance().func_44023_a(code);mc.hideQuitButton=hide;GuiMainMenu screen=new GuiMainMenu();screen.setWorldAndResolution(mc,size[0],size[1]);text(o,code);o.writeInt(size[0]);o.writeInt(size[1]);o.writeBoolean(hide);buttons(o,screen);mc.transitions=0;screen.actionPerformed((GuiButton)screen.controlList.get(screen.controlList.size()-1));o.writeInt(mc.transitions);o.writeBoolean(mc.returned instanceof GuiLanguage&&((GuiLanguage)mc.returned).field_44009_a==screen);
   }
   o.writeInt(36);
   for(String code:codes)for(int[] size:new int[][]{{427,240},{854,480},{320,200}})for(boolean enabled:new boolean[]{false,true}){
    directory=new File(args[1],code+size[0]+enabled);directory.mkdirs();new File(directory,"options.txt").delete();mc.gameSettings=new GameSettings(mc,directory);mc.gameSettings.field_44018_Q=code;StringTranslate.getInstance().func_44023_a(code);GuiOptions screen=new GuiOptions(new GuiScreen(),mc.gameSettings);screen.setWorldAndResolution(mc,size[0],size[1]);text(o,code);o.writeInt(size[0]);o.writeInt(size[1]);o.writeBoolean(enabled);buttons(o,screen);GuiButton button=null;for(Object value:screen.controlList)if(((GuiButton)value).id==102)button=(GuiButton)value;button.enabled=enabled;mc.transitions=0;mc.returned=null;mc.saved="<missing>";screen.actionPerformed(button);o.writeInt(mc.transitions);o.writeBoolean(mc.returned instanceof GuiLanguage&&((GuiLanguage)mc.returned).field_44009_a==screen);text(o,saved());text(o,mc.saved);
   }
   o.writeInt(18);for(String code:codes)for(int[] size:new int[][]{{427,240},{854,480},{320,200}}){StringTranslate.getInstance().func_44023_a(code);GuiIngameMenu screen=new GuiIngameMenu();screen.setWorldAndResolution(mc,size[0],size[1]);text(o,code);o.writeInt(size[0]);o.writeInt(size[1]);buttons(o,screen);}
   o.writeInt(64);
   for(int[] xy:new int[][]{{7,11},{-45,-31}})for(boolean visible:new boolean[]{false,true})for(boolean enabled:new boolean[]{false,true})for(int[] mouse:new int[][]{{-1,0},{0,-1},{0,0},{19,19},{20,19},{19,20},{20,20},{10,10}}){Button b=new Button(5,xy[0],xy[1]);b.drawButton=visible;b.enabled=enabled;RendererOracle.reset(0);b.drawButton(mc,xy[0]+mouse[0],xy[1]+mouse[1]);for(int n:new int[]{xy[0],xy[1],mouse[0],mouse[1]})o.writeInt(n);o.writeBoolean(visible);o.writeBoolean(enabled);o.writeInt(b.rectangles.size());for(int[] rect:b.rectangles)for(int n:rect)o.writeInt(n);}
  }
 }
}
