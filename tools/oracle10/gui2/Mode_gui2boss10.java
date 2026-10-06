package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;import net.minecraft.client.Minecraft;
public final class Mode_gui2boss10 {
    static class Spy extends GuiIngame {
        ArrayList<int[]> rectangles;
        Spy(){super(null);}
        public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rectangles.add(new int[]{x,y,u,v,w,h});}
    }
    static class Font extends FontRenderer {
        ArrayList<String> texts;ArrayList<int[]> labels;
        Font()throws Exception{super(new GameSettings(),"/font/default.png",(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class));}
        public void drawStringWithShadow(String s,int x,int y,int color){texts.add(s);labels.add(new int[]{x,y,color});}
    }
    public static void main(String[] a)throws Exception {
        Block.stone.getClass();Spy g=(Spy)Oracle.allocate(Spy.class);g.rectangles=new ArrayList<int[]>();
        Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.gameSettings=new GameSettings();mc.gameSettings.guiScale=1;mc.displayHeight=480;mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);
        Font font=new Font();font.texts=new ArrayList<String>();font.labels=new ArrayList<int[]>();mc.fontRenderer=font;Gui2Fixture10.field(GuiIngame.class,"mc").set(g,mc);
        EntityDragon dragon=(EntityDragon)Oracle.allocate(EntityDragon.class);dragon.field_40157_aB=200;dragon.dataWatcher=new DataWatcher();dragon.dataWatcher.addObject(16,Integer.valueOf(200));
        Method draw=GuiIngame.class.getDeclaredMethod("func_41039_c");draw.setAccessible(true);Random r=new Random(11415);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<1000;c++) {
                mc.displayWidth=320+r.nextInt(1000);dragon.dataWatcher.updateObject(16,Integer.valueOf(r.nextInt(251)-25));RenderDragon.field_41038_a=dragon;
                g.rectangles.clear();font.texts.clear();font.labels.clear();draw.invoke(g);draw.invoke(g);
                Gui2Fixture10.i(o,g.rectangles.size());for(int[] rect:g.rectangles)for(int n:rect)Gui2Fixture10.i(o,n);o.writeBoolean(RenderDragon.field_41038_a==null);
                Gui2Fixture10.i(o,font.texts.size());for(int n=0;n<font.texts.size();n++){Gui2Fixture10.s(o,font.texts.get(n));for(int v:font.labels.get(n))Gui2Fixture10.i(o,v);}
            }
        }
    }
}
