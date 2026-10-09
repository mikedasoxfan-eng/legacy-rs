package net.minecraft.src;
import java.io.*;import java.lang.reflect.*;
/** Removed empty override exposes unchanged base callback and actual slider release. */
public final class ContainerRelease11 {
 static class ContainerScreen extends GuiContainer {ContainerScreen(){super(null);}protected void drawGuiContainerBackgroundLayer(float p,int x,int y){}}
 public static void main(String[] args)throws Exception{
  Field selected=GuiScreen.class.getDeclaredField("selectedButton");selected.setAccessible(true);Field dragging=GuiSlider.class.getDeclaredField("dragging");dragging.setAccessible(true);
  try(DataOutputStream o=new DataOutputStream(new FileOutputStream(args[0]))){o.writeInt(0x43524c31);o.writeInt(16);
   for(boolean present:new boolean[]{false,true})for(boolean initial:new boolean[]{false,true})for(int button:new int[]{-1,0,1,2}){ContainerScreen screen=new ContainerScreen();GuiSlider slider=(GuiSlider)Oracle.allocate(GuiSlider.class);dragging.setBoolean(slider,initial);selected.set(screen,present?slider:null);o.writeBoolean(present);o.writeBoolean(initial);o.writeInt(button);screen.mouseMovedOrUp(-17,91,button);o.writeBoolean(selected.get(screen)!=null);o.writeBoolean(dragging.getBoolean(slider));screen.mouseMovedOrUp(11,-23,0);o.writeBoolean(selected.get(screen)!=null);o.writeBoolean(dragging.getBoolean(slider));}
  }
 }
}
