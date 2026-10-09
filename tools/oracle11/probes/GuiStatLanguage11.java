package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
/** Invoke unchanged stat/achievement methods after registration, across all installed locale switches. */
public final class GuiStatLanguage11 {
 static class Client extends Minecraft {Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}}
 static void text(DataOutputStream o,String s)throws Exception{o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 static String field(GuiAchievement a,String name)throws Exception{java.lang.reflect.Field f=GuiAchievement.class.getDeclaredField(name);f.setAccessible(true);return (String)f.get(a);}
 public static void main(String[] args)throws Exception{
  Locale.setDefault(Locale.US);Block.stone.getClass();AchievementList.openInventory.getClass();
  Client mc=(Client)Oracle.allocate(Client.class);mc.gameSettings=new GameSettings();AchievementList.openInventory.setStatStringFormatter(new StatStringFormatKeyInv(mc));
  ArrayList<String> codes=new ArrayList<String>(StringTranslate.getInstance().func_44022_b().keySet());codes.add("en_US");
  try(DataOutputStream o=new DataOutputStream(new BufferedOutputStream(new FileOutputStream(args[0])))){
   o.writeInt(0x47534c31);o.writeInt(codes.size());
   for(String code:codes){StringTranslate.getInstance().func_44023_a(code);text(o,code);for(String key:new String[]{"menu.switchingLevel","menu.generatingLevel","menu.loadingLevel","menu.generatingTerrain","menu.simulating","menu.respawning"})text(o,StatCollector.translateToLocal(key));o.writeInt(StatList.generalStats.size());
    for(Object value:StatList.generalStats){StatBase s=(StatBase)value;o.writeInt(s.statId);text(o,s.func_44020_i());text(o,s.toString());}
    o.writeInt(AchievementList.achievementList.size());for(Object value:AchievementList.achievementList){Achievement a=(Achievement)value;o.writeInt(a.statId);text(o,a.func_44020_i());text(o,a.toString());o.writeInt(12);
     for(int key:new int[]{18,0,54,200,255,-100,-99,-101,256,1024,Integer.MIN_VALUE,Integer.MAX_VALUE}){mc.gameSettings.keyBindInventory.keyCode=key;o.writeInt(key);text(o,a.getDescription());}
     GuiAchievement toast=new GuiAchievement(mc);toast.queueTakenAchievement(a);text(o,field(toast,"achievementGetLocalText"));text(o,field(toast,"achievementStatName"));toast.queueAchievementInformation(a);text(o,field(toast,"achievementGetLocalText"));text(o,field(toast,"achievementStatName"));
    }
   }
  }
 }
}
