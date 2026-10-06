package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
public final class Mode_gui2menudraw10 {
    static class Spy extends GuiMainMenu {
        ArrayList<int[]> rects=new ArrayList<int[]>(),gradients=new ArrayList<int[]>(),labels=new ArrayList<int[]>();ArrayList<String> text=new ArrayList<String>();
        public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rects.add(new int[]{x,y,u,v,w,h});}
        protected void drawGradientRect(int x,int y,int w,int h,int top,int bottom){gradients.add(new int[]{x,y,w,h,top,bottom});}
        public void drawString(FontRenderer font,String s,int x,int y,int color){text.add(s);labels.add(new int[]{x,y,color});}
        public void drawCenteredString(FontRenderer font,String s,int x,int y,int color){}
    }
    public static void main(String[] a)throws Exception {
        Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.gameSettings=new GameSettings();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);mc.displayWidth=854;mc.displayHeight=480;
        FontRenderer font=Gui2Fixture10.font();float[] counters={0F,1e-6F,1e-4F,0.5F,-1F,Float.NaN};
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(float counter:counters){Spy g=new Spy();g.mc=mc;g.fontRenderer=font;g.width=427;g.height=240;Gui2Fixture10.field(GuiMainMenu.class,"updateCounter").setFloat(g,counter);g.drawScreen(0,0,0.5F);
                for(ArrayList<int[]> list:new ArrayList[]{g.gradients,g.rects}){Gui2Fixture10.i(o,list.size());for(int[] row:list)for(int n:row)Gui2Fixture10.i(o,n);}
                Gui2Fixture10.i(o,g.text.size());for(int n=0;n<g.text.size();n++){Gui2Fixture10.s(o,g.text.get(n));for(int value:g.labels.get(n))Gui2Fixture10.i(o,value);}
                Oracle.weatherVertices.clear();Oracle.weatherLights.clear();
            }
        }
    }
}
