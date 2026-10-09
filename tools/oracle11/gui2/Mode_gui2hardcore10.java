package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;import net.minecraft.client.Minecraft;
public final class Mode_gui2hardcore10 {
    static class Client extends Minecraft {
        int launched,type;boolean hardcore,features;long seed;
        Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new AssertionError();}
        public ISaveFormat getSaveLoader(){return (ISaveFormat)Proxy.newProxyInstance(ISaveFormat.class.getClassLoader(),new Class[]{ISaveFormat.class},(p,m,a)->m.getName().equals("getWorldInfo")?null:null);}
        public void displayGuiScreen(GuiScreen s){}
        public void startWorld(String folder,String name,WorldSettings s){launched++;type=s.getGameType();hardcore=s.getHardcoreEnabled();features=s.isMapFeaturesEnabled();seed=s.getSeed();}
    }
    public static void main(String[] a)throws Exception {
        Block.stone.getClass();Client mc=(Client)Oracle.allocate(Client.class);mc.gameSettings=new GameSettings();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);FontRenderer font=Gui2Fixture10.font();Random r=new Random(10910);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<300;c++) {
                GuiCreateWorld g=new GuiCreateWorld(new GuiScreen());g.mc=mc;g.fontRenderer=font;g.width=320+r.nextInt(1000);g.height=240+r.nextInt(1000);g.initGui();g.keyTyped('x',45);
                for(int t=0;t<c%3;t++)g.actionPerformed((GuiButton)g.controlList.get(2));
                g.actionPerformed((GuiButton)g.controlList.get(3));g.mouseClicked(g.width/2,65,0);g.keyTyped('7',8);
                g.width=320+r.nextInt(1000);g.height=240+r.nextInt(1000);g.controlList.clear();g.initGui();
                Gui2Fixture10.buttons(o,g);Gui2Fixture10.s(o,((GuiTextField)Gui2Fixture10.field(GuiCreateWorld.class,"textboxWorldName").get(g)).getText());Gui2Fixture10.s(o,((GuiTextField)Gui2Fixture10.field(GuiCreateWorld.class,"textboxSeed").get(g)).getText());
                Gui2Fixture10.s(o,(String)Gui2Fixture10.field(GuiCreateWorld.class,"gameMode").get(g));o.writeBoolean(Gui2Fixture10.field(GuiCreateWorld.class,"field_40232_h").getBoolean(g));
                mc.launched=0;g.actionPerformed((GuiButton)g.controlList.get(0));Gui2Fixture10.i(o,mc.launched);Gui2Fixture10.i(o,mc.type);o.writeBoolean(mc.hardcore);o.writeBoolean(mc.features);Gui2Fixture10.l(o,mc.seed);
            }
            for(int c=0;c<1000;c++) {
                boolean hardcore=c%2==0,worldOpen=c%3!=0;World w=(World)Oracle.allocate(World.class);w.worldInfo=new WorldInfo(new WorldSettings(1,0,true,hardcore, EnumWorldType.DEFAULT),"fixture");mc.theWorld=worldOpen?w:null;
                GuiOptions g=new GuiOptions(new GuiScreen(),mc.gameSettings);g.mc=mc;g.width=320+r.nextInt(1000);g.height=240+r.nextInt(1000);g.initGui();Gui2Fixture10.buttons(o,g);
            }
        }
    }
}
