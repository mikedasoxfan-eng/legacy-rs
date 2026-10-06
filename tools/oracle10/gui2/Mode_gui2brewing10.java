package net.minecraft.src;
import java.io.*;import java.util.*;
import net.minecraft.client.Minecraft;
public final class Mode_gui2brewing10 {
    static class Tile extends TileEntityBrewingStand {int time;public int func_40053_g(){return time;}}
    static class Spy extends GuiBrewingStand {
        ArrayList<int[]> rects=new ArrayList<int[]>();Spy(){super(null,null);}
        public void drawTexturedModalRect(int x,int y,int u,int v,int w,int h){rects.add(new int[]{x,y,u,v,w,h});}
    }
    public static void main(String[] a)throws Exception {
        Spy g=(Spy)Oracle.allocate(Spy.class);g.rects=new ArrayList<int[]>();g.xSize=176;g.ySize=166;
        g.mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);g.mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);
        Tile tile=new Tile();Gui2Fixture10.field(GuiBrewingStand.class,"field_40217_h").set(g,tile);
        Random r=new Random(10304);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int t=-10;t<=1200;t++) {
                g.width=r.nextInt(1800);g.height=r.nextInt(1200);tile.time=t;g.rects.clear();g.drawGuiContainerBackgroundLayer(0.5F,0,0);
                Gui2Fixture10.i(o,g.rects.size());for(int[] rect:g.rects)for(int n:rect)Gui2Fixture10.i(o,n);
            }
        }
    }
}
