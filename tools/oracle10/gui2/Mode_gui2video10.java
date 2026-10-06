package net.minecraft.src;
import java.io.*;import java.util.*;import net.minecraft.client.Minecraft;
public final class Mode_gui2video10 {
    static class Settings extends GameSettings {public void saveOptions(){}}
    static class Client extends Minecraft {Client(){super(null,null,null,1,1,false);}public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new AssertionError();}public void displayGuiScreen(GuiScreen s){}}
    static class Spy extends GuiVideoSettings {
        ArrayList<String> text=new ArrayList<String>();ArrayList<int[]> labels=new ArrayList<int[]>();
        Spy(GameSettings s){super(new GuiScreen(),s);}
        public void drawDefaultBackground(){}
        public void drawCenteredString(FontRenderer f,String s,int x,int y,int color){text.add(s);labels.add(new int[]{x,y,color});}
    }
    public static void main(String[] a)throws Exception {
        Client mc=(Client)Oracle.allocate(Client.class);mc.fontRenderer=Gui2Fixture10.font();mc.displayWidth=854;mc.displayHeight=480;Random r=new Random(11314);
        System.setProperty("com.ibm.vm.bitmode","32");System.setProperty("os.arch","x86");
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<300;c++) {
                Settings s=new Settings();s.guiScale=c%4;s.renderDistance=c%3;mc.gameSettings=s;Spy g=new Spy(s);g.mc=mc;g.fontRenderer=mc.fontRenderer;g.width=320+r.nextInt(1000);g.height=240+r.nextInt(1000);
                System.setProperty("sun.arch.data.model",c%2==0?"64":"32");g.initGui();
                for(int id:new int[]{15,16,14,200}){
                    GuiButton chosen=null;for(Object b:g.controlList)if(((GuiButton)b).id==id)chosen=(GuiButton)b;g.actionPerformed(chosen);
                    Gui2Fixture10.i(o,g.width);Gui2Fixture10.i(o,g.height);Gui2Fixture10.i(o,s.guiScale);Gui2Fixture10.i(o,s.field_41087_P);o.writeBoolean(s.clouds);Gui2Fixture10.buttons(o,g);
                    ArrayList buttons=new ArrayList(g.controlList);g.controlList.clear();g.text.clear();g.labels.clear();g.drawScreen(0,0,0);
                    Gui2Fixture10.i(o,g.text.size());for(int n=0;n<g.text.size();n++){Gui2Fixture10.s(o,g.text.get(n));for(int v:g.labels.get(n))Gui2Fixture10.i(o,v);}g.controlList=buttons;
                }
            }
        }
    }
}
