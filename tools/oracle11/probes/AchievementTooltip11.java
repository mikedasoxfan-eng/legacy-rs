package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
/** Calls unchanged original achievement background, including both tooltip branches. */
public final class AchievementTooltip11 {
 static class Client extends Minecraft {Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}}
 static class Font extends FontRenderer {ArrayList<String> widths=new ArrayList<String>(),names=new ArrayList<String>(),splits=new ArrayList<String>();Font(RenderEngine e){super(new GameSettings(),"/font/default.png",e,false);}public int getStringWidth(String s){widths.add(s);return super.getStringWidth(s);}public void drawStringWithShadow(String s,int x,int y,int c){names.add(s);}public void drawSplitString(String s,int x,int y,int w,int c){splits.add(s);}}
 public static void main(String[] args)throws Exception{
  Block.stone.getClass();AchievementList.openInventory.getClass();Client mc=(Client)Oracle.allocate(Client.class);mc.gameSettings=new GameSettings();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);Font font=new Font(mc.renderEngine);mc.fontRenderer=font;
  ArrayList<String> codes=new ArrayList<String>(StringTranslate.getInstance().func_44022_b().keySet());codes.add("en_US");int per=0;for(Object v:AchievementList.achievementList)per+=((Achievement)v).parentAchievement==null?1:2;
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){o.writeInt(0x41545431);o.writeInt(per*codes.size());for(String code:codes){StringTranslate.getInstance().func_44023_a(code);for(int index=0;index<AchievementList.achievementList.size();index++){Achievement a=(Achievement)AchievementList.achievementList.get(index);for(boolean can:new boolean[]{true,false}){if(!can&&a.parentAchievement==null)continue;
   StatFileWriter stats=(StatFileWriter)Oracle.allocate(StatFileWriter.class);HashMap<Achievement,Integer> map=new HashMap<Achievement,Integer>();if(can&&a.parentAchievement!=null)map.put(a.parentAchievement,1);Gui2Fixture10.field(StatFileWriter.class,"field_25102_a").set(stats,map);
   GuiAchievements g=new GuiAchievements(stats);g.setWorldAndResolution(mc,427,240);g.field_27116_m=g.guiMapX=a.displayColumn*24-100;g.field_27115_n=g.guiMapY=a.displayRow*24-100;font.names.clear();font.splits.clear();font.widths.clear();Throwable failure=null;try{g.genAchievementBackground(211,146,0);}catch(java.util.IllegalFormatException e){failure=e;}
   String name=StatCollector.translateToLocal(a.func_44020_i());if(!font.widths.contains(name))throw new AssertionError("hover not reached "+index);SmallGuiLanguage11.text(o,code);o.writeInt(index);o.writeBoolean(can);SmallGuiLanguage11.text(o,name);o.writeBoolean(failure==null);if(failure!=null){SmallGuiLanguage11.text(o,failure.getClass().getName());SmallGuiLanguage11.text(o,failure.getMessage());}else{if(!font.names.get(font.names.size()-1).equals(name))throw new AssertionError("name draw");SmallGuiLanguage11.text(o,can?"":font.splits.get(font.splits.size()-1));}
  }}}}
 }
}
