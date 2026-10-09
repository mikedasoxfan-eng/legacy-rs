package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
/** Invoke unchanged private Minecraft.convertMapFormat, intercepting only external save and world-open boundaries. */
public final class MinecraftConvert11 {
 static class Client extends Minecraft {
  WorldSettings captured;String folder,name;
  Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new AssertionError();}
  public void startWorld(String f,String n,WorldSettings s){folder=f;name=n;captured=s;}
 }
 static class Loading extends LoadingScreenRenderer {
  Loading(){super(null);}public void printText(String s){}public void displayLoadingString(String s){}
 }
 static class Saves implements ISaveFormat {
  int calls;public String getFormatName(){return "McRegion";}public ISaveHandler getSaveLoader(String s,boolean b){return null;}public List getSaveList(){return null;}public void flushCache(){}public WorldInfo getWorldInfo(String s){return null;}public void deleteWorldDirectory(String s){}public void renameWorld(String s,String t){}public boolean isOldMapFormat(String s){return true;}public boolean convertMapFormat(String s,IProgressUpdate p){calls++;return true;}
 }
 public static void main(String[] args)throws Exception {
  Client mc=(Client)Oracle.allocate(Client.class);mc.loadingScreen=new Loading();Saves save=new Saves();java.lang.reflect.Field loader=Minecraft.class.getDeclaredField("saveLoader");loader.setAccessible(true);loader.set(mc,save);
  java.lang.reflect.Method callback=Minecraft.class.getDeclaredMethod("convertMapFormat",String.class,String.class);callback.setAccessible(true);callback.invoke(mc,"legacy-save","Existing world");
  WorldSettings s=mc.captured;
  try(PrintWriter out=new PrintWriter(args[0])){out.println(s.getSeed()+"|"+s.getGameType()+"|"+s.isMapFeaturesEnabled()+"|"+s.getHardcoreEnabled()+"|"+s.func_46107_e().name()+"|"+save.calls+"|"+mc.folder+"|"+mc.name);}
 }
}

