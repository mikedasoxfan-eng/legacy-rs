package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import net.minecraft.client.Minecraft;
/** Actual original screen key callbacks, resize and startWorld arguments. */
public final class CreateWorldUnicode11 {
 static class Client extends Minecraft {String folder,name;WorldSettings launched;Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}public void displayGuiScreen(GuiScreen s){}public ISaveFormat getSaveLoader(){return (ISaveFormat)Proxy.newProxyInstance(ISaveFormat.class.getClassLoader(),new Class[]{ISaveFormat.class},(p,m,a)->null);}public void startWorld(String f,String n,WorldSettings s){folder=f;name=n;launched=s;}}
 static GuiTextField field(GuiCreateWorld g,String key)throws Exception{return (GuiTextField)Gui2Fixture10.field(GuiCreateWorld.class,key).get(g);}
 public static void main(String[] args)throws Exception {
  Block.stone.getClass();Client mc=(Client)Oracle.allocate(Client.class);mc.gameSettings=new GameSettings();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);mc.fontRenderer=Gui2Fixture10.font();
  int[][] sequences={{0xD83D},{0xD83D,0xDE00},{0xD83D,0xDE00,-14},{0xDE00}};
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){o.writeInt(0x43575531);o.writeInt(96);
   for(int initial:new int[]{0,30,31,32,63,128})for(int seq=0;seq<4;seq++)for(int seedInitial:new int[]{0,127,128,129}){
    GuiCreateWorld g=new GuiCreateWorld(new GuiScreen());g.setWorldAndResolution(mc,427,240);field(g,"textboxWorldName").setText(new String(new char[initial]).replace('\0','A'));for(int unit:sequences[seq])g.keyTyped(unit<0?0:(char)unit,unit<0?-unit:0);
    g.actionPerformed(CreateWorldType11.button(g,3));GuiTextField sf=field(g,"textboxSeed");sf.setText(new String(new char[seedInitial]).replace('\0','S'));field(g,"textboxWorldName").isFocused=false;sf.isFocused=true;for(int unit:sequences[seq])g.keyTyped(unit<0?0:(char)unit,unit<0?-unit:0);g.keyTyped('X',0);
    o.writeInt(initial);o.writeInt(seq);o.writeInt(seedInitial);SmallGuiLanguage11.text(o,field(g,"textboxWorldName").getText());SmallGuiLanguage11.text(o,sf.getText());
    g.setWorldAndResolution(mc,444,251);SmallGuiLanguage11.text(o,field(g,"textboxWorldName").getText());SmallGuiLanguage11.text(o,field(g,"textboxSeed").getText());mc.launched=null;g.actionPerformed(CreateWorldType11.button(g,0));if(mc.launched==null)throw new AssertionError("not launched");SmallGuiLanguage11.text(o,mc.folder);SmallGuiLanguage11.text(o,mc.name);o.writeLong(mc.launched.getSeed());
    ByteArrayOutputStream bytes=new ByteArrayOutputStream();new DataOutputStream(bytes).writeUTF(mc.name);o.writeInt(bytes.size());o.write(bytes.toByteArray());
   }
  }
 }
}
