package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
public final class Mode_gui2disconnected10 {
    static class Client extends Minecraft {int displays;Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new AssertionError();}public void displayGuiScreen(GuiScreen s){if(!(s instanceof GuiMainMenu))throw new AssertionError();displays++;}}
    static class Spy extends GuiDisconnected {
        ArrayList<String> text=new ArrayList<String>();ArrayList<int[]> labels=new ArrayList<int[]>();
        Spy(String message,String detail,Object[] args){super(message,detail,args);}
        public void drawDefaultBackground(){}
        public void drawCenteredString(FontRenderer f,String s,int x,int y,int color){text.add(s);labels.add(new int[]{x,y,color});}
    }
    public static void main(String[] a)throws Exception {
        Client mc=(Client)Oracle.allocate(Client.class);Random r=new Random(11112);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<1000;c++) {
                String message=c%2==0?"disconnect.genericReason":"missing.key",detail=c%3==0?"disconnect.genericReason":"disconnect.timeout";
                Spy g=new Spy(message,detail,c%2==0?new Object[]{"test reason"}:null);g.mc=mc;g.width=r.nextInt(1800);g.height=r.nextInt(1200);g.initGui();Gui2Fixture10.buttons(o,g);
                g.controlList.clear();g.drawScreen(0,0,0);Gui2Fixture10.i(o,g.text.size());for(int n=0;n<g.text.size();n++){Gui2Fixture10.s(o,g.text.get(n));for(int value:g.labels.get(n))Gui2Fixture10.i(o,value);}
                mc.displays=0;g.keyTyped('x',1);Gui2Fixture10.i(o,mc.displays);for(int id:new int[]{-1,0,1}){mc.displays=0;g.actionPerformed(new GuiButton(id,0,0,""));Gui2Fixture10.i(o,mc.displays);}
            }
        }
    }
}
