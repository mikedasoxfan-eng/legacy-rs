package net.minecraft.src;
import java.io.*;import net.minecraft.client.Minecraft;
/** Calls original HUD entrypoint; stops at the first debug font call. */
public final class IngameVersion11 {
 static class Stop extends RuntimeException {}
 static class Client extends Minecraft {Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new RuntimeException();}}
 static class Overlay extends EntityRenderer {Overlay(){super(null);}public void setupOverlayRendering(){}}
 static class Control extends PlayerControllerSP {Control(){super(null);}public boolean func_35643_e(){return true;}public boolean func_35642_f(){return false;}}
 static class Player extends EntityPlayerSP {Player(){super(null,null,null,0);}public boolean isPotionActive(Potion p){return false;}public int getSleepTimer(){return 0;}}
 static class Font extends FontRenderer {String text;int x,y,color;Font(){super(null,null,null,false);}public void drawStringWithShadow(String s,int x,int y,int color){this.text=s;this.x=x;this.y=y;this.color=color;throw new Stop();}}
 static void text(DataOutputStream o,String s)throws Exception{o.writeInt(s.length());for(int i=0;i<s.length();i++)o.writeChar(s.charAt(i));}
 public static void main(String[] args)throws Exception {
  Block.stone.getClass();Client mc=(Client)Oracle.allocate(Client.class);mc.gameSettings=new GameSettings();mc.gameSettings.showDebugInfo=true;mc.displayWidth=854;mc.displayHeight=480;mc.entityRenderer=(Overlay)Oracle.allocate(Overlay.class);mc.playerController=(Control)Oracle.allocate(Control.class);Player p=(Player)Oracle.allocate(Player.class);p.inventory=new InventoryPlayer(p);mc.thePlayer=p;Font f=(Font)Oracle.allocate(Font.class);mc.fontRenderer=f;
  try(DataOutputStream o=new DataOutputStream(new FileOutputStream(args[0]))){o.writeInt(0x49475631);String[] debug={"","60 fps, 0 chunk updates","\u00a7a42 fps","\u0394\u05d0\u65e5","null"};o.writeInt(debug.length);for(String s:debug){mc.debug=s;boolean stopped=false;try{new GuiIngame(mc).renderGameOverlay(0.5F,false,0,0);}catch(Stop expected){stopped=true;}if(!stopped)throw new AssertionError("debug not reached");text(o,s);text(o,f.text);o.writeInt(f.x);o.writeInt(f.y);o.writeInt(f.color);}}
 }
}
