package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
/** Original resource parsing, per-line heights, resize preservation and background vertices. */
public final class Mode_gui2win10 {
    public static void main(String[] a)throws Exception {
        Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.session=new Session("Player_10","");mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);
        GuiWinGame g=new GuiWinGame();g.mc=mc;g.fontRenderer=Gui2Fixture10.font();g.width=427;g.height=240;g.initGui();
        Gui2Fixture10.field(Tessellator.class,"convertQuadsToTriangles").setBoolean(null,false);
        java.lang.reflect.Method background=GuiWinGame.class.getDeclaredMethod("func_41040_b",int.class,int.class,float.class);background.setAccessible(true);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            List lines=(List)Gui2Fixture10.field(GuiWinGame.class,"field_41044_b").get(g);int[] heights=(int[])Gui2Fixture10.field(GuiWinGame.class,"field_41045_c").get(g);
            Gui2Fixture10.i(o,lines.size());for(int n=0;n<lines.size();n++){Gui2Fixture10.s(o,(String)lines.get(n));Gui2Fixture10.i(o,heights[n]);}
            Gui2Fixture10.i(o,Gui2Fixture10.field(GuiWinGame.class,"field_41042_d").getInt(g));Gui2Fixture10.i(o,g.fontRenderer.field_41063_b);o.writeBoolean(g.doesGuiPauseGame());
            g.height=360;g.initGui();Gui2Fixture10.i(o,Gui2Fixture10.field(GuiWinGame.class,"field_41042_d").getInt(g));
            Random r=new Random(11213);for(int c=0;c<1000;c++) {
                int tick=r.nextInt(100000)-1000;float partial=r.nextFloat();Gui2Fixture10.field(GuiWinGame.class,"field_41046_a").setInt(g,tick);
                Oracle.weatherVertices.clear();background.invoke(g,0,0,partial);
                for(float[] v:Oracle.weatherVertices) {for(int n=1;n<=5;n++)Gui2Fixture10.f(o,v[n]);for(int n=6;n<=9;n++)Gui2Fixture10.f(o,v[n]);}
            }
        }
    }
}
