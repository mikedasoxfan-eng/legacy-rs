package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;import net.minecraft.client.Minecraft;
/** Unchanged GuiLanguage/GuiSlotLanguage callbacks, live settings and original glyph draws. */
public final class LanguageWorkflow11 {
 static File directory;
 static class Client extends Minecraft {
  GuiScreen returned;int transitions;String savedAtTransition;
  Client(){super(null,null,null,1,1,false);}
  public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new AssertionError();}
  public void displayGuiScreen(GuiScreen s){returned=s;transitions++;try{savedAtTransition=savedLanguage(directory);}catch(Exception e){throw new RuntimeException(e);}}
 }
 static class Packs extends TexturePackList {int calls;Packs(){super(null,null);}public void updateAvaliableTexturePacks(){calls++;}}
 static Object field(Object o,Class<?> c,String name)throws Exception{Field f=c.getDeclaredField(name);f.setAccessible(true);return f.get(o);}
 static boolean flag(FontRenderer f,String name)throws Exception{return (Boolean)field(f,FontRenderer.class,name);}
 static void text(DataOutputStream o,String s)throws Exception{o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 static String savedLanguage(File dir)throws Exception{File f=new File(dir,"options.txt");if(!f.isFile())return "<missing>";try(BufferedReader in=new BufferedReader(new FileReader(f))){String line;while((line=in.readLine())!=null)if(line.startsWith("lang:"))return line.substring(5);}return "<missing>";}
 static void state(DataOutputStream o,Client mc,GuiLanguage screen)throws Exception{
  text(o,mc.gameSettings.field_44018_Q);text(o,StringTranslate.getInstance().func_44024_c());o.writeBoolean(flag(mc.fontRenderer,"field_44037_j"));o.writeBoolean(flag(mc.fontRenderer,"field_46125_m"));
  text(o,((GuiButton)field(screen,GuiLanguage.class,"field_46029_e")).displayString);o.writeInt((Integer)field(screen,GuiLanguage.class,"field_44007_b"));
 }
 static void vertices(DataOutputStream o,FontRenderer font)throws Exception{
  int total=0;for(RendererOracle.Draw10 d:RendererOracle.draws10)if(d.face.mode==5)total+=d.face.vertices.size();o.writeInt(total);
  int[] pages=(int[])field(font,FontRenderer.class,"field_44034_f");
  for(RendererOracle.Draw10 d:RendererOracle.draws10)if(d.face.mode==5){int texture=256;if(d.texture!=font.fontTextureName){texture=-1;for(int p=0;p<256;p++)if(pages[p]==d.texture){texture=p;break;}if(texture<0)throw new AssertionError("Nonfont strip");}
   for(float[] v:d.face.vertices){o.writeInt(texture);for(float f:v)o.writeFloat(f);for(float f:d.material)o.writeFloat(f);}
  }
 }
 public static void main(String[] args)throws Exception{
  Block.stone.getClass();RenderManager manager=RendererOracle.manager(null);RendererOracle.field("convertQuadsToTriangles").setBoolean(null,false);
  Client mc=(Client)Oracle.allocate(Client.class);mc.renderEngine=manager.renderEngine;Packs packs=(Packs)Oracle.allocate(Packs.class);mc.texturePackList=packs;
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x4c574631);o.writeInt(3);
   for(int[] size:new int[][]{{427,240},{854,480},{320,200}}){
    directory=new File(args[1],size[0]+"-"+size[1]);directory.mkdirs();new File(directory,"options.txt").delete();
    mc.gameSettings=new GameSettings(mc,directory);mc.fontRenderer=new FontRenderer(mc.gameSettings,"/font/default.png",mc.renderEngine,false);StringTranslate.getInstance().func_44023_a("en_US");
    GuiScreen parent=new GuiScreen();GuiLanguage screen=new GuiLanguage(parent,mc.gameSettings);screen.setWorldAndResolution(mc,size[0],size[1]);GuiSlotLanguage slot=(GuiSlotLanguage)field(screen,GuiLanguage.class,"field_44008_c");
    ArrayList<String> codes=new ArrayList<String>(StringTranslate.getInstance().func_44022_b().keySet());
    o.writeInt(size[0]);o.writeInt(size[1]);o.writeInt(codes.size());o.writeInt(slot.getContentHeight());
    for(String code:new String[]{"en_US","ar_SA","he_IL","ja_JP","el_GR","de_DE"}){
     parent=new GuiScreen();screen=new GuiLanguage(parent,mc.gameSettings);screen.setWorldAndResolution(mc,size[0],size[1]);slot=(GuiSlotLanguage)field(screen,GuiLanguage.class,"field_44008_c");
     int index=codes.indexOf(code);slot.elementClicked(index,false);state(o,mc,screen);text(o,savedLanguage(directory));
     for(int row=0;row<codes.size();row++){
      RendererOracle.reset10();slot.drawSlot(row,-99,37+row%3*18,14,Tessellator.instance);vertices(o,mc.fontRenderer);o.writeBoolean(flag(mc.fontRenderer,"field_46125_m"));
     }
     for(int updates:new int[]{0,1,20}){
      for(int i=0;i<updates;i++)screen.updateScreen();org.lwjgl.input.Mouse.reset();RendererOracle.reset10();int before=packs.calls;screen.drawScreen(0,0,0);o.writeInt(packs.calls-before);state(o,mc,screen);vertices(o,mc.fontRenderer);
     }
     for(int i=0;i<4;i++){
      GuiButton scroll=new GuiButton(i==2?7:8,0,0,"scroll");scroll.enabled=i!=3;screen.actionPerformed(scroll);org.lwjgl.input.Mouse.reset();RendererOracle.reset10();int before=packs.calls;screen.drawScreen(0,0,0);o.writeInt(packs.calls-before);state(o,mc,screen);vertices(o,mc.fontRenderer);
     }
     GuiButton done=(GuiButton)field(screen,GuiLanguage.class,"field_46029_e");mc.transitions=0;done.enabled=false;screen.actionPerformed(done);o.writeInt(mc.transitions);text(o,savedLanguage(directory));done.enabled=true;
     screen.actionPerformed(new GuiButton(5,0,0,"unused"));o.writeInt(mc.transitions);text(o,savedLanguage(directory));
     screen.actionPerformed(done);o.writeInt(mc.transitions);o.writeBoolean(mc.returned==parent);text(o,mc.savedAtTransition);text(o,savedLanguage(directory));
     GameSettings reopened=new GameSettings(mc,directory);text(o,reopened.field_44018_Q);
     StringTranslate.getInstance().func_44023_a(reopened.field_44018_Q);FontRenderer restarted=new FontRenderer(reopened,"/font/default.png",mc.renderEngine,StringTranslate.getInstance().func_46110_d());restarted.func_46123_b(StringTranslate.func_46109_d(reopened.field_44018_Q));
     o.writeBoolean(flag(restarted,"field_44037_j"));o.writeBoolean(flag(restarted,"field_46125_m"));
    }
   }
  }
  System.out.println("LanguageWorkflow11:3resolutions,18selections,1008listdraws,126frames,18save/reopens");
 }
}
