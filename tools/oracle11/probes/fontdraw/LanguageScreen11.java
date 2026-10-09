package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import java.util.*;import net.minecraft.client.Minecraft;
public final class LanguageScreen11 {
 static class Client extends Minecraft{Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}}
 static Object allocate(Class<?> c)throws Exception{Class<?> u=Class.forName("sun.misc.Unsafe");Field f=u.getDeclaredField("theUnsafe");f.setAccessible(true);return u.getMethod("allocateInstance",Class.class).invoke(f.get(null),c);}
 static Object get(Object value,String name)throws Exception{Field f=value.getClass().getDeclaredField(name);f.setAccessible(true);return f.get(value);}
 static boolean flag(FontRenderer font,String name)throws Exception{Field f=FontRenderer.class.getDeclaredField(name);f.setAccessible(true);return f.getBoolean(font);}
 static void text(DataOutputStream o,String s)throws Exception{o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 static void buttons(DataOutputStream o,GuiScreen g)throws Exception{o.writeInt(g.controlList.size());for(Object value:g.controlList){GuiButton b=(GuiButton)value;for(int n:new int[]{b.id,b.xPosition,b.yPosition,b.width,b.height})o.writeInt(n);o.writeBoolean(b.enabled);o.writeBoolean(b.drawButton);text(o,b.displayString);}}
 public static void main(String[] args)throws Exception{
  Client mc=(Client)allocate(Client.class);mc.gameSettings=new GameSettings();mc.fontRenderer=new FontRenderer(mc.gameSettings,"/font/default.png",new FontDraw11.Textures(),false);
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x4c534331);o.writeInt(3);
   for(int[] size:new int[][]{{427,240},{854,480},{320,200}}){
    StringTranslate.getInstance().func_44023_a("en_US");mc.gameSettings.field_44018_Q="en_US";mc.fontRenderer.func_44032_a(false);mc.fontRenderer.func_46123_b(false);
    o.writeInt(size[0]);o.writeInt(size[1]);GuiOptions options=new GuiOptions(new GuiScreen(),mc.gameSettings);options.setWorldAndResolution(mc,size[0],size[1]);buttons(o,options);
    GuiLanguage screen=new GuiLanguage(options,mc.gameSettings);screen.setWorldAndResolution(mc,size[0],size[1]);buttons(o,screen);GuiSlotLanguage slot=(GuiSlotLanguage)get(screen,"field_44008_c");
    o.writeInt(slot.getSize());ArrayList<String> codes=new ArrayList<String>(StringTranslate.getInstance().func_44022_b().keySet());for(String code:codes){text(o,code);text(o,(String)StringTranslate.getInstance().func_44022_b().get(code));}
    for(int index=0;index<codes.size();index++){slot.elementClicked(index,(index&1)!=0);text(o,mc.gameSettings.field_44018_Q);text(o,StringTranslate.getInstance().func_44024_c());o.writeBoolean(flag(mc.fontRenderer,"field_44037_j"));o.writeBoolean(flag(mc.fontRenderer,"field_46125_m"));text(o,((GuiButton)get(screen,"field_46029_e")).displayString);for(int k=0;k<codes.size();k++)o.writeBoolean(slot.isSelected(k));}
   }
  }
 }
}
