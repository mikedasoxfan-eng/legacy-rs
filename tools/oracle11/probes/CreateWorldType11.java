package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;import net.minecraft.client.Minecraft;
public final class CreateWorldType11 {
 static class Client extends Minecraft {WorldSettings launched;Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}public void displayGuiScreen(GuiScreen s){}public ISaveFormat getSaveLoader(){return (ISaveFormat)Proxy.newProxyInstance(ISaveFormat.class.getClassLoader(),new Class[]{ISaveFormat.class},(p,m,a)->null);}public void startWorld(String folder,String name,WorldSettings s){launched=s;}}
 static GuiButton button(GuiScreen s,int id){for(Object b:s.controlList)if(((GuiButton)b).id==id)return (GuiButton)b;throw new AssertionError(id);}
 public static void main(String[] args)throws Exception{
  Block.stone.getClass();Client mc=(Client)Oracle.allocate(Client.class);mc.gameSettings=new GameSettings();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);mc.fontRenderer=Gui2Fixture10.font();
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x43575431);o.writeInt(324);
   for(String code:new String[]{"en_US","ar_SA","he_IL","ja_JP","el_GR","de_DE"})for(int[] size:new int[][]{{427,240},{854,480},{320,200}})for(int mode=0;mode<3;mode++)for(int feature=0;feature<2;feature++)for(int cycles=0;cycles<3;cycles++){
    StringTranslate.getInstance().func_44023_a(code);GuiCreateWorld g=new GuiCreateWorld(new GuiScreen());g.setWorldAndResolution(mc,size[0],size[1]);SmallGuiLanguage11.text(o,code);o.writeInt(size[0]);o.writeInt(size[1]);o.writeInt(mode);o.writeInt(feature);o.writeInt(cycles);SmallGuiLanguage11.buttons(o,g);
    for(int n=0;n<mode;n++)g.actionPerformed(button(g,2));g.actionPerformed(button(g,3));if(feature==1)g.actionPerformed(button(g,4));for(int n=0;n<cycles;n++)g.actionPerformed(button(g,5));SmallGuiLanguage11.buttons(o,g);
    g.setWorldAndResolution(mc,size[0]+17,size[1]+11);SmallGuiLanguage11.buttons(o,g);((GuiTextField)Gui2Fixture10.field(GuiCreateWorld.class,"textboxSeed").get(g)).setText("4181");mc.launched=null;g.actionPerformed(button(g,0));o.writeBoolean(mc.launched!=null);WorldSettings s=mc.launched;o.writeLong(s.getSeed());o.writeInt(s.getGameType());o.writeBoolean(s.isMapFeaturesEnabled());o.writeBoolean(s.getHardcoreEnabled());SmallGuiLanguage11.text(o,s.func_46107_e().func_46136_a());
   }
  }
 }
}
