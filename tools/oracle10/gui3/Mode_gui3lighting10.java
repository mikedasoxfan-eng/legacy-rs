package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;import net.minecraft.client.Minecraft;import org.lwjgl.opengl.GL11;
public final class Mode_gui3lighting10 {
 static void lightCalls(DataOutputStream o)throws Exception {
  ArrayList<float[]> ops=new ArrayList<float[]>();for(int n=0;n+1<GL11.gui3Ops.size();n++){float[] a=GL11.gui3Ops.get(n),b=GL11.gui3Ops.get(n+1);if(a[1]==2&&a[2]==-30&&a[3]==0&&a[4]==1&&a[5]==0&&b[1]==2&&b[2]==165&&b[3]==1&&b[4]==0&&b[5]==0){ops.add(a);ops.add(b);}}
  Gui3Fixture10.i(o,ops.size()/2);for(float[] op:ops)for(int k=2;k<6;k++)Gui3Fixture10.f(o,op[k]);
 }
 public static void main(String[] a)throws Exception {
  Block.stone.getClass();Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.gameSettings=new GameSettings();mc.displayWidth=854;mc.displayHeight=480;mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);mc.fontRenderer=Gui2Fixture10.font();TileEntityRenderer.instance.renderEngine=mc.renderEngine;
  StatFileWriter stats=(StatFileWriter)Oracle.allocate(StatFileWriter.class);Gui2Fixture10.field(StatFileWriter.class,"field_25102_a").set(stats,new HashMap());Gui2Fixture10.field(StatFileWriter.class,"field_25101_b").set(stats,new HashMap());
  GuiStats g=new GuiStats(null,stats);g.mc=mc;g.fontRenderer=mc.fontRenderer;Method draw=GuiStats.class.getDeclaredMethod("drawItemSprite",int.class,int.class,int.class);draw.setAccessible(true);
  GuiAchievements achievements=new GuiAchievements(stats);achievements.mc=mc;achievements.fontRenderer=mc.fontRenderer;achievements.width=427;achievements.height=240;achievements.controlList=new ArrayList();
  GuiAchievement toast=new GuiAchievement(mc);Gui2Fixture10.field(GuiAchievement.class,"theAchievement").set(toast,AchievementList.openInventory);Gui2Fixture10.field(GuiAchievement.class,"achievementGetLocalText").set(toast,"Achievement get!");Gui2Fixture10.field(GuiAchievement.class,"achievementStatName").set(toast,"Taking Inventory");
  try(DataOutputStream o=Gui2Fixture10.out(a[0])){for(int n=0;n<100;n++){
   GL11.gui3Ops.clear();draw.invoke(g,17,23,1);lightCalls(o);
   GL11.gui3Ops.clear();achievements.drawScreen(-1,-1,n/100.0F);lightCalls(o);
   Gui2Fixture10.field(GuiAchievement.class,"achievementTime").setLong(toast,System.currentTimeMillis()-1500);GL11.gui3Ops.clear();toast.updateAchievementWindow();lightCalls(o);
  }}
 }
}
