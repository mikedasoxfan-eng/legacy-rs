package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;import org.lwjgl.input.Mouse;
public final class Mode_gui2slot10 {
    static class List extends GuiSlot {
        int size;List(Minecraft mc,int w,int h,int top,int bottom,int sh){super(mc,w,h,top,bottom,sh);}
        protected int getSize(){return size;}protected boolean isSelected(int i){return false;}protected void elementClicked(int i,boolean d){}protected void drawBackground(){}protected void drawSlot(int i,int x,int y,int h,Tessellator t){}
    }
    public static void main(String[] a)throws Exception {
        Minecraft mc=(Minecraft)Oracle.allocate(PlayerOracle.HeadlessMinecraft.class);mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);Random r=new Random(11011);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<1000;c++) {
                int width=320+r.nextInt(800),height=200+r.nextInt(500),top=20+r.nextInt(40),bottom=height-30,sh=8+r.nextInt(60);
                List list=new List(mc,width,height,top,bottom,sh);list.size=r.nextInt(40);Mouse.reset();
                for(int t=0;t<8;t++){
                    int wheel=r.nextInt(1001)-500;boolean down=r.nextBoolean();Mouse.queue(0,down,wheel);
                    Gui2Fixture10.field(GuiSlot.class,"initialClickY").setFloat(list,-2F);list.drawScreen(0,0,0);
                    Gui2Fixture10.f(o,Gui2Fixture10.field(GuiSlot.class,"amountScrolled").getFloat(list));
                    Gui2Fixture10.f(o,Gui2Fixture10.field(GuiSlot.class,"initialClickY").getFloat(list));
                    Gui2Fixture10.i(o,list.func_27256_c(width/2,top+sh/2));
                    Oracle.weatherVertices.clear();Oracle.weatherLights.clear();
                }
            }
        }
    }
}
