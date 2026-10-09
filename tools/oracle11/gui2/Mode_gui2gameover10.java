package net.minecraft.src;
import java.io.*;import java.util.*;import java.lang.reflect.*;import net.minecraft.client.Minecraft;
public final class Mode_gui2gameover10 {
    static class Client extends Minecraft {
        int action;Client(){super(null,null,null,1,1,false);}
        public void displayUnexpectedThrowable(UnexpectedThrowable t){throw new AssertionError();}
        public void displayGuiScreen(GuiScreen g){}
        public void changeWorld1(World w){action=2;}
        public void exitToMainMenu(String message){action=3;}
        public ISaveFormat getSaveLoader(){return (ISaveFormat)Proxy.newProxyInstance(ISaveFormat.class.getClassLoader(),new Class[]{ISaveFormat.class},(p,m,a)->m.getReturnType()==boolean.class?Boolean.TRUE:null);}
    }
    static class Player extends EntityPlayerSP {
        int score;Client client;Player(){super(null,null,new Session("Player",""),0);}
        public int getScore(){return score;}
        public void respawnPlayer(){client.action=1;}
    }
    static class WorldSpy extends World {
        WorldSpy(){super(null,"fixture",new WorldSettings(1,0,true,false, EnumWorldType.DEFAULT));}
        public ISaveHandler getSaveHandler(){return (ISaveHandler)Proxy.newProxyInstance(ISaveHandler.class.getClassLoader(),new Class[]{ISaveHandler.class},(p,m,a)->m.getName().equals("getSaveDirectoryName")?"fixture":null);}
    }
    static class Spy extends GuiGameOver {
        ArrayList<String> texts=new ArrayList<String>();ArrayList<int[]> labels=new ArrayList<int[]>();
        public void drawGradientRect(int x,int y,int w,int h,int a,int b){}
        public void drawCenteredString(FontRenderer font,String s,int x,int y,int color){texts.add(s);labels.add(new int[]{x,y,color});}
    }
    public static void main(String[] a)throws Exception {
        Block.stone.getClass();Client mc=(Client)Oracle.allocate(Client.class);mc.gameSettings=new GameSettings();mc.renderEngine=(Gui2Fixture10.Engine)Oracle.allocate(Gui2Fixture10.Engine.class);
        Player player=(Player)Oracle.allocate(Player.class);player.client=mc;mc.thePlayer=player;WorldSpy world=(WorldSpy)Oracle.allocate(WorldSpy.class);mc.theWorld=world;
        Random r=new Random(11516);
        try(DataOutputStream o=Gui2Fixture10.out(a[0])) {
            for(int c=0;c<1000;c++) {
                boolean hardcore=c%2==0;world.worldInfo=new WorldInfo(new WorldSettings(1,0,true,hardcore, EnumWorldType.DEFAULT),"fixture");mc.session=c%3==0?null:new Session("Player","");player.score=r.nextInt();
                Spy g=new Spy();g.mc=mc;g.width=320+r.nextInt(1000);g.height=240+r.nextInt(1000);g.initGui();Gui2Fixture10.buttons(o,g);
                List controls=g.controlList;g.controlList=new ArrayList();g.drawScreen(0,0,.5F);g.controlList=controls;
                Gui2Fixture10.i(o,g.texts.size());for(int n=0;n<g.texts.size();n++){Gui2Fixture10.s(o,g.texts.get(n));for(int v:g.labels.get(n))Gui2Fixture10.i(o,v);o.writeBoolean(n==0);}
                o.writeBoolean(g.doesGuiPauseGame());mc.action=0;g.keyTyped('x',1);Gui2Fixture10.i(o,mc.action);
                for(int id:new int[]{-1,0,1,2}){mc.action=0;g.actionPerformed(new GuiButton(id,0,0,""));Gui2Fixture10.i(o,mc.action);}
            }
        }
    }
}
